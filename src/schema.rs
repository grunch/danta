// @generated automatically by Diesel CLI.

diesel::table! {
    attendees (id) {
        id -> Integer,
        hash -> Text,
        preimage -> Text,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        data1 -> Text,
        paid -> Bool,
        created_at -> Timestamp,
    }
}
