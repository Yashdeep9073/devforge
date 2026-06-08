use crate::api_client::methods::HttpMethod;

pub struct ApiRequest {
    pub method: HttpMethod,
    pub url: String,

    pub query_params: Option<Vec<(String, String)>>,
    pub headers: Vec<(String, String)>,

    pub body: Option<String>,
}
