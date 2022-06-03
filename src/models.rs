use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }
}
impl Responder for Post {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)

    }
}
