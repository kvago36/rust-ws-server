mod book;
mod constants;
mod error;
mod message;
mod state;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use serde_json::json;

use constants::*;
use book::{Book, BookPayload};
use error::MyError;
use message::Message;
use state::AppState;

async fn books(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            if let Ok(AggregatedMessage::Text(text)) = msg {
                let json_string = String::from_utf8(text.as_bytes().to_vec()).unwrap();
                let payload = serde_json::from_str::<BookPayload>(&json_string);

                if let Ok(payload) = payload {
                    let action_type = payload.action.clone();
                    let message = BookPayload::parse(payload);

                    let value = message.map_or_else(
                        |e| json!({"action": action_type, "status": "error", "error": e.to_string() }),
                        |message| data.apply_message(message),
                    );

                    let json_string = value.to_string();

                    session.text(json_string).await.unwrap();
                } else {
                    session
                        .text(MyError::ParsingError.to_string())
                        .await
                        .unwrap();
                }
            }
        }
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Injecting the app state into the application
            .route("/books", web::get().to(books))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_http::ws::Frame;
    use actix_web::{App, test, web};
    use actix_ws::Message;
    use futures_util::SinkExt;

    #[actix_web::test]
    async fn test_invalid_json() {
        let app_state = web::Data::new(AppState::new());

        let mut srv = actix_test::start(move || {
            App::new()
                .app_data(app_state.clone())
                .route("/books", web::get().to(books))
        });

        let mut framed = srv.ws_at("/books").await.unwrap();

        framed.send(Message::Text("123".into())).await.unwrap();

        let frame = framed.next().await.unwrap().unwrap();

        assert_eq!(frame, Frame::Text(MyError::ParsingError.to_string().into()));
    }

    #[actix_web::test]
    async fn test_add_book() {
        let app_state = web::Data::new(AppState::new());

        let mut srv = actix_test::start(move || {
            App::new()
                .app_data(app_state.clone())
                .route("/books", web::get().to(books))
        });

        let mut framed = srv.ws_at("/books").await.unwrap();

        let value = json!({ "action": ACTION_ADD_BOOK, "book": { "title": "123", "year": "123", "author": "Bill"} }).to_string();

        framed.send(Message::Text(value.into())).await.unwrap();

        let frame = framed.next().await.unwrap().unwrap();

        let valid_json = json!({ "action": ACTION_ADD_BOOK, "status": "ok", "book": { "title": "123", "year": "123", "author": "Bill"} });
        let valid_json_string = valid_json.to_string();

        assert_eq!(frame, Frame::Text(valid_json_string.into()));
    }
}
