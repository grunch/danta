table! {
    attendees (id) {
        id -> Integer,
        hash -> Text,
        preimage -> Text,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        paid -> Bool,
        created_at -> Timestamp,
    }
}
