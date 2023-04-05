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
use actix_web::http::{header, Method};
use actix_web::web::{Buf, BufMut, Bytes, BytesMut};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use mime::Mime;
use reqwest::{RequestBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use tempfile::NamedTempFile;

use crate::posicube::utils::{create_response_to_client, get_from_client_request_properties, };

// feature를 impl해야 next()를 할 수 있음...
pub async fn multipart(req: HttpRequest, mut payload: Multipart) -> HttpResponse {
    let mut send_form = Form::new();

    while let item_try = payload.try_next().await {
        match item_try {
            Ok(Some(mut item)) => {
                let mut bytes = BytesMut::new();
                while let Some(chunk_result) = item.next().await {
                    match chunk_result {
                        Ok(data) => {
                            bytes.put_slice(&data);
                        }
                        Err(e) => {
                            eprintln!("Error while reading chunk: {:?}", e);
                            return HttpResponse::InternalServerError().body(format!("Error while reading multipart data : {}", e));
                        }
                    }
                }

                let content_disposition = item.content_disposition();
                let name = content_disposition.get_name().unwrap().to_string();
                let file_name = content_disposition.get_filename().unwrap_or("").to_string();
                println!("content_disposition : {:?}", content_disposition);
                println!("name : {:?}", name);
                println!("file_name : {:?}", file_name);

                if file_name.is_empty() {
                    // this case not file
                    match String::from_utf8(bytes.to_vec()) {
                        Ok(s) => {
                            println!("value is {:?}", s);
                            send_form = send_form.text(name, s)
                        }
                        Err(e) => return HttpResponse::InternalServerError().body(format!("Error converting BytesMut to String: {}", e))
                    };
                } else {
                    // this case is file
                    if let Some(content_type) = item.content_type() {
                        let mut file_contents = Vec::new();
                        Write::write_all(&mut file_contents, &bytes).unwrap();

                        let part = Part::bytes(file_contents)
                            .file_name(file_name.clone())
                            .mime_str(content_type.clone().as_ref())
                            .unwrap();

                        send_form = send_form.part(name, part);
                    } else {
                        return HttpResponse::InternalServerError().body("Failed to parse MIME type");
                    }
                }
            }
            Ok(None) => {
                println!("item is none");
                break;
            }
            Err(e) => {
                eprintln!("error : {:?}", e.to_string());
                eprintln!("error request : {:?}", req);
                return HttpResponse::InternalServerError().body(e.to_string());
            }
        }
    }

    println!("form-data : {:?}", send_form);

    let dynamic_request_properties = get_from_client_request_properties(&req).await;
    let uri = dynamic_request_properties.2;

    let client = reqwest::Client::new();
    let result = client
        .post("http://localhost:8081".to_string() + uri.as_str())
        .multipart(send_form)
        .send()
        .await;

    create_response_to_client(result).await
}
