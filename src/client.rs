extern crate hyper;

use self::hyper::client;
use std::thread;
use std::io::Read;
use request::Request;
use self::hyper::status::StatusCode;
use response::Response;

pub type ClientId = u32;
pub type ClientKey = String;

static WSAPI_URLS: &'static [&'static str] = &["https://api.yubico.com/wsapi/2.0/verify",
                                               "https://api2.yubico.com/wsapi/2.0/verify",
                                               "https://api3.yubico.com/wsapi/2.0/verify",
                                               "https://api4.yubico.com/wsapi/2.0/verify",
                                               "https://api5.yubico.com/wsapi/2.0/verify"];

pub struct Client {
    client_id: ClientId,
    client_key: ClientKey,
}

impl Client {
    pub fn new(client_id: ClientId, client_key: ClientKey) -> Client {
        Client {
            client_id: client_id,
            client_key: client_key,
        }
    }

    pub fn verify(&self, request: Request) -> Result<(), &'static str> {
        try!(request.otp.is_valid());
        let query_str = self.prepare_query_string(&request);
        let result = self.query_webservices(&request);

        Ok(())
    }

    fn prepare_query_string(&self, request: &Request) -> String {
        String::new()
    }

    fn query_webservices(&self, request: &Request) -> Result<(), &'static str> {
        let mut threads = Vec::new();

        for url in WSAPI_URLS {
            let thread = thread::spawn(move || {
                let client = client::Client::new();
                // Better not unwrap
                let mut response = client.get(*url).send().unwrap();
                if response.status != StatusCode::Ok {
                    Err(format!("Invalid response: {}", response.status))
                } else {
                    let mut body = String::new();
                    response.read_to_string(&mut body);
                    Ok(Response::new(&*body))
                }
            });
            threads.push(thread);
        }

        for res in threads {
            println!("{:?}", res.join().unwrap());
        }

        // TODO: Iterate over threads and gather the responses
        Ok(())
    }
}
