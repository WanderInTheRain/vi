use std::io::Result;

pub trait Outt {
    fn clearall(&mut self) -> Result<()>;
    fn clearline(&mut self) -> Result<()>;
    fn fpt(&mut self, s: &str) -> Result<()>;
    fn curmov(&mut self,cur: (u16, u16)) -> Result<()>;
    fn display(&mut self) -> Result<()>;
}