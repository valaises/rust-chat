
pub static BACKEND_URL: &str = "https://llmtools.valerii.cc/v1";
pub const API_KEY: &str = match option_env!("REMOTE_VALERIICC_API_KEY") {
    Some(endpoint) => endpoint,
    None => "http://default-endpoint.com",
};
