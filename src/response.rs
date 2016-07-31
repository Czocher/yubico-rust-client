extern crate time;
use self::time::Tm;
use std::str::FromStr;

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

pub struct Response {
    pub otp: String,
    pub nonce: String,
    pub h: String,
    pub t: Tm,
    pub status: Option<Status>,
    pub timestamp: String,
    pub session_counter: String,
    pub session_use: String,
    pub sl: u8
}

impl Response {
    pub fn new(s: &str) -> Result<Response, ()> {
        Response::from_str(s)
    }
}

impl FromStr for Response {

    type Err = ();

    fn from_str(s: &str) -> Result<Response, ()> {
        let mut response = Response{
            otp: String::new(),
            nonce: String::new(),
            h: String::new(),
            t: time::empty_tm(),
            status: None,
            timestamp: String::new(),
            session_counter: String::new(),
            session_use: String::new(),
            sl: 0,
        };

        for part in s.split("\n").filter(|kv| kv.contains("=")) {
            // Can unwrap because we're sure there is an equals sign
            let (key, value) = part.split_at(part.find("=").unwrap());
            match key {
                "otp" => response.otp = value.to_owned(),
                "nonce" => response.nonce = value.to_owned(),
                "h" => response.h = value.to_owned(),
                "t" => {},
                "status" => {},
                "timestamp" => {},
                "sessioncounter" => {},
                "sessionuse" => {},
                "sl" => {},
                _ => {},
            }
        }

        Ok(response)
    }

}
