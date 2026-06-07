use crate::api_client::methods::HttpMethod;

pub struct ApiRequest {
    pub method: HttpMethod,
    pub url: String,
}
