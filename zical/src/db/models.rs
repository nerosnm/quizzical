//  db/models.rs
//  zical
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-15.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::schema::teams;

#[derive(Queryable, Serialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
}
