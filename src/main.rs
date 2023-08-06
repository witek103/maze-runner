mod communication;
mod random_runner;

use communication::*;
use random_runner::RandomRunner;

fn main() -> Result<(), String> {
    let mut api = MazeRunnerApi::new()?;

    let mut runner = RandomRunner::new(&mut api)?;

    runner.run()?;

    Ok(())
}
