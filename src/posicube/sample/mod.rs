use actix_web::web;
use actix_web::web::{delete, get, post, resource, scope};

use crate::posicube::json_api::send_application_json_type_api;
use crate::posicube::multipart_api::send_multipart_api;

pub fn agent_super_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/v1").service(
            scope("/sample").service(
                scope("/entity")
                    .route("", get().to(send_application_json_type_api))
                    .route("", post().to(send_multipart_api))
                    .service(resource("/bulk}").route(post().to(send_application_json_type_api)))
                    .service(
                        resource("/{entityId}").route(delete().to(send_application_json_type_api)),
                    )
                    .service(
                        resource("/params/static").route(get().to(send_application_json_type_api)),
                    ),
            ),
        ),
    );
}
