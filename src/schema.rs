table! {
    use diesel::sql_types::*;
    use crate::model::*;

    races (id) {
        id -> Integer,
        date -> Date,
        track -> Text,
        laps -> Nullable<Integer>,
        minutes -> Nullable<Integer>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::model::*;

    race_entrants (race_id, user_id) {
        race_id -> Integer,
        user_id -> Integer,
        position -> Nullable<Integer>,
        vehicle -> Nullable<Text>,
        time -> Nullable<Integer>,
        best_lap -> Nullable<Integer>,
        lap -> Nullable<Integer>,
        reason -> Nullable<ReasonMapping>,
        ping -> Nullable<Integer>,
        fps -> Nullable<Integer>,
        fps_locked -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::model::*;

    users (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(race_entrants -> races (race_id));
joinable!(race_entrants -> users (user_id));

allow_tables_to_appear_in_same_query!(
    races,
    race_entrants,
    users,
);
