use std::{io, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

mod game;
mod ui;

use game::GameState;
use ui::GameUI;

const TICK_RATE: Duration = Duration::from_millis(1000);
const FRAME_RATE: Duration = Duration::from_millis(10);

pub struct App {
    game: GameState,
    ui: GameUI,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: GameState::new(1, 100, 10),
            ui: GameUI::default(),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut last_render = Instant::now();
        
        while !self.exit {
            if last_render.elapsed() >= FRAME_RATE {
                terminal.draw(|frame| self.draw(frame))?;
                last_render = Instant::now();
            }
            
            let timeout = TICK_RATE
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            
            if event::poll(timeout)? {
                self.handle_events()?;
            }
            
            if last_tick.elapsed() >= TICK_RATE {
                self.game.update();
                last_tick = Instant::now();
            }          
        }
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame) {
        self.ui.draw(frame, &self.game);
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}