use actix_web::http::StatusCode;
use actix_web::{post, App, HttpResponse, HttpServer, Responder, web};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use actix_cors::Cors;

use dotenv::dotenv;

fn send_mail(name: &String, message: &String) -> impl Responder {
    let email = Message::builder()
        .from("ribru17@gmail.com".parse().unwrap())
        .to("ribru17@gmail.com".parse().unwrap())
        .subject(format!("Web message from {}", name))
        .body(format!("{}", message))
        .unwrap();

    let creds = Credentials::new(
        std::env::var("EMAIL_USERNAME")
            .expect("Must set EMAIL_USERNAME environment variable"),
        std::env::var("EMAIL_PW")
            .expect("Must set EMAIL_PW environment variable")
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    return match mailer.send(&email) {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_e) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct MessageParams {
    name: String,
    message: String
}

#[post("/api/contact")]
async fn contact(req: web::Json<MessageParams>) -> impl Responder {
    send_mail(&req.name.clone(), &req.message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // port is PORT || 8080
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string()).parse().unwrap();
    let location = match std::env::var("IS_DEV") {
        Ok(_) => "127.0.0.1",
        Err(_) => "0.0.0.0"
    };
    HttpServer::new(|| {
        // allows all requests; temporary, not recommended solution
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(contact)
    })
    .bind((location, port))?
    .run()
    .await
}