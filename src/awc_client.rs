use awc::http::header::{ACCEPT, CONTENT_TYPE};

pub fn client() -> awc::Client {
    awc::Client::builder()
        .add_default_header((ACCEPT, "application/json"))
        .add_default_header((CONTENT_TYPE, "application/json"))
        .finish()
}
