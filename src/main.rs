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

    print_send(&mut api, MazeRunnerRequest::ClearCell { x: 0, y: 0 })?;
    print_send(&mut api, MazeRunnerRequest::ClearCell { x: 0, y: 1 })?;
    print_send(&mut api, MazeRunnerRequest::Initialize)?;
    print_send(&mut api, MazeRunnerRequest::GetButtonsState)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(&mut api, MazeRunnerRequest::GetWallFront)?;
    print_send(&mut api, MazeRunnerRequest::GetWallRight)?;
    print_send(
        &mut api,
        MazeRunnerRequest::UpdateCellState {
            x: 0,
            y: 0,
            state: CellState::EastWall
                | CellState::SouthWall
                | CellState::WestWall
                | CellState::Visited,
        },
    )?;
    print_send(
        &mut api,
        MazeRunnerRequest::UpdateCellValue {
            x: 0,
            y: 0,
            value: 7,
        },
    )?;
    print_send(&mut api, MazeRunnerRequest::MoveForward)?;
    print_send(&mut api, MazeRunnerRequest::GetWallLeft)?;
    print_send(
        &mut api,
        MazeRunnerRequest::UpdateCellState {
            x: 0,
            y: 1,
            state: CellState::WestWall | CellState::Visited,
        },
    )?;
    print_send(
        &mut api,
        MazeRunnerRequest::UpdateCellValue {
            x: 0,
            y: 1,
            value: 99,
        },
    )?;
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
    print_send(&mut api, MazeRunnerRequest::GetButtonsState)?;

    Ok(())
}
