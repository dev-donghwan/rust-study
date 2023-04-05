use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use reqwest::{Client, Method, RequestBuilder, Response};
use reqwest::header::HeaderMap;
use serde_json::Value;

use crate::posicube::utils::{create_response_to_client, get_from_client_request_properties};

pub async fn send_application_json_type_api(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    println!("start application/json type api generation");
    // generate dynamic properties [ headers, method, uri ... ]
    let dynamic_request_properties = get_from_client_request_properties(&req).await;
    let headers = dynamic_request_properties.0;
    let method = dynamic_request_properties.1;
    let uri = dynamic_request_properties.2;

    // generate request client
    let request_body = serde_json::from_slice(&body).unwrap_or(Value::String("".to_string()));
    let client = Client::new();
    let full_url = "http://localhost:8081".to_owned() + uri.as_str();

    let result = client
        .request(method, full_url)
        .headers(headers)
        .body(request_body.to_string())
        .send()
        .await;

    create_response_to_client(result).await
}
