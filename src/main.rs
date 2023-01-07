use actix_web::http::StatusCode;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use dotenv::dotenv;

fn send_mail() -> impl Responder {
    let email = Message::builder()
        .from("ribru17@gmail.com".parse().unwrap())
        .to("ribru17@gmail.com".parse().unwrap())
        .subject("Web message from {name}")
        .body(String::from("<Message here>"))
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

#[post("/api/contact")]
async fn contact() -> impl Responder {
    send_mail()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // port is PORT || 8080
    let port_string = std::env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse().unwrap();
    HttpServer::new(|| {
        App::new()
            .service(contact)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}