// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::controller;

pub fn serve() {
    rocket::ignite()
        .mount("/", routes![controller::home::home])
        .mount("/", routes![controller::health::health])
        .mount("/", routes![controller::ready::ready])
        .launch();
}
