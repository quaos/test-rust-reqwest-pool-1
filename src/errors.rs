#[derive(Debug)]
pub enum AppError {
  HttpClientError(reqwest::Error),
  OtherError(String),  
}

impl From<reqwest::Error> for AppError {
  fn from(src: reqwest::Error) -> Self {
    Self::HttpClientError(src)
  }
}

impl From<Box<dyn std::error::Error>> for AppError {
  fn from(src: Box<dyn std::error::Error>) -> Self {
    Self::other(&src)
  }
}

impl AppError {
  pub fn other<T: std::fmt::Debug>(src: &T) -> Self {
    Self::OtherError(format!("{:?}", src))
  }
}
