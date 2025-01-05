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
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
            .split(area);

        // Player layout split
        let player_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(main_chunks[0]);

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
                " (Base) + ".into(),
                self.0.player_weapon().damages().to_string().red(),
                " (".into(),
                self.0.player_weapon().name().to_string().reset(),
                ")".into(),
            ]),
        ]);

        Paragraph::new(stats_text)
            .block(
                Block::bordered()
                    .title(" Player ")
                    .border_style(Style::default().fg(match self.0.status() {
                        GameStatus::PlayerSelection => Color::Reset,
                        GameStatus::Playing => Color::Reset,
                        GameStatus::Victory => Color::Green,
                        GameStatus::GameOver => Color::Red,
                    })),
            )
            .render(player_chunks[0], buf);

        // Render status widget
        let mut selection_lines = vec![];
        for (i, weapon) in self.0.weapons().iter().enumerate() {
            let style = if i == self.0.current_selection() {
                Style::default().bold().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default()
            };
            selection_lines.push(Line::styled(weapon.name().to_string(), style));
        }

        let enemy_lines = match self.0.enemy() {
            Some(enemy) => Text::from(vec![
                Line::from(vec!["Health: ".into(), enemy.health().to_string().green()]),
                Line::from(vec!["Attack: ".into(), enemy.attack().to_string().red()]),
            ]),
            None => Text::from("No enemy"),
        };

        Paragraph::new(match self.0.status() {
            GameStatus::PlayerSelection => Text::from(selection_lines),
            GameStatus::Playing => enemy_lines,
            GameStatus::Victory => Text::from(vec![Line::from("".to_string())]),
            GameStatus::GameOver => Text::from(vec![Line::from("".to_string())]),
        })
        .block(
            Block::bordered()
                .title(match self.0.status() {
                    GameStatus::PlayerSelection => " Select an option ",
                    GameStatus::Playing => " Current enemy ",
                    GameStatus::Victory => " Menu ",
                    GameStatus::GameOver => " Menu ",
                })
                .border_style(Style::default().fg(match self.0.status() {
                    GameStatus::PlayerSelection => Color::Yellow,
                    GameStatus::Playing => Color::Reset,
                    GameStatus::Victory => Color::Green,
                    GameStatus::GameOver => Color::Red,
                })),
        )
        .render(player_chunks[1], buf);

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
                    .title(" Logs history ")
                    .border_style(Style::default().fg(match self.0.status() {
                        GameStatus::PlayerSelection => Color::Reset,
                        GameStatus::Playing => Color::Yellow,
                        GameStatus::Victory => Color::Green,
                        GameStatus::GameOver => Color::Red,
                    })),
            )
            .render(main_chunks[1], buf);
    }
}
