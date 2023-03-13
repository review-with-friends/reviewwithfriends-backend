use rand::Rng;

/// Generates a new 9 digit auth code authenticated via phone.
/// I believe this generation is OK. The endpoint for validating
/// the code restricts the code to a limited lifetime and we limit
/// the amount of code attempts.
pub fn get_new_auth_code() -> String {
    let mut rng = rand::thread_rng();
    let mut code = String::from("");

    for _ in 0..9 {
        let num = rng.gen_range(0..9);
        code.push_str(&num.to_string());
    }

    return code;
}
