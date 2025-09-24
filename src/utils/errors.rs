use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub description: &'static str,
}

impl ApiError {
    pub const fn new(code: &'static str, description: &'static str) -> Self {
        Self { code, description }
    }
}

// Common error constants
pub const INVALID_CREDENTIALS: ApiError = ApiError::new(
    "INVALID_CREDENTIALS",
    "Email or password is incorrect",
);

pub const UNAUTHORIZED: ApiError = ApiError::new(
    "UNAUTHORIZED",
    "Authorization token is missing or invalid",
);

pub const VALIDATION_FAILED: ApiError = ApiError::new(
    "VALIDATION_FAILED",
    "One or more fields failed validation",
);

pub const USER_ALREADY_EXISTS: ApiError = ApiError::new(
    "USER_ALREADY_EXISTS",
    "A user with these credentials already exists",
);

pub const INTERNAL_SERVER_ERROR: ApiError = ApiError::new(
    "INTERNAL_SERVER_ERROR",
    "An unexpected error occurred",
);


