extern crate uuid;

use self::uuid::Uuid;
use otp::Otp;

pub type Nonce = String;

pub enum Syncing {
    FAST,
    SECURE
}

pub struct Request {
    pub otp: Otp,
    pub h: Option<String>,
    pub timestamp: bool,
    nonce: Nonce,
    pub sl: Option<Syncing>,
    pub timeout: Option<u32>,
}

impl Request {
    pub fn new(otp: Otp) -> Request {
        Request{
            nonce: Uuid::new_v4().simple().to_string(),
            otp: otp,
            sl: None,
            timeout: None,
            timestamp: true,
            h: None
        }
    }
}
