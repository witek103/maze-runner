use std::{thread::sleep, time::Duration};

use rand::seq::SliceRandom;

use crate::communication::{
    ButtonsState, CellState, MazeRunnerApi, MazeRunnerRequest, MazeRunnerResponse,
};

#[derive(Clone)]
enum RobotOrientation {
    Front,
    Left,
    Right,
    Back,
}

enum MazeOrientation {
    North,
    East,
    South,
    West,
}

pub struct RandomRunner<'a> {
    api: &'a mut MazeRunnerApi,
    orientation: MazeOrientation,
    position_x: usize,
    position_y: usize,
    move_count: i32,
    visited_history: [[bool; 16]; 16],
}

impl<'a> RandomRunner<'a> {
    pub fn new(api: &'a mut MazeRunnerApi) -> Result<Self, String> {
        api.send(MazeRunnerRequest::GetButtonsState)?;

        Self::clear_maze(api)?;

        api.send(MazeRunnerRequest::Initialize)?;

        println!("Runner initialized");

        Self::wait_for_btn1(api)?;

        Ok(Self {
            api,
            orientation: MazeOrientation::North,
            position_x: 0,
            position_y: 0,
            move_count: 0,
            visited_history: [[false; 16]; 16],
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("Runner started");

        loop {
            if self.finished() {
                break;
            }

            self.mark_cell()?;

            let next_move = self.get_next_move()?;

            self.make_move(next_move)?;
        }

        Ok(())
    }

    fn mark_cell(&mut self) -> Result<(), String> {
        if !self.visited_history[self.position_x][self.position_y] {
            self.api.send(MazeRunnerRequest::UpdateCellState {
                x: self.position_x,
                y: self.position_y,
                state: CellState::Visited,
            })?;

            self.visited_history[self.position_x][self.position_y] = true;

            self.api.send(MazeRunnerRequest::UpdateCellValue {
                x: self.position_x,
                y: self.position_y,
                value: self.move_count,
            })?;
        }

        Ok(())
    }

    fn finished(&self) -> bool {
        if (self.position_x == 7 || self.position_x == 8)
            && (self.position_y == 7 || self.position_y == 8)
        {
            println!("Finished!");

            return true;
        }

        false
    }

    fn get_next_move(&mut self) -> Result<RobotOrientation, String> {
        let mut possible_moves = Vec::<RobotOrientation>::new();

        if let MazeRunnerResponse::WallDetected(detected) =
            self.api.send(MazeRunnerRequest::GetWallFront)?
        {
            if detected {
                self.add_wall(RobotOrientation::Front)?;
            } else {
                possible_moves.push(RobotOrientation::Front);
            }
        }

        if let MazeRunnerResponse::WallDetected(detected) =
            self.api.send(MazeRunnerRequest::GetWallLeft)?
        {
            if detected {
                self.add_wall(RobotOrientation::Left)?;
            } else {
                possible_moves.push(RobotOrientation::Left);
            }
        }

        if let MazeRunnerResponse::WallDetected(detected) =
            self.api.send(MazeRunnerRequest::GetWallRight)?
        {
            if detected {
                self.add_wall(RobotOrientation::Right)?;
            } else {
                possible_moves.push(RobotOrientation::Right);
            }
        }

        Ok(possible_moves
            .choose(&mut rand::thread_rng())
            .unwrap_or(&RobotOrientation::Back)
            .clone())
    }

    fn add_wall(&mut self, robot_orienation: RobotOrientation) -> Result<(), String> {
        let state = match robot_orienation {
            RobotOrientation::Front => match self.orientation {
                MazeOrientation::North => CellState::NorthWall,
                MazeOrientation::East => CellState::EastWall,
                MazeOrientation::South => CellState::SouthWall,
                MazeOrientation::West => CellState::WestWall,
            },
            RobotOrientation::Left => match self.orientation {
                MazeOrientation::North => CellState::WestWall,
                MazeOrientation::East => CellState::NorthWall,
                MazeOrientation::South => CellState::EastWall,
                MazeOrientation::West => CellState::SouthWall,
            },
            RobotOrientation::Right => match self.orientation {
                MazeOrientation::North => CellState::EastWall,
                MazeOrientation::East => CellState::SouthWall,
                MazeOrientation::South => CellState::WestWall,
                MazeOrientation::West => CellState::NorthWall,
            },
            RobotOrientation::Back => return Ok(()),
        };

        self.api.send(MazeRunnerRequest::UpdateCellState {
            x: self.position_x,
            y: self.position_y,
            state,
        })?;

        Ok(())
    }

    fn make_move(&mut self, next_move: RobotOrientation) -> Result<(), String> {
        match next_move {
            RobotOrientation::Front => {
                self.move_forward()?;
            }
            RobotOrientation::Left => {
                self.rotate_left()?;
                self.move_forward()?;
            }
            RobotOrientation::Right => {
                self.rotate_right()?;
                self.move_forward()?;
            }
            RobotOrientation::Back => {
                self.rotate_left()?;
                self.rotate_left()?;
                self.move_forward()?;
            }
        }

        self.move_count += 1;

        Ok(())
    }

    fn rotate_left(&mut self) -> Result<(), String> {
        self.api.send(MazeRunnerRequest::RotateLeft90)?;

        self.orientation = match self.orientation {
            MazeOrientation::North => MazeOrientation::West,
            MazeOrientation::East => MazeOrientation::North,
            MazeOrientation::South => MazeOrientation::East,
            MazeOrientation::West => MazeOrientation::South,
        };

        Ok(())
    }

    fn rotate_right(&mut self) -> Result<(), String> {
        self.api.send(MazeRunnerRequest::RotateRight90)?;

        self.orientation = match self.orientation {
            MazeOrientation::North => MazeOrientation::East,
            MazeOrientation::East => MazeOrientation::South,
            MazeOrientation::South => MazeOrientation::West,
            MazeOrientation::West => MazeOrientation::North,
        };

        Ok(())
    }

    fn move_forward(&mut self) -> Result<(), String> {
        self.api.send(MazeRunnerRequest::MoveForward)?;

        match self.orientation {
            MazeOrientation::North => self.position_y += 1,
            MazeOrientation::East => self.position_x += 1,
            MazeOrientation::South => self.position_y -= 1,
            MazeOrientation::West => self.position_x -= 1,
        };

        Ok(())
    }

    fn clear_maze(api: &mut MazeRunnerApi) -> Result<(), String> {
        for x in 0..16 {
            for y in 0..16 {
                api.send(MazeRunnerRequest::ClearCell { x, y })?;
            }
        }

        Ok(())
    }

    fn wait_for_btn1(api: &mut MazeRunnerApi) -> Result<(), String> {
        println!("Press BTN1 to start Runner");

        loop {
            let response = api.send(MazeRunnerRequest::GetButtonsState)?;
            match response {
                MazeRunnerResponse::Buttons(buttons) => {
                    if buttons.contains(ButtonsState::Button1) {
                        break;
                    }
                }
                r => return Err(format!("Unexpected response: {r:?}")),
            }

            sleep(Duration::from_millis(1000));
        }

        Ok(())
    }
}
