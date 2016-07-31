extern crate yubico_otp;

#[cfg(test)]
mod request {
    use yubico_otp::request::Request;
    use yubico_otp::otp::Otp;

    #[test]
    fn create() {
        Request::new(Otp::new(String::from("bla")));
    }

}
