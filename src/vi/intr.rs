use std::io::Result;
use crossterm::event::Event;


pub trait Intr {
    fn keyget(&self) -> Result<Event>;
}

