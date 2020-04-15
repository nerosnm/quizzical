//  main.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-15.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

#![feature(proc_macro_hygiene, decl_macro)]
#![allow(clippy::unit_arg, clippy::let_unit_value)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use rocket_contrib::json::Json;

use zical::db::models::*;

#[database("zical")]
struct DbConn(diesel::PgConnection);

embed_migrations!();

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_launch(
            "Run Migrations",
            |rocket| {
                DbConn::get_one(&rocket).and_then(|conn| embedded_migrations::run(&*conn).ok());
            },
        ))
        .mount("/", routes![healthcheck, get_teams, new_team])
        .launch();
}

/// Server health check.
///
/// Always returns `200 OK` with no content when app is running.
#[get("/healthcheck")]
fn healthcheck() {
}

/// Return a list of all the teams.
#[get("/teams")]
fn get_teams(conn: DbConn) -> Json<Vec<Team>> {
    use zical::db::schema::teams::dsl::*;

    let results = teams.load::<Team>(&*conn).expect("Error loading teams");

    Json(results)
}

/// Create a new team.
#[post("/teams", format = "json", data = "<team>")]
fn new_team(conn: DbConn, team: Json<NewTeam>) -> Json<Team> {
    use zical::db::schema::teams;

    let team = diesel::insert_into(teams::table)
        .values(&team.0)
        .get_result(&*conn)
        .expect("Error saving new post");

    Json(team)
}
