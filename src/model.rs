use crate::schema::{race_entrants, races, users};
use chrono::naive::NaiveDate;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use juniper::{FieldResult, GraphQLEnum};

pub(crate) struct Context {
    pub(crate) db: MysqlConnection,
}

impl juniper::Context for Context {}

pub(crate) struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: i32) -> FieldResult<Option<User>> {
        use self::users::dsl::*;
        Ok(users.find(id).first(&context.db).optional()?)
    }

    fn username(context: &Context, name: String) -> FieldResult<Option<User>> {
        use self::users::dsl::*;
        Ok(users
            .filter(self::users::dsl::name.eq(name))
            .first(&context.db)
            .optional()?)
    }

    fn race(context: &Context, id: i32) -> FieldResult<Option<Race>> {
        use self::races::dsl::*;
        Ok(races.find(id).first(&context.db).optional()?)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation>;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
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
        Ok(race_entrants
            .filter(user_id.eq(self.id))
            .load(&context.db)?)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub(crate) struct UserName {
    pub(crate) name: String,
}

impl UserName {
    pub(crate) fn get_or_insert(self, conn: &MysqlConnection) -> anyhow::Result<i32> {
        use self::users::dsl::*;
        diesel::insert_or_ignore_into(users)
            .values(self.clone())
            .execute(conn)?;

        Ok(users.select(id).filter(name.eq(self.name)).first(conn)?)
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
        Ok(race_entrants
            .filter(race_id.eq(self.id))
            .load(&context.db)?)
    }
}

#[derive(Debug, Clone, Identifiable, Insertable, Queryable)]
#[primary_key(race_id, user_id)]
pub(crate) struct RaceEntrant {
    pub(crate) race_id: i32,
    pub(crate) user_id: i32,
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

#[juniper::object(Context = Context)]
impl RaceEntrant {
    fn race_id(&self) -> i32 {
        self.race_id
    }

    fn race(&self, context: &Context) -> FieldResult<Race> {
        use self::races::dsl::*;
        Ok(races.find(self.race_id).first(&context.db)?)
    }

    fn user_id(&self) -> i32 {
        self.user_id
    }

    fn user(&self, context: &Context) -> FieldResult<User> {
        use self::users::dsl::*;
        Ok(users.find(self.user_id).first(&context.db)?)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, GraphQLEnum)]
pub(crate) enum Reason {
    Dns,
    Dnf,
    Dsq,
}
