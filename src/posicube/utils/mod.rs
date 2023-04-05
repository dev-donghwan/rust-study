use std::mem::take;

use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use actix_web::http::header;
use actix_web::web::Path;
use futures::{StreamExt, TryStreamExt};
use reqwest::{Error, header::HeaderMap, Method, RequestBuilder, Response};

use crate::posicube::json_api::send_application_json_type_api;

pub async fn get_from_client_request_properties(request: &HttpRequest) -> (HeaderMap, Method, String) {
    let mut headers: HeaderMap = HeaderMap::new();
    println!("request client to this app");
    for (header_name, header_value) in request.headers().iter() {
        let name = header_name.clone();
        let value = header_value.clone();
        println!("header name: {:?}, value: {:?}", name, value);
        headers.append(name, value);
    }

    let method = request.method();
    let uri = request.uri().to_string().replace("/by-pass", "");
    println!("method : {:?}", method);
    println!("uri : {:?}", uri);
    (headers.clone(), Method::from(method), uri)
}

pub async fn create_response_to_client(result: Result<Response, Error>) -> HttpResponse {
    println!("this app response to client");
    return match result {
        Ok(response) => {
            let status_code = response.status();
            let mut res = HttpResponseBuilder::new(status_code);
            for (header_name, header_value) in response.headers().iter() {
                let name = header_name.clone();
                let value = header_value.clone();
                println!("header name: {:?}, value: {:?}", name, value);
                res.insert_header((name, value));
            }

            let body = match response.text().await {
                Ok(body_string) => body_string,
                Err(e) => e.to_string(),
            };

            res.body(body).into()
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string()).into()
        }
    };
}

pub async fn is_content_type_multipart(req: &HttpRequest) -> bool {
    let content_type = req.headers().get(header::CONTENT_TYPE);

    if let Some(content_type) = content_type {
        let content_type = content_type.to_str().unwrap_or("");
        if content_type.starts_with("multipart/form-data") {
            return true;
        }
    }

    false
}