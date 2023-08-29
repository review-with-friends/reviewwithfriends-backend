use std::io::Cursor;

use base64::engine::general_purpose;
use jpeg_decoder::Decoder;
use validator::Validate;

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

/// We expect emails formatted like: "support@spacedoglabs.com"
///
/// ```
/// assert!(validation::validate_email("").is_err());
/// assert!(validation::validate_email("support@spacedoglabs.com").is_ok());
/// ```
pub fn validate_email(email: &str) -> Result<(), String> {
    let email_validator = EmailValidator {
        email: email.to_string(),
    };

    match email_validator.validate() {
        Ok(_) => Ok(()),
        Err(_) => Err("invalid email".to_string()),
    }
}

#[derive(Validate)]
struct EmailValidator {
    #[validate(email)]
    email: String,
}

/// Validates the profile pic and returns the size.
///
/// Tuple return is (metadata.width, metadata.height)
pub fn validate_profile_pic(bytes: &[u8]) -> Result<(u16, u16), String> {
    if bytes.len() > 500_000 {
        return Err("pic too large".to_string());
    }

    let mut decoder = Decoder::new(bytes);
    let decode_res = decoder.decode();

    match decode_res {
        Ok(_) => {
            if let Some(metadata) = decoder.info() {
                if metadata.height > 1024 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 1024 {
                    return Err("image too wide".to_string());
                }

                return Ok((metadata.width, metadata.height));
            } else {
                return Err("metadata unreadable".to_string());
            }
        }
        Err(_) => return Err("unable to decode pic".to_string()),
    }
}

/// Validates the review pic and returns the size.
///
/// Tuple return is (metadata.width, metadata.height)
pub fn validate_review_pic(bytes: &[u8]) -> Result<(u16, u16), String> {
    if bytes.len() > 3_250_000 {
        return Err("pic too large".to_string());
    }

    let mut decoder = Decoder::new(bytes);
    let decode_res = decoder.read_info();

    match decode_res {
        Ok(_) => {
            if let Some(metadata) = decoder.info() {
                if metadata.height > 4032 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 3024 {
                    return Err("image too wide".to_string());
                }

                let ratio_p = metadata.height / metadata.width;
                let ratio_q = metadata.width / metadata.height;

                if ratio_p >= 4 || ratio_q >= 4 {
                    return Err("aspect ratio of image isnt allowed".to_string());
                }

                return Ok((metadata.width, metadata.height));
            } else {
                return Err("metadata unreadable".to_string());
            }
        }
        Err(_) => return Err("unable to decode pic".to_string()),
    }
}

/// Validates the review pic and returns the size.
///
/// Tuple return is (metadata.width, metadata.height)
pub fn validate_review_pic_b64(b64: &str) -> Result<(u16, u16), String> {
    let mut wrapped_reader = Cursor::new(b64);
    let b64_decoder =
        base64::read::DecoderReader::new(&mut wrapped_reader, &general_purpose::STANDARD);

    let mut decoder = Decoder::new(b64_decoder);
    let decode_res = decoder.read_info();

    match decode_res {
        Ok(_) => {
            if let Some(metadata) = decoder.info() {
                if metadata.height > 4032 {
                    return Err("image too tall".to_string());
                }

                if metadata.width > 3024 {
                    return Err("image too wide".to_string());
                }

                let ratio_p = metadata.height / metadata.width;
                let ratio_q = metadata.width / metadata.height;

                if ratio_p >= 4 || ratio_q >= 4 {
                    return Err("aspect ratio of image isnt allowed".to_string());
                }

                return Ok((metadata.width, metadata.height));
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
/// assert!(validation::validate_display_name("TES").is_ok());
/// assert!(validation::validate_display_name("💜").is_err());
/// assert!(validation::validate_display_name(&"💜".repeat(27)).is_err());
/// ```
pub fn validate_display_name(text: &str) -> Result<(), String> {
    if text.chars().count() > 26 {
        return Err("display name too long - max 26 chars".to_string());
    }

    if text.chars().count() < 3 {
        return Err("display name too short - min 3 chars".to_string());
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
/// assert!(validation::validate_location_name("").is_err());
/// assert!(validation::validate_location_name("💜").is_ok());
/// assert!(validation::validate_location_name(&"💜".repeat(24)).is_ok());
/// assert!(validation::validate_location_name(&"💜".repeat(97)).is_err());
/// ```
pub fn validate_location_name(text: &str) -> Result<(), String> {
    if text.chars().count() > 96 {
        return Err("location_name too long - max 96 chars".to_string());
    }

    if text.chars().count() < 1 {
        return Err("location_name too short - min 1 chars".to_string());
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

/// ```
/// assert!(validation::validate_review_category("bank").is_ok());
/// assert!(validation::validate_review_category("").is_ok());
/// assert!(validation::validate_review_category("💜").is_err());
/// ```
pub fn validate_review_category(category: &str) -> Result<(), String> {
    if !VALID_CATEGORIES.contains(&category) {
        return Err("not a valid category".to_string());
    }

    return Ok(());
}

const VALID_CATEGORIES: [&str; 40] = [
    "airport",
    "amusementPark",
    "aquarium",
    "atm",
    "bakery",
    "bank",
    "beach",
    "brewery",
    "cafe",
    "campground",
    "carRental",
    "evCharger",
    "fireStation",
    "fitnessCenter",
    "foodMarket",
    "gasStation",
    "hospital",
    "hotel",
    "laundry",
    "marina",
    "movieTheater",
    "museum",
    "nationalPark",
    "nightlife",
    "park",
    "parking",
    "pharmacy",
    "police",
    "postOffice",
    "publicTransport",
    "restaurant",
    "restroom",
    "school",
    "stadium",
    "store",
    "theater",
    "university",
    "winery",
    "zoo",
    "",
];
