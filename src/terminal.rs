use std::io::{self, Write};

pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn default() -> Result<Self, std::io::Error> {
        let size: (u16, u16) = crossterm::terminal::size()?;
        Ok(Self {
            size: Size::new(size.0, size.1),
        })
    }

    pub fn flush() -> Result<(), std::io::Error> {
        print!(
            "{} {}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
        io::stdout().flush()
    }

    pub fn clear() {
        print!(
            "{} {}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
    }

    pub fn enable_raw_mode() -> Result<(), std::io::Error> {
        crossterm::terminal::enable_raw_mode()
    }

    pub fn disable_raw_mode() -> Result<(), std::io::Error> {
        crossterm::terminal::disable_raw_mode()
    }
}
