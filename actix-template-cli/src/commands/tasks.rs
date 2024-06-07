use actix_web_starter_client::{
    apis::tasks_api::{
        self, CreateTaskSuccess, GetTaskParams, UpdateTaskParams, UpdateTaskSuccess,
    },
    models::{CreateTaskReqPayload, Task, UpdateTaskReqPayload},
};

use crate::{errors::DefaultError, ui::get_cancelable_render_config};

use super::configure::ActixTemplateConfiguration;

fn transform_option<T>(opt: Option<T>) -> Option<Option<T>> {
    opt.map(Some)
}

fn reduce_option(opt: Option<Option<String>>) -> String {
    opt.unwrap_or(None).unwrap_or("".to_string())
}

pub async fn create_task_cmd(config: ActixTemplateConfiguration) -> Result<Task, DefaultError> {
    // Get description
    let description = inquire::Text::new("Enter description (or ESC):")
        .with_render_config(get_cancelable_render_config("No Description"))
        .prompt_skippable()?;
    // Get due date? (optional)
    let due_date = inquire::DateSelect::new("Enter a due date (or ESC):")
        .with_render_config(get_cancelable_render_config("No Due Date"))
        .prompt_skippable()?
        .map(|d| {
            // Convert to 2021-01-01T00:00:00 style string
            d.format("%Y-%m-%dT00:00:00").to_string()
        });

    // Send to API
    let result = tasks_api::create_task(
        &config.clone().into(),
        tasks_api::CreateTaskParams {
            organization: config.org_id,
            create_task_req_payload: CreateTaskReqPayload {
                description: transform_option(description),
                contact_id: None,
                deadline: transform_option(due_date),
            },
        },
    )
    .await?
    .entity
    .unwrap();
    // Return task
    match result {
        CreateTaskSuccess::Status201(task) => {
            println!("Task created successfully: {}", task.id);
            Ok(task)
        }
        CreateTaskSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body creating task",
        )),
    }
}

pub async fn get_task(
    config: ActixTemplateConfiguration,
    task_id: String,
) -> Result<Task, DefaultError> {
    let response = tasks_api::get_task(
        &config.clone().into(),
        GetTaskParams {
            task_id,
            organization: config.org_id,
        },
    )
    .await?
    .entity
    .unwrap();

    match response {
        tasks_api::GetTaskSuccess::Status200(task) => Ok(task),
        tasks_api::GetTaskSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body getting task by id",
        )),
    }
}

pub async fn edit_task_cmd(config: ActixTemplateConfiguration, task_id: String) {
    let task = get_task(config.clone(), task_id).await.unwrap();
    let description = inquire::Text::new("Enter description (or ESC):")
        .with_render_config(get_cancelable_render_config("No Description"))
        .prompt_skippable()
        .expect("Prompt renders correctly");
    let due_date = inquire::DateSelect::new("Enter a due date (or ESC):")
        .with_render_config(get_cancelable_render_config("No Due Date"))
        .prompt_skippable()
        .expect("Prompt renders correctly")
        .map(|d| d.format("%Y-%m-%dT00:00:00").to_string());

    let update_params = UpdateTaskParams {
        task_id: task.id,
        organization: config.clone().org_id,
        update_task_req_payload: UpdateTaskReqPayload {
            contact_id: None,
            description: Some(description),
            deadline: Some(due_date),
        },
    };

    let result = tasks_api::update_task(&config.into(), update_params)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error updating task: {:?}", e);
            std::process::exit(1);
        })
        .entity
        .expect("Task should be returned");

    match result {
        UpdateTaskSuccess::Status200(task) => {
            println!("Task updated successfully: {}", task.id);
        }

        UpdateTaskSuccess::UnknownValue(_) => {
            eprintln!("Could not parse response body when updating task");
            std::process::exit(1);
        }
    }
}

pub async fn view_task_cmd(config: ActixTemplateConfiguration, task_id: String) {
    let task = get_task(config, task_id).await.unwrap();
    let description = reduce_option(task.description);
    let deadline = reduce_option(task.deadline);
    println!("Task ID: {}", task.id);
    println!("Description: {}", description);
    println!("Deadline: {}", deadline);
}

pub async fn delete_task_cmd(
    config: ActixTemplateConfiguration,
    task_id: Option<String>,
) -> Result<(), DefaultError> {
    let task_id = if task_id.is_none() {
        inquire::Text::new("Enter task ID to delete:").prompt()?
    } else {
        task_id.unwrap()
    };

    let delete_response = tasks_api::delete_task(
        &config.clone().into(),
        tasks_api::DeleteTaskParams {
            task_id,
            organization: config.org_id,
        },
    )
    .await?
    .status
    .is_success();

    match delete_response {
        true => {
            println!("Task deleted successfully");
            Ok(())
        }
        false => {
            println!("Task could not be deleted");
            Err(DefaultError::new("Task could not be deleted"))
        }
    }
}