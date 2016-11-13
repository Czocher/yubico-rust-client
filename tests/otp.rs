extern crate yubico_rust_client;

#[cfg(test)]
mod otp {
    use yubico_rust_client::otp::Otp;

    #[test]
    fn length_too_short() {
        let otp = Otp::new(String::from("too short otp"));
        match otp.is_valid() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn length_too_long() {
        let otp = Otp::new(String::from("longlonglonglonglonglonglonglonglonglonglonglonglonglonglong"));
        match otp.is_valid() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn invalid_characters_low() {
        let otp = Otp::new(String::from("\n"));
        match otp.is_valid() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn invalid_characters_high() {
        let otp = Otp::new(String::from("ź"));
        match otp.is_valid() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn valid() {
        let otp = Otp::new(String::from("ccccccfgidngvuntrenietdcterjlukvflncdtrdgcbu"));
        match otp.is_valid() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}
