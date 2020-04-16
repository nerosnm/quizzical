//  api/teams.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-15.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

//! Endpoints for working with teams.

use log::info;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::{
    db::{models::*, DbConn},
    service::teams,
};

/// Root mount point for routes in this module.
pub const MOUNT_POINT: &str = "/teams";

/// Return a list of the routes to mount from this module.
pub fn routes() -> Vec<Route> {
    routes![new_team, get_teams, get_teams_search, get_team, delete_team]
}

/// Create a new team.
#[post("/", format = "json", data = "<team>")]
fn new_team(conn: DbConn, team: Json<NewTeam>) -> Json<Team> {
    info!("Creating new team with name \"{}\"", team.name);

    Json(teams::new_team(&*conn, &*team))
}

/// Return a list of all the teams.
#[get("/")]
fn get_teams(conn: DbConn) -> Json<Vec<Team>> {
    info!("Retrieving a list of all teams");

    Json(teams::get_teams(&*conn))
}

/// Return a list of all teams whose name contains the given substring.
#[get("/?<search>")]
fn get_teams_search(conn: DbConn, search: String) -> Json<Vec<Team>> {
    info!("Searching for teams with name matching \"{}\"", search);

    Json(teams::search_teams(&*conn, &search))
}

/// Return the team with the given ID, if found.
#[get("/<id>")]
fn get_team(conn: DbConn, id: i32) -> Json<Option<Team>> {
    info!("Getting team with ID {}", id);

    Json(teams::get_team(&*conn, id))
}

/// Delete an existing team.
///
/// Returns the ID of the deleted team if successful.
#[delete("/<id>")]
fn delete_team(conn: DbConn, id: i32) -> Json<Option<i32>> {
    info!("Deleting team with ID {}", id);

    Json(teams::delete_team(&*conn, id))
}
