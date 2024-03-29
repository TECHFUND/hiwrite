diesel::table! {
    category (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    modules (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        category_uuid -> Nullable<Varchar>,
        title -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    pages (uuid) {
        uuid -> Varchar,
        page_name -> Varchar,
        page_url -> Varchar,
        page_title -> Varchar,
        time_created -> Timestamp,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Varchar,
        username -> Varchar,
        password -> Varchar,
        token -> Nullable<Varchar>,
    }
}

diesel::joinable!(category -> pages (page_uuid));
diesel::joinable!(modules -> category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));

diesel::allow_tables_to_appear_in_same_query!(category, modules, pages, users,);
