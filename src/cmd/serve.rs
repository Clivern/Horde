// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::controller;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

#[derive(Debug)]
pub struct ApiKey(String);

#[derive(Debug)]
pub struct BasicAuth(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let keys: Vec<_> = request
            .headers()
            .iter()
            .map(|x| x.name().to_string())
            .collect();
        if keys.contains(&"x-api-key".to_owned()) {
            let value = request.headers().get("x-api-key").next().unwrap();
            return Outcome::Success(ApiKey(value.to_string()));
        }

        Outcome::Failure((Status::BadRequest, ()))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let auth = request.headers().get_one("Authorization");

        match auth {
            Some(value) => {
                return Outcome::Success(BasicAuth(value.to_owned()));
            }
            None => {}
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

pub fn serve() {
    rocket::ignite()
        .mount("/", routes![controller::home::home])
        .mount("/", routes![controller::health::health])
        .mount("/", routes![controller::ready::ready])
        .mount("/", routes![controller::project::create_project])
        .mount("/", routes![controller::project::delete_project])
        .mount("/", routes![controller::project::get_project])
        .mount("/", routes![controller::lock::lock])
        .mount("/", routes![controller::lock::unlock])
        .mount("/", routes![controller::state::get_state])
        .mount("/", routes![controller::state::update_state])
        .mount("/", routes![controller::state::delete_state])
        .launch();
}
