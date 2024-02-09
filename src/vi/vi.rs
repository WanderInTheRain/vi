use std::io::{stdout, 
    Result, Stdout, Write};
use crossterm::{
    execute,cursor,
    event::{Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use crate::vi::out::Outt;
use crate::vi::intr::Intr;
use crate::vi::itoo::Itoo;
use crate::vi::shell::Shell;
use crate::vi::itoo::Cr;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub enum Rs {
    Q,
    S,
    C,
}

pub struct Vi{
    out: Stdout,
    cur: (u16,u16),
    text: Vec<String>,
    iflag: bool,
}

impl Vi {
    pub fn new() -> Self{
        let mut text = Vec::new();
        text.push(String::from("_"));
        Vi{
            out: stdout(),
            cur: (0,0),
            text,
            iflag: true,
        }
    }
}

impl Intr for Vi{
    fn keyget(&self) -> Result<Event>{
        let key = crossterm::event::read()?;
        Ok(key)
    }
}

impl Outt for Vi{
    fn clearall(&mut self) -> Result<()>{
        self.out.execute(Clear(ClearType::All))?;

        self.curmov((0,0))?;

        self.out.flush()?;

        Ok(())
    }

    fn display(&mut self) -> Result<()>{
        self.clearall()?;
        self.out.execute(MoveTo(0, 0))?;
        if self.iflag {
            println!("insert mode: ESC to save mode");
        }
        else{
            println!("save mode: s-save q-quit i-insert");
        }

        for i in 0..self.text.len() {
            self.curmov((0, i as u16))?;
            println!("{}", self.text[i]);
        }
        
        self.out.flush()?;
        Ok(())
    }

    fn curmov(&mut self,cur: (u16, u16)) -> Result<()>{
        let (curx,cury) = cur;

        self.out.execute(MoveTo(curx, cury+1))?;
        
        self.out.flush()?;

        Ok(())
    }

}

impl Itoo for Vi {
    fn _mv(&mut self,mv: Cr) -> Result<()> {
        let (cx, cy) = self.cur;

        match mv {
            Cr::R => {
                let s = &mut self.text[cy as usize];
                if (cx as usize) < (s.len()-1){
                    let mut bytes = s.as_bytes().to_vec();
                    bytes.swap(cx as usize, (cx+1) as usize);
                    *s = String::from_utf8_lossy(&bytes).to_string();
                    self.cur = (cx+1,cy);
                }
            }
            Cr::L => {
                if cx >= 1{
                    let s = &mut self.text[cy as usize];
                    let mut bytes = s.as_bytes().to_vec();
                    bytes.swap(cx as usize, (cx-1) as usize);
                    *s = String::from_utf8_lossy(&bytes).to_string();
                    self.cur = (cx-1,cy);
                }
            }
            Cr::Dn => {
                if (cy as usize) < (self.text.len()-1){
                    let s = &mut self.text[cy as usize];
                    s.remove(cx as usize);
                    let l = self.text[(cy+1) as usize].len();
                    let ds = &mut self.text[(cy+1) as usize];
                    let nx = min(cx as usize,l);
                    ds.insert(nx,'_');
                    self.cur = (nx as u16,cy+1);
                }
            }
            Cr::Up => {
                if cy >= 1{
                    let s = &mut self.text[cy as usize];
                    s.remove(cx as usize);
                    let l = self.text[(cy-1) as usize].len();
                    let us = &mut self.text[(cy-1) as usize];
                    let nx = min(cx as usize,l);
                    us.insert(nx,'_');
                    self.cur = (nx as u16,cy-1);
                }
            }
        }
        Ok(())
    }
    fn _app(&mut self,ch: char) -> Result<()> {
        let (cx,cy) = self.cur;
        let s = &mut self.text[cy as usize];
        s.insert((cx+1) as usize,ch);
        Ok(())
    }
    fn _ent(&mut self) -> Result<()>{
        let (cx,cy) = self.cur;
        let new_s = self.text[cy as usize].split_off(cx as usize);
        self.text.insert((cy+1) as usize, new_s);  
        self.cur = (0,cy+1);
        Ok(())
    }
    fn _del(&mut self) -> Result<()> {
        let (cx,cy) = self.cur;
        if cx > 0{
            let s = &mut self.text[cy as usize];
            s.remove((cx-1) as usize);
            self.cur = (cx-1,cy);
        }else if cy>0{
            let tmp = self.text[cy as usize].clone();
            let l = self.text[(cy-1) as usize].len() as u16;
            self.text[(cy-1) as usize].push_str(tmp.as_str());
            self.text.remove(cy as usize);
            self.cur = (l,cy-1);
        }
        Ok(())
    }
    fn _q(&mut self) -> Result<()> {
        let (cx,cy) = self.cur;
        let s = &mut self.text[cy as usize];
        s.remove(cx as usize);
        Ok(())
    }
    fn trans(&mut self,key: Event) -> Result<Rs> {
        if let Event::Key(KeyEvent { code, .. }) = key {
            if self.iflag {
                match code {
                    KeyCode::Esc => {
                        self.iflag = false;
                    }
                    KeyCode::Char(ch) => {
                        self._app(ch)?;
                        self._mv(Cr::R)?;
                    }
                    KeyCode::Enter =>{
                        self._ent()?;
                    }
                    KeyCode::Left =>{
                        self._mv(Cr::L)?;
                    }
                    KeyCode::Right =>{
                        self._mv(Cr::R)?;
                    }
                    KeyCode::Up =>{
                        self._mv(Cr::Up)?;
                    }
                    KeyCode::Down =>{
                        self._mv(Cr::Dn)?;
                    }
                    KeyCode::Backspace => {
                        self._del()?;
                    }
                    KeyCode::Tab => {
                        for _ in 0..4{
                            self._app(' ')?;
                            self._mv(Cr::R)?;
                        }
                    }
                    _ => {}
                }
            }
            else{
                match code {
                    KeyCode::Char('s')  => {
                        self._q()?;
                        return Ok(Rs::S);
                    }
                    KeyCode::Char('q')  => {
                        self._q()?;
                        return Ok(Rs::Q);
                    }
                    KeyCode::Char('i') => {
                        self.iflag = true;
                    }
                    _ => {}
                }
            }
        }
        Ok(Rs::C)
    }
}


impl Shell for Vi {
    fn init(&mut self,path: &String) -> Result<()>{
        let mut file = File::open(path.as_str())?;

        execute!(self.out, cursor::Hide)?;
        enable_raw_mode()?;
    
        let mut content = String::new();
    
        file.read_to_string(&mut content)?;
        let lines: Vec<&str> = content.split('\n').collect();

        for i in 0..(lines.len()-1){
            for ch in lines[i].chars(){
                self.display()?;
                self._app(ch)?;
                self._mv(Cr::R)?;
            }
            self.text.push(String::new());
            self._mv(Cr::Dn)?;
        }

        for ch in lines[lines.len()-1].chars(){
            self.display()?;
            self._app(ch)?;
            self._mv(Cr::R)?;
        }

        self.run(path)?;
        Ok(())
    }

    fn run(&mut self,path: &String) -> Result<()> {
        loop {
            self.display()?;
            
            let key = self.keyget()?;
            if let Ok(st) = self.trans(key){
                match st {
                    Rs::S => {
                        self.save(path)?;
                        break;
                    }
                    Rs::Q => {
                        break;
                    }
                    _ =>{}
                }
            }
        }
        execute!(self.out, cursor::Show)?;
        self.clearall()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn save(&mut self,path: &String) -> Result<()> {
        let mut file = File::create(path)?;
        let n = self.text.len();
        for i in 0..n-1{
            self.text[i].push('\n');
            file.write_all(self.text[i].as_bytes())?;
        }

        file.write_all(self.text[n-1].as_bytes())?;

        Ok(())
    }
}
