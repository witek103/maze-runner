use heapless::Deque;
use std::{thread::sleep, time::Duration};

use crate::communication::{
    ButtonsState, CellState, MazeRunnerApi, MazeRunnerRequest, MazeRunnerResponse,
};

#[derive(Clone, Copy, Debug)]
enum RunnerSide {
    Front,
    Left,
    Right,
    Back,
}

#[derive(Clone, Copy, Debug)]
enum MazeOrientation {
    North,
    East,
    South,
    West,
}

impl MazeOrientation {
    fn shifted(&self, runner_side: RunnerSide) -> Self {
        match self {
            MazeOrientation::North => match runner_side {
                RunnerSide::Front => MazeOrientation::North,
                RunnerSide::Left => MazeOrientation::West,
                RunnerSide::Right => MazeOrientation::East,
                RunnerSide::Back => MazeOrientation::South,
            },
            MazeOrientation::East => match runner_side {
                RunnerSide::Front => MazeOrientation::East,
                RunnerSide::Left => MazeOrientation::North,
                RunnerSide::Right => MazeOrientation::South,
                RunnerSide::Back => MazeOrientation::West,
            },
            MazeOrientation::South => match runner_side {
                RunnerSide::Front => MazeOrientation::South,
                RunnerSide::Left => MazeOrientation::East,
                RunnerSide::Right => MazeOrientation::West,
                RunnerSide::Back => MazeOrientation::North,
            },
            Self::West => match runner_side {
                RunnerSide::Front => MazeOrientation::West,
                RunnerSide::Left => MazeOrientation::South,
                RunnerSide::Right => MazeOrientation::North,
                RunnerSide::Back => MazeOrientation::East,
            },
        }
    }

    fn wall(&self) -> CellState {
        match self {
            MazeOrientation::North => CellState::NorthWall,
            MazeOrientation::East => CellState::EastWall,
            MazeOrientation::South => CellState::SouthWall,
            MazeOrientation::West => CellState::WestWall,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    x: u8,
    y: u8,
}

impl Cell {
    fn new(x: i16, y: i16) -> Result<Self, String> {
        if x < 0 || x >= 16 || y < 0 || y >= 16 {
            return Err(format!("Coordinates out of bands"));
        }

        Ok(Cell {
            x: x as u8,
            y: y as u8,
        })
    }

    fn neighbour(&self, orientation: MazeOrientation) -> Result<Cell, String> {
        match orientation {
            MazeOrientation::North => Self::new(self.x as i16, self.y as i16 + 1),
            MazeOrientation::East => Self::new(self.x as i16 + 1, self.y as i16),
            MazeOrientation::South => Self::new(self.x as i16, self.y as i16 - 1),
            MazeOrientation::West => Self::new(self.x as i16 - 1, self.y as i16),
        }
    }
}

#[derive(Debug)]
struct RunnerPosition {
    cell: Cell,
    orientation: MazeOrientation,
}

impl RunnerPosition {
    fn starting_position() -> Self {
        Self {
            cell: Cell::new(0, 0).expect("Hardcoded coordinates"),
            orientation: MazeOrientation::North,
        }
    }
}

pub struct FloodfillRunner<'a> {
    api: &'a mut MazeRunnerApi,
    position: RunnerPosition,
    values: [[u8; 16]; 16],
    maze: [[CellState; 16]; 16],
    stack: Deque<Cell, 1024>,
}

impl<'a> FloodfillRunner<'a> {
    pub fn new(api: &'a mut MazeRunnerApi) -> Result<Self, String> {
        api.send(MazeRunnerRequest::GetButtonsState)?;

        Self::clear_maze(api)?;

        api.send(MazeRunnerRequest::Initialize)?;

        let mut runner = Self {
            api,
            position: RunnerPosition::starting_position(),
            values: [[255; 16]; 16],
            maze: [[CellState::default(); 16]; 16],
            stack: Deque::new(),
        };

        runner.init_maze();

        Ok(runner)
    }

    fn init_maze(&mut self) {
        self.clear_square_values();

        for x in 0..16 {
            let cell = Cell::new(x, 0).expect("Hardcoded coordinates");
            self.set_cell_state(cell, CellState::SouthWall);

            let cell: Cell = Cell::new(x, 15).expect("Hardcoded coordinates");
            self.set_cell_state(cell, CellState::NorthWall);
        }

        for y in 0..16 {
            let cell = Cell::new(0, y).expect("Hardcoded coordinates");
            self.set_cell_state(cell, CellState::WestWall);

            let cell: Cell = Cell::new(15, y).expect("Hardcoded coordinates");
            self.set_cell_state(cell, CellState::EastWall);
        }
    }

    fn first_flood(&mut self) {
        self.stack.clear();

        for x in 0..16 {
            for y in 0..16 {
                let cell: Cell = Cell::new(x, y).expect("Hardcoded coordinates");

                if self.is_target_cell(cell) {
                    self.set_cell_value(cell, 0);

                    self.process_open_neighbours(cell);
                } else {
                    self.set_cell_value(cell, 255);
                }
            }
        }

        self.recalculate_values()
    }

    fn queue_for_recalculation(&mut self, cell: Cell) -> Result<(), String> {
        self.stack
            .push_back(cell)
            .map_err(|_| format!("Stack if full"))
    }

    fn recalculate_values(&mut self) {
        while let Some(cell) = self.stack.pop_back() {
            if !self.is_target_cell(cell) {
                let new_value = match self.get_open_neighbours_min_value(cell) {
                    255 => 255,
                    other => other + 1,
                };

                if self.get_cell_value(cell) != new_value {
                    self.set_cell_value(cell, new_value);

                    self.process_open_neighbours(cell);
                }
            }
        }
    }

    fn process_open_neighbours(&mut self, cell: Cell) {
        for orientation in [
            MazeOrientation::North,
            MazeOrientation::East,
            MazeOrientation::South,
            MazeOrientation::West,
        ] {
            if let Ok(neighbour) = cell.neighbour(orientation) {
                if !self.is_wall_at(cell, orientation) {
                    self.queue_for_recalculation(neighbour).unwrap();
                }
            }
        }
    }

    fn get_open_neighbours_min_value(&self, cell: Cell) -> u8 {
        let mut minimal = 255;

        for orientation in [
            MazeOrientation::North,
            MazeOrientation::East,
            MazeOrientation::South,
            MazeOrientation::West,
        ] {
            if let Ok(neighbour) = cell.neighbour(orientation) {
                if !self.is_wall_at(cell, orientation) {
                    minimal = core::cmp::min(minimal, self.get_cell_value(neighbour));
                }
            }
        }

        minimal
    }

    fn get_cell_state(&self, cell: Cell) -> CellState {
        self.maze[cell.x as usize][cell.y as usize]
    }

    fn set_cell_state(&mut self, cell: Cell, state: CellState) {
        self.maze[cell.x as usize][cell.y as usize].set(state, true);

        self.send(MazeRunnerRequest::UpdateCellState {
            x: cell.x as usize,
            y: cell.y as usize,
            state,
        });
    }

    fn get_cell_value(&self, cell: Cell) -> u8 {
        self.values[cell.x as usize][cell.y as usize]
    }

    fn set_cell_value(&mut self, cell: Cell, value: u8) {
        self.values[cell.x as usize][cell.y as usize] = value;

        self.send(MazeRunnerRequest::UpdateCellValue {
            x: cell.x as usize,
            y: cell.y as usize,
            value: value as i32,
        });
    }

    fn clear_square_values(&mut self) {
        for x in 0..16 {
            for y in 0..16 {
                let cell = Cell::new(x, y).expect("Hardcoded coordinates");
                self.set_cell_value(cell, 0);
            }
        }
    }

    fn is_target_cell(&self, cell: Cell) -> bool {
        if (cell.x == 7 || cell.x == 8) && (cell.y == 7 || cell.y == 8) {
            return true;
        }

        false
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.send(MazeRunnerRequest::Initialize);
            self.position = RunnerPosition::starting_position();

            if !self.continue_attempts() {
                break;
            }

            println!("Runner started");

            self.first_flood();

            loop {
                if self.finished() {
                    break;
                }

                self.queue_for_recalculation(self.position.cell)?;

                if !self.is_current_visited() {
                    self.process_walls();

                    self.mark_current_visited();
                }

                self.recalculate_values();

                let direction = self.get_next_move();

                self.make_move(direction)?;
            }
        }

        Ok(())
    }

    fn is_current_visited(&self) -> bool {
        self.get_cell_state(self.position.cell)
            .contains(CellState::Visited)
    }

    fn mark_current_visited(&mut self) {
        self.set_cell_state(self.position.cell, CellState::Visited);
    }

    fn process_walls(&mut self) {
        for (request, side) in [
            (MazeRunnerRequest::GetWallFront, RunnerSide::Front),
            (MazeRunnerRequest::GetWallRight, RunnerSide::Right),
            (MazeRunnerRequest::GetWallLeft, RunnerSide::Left),
        ] {
            if let MazeRunnerResponse::WallDetected(detected) = self.send(request) {
                if detected {
                    self.add_wall(side);

                    if let Ok(neighbour) = self
                        .position
                        .cell
                        .neighbour(self.position.orientation.shifted(side))
                    {
                        self.queue_for_recalculation(neighbour).unwrap();
                    }
                }
            }
        }
    }

    fn add_wall(&mut self, side: RunnerSide) {
        self.set_cell_state(
            self.position.cell,
            self.position.orientation.shifted(side).wall(),
        );

        if let Ok(neighbour) = self
            .position
            .cell
            .neighbour(self.position.orientation.shifted(side))
        {
            self.set_cell_state(
                neighbour,
                self.position
                    .orientation
                    .shifted(side)
                    .shifted(RunnerSide::Back)
                    .wall(),
            )
        }
    }

    fn finished(&self) -> bool {
        if self.is_target_cell(self.position.cell) {
            println!("Finished!");

            return true;
        }

        false
    }

    fn get_next_move(&self) -> RunnerSide {
        let mut minimal = 255;
        let mut next_move = RunnerSide::Back;

        for side in [RunnerSide::Front, RunnerSide::Right, RunnerSide::Left] {
            if !self.is_wall_next_to_runner(side) {
                if let Ok(neighbour) = self
                    .position
                    .cell
                    .neighbour(self.position.orientation.shifted(side))
                {
                    let value = self.get_cell_value(neighbour);

                    if value < minimal {
                        minimal = value;
                        next_move = side;
                    }
                }
            }
        }

        next_move
    }

    fn is_wall_next_to_runner(&self, side: RunnerSide) -> bool {
        let cell = self.position.cell;

        self.is_wall_at(cell, self.position.orientation.shifted(side))
    }

    fn is_wall_at(&self, cell: Cell, orientation: MazeOrientation) -> bool {
        let state = self.get_cell_state(cell);

        state.contains(orientation.wall())
    }

    fn make_move(&mut self, move_direction: RunnerSide) -> Result<(), String> {
        match move_direction {
            RunnerSide::Front => {
                self.move_forward()?;
            }
            RunnerSide::Left => {
                self.rotate_left();
                self.move_forward()?;
            }
            RunnerSide::Right => {
                self.rotate_right();
                self.move_forward()?;
            }
            RunnerSide::Back => {
                self.rotate_left();
                self.rotate_left();
                self.move_forward()?;
            }
        }

        Ok(())
    }

    fn rotate_left(&mut self) {
        self.send(MazeRunnerRequest::RotateLeft90);

        self.position.orientation = self.position.orientation.shifted(RunnerSide::Left);
    }

    fn rotate_right(&mut self) {
        self.send(MazeRunnerRequest::RotateRight90);

        self.position.orientation = self.position.orientation.shifted(RunnerSide::Right);
    }

    fn move_forward(&mut self) -> Result<(), String> {
        self.send(MazeRunnerRequest::MoveForward);

        self.position.cell = self.position.cell.neighbour(self.position.orientation)?;

        Ok(())
    }

    fn send(&mut self, request: MazeRunnerRequest) -> MazeRunnerResponse {
        self.api
            .send(request)
            .expect("Communication should be stable")
    }

    fn clear_maze(api: &mut MazeRunnerApi) -> Result<(), String> {
        for x in 0..16 {
            for y in 0..16 {
                api.send(MazeRunnerRequest::ClearCell { x, y })?;
            }
        }

        Ok(())
    }

    fn continue_attempts(&mut self) -> bool {
        println!("Press BTN1 to start attempt or BTN4 to end");

        loop {
            let response = self.send(MazeRunnerRequest::GetButtonsState);

            match response {
                MazeRunnerResponse::Buttons(buttons) => {
                    if buttons.contains(ButtonsState::Button4) {
                        return false;
                    }
                    if buttons.contains(ButtonsState::Button1) {
                        return true;
                    }
                }
                _ => {}
            }

            sleep(Duration::from_millis(1000));
        }
    }
}
