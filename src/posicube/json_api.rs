use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use reqwest::{Client, Method, RequestBuilder, Response};
use reqwest::header::HeaderMap;
use serde_json::Value;

pub async fn send_application_json_type_api(req: HttpRequest, body: web::Bytes) -> impl Responder {
    println!("start application/json type api generation");
    // generate dynamic properties [ headers, method, uri ... ]
    let dynamic_request_properties = get_from_client_request_properties(&req).await;
    let headers = dynamic_request_properties.0;
    let method = dynamic_request_properties.1;
    let uri = dynamic_request_properties.2;

    // generate request client
    let request_body = get_from_client_request_body(&body).await;
    let client = Client::new();
    let full_url = "http://localhost:8081".to_owned() + uri.as_str();

    let json_type_request = client
        .request(method, full_url)
        .headers(headers)
        .body(request_body.to_string());

    if let Some(response) = request_to_server(json_type_request).await {
        create_response_to_client(response).await
    } else {
        HttpResponse::InternalServerError().body("response is error or none")
    }
}

async fn request_to_server(builder: RequestBuilder) -> Option<Response> {
    println!("request this app to backend...");
    match builder.send().await {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

async fn get_from_client_request_properties(request: &HttpRequest) -> (HeaderMap, Method, String) {
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

async fn get_from_client_request_body(request_body: &web::Bytes) -> Value {
    serde_json::from_slice(&request_body).unwrap_or(Value::String("".to_string()))
}

async fn create_response_to_client(response: Response) -> HttpResponse {
    println!("this app response to client");
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
