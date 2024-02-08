mod vi;

use vi::{Vi,Shell};

fn main() -> std::io::Result<()> {
    let mut vi = Vi::new();
    vi.run()?;
    Ok(())
}