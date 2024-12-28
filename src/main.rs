use std::{io, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

const TICK_RATE: Duration = Duration::from_millis(100); // Logic update rate
const FRAME_RATE: Duration = Duration::from_millis(10);  // ~100 fps
const PLAYER_MAX_LEVEL: u8 = 100;

#[derive(Debug, Default)]
pub struct App {
    player_level: u8,
    exit: bool,
}

impl App {
    // Run until user quit
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut last_render = Instant::now();
        
        while !self.exit {
            // Only render frame if enough time has passed
            if last_render.elapsed() >= FRAME_RATE {
                terminal.draw(|frame| self.draw(frame))?;
                last_render = Instant::now();
            }
            
            // Time management for game logic
            let timeout = TICK_RATE
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            
            // Poll for events with a timeout
            if event::poll(timeout)? {
                self.handle_events()?;
            }
            
            // Update game state if it's time for a new tick
            if last_tick.elapsed() >= TICK_RATE {
                self.level_up();
                last_tick = Instant::now();
            }          
        }
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
    
    fn level_up(&mut self) {
        self.player_level += 1;
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
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
            self.player_level.to_string().yellow(),
        ])]);
        
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}