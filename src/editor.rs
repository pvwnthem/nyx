use std::io::{self, Write};

use crossterm::terminal;


pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn run (&mut self) {
        terminal::enable_raw_mode().expect("Could not enable Raw mode");

        loop {
            if let Err(error) =  io::stdout().flush()  {
                panic!("Could not flush stdout: {}", error);
            }

            if self.quit {
                terminal::disable_raw_mode().expect("Could not disable Raw mode");
                break;
            }

            match crossterm::event::read().expect("Could not read event") {
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char('q') => self.quit = true,
                        _ => (),
                    }
                },
                _ => (),
            }
        }
        
    }
    pub fn default () -> Self {
        Self { quit: false }
    }
}