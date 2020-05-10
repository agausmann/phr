use crate::schema::{race_entrants, races, users};
use chrono::naive::NaiveDate;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;
use std::io::Write;

#[derive(Debug, Clone, Identifiable, Queryable)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug, Clone, Identifiable, Queryable)]
struct Race {
    id: i32,
    date: NaiveDate,
    track: String,
    laps: Option<i32>,
    minutes: Option<i32>,
}

#[derive(Debug, Clone, Associations, Identifiable, Queryable)]
#[belongs_to(Race)]
#[primary_key(race_id, user_id)]
struct RaceEntrant {
    race_id: i32,
    user_id: i32,
    position: Option<i32>,
    vehicle: Option<String>,
    time: Option<i32>,
    best_lap: Option<i32>,
    lap: Option<i32>,
    reason: Option<Reason>,
    ping: Option<i32>,
    fps: Option<i32>,
    fps_locked: bool,
}

#[derive(Debug, Clone)]
enum Reason {
    Dns,
    Dnf,
    Dsq,
}

impl<DB> FromSql<Integer, DB> for Reason
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Reason::Dns),
            2 => Ok(Reason::Dnf),
            3 => Ok(Reason::Dsq),
            x => Err(format!("unrecognized variant {}", x).into()),
        }
    }
}

impl<DB> ToSql<Integer, DB> for Reason
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        match self {
            Reason::Dns => 1,
            Reason::Dnf => 2,
            Reason::Dsq => 3,
        }
        .to_sql(out)
    }
}
