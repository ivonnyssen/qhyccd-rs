//! Tests for the ImageGenerator module

use qhyccd_rs::simulation::{ImageGenerator, ImagePattern};

#[test]
fn test_generate_8bit() {
    let gen = ImageGenerator::default();
    let data = gen.generate_8bit(100, 100, 1);
    assert_eq!(data.len(), 10000);
}

#[test]
fn test_generate_16bit() {
    let gen = ImageGenerator::default();
    let data = gen.generate_16bit(100, 100, 1);
    assert_eq!(data.len(), 20000); // 2 bytes per pixel
}

#[test]
fn test_starfield_pattern() {
    let gen = ImageGenerator::new(ImagePattern::StarField);
    let data = gen.generate_16bit(200, 200, 1);
    assert_eq!(data.len(), 80000);
}

#[test]
fn test_multi_channel() {
    let gen = ImageGenerator::default();
    let data = gen.generate_8bit(100, 100, 3);
    assert_eq!(data.len(), 30000); // 3 channels
}

#[test]
fn test_flat_pattern_8bit() {
    let gen = ImageGenerator::new(ImagePattern::Flat);
    let data = gen.generate_8bit(100, 100, 1);
    assert_eq!(data.len(), 10000);

    // Flat pattern should have relatively uniform values (within noise)
    let base = (1000u16 >> 8) as u8; // default base_level >> 8
    let mut sum: u64 = 0;
    for &val in &data {
        sum += val as u64;
    }
    let avg = (sum / data.len() as u64) as u8;
    // Average should be close to base level
    assert!((avg as i16 - base as i16).abs() < 20);
}

#[test]
fn test_flat_pattern_16bit() {
    let gen = ImageGenerator::new(ImagePattern::Flat);
    let data = gen.generate_16bit(100, 100, 1);
    assert_eq!(data.len(), 20000);

    // Flat pattern should have relatively uniform values around base_level
    let mut sum: u64 = 0;
    for i in (0..data.len()).step_by(2) {
        let val = u16::from_le_bytes([data[i], data[i + 1]]);
        sum += val as u64;
    }
    let avg = sum / 10000;
    // Average should be close to default base level (1000)
    assert!((avg as i64 - 1000).abs() < 500);
}

#[test]
fn test_test_pattern_8bit() {
    let gen = ImageGenerator::new(ImagePattern::TestPattern);
    let data = gen.generate_8bit(256, 256, 1);
    assert_eq!(data.len(), 65536);

    // Test pattern has checkerboard - check that we have both light and dark blocks
    let mut has_light = false;
    let mut has_dark = false;
    for &val in &data {
        if val > 150 {
            has_light = true;
        }
        if val < 100 {
            has_dark = true;
        }
    }
    assert!(has_light);
    assert!(has_dark);
}

#[test]
fn test_test_pattern_16bit() {
    let gen = ImageGenerator::new(ImagePattern::TestPattern);
    let data = gen.generate_16bit(256, 256, 1);
    assert_eq!(data.len(), 131072); // 256 * 256 * 2 bytes

    // Test pattern has checkerboard - verify range of values
    let mut min_val = u16::MAX;
    let mut max_val = u16::MIN;
    for i in (0..data.len()).step_by(2) {
        let val = u16::from_le_bytes([data[i], data[i + 1]]);
        min_val = min_val.min(val);
        max_val = max_val.max(val);
    }
    // Should have significant contrast between light and dark blocks
    assert!(max_val - min_val > 20000);
}

#[test]
fn test_starfield_8bit() {
    let gen = ImageGenerator::new(ImagePattern::StarField);
    let data = gen.generate_8bit(200, 200, 1);
    assert_eq!(data.len(), 40000);

    // Starfield should have some bright stars
    let max_val = *data.iter().max().unwrap();
    let min_val = *data.iter().min().unwrap();
    // Should have contrast between background and stars
    assert!(max_val - min_val > 50);
}

#[test]
fn test_gradient_8bit() {
    let gen = ImageGenerator::new(ImagePattern::Gradient);
    let data = gen.generate_8bit(100, 100, 1);
    assert_eq!(data.len(), 10000);

    // Gradient should be brighter on the right than on the left
    // Compare left column average vs right column average
    let mut left_sum: u32 = 0;
    let mut right_sum: u32 = 0;
    for y in 0..100 {
        left_sum += data[y * 100] as u32;
        right_sum += data[y * 100 + 99] as u32;
    }
    assert!(right_sum > left_sum);
}

#[test]
fn test_with_noise_level() {
    // Test with zero noise
    let gen = ImageGenerator::new(ImagePattern::Flat).with_noise_level(0.0);
    let data = gen.generate_8bit(100, 100, 1);

    // With zero noise, all pixels should be nearly identical
    let first_val = data[0];
    let mut max_diff = 0i16;
    for &val in &data {
        let diff = (val as i16 - first_val as i16).abs();
        max_diff = max_diff.max(diff);
    }
    // All values should be identical with zero noise
    assert_eq!(max_diff, 0);
}

#[test]
fn test_with_base_level() {
    let gen = ImageGenerator::new(ImagePattern::Flat)
        .with_noise_level(0.0)
        .with_base_level(32768);
    let data = gen.generate_16bit(10, 10, 1);

    // All pixels should be at base level with zero noise
    for i in (0..data.len()).step_by(2) {
        let val = u16::from_le_bytes([data[i], data[i + 1]]);
        assert_eq!(val, 32768);
    }
}

#[test]
fn test_zero_noise() {
    // Zero noise should produce deterministic output for flat pattern
    let gen = ImageGenerator::new(ImagePattern::Flat).with_noise_level(0.0);
    let data1 = gen.generate_8bit(50, 50, 1);
    let data2 = gen.generate_8bit(50, 50, 1);

    // Both should be identical with zero noise
    assert_eq!(data1, data2);
}

#[test]
fn test_multi_channel_16bit() {
    let gen = ImageGenerator::default();
    let data = gen.generate_16bit(100, 100, 3);
    assert_eq!(data.len(), 60000); // 100 * 100 * 3 channels * 2 bytes
}

#[test]
fn test_image_pattern_default() {
    let pattern = ImagePattern::default();
    assert_eq!(pattern, ImagePattern::Gradient);
}

#[test]
fn test_noise_level_clamping() {
    // Test that noise level is clamped to [0.0, 1.0]
    let gen = ImageGenerator::default().with_noise_level(2.0);
    // Should be clamped to 1.0, generator should still work
    let data = gen.generate_8bit(10, 10, 1);
    assert_eq!(data.len(), 100);

    let gen2 = ImageGenerator::default().with_noise_level(-1.0);
    // Should be clamped to 0.0
    let data2 = gen2.generate_8bit(10, 10, 1);
    assert_eq!(data2.len(), 100);
}
