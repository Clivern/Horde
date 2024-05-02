// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::controller;

/// Run the application
pub async fn run() {
    let _ = rocket::build()
        .mount("/", routes![controller::home::home])
        .mount("/", routes![controller::health::health])
        .mount("/", routes![controller::ready::ready])
        .mount("/", routes![controller::email::generate_random_email])
        .mount("/", routes![controller::message::get_email_messages])
        .launch()
        .await;
}
