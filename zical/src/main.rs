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

//! Zical is a backend server for running virtual pub quizzes.

#![feature(proc_macro_hygiene, decl_macro)]
#![allow(clippy::unit_arg, clippy::let_unit_value)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod api;
pub mod db;

use diesel_migrations::embed_migrations;

use db::DbConn;

embed_migrations! {}

fn main() {
    api::ignite()
        .attach(DbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_launch(
            "Run Migrations",
            |rocket| {
                DbConn::get_one(&rocket).and_then(|conn| embedded_migrations::run(&*conn).ok());
            },
        ))
        .launch();
}
