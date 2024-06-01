use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ZBKLIBRARYASSET")]
pub struct Model {
    #[sea_orm(column_name = "Z_PK", primary_key)]
    pub id: u32,
    #[sea_orm(column_name = "ZTITLE")]
    pub title: String,
    #[sea_orm(column_name = "ZAUTHOR")]
    pub author: String,
    #[sea_orm(column_name = "ZCONTENTTYPE")]
    pub content_type: u8,
    #[sea_orm(column_name = "ZISNEW")]
    pub is_new: bool,
    #[sea_orm(column_name = "ZISFINISHED")]
    pub is_finished: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
