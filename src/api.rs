use crate::model::{Race, RaceEntrant, Reason, User};
use chrono::naive::NaiveDate;
use diesel::prelude::*;
use juniper::FieldResult;

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation>;

pub(crate) struct Context {
    pub(crate) db: MysqlConnection,
}

impl juniper::Context for Context {}

pub(crate) struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn user(context: &Context, id: i32) -> FieldResult<Option<User>> {
        use crate::schema::users::dsl::*;
        Ok(users.find(id).first(&context.db).optional()?)
    }

    fn username(context: &Context, name: String) -> FieldResult<Option<User>> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(crate::schema::users::dsl::name.eq(name))
            .first(&context.db)
            .optional()?)
    }

    fn race(context: &Context, id: i32) -> FieldResult<Option<Race>> {
        use crate::schema::races::dsl::*;
        Ok(races.find(id).first(&context.db).optional()?)
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
        use crate::schema::race_entrants::dsl::*;
        Ok(race_entrants
            .filter(user_id.eq(self.id))
            .load(&context.db)?)
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
        use crate::schema::race_entrants::dsl::*;
        Ok(race_entrants
            .filter(race_id.eq(self.id))
            .load(&context.db)?)
    }
}

#[juniper::object(Context = Context)]
impl RaceEntrant {
    fn race_id(&self) -> i32 {
        self.race_id
    }

    fn race(&self, context: &Context) -> FieldResult<Race> {
        use crate::schema::races::dsl::*;
        Ok(races.find(self.race_id).first(&context.db)?)
    }

    fn user_id(&self) -> i32 {
        self.user_id
    }

    fn user(&self, context: &Context) -> FieldResult<User> {
        use crate::schema::users::dsl::*;
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
