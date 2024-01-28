use std::fmt::Debug;

/// Bounds that the application error type must satisfy.
pub trait AppError: Debug + std::error::Error + Send + Sync + Unpin + 'static {}

impl<T> AppError for T where T: Debug + std::error::Error + Send + Sync + Unpin + 'static {}
