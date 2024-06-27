#![no_std]

use gstd::{msg, prelude::*};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    // 从消息中获取初始化参数
    let config: PebblesInit = msg::load().expect("Unable to decode PebblesInit");
    let state = GameState {
        pebbles_count: config.pebbles_count,
        max_pebbles_per_turn: config.max_pebbles_per_turn,
        pebbles_remaining: config.pebbles_count,
        difficulty: config.difficulty,
        first_player: Player::User, // 默认玩家先手
        winner: None,
    };
    unsafe {
        PEBBLES_GAME = Some(state);
    }
}

#[no_mangle]
extern "C" fn handle() {
    // 从消息中获取操作
    let action: PebblesAction = msg::load().expect("Unable to decode PebblesAction");
    unsafe {
        if let Some(ref mut state) = PEBBLES_GAME {
            match action {
                PebblesAction::Turn(count) => {
                    // 检查操作合法性
                    if count <= state.max_pebbles_per_turn && count <= state.pebbles_remaining {
                        state.pebbles_remaining -= count;
                        // 检查玩家是否赢了
                        if state.pebbles_remaining == 0 {
                            state.winner = Some(Player::User);
                            msg::reply(PebblesEvent::Won(Player::User), 0).expect("Unable to reply with event");
                        } else {
                            // 程序回合逻辑
                            let program_turn = (state.pebbles_remaining - 1) % (state.max_pebbles_per_turn + 1) + 1;
                            state.pebbles_remaining -= program_turn;
                            // 检查程序是否赢了
                            if state.pebbles_remaining == 0 {
                                state.winner = Some(Player::Program);
                                msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply with event");
                            } else {
                                msg::reply(PebblesEvent::CounterTurn(program_turn), 0).expect("Unable to reply with event");
                            }
                        }
                    }
                }
                PebblesAction::GiveUp => {
                    // 玩家放弃，程序获胜
                    state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply with event");
                }
                PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
                    // 重新开始游戏，重置状态
                    state.pebbles_count = pebbles_count;
                    state.max_pebbles_per_turn = max_pebbles_per_turn;
                    state.pebbles_remaining = pebbles_count;
                    state.difficulty = difficulty;
                    state.first_player = Player::User;
                    state.winner = None;
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn handle_reply(){
}

#[no_mangle]
extern "C" fn state() {
    // 回复当前游戏状态
    unsafe {
        if let Some(ref state) = PEBBLES_GAME {
            msg::reply(state, 0).expect("Unable to reply with state");
        }
    }
}
