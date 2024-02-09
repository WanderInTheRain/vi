mod vi {
    pub mod intr;
    pub mod vi;
    pub mod shell;
    pub mod out;
    pub mod itoo;
}

use vi::vi::Vi;
use vi::shell::Shell;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        let mut vi = Vi::new();
        vi.init(&args[0])?;
    }
    Ok(())
}