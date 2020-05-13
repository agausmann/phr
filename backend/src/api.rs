use crate::model::{Race, RaceEntrant, Reason, User};
use anyhow::Context as _;
use chrono::naive::NaiveDate;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use juniper::FieldResult;

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation>;

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) db: Option<Pool<ConnectionManager<MysqlConnection>>>,
}

impl Context {
    pub(crate) fn new(db_url: &str) -> anyhow::Result<Context> {
        Ok(Context {
            db: Some(Pool::new(ConnectionManager::new(db_url))?),
        })
    }

    pub(crate) fn without_database() -> Context {
        Context { db: None }
    }

    fn db(&self) -> anyhow::Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
        Ok(self
            .db
            .as_ref()
            .context("not connected to a database")?
            .get()?)
    }
}

impl juniper::Context for Context {}

pub(crate) struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: i32) -> FieldResult<Option<User>> {
        let db = context.db()?;
        use crate::schema::users::dsl::users;
        Ok(users.find(id).first(&db).optional()?)
    }

    fn username(context: &Context, name: String) -> FieldResult<Option<User>> {
        let db = context.db()?;
        use crate::schema::users::dsl::{self, users};
        Ok(users.filter(dsl::name.eq(name)).first(&db).optional()?)
    }

    fn race(context: &Context, id: i32) -> FieldResult<Option<Race>> {
        let db = context.db()?;
        use crate::schema::races::dsl::races;
        Ok(races.find(id).first(&db).optional()?)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn entries(&self, context: &Context) -> FieldResult<Vec<RaceEntrant>> {
        let db = context.db()?;
        use crate::schema::race_entrants::dsl::{race_entrants, user_id};
        Ok(race_entrants.filter(user_id.eq(self.id)).load(&db)?)
    }
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
        let db = context.db()?;
        use crate::schema::race_entrants::dsl::{race_entrants, race_id};
        Ok(race_entrants.filter(race_id.eq(self.id)).load(&db)?)
    }
}

#[juniper::object(Context = Context)]
impl RaceEntrant {
    fn race_id(&self) -> i32 {
        self.race_id
    }

    fn race(&self, context: &Context) -> FieldResult<Race> {
        let db = context.db()?;
        use crate::schema::races::dsl::races;
        Ok(races.find(self.race_id).first(&db)?)
    }

    fn user_id(&self) -> i32 {
        self.user_id
    }

    fn user(&self, context: &Context) -> FieldResult<User> {
        let db = context.db()?;
        use crate::schema::users::dsl::users;
        Ok(users.find(self.user_id).first(&db)?)
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
