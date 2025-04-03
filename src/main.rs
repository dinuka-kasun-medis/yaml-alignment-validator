use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FileUpload {
    file: String,
}

#[derive(Serialize)]
struct ValidationResult {
    valid: bool,
    message: String,
}

fn validate_yaml_alignment(file_content: &str) -> ValidationResult {
    let lines: Vec<&str> = file_content.lines().collect();
    // let mut last_indent = 0;

    for (index, line) in lines.iter().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        let current_indent = line.chars().take_while(|c| c.is_whitespace()).count();

        if current_indent % 2 != 0 {
            return ValidationResult {
                valid: false,
                message: format!("Indentation error on line {}: Indentation should be a multiple of 2", index + 1),
            };
        }

        // last_indent = current_indent;
    }

    ValidationResult {
        valid: true,
        message: "YAML file is properly formatted.".to_string(),
    }
}

async fn upload_yaml(file: web::Json<FileUpload>) -> impl Responder {
    let content = &file.file;
    let result = validate_yaml_alignment(content);
    HttpResponse::Ok().json(result)
}

async fn index() -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_yaml))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// To run this tool, use the command:
// cargo run --edition 2024
// Then, access the web UI via: http://127.0.0.1:8080/
