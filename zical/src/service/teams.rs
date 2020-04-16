//  service/teams.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-16.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

//! Service for working with teams.

use diesel::{prelude::*, PgConnection};
use log::warn;

use crate::db::{
    models::{NewTeam, Team},
    schema::{teams, teams::dsl::*},
};

pub fn new_team(conn: &PgConnection, team: &NewTeam) -> Team {
    diesel::insert_into(teams::table)
        .values(team)
        .get_result(conn)
        .expect("Unable to create new team")
}

pub fn get_teams(conn: &PgConnection) -> Vec<Team> {
    teams.load::<Team>(&*conn).expect("Error loading teams")
}

pub fn search_teams(conn: &PgConnection, search: &str) -> Vec<Team> {
    teams
        .filter(name.ilike(format!("%{}%", search)))
        .load(&*conn)
        .expect("Error searching teams")
}

pub fn get_team(conn: &PgConnection, team_id: i32) -> Option<Team> {
    let result = teams
        .find(team_id)
        .limit(1)
        .load::<Team>(&*conn)
        .expect("Error loading teams");

    if result.is_empty() {
        warn!("No team found with ID {}", team_id);
        None
    } else {
        result.first().cloned()
    }
}

pub fn delete_team(conn: &PgConnection, team_id: i32) -> Option<i32> {
    get_team(conn, team_id).and_then(|_| {
        diesel::delete(teams.filter(id.eq(team_id)))
            .execute(&*conn)
            .map(|_| Some(team_id))
            .unwrap_or_else(|e| panic!("Unable to delete team: {}", e))
    })
}
