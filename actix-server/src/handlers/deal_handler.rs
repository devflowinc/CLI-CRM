use super::auth_handler::OwnerMember;
use crate::{
    data::models::{Contact, Deal, DealContact, PgPool},
    operators::{
        contact_operator::get_contacts_by_deal_id_query,
        deal_operator::{
            create_contact_for_deal_query, create_deal_query, delete_contact_from_deal_query,
            delete_deal_query, get_deal_by_id_query, list_deals_by_org_id_query, update_deal_query,
        },
    },
    prefixes::{ContactPrefix, DealPrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateDealReqPayload {
    size: Option<f32>,
    name: Option<String>,
    active: Option<bool>,
}

#[utoipa::path(
  post,
  path = "/deals",
  context_path = "/api",
  tag = "deals",
  request_body(content = CreateDealReqPayload, description = "JSON request payload to create a new deal", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the deal that was created", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("Organization" = String, Header, description = "The org id to use for the request"),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_deal(
    req_payload: web::Json<CreateDealReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal = create_deal_query(
        org_user.org_id,
        req_payload.name.clone(),
        req_payload.size,
        req_payload.active.unwrap_or_default(),
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Created().json(deal))
}

#[utoipa::path(
  delete,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 204, description = "No content response indicating that the deal was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_deal(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<DealPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    delete_deal_query(deal_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 200, description = "JSON object representing the requested deal", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_deal(
    path: web::Path<PrefixedUuid<DealPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    match get_deal_by_id_query(deal_id, pg_pool).await {
        Ok(deal) => Ok(HttpResponse::Ok().json(deal)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateDealReqPayload {
    name: Option<String>,
    size: Option<f32>,
    active: Option<bool>,
}

#[utoipa::path(
  put,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  request_body(content = UpdateDealReqPayload, description = "JSON request payload to update the deal", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed deal", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_deal(
    req_payload: web::Json<UpdateDealReqPayload>,
    path: web::Path<PrefixedUuid<DealPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    let deal = update_deal_query(
        deal_id,
        req_payload.name.clone(),
        req_payload.size,
        req_payload.active,
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Ok().json(deal))
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum DealResType {
    Contact,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum DealResource {
    Contact(DealContact),
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum DealResourceList {
    Contact(Vec<Contact>),
}

#[utoipa::path(
  post,
  path = "/deals/{deal_id}/{resource_type}/{resource_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 200, description = "Object representing the renamed relationship", body = DealResource),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("resource_type" = DealResType, description = "The resource type to use for the request"),
    ("resource_id" = String, description = "The resource id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_deal_resource(
    path: web::Path<(PrefixedUuid<DealPrefix>, DealResType, String)>,
    _org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (deal_id, resource, resource_id) = path.into_inner();
    match resource {
        DealResType::Contact => {
            let contact_id = PrefixedUuid::<ContactPrefix>::from_str(&resource_id)?;
            let deal_contact = create_contact_for_deal_query(deal_id, contact_id, pg_pool).await?;
            Ok(HttpResponse::Ok().json(DealResource::Contact(deal_contact)))
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListDealResourceQuery {
    limit: Option<i64>,
    offset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DealResourceListWithPagination {
    pub data: DealResourceList,
    pub total: i64,
}

#[utoipa::path(
  get,
  path = "/deals/{deal_id}/{resource_type}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 200, description = "List of objects of resource_type", body = DealResourceListWithPagination),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("resource_type" = DealResType, description = "The resource type to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("limit" = Option<i64>, Query, description = "The number of records to return"),
    ("offset" = Option<String>, Query, description = "The UUID of the record to start from"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn list_deal_resource(
    path: web::Path<(PrefixedUuid<DealPrefix>, DealResType)>,
    query: web::Query<ListDealResourceQuery>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (deal_id, resource) = path.into_inner();
    let ListDealResourceQuery { limit, offset } = query.into_inner();
    match resource {
        DealResType::Contact => {
            let offset = PrefixedUuid::<ContactPrefix>::from_optional_str(offset)?;
            let (contacts, total) =
                get_contacts_by_deal_id_query(deal_id, pg_pool, limit, offset).await?;
            Ok(HttpResponse::Ok().json(DealResourceListWithPagination {
                data: DealResourceList::Contact(contacts),
                total,
            }))
        }
    }
}

#[utoipa::path(
  delete,
  path = "/deals/{deal_id}/{resource_type}/{resource_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 204, description = "No content response indicating that the deal resource was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("resource_type" = DealResType, description = "The resource type to use for the request"),
    ("resource_id" = String, description = "The resource id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn delete_deal_resource(
    path: web::Path<(PrefixedUuid<DealPrefix>, DealResType, String)>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (deal_id, resource, resource_id) = path.into_inner();
    match resource {
        DealResType::Contact => {
            let contact_id = PrefixedUuid::<ContactPrefix>::from_str(&resource_id)?;
            delete_contact_from_deal_query(deal_id, contact_id, pg_pool).await?;
        }
    }
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ListDealByOrgRespBody {
    pub deals: Vec<Deal>,
    pub total: i64,
}

#[utoipa::path(
  get,
  path = "/deals/list/org",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 200, description = "List of deals", body = ListDealByOrgRespBody),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("limit" = Option<i64>, Query, description = "The number of records to return"),
    ("offset" = Option<String>, Query, description = "The UUID of the record to start from"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn list_deal_by_org(
    query: web::Query<ListDealResourceQuery>,
    pg_pool: web::Data<PgPool>,
    org_user: OwnerMember,
) -> Result<HttpResponse, actix_web::Error> {
    let offset = PrefixedUuid::<DealPrefix>::from_optional_str(query.offset.clone())?;
    let deals = list_deals_by_org_id_query(org_user.org_id, pg_pool, query.limit, offset).await?;
    Ok(HttpResponse::Ok().json(ListDealByOrgRespBody {
        deals: deals.0,
        total: deals.1,
    }))
}
