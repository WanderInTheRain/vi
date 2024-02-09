
use std::io::Result;
pub trait Shell {
    fn init(&mut self,path: &String) -> Result<()>;
    fn run(&mut self,path: &String) -> Result<()>;
    fn save(&mut self,path: &String) -> Result<()>;
}