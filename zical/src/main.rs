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
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Healthcheck endpoint, always returns `200 OK` with no content when app is running.
#[get("/healthcheck")]
fn healthcheck() {
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, healthcheck])
        .launch();
}
