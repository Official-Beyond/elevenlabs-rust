use std::env;

pub fn load_api_key() -> Result<String, env::VarError> {
    env::var("ELEVENLABS_API_KEY")
}

pub fn load_api_url() -> Result<String, env::VarError> {
    env::var("ELEVENLABS_API_URL")
}
