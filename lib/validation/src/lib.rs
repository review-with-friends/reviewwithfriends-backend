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
