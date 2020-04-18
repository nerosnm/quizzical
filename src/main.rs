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
pub mod service;

use diesel_migrations::embed_migrations;
use log::info;
use rocket::{config::Environment, fairing::AdHoc};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use std::str::FromStr;

embed_migrations! {}

fn main() {
    api::ignite()
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_attach("Configure CORS", |rocket| {
            // Construct a CORS fairing based on Rocket's configured environment.
            let cors = cors_fairing(rocket.config().environment);
            // Attach the constructed fairing.
            Ok(rocket.attach(cors))
        }))
        .attach(AdHoc::on_launch("Run Migrations", |rocket| {
            db::DbConn::get_one(&rocket).and_then(|conn| embedded_migrations::run(&*conn).ok());
        }))
        .launch();
}

/// Construct a CORS fairing, configured partly based on the environment Rocket is being launched
/// in.
fn cors_fairing(env: Environment) -> Cors {
    // Allow FRONTEND_URL if provided. If not provided and we're in development mode, then instead
    // allow http://localhost:5000.
    let origins = std::env::var("FRONTEND_URL")
        .ok()
        .or_else(|| match env {
            Environment::Development => Some("http://localhost:5000".into()),
            _ => None,
        })
        .map(|url| vec![url])
        .unwrap_or_default();

    info!("Allowing origins: {:?}", origins);

    CorsOptions::default()
        .allowed_headers(AllowedHeaders::all())
        .allowed_methods(
            ["Get", "Post", "Delete"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allowed_origins(AllowedOrigins::some_exact(&*origins))
        .allow_credentials(true)
        .to_cors()
        .expect("Cors fairing cannot be created")
}
