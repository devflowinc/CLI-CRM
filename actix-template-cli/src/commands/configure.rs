use crate::{commands::login_server::server, errors::DefaultError, Login};
use actix_web_starter_client::apis::{
    auth_api::{whoami, WhoamiSuccess},
    configuration::{ApiKey, Configuration},
};
use inquire::{Confirm, Text};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use tokio::sync::mpsc;

use super::orgs::{create_org, select_from_my_orgs, OrgSelectError};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ActixTemplateConfiguration {
    pub api_key: String,
    pub org_id: String,
    pub api_url: String,
}

impl From<ActixTemplateConfiguration> for Configuration {
    fn from(config: ActixTemplateConfiguration) -> Self {
        Configuration {
            base_path: config.api_url.clone(),
            api_key: Some(ApiKey {
                prefix: None,
                key: config.api_key.clone(),
            }),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActixTemplateProfileInner {
    pub name: String,
    pub settings: ActixTemplateConfiguration,
    pub selected: bool,
}

impl Default for ActixTemplateProfileInner {
    fn default() -> Self {
        ActixTemplateProfileInner {
            name: "default".to_string(),
            settings: ActixTemplateConfiguration::default(),
            selected: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActixTemplateProfile {
    pub inner: Vec<ActixTemplateProfileInner>,
}

impl Default for ActixTemplateProfile {
    fn default() -> Self {
        ActixTemplateProfile {
            inner: vec![ActixTemplateProfileInner::default()],
        }
    }
}

impl Deref for ActixTemplateProfile {
    type Target = Vec<ActixTemplateProfileInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ActixTemplateProfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Default for ActixTemplateConfiguration {
    fn default() -> Self {
        ActixTemplateConfiguration {
            api_key: "".to_string(),
            org_id: "".to_string(),
            api_url: "http://localhost:8090".to_string(),
        }
    }
}

pub async fn get_user(
    api_url: String,
    api_key: String,
) -> Result<actix_web_starter_client::apis::auth_api::WhoamiSuccess, DefaultError> {
    let configuration = Configuration {
        base_path: api_url.clone(),
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key.clone(),
        }),
        ..Default::default()
    };

    Ok(whoami(&configuration)
        .await
        .map_err(|e| DefaultError::new(format!("Error getting user: {:?}", e).as_str()))?
        .entity
        .expect("User entity not found"))
}

async fn configure(
    api_url: String,
    mut api_key: Option<String>,
) -> Result<ActixTemplateConfiguration, DefaultError> {
    if api_key.is_none() {
        let (tx, mut rx) = mpsc::channel::<String>(100);

        let server = tokio::spawn(async move {
            server(tx.clone()).await.map_err(|e| {
                eprintln!("Error starting server: {:?}", e);
                std::process::exit(1);
            })
        });

        let auth_url = format!(
            "{api_url}/api/auth?redirect_uri={api_url}/auth/cli%3Fhost={api_url}",
            api_url = api_url
        );

        let _ = Text::new("Press Enter to authenticate in browser: ").prompt()?;

        if open::that(auth_url.clone()).is_err() {
            eprintln!("Error opening browser. Please visit the URL manually.");
            println!(
                "\nPlease go to the following URL to get a Trieve API Key: {}",
                auth_url
            );
        }

        api_key = Some(rx.recv().await.unwrap());

        server.abort();
    }

    let result = get_user(api_url.clone(), api_key.clone().unwrap()).await?;

    let temporary_config = Configuration {
        base_path: api_url.clone(),
        api_key: Some(actix_web_starter_client::apis::configuration::ApiKey {
            prefix: None,
            key: api_key.clone().expect("Able to get api key"),
        }),
        ..Default::default()
    };

    match result {
        WhoamiSuccess::Status200(_) => {
            let potential_org =
                select_from_my_orgs(&temporary_config, "Select an org to use for this profile")
                    .await;

            let org_id = match potential_org {
                Ok(selection) => Ok(selection.id),
                Err(OrgSelectError::NoOrgs) => {
                    // Create the org
                    let created = create_org(
                        ActixTemplateConfiguration {
                            api_key: api_key.clone().unwrap(),
                            org_id: "".to_string(),
                            api_url: api_url.clone(),
                        },
                        None,
                    )
                    .await?;
                    Ok(created.id)
                }
                Err(OrgSelectError::CancelInput) => {
                    Err(DefaultError::new("Org selection cancelled"))
                }
                Err(OrgSelectError::OrgFetchFailure) => {
                    Err(DefaultError::new("Error fetching orgs"))
                }
            }?;

            Ok(ActixTemplateConfiguration {
                // Prompt user to select an org or create one
                api_key: api_key.unwrap(),
                api_url: api_url.clone(),
                org_id,
            })
        }
        _ => Err(DefaultError::new("Error getting user")),
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ApiKeyResponse {
    api_key: String,
}

pub async fn login(init: Login, settings: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let api_key = init.api_key;
    let mut api_url = init.api_url;

    if settings.api_key.is_empty() {
        println!(
            "Welcome to the Actix Template CLI! Let's get started by configuring your API Key."
        );
    } else {
        println!("Welcome back to the Actix Template CLI! Let's update your configuration.");
    }

    if api_url.is_none() {
        let use_default = Confirm::new(
            "Would you like to use the default URL for the Actix Template server (http://localhost:8090)?",
        )
        .with_default(true)
        .prompt()?;

        if use_default {
            api_url = Some("http://localhost:8090".to_string());
        } else {
            Text::new("Enter the URL of the Actix Template server:")
                .with_default("http://localhost:8090")
                .prompt()
                .unwrap_or("http://localhost:8090".to_string());
        }
    }

    let config = configure(api_url.unwrap().clone(), api_key).await?;

    let profile_name = if init.profile_name.is_none() {
        let profile_name = Text::new("Enter a name for this profile:")
            .with_default("default")
            .prompt()
            .unwrap_or("default".to_string());
        println!(
            "Configuration complete! Your profile has been saved as '{}'.",
            profile_name
        );
        profile_name
    } else {
        init.profile_name.unwrap()
    };

    let mut profiles: ActixTemplateProfile = confy::load("actix_template", "profiles")
        .map_err(|e| {
            eprintln!("Error loading configuration: {:?}", e);
        })
        .unwrap_or_default();

    if profiles.iter().any(|p| p.name == profile_name) {
        let overwrite = Confirm::new("Profile already exists. Overwrite?")
            .with_default(false)
            .prompt()?;

        if !overwrite {
            return Ok(());
        }

        profiles.retain(|p| p.name != profile_name);
    }

    profiles.dedup_by_key(|p| p.name.clone());
    profiles.iter_mut().for_each(|p| p.selected = false);

    profiles.push(ActixTemplateProfileInner {
        name: profile_name,
        settings: config,
        selected: true,
    });

    confy::store("actix_template", "profiles", profiles)
        .map_err(|_| DefaultError::new("Error saving configuration"))
}
