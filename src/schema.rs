table! {
    categories (id) {
        id -> Int4,
        name -> Nullable<Text>,
        parent_id -> Nullable<Int4>,
        position -> Nullable<Int4>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
        ads -> Nullable<Text>,
        deleted_at -> Nullable<Text>,
        image -> Nullable<Text>,
        active -> Nullable<Int4>,
        slug -> Nullable<Text>,
        product_recommendation_id -> Nullable<Text>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    questions (id) {
        id -> Int4,
        header -> Nullable<Text>,
        status -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        admin_user_id -> Nullable<Text>,
        answer -> Nullable<Text>,
        further_information -> Nullable<Text>,
        experiment -> Nullable<Text>,
        exploration -> Nullable<Text>,
        sources_links -> Nullable<Text>,
        is_answer -> Nullable<Int4>,
        image -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
        related_id -> Nullable<Text>,
        delta -> Nullable<Int4>,
        of_the_day -> Nullable<Int4>,
        author_name -> Nullable<Text>,
        author_link -> Nullable<Text>,
        author_id -> Nullable<Int4>,
        link -> Nullable<Text>,
        slug -> Nullable<Text>,
        product_recommendation_id -> Nullable<Text>,
        category_id -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    posts,
    questions,
);
