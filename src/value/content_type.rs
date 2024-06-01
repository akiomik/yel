use std::fmt::Display;

use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum ContentType {
    Book = 1,
    Pdf = 3,
    Series = 5,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Book => "Book",
            Self::Pdf => "PDF",
            Self::Series => "Series",
        };

        write!(f, "{s}")
    }
}
