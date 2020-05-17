table! {
    drivers (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    races (id) {
        id -> Integer,
        date -> Date,
        track -> Text,
        laps -> Nullable<Integer>,
        minutes -> Nullable<Integer>,
    }
}

table! {
    race_entrants (race_id, driver_id) {
        race_id -> Integer,
        driver_id -> Integer,
        position -> Nullable<Integer>,
        vehicle -> Nullable<Text>,
        time -> Nullable<Integer>,
        best_lap -> Nullable<Integer>,
        lap -> Nullable<Integer>,
        reason -> Nullable<crate::model::ReasonMapping>,
        ping -> Nullable<Integer>,
        fps -> Nullable<Integer>,
        fps_locked -> Bool,
    }
}

joinable!(race_entrants -> drivers (driver_id));
joinable!(race_entrants -> races (race_id));

allow_tables_to_appear_in_same_query!(
    drivers,
    races,
    race_entrants,
);
