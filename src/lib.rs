//! # Video URL Validator
//!
//! A Rust crate for validating video URLs from popular platforms including:
//! - YouTube
//! - Facebook
//! - Vimeo
//! - DailyMotion
//! - Wistia
//!
//! ## Example
//!
//! ```rust
//! use video_url_validator::VideoUrlValidator;
//!
//! let validator = VideoUrlValidator::new();
//!
//! // Validate YouTube URL
//! assert!(validator.validate_youtube_video_url("https://youtube.com/watch?v=23433"));
//!
//! // Validate any supported platform
//! assert!(validator.is_valid_video_url("https://vimeo.com/12343432"));
//! `

pub mod error;
pub mod utils;

pub use error::ValidationError;
pub use utils::*;

use regex::Regex;
use std::collections::HashMap;

/// Supported video platforms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VideoPlatform {
    YouTube,
    Facebook,
    Vimeo,
    DailyMotion,
    Wistia,
}

impl VideoPlatform {
    /// Get the platform name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoPlatform::YouTube => "YouTube",
            VideoPlatform::Facebook => "Facebook",
            VideoPlatform::Vimeo => "Vimeo",
            VideoPlatform::DailyMotion => "DailyMotion",
            VideoPlatform::Wistia => "Wistia",
        }
    }
}

/// Main validator struct for video URLs
pub struct VideoUrlValidator {
    patterns: HashMap<VideoPlatform, Regex>,
}

impl VideoUrlValidator {
    /// Create a new VideoUrlValidator instance
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // YouTube patterns - supports multiple URL formats
        let youtube_pattern = Regex::new(
            r"^https?://(www\.)?(youtube\.com/(watch\?v=|embed/|v/)|youtu\.be/)[\w-]+(&[\w=&-]*)?$",
        )
        .expect("Invalid YouTube regex pattern");
        patterns.insert(VideoPlatform::YouTube, youtube_pattern);

        // Facebook video patterns
        let facebook_pattern = Regex::new(
            r"^https?://(www\.)?facebook\.com/(.*/)?(video\.php\?v=|videos/)[\d]+/?(\?.*)?$",
        )
        .expect("Invalid Facebook regex pattern");
        patterns.insert(VideoPlatform::Facebook, facebook_pattern);

        // Vimeo patterns
        let vimeo_pattern = Regex::new(r"^https?://(www\.)?vimeo\.com/[\d]+/?(\?.*)?$")
            .expect("Invalid Vimeo regex pattern");
        patterns.insert(VideoPlatform::Vimeo, vimeo_pattern);

        // DailyMotion patterns
        let dailymotion_pattern =
            Regex::new(r"^https?://(www\.)?(dailymotion\.com/video/|dai\.ly/)[\w-]+/?(\?.*)?$")
                .expect("Invalid DailyMotion regex pattern");
        patterns.insert(VideoPlatform::DailyMotion, dailymotion_pattern);

        // Wistia patterns
        let wistia_pattern =
            Regex::new(r"^https?://([^.]+\.)?wistia\.com/(medias|embed)/[\w-]+/?(\?.*)?$")
                .expect("Invalid Wistia regex pattern");
        patterns.insert(VideoPlatform::Wistia, wistia_pattern);

        Self { patterns }
    }

    /// Validate a YouTube video URL
    pub fn validate_youtube_video_url(&self, url: &str) -> bool {
        self.patterns
            .get(&VideoPlatform::YouTube)
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }

    /// Validate a Facebook video URL
    pub fn validate_facebook_video_url(&self, url: &str) -> bool {
        self.patterns
            .get(&VideoPlatform::Facebook)
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }

    /// Validate a Vimeo video URL
    pub fn validate_vimeo_video_url(&self, url: &str) -> bool {
        self.patterns
            .get(&VideoPlatform::Vimeo)
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }

    /// Validate a DailyMotion video URL
    pub fn validate_dailymotion_video_url(&self, url: &str) -> bool {
        self.patterns
            .get(&VideoPlatform::DailyMotion)
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }

    /// Validate a Wistia video URL
    pub fn validate_wistia_video_url(&self, url: &str) -> bool {
        self.patterns
            .get(&VideoPlatform::Wistia)
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }

    /// Validate any video URL and return the detected platform
    pub fn validate_video_url(&self, url: &str) -> Option<VideoPlatform> {
        for (platform, regex) in &self.patterns {
            if regex.is_match(url) {
                return Some(platform.clone());
            }
        }
        None
    }

    /// Check if a URL is a valid video URL from any supported platform
    pub fn is_valid_video_url(&self, url: &str) -> bool {
        self.validate_video_url(url).is_some()
    }

    /// Get all supported platforms
    pub fn supported_platforms(&self) -> Vec<VideoPlatform> {
        self.patterns.keys().cloned().collect()
    }

    /// Validate multiple URLs at once
    pub fn validate_multiple(&self, urls: &[&str]) -> Vec<(String, Option<VideoPlatform>)> {
        urls.iter()
            .map(|url| (url.to_string(), self.validate_video_url(url)))
            .collect()
    }
}

impl Default for VideoUrlValidator {
    fn default() -> Self {
        Self::new()
    }
}
