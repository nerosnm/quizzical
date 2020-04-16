//  db/mod.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-15.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

//! Database functionality and types.
//!
//! This module includes Diesel-generated schema, models, and a `DbConn` type for use with Rocket.

pub mod models;
pub mod schema;

#[database("zical")]
pub struct DbConn(diesel::PgConnection);

impl std::fmt::Debug for DbConn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DbConn")
    }
}
