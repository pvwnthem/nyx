pub enum Size {
    Width(u16),
    Height(u16),
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn size (&self) -> &Size {
        &self.size
    }

    pub fn default () -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size()?;
        Ok(Self { size: Size::Width(size.0) })
    }
}