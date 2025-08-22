//! Utility functions for URL processing

/// Extract video ID from YouTube URLs
pub fn extract_youtube_id(url: &str) -> Option<String> {
    use regex::Regex;

    let patterns = [
        r"youtube\.com/watch\?v=([^&]+)",
        r"youtube\.com/embed/([^/?]+)",
        r"youtube\.com/v/([^/?]+)",
        r"youtu\.be/([^/?]+)",
    ];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if let Some(captures) = regex.captures(url) {
                if let Some(id) = captures.get(1) {
                    return Some(id.as_str().to_string());
                }
            }
        }
    }
    None
}

/// Extract video ID from Vimeo URLs
pub fn extract_vimeo_id(url: &str) -> Option<String> {
    use regex::Regex;

    let pattern = r"vimeo\.com/(\d+)";
    if let Ok(regex) = Regex::new(pattern) {
        if let Some(captures) = regex.captures(url) {
            if let Some(id) = captures.get(1) {
                return Some(id.as_str().to_string());
            }
        }
    }
    None
}

/// Normalize URL by removing common variations
pub fn normalize_url(url: &str) -> String {
    let mut normalized = url.to_lowercase();

    // Remove www. prefix
    if normalized.starts_with("http://www.") {
        normalized = normalized.replace("http://www.", "http://");
    } else if normalized.starts_with("https://www.") {
        normalized = normalized.replace("https://www.", "https://");
    }

    // Remove trailing slash
    if normalized.ends_with('/') && normalized.len() > 1 {
        normalized.pop();
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VideoPlatform, VideoUrlValidator};

    #[test]
    fn test_youtube_validation() {
        let validator = VideoUrlValidator::new();

        // Valid YouTube URLs
        assert!(validator.validate_youtube_video_url("https://youtube.com/watch?v=23433"));
        assert!(validator.validate_youtube_video_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(validator.validate_youtube_video_url("https://youtu.be/dQw4w9WgXcQ"));
        assert!(validator.validate_youtube_video_url("https://youtube.com/embed/dQw4w9WgXcQ"));

        // Invalid YouTube URLs
        assert!(!validator.validate_youtube_video_url("https://youtube.com"));
        assert!(!validator.validate_youtube_video_url("https://google.com"));
        assert!(!validator.validate_youtube_video_url("not_a_url"));
    }

    #[test]
    fn test_facebook_validation() {
        let validator = VideoUrlValidator::new();

        // Valid Facebook URLs
        assert!(validator.validate_facebook_video_url("https://facebook.com/videos/123456"));
        assert!(
            validator.validate_facebook_video_url("https://www.facebook.com/user/videos/123456")
        );

        // Invalid Facebook URLs
        assert!(!validator.validate_facebook_video_url("https://facebook.com"));
        assert!(!validator.validate_facebook_video_url("https://youtube.com/watch?v=123"));
    }

    #[test]
    fn test_vimeo_validation() {
        let validator = VideoUrlValidator::new();

        // Valid Vimeo URLs
        assert!(validator.validate_vimeo_video_url("https://vimeo.com/12343432"));
        assert!(validator.validate_vimeo_video_url("https://www.vimeo.com/123456789"));

        // Invalid Vimeo URLs
        assert!(!validator.validate_vimeo_video_url("https://vimeo.com"));
        assert!(!validator.validate_vimeo_video_url("https://vimeo.com/notanumber"));
    }

    #[test]
    fn test_dailymotion_validation() {
        let validator = VideoUrlValidator::new();

        // Valid DailyMotion URLs
        assert!(validator.validate_dailymotion_video_url("https://dailymotion.com/video/x123abc"));
        assert!(validator.validate_dailymotion_video_url("https://dai.ly/x123abc"));

        // Invalid DailyMotion URLs
        assert!(!validator.validate_dailymotion_video_url("https://dailymotion.com"));
        assert!(!validator.validate_dailymotion_video_url("https://youtube.com/watch?v=123"));
    }

    #[test]
    fn test_wistia_validation() {
        let validator = VideoUrlValidator::new();

        // Valid Wistia URLs
        assert!(validator.validate_wistia_video_url("https://home.wistia.com/medias/434234231"));
        assert!(validator.validate_wistia_video_url("https://company.wistia.com/embed/abc123"));

        // Invalid Wistia URLs
        assert!(!validator.validate_wistia_video_url("https://wistia.com"));
        assert!(!validator.validate_wistia_video_url("https://youtube.com/watch?v=123"));
    }

    #[test]
    fn test_general_validation() {
        let validator = VideoUrlValidator::new();

        // Test validate_video_url method
        assert_eq!(
            validator.validate_video_url("https://youtube.com/watch?v=23433"),
            Some(VideoPlatform::YouTube)
        );
        assert_eq!(
            validator.validate_video_url("https://vimeo.com/12343432"),
            Some(VideoPlatform::Vimeo)
        );
        assert_eq!(
            validator.validate_video_url("https://invalid-site.com/video"),
            None
        );

        // Test is_valid_video_url method
        assert!(validator.is_valid_video_url("https://youtube.com/watch?v=23433"));
        assert!(!validator.is_valid_video_url("https://invalid-site.com/video"));
    }

    #[test]
    fn test_multiple_validation() {
        let validator = VideoUrlValidator::new();
        let urls = [
            "https://youtube.com/watch?v=23433",
            "https://vimeo.com/12343432",
            "https://invalid-site.com/video",
        ];

        let results = validator.validate_multiple(&urls);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].1, Some(VideoPlatform::YouTube));
        assert_eq!(results[1].1, Some(VideoPlatform::Vimeo));
        assert_eq!(results[2].1, None);
    }

    #[test]
    fn test_utility_functions() {
        // Test YouTube ID extraction
        assert_eq!(
            extract_youtube_id("https://youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            extract_youtube_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );

        // Test Vimeo ID extraction
        assert_eq!(
            extract_vimeo_id("https://vimeo.com/123456789"),
            Some("123456789".to_string())
        );

        // Test URL normalization
        assert_eq!(
            normalize_url("https://www.YouTube.com/Watch?v=ABC123/"),
            "https://youtube.com/watch?v=abc123"
        );
    }
}
