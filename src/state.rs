use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::book::Book;
use crate::error::MyError;
use crate::message::Message;
use crate::constants::*;

pub struct AppState {
    books: RwLock<HashMap<String, Book>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            books: RwLock::new(HashMap::new()),
        }
    }

    pub fn apply_message(&self, message: Message) -> Value {
        match message {
            Message::GetAll => {
                let books = self.get_all_book();

                json!({ "action": ACTION_GET_BOOKS, "status": "ok", "books": books })
            }
            Message::Get(id) => {
                self.get_book(&id).map_or(
                    json!({ "action": ACTION_GET_BOOK, "status": "error", "error": MyError::WrongId.to_string() }),
                    |b| json!({ "action": ACTION_GET_BOOK, "status": "ok", "book": b.clone() })
                )
            }
            Message::Add(book) => {
                self.add_book(book.title.clone(), book.clone()).map_or(
                    json!({ "action": ACTION_ADD_BOOK, "status": "ok", "book": book }),
                    |_| json!({ "action": ACTION_ADD_BOOK, "status": "error", "error": MyError::BookAlreadyExists.to_string() })
                )
            }
            Message::Update(id, book) => self.update_book(id, book).map_or(
                json!({ "action": ACTION_UPDATE_BOOK, "status": "error", "error": MyError::WrongId.to_string() }),
                |b| json!({ "action": ACTION_UPDATE_BOOK, "status": "ok", "book": b }),
            ),
            Message::Delete(id) => {
                self.remove_book(&id).map_or(
                    json!({ "action": ACTION_DELETE_BOOK, "status": "error", "error": MyError::WrongId.to_string() }),
                    |b| json!({ "action": ACTION_DELETE_BOOK, "status": "ok", "book": b }),
                )
            }
        }
    }

    fn remove_book(&self, id: &str) -> Option<Book> {
        self.books.write().unwrap().remove(id)
    }

    fn update_book(&self, id: String, book: Book) -> Result<Book, MyError> {
        if let Some(value) = self.books.write().unwrap().get_mut(&id) {
            *value = book.clone();
            Ok(book)
        } else {
            Err(MyError::WrongId)
        }
    }
    fn add_book(&self, title: String, book: Book) -> Option<Book> {
        self.books.write().unwrap().insert(title, book)
    }
    fn get_book(&self, id: &str) -> Option<Book> {
        self.books.read().unwrap().get(id).cloned()
    }

    fn get_all_book(&self) -> Vec<Book> {
        let mut books = Vec::new();

        for (_, book) in self.books.read().unwrap().iter() {
            books.push(book.clone());
        }

        books
    }
}
