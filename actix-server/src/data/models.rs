use super::schema::*;
use crate::prefixes::*;
use bb8_redis::{bb8, RedisConnectionManager};
use diesel::expression::ValidGrouping;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type PgPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;
pub type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(
    Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema, Identifiable,
)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "email": "developers@trieve.ai",
    "name": "Trieve Team",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = users)]
pub struct User {
    pub id: PrefixedUuid<UserPrefix>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: Option<String>,
}

impl User {
    pub fn from_details(email: String, name: Option<String>) -> Self {
        User {
            id: PrefixedUuid::create(UserPrefix),
            email: email.into(),
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
            name: name.map(|n| n.into()),
        }
    }

    pub fn from_details_with_id(
        id: PrefixedUuid<UserPrefix>,
        email: String,
        name: Option<String>,
    ) -> Self {
        User {
            id: id.into(),
            email: email.into(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema, AsChangeset,
)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "name": "Trieve Team",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = orgs)]
pub struct Org {
    pub id: PrefixedUuid<OrgPrefix>,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Org {
    pub fn from_details(name: String) -> Self {
        Org {
            id: PrefixedUuid::create(OrgPrefix),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }

    pub fn from_details_with_id(id: PrefixedUuid<OrgPrefix>, name: String) -> Self {
        Org {
            id: id.into(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "size": 4.0,
    "active": true,
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = deals)]
pub struct Deal {
    pub id: PrefixedUuid<DealPrefix>,
    pub name: Option<String>,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub size: Option<f32>,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Deal {
    pub fn from_details(
        org_id: PrefixedUuid<OrgPrefix>,
        name: Option<String>,
        size: Option<f32>,
        active: bool,
    ) -> Self {
        Deal {
            id: PrefixedUuid::create(DealPrefix),
            name,
            org_id,
            size,
            active,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Associations,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
)]
#[schema(example = json!({
    "id": "dealcontact-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "deal_id": "deal-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "contact_id": "contact-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(belongs_to(Contact))]
#[diesel(belongs_to(Deal))]
#[diesel(table_name=deal_contacts)]
pub struct DealContact {
    pub id: PrefixedUuid<DealContactPrefix>,
    pub deal_id: PrefixedUuid<DealPrefix>,
    pub contact_id: PrefixedUuid<ContactPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl DealContact {
    pub fn from_details(
        deal_id: PrefixedUuid<DealPrefix>,
        contact_id: PrefixedUuid<ContactPrefix>,
    ) -> Self {
        DealContact {
            id: PrefixedUuid::create(DealContactPrefix),
            deal_id,
            contact_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UserRole {
    Owner = 2,
    Admin = 1,
    User = 0,
}

impl From<i32> for UserRole {
    fn from(role: i32) -> Self {
        match role {
            2 => UserRole::Owner,
            1 => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Owner => 2,
            UserRole::Admin => 1,
            UserRole::User => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1",
    "user_id": "8w8w8w8w-8w8w-8w8w-8w8w-8w8w8w8w8w8w",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
}))]
#[diesel(table_name = org_users)]
pub struct OrgUserLink {
    pub id: PrefixedUuid<OrgUserPrefix>,
    pub user_id: PrefixedUuid<UserPrefix>,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub role: i32,
}
impl OrgUserLink {
    pub fn from_details(
        user_id: PrefixedUuid<UserPrefix>,
        org_id: PrefixedUuid<OrgPrefix>,
        role: UserRole,
    ) -> Self {
        OrgUserLink {
            id: PrefixedUuid::create(OrgUserPrefix),
            user_id,
            org_id,
            role: role.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "stripe_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "num_users": 4,
    "num_deals": 5,
    "price_per_month": 40,
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=plans)]
pub struct Plan {
    pub id: uuid::Uuid,
    pub stripe_id: String,
    pub num_users: i32,
    pub num_deals: i32,
    pub price_per_month: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Plan {
    pub fn from_details(
        stripe_id: String,
        num_users: i32,
        num_deals: i32,
        price_per_month: i32,
    ) -> Self {
        Plan {
            id: uuid::Uuid::new_v4(),
            stripe_id,
            num_users,
            num_deals,
            price_per_month,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, ValidGrouping, ToSchema)]
#[diesel(table_name = invitations)]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub organization_id: PrefixedUuid<OrgPrefix>,
    pub used: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub role: i32,
}

// any type that implements Into<String> can be used to create Invitation
impl Invitation {
    pub fn from_details(
        email: String,
        organization_id: PrefixedUuid<OrgPrefix>,
        role: i32,
    ) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            email,
            organization_id,
            used: false,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
            role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "afafafaf-afaf-afaf-afaf-afafafafafaf",
    "user_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "name": "my-api-key",
    "blake3_hash": "blake3-hash",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub user_id: PrefixedUuid<UserPrefix>,
    pub name: String,
    pub blake3_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl ApiKey {
    pub fn from_details(
        user_id: PrefixedUuid<UserPrefix>,
        name: String,
        blake3_hash: String,
    ) -> Self {
        ApiKey {
            id: uuid::Uuid::new_v4(),
            user_id,
            name,
            blake3_hash,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = contacts)]
pub struct Contact {
    pub id: PrefixedUuid<ContactPrefix>,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Contact {
    pub fn from_details(
        org_id: PrefixedUuid<OrgPrefix>,
        first_name: String,
        last_name: String,
    ) -> Self {
        Contact {
            id: PrefixedUuid::create(ContactPrefix),
            org_id,
            first_name,
            last_name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "note-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "title": "My Note Title",
    "body": "Note contents...",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=notes)]
pub struct Note {
    pub id: PrefixedUuid<NotePrefix>,
    pub title: String,
    pub body: String,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Note {
    pub fn from_title(title: String, org_id: PrefixedUuid<OrgPrefix>) -> Self {
        Note {
            id: PrefixedUuid::create(NotePrefix),
            title,
            body: String::new(),
            org_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema, Identifiable,
)]
#[schema(example = json!({
    "id": "link-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "link": "https://trieve.ai",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = links)]
pub struct Link {
    pub id: PrefixedUuid<LinkPrefix>,
    pub link: String,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Link {
    pub fn from_details(link: String, org_id: PrefixedUuid<OrgPrefix>) -> Self {
        Link {
            id: PrefixedUuid::create(LinkPrefix),
            org_id,
            link,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "email": "example@example.com",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
}))]
#[diesel(table_name = emails)]
pub struct Email {
    pub id: PrefixedUuid<EmailPrefix>,
    pub email: String,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Email {
    pub fn from_details(email: String, org_id: PrefixedUuid<OrgPrefix>) -> Self {
        Email {
            id: PrefixedUuid::create(EmailPrefix),
            email,
            org_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "phone": "+1234567890",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = phones)]
pub struct Phone {
    pub id: PrefixedUuid<PhonePrefix>,
    pub number: String,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Phone {
    pub fn from_details(org_id: PrefixedUuid<OrgPrefix>, number: String) -> Self {
        Self {
            id: PrefixedUuid::create(PhonePrefix),
            number,
            org_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
)]
#[schema(example = json!({
    "id": "task-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "deadline": "2021-01-01T00:00:00",
    "description": "Task description...",
    "contact_id": "contact-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: PrefixedUuid<TaskPrefix>,
    pub deadline: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub contact_id: Option<PrefixedUuid<ContactPrefix>>,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Task {
    pub fn from_details(
        org_id: PrefixedUuid<OrgPrefix>,
        deadline: Option<chrono::NaiveDateTime>,
        description: Option<String>,
        contact_id: Option<PrefixedUuid<ContactPrefix>>,
    ) -> Self {
        Self {
            id: PrefixedUuid::create(TaskPrefix),
            deadline,
            description,
            contact_id,
            org_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "company-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "name": "Company Name",
    "org_id": "org-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=companies)]
pub struct Company {
    pub id: PrefixedUuid<CompanyPrefix>,
    pub name: String,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Company {
    pub fn from_name(name: String, org_id: PrefixedUuid<OrgPrefix>) -> Self {
        Company {
            id: PrefixedUuid::create(CompanyPrefix),
            name,
            org_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Associations,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
)]
#[schema(example = json!({
    "id": "taskdeal-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "task_id": "task-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "deal_id": "deal-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(belongs_to(Task))]
#[diesel(belongs_to(Deal))]
#[diesel(table_name=task_deals)]
pub struct TaskDeal {
    pub id: PrefixedUuid<TaskDealPrefix>,
    pub task_id: PrefixedUuid<TaskPrefix>,
    pub deal_id: PrefixedUuid<DealPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TaskDeal {
    pub fn from_details(
        task_id: PrefixedUuid<TaskPrefix>,
        deal_id: PrefixedUuid<DealPrefix>,
    ) -> Self {
        TaskDeal {
            id: PrefixedUuid::create(TaskDealPrefix),
            task_id,
            deal_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
    Associations,
)]
#[schema(example = json!({
    "id": "tasklink-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "task_id": "task-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "link_id": "link-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=task_links)]
#[diesel(belongs_to(Task))]
#[diesel(belongs_to(Link))]
pub struct TaskLink {
    pub id: PrefixedUuid<TaskLinkPrefix>,
    pub task_id: PrefixedUuid<TaskPrefix>,
    pub link_id: PrefixedUuid<LinkPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TaskLink {
    pub fn from_details(
        task_id: PrefixedUuid<TaskPrefix>,
        link_id: PrefixedUuid<LinkPrefix>,
    ) -> Self {
        TaskLink {
            id: PrefixedUuid::create(TaskLinkPrefix),
            task_id,
            link_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Selectable,
    Clone,
    ToSchema,
    Identifiable,
    Associations,
)]
#[schema(example = json!({
    "id": "taskuser-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "task_id": "task-c7c7c7c7-c7c7-c7c7-c7c7-c7c7c7c7c7c7",
    "user_id": "user-b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=task_users)]
#[diesel(belongs_to(Task))]
#[diesel(belongs_to(User))]
pub struct TaskUser {
    pub id: PrefixedUuid<TaskUserPrefix>,
    pub task_id: PrefixedUuid<TaskPrefix>,
    pub user_id: PrefixedUuid<UserPrefix>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TaskUser {
    pub fn from_details(
        task_id: PrefixedUuid<TaskPrefix>,
        user_id: PrefixedUuid<UserPrefix>,
    ) -> Self {
        TaskUser {
            id: PrefixedUuid::create(TaskUserPrefix),
            task_id,
            user_id,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}
