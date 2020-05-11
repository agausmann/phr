// this is still needed because diesel devs can't be bothered to implement it
// in a backwards compatible way. ugh.
#[macro_use]
extern crate diesel;

mod model;
mod parser;
mod schema;
