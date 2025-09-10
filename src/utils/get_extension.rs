use std::env::consts;

/// Return the correct file extension for the current OS, or an error if the OS is unsupported.
pub fn new() -> Result<&'static str, Box<dyn std::error::Error>>{
    return match consts::OS {
        "windows" => Ok(".exe"),
        "linux" => Ok(""),
        "macos" => Ok(""),
        "android" => Ok(""),
        _ => Err("Unsupported OS".into()),
    };
}
