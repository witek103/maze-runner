mod communication;
mod floodfill_runner;

use communication::*;
use floodfill_runner::FloodfillRunner;

fn main() -> Result<(), String> {
    let mut api = MazeRunnerApi::new()?;

    let mut runner = FloodfillRunner::new(&mut api)?;

    runner.run()?;

    Ok(())
}
