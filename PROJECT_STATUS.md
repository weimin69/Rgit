# PROJECT STATUS

## Current Stage

Stage 1 — Git Object Database

当前目标是理解并实现 Git object database 的最小版本。

项目不会完整复刻 Git，而是优先实现 Git 最核心、最有学习价值的功能。

## Completed

- [x] 创建 Rust 项目
- [x] 建立基础目录结构
- [x] 实现 CLI 入口
- [x] 实现 `Rgit init`
- [x] 手动验证 `Rgit init` 生成的 `.git` 目录结构
- [x] 实现 `Rgit hash-object <file>` 计算 blob object id

## In Progress

当前正在实现 Git loose object 写入。

重点概念：

- blob object
- Git object header
- SHA-1 object id
- zlib compression
- loose object storage

## Next Step

扩展：

```bash
Rgit hash-object -w <file>
```

目标：

- 读取文件内容
- 构造 Git blob 数据：`blob <size>\0<content>`
- 计算 SHA-1
- 使用 zlib 压缩 object 数据
- 写入 `.git/objects/xx/yyyy...`
- 继续打印 object id

同时理解：

> Git object id 决定 object 的存储路径，object 内容会被压缩后作为 loose object 保存。

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
