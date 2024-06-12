use super::configure::ActixTemplateConfiguration;
use crate::{errors::DefaultError, ui::get_cancelable_render_config};
use actix_web_starter_client::{
    apis::deals_api::{self, CreateDealParams, CreateDealSuccess, GetDealParams, UpdateDealParams},
    models::{self, CreateDealReqPayload, Deal, UpdateDealReqPayload},
};
use clap::{Args, Subcommand};
use inquire::validator::Validation;
use tabled::settings::Style;

#[derive(Subcommand)]
pub enum DealCommands {
    Create,
    Delete(DeleteDeal),
    Edit(EditDeal),
    View(ViewDeal),
    List,
    #[command(subcommand, about = "Commands to manage contacts for a deal")]
    ManageContacts(ManageContactsCommands),
}

#[derive(Args)]
pub struct DeleteDeal {
    /// The id of the deal you want to delete
    pub id: Option<String>,
}

#[derive(Args)]
pub struct EditDeal {
    /// The id of the deal you want to edit
    pub id: String,
}

#[derive(Args)]
pub struct ViewDeal {
    /// The id of the deal you want to delete
    pub id: String,
}

pub async fn create_deal_cmd(config: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let name = inquire::Text::new("Enter deal name:")
        .with_render_config(get_cancelable_render_config("No Description"))
        .prompt()?;

    let active = inquire::Confirm::new("Is this deal active?")
        .with_default(true)
        .prompt()?;

    let size = inquire::CustomType::<f32>::new("Enter deal size:")
        .with_default(0.0)
        .prompt()?;

    let result = deals_api::create_deal(
        &config.clone().into(),
        CreateDealParams {
            organization: config.org_id,
            create_deal_req_payload: CreateDealReqPayload {
                active: Some(Some(active)),
                name: Some(Some(name)),
                size: Some(Some(size)),
            },
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for create_deal"))?;

    match result {
        CreateDealSuccess::Status201(_) => {
            println!("Deal created successfully!");
            Ok(())
        }
        CreateDealSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown response from API for create_deal",
        )),
    }
}

pub async fn get_deal(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<Deal, DefaultError> {
    let response = deals_api::get_deal(
        &config.clone().into(),
        GetDealParams {
            organization: config.org_id,
            deal_id,
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for get_deal"))?;

    match response {
        deals_api::GetDealSuccess::Status200(deal) => Ok(deal),
        deals_api::GetDealSuccess::UnknownValue(_) => {
            Err(DefaultError::new("Unknown response from API for get_deal"))
        }
    }
}

pub async fn edit_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<(), DefaultError> {
    let deal = get_deal(config.clone(), deal_id.clone()).await?;
    let prev_deal_name = deal
        .name
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());
    let prev_deal_size = deal.size.unwrap_or(Some(0.0)).unwrap_or(0.0);

    let name = inquire::Text::new("Enter deal name:")
        .with_default(&prev_deal_name)
        .prompt()
        .unwrap_or(prev_deal_name);

    let active = inquire::Confirm::new("Is this deal active?")
        .with_default(deal.active)
        .prompt()
        .unwrap_or(deal.active);

    let size = inquire::CustomType::<f32>::new("Enter deal size:")
        .with_default(prev_deal_size)
        .prompt()
        .unwrap_or(prev_deal_size);

    let result = deals_api::update_deal(
        &config.clone().into(),
        UpdateDealParams {
            organization: config.org_id,
            deal_id,
            update_deal_req_payload: UpdateDealReqPayload {
                active: Some(Some(active)),
                name: Some(Some(name)),
                size: Some(Some(size)),
            },
        },
    )
    .await
    .map_err(|e| DefaultError::new(format!("Error updating deal: {:?}", e).as_str()))?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for update_deal"))?;

    match result {
        deals_api::UpdateDealSuccess::Status200(_) => {
            println!("Deal edited successfully!");
            Ok(())
        }
        deals_api::UpdateDealSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown response from API for update_deal",
        )),
    }
}

pub async fn view_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: String,
) -> Result<(), DefaultError> {
    let deal = get_deal(config.clone(), deal_id.clone()).await?;
    let name = deal
        .name
        .clone()
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());
    let size = deal.size.unwrap_or(Some(0.0)).unwrap_or(0.0);
    let active = deal.active;

    println!("Deal ID: {}", deal.id);
    println!("Name: {}", name);
    println!("Size: {}", size);
    println!("Active: {}", active);

    Ok(())
}

pub async fn delete_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: Option<String>,
) -> Result<(), DefaultError> {
    let deal_id = if deal_id.is_none() {
        inquire::Text::new("Enter deal ID to delete:").prompt()?
    } else {
        deal_id.unwrap()
    };

    let delete_response = deals_api::delete_deal(
        &config.clone().into(),
        deals_api::DeleteDealParams {
            deal_id,
            organization: config.org_id,
        },
    )
    .await?
    .status
    .is_success();

    match delete_response {
        true => {
            println!("Deal deleted successfully");
            Ok(())
        }
        false => Err(DefaultError::new("Error deleting deal")),
    }
}

pub async fn list_deals_cmd(config: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let options = vec!["Next Page", "Previous Page", "Stop"];
    let mut offsets: Vec<Option<String>> = vec![None];
    let mut page_num: usize = 0;
    let mut stop = false;
    let limit = get_limit()?;

    while !stop {
        let offset = offsets.get(page_num).cloned().flatten();
        let deals = list_deals(config.clone(), Some(limit), offset).await?;

        let table = build_deals_table(&deals);
        println!("{}", table);

        if let Some(last_deal) = deals.last() {
            if page_num + 1 == offsets.len() {
                offsets.push(Some(last_deal.id.clone()));
            } else {
                offsets[page_num + 1] = Some(last_deal.id.clone());
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

fn build_deals_table(deals: &[Deal]) -> String {
    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["ID", "Name", "Size", "Active"]);

    if deals.is_empty() {
        builder.push_record(["No deals found", "", "", ""]);
    } else {
        for deal in deals {
            builder.push_record([
                deal.id.clone(),
                deal.name
                    .clone()
                    .unwrap_or(Some("".to_string()))
                    .unwrap_or("".to_string()),
                deal.size.unwrap_or(Some(0.0)).unwrap_or(0.0).to_string(),
                deal.active.to_string(),
            ]);
        }
    }

    builder.build().with(Style::rounded()).to_string()
}

async fn list_deals(
    config: ActixTemplateConfiguration,
    limit: Option<i64>,
    offset: Option<String>,
) -> Result<Vec<Deal>, DefaultError> {
    let deals = deals_api::list_deal_by_org(
        &config.clone().into(),
        deals_api::ListDealByOrgParams {
            organization: config.org_id,
            limit,
            offset,
        },
    )
    .await?
    .entity
    .ok_or_else(|| DefaultError::new("No entity returned from API for list_deal_by_org"))?;
    match deals {
        deals_api::ListDealByOrgSuccess::Status200(deals) => Ok(deals.deals),
        deals_api::ListDealByOrgSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Unknown response from API for list_deal_by_org",
        )),
    }
}

pub async fn manage_contacts_cmd(
    config: ActixTemplateConfiguration,
    cmd: ManageContactsCommands,
) -> Result<(), DefaultError> {
    match cmd {
        ManageContactsCommands::Add(add_args) => {
            add_contact_to_deal_cmd(config, add_args.deal_id, add_args.id).await
        }
        ManageContactsCommands::Remove(remove_args) => {
            remove_contact_from_deal_cmd(config, remove_args.deal_id, remove_args.id).await
        }
    }
}

#[derive(Subcommand)]
pub enum ManageContactsCommands {
    Add(AddContact),
    Remove(RemoveContact),
}

#[derive(Args)]
pub struct AddContact {
    /// The id of the deal you want to add the contact to
    pub deal_id: Option<String>,
    /// The id of the contact you want to add
    pub id: Option<String>,
}

#[derive(Args)]
pub struct RemoveContact {
    /// The id of the deal you want to remove the contact from
    pub deal_id: Option<String>,
    /// The id of the contact you want to remove
    pub id: Option<String>,
}

pub async fn add_contact_to_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: Option<String>,
    contact_id: Option<String>,
) -> Result<(), DefaultError> {
    let deal_id = if deal_id.is_none() {
        inquire::Text::new("Enter deal ID to add contact to:").prompt()?
    } else {
        deal_id.unwrap()
    };
    let contact_id = if contact_id.is_none() {
        inquire::Text::new("Enter contact ID to add:").prompt()?
    } else {
        contact_id.unwrap()
    };
    let params = deals_api::CreateDealResourceParams {
        deal_id,
        resource_type: models::DealResType::Contact,
        resource_id: contact_id,
        organization: config.org_id.clone(),
    };
    let response = deals_api::create_deal_resource(&config.clone().into(), params)
        .await?
        .status
        .is_success();
    match response {
        true => {
            println!("Contact added to deal successfully");
            Ok(())
        }
        false => Err(DefaultError::new("Error adding contact to deal")),
    }
}

pub async fn remove_contact_from_deal_cmd(
    config: ActixTemplateConfiguration,
    deal_id: Option<String>,
    contact_id: Option<String>,
) -> Result<(), DefaultError> {
    let deal_id = if deal_id.is_none() {
        inquire::Text::new("Enter deal ID to add contact to:").prompt()?
    } else {
        deal_id.unwrap()
    };
    let contact_id = if contact_id.is_none() {
        inquire::Text::new("Enter contact ID to add:").prompt()?
    } else {
        contact_id.unwrap()
    };
    let params = deals_api::DeleteDealResourceParams {
        deal_id,
        resource_id: contact_id,
        resource_type: models::DealResType::Contact,
        organization: config.org_id.clone(),
    };
    let response = deals_api::delete_deal_resource(&config.clone().into(), params)
        .await?
        .status
        .is_success();
    match response {
        true => {
            println!("Contact removed from deal successfully");
            Ok(())
        }
        false => Err(DefaultError::new("Error removing contact from deal")),
    }
}
