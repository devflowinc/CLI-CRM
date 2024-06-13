// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Text,
        blake3_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    companies (id) {
        id -> Uuid,
        name -> Text,
        org_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    contacts (id) {
        id -> Uuid,
        org_id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    deal_contacts (id) {
        id -> Uuid,
        deal_id -> Uuid,
        contact_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    deals (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        org_id -> Uuid,
        size -> Nullable<Float4>,
        active -> Bool,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    emails (id) {
        id -> Uuid,
        email -> Text,
        org_id -> Uuid,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    invitations (id) {
        id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        organization_id -> Uuid,
        used -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        role -> Int4,
    }
}

diesel::table! {
    links (id) {
        id -> Uuid,
        link -> Text,
        org_id -> Uuid,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    notes (id) {
        id -> Uuid,
        title -> Text,
        body -> Text,
        org_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    org_users (id) {
        id -> Uuid,
        user_id -> Uuid,
        org_id -> Uuid,
        role -> Int4,
    }
}

diesel::table! {
    orgs (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    phones (id) {
        id -> Uuid,
        number -> Text,
        org_id -> Uuid,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    plans (id) {
        id -> Uuid,
        stripe_id -> Text,
        num_users -> Int4,
        num_deals -> Int4,
        price_per_month -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Uuid,
        stripe_id -> Text,
        org_id -> Uuid,
        plan_id -> Uuid,
        stripe_plan_id -> Text,
        next_billing_date -> Timestamp,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    task_deals (id) {
        id -> Uuid,
        task_id -> Uuid,
        deal_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    task_links (id) {
        id -> Uuid,
        task_id -> Uuid,
        link_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    task_users (id) {
        id -> Uuid,
        task_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        deadline -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        contact_id -> Nullable<Uuid>,
        org_id -> Uuid,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(companies -> orgs (org_id));
diesel::joinable!(contacts -> orgs (org_id));
diesel::joinable!(deal_contacts -> contacts (contact_id));
diesel::joinable!(deal_contacts -> deals (deal_id));
diesel::joinable!(deals -> orgs (org_id));
diesel::joinable!(emails -> orgs (org_id));
diesel::joinable!(invitations -> orgs (organization_id));
diesel::joinable!(links -> orgs (org_id));
diesel::joinable!(notes -> orgs (org_id));
diesel::joinable!(org_users -> orgs (org_id));
diesel::joinable!(org_users -> users (user_id));
diesel::joinable!(phones -> orgs (org_id));
diesel::joinable!(subscriptions -> orgs (org_id));
diesel::joinable!(task_deals -> deals (deal_id));
diesel::joinable!(task_deals -> tasks (task_id));
diesel::joinable!(task_links -> links (link_id));
diesel::joinable!(task_links -> tasks (task_id));
diesel::joinable!(task_users -> tasks (task_id));
diesel::joinable!(task_users -> users (user_id));
diesel::joinable!(tasks -> contacts (contact_id));
diesel::joinable!(tasks -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    companies,
    contacts,
    deal_contacts,
    deals,
    emails,
    invitations,
    links,
    notes,
    org_users,
    orgs,
    phones,
    plans,
    subscriptions,
    task_deals,
    task_links,
    task_users,
    tasks,
    users,
);
