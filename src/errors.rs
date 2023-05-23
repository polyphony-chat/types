#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "sqlx")]
    #[error("SQLX error: {0}")]
    SQLX(#[from] sqlx::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    
    #[error(transparent)]
    FieldFormat(#[from] FieldFormatError)
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum FieldFormatError {
    #[error("Password must be between 1 and 72 characters.")]
    PasswordError,
    #[error("Username must be between 2 and 32 characters.")]
    UsernameError,
    #[error("Consent must be 'true' to register.")]
    ConsentError,
    #[error("The provided email address is in an invalid format.")]
    EmailError,
}
