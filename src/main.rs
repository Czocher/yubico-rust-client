extern crate yubico_rust_client;
use yubico_rust_client::otp::Otp;
use yubico_rust_client::client::Client;
use yubico_rust_client::request::Request;

fn main() {
    let c = Client::new(21, String::from("test"));
    let r = Request::new(Otp::new(String::from("ccccccfgidngnbdvtrhhhfnuldjurunlvgutndfdcuvl")));
    println!("{:?}", c.verify(r));

}
