// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::db::migrate;
use crate::util;

/// Truncate the database
pub fn truncate() {
    migrate::down(util::config::get_db_path().as_str());
}
