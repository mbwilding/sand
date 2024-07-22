use anyhow::Result;
use sand::Sand;

mod block;
mod run;
mod sand;

fn main() -> Result<()> {
    let sand = Sand::new(60, 3)?;

    run::run(sand)
}
