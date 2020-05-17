use crate::schema::{drivers, race_entrants, races};
use chrono::naive::NaiveDate;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use juniper::GraphQLEnum;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub(crate) struct Driver {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "drivers"]
pub(crate) struct DriverName {
    pub(crate) name: String,
}

impl DriverName {
    pub(crate) fn get_or_insert(self, conn: &MysqlConnection) -> anyhow::Result<i32> {
        use self::drivers::dsl::*;
        diesel::insert_or_ignore_into(drivers)
            .values(self.clone())
            .execute(conn)?;

        Ok(drivers.select(id).filter(name.eq(self.name)).first(conn)?)
    }
}

#[derive(Debug, Clone, Identifiable, Insertable, Queryable)]
pub(crate) struct Race {
    pub(crate) id: i32,
    pub(crate) date: NaiveDate,
    pub(crate) track: String,
    pub(crate) laps: Option<i32>,
    pub(crate) minutes: Option<i32>,
}

#[derive(Debug, Clone, Identifiable, Insertable, Queryable)]
#[primary_key(race_id, driver_id)]
pub(crate) struct RaceEntrant {
    pub(crate) race_id: i32,
    pub(crate) driver_id: i32,
    pub(crate) position: Option<i32>,
    pub(crate) vehicle: Option<String>,
    pub(crate) time: Option<i32>,
    pub(crate) best_lap: Option<i32>,
    pub(crate) lap: Option<i32>,
    pub(crate) reason: Option<Reason>,
    pub(crate) ping: Option<i32>,
    pub(crate) fps: Option<i32>,
    pub(crate) fps_locked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, GraphQLEnum)]
pub(crate) enum Reason {
    Dns,
    Dnf,
    Dsq,
}
