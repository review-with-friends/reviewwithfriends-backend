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

/// We expect phone numbers formatted like: "+1 7014910059"
///
/// Phone numbers must begin with +, contain a space, and contain
/// no non-ascii characters.
/// ```
/// assert!(validation::validate_phone("").is_err())
/// ```
/// ```
/// assert!(validation::validate_phone("1 7014910059").is_ok())
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
    if phone.len() != 12 {
        return Err("incorrect phone length".to_string());
    }

    if !phone.starts_with("1 ") {
        return Err("unsupported country phone".to_string());
    }

    if phone.chars().filter(|&c| c == ' ').count() != 1 {
        return Err("phone must have exactly 1 space".to_string());
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
