// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::db;

pub fn migrate() {
    db::db::tables();
    db::db::changes();
}
