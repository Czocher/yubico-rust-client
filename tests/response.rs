extern crate yubico_otp;

#[cfg(test)]
mod request {
    use yubico_otp::response::Response;

    #[test]
    fn parse() {
        let body = String::from("h=YSCznovGpIrdE/Hnqg8aqGc5+Xk=\nt=2016-07-30T21:47:44Z0497\nstatus=MISSING_PARAMETER");
        let response = Response::new(body).unwrap();
        assert_eq!(response.h, "YSCznovGpIrdE/Hnqg8aqGc5+Xk=")
    }

}
