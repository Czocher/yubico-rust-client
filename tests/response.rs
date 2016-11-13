extern crate yubico_rust_client;

#[cfg(test)]
mod request {
    use yubico_rust_client::response::Response;
    use yubico_rust_client::response::Status;
    use std::str::FromStr;

    #[test]
    fn parse() {
        let body = "h=YSCznovGpIrdE/Hnqg8aqGc5+Xk=\nt=2016-07-30T21:47:44Z0497\nstatus=MISSING_PARAMETER";
        let response = Response::new(body).unwrap();
        assert_eq!(response.h, "YSCznovGpIrdE/Hnqg8aqGc5+Xk=")
    }


    #[test]
    fn create_status() {
        assert_eq!(Status::from_str("OK").unwrap(), Status::Ok);
        assert_eq!(Status::from_str("BAD_OTP").unwrap(), Status::BadOtp);
        assert_eq!(Status::from_str("REPLAYED_OTP").unwrap(), Status::ReplayedOtp);
        assert_eq!(Status::from_str("BAD_SIGNATURE").unwrap(), Status::BadSignature);
        assert_eq!(Status::from_str("MISSING_PARAMETER").unwrap(), Status::MissingParameter);
        assert_eq!(Status::from_str("NO_SUCH_CLIENT").unwrap(), Status::NoSuchClient);
        assert_eq!(Status::from_str("OPERATION_NOT_ALLOWED").unwrap(), Status::OperationNotAllowed);
        assert_eq!(Status::from_str("BACKEND_ERROR").unwrap(), Status::BackendError);
        assert_eq!(Status::from_str("NOT_ENOUGH_ANSWERS").unwrap(), Status::NotEnoughAnswers);
        assert_eq!(Status::from_str("REPLAYED_REQUEST").unwrap(), Status::ReplayedRequest);
    }
}
