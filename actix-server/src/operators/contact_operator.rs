use crate::{
    data::models::{Contact, Deal, DealContact, PgPool},
    errors::ServiceError,
    prefixes::{ContactPrefix, DealPrefix, OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_contact_query(
    org_id: PrefixedUuid<OrgPrefix>,
    first_name: String,
    last_name: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_contact = Contact::from_details(org_id, first_name, last_name);
    let contact = diesel::insert_into(contacts_columns::contacts)
        .values(&new_contact)
        .get_result::<Contact>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating contact".to_string()))?;
    Ok(contact)
}

pub async fn delete_contact_query(
    contact_id: PrefixedUuid<ContactPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(contacts_columns::contacts)
        .filter(contacts_columns::id.eq(contact_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting contact".to_string()))?;

    Ok(())
}

pub async fn update_contact_query(
    contact_id: PrefixedUuid<ContactPrefix>,
    first_name: Option<String>,
    last_name: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = contacts_columns::contacts.filter(contacts_columns::id.eq(contact_id));
    let updated_contact = diesel::update(target)
        .set((
            first_name.map(|first_name| contacts_columns::first_name.eq(first_name)),
            last_name.map(|last_name| contacts_columns::last_name.eq(last_name)),
        ))
        .get_result::<Contact>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating contact".to_string()))?;
    Ok(updated_contact)
}

pub async fn get_contact_by_id_query(
    contact_id: PrefixedUuid<ContactPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let contact = contacts_columns::contacts
        .filter(contacts_columns::id.eq(contact_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(contact)
}

pub async fn get_contacts_by_deal_id_query(
    deal_id: PrefixedUuid<DealPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<PrefixedUuid<ContactPrefix>>,
) -> Result<(Vec<Contact>, i64), ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(PrefixedUuid::zero_id(ContactPrefix));
    let deal = deals_columns::deals
        .filter(deals_columns::id.eq(deal_id))
        .first::<Deal>(&mut conn)
        .await?;
    let contacts = DealContact::belonging_to(&deal)
        .inner_join(contacts_columns::contacts)
        .select(Contact::as_select())
        .filter(contacts_columns::id.gt(offset))
        .order((contacts_columns::updated_at, contacts_columns::id))
        .limit(limit)
        .load::<Contact>(&mut conn)
        .await?;
    let count = DealContact::belonging_to(&deal)
        .inner_join(contacts_columns::contacts)
        .select(contacts_columns::id)
        .count()
        .get_result::<i64>(&mut conn)
        .await?;
    Ok((contacts, count))
}

pub async fn get_contacts_by_org_id_query(
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<PrefixedUuid<ContactPrefix>>,
) -> Result<(Vec<Contact>, i64), ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(PrefixedUuid::zero_id(ContactPrefix));
    let contacts = contacts_columns::contacts
        .filter(contacts_columns::org_id.eq(org_id))
        .filter(contacts_columns::id.gt(offset))
        .order((contacts_columns::updated_at, contacts_columns::id))
        .limit(limit)
        .load::<Contact>(&mut conn)
        .await?;
    let count = contacts_columns::contacts
        .filter(contacts_columns::org_id.eq(org_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;
    println!("offset: {:?}", offset);
    Ok((contacts, count))
}
