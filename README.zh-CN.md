# Encoding Toolbox

[English](./README.md)

[![CI](https://github.com/Tinkora/encoding_toolbox/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/encoding_toolbox/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)

一个面向开发者和 AI Agent 的本地优先编码、摘要与 HMAC 工具箱。自动化场景可使用
具有确定性输出的 CLI，交互操作可使用完全在浏览器本地运行的 WebAssembly 页面。

[打开浏览器工具](https://tinkora.github.io/encoding_toolbox/) | [下载 Release](https://github.com/Tinkora/encoding_toolbox/releases)

## 为什么需要它

Agent 工作流经常需要在两个工具之间执行一个很小但必须可靠的步骤，例如解码 Base64
载荷、校验下载文件，或者在不上传敏感内容的情况下生成 HMAC。Encoding Toolbox 将这些
操作保留在本地，并为自动化提供稳定的命令输出。

## 功能

| 操作 | 算法 |
| --- | --- |
| 编码 / 解码 | Base64、无填充 Base64URL、Base32、Base32Hex、Hex |
| 摘要 | SHA-256、SHA-384、SHA-512、BLAKE3、兼容旧系统的 MD5 |
| HMAC | HMAC-SHA-256、HMAC-SHA-512 |

- CLI 支持 stdin 或文件输入、稳定退出码和可选的 JSON 输出。
- 浏览器文本与本地文件操作通过 WASM 共用同一个 Rust core。
- CLI 可输出任意二进制解码结果，浏览器可下载二进制结果。
- 不上传数据，不使用遥测、Cookie、浏览器存储、CDN 或第三方运行时资源。

MD5 仅用于比对旧系统校验值。它不具备抗碰撞能力，不能用于新的安全敏感设计。

## CLI 快速开始

从源码构建：

```bash
git clone https://github.com/Tinkora/encoding_toolbox.git
cd encoding_toolbox
cargo build --release -p encoding_toolbox_cli
```

编码与解码：

```bash
printf 'hello' | target/release/tinkora-encoding encode --algorithm base64
printf 'aGVsbG8=' | target/release/tinkora-encoding decode --algorithm base64
```

计算文件摘要并获取带版本的 JSON 结果：

```bash
target/release/tinkora-encoding digest --algorithm sha256 ./artifact.bin
target/release/tinkora-encoding --json digest --algorithm blake3 ./artifact.bin
```

通过环境变量读取 HMAC key，避免它出现在命令历史和进程参数中：

```bash
export TOOLBOX_HMAC_KEY='replace-me'
printf 'message' | target/release/tinkora-encoding hmac \
  --algorithm sha256 \
  --key-env TOOLBOX_HMAC_KEY
unset TOOLBOX_HMAC_KEY
```

成功的 JSON 输出使用 schema version 1：

```json
{"schema_version":1,"operation":"digest","algorithm":"sha256","result":"..."}
```

解码结果可能是任意字节，因此 `decode` 不能与 `--json` 同时使用。错误会以
`error [CODE]: message` 写入 stderr；参数错误退出码为 2，运行错误退出码为 1。

## 浏览器工具

线上工具只在浏览器本地处理输入。本地运行方式：

```bash
cargo install wasm-pack --version 0.15.0 --locked
cd crates/encoding_toolbox_web
npm ci
npm run prepare:wasm
npm run serve
```

打开 `http://127.0.0.1:4197/static/`。

## 限制与安全边界

| 边界 | 上限 |
| --- | ---: |
| CLI stdin 或文件输入 | 100 MiB |
| 浏览器文本输入 | 1 MiB |
| 浏览器本地文件输入 | 20 MiB |
| 解码操作接受的编码文本 | 8 MiB |

这些限制用于约束内存占用；当前版本不提供流式转换。校验值相同只能帮助发现意外变化，
不能证明文件安全或来源可信。需要确认真实性时，应同时验证签名或构建来源证明。

## 开发

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p encoding_toolbox_web --target wasm32-unknown-unknown --locked
wasm-pack test --node crates/encoding_toolbox_web --locked

cd crates/encoding_toolbox_web
npm ci
npm test
```

PR 流程见 [CONTRIBUTING.md](./CONTRIBUTING.md)，私下报告安全问题的方式见
[SECURITY.md](./SECURITY.md)。

## 许可证

MIT，详见 [LICENSE](./LICENSE)。
