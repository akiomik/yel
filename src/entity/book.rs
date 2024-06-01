use sea_orm::entity::prelude::*;
use tabled::Tabled;

use crate::ContentType;

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
    #[tabled(rename = "Author")]
    pub author: String,
    #[sea_orm(column_name = "ZCONTENTTYPE")]
    #[tabled(rename = "Type")]
    pub content_type: ContentType,
    #[sea_orm(column_name = "ZISNEW")]
    #[tabled(rename = "New", display_with = "display_option")]
    pub is_new: Option<bool>,
    #[sea_orm(column_name = "ZISFINISHED")]
    #[tabled(rename = "Finished", display_with = "display_option")]
    pub is_finished: Option<bool>,
}

fn display_option(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("{s}"),
        None => "-".to_string(),
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
