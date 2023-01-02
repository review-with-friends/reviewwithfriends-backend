use jpeg_decoder::Decoder;

/// All issued codes are exactly 9 in length and contain
/// no non-ascii characters.
///
/// ```
/// assert!(validation::validate_code("000000000").is_ok());
/// assert!(validation::validate_code("123456789").is_ok());
/// assert!(validation::validate_code("12345678").is_err());
/// assert!(validation::validate_code("12345678").is_err());
/// assert!(validation::validate_code("12345678a").is_err());
/// assert!(validation::validate_code("a23456789").is_err());
/// assert!(validation::validate_code("").is_err());
/// ```
pub fn validate_code(code: &str) -> Result<(), String> {
    if code.chars().count() != 9 {
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
/// assert!(validation::validate_phone("").is_err());
/// assert!(validation::validate_phone("17014910059").is_ok());
/// assert!(validation::validate_phone("1 7014910059").is_err());
/// assert!(validation::validate_phone("44 445566").is_err());
/// assert!(validation::validate_phone("+44 445").is_err());
/// assert!(validation::validate_phone("+++++").is_err());
/// assert!(validation::validate_phone("+44445434434").is_err());
/// ```
pub fn validate_phone(phone: &str) -> Result<(), String> {
    if phone.chars().count() != 11 {
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
                if metadata.height > 512 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 512 {
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
                if metadata.height > 4032 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 3024 {
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
/// assert!(validation::validate_reply_text("Test :D").is_ok());
/// assert!(validation::validate_reply_text(&"1".repeat(451)).is_err());
/// assert!(validation::validate_reply_text(&"1".repeat(450)).is_ok());
/// ```
pub fn validate_reply_text(text: &str) -> Result<(), String> {
    if text.chars().count() > 450 {
        return Err("text too long".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_reply_text("Test :D").is_ok());
/// assert!(validation::validate_reply_text(&"1".repeat(451)).is_err());
/// assert!(validation::validate_reply_text(&"1".repeat(450)).is_ok());
/// ```
pub fn validate_review_text(text: &str) -> Result<(), String> {
    if text.chars().count() > 450 {
        return Err("text too long".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_name("Test :D").is_err());
/// assert!(validation::validate_name("test").is_ok());
/// assert!(validation::validate_name("1234").is_ok());
/// assert!(validation::validate_name("1234💜").is_err());
/// assert!(validation::validate_name("💜").is_err());
/// ```
pub fn validate_name(text: &str) -> Result<(), String> {
    if text.chars().count() > 26 {
        return Err("name too long - max 26 chars".to_string());
    }

    if text.chars().count() < 4 {
        return Err("name too short - min 4 chars".to_string());
    }

    if !text.is_ascii() {
        return Err("0-9 and a-z only".to_string());
    }

    if !text
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
    {
        return Err("not all are digits or char".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_display_name("Test :D").is_ok());
/// assert!(validation::validate_display_name("TES💜T").is_ok());
/// assert!(validation::validate_display_name("TES💜T  2321").is_ok());
/// assert!(validation::validate_display_name("TES").is_err());
/// assert!(validation::validate_display_name("💜").is_err());
/// assert!(validation::validate_display_name(&"💜".repeat(27)).is_err());
/// ```
pub fn validate_display_name(text: &str) -> Result<(), String> {
    println!("{}", text.len());
    if text.chars().count() > 26 {
        return Err(
            "display name too long - max 26 chars (emojis count for 2-4 chars)".to_string(),
        );
    }

    if text.chars().count() < 4 {
        return Err("display name too short - min 4 chars".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_latitude(10.0).is_ok());
/// assert!(validation::validate_latitude(-10.0).is_ok());
/// assert!(validation::validate_latitude(91.0).is_err());
/// assert!(validation::validate_latitude(-91.0).is_err());
/// ```
pub fn validate_latitude(latitude: f64) -> Result<(), String> {
    if latitude > 90.0 {
        return Err("latitude out of range -90:90".to_string());
    }

    if latitude < -90.0 {
        return Err("latitude out of range -90:90".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_longitude(10.0).is_ok());
/// assert!(validation::validate_longitude(-10.0).is_ok());
/// assert!(validation::validate_longitude(181.0).is_err());
/// assert!(validation::validate_longitude(-181.0).is_err());
/// ```
pub fn validate_longitude(latitude: f64) -> Result<(), String> {
    if latitude > 180.0 {
        return Err("longitude out of range -180:180".to_string());
    }

    if latitude < -180.0 {
        return Err("longitude out of range -180:180".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_location_name("Test :D").is_ok());
/// assert!(validation::validate_location_name("💜").is_err());
/// assert!(validation::validate_location_name(&"💜".repeat(24)).is_ok());
/// assert!(validation::validate_location_name(&"💜".repeat(97)).is_err());
/// ```
pub fn validate_location_name(text: &str) -> Result<(), String> {
    if text.chars().count() > 96 {
        return Err("location_name too long - max 96 chars".to_string());
    }

    if text.chars().count() < 4 {
        return Err("location_name too short - min 4 chars".to_string());
    }

    return Ok(());
}

/// ```
/// assert!(validation::validate_stars(0).is_ok());
/// assert!(validation::validate_stars(5).is_ok());
/// assert!(validation::validate_stars(6).is_err());
/// ```
pub fn validate_stars(stars: u8) -> Result<(), String> {
    if stars > 5 {
        return Err("too many stars".to_string());
    }

    return Ok(());
}
