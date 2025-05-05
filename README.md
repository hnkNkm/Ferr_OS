# my_os

## 動作環境

- macOS (Apple Silicon M1/M2)
- Rust nightly + `x86_64-unknown-uefi` ターゲット
- QEMU 8.1 以上
- OVMF (別途取得: `third_party/ovmf` に `OVMF_CODE.fd`, `OVMF_VARS.fd` を配置)

## セットアップ

```bash
# Rust
rustup override set nightly
rustup target add x86_64-unknown-uefi

# 依存ツール
brew install qemu llvm         # macOS
cargo install bootimage        # UEFI 用ビルドサポート
```

## ビルド & 実行

```bash
cargo run                # ビルド → QEMU 起動
cargo run -- -nographic  # テキストモードのみ
```
