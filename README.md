# Aider Model Selector

A Rust CLI tool that helps you select and use AI models from Aider via OpenRouter.

## Features

- Fetches available models from Aider
- Interactive fuzzy search selection
- Launches Aider with selected model

## Prerequisites

- Rust installed (https://www.rust-lang.org/tools/install)
- Aider installed (https://aider.chat)
- OpenRouter API key configured in Aider
- Environment variable `OPENROUTER_API_KEY` set (or configured in Aider's settings)

## Installation

```bash
# プロジェクトをビルドしてバイナリをインストール
cargo install --path .

# インストール後、バイナリは ~/.cargo/bin/ に配置され、
# どこからでも実行可能になります
```

## Usage

1. Run the tool:
```bash
aider-model-selector
```

2. Use fuzzy search to select your preferred model from the list

3. The tool will automatically launch Aider with your selected model

## Building from Source

```bash
git clone https://github.com/kawaji-r/aider-model-selector.git
cd aider-model-selector
cargo build --release
```

The binary will be at `target/release/aider-model-selector`

## License

MIT
