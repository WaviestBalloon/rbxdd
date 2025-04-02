use std::fmt::Debug;

#[derive(Debug)]
pub struct NoAccessError {
	pub http_code: u32,
	pub http_body: String,
	pub message: String
}

#[derive(Debug)]
pub struct NotFoundError {
	pub http_code: u32,
	pub http_body: String,
	pub message: String
}

#[derive(Debug)]
pub struct UnexpectedStatusCode {
	pub http_code: u32,
	pub http_body: String,
	pub message: String
}

#[derive(Debug)]
pub enum GetLatestVersionErrors {
	NoAccessError(NoAccessError),
	UnexpectedStatusCode(UnexpectedStatusCode)
}

#[derive(Debug)]
pub enum GetManifestErrors {
	NotFoundError(NotFoundError),
	UnexpectedStatusCode(UnexpectedStatusCode)
}

