use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{DbErr, QueryOrder};

use crate::book::Entity as Book;
use crate::book::{Column, Model};

pub struct BookRepository {
    db: DatabaseConnection,
}

impl BookRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Book::find().order_by_asc(Column::Id).all(&self.db).await
    }
}
