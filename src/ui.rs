use crate::game::{GameState, GameStatus, LogType};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};

#[derive(Debug, Default)]
pub struct GameUI;

impl GameUI {
    pub fn draw(&self, frame: &mut Frame, game: &GameState) {
        frame.render_widget(GameWidget(game), frame.area());
    }
}

struct GameWidget<'a>(&'a GameState);

impl Widget for GameWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main layout split
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // Render player stats
        let stats_text = Text::from(vec![
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
            .block(
                Block::bordered()
                    .title(" Player")
                    .border_style(Style::default().fg(match self.0.status() {
                        GameStatus::Playing => Color::Reset,
                        GameStatus::Victory => Color::Green,
                        GameStatus::GameOver => Color::Red,
                    })),
            )
            .render(chunks[0], buf);

        // Render fight log
        let mut log_lines = vec![];
        for log in self.0.logs() {
            let colored_line = match log.log_type {
                LogType::System => Line::from(log.message.clone()).yellow().bold(),
                LogType::Status => Line::from(log.message.clone()).blue(),
                LogType::Fight => Line::from(log.message.clone()).red(),
            };
            log_lines.push(colored_line);
        }

        let log_text = Text::from(log_lines);

        Paragraph::new(log_text)
            .block(
                Block::bordered()
                    .title(" Combat")
                    .border_style(Style::default().fg(match self.0.status() {
                        GameStatus::Playing => Color::Reset,
                        GameStatus::Victory => Color::Green,
                        GameStatus::GameOver => Color::Red,
                    })),
            )
            .render(chunks[1], buf);
    }
}
