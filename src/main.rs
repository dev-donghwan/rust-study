use std::fs::File;
use std::future::Future;
use std::io::{Read, Write};
use std::process::Output;
use std::ptr::null;
use std::str::FromStr;

use actix_multipart::{Field, Multipart};
use actix_multipart::form::{bytes, MultipartCollect};
use actix_web::{App, Error, FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder, web};
use actix_web::body::MessageBody;
use actix_web::http::{header, Method};
use actix_web::http::header::ContentDisposition;
use actix_web::web::{Buf, BufMut, Bytes, BytesMut};
use actix_web::web::resource;
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use mime::Mime;
use reqwest::{RequestBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use tempfile::NamedTempFile;

use libs::posicube::json_api::send_application_json_type_api;
use libs::posicube::multipart_api::multipart;
use libs::posicube::utils::{create_response_to_client, is_content_type_multipart};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resource("/by-pass/{dynamic_path:.*}").to(api_handler))
    }).bind("localhost:8080")?
        .run()
        .await
}

pub async fn api_handler(
    req: HttpRequest,
    mut payload: Multipart,
    body: Bytes,
) -> HttpResponse {
    let is_multipart_from_content_type = is_content_type_multipart(&req).await;
    println!("is_multipart_from_content_type {:?}", is_multipart_from_content_type);

    if is_multipart_from_content_type {
        multipart(req, payload).await
    } else {
        send_application_json_type_api(req, body).await
    }
}