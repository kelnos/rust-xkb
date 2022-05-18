//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use xcb;
use ffi::*;
use crate::{Keycode, ModMask, LayoutMask};
use crate::{Context, Keymap, State};
use crate::keymap::compile;
use bitflags::bitflags;
use std::mem::MaybeUninit;

pub const MIN_MAJOR_XKB_VERSION: u16 = XKB_X11_MIN_MAJOR_XKB_VERSION as u16;
pub const MIN_MINOR_XKB_VERSION: u16 = XKB_X11_MIN_MINOR_XKB_VERSION as u16;

impl From<u8> for Keycode {
	fn from(value: u8) -> Self {
		Keycode(value as xkb_keycode_t)
	}
}

impl From<u8> for ModMask {
	fn from(value: u8) -> Self {
		ModMask(value as xkb_mod_mask_t)
	}
}

impl From<i16> for ModMask {
	fn from(value: i16) -> Self {
		ModMask(value as xkb_mod_mask_t)
	}
}

impl From<u8> for LayoutMask {
	fn from(value: u8) -> Self {
		LayoutMask(value as xkb_layout_mask_t)
	}
}

impl From<i16> for LayoutMask {
	fn from(value: i16) -> Self {
		LayoutMask(value as xkb_layout_mask_t)
	}
}

bitflags! {
	pub struct Flags: xkb_x11_setup_xkb_extension_flags {
		const NO_FLAGS = XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS;
	}
}

pub const NO_FLAGS: Flags = Flags::NO_FLAGS;

impl Default for Flags {
	fn default() -> Self {
		NO_FLAGS
	}
}

#[inline]
pub fn setup(connection: &xcb::Connection, major_version: u16, minor_version: u16, flags: Flags) -> Result<(u16, u16, u8, u8), ()> {
    let mut actual_major = MaybeUninit::uninit();
    let mut actual_minor = MaybeUninit::uninit();
    let mut base_event = MaybeUninit::uninit();
    let mut base_error = MaybeUninit::uninit();

    let ret = unsafe {
        xkb_x11_setup_xkb_extension(
            connection.get_raw_conn() as *mut _,
            major_version,
            minor_version,
            flags.bits(),
            actual_major.as_mut_ptr(),
            actual_minor.as_mut_ptr(),
            base_event.as_mut_ptr(),
            base_error.as_mut_ptr()
        )
    };

    if ret != 1 {
        Err(())
    } else {
        Ok(unsafe { (actual_major.assume_init(), actual_minor.assume_init(), base_event.assume_init(), base_error.assume_init()) })
    }
}

#[inline]
pub fn device(connection: &xcb::Connection) -> Result<i32, ()> {
	unsafe {
		match xkb_x11_get_core_keyboard_device_id(connection.get_raw_conn() as *mut _) {
			-1 => Err(()),
			id => Ok(id)
		}
	}
}

#[inline]
pub fn keymap(connection: &xcb::Connection, device: i32, context: &Context, flags: compile::Flags) -> Result<Keymap, ()> {
	unsafe {
		xkb_x11_keymap_new_from_device(context.as_ptr(), connection.get_raw_conn() as *mut _, device, flags.bits())
			.as_mut().map(|ptr| Keymap::from_ptr(ptr)).ok_or(())
	}
}

#[inline]
pub fn state(connection: &xcb::Connection, device: i32, keymap: &Keymap) -> Result<State, ()> {
	unsafe {
		xkb_x11_state_new_from_device(keymap.as_ptr(), connection.get_raw_conn() as *mut _, device)
			.as_mut().map(|ptr| State::from_ptr(ptr)).ok_or(())
	}
}
