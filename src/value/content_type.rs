use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum ContentType {
    Book = 1,
    Pdf = 3,
    Series = 5,
}
