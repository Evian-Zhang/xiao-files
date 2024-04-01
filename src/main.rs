use std::path::PathBuf;

use actix_files::Files;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{Data, Redirect},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;

/// Minimal file exchange server designed for clients with browsers only.
#[derive(Parser)]
struct Arg {
    /// IP address to bind
    #[arg(long)]
    address: String,
    /// Port to bind
    #[arg(long)]
    port: u16,
    /// Real path of hosted directory
    #[arg(long)]
    real_path: PathBuf,
    /// Number of threads used
    #[arg(long, short)]
    jobs: Option<usize>,
}

struct Config {
    real_path: PathBuf,
}

#[get("/upload")]
async fn upload_page() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../html/upload.html"))
}

#[derive(MultipartForm)]
struct Upload {
    file: TempFile,
}

#[post("/upload-files")]
async fn receive_uploaded_files(
    req: HttpRequest,
    config: Data<Config>,
    MultipartForm(Upload { file }): MultipartForm<Upload>,
) -> impl Responder {
    let Some(file_name) = file.file_name else {
        return HttpResponse::BadRequest().body("No file name specified");
    };
    let file_path = config.real_path.join(file_name);
    if let Err(err) = file.file.persist(file_path) {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to save temp file: {err}"));
    }
    Redirect::to("/files")
        .see_other()
        .respond_to(&req)
        .map_into_boxed_body()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let Arg {
        address,
        port,
        real_path,
        jobs,
    } = Arg::parse();

    log::info!("Starting HTTP server at http://{address}:{port}");
    log::info!("Go to http://{address}:{port}/files for files downloading.");
    log::info!("Go to http://{address}:{port}/upload for files uploading.");

    let mut server = HttpServer::new(move || {
        App::new()
            .service(upload_page)
            .service(receive_uploaded_files)
            .service(Files::new("/files", &real_path).show_files_listing())
            .app_data(Data::new(Config {
                real_path: real_path.clone(),
            }))
            .wrap(Logger::default())
    })
    .bind((address, port))?;
    if let Some(jobs) = jobs {
        server = server.workers(jobs);
    }
    server.run().await
}
