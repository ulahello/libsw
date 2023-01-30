// TODO: std::time::SystemTime support?

#[cfg(feature = "std_instant")]
mod std_instant;

#[cfg(feature = "tokio")]
mod tokio;
