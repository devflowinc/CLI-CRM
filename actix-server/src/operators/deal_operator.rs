use crate::{
    data::models::{Deal, DealContact, PgPool, Task, TaskDeal},
    errors::ServiceError,
    prefixes::{ContactPrefix, DealPrefix, OrgPrefix, PrefixedUuid, TaskPrefix},
};
use actix_web::web;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_deal_query(
    org_id: PrefixedUuid<OrgPrefix>,
    name: Option<String>,
    size: Option<f32>,
    active: bool,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_deal = Deal::from_details(org_id, name, size, active);
    let deal = diesel::insert_into(deals_columns::deals)
        .values(&new_deal)
        .get_result::<Deal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating deal".to_string()))?;
    Ok(deal)
}

pub async fn delete_deal_query(
    deal_id: PrefixedUuid<DealPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(deals_columns::deals)
        .filter(deals_columns::id.eq(deal_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting deal".to_string()))?;

    Ok(())
}

pub async fn update_deal_query(
    deal_id: PrefixedUuid<DealPrefix>,
    name: Option<String>,
    size: Option<f32>,
    active: Option<bool>,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = deals_columns::deals.filter(deals_columns::id.eq(deal_id));
    let updated_deal = diesel::update(target)
        .set((
            name.map(|name| deals_columns::name.eq(name)),
            size.map(|size| deals_columns::size.eq(size)),
            active.map(|active| deals_columns::active.eq(active)),
        ))
        .get_result::<Deal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating deal".to_string()))?;
    Ok(updated_deal)
}

pub async fn get_deal_by_id_query(
    deal_id: PrefixedUuid<DealPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let deal = deals_columns::deals
        .filter(deals_columns::id.eq(deal_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(deal)
}

pub async fn list_deals_by_task_id_query(
    task_id: PrefixedUuid<TaskPrefix>,
    pg_pool: web::Data<PgPool>,
    offset: Option<PrefixedUuid<DealPrefix>>,
    limit: Option<i64>,
) -> Result<(Vec<Deal>, i64), ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    use crate::data::schema::tasks::dsl as tasks_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(PrefixedUuid::zero_id(DealPrefix));
    let task = tasks_columns::tasks
        .filter(tasks_columns::id.eq(task_id))
        .first::<Task>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    let deals = TaskDeal::belonging_to(&task)
        .inner_join(deals_columns::deals)
        .select(Deal::as_select())
        .filter(deals_columns::id.gt(offset))
        .order((deals_columns::updated_at, deals_columns::id))
        .limit(limit)
        .load::<Deal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error fetching deals".to_string()))?;
    let count = TaskDeal::belonging_to(&task)
        .inner_join(deals_columns::deals)
        .select(deals_columns::id)
        .count()
        .get_result(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error counting deals".to_string()))?;
    Ok((deals, count))
}

pub async fn create_contact_for_deal_query(
    deal_id: PrefixedUuid<DealPrefix>,
    contact_id: PrefixedUuid<ContactPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<DealContact, ServiceError> {
    use crate::data::schema::deal_contacts::dsl as deal_contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_deal_contact = DealContact::from_details(deal_id, contact_id);
    let deal_contact = diesel::insert_into(deal_contacts_columns::deal_contacts)
        .values(&new_deal_contact)
        .get_result::<DealContact>(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::InternalServerError("Error creating deal contact".to_string())
        })?;
    Ok(deal_contact)
}

pub async fn delete_contact_from_deal_query(
    deal_id: PrefixedUuid<DealPrefix>,
    contact_id: PrefixedUuid<ContactPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::deal_contacts::dsl as deal_contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    diesel::delete(deal_contacts_columns::deal_contacts)
        .filter(deal_contacts_columns::deal_id.eq(deal_id))
        .filter(deal_contacts_columns::contact_id.eq(contact_id))
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub async fn list_deals_by_org_id_query(
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<PrefixedUuid<DealPrefix>>,
) -> Result<(Vec<Deal>, i64), ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(PrefixedUuid::zero_id(DealPrefix));
    let deals = deals_columns::deals
        .filter(deals_columns::org_id.eq(org_id))
        .filter(deals_columns::id.gt(offset))
        .order((deals_columns::updated_at, deals_columns::id))
        .limit(limit)
        .load::<Deal>(&mut conn)
        .await?;
    let count = deals_columns::deals
        .filter(deals_columns::org_id.eq(org_id))
        .count()
        .get_result(&mut conn)
        .await?;
    Ok((deals, count))
}
