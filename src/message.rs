use crate::Book;

pub enum Message {
    GetAll,
    Get(String),
    Add(Book),
    Update(String, Book),
    Delete(String),
}
