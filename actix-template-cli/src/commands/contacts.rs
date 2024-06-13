use super::configure::ActixTemplateConfiguration;
use crate::{errors::DefaultError, ui::get_cancelable_render_config};
use actix_web_starter_client::{
    apis::contacts_api,
    models::{self, Contact},
};
use clap::{Args, Subcommand};
use inquire::validator::Validation;
use tabled::settings::Style;

#[derive(Subcommand)]
pub enum ContactCommands {
    Create,
    Delete(DeleteContact),
    Edit(EditContact),
    View(ViewContact),
    List,
}

#[derive(Args)]
pub struct DeleteContact {
    /// The id of the contact to delete
    pub id: Option<String>,
}

#[derive(Args)]
pub struct EditContact {
    /// The id of the contact to edit
    pub id: Option<String>,
}

#[derive(Args)]
pub struct ViewContact {
    /// The id of the contact to view
    pub id: Option<String>,
}

pub async fn create_contact_cmd(config: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let first_name = inquire::Text::new("Enter the first name of the contact:")
        .with_render_config(get_cancelable_render_config("No First Name Entered"))
        .prompt()?;
    let last_name = inquire::Text::new("Enter the last name of the contact:")
        .with_render_config(get_cancelable_render_config("No Last Name Entered"))
        .prompt()?;
    let result = contacts_api::create_contact(
        &config.clone().into(),
        contacts_api::CreateContactParams {
            organization: config.org_id,
            create_contact_req_payload: actix_web_starter_client::models::CreateContactReqPayload {
                first_name,
                last_name,
            },
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for create contact"))?;

    match result {
        contacts_api::CreateContactSuccess::Status201(_) => {
            println!("Contact created successfully");
            Ok(())
        }
        contacts_api::CreateContactSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown value returned from API for create contact",
        )),
    }
}

pub async fn get_contact(
    config: ActixTemplateConfiguration,
    contact_id: String,
) -> Result<Contact, DefaultError> {
    let result = contacts_api::get_contact(
        &config.clone().into(),
        contacts_api::GetContactParams {
            contact_id,
            organization: config.org_id,
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for view contact"))?;
    match result {
        contacts_api::GetContactSuccess::Status200(contact) => Ok(contact),
        contacts_api::GetContactSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown value returned from API for view contact",
        )),
    }
}

pub async fn delete_contact_cmd(
    config: ActixTemplateConfiguration,
    id: Option<String>,
) -> Result<(), DefaultError> {
    let id = if let Some(id) = id {
        id
    } else {
        let id = inquire::Text::new("Enter the id of the contact to delete:")
            .with_render_config(get_cancelable_render_config("No ID Entered"))
            .prompt()?;
        id
    };
    let result = contacts_api::delete_contact(
        &config.clone().into(),
        contacts_api::DeleteContactParams {
            contact_id: id,
            organization: config.org_id,
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for delete contact"))?;

    match result {
        contacts_api::DeleteContactSuccess::Status204() => {
            println!("Contact deleted successfully");
            Ok(())
        }
        contacts_api::DeleteContactSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown value returned from API for delete contact",
        )),
    }
}

pub async fn view_contact_cmd(
    config: ActixTemplateConfiguration,
    id: Option<String>,
) -> Result<(), DefaultError> {
    let id = if let Some(id) = id {
        id
    } else {
        let id = inquire::Text::new("Enter the id of the contact to view:")
            .with_render_config(get_cancelable_render_config("No ID Entered"))
            .prompt()?;
        id
    };
    let contact = get_contact(config, id).await?;
    println!("First Name: {}", contact.first_name);
    println!("Last Name: {}", contact.last_name);
    println!("ID: {}", contact.id);
    Ok(())
}

pub async fn edit_contact_cmd(
    config: ActixTemplateConfiguration,
    id: Option<String>,
) -> Result<(), DefaultError> {
    let id = if let Some(id) = id {
        id
    } else {
        let id = inquire::Text::new("Enter the id of the contact to edit:")
            .with_render_config(get_cancelable_render_config("No ID Entered"))
            .prompt()?;
        id
    };
    let contact = get_contact(config.clone(), id.clone()).await?;
    let first_name = inquire::Text::new("Enter the first name of the contact:")
        .with_default(&contact.first_name)
        .with_render_config(get_cancelable_render_config("No First Name Entered"))
        .prompt()?;
    let last_name = inquire::Text::new("Enter the last name of the contact:")
        .with_default(&contact.last_name)
        .with_render_config(get_cancelable_render_config("No Last Name Entered"))
        .prompt()?;
    let result = contacts_api::update_contact(
        &config.clone().into(),
        contacts_api::UpdateContactParams {
            contact_id: id,
            organization: config.org_id,
            update_contact_req_payload: models::UpdateContactReqPayload {
                first_name: Some(Some(first_name)),
                last_name: Some(Some(last_name)),
            },
        },
    )
    .await
    .unwrap_or_else(|e| {
        eprintln!("Error updating contact: {:?}", e);
        std::process::exit(1);
    })
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for update contact"))?;

    match result {
        contacts_api::UpdateContactSuccess::Status200(_) => {
            println!("Contact updated successfully");
            Ok(())
        }
        contacts_api::UpdateContactSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown value returned from API for update contact",
        )),
    }
}

pub async fn list_contacts_cmd(config: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let options = vec!["Next Page", "Previous Page", "Stop"];
    let mut offsets: Vec<Option<String>> = vec![None];
    let mut page_num: usize = 0;
    let mut stop = false;
    let limit = get_limit()?;

    while !stop {
        let offset = offsets.get(page_num).cloned().flatten();
        println!("offset: {}", offset.as_deref().unwrap_or("None"));
        let contacts = list_contacts(config.clone(), Some(limit), offset).await?;
        println!("contacts: {:?}", contacts.len());
        println!("contacts: {:?}", contacts);

        let table = build_contacts_table(&contacts);
        println!("{}", table);

        if let Some(last_contact) = contacts.last() {
            if page_num + 1 == offsets.len() {
                offsets.push(Some(last_contact.id.clone()));
            } else {
                offsets[page_num + 1] = Some(last_contact.id.clone());
            }
        }

        let next = prompt_next_page(&options)?;
        match next.as_str() {
            "Stop" => stop = true,
            "Previous Page" => page_num = page_num.saturating_sub(1),
            _ => page_num = (page_num + 1).min(offsets.len() - 1),
        }
    }

    Ok(())
}

fn get_limit() -> Result<i64, inquire::InquireError> {
    inquire::CustomType::<i64>::new("Enter the number of results to return per page:")
        .with_validator(|value: &i64| {
            if *value < 1 {
                Ok(Validation::Invalid("Limit must be greater than 0".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_render_config(get_cancelable_render_config("No Limit Entered"))
        .prompt()
}

fn prompt_next_page(options: &[&str]) -> Result<String, inquire::InquireError> {
    inquire::Select::new("Select page", options.to_vec())
        .with_render_config(get_cancelable_render_config("No Next Page Entered"))
        .prompt()
        .map(|s| s.to_string())
}

fn build_contacts_table(contacts: &[Contact]) -> String {
    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["ID", "First Name", "Last Name"]);
    if contacts.is_empty() {
        builder.push_record(["No Contacts found", "", ""]);
    } else {
        for contact in contacts {
            builder.push_record([
                contact.id.clone(),
                contact.first_name.clone(),
                contact.last_name.clone(),
            ]);
        }
    }
    builder.build().with(Style::rounded()).to_string()
}

async fn list_contacts(
    config: ActixTemplateConfiguration,
    limit: Option<i64>,
    offset: Option<String>,
) -> Result<Vec<Contact>, DefaultError> {
    let contacts = contacts_api::list_contacts(
        &config.clone().into(),
        contacts_api::ListContactsParams {
            limit,
            offset,
            organization: config.org_id.clone(),
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for list contacts"))?;
    match contacts {
        contacts_api::ListContactsSuccess::Status200(contacts) => Ok(contacts.contacts),
        contacts_api::ListContactsSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown value returned from API for list contacts",
        )),
    }
}
