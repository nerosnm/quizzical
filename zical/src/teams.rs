//  teams.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-15.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

use diesel::prelude::*;

use rocket::Route;
use rocket_contrib::json::Json;
use tracing::instrument;

use crate::{db::models::*, DbConn};

pub const MOUNT_POINT: &str = "/teams";

pub fn routes() -> Vec<Route> {
    routes![get_teams, new_team]
}

/// Return a list of all the teams.
#[instrument]
#[get("/")]
fn get_teams(conn: DbConn) -> Json<Vec<Team>> {
    use crate::db::schema::teams::dsl::*;

    let results = teams.load::<Team>(&*conn).expect("Error loading teams");

    Json(results)
}

/// Create a new team.
#[instrument]
#[post("/", format = "json", data = "<team>")]
fn new_team(conn: DbConn, team: Json<NewTeam>) -> Json<Team> {
    use crate::db::schema::teams;

    let team = diesel::insert_into(teams::table)
        .values(&team.0)
        .get_result(&*conn)
        .expect("Error saving new post");

    Json(team)
}
