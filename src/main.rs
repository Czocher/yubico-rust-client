extern crate yubico_otp;
use yubico_otp::otp::Otp;
use yubico_otp::client::Client;
use yubico_otp::request::Request;

fn main() {
    let c = Client::new(21, String::from("test"));
    let r = Request::new(Otp::new(String::from("ccccccfgidngnbdvtrhhhfnuldjurunlvgutndfdcuvl")));
    println!("{:?}", c.verify(r));

}
