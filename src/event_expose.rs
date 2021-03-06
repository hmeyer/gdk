// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use cairo::RectangleInt;
use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventExpose(::Event);

event_wrapper!(EventExpose, GdkEventExpose);
event_subtype!(EventExpose, Expose | Damage);

impl EventExpose {
    pub fn get_region(&self) -> RectangleInt {
        self.as_ref().area
    }

    pub fn get_count(&self) -> u32 {
        self.as_ref().count as u32
    }
}
