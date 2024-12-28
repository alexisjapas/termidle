use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use crate::game::{GameState, GameStatus};

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
        let title = Line::from("  TERMIDLE ".bold());
        let instructions = Line::from(vec![
            "  Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let counter_text = Text::from(vec![Line::from(vec![
            "Level: ".into(),
            self.0.player_level().to_string().yellow(),
        ])]);
        
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}