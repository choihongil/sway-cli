use std::os::unix::net::UnixStream;
use std::{env, io, io::Read, io::Write};

const ENV_NAME: &str = "SWAYSOCK";
const MAGIC_STRING: &str = "i3-ipc";

pub enum MessageType<'a> {
    RunCommand(&'a str),
    GetWorkspaces,
    Subscribe,
    GetOutputs,
    GetTree,
    GetMarks,
    GetBarConfig,
    GetVersion,
    GetBindingModes,
    GetConfig,
    SendTick,
    Sync,
    GetBindingState,
    GetInputs,
    GetSeats,
}

impl<'a> MessageType<'a> {
    pub fn execute(&self) -> Result<Vec<u8>, crate::error::Error> {
        let mut stream = UnixStream::connect(path()?)?;
        let msg = match self {
            MessageType::RunCommand(command) => message(command, 0),
            MessageType::GetWorkspaces => message("", 1),
            MessageType::Subscribe => message("", 2),
            MessageType::GetOutputs => message("", 3),
            MessageType::GetTree => message("", 4),
            MessageType::GetMarks => message("", 5),
            MessageType::GetBarConfig => message("", 6),
            MessageType::GetVersion => message("", 7),
            MessageType::GetBindingModes => message("", 8),
            MessageType::GetConfig => message("", 9),
            MessageType::SendTick => message("", 10),
            MessageType::Sync => message("", 11),
            MessageType::GetBindingState => message("", 12),
            MessageType::GetInputs => message("", 100),
            MessageType::GetSeats => message("", 101),
        };
        stream.write_all(&msg)?;
        Ok(reply(stream)?)
    }
}

fn path() -> Result<String, env::VarError> {
    env::var(ENV_NAME)
}

fn message(payload: &str, payload_type: u32) -> Vec<u8> {
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(MAGIC_STRING.as_bytes().iter());

    let payload_len = payload.len() as u32;
    msg.extend(payload_len.to_ne_bytes().iter());
    msg.extend(payload_type.to_ne_bytes().iter());
    msg.extend(payload.as_bytes().iter());

    return msg;
}

fn reply(mut stream: UnixStream) -> io::Result<Vec<u8>> {
    let mut magic_string_buf = [0_u8; 6];
    stream.read_exact(&mut magic_string_buf)?;
    let mut payload_len_buf = [0_u8; 4];
    stream.read_exact(&mut payload_len_buf)?;
    let payload_len = u32::from_ne_bytes(payload_len_buf);
    let mut payload_type_buf = [0_u8; 4];
    stream.read_exact(&mut payload_type_buf)?;

    let mut payload = vec![0_u8; payload_len as usize];
    stream.read_exact(&mut payload)?;
    Ok(payload)
}
