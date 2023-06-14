mod pb;

use pb::abi::command_request::RequestData;
use pb::abi::*;
use prost::Message;
use std::io::Cursor;
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

use rcc_signer::{Signer, SigningAlgorithm};

pub struct RCC;

impl RCC {
    pub fn new() -> RCC {
        RCC {}
    }

    fn excute(&mut self, command: CommandRequest) -> CommandResponse {
        match command.request_data {
            Some(RequestData::SignRequest(params)) => {
                // let signer: Signer = Signer::new_with_mini();
                let sign_data = match hex::decode(&params.data) {
                    Ok(data) => data,
                    Err(_) => {
                        return CommandResponse::error(
                            command.request_id,
                            "sign data decode failed".to_string(),
                        )
                    }
                };

                let algo = match params.algo {
                    0 => SigningAlgorithm::Secp256k1,
                    _ => {
                        return CommandResponse::error(
                            command.request_id,
                            "Algo is not supported".to_string(),
                        )
                    }
                };

                return CommandResponse::success(command.request_id, hex::encode(sign_data))

            }
            None => {
                CommandResponse::error(command.request_id, "Request is not supported".to_string())
            }
        }
    }

    fn deserialize_to_string(&self, command_respones: CommandResponse) -> String {
        let mut buf = Vec::new();
        buf.reserve(command_respones.encoded_len());

        command_respones.encode(&mut buf);
        hex::encode(buf)
    }

    pub fn process_command(&mut self, command_string: String) -> String {
        let buf: Vec<u8>;
        match hex::decode(&command_string) {
            Ok(data) => buf = data,
            Err(_) => {
                let cp = CommandResponse::error(
                    0,
                    "invaild command, decode protobuf hex error".to_string(),
                );
                return self.deserialize_to_string(cp);
            }
        }

        let cmd: CommandRequest;

        match CommandRequest::decode(&mut Cursor::new(&buf)) {
            Ok(command) => cmd = command,
            Err(_) => {
                let cp =
                    CommandResponse::error(0, "invaild command, decode protobuf error".to_string());
                return self.deserialize_to_string(cp);
            }
        }
        let cp = self.excute(cmd);
        self.deserialize_to_string(cp)
    }
}


#[no_mangle]
pub extern fn process_command(command: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(command) };
    let recipient = match c_str.to_str() {
        Err(_) => "error".to_string(),
        Ok(string) => string.to_string(),
    };

    let mut rcc = RCC::new();
    let resp = rcc.process_command(recipient);

    CString::new(resp).unwrap().into_raw()
}

#[no_mangle]
pub extern fn fee_result(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}