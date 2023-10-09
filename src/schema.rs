// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Nullable<Integer>,
        name -> Text,
        age -> Nullable<Integer>,
    }
}
