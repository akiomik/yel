use sea_orm::entity::prelude::*;
use tabled::Tabled;

use crate::{display_option_bool, ContentType};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Tabled)]
#[sea_orm(table_name = "ZBKLIBRARYASSET")]
pub struct Model {
    #[sea_orm(column_name = "Z_PK", primary_key)]
    #[tabled(skip)]
    pub id: u32,
    #[sea_orm(column_name = "ZTITLE")]
    #[tabled(rename = "Title")]
    pub title: String,
    #[sea_orm(column_name = "ZAUTHOR")]
    #[tabled(rename = "Author", display("Self::display_author", self))]
    pub author: String,
    #[sea_orm(column_name = "ZCONTENTTYPE")]
    #[tabled(rename = "Type")]
    pub content_type: ContentType,
    #[sea_orm(column_name = "ZISNEW")]
    #[tabled(rename = "New", display = "display_option_bool")]
    pub is_new: Option<bool>,
    #[sea_orm(column_name = "ZISFINISHED")]
    #[tabled(rename = "Finished", display = "display_option_bool")]
    pub is_finished: Option<bool>,
}

impl Model {
    pub fn is_unknown_author(&self) -> bool {
        self.author == "\u{e83a}UnknownAuthor"
    }

    pub fn display_author(_author: &String, model: &Self) -> String {
        if model.is_unknown_author() {
            "Unknown Author".to_string()
        } else {
            model.author.clone()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
