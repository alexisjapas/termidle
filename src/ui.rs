use ratatui::{
    buffer::Buffer,
    layout::{Rect, Layout, Direction, Constraint},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use crate::game::{GameState, GameStatus, LogType};

#[derive(Debug, Default)]
pub struct GameUI;

impl GameUI {
    pub fn draw(&self, frame: &mut Frame, game: &GameState) {
        match game.status() {
            GameStatus::Playing => frame.render_widget(GameWidget(game), frame.area()),
            GameStatus::Victory => frame.render_widget(VictoryWidget, frame.area()),
            GameStatus::GameOver => frame.render_widget(GameOverWidget, frame.area()),
        }
    }
}

struct VictoryWidget;

impl Widget for VictoryWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let victory_text = Text::from("Victory! You reached max level!").green().bold();
        Paragraph::new(victory_text)
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    }
}

struct GameOverWidget;

impl Widget for GameOverWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let game_over_text = Text::from("Game Over!").red().bold();
        Paragraph::new(game_over_text)
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    }
}

struct GameWidget<'a>(&'a GameState);

impl Widget for GameWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main layout split
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(area);
        
        // Render player stats
        let stats_text = Text::from(vec![
            Line::from("Player Stats").bold(),
            Line::from(""),
            Line::from(vec![
                "Level: ".into(),
                self.0.player_level().to_string().yellow(),
            ]),
            Line::from(vec![
                "Health: ".into(),
                self.0.player_health().to_string().green(),
            ]),
            Line::from(vec![
                "Attack: ".into(),
                self.0.player_attack().to_string().red(),
            ]),
        ]);
        
        Paragraph::new(stats_text)
            .block(Block::bordered().title(" Player"))
            .render(chunks[0], buf);
        
        // Render fight log
        let mut log_lines = vec![];
        for log in self.0.logs() {
            let colored_line = match log.log_type {
                LogType::System => Line::from(log.message.clone()).yellow(),
                LogType::Status => Line::from(log.message.clone()).blue(),
                LogType::Fight => Line::from(log.message.clone()).red(),
            };
            log_lines.push(colored_line);
        }
        
        let log_text = Text::from(log_lines);

        Paragraph::new(log_text)
            .block(Block::bordered().title(" Combat"))
            .render(chunks[1], buf);
    }
}