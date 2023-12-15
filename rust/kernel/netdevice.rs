// SPDX-License-Identifier: GPL-2.0

//! netdevice
//!
//! C header: [`include/linux/netdevice.h`](../../../../include/linux/netdevice.h)

use crate::{bindings, c_types};

pub fn dev_queue_xmit(skb: *mut bindings::sk_buff) -> c_types::c_int {
    unsafe { bindings::dev_queue_xmit(skb) }
}
