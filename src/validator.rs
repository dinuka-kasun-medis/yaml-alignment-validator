use crate::models::ValidationResult;

pub fn validate_yaml_alignment(file_content: &str) -> ValidationResult {
    let lines: Vec<&str> = file_content.lines().collect();

    for (index, line) in lines.iter().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        let current_indent = line.chars().take_while(|c| c.is_whitespace()).count();

        if current_indent % 2 != 0 {
            return ValidationResult {
                valid: false,
                message: format!("Indentation error on line {}: Indentation should be a multiple of 2", index + 1),
            };
        }
    }

    ValidationResult {
        valid: true,
        message: "YAML file is properly formatted.".to_string(),
    }
}
