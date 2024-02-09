use std::io::Result;

pub trait Outt {
    fn clearall(&mut self) -> Result<()>;
    fn curmov(&mut self,cur: (u16, u16)) -> Result<()>;
    fn display(&mut self) -> Result<()>;
}