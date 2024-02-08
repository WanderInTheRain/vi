use crossterm::cursor::{MoveTo};
use crossterm::terminal::{Clear, ClearType,size};
use std::io::{stdin, stdout, Result, Stdin, Stdout, Write, BufRead};
use crossterm::ExecutableCommand;

pub trait Outt {
    fn clearall(&mut self) -> Result<()>;
    fn clearline(&mut self) -> Result<()>;
    fn fpt(&mut self, s: &str) -> Result<()>;
    fn curmov(&mut self,cur: (u16, u16)) -> Result<()>;
    fn display(&mut self) -> Result<()>;
}

pub trait Intr {
    fn read(&self) -> Result<String>;
}

pub trait Shell {
    fn run(&mut self) -> Result<()>;
}

type Out = Stdout;
type In = Stdin;

pub struct Vi{
    out: Out,
    sin: In,
    cur: (u16,u16),
    text: Vec<String>,
}

impl Vi {
    pub fn new() -> Self{
        Vi{
            out: stdout(),
            sin: stdin(),
            cur: (0,0),
            text: Vec::new(),
        }
    }
}

impl Outt for Vi{
    fn clearall(&mut self) -> Result<()>{
        self.out.execute(Clear(ClearType::All))?;

        self.curmov((0,0))?;

        self.out.flush()?;

        Ok(())
    }

    fn clearline(&mut self) -> Result<()>{
        self.out.execute(Clear(ClearType::CurrentLine))?;

        self.out.flush()?;

        Ok(())
    }

    fn fpt(&mut self, s: &str) -> Result<()>{
        print!("{s}");

        self.out.flush()?;

        Ok(())
    }

    fn display(&mut self) -> Result<()>{
        self.clearall()?;

        for s in &self.text{
            print!("{s}");
        }

        self.out.flush()?;
        Ok(())
    }

    fn curmov(&mut self,cur: (u16, u16)) -> Result<()>{
        let (curx,cury) = cur;

        self.out.execute(MoveTo(curx, cury))?;
        
        self.out.flush()?;

        Ok(())
    }

}

impl Intr for Vi{
    fn read(&self) -> Result<String>{
        let mut input = String::new();
        self.sin.lock().read_line(&mut input)?;

        let input = input;

        Ok(input.to_string())
    }
}

impl Shell for Vi {
    fn run(&mut self) -> Result<()> {
        let mut iflag = true;
        loop {
            let (_,wy) = size()?;

            self.display()?;
            
            if iflag{
                let input = self.read()?;
                match input {
                    [27, 91, 65] => stdout.execute(MoveUp(1))?,    // 上箭头
                    [27, 91, 66] => stdout.execute(MoveDown(1))?,  // 下箭头
                    [27, 91, 68] => stdout.execute(MoveLeft(1))?,  // 左箭头
                    [27, 91, 67] => stdout.execute(MoveRight(1))?, // 右箭头
                    _ => {}
                }
                self.text.push(input.clone());
                if input.as_str() == "iq\n"{
                    iflag = false;
                }
            }
            else{
                self.curmov((0, wy - 1))?;
                self.clearline()?;
                self.fpt(":")?;
                let input = self.read()?;
                if input == "qq\n"{
                    break;
                }
                if input == "i\n"{
                    iflag = true;
                }
            }
        }
        self.clearall()?;
        Ok(())
    }
}
