extern crate time;
use self::time::Tm;
use self::time::strptime;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    BadOtp,
    ReplayedOtp,
    BadSignature,
    MissingParameter,
    NoSuchClient,
    OperationNotAllowed,
    BackendError,
    NotEnoughAnswers,
    ReplayedRequest,
}

impl FromStr for Status {
 type Err = String;

    fn from_str(s: &str) -> Result<Status, String> {
        match s {
            "OK" => Ok(Status::Ok),
            "BAD_OTP" => Ok(Status::BadOtp),
            "REPLAYED_OTP" => Ok(Status::ReplayedOtp),
            "BAD_SIGNATURE" => Ok(Status::BadSignature),
            "MISSING_PARAMETER" => Ok(Status::MissingParameter),
            "NO_SUCH_CLIENT" => Ok(Status::NoSuchClient),
            "OPERATION_NOT_ALLOWED" => Ok(Status::OperationNotAllowed),
            "BACKEND_ERROR" => Ok(Status::BackendError),
            "NOT_ENOUGH_ANSWERS" => Ok(Status::NotEnoughAnswers),
            "REPLAYED_REQUEST" => Ok(Status::ReplayedRequest),
            _ => Err(format!("Unknown status code {}", s))
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub otp: Option<String>,
    pub nonce: Option<String>,
    pub h: String,
    pub t: Tm,
    pub status: Option<Status>,
    pub timestamp: Option<String>,
    pub session_counter: Option<String>,
    pub session_use: Option<String>,
    pub sl: Option<u8>,
}

impl Response {
    pub fn new(s: &str) -> Result<Response, ()> {
        Response::from_str(s)
    }
}

impl FromStr for Response {
    type Err = ();

    fn from_str(s: &str) -> Result<Response, ()> {
        let mut response = Response {
            otp: None,
            nonce: None,
            h: String::new(),
            t: time::empty_tm(),
            status: None,
            timestamp: None,
            session_counter: None,
            session_use: None,
            sl: None,
        };

        for part in s.lines().filter(|kv| kv.contains("=")) {
            // Can unwrap because we're sure there is an equals sign
            let split = part.splitn(2, '=').collect::<Vec<_>>();
            let (key, value) = (split[0], split[1]);

            match key {
                "otp" => response.otp = Some(value.to_owned()),
                "nonce" => response.nonce = Some(value.to_owned()),
                "h" => response.h = value.to_owned(),
                "t" => response.t = strptime(value, "%Y-%m-%dT%H:%M:%SZ%f").unwrap(),
                "status" => response.status = Status::from_str(value).ok(),
                "timestamp" => response.timestamp = Some(value.to_owned()),
                "sessioncounter" => response.session_counter = Some(value.to_owned()),
                "sessionuse" => response.session_use = Some(value.to_owned()),
                "sl" => response.sl = value.parse().ok(),
                _ => {}
            }
        }

        Ok(response)
    }
}
