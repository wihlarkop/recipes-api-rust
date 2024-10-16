use validator::ValidationError;

pub fn validate_difficulty(difficulty: &String) -> Result<(), ValidationError> {
    let valid_levels = vec!["easy", "medium", "hard"];
    if !valid_levels.contains(&difficulty.to_lowercase().as_str()) {
        return Err(ValidationError::new("invalid_difficulty"));
    }
    Ok(())
}
