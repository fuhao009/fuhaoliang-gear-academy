```shell
cargo new --lib pebbles-game 

cargo new --lib io
cargo test

cargo build -r

gear --dev --rpc-port 9944

https://idea.gear-tech.io/programs?node=ws://localhost:9944

Program ID: 0xf133d604dfb74ca6fa4ec143ecdb6bbfa408cf8e3cd4796074ec5ad3d766f852
```

### 游戏规则：

1. **玩家**：游戏有两个玩家：
    - **用户**：玩家。
    - **程序**：电脑

2. **初始设置**：
    - 游戏开始时有 \( N \) 个石子。示例中，\( N \) 为 15。

3. **游戏玩法**：
    - 第一个玩家随机选择。
    - 在每个玩家的回合，他们必须从总数中移除 1 到 \( K \) 个石子。例如，如果 \( K \) 为 2，则玩家每次可以移除 1 或 2 个石子。

4. **获胜条件**：
    - 拿到最后一个（或多个）石子的玩家获胜。


简单模式（DifficultyLevel::Easy）：机器人随机选择要移除的卵石数量。
困难模式（DifficultyLevel::Hard）：机器人使用策略确保剩余的卵石数量是 K+1 的倍数，以增加程序获胜的概率。

### 参考文档

```
https://docs.gear.rs/gstd/index.html
https://wiki.gear-tech.io/docs/examples/Gaming/rock-paper-scissors
https://docs.gear.rs/gtest/index.html
https://docs.gear.rs/gtest/struct.Program.html#method.send
https://docs.gear.rs/gtest/struct.Log.html
```



### Project Structure

```
pebbles-game
├── io
│   ├── src
│   │   └── lib.rs
│   └── Cargo.toml
├── src
│   └── lib.rs
├── tests
│   └── basic.rs
├── Cargo.lock
├── Cargo.toml
└── build.rs
```



