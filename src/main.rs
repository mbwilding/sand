use anyhow::Result;
use sand::Sand;

mod block;
mod run;
mod sand;

fn main() -> Result<()> {
    let sand = Sand::new(5)?;

    run::run(sand)
}
