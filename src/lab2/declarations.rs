//This file includes types, constants, and static variables used throughout the program

pub const MIN_ARGS: usize = 2;
pub const MAX_ARGS: usize = 3;
pub const PROG_NAME: usize = 0;
pub const CONFIG_FILE: usize = 1;
pub const WHINGE_MODE: usize = 2;

pub const ERR_CMD_LINE: u8= 1;
pub const ERR_SCRIPT_GEN: u8= 2;

use std::sync::atomic::AtomicBool;
pub static WHINGE_ON: AtomicBool = AtomicBool::new(false);
