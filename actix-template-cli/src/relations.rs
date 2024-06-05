use actix_web_starter_client::models::Note;

use crate::{commands::configure::ActixTemplateConfiguration, errors::DefaultError};

// Either phone, contact etc... will have a bunch of matches
pub trait Relation {
    fn fn_get_extension_path(&self) -> String;
}

pub trait HasRelations<D>
where
    D: Relation,
{
    fn get_relations<T>(
        &self,
        config: ActixTemplateConfiguration,
        relation: D,
    ) -> Result<Vec<T>, DefaultError>;
    fn add_relation(
        &self,
        config: ActixTemplateConfiguration,
        relation: D,
        other_id: String,
    ) -> Result<(), DefaultError>;
    fn delete_relation(
        &self,
        config: ActixTemplateConfiguration,
        relation: D,
        other_id: String,
    ) -> Result<(), DefaultError>;
}

pub enum NoteRelation {
    Task,
    Contact,
}

impl Relation for NoteRelation {
    fn fn_get_extension_path(&self) -> String {
        match self {
            NoteRelation::Task => "tasks".to_string(),
            NoteRelation::Contact => "contacts".to_string(),
        }
    }
}

impl HasRelations<NoteRelation> for Note {
    // not implemented
    fn get_relations<T>(
        &self,
        config: ActixTemplateConfiguration,
        relation: NoteRelation,
    ) -> Result<Vec<T>, DefaultError> {
        unimplemented!()
    }

    fn add_relation(
        &self,
        config: ActixTemplateConfiguration,
        relation: NoteRelation,
        other_id: String,
    ) -> Result<(), DefaultError> {
        unimplemented!()
    }

    fn delete_relation(
        &self,
        config: ActixTemplateConfiguration,
        relation: NoteRelation,
        other_id: String,
    ) -> Result<(), DefaultError> {
        unimplemented!()
    }
}
