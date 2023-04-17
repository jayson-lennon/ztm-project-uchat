// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (user_id, post_id) {
        user_id -> Uuid,
        post_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    boosts (post_id, user_id) {
        post_id -> Uuid,
        user_id -> Uuid,
        boosted_at -> Timestamptz,
    }
}

diesel::table! {
    followers (user_id, follows) {
        user_id -> Uuid,
        follows -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    poll_choices (id) {
        id -> Uuid,
        choice -> Text,
        post_id -> Uuid,
    }
}

diesel::table! {
    poll_votes (user_id, post_id) {
        user_id -> Uuid,
        post_id -> Uuid,
        choice_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        user_id -> Uuid,
        content -> Jsonb,
        time_posted -> Timestamptz,
        direct_message_to -> Nullable<Uuid>,
        reply_to -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reactions (user_id, post_id) {
        user_id -> Uuid,
        post_id -> Uuid,
        created_at -> Timestamptz,
        like_status -> Int2,
        reaction -> Nullable<Jsonb>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Nullable<Text>,
        email_confirmed -> Nullable<Timestamptz>,
        password_hash -> Text,
        display_name -> Nullable<Text>,
        handle -> Text,
        created_at -> Timestamptz,
        profile_image -> Nullable<Text>,
    }
}

diesel::table! {
    web (id) {
        id -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        fingerprint -> Jsonb,
    }
}

diesel::joinable!(followers -> users (follows));
diesel::joinable!(poll_votes -> poll_choices (choice_id));
diesel::joinable!(bookmarks -> posts (post_id));
diesel::joinable!(reactions -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    boosts,
    followers,
    poll_choices,
    poll_votes,
    posts,
    reactions,
    users,
    web,
);
