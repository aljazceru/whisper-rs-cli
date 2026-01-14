# Whisper CLI

A fast command-line interface for Whisper speech recognition using whisper-rs.

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Transcription

Transcribe an audio file:

```bash
whisper-rs-cli transcribe audio.mp3
```

### Using Different Models

Specify a model size (tiny, base, small, medium, large):

```bash
whisper-rs-cli transcribe audio.mp3 --model small
```

### Language-Specific Models

Use English-only models for faster processing:

```bash
whisper-rs-cli transcribe audio.mp3 --model base.en
```

### Other Options

```bash
# Specify language for improved accuracy
whisper-rs-cli transcribe audio.mp3 --language es

# Transcribe with timestamps
whisper-rs-cli transcribe audio.mp3 --output-format json

# Enable debug mode (verbose output)
whisper-rs-cli transcribe audio.mp3 --debug
```

## Model Locations

The CLI searches for models in the following locations (in order):

1. `~/.cache/whispercpp/`
2. `~/.local/share/whisper`
3. `~/.local/share/pywhispercpp/models/` (legacy)
4. `./models/` (relative to current directory)

If a model is not found in any of these locations, it will be automatically downloaded to:

**`~/.local/share/whisper/`**

## Model Files

Models are stored as:
- `ggml-{model_name}.bin` (general models)
- `ggml-{model_name}.en.bin` (English-only models)
- `ggml-{model_name}.{language}.bin` (language-specific models)

Example: `ggml-base.bin`, `ggml-small.en.bin`, `ggml-medium.es.bin`

## Supported Model Sizes

- `tiny` (~39MB) - Fastest, least accurate
- `base` (~74MB) - Good balance
- `small` (~244MB) - More accurate
- `medium` (~769MB) - High accuracy
- `large` (~1.5GB) - Most accurate, slowest
