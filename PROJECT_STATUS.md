# PROJECT STATUS

## Current Stage

Stage 0 — Project Initialization

当前目标是创建一个使用 Rust 编写的最小 Git 实现。

项目不会完整复刻 Git，而是优先实现 Git 最核心、最有学习价值的功能。

## Completed

- [ ] 创建 Rust 项目
- [ ] 建立基础目录结构
- [ ] 实现 CLI 入口

## In Progress

当前正在建立项目基础结构，并准备理解 Git repository 的基本组成。

重点概念：

- `.git` 目录
- objects
- refs
- HEAD
- index
- working tree

## Next Step

实现第一个命令：

```bash
rust-git init
```

目标：

- 创建 `.git/`
- 创建 `.git/objects/`
- 创建 `.git/refs/heads/`
- 创建 `.git/HEAD`

同时理解：

> 一个 Git repository 初始化之后，磁盘上到底创建了什么？

## Architecture Notes

初始结构保持简单：

```text
src/
├── main.rs
├── cli.rs
├── repository.rs
└── commands/
    ├── mod.rs
    └── init.rs
```

后续只在真正需要时增加模块。

## Open Questions

- Git object 模块应该如何组织？
- index 是否自己实现二进制格式？
- 是否追求与真实 Git repository 兼容？
- remote push 最终实现到什么程度？

这些问题暂时不需要立即决定。

## Technical Debt

暂无。
