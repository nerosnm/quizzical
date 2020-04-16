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
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod db;
pub mod teams;

use diesel_migrations::embed_migrations;

embed_migrations! {}

#[database("zical")]
pub struct DbConn(diesel::PgConnection);

impl std::fmt::Debug for DbConn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DbConn")
    }
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_launch(
            "Run Migrations",
            |rocket| {
                DbConn::get_one(&rocket).and_then(|conn| embedded_migrations::run(&*conn).ok());
            },
        ))
        .mount("/", routes![healthcheck])
        .mount(teams::MOUNT_POINT, teams::routes())
        .launch();
}

/// Server health check.
///
/// Always returns `200 OK` with no content when app is running.
#[get("/healthcheck")]
fn healthcheck() {
}
