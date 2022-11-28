use jpeg_decoder::Decoder;

/// All issued codes are exactly 9 in length and contain
/// no non-ascii characters.
///
/// ```
/// assert!(validation::validate_code("000000000").is_ok())
/// ```
/// ```
/// assert!(validation::validate_code("123456789").is_ok())
/// ```
/// ```
/// assert!(validation::validate_code("12345678").is_err())
/// ```
/// ```
/// assert!(validation::validate_code("12345678").is_err())
/// ```
/// ```
/// assert!(validation::validate_code("12345678a").is_err())
/// ```
/// ```
/// assert!(validation::validate_code("a23456789").is_err())
/// ```
/// ```
/// assert!(validation::validate_code("").is_err())
/// ```
pub fn validate_code(code: &str) -> Result<(), String> {
    if code.len() != 9 {
        return Err("incorrect code length".to_string());
    }

    if !code.chars().all(|c| c.is_ascii_digit()) {
        return Err("not all digits".to_string());
    }

    Ok(())
}

/// We expect phone numbers formatted like: "17014910059"
///
/// Phone numbers are expected in the E.164 Format with digits only.
///
/// We will preprend the '+' when making an auth call.
/// ```
/// assert!(validation::validate_phone("").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("17014910059").is_ok())
/// ```
/// ```
/// assert!(validation::validate_phone("1 7014910059").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("44 445566").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("+44 445").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("+++++").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("+44445434434").is_err())
/// ```
pub fn validate_phone(phone: &str) -> Result<(), String> {
    if phone.len() != 11 {
        return Err("incorrect phone length".to_string());
    }

    if !phone.starts_with("1") {
        return Err("unsupported country code".to_string());
    }

    if !phone.chars().all(|p| p.is_ascii_digit()) {
        return Err("phone not all digits".to_string());
    }

    Ok(())
}

pub fn validate_profile_pic(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() > 250_000 {
        return Err("pic too large".to_string());
    }

    let mut decoder = Decoder::new(bytes);
    let decode_res = decoder.decode();

    match decode_res {
        Ok(_) => {
            if let Some(metadata) = decoder.info() {
                if metadata.height > 500 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 500 {
                    return Err("image too wide".to_string());
                }

                return Ok(());
            } else {
                return Err("metadata unreadable".to_string());
            }
        }
        Err(_) => return Err("unable to decode pic".to_string()),
    }
}

pub fn validate_review_pic(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() > 2_250_000 {
        return Err("pic too large".to_string());
    }

    let mut decoder = Decoder::new(bytes);
    let decode_res = decoder.decode();

    match decode_res {
        Ok(_) => {
            if let Some(metadata) = decoder.info() {
                if metadata.height > 3000 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 4000 {
                    return Err("image too wide".to_string());
                }

                return Ok(());
            } else {
                return Err("metadata unreadable".to_string());
            }
        }
        Err(_) => return Err("unable to decode pic".to_string()),
    }
}

/// ```
/// assert!(validation::validate_reply_text("Test :D").is_ok())
/// ```
/// ```
/// assert!(validation::validate_reply_text(&"1".repeat(451)).is_err())
/// ```
/// ```
/// assert!(validation::validate_reply_text(&"1".repeat(450)).is_ok())
/// ```
pub fn validate_reply_text(text: &str) -> Result<(), String> {
    if text.len() > 450 {
        return Err("text too long".to_string());
    }

    return Ok(());
}
