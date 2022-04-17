// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use custom_error::custom_error;

custom_error! { pub TimeflakeError
    RNGError{msg: String} = "RNG Error: {msg}",
    SystemTimeError{msg: String} = "SystemTime Error: {msg}",
    MalformedData{msg: String} = "Malformed data: {msg}",
}
