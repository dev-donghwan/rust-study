use std::fs::File;
use std::future::Future;
use std::io::{Read, Write};
use std::process::Output;
use std::ptr::null;
use std::str::FromStr;

use actix_multipart::{Field, Multipart};
use actix_multipart::form::{bytes, MultipartCollect};
use actix_web::{Error, FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web::Payload};
use actix_web::body::MessageBody;
use actix_web::http::header::ContentDisposition;
use actix_web::http::Method;
use actix_web::web::{Buf, BufMut, Bytes, BytesMut};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use mime::Mime;
use reqwest::{RequestBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use tempfile::NamedTempFile;

use crate::posicube::utils::{create_response_to_client, get_from_client_request_properties, request_to_other_server};

// feature를 impl해야 next()를 할 수 있음...
pub async fn send_multipart_api(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let mut send_form = Form::new();
    while let Some(mut item) = payload.try_next().await.unwrap() {
        let mut bytes = BytesMut::new();
        while let Some(chunk) = item.next().await {
            let data = chunk.unwrap();
            bytes.put_slice(&data);
        }

        let content_disposition = &item.content_disposition();
        let name = content_disposition.get_name().unwrap().to_string();
        let file_name = content_disposition.get_filename().unwrap_or("").to_string();

        if file_name.is_empty() {
            // this case not file
            send_form = send_form.text(name, "aa");
        } else {
            let content_type = Mime::from_str(&item.content_type().unwrap().to_string()).expect("Failed to parse MIME type");

            println!("contentType : {:?}", content_type);
            // this case is file
            let mut file_contents = Vec::new();
            Write::write_all(&mut file_contents, &bytes).unwrap();

            let mut part = Part::bytes(file_contents)
                .file_name(file_name.clone())
                .mime_str(content_type.as_ref()).unwrap();


            send_form = send_form.part(name, part);
        }
    }
    println!("form-data : {:?}", send_form);

    let dynamic_request_properties = get_from_client_request_properties(&req).await;
    let uri = dynamic_request_properties.2;

    let builder = reqwest::Client::new()
        .post("http://localhost:8081".to_string() + uri.as_str())
        .multipart(send_form);

    request_to_other_server(builder)
}
