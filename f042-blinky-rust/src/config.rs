// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::common::{Pin, Port};

/**
 * note: set output to `0` to turn on builtin LED
 */
pub static BUILTIN_LED_PIN: Pin = Pin::new(Port::A, 5);
