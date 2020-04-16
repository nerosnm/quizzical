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
use log::info;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::{db::models::*, DbConn};

/// Root mount point for routes in this module.
pub const MOUNT_POINT: &str = "/teams";

/// Return a list of the routes to mount from this module.
pub fn routes() -> Vec<Route> {
    routes![new_team, get_teams, delete_team]
}

/// Create a new team.
#[post("/", format = "json", data = "<team>")]
fn new_team(conn: DbConn, team: Json<NewTeam>) -> Json<Team> {
    use crate::db::schema::teams;

    info!("Creating new team with name {}", team.name);

    let team = diesel::insert_into(teams::table)
        .values(&team.0)
        .get_result(&*conn)
        .expect("Error saving new post");

    Json(team)
}

/// Return a list of all the teams.
#[get("/")]
fn get_teams(conn: DbConn) -> Json<Vec<Team>> {
    use crate::db::schema::teams::dsl::*;

    info!("Retrieving a list of all teams");

    let results = teams.load::<Team>(&*conn).expect("Error loading teams");

    Json(results)
}

/// Delete an existing team.
#[delete("/<team_id>")]
fn delete_team(conn: DbConn, team_id: i32) -> Json<Result<(), String>> {
    use crate::db::schema::teams::dsl::*;

    info!("Deleting team with ID {}", team_id);

    let result = diesel::delete(teams.filter(id.eq(team_id)))
        .execute(&*conn)
        .map(|_| ())
        .map_err(|e| format!("Unable to delete team: {}", e));

    Json(result)
}
