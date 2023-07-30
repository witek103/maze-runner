use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

const SOCKET: &str = "/tmp/micromouse_simulator_socket";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MazeRunnerRequest {
    Initialize,
    MoveForward,
    RotateRight90,
    RotateLeft90,
    GetWallFront,
    GetWallRight,
    GetWallLeft,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MazeRunnerResponse {
    Ack,
    Error,
    WallDetected(bool),
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
