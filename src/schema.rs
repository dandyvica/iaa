// Define the ARTEFACT table
diesel::table! {
    artefact (id) {
        id -> Integer,
        path -> Text,
        name -> Text,
        ext -> Text,
        r#type -> Text,
        len -> BigInt,
        created -> Timestamp,
        accessed -> Timestamp,
        modified -> Timestamp,
        sha256 -> Text,
        blake3 -> Text,
        entropy -> Float,
        mime -> Text,
        metadata -> Jsonb
    }
}

// run history
diesel::table! {
    run_history (start_time) {
        start_time -> Timestamp,
        end_time -> Timestamp,
        nb_files -> BigInt,
        args -> Text,
        tags -> Text
    }
}
