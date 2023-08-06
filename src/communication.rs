use bitflags::bitflags;
use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

const SOCKET: &str = "/tmp/micromouse_simulator_socket";

bitflags! {
    #[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct ButtonsState: u8 {
        const Reset = 0b00000001;
        const Button1 = 0b00000010;
        const Button2 = 0b00000100;
        const Button3 = 0b00001000;
        const Button4 = 0b00010000;
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct CellState: u8 {
        const NorthWall = 0b00000001;
        const EastWall = 0b00000010;
        const SouthWall = 0b00000100;
        const WestWall = 0b00001000;
        const Visited = 0b00010000;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MazeRunnerRequest {
    Initialize,
    MoveForward,
    RotateRight90,
    RotateLeft90,
    GetWallFront,
    GetWallRight,
    GetWallLeft,
    GetButtonsState,
    UpdateCellState {
        x: usize,
        y: usize,
        state: CellState,
    },
    ClearCell {
        x: usize,
        y: usize,
    },
    UpdateCellValue {
        x: usize,
        y: usize,
        value: i32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MazeRunnerResponse {
    Ack,
    Error,
    WallDetected(bool),
    Buttons(ButtonsState),
}

pub struct MazeRunnerApi {
    stream: UnixStream,
}

impl MazeRunnerApi {
    pub fn new() -> Result<Self, String> {
        let stream =
            UnixStream::connect(SOCKET).map_err(|e| format!("Could not create stream: {e}"))?;

        Ok(Self { stream })
    }

    pub fn send(&mut self, request: MazeRunnerRequest) -> Result<MazeRunnerResponse, String> {
        let request: Vec<u8> =
            to_stdvec(&request).map_err(|e| format!("Could not serialize request: {e}"))?;

        self.stream
            .write_all(request.as_slice())
            .map_err(|e| format!("Could not send request: {e}"))?;

        self.stream
            .flush()
            .map_err(|e| format!("Could not flush the stream: {e}"))?;

        let mut buffer = [0; 100];

        let n = self
            .stream
            .read(&mut buffer[..])
            .map_err(|e| format!("Could not recieve response: {e}"))?;

        if n == 0 {
            return Err(format!("Server ended connection"));
        }

        from_bytes(&buffer).map_err(|e| format!("Failed to deserialize response: {e}"))
    }
}
