use std::io::Result;
use crossterm::event::Event;
use crate::vi::vi::Rs;

pub enum Cr {
    Up,
    Dn,
    L,
    R,
}

pub trait Itoo{
    fn _q(&mut self) -> Result<()>;
    fn _del(&mut self) -> Result<()>;
    fn _ent(&mut self) -> Result<()>;
    fn _mv(&mut self,mv: Cr) -> Result<()>;
    fn _app(&mut self,ch: char) -> Result<()>;
    fn trans(&mut self,key: Event) -> Result<Rs>;
}