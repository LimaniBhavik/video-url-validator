# Video URL Validator

[![Crates.io](https://img.shields.io/crates/v/video-url-validator)](https://crates.io/crates/video-url-validator)
[![Documentation](https://docs.rs/video-url-validator/badge.svg)](https://docs.rs/video-url-validator)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A Rust crate for validating video URLs from popular platforms.

## Supported Platforms

- 🎬 **YouTube** - youtube.com, youtu.be
- 📘 **Facebook** - facebook.com/videos  
- 🎭 **Vimeo** - vimeo.com
- 🎪 **DailyMotion** - dailymotion.com, dai.ly
- 🎯 **Wistia** - wistia.com

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
video-url-validator = "0.1.0"
```

## Quick Start
```rust
use video_url_validator::{VideoUrlValidator, VideoPlatform};

let validator = VideoUrlValidator::new();

// Validate specific platform
if validator.validate_youtube_video_url("https://youtube.com/watch?v=dQw4w9WgXcQ") {
    println!("Valid YouTube URL!");
}

// Auto-detect platform  
match validator.validate_video_url("https://vimeo.com/123456789") {
    Some(platform) => println!("Valid {} URL!", platform.as_str()),
    None => println!("Invalid or unsupported video URL"),
}

// Batch validation
let urls = ["https://youtube.com/watch?v=abc", "https://vimeo.com/123"];
let results = validator.validate_multiple(&urls);
```

## API Reference

### Core Methods
 - validate_youtube_video_url(url: &str) -> bool
 - validate_facebook_video_url(url: &str) -> bool
 - validate_vimeo_video_url(url: &str) -> bool
 - validate_dailymotion_video_url(url: &str) -> bool
 - validate_wistia_video_url(url: &str) -> bool
 - validate_video_url(url: &str) -> Option<VideoPlatform>
 - is_valid_video_url(url: &str) -> bool
 - validate_multiple(urls: &[&str]) -> Vec<(String, Option<VideoPlatform>)>

### Utility Functions
 - extract_youtube_id(url: &str) -> Option<String>
 - extract_vimeo_id(url: &str) -> Option<String>
 - normalize_url(url: &str) -> String

## Examples

### Run the example:
```bash
cargo run --example basic_usage
```

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_youtube_validation
```

## License

MIT License
Copyright 2025 BhavikLimani

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
