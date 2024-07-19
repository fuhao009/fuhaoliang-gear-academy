use gstd::{prelude::*};
use gtest::{Log, Program, System};
use pebbles_game_io::{GameState, DifficultyLevel, PebblesInit, PebblesAction,PebblesEvent,Player};

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
            // 简单是随机值,困难需要手动填值
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        },
    );

    // 确认初始化成功
    assert!(!result.main_failed());

    let res = program.send(user_id, PebblesAction::Turn(3));
    assert!(!result.main_failed());

    // 检查玩家操作的结果
    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesAction::Turn(3));
    assert!(res.contains(&log));

    // 检查游戏状态
    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesEvent::CounterTurn(3));
    assert!(res.contains(&log));

    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesEvent::InvalidTurn);
    assert!(res.contains(&log));

    // 检查游戏状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 9);

    // 玩家胜利
    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesEvent::Won(Player::User));
    println!("Expected log: {:?}", log);


    // 玩家放弃游戏 系统胜利
    let res = program.send(user_id, PebblesAction::GiveUp);
    assert!(!res.main_failed());
    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesEvent::Won(Player::Program));
    assert!(res.contains(&log));


    // 重启游戏
    let res = program.send(
        user_id,
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 20,
            max_pebbles_per_turn: 5,
        },
    );
    assert!(!res.main_failed());

    // 检查重启后的游戏状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_count, 20);
    assert_eq!(state.max_pebbles_per_turn, 5);
    assert_eq!(state.pebbles_remaining, 20);
    assert!(state.winner.is_none());

    let default_difficulty = DifficultyLevel::default(); // 由于 #[default] 属性，这将是 DifficultyLevel::Easy
    println!("{:?}", default_difficulty); // 输出：Easy

    let hard_difficulty = DifficultyLevel::Hard;
    println!("{:?}", hard_difficulty); // 输出：Hard
}

#[test]
fn test_hard_difficulty() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let user_id: u64 = 100001;

    // 初始化游戏（10个石子，每次最多3个石子，困难难度）
    let result = program.send(
        user_id,
        PebblesInit {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        },
    );

    // 确认初始化成功
    assert!(!result.main_failed());

    // 模拟用户移除2个石子
    let res = program.send(user_id, PebblesAction::Turn(2));
    assert!(!res.main_failed());
    let res = program.send(user_id, PebblesAction::Turn(1));
    assert!(!res.main_failed());
    let res = program.send(user_id, PebblesAction::Turn(2));
    assert!(!res.main_failed());
    // 检查游戏状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 8);

    // 检查程序的回合
    let log = Log::builder().source(program.id()).dest(user_id).payload(PebblesEvent::CounterTurn(3));
    assert!(res.contains(&log));

    // 检查游戏状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 5);
}
