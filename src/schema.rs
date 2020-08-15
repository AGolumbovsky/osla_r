table! {
    test_osla_diesel (id) {
        id -> Int4,
        word -> Varchar,
        part_of_speech -> Text,
        meaning -> Text,
        approved -> Bool,
    }
}
