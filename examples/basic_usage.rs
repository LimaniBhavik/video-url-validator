use video_url_validator::{VideoPlatform, VideoUrlValidator};

fn main() {
    let validator = VideoUrlValidator::new();

    // Example URLs to test
    let test_urls = [
        "https://youtube.com/watch?v=dQw4w9WgXcQ",
        "https://www.youtube.com/watch?v=abc123&t=30s",
        "https://youtu.be/xyz789",
        "https://vimeo.com/123456789",
        "https://www.facebook.com/user/videos/987654321",
        "https://dailymotion.com/video/x7abc123",
        "https://dai.ly/x7def456",
        "https://company.wistia.com/medias/abc123def456",
        "https://invalid-url.com/video",
    ];

    println!("Video URL Validation Examples\n");
    println!("{:-<60}", "");

    for url in &test_urls {
        match validator.validate_video_url(url) {
            Some(platform) => {
                println!("✅ {} -> {}", platform.as_str(), url);

                // Show platform-specific validation
                match platform {
                    VideoPlatform::YouTube => {
                        if let Some(id) = video_url_validator::extract_youtube_id(url) {
                            println!("   Video ID: {}", id);
                        }
                    }
                    VideoPlatform::Vimeo => {
                        if let Some(id) = video_url_validator::extract_vimeo_id(url) {
                            println!("   Video ID: {}", id);
                        }
                    }
                    _ => {}
                }
            }
            None => {
                println!("❌ Invalid/Unsupported -> {}", url);
            }
        }
        println!();
    }

    // Demonstrate batch validation
    println!("\nBatch Validation Results:");
    println!("{:-<60}", "");

    let results = validator.validate_multiple(&test_urls);
    let valid_count = results
        .iter()
        .filter(|(_, platform)| platform.is_some())
        .count();

    println!("Total URLs: {}", results.len());
    println!("Valid URLs: {}", valid_count);
    println!("Invalid URLs: {}", results.len() - valid_count);

    // Show supported platforms
    println!("\nSupported Platforms:");
    println!("{:-<60}", "");
    for platform in validator.supported_platforms() {
        println!("• {}", platform.as_str());
    }
}
