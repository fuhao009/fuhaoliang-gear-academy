#![no_std]

use gstd::{msg, prelude::*};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
}

#[no_mangle]
extern "C" fn handle() {

}

#[no_mangle]
extern "C" fn handle_reply(){
}

#[no_mangle]
extern "C" fn state() {

}
