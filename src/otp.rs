pub struct Otp {
    otp: String
}

impl Otp {
    pub fn new(string : String) -> Otp {
        Otp{otp: string, }
    }

    pub fn is_valid(&self) -> Result<(), &'static str> {
        let otp_min_len = 32;
        let otp_max_len = 48;
        let len = self.otp.len();

        if !(otp_min_len <= len && len <= otp_max_len) {
            Err("Invalid OTP length")
        } else if !self.otp.chars().all(|c| Otp::is_ascii_printable(c)) {
            Err("Invalid OTP content")
        } else {
            Ok(())
        }
    }

    fn is_ascii_printable(c: char) -> bool {
        let i = c as u32;
        i >= 0x20 && i <= 0x7E
    }
}
