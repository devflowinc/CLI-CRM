use actix_web_starter_client::models::{Note, Task};

use crate::errors::DefaultError;

pub trait Relation {
    fn fn_get_extension_path(&self) -> String;
}

pub trait HasRelations<D>
where
    D: Relation,
{
    fn get_relation<T>(&self, other: &str) -> Result<T, DefaultError>;
}
