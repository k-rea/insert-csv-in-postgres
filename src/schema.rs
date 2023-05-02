// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Int4,
        family_name -> Nullable<Varchar>,
        first_name -> Varchar,
        age -> Int4,
    }
}
