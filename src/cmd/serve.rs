// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::controller;

pub fn serve() {
    rocket::ignite()
        .mount("/", routes![controller::home::home])
        .mount("/", routes![controller::health::health])
        .mount("/", routes![controller::ready::ready])
        .mount("/", routes![controller::project::create_project])
        .mount("/", routes![controller::project::delete_project])
        .mount("/", routes![controller::project::update_project])
        .mount("/", routes![controller::lock::lock])
        .mount("/", routes![controller::lock::unlock])
        .mount("/", routes![controller::state::get_state])
        .mount("/", routes![controller::state::update_state])
        .mount("/", routes![controller::state::delete_state])
        .launch();
}
