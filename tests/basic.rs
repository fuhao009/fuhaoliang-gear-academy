use gstd::{prelude::*};
use gtest::{Log, Program, System};
use pebbles_game_io::{GameState,DifficultyLevel, PebblesInit, PebblesAction};

#[test]
fn test_pebbles_game() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let user_id: u64 = 100001;

    // 初始化游戏（15个石子，每次最多3个石子，简单难度）
    let result = program.send(
        user_id,
        PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        },
    );

    // 确认初始化成功
    assert!(!result.main_failed());

    let res = program.send(user_id, PebblesAction::Turn(3));
    assert!(!result.main_failed());

    // 检查玩家操作的结果
    let log = Log::builder().source(program.id()).dest(user_id).payload_bytes("PONG");
    assert!(res.contains(&log));
    assert!(res.log().is_empty());
}
