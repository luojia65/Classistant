pub mod api_191017;

pub fn get(version: &str) -> AppApi {
    if version == api_191017::VERSION {
        return AppApi::Api191017
    }
    return AppApi::Invalid;
}

pub enum AppApi {
    Api191017,
    Invalid,
}
