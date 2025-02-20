use serde::{Deserialize, Serialize};

use crate::{Message, MyError};
use crate::constants::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: String,
}

#[derive(Deserialize, Serialize)]
pub struct BookPayload {
    pub action: String,
    pub id: Option<String>,
    pub book: Option<Book>,
}

impl BookPayload {
    pub fn parse(payload: BookPayload) -> Result<Message, MyError> {
        match payload.action.as_str() {
            ACTION_GET_BOOKS => Ok(Message::GetAll),
            ACTION_GET_BOOK => payload
                .id
                .map(|id| Message::Get(id))
                .ok_or(MyError::EmptyId),
            ACTION_ADD_BOOK => payload
                .book
                .map(|b| Message::Add(b))
                .ok_or(MyError::EmptyBody),
            ACTION_UPDATE_BOOK => {
                if let Some(id) = payload.id {
                    if let Some(book) = payload.book {
                        Ok(Message::Update(id, book))
                    } else {
                        Err(MyError::EmptyBody)
                    }
                } else {
                    Err(MyError::EmptyId)
                }
            }
            ACTION_DELETE_BOOK => payload
                .id
                .map(|id| Message::Delete(id))
                .ok_or(MyError::EmptyId),
            _ => Err(MyError::UnknownAction),
        }
    }
}
