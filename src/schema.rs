// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}
