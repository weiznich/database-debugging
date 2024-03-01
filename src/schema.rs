// @generated automatically by Diesel CLI.

diesel::table! {
    principals (tconst, ordering, nconst) {
        tconst -> Int4,
        ordering -> Int4,
        nconst -> Int4,
        category -> Varchar,
        job -> Nullable<Varchar>,
        characters -> Nullable<Varchar>,
    }
}
