
pub static BACKEND_URL: &str = "https://llmproxy.xi.valerii.cc/v1";
pub const API_KEY: &str = match option_env!("REMOTE_VALERIICC_API_KEY") {
    Some(api_key) => api_key,
    None => "admin1234",
};
