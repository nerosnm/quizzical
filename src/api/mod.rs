//  api/mod.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-16.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

//! Controller logic for the Zical API.
//!
//! Each module under `api` represents a different root level mount-point for endpoints, such as
//! [`teams`][teams] for `/teams`. Every module's mount-point is described by its `MOUNT_POINT`
//! value, and its list of routes by `routes()`.
//!
//! [teams]: ./teams/index.html

pub mod teams;

use rocket::{Rocket, Route};

/// Ignite a [`Rocket`][rocket], attach endpoints and return it.
///
/// [rocket]: ../../rocket/struct.Rocket.html
pub fn ignite() -> Rocket {
    rocket::ignite()
        .mount(self::MOUNT_POINT, self::routes())
        .mount(teams::MOUNT_POINT, teams::routes())
}

/// Root mount point for routes in this module.
pub const MOUNT_POINT: &str = "/";

/// Return a list of the routes to mount from this module.
pub fn routes() -> Vec<Route> {
    routes![healthcheck]
}

/// API health check.
///
/// Always returns `200 OK` with no content when app is running.
#[get("/healthcheck")]
fn healthcheck() {
}
