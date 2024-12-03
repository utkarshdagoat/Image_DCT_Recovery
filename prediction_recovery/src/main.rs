/// this contains the code for the web server which is irrelevant to the dip checkout recovery.rs and dct.rs for image processing
use crate::recovery::*;
use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{http::header, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
mod dct;
mod recovery;

async fn process_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    if let Ok(Some(mut field)) = payload.try_next().await {
        let mut bytes = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(&chunk?);
        }
        let img =
            image::load_from_memory(&bytes).map_err(|e| actix_web::error::ErrorBadRequest(e))?;
        let res  = img.resize_exact(512, 512, image::imageops::FilterType::Gaussian);
        let id = Uuid::new_v4().to_string();
        simulate(&res, id.clone())?;

        Ok(HttpResponse::Ok().json(id))
    } else {
        Ok(HttpResponse::BadRequest().body("No image provided"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/process", web::post().to(process_image))
            .service(actix_files::Files::new("/images", "./images"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
