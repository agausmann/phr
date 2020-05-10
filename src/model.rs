use crate::schema::{race_entrants, races, users};
use chrono::naive::NaiveDate;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;
use juniper::{FieldResult, GraphQLEnum};
use std::io::Write;

struct Context {
    db: MysqlConnection,
}

impl juniper::Context for Context {}

struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: i32) -> FieldResult<Option<User>> {
        use self::users::dsl::*;
        users
            .find(id)
            .first(&context.db)
            .optional()
            .map_err(Into::into)
    }

    fn username(context: &Context, name: String) -> FieldResult<Option<User>> {
        use self::users::dsl::*;
        users
            .filter(self::users::dsl::name.eq(name))
            .first(&context.db)
            .optional()
            .map_err(Into::into)
    }

    fn race(context: &Context, id: i32) -> FieldResult<Option<Race>> {
        use self::races::dsl::*;
        races
            .find(id)
            .first(&context.db)
            .optional()
            .map_err(Into::into)
    }
}

struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

type Schema = juniper::RootNode<'static, Query, Mutation>;

#[derive(Debug, Clone, Identifiable, Queryable)]
struct User {
    id: i32,
    name: String,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn entries(&self, context: &Context) -> FieldResult<Vec<RaceEntrant>> {
        use self::race_entrants::dsl::*;
        race_entrants
            .filter(user_id.eq(self.id))
            .load(&context.db)
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, Identifiable, Queryable)]
struct Race {
    id: i32,
    date: NaiveDate,
    track: String,
    laps: Option<i32>,
    minutes: Option<i32>,
}

#[juniper::object(Context = Context)]
impl Race {
    fn id(&self) -> i32 {
        self.id
    }

    fn date(&self) -> NaiveDate {
        self.date
    }

    fn track(&self) -> &str {
        &self.track
    }

    fn laps(&self) -> Option<i32> {
        self.laps
    }

    fn minutes(&self) -> Option<i32> {
        self.minutes
    }

    fn entrants(&self, context: &Context) -> FieldResult<Vec<RaceEntrant>> {
        use self::race_entrants::dsl::*;
        race_entrants
            .filter(race_id.eq(self.id))
            .load(&context.db)
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, Identifiable, Queryable)]
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

#[juniper::object(Context = Context)]
impl RaceEntrant {
    fn race_id(&self) -> i32 {
        self.race_id
    }

    fn race(&self, context: &Context) -> FieldResult<Race> {
        use self::races::dsl::*;
        races
            .find(self.race_id)
            .first(&context.db)
            .map_err(Into::into)
    }

    fn user_id(&self) -> i32 {
        self.user_id
    }

    fn user(&self, context: &Context) -> FieldResult<User> {
        use self::users::dsl::*;
        users
            .find(self.user_id)
            .first(&context.db)
            .map_err(Into::into)
    }

    fn position(&self) -> Option<i32> {
        self.position
    }

    fn vehicle(&self) -> Option<&str> {
        self.vehicle.as_ref().map(String::as_str)
    }

    fn time(&self) -> Option<i32> {
        self.time
    }

    fn best_lap(&self) -> Option<i32> {
        self.best_lap
    }

    fn lap(&self) -> Option<i32> {
        self.lap
    }

    fn reason(&self) -> Option<Reason> {
        self.reason
    }

    fn ping(&self) -> Option<i32> {
        self.ping
    }

    fn fps(&self) -> Option<i32> {
        self.fps
    }

    fn fps_locked(&self) -> bool {
        self.fps_locked
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, GraphQLEnum)]
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
