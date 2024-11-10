use std::fmt::{Debug, Display};

pub struct NoAccessError {
	pub http_code: u32,
	pub http_body: String,
	pub message: String
}
impl Display for NoAccessError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NoAccessError: {}", self.message)
	}
}
impl Debug for NoAccessError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NoAccessError {{ http_code: {}, http_body: {}, message: {} }}", self.http_code, self.http_body, self.message)
	}
}

pub struct NotFoundError {
	pub http_code: u32,
	pub http_body: String,
	pub message: String
}
impl Display for NotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NotFoundError: {}", self.message)
	}
}
impl Debug for NotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NotFoundError {{ http_code: {}, http_body: {}, message: {} }}", self.http_code, self.http_body, self.message)
	}
}
