//! Input Validation (dummy, extend with validator crate)

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

// Example real-world: use validator crate for struct/field-level validation
