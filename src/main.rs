mod communication;

use communication::*;

fn print_send(
    api: &mut MazeRunnerApi,
    request: MazeRunnerRequest,
) -> Result<MazeRunnerResponse, String> {
    let response = api.send(request)?;

    println!("{:?} -> {:?}", request, response);

    Ok(response)
}

fn main() -> Result<(), String> {
    println!("Starting micromouse!");

    let mut api = MazeRunnerApi::new()?;

    print_send(&mut api, MazeRunnerRequest::Initialize)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::MoveForward)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::RotateRight90)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::RotateRight90)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::MoveForward)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::RotateLeft90)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(&mut api, MazeRunnerRequest::RotateLeft90)?;

    Ok(())
}
