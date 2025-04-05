use actix_web::{web, HttpResponse, Responder};
use crate::models::{FileUpload, ValidationResult};
use crate::validator::validate_yaml_alignment;
use std::fs;

pub async fn upload_yaml(file: web::Json<FileUpload>) -> impl Responder {
    let result: ValidationResult = validate_yaml_alignment(&file.file);
    HttpResponse::Ok().json(result)
}

pub async fn index() -> impl Responder {
    let html_path: &str = "src/template/index.html";
    match fs::read_to_string(html_path) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => {
            eprintln!("Failed to load HTML file: {}", err);
            HttpResponse::InternalServerError().body("Error loading the HTML file")
        }
    }
}
