pub mod abi;

use abi::command_request::RequestData;
use abi::{CommandRequest, CommandResponse, SignRequest};

impl CommandRequest {
    pub fn new_sign_request(
        id: u32,
        seed_id: u32,
        algo: i32,
        password: String,
        data: String,
        derivation_path: String,
        port_name: String,
    ) -> Self {
        Self {
            request_id: id,
            request_data: Some(RequestData::SignRequest(SignRequest {
                seed_id,
                algo,
                password,
                data,
                derivation_path,
                port_name,
            })),
        }
    }
}

impl CommandResponse {
    pub fn success(id: u32, response: String) -> Self {
        Self {
            response_id: id,
            status: 200,
            response,
            error_message: "".to_string(),
        }
    }

    pub fn error(id: u32, error: String) -> Self {
        Self {
            response_id: id,
            status: 500,
            response: "".to_string(),
            error_message: error,
        }
    }
}
