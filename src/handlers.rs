use actix_web::{web, HttpResponse, Responder};
use crate::models::{FileUpload, ValidationResult};
use crate::validator::validate_yaml_alignment;

pub async fn upload_yaml(file: web::Json<FileUpload>) -> impl Responder {
    let result: ValidationResult = validate_yaml_alignment(&file.file);
    HttpResponse::Ok().json(result)
}

pub async fn index() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>YAML Alignment Validator</title>
    </head>
    <body>
        <h1>Upload your YAML file</h1>
        <form id="upload-form">
            <textarea id="yaml-content" rows="20" cols="80" placeholder="Paste your YAML content here..."></textarea><br>
            <button type="button" onclick="uploadFile()">Validate</button>
        </form>
        <div id="result"></div>

        <script>
            async function uploadFile() {
                const content = document.getElementById('yaml-content').value;
                const response = await fetch('/upload', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ file: content })
                });
                const result = await response.json();
                document.getElementById('result').innerText = result.message;
            }
        </script>
    </body>
    </html>
    "#;
    HttpResponse::Ok().body(html)
}
