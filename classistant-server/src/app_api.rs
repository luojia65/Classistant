pub mod api_191017;

pub fn get(version: &str) -> Option<AppApi> {
    if version == api_191017::VERSION {
        return Some(AppApi::Api191017(api_191017::Api))
    }
    return None;
}

pub enum AppApi {
    Api191017(api_191017::Api)
}
