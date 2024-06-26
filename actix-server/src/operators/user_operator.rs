use crate::{
    data::models::{PgPool, Task, TaskUser, User},
    errors::ServiceError,
    operators::api_key_operator::hash_api_key,
    prefixes::{PrefixedUuid, TaskPrefix, UserPrefix},
};
use actix_web::web;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn get_user_by_id_query(
    user_id: &PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::users::dsl as users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let user: User = users_columns::users
        .filter(users_columns::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::BadRequest(format!(
                "Error loading user for get_user_by_id_query: {}",
                e
            ))
        })?;

    Ok(user)
}

#[tracing::instrument(skip(pg_pool))]
pub async fn create_user_query(
    user_id: PrefixedUuid<UserPrefix>,
    email: String,
    name: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::users::dsl as users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let user = User::from_details_with_id(user_id, email, name);

    let user = diesel::insert_into(users_columns::users)
        .values(&user)
        .get_result::<User>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::BadRequest(format!("Error creating user for create_user_query: {}", e))
        })?;

    Ok(user)
}

#[tracing::instrument(skip(pg_pool))]
pub async fn get_user_from_api_key_query(
    api_key: &str,
    pg_pool: web::Data<PgPool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::api_keys::dsl as api_keys_columns;
    use crate::data::schema::users::dsl as users_columns;

    let blake3_hash = hash_api_key(api_key);

    let mut conn = pg_pool.get().await.unwrap();

    let user: User = users_columns::users
        .inner_join(api_keys_columns::api_keys)
        .filter(api_keys_columns::blake3_hash.eq(blake3_hash))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::BadRequest(format!(
                "Error loading user for get_user_from_api_key_query: {}",
                e
            ))
        })?;

    Ok(user)
}

#[tracing::instrument(skip(pg_pool))]
pub async fn get_user_by_email_query(
    email: &str,
    pg_pool: web::Data<PgPool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::users::dsl as users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let user: User = users_columns::users
        .filter(users_columns::email.eq(email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::BadRequest(format!(
                "Error loading user for get_user_by_email_query: {}",
                e
            ))
        })?;

    Ok(user)
}

pub async fn list_users_by_task_id_query(
    task_id: PrefixedUuid<TaskPrefix>,
    pg_pool: web::Data<PgPool>,
    offset: Option<PrefixedUuid<UserPrefix>>,
    limit: Option<i64>,
) -> Result<(Vec<User>, i64), ServiceError> {
    use crate::data::schema::tasks::dsl as tasks_columns;
    use crate::data::schema::users::dsl as users_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(PrefixedUuid::zero_id(UserPrefix));
    let task = tasks_columns::tasks
        .filter(tasks_columns::id.eq(task_id))
        .first::<Task>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    let users = TaskUser::belonging_to(&task)
        .inner_join(users_columns::users)
        .select(User::as_select())
        .filter(users_columns::id.gt(offset))
        .order((users_columns::updated_at, users_columns::id))
        .limit(limit)
        .load::<User>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error fetching user".to_string()))?;

    let count = TaskUser::belonging_to(&task)
        .inner_join(users_columns::users)
        .select(users_columns::id)
        .count()
        .get_result(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error counting deals".to_string()))?;

    Ok((users, count))
}
