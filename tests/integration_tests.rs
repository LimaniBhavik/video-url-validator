use video_url_validator::{VideoPlatform, VideoUrlValidator};

#[test]
fn test_all_platforms_integration() {
    let validator = VideoUrlValidator::new();

    // Test each platform
    let test_cases = [
        (
            "https://youtube.com/watch?v=abc123",
            Some(VideoPlatform::YouTube),
        ),
        ("https://vimeo.com/123456789", Some(VideoPlatform::Vimeo)),
        (
            "https://facebook.com/videos/123456",
            Some(VideoPlatform::Facebook),
        ),
        (
            "https://dailymotion.com/video/x7abc123",
            Some(VideoPlatform::DailyMotion),
        ),
        (
            "https://company.wistia.com/medias/abc123",
            Some(VideoPlatform::Wistia),
        ),
        ("https://invalid-site.com/video", None),
    ];

    for (url, expected) in &test_cases {
        assert_eq!(validator.validate_video_url(url), *expected);
    }
}

#[test]
fn test_validator_default() {
    let validator = VideoUrlValidator::default();
    assert!(validator.is_valid_video_url("https://youtube.com/watch?v=abc123"));
}

#[test]
fn test_batch_validation() {
    let validator = VideoUrlValidator::new();
    let urls = ["https://youtube.com/watch?v=abc", "https://invalid.com"];
    let results = validator.validate_multiple(&urls);

    assert_eq!(results.len(), 2);
    assert!(results[0].1.is_some());
    assert!(results[1].1.is_none());
}
