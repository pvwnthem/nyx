use crate::terminal::Terminal;
use crossterm::event::{self, KeyCode};

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::enable_raw_mode().expect("Could not enable Raw mode");

        loop {
            if let Err(error) = self.refresh() {
                panic!("Could not flush stdout: {}", error);
            }

            if self.quit {
                Terminal::disable_raw_mode().expect("Could not disable Raw mode");
                break;
            } else {
                // main action
                self.draw_rows();
            }

            match event::read().expect("Could not read event") {
                event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => self.quit = true,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height() - 1 {
            println!("~");
        }
    }

    pub fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().expect("Could not initialize terminal"),
        }
    }

    pub fn refresh(&self) -> Result<(), std::io::Error> {
        if self.quit {
            Terminal::clear();
        } else {
            Terminal::clear();
            self.draw_rows();
            print!(
                "{} {}",
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 0)
            );
        }
        Terminal::flush()
    }
}
