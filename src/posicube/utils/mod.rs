use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use reqwest::{Error, header::HeaderMap, Method, RequestBuilder, Response};

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
    let uri = request.uri().to_string();
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