//! Image generation utilities for simulated cameras

use rand::Rng;

/// Pattern type for generated images
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImagePattern {
    /// Gradient from dark to light with noise
    Gradient,
    /// Simulated star field
    StarField,
    /// Flat field with noise
    Flat,
    /// Test pattern with geometric shapes
    TestPattern,
}

impl Default for ImagePattern {
    fn default() -> Self {
        Self::Gradient
    }
}

/// Generates test images for simulated camera capture
#[derive(Debug, Clone)]
pub struct ImageGenerator {
    pattern: ImagePattern,
    noise_level: f64,
    base_level: u16,
}

impl Default for ImageGenerator {
    fn default() -> Self {
        Self {
            pattern: ImagePattern::Gradient,
            noise_level: 0.05, // 5% noise
            base_level: 1000,  // Base ADU level
        }
    }
}

impl ImageGenerator {
    /// Creates a new generator with the specified pattern
    pub fn new(pattern: ImagePattern) -> Self {
        Self {
            pattern,
            ..Default::default()
        }
    }

    /// Sets the noise level (0.0 to 1.0)
    pub fn with_noise_level(mut self, level: f64) -> Self {
        self.noise_level = level.clamp(0.0, 1.0);
        self
    }

    /// Sets the base signal level
    pub fn with_base_level(mut self, level: u16) -> Self {
        self.base_level = level;
        self
    }

    /// Generates an 8-bit image
    pub fn generate_8bit(&self, width: u32, height: u32, channels: u32) -> Vec<u8> {
        let pixel_count = (width * height) as usize;
        let total_size = pixel_count * channels as usize;
        let mut data = vec![0u8; total_size];
        let mut rng = rand::thread_rng();

        match self.pattern {
            ImagePattern::Gradient => {
                self.generate_gradient_8bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::StarField => {
                self.generate_starfield_8bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::Flat => {
                self.generate_flat_8bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::TestPattern => {
                self.generate_test_pattern_8bit(&mut data, width, height, channels, &mut rng)
            }
        }

        data
    }

    /// Generates a 16-bit image
    pub fn generate_16bit(&self, width: u32, height: u32, channels: u32) -> Vec<u8> {
        let pixel_count = (width * height) as usize;
        let total_size = pixel_count * channels as usize * 2; // 2 bytes per sample
        let mut data = vec![0u8; total_size];
        let mut rng = rand::thread_rng();

        match self.pattern {
            ImagePattern::Gradient => {
                self.generate_gradient_16bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::StarField => {
                self.generate_starfield_16bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::Flat => {
                self.generate_flat_16bit(&mut data, width, height, channels, &mut rng)
            }
            ImagePattern::TestPattern => {
                self.generate_test_pattern_16bit(&mut data, width, height, channels, &mut rng)
            }
        }

        data
    }

    fn generate_gradient_8bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let base = (self.base_level >> 8) as u8;
        let noise_range = (255.0 * self.noise_level) as i16;

        for y in 0..height {
            for x in 0..width {
                let gradient = ((x as f64 / width as f64) * 200.0) as u8;
                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value =
                    (base as i16 + gradient as i16 + noise).clamp(0, 255) as u8;

                let idx = ((y * width + x) * channels) as usize;
                for c in 0..channels as usize {
                    data[idx + c] = value;
                }
            }
        }
    }

    fn generate_gradient_16bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let noise_range = (65535.0 * self.noise_level) as i32;

        for y in 0..height {
            for x in 0..width {
                let gradient = ((x as f64 / width as f64) * 50000.0) as u16;
                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value = (self.base_level as i32 + gradient as i32 + noise)
                    .clamp(0, 65535) as u16;

                let idx = ((y * width + x) * channels) as usize * 2;
                let bytes = value.to_le_bytes();
                for c in 0..channels as usize {
                    data[idx + c * 2] = bytes[0];
                    data[idx + c * 2 + 1] = bytes[1];
                }
            }
        }
    }

    fn generate_starfield_8bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        // Fill with background noise
        let base = (self.base_level >> 8) as u8;
        let noise_range = (255.0 * self.noise_level * 0.5) as i16; // Less noise for starfield

        for i in 0..data.len() {
            let noise = if noise_range > 0 {
                rng.gen_range(-noise_range..=noise_range)
            } else {
                0
            };
            data[i] = (base as i16 + noise).clamp(0, 255) as u8;
        }

        // Add stars
        let num_stars = ((width * height) as f64 * 0.001) as usize; // ~0.1% coverage
        for _ in 0..num_stars {
            let x = rng.gen_range(1..width - 1);
            let y = rng.gen_range(1..height - 1);
            let brightness = rng.gen_range(150..255) as u8;
            let size = rng.gen_range(1..=3);

            self.draw_star_8bit(data, width, height, channels, x, y, brightness, size);
        }
    }

    fn generate_starfield_16bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        // Fill with background noise
        let noise_range = (65535.0 * self.noise_level * 0.3) as i32;

        for y in 0..height {
            for x in 0..width {
                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value = (self.base_level as i32 + noise).clamp(0, 65535) as u16;

                let idx = ((y * width + x) * channels) as usize * 2;
                let bytes = value.to_le_bytes();
                for c in 0..channels as usize {
                    data[idx + c * 2] = bytes[0];
                    data[idx + c * 2 + 1] = bytes[1];
                }
            }
        }

        // Add stars
        let num_stars = ((width * height) as f64 * 0.001) as usize;
        for _ in 0..num_stars {
            let x = rng.gen_range(2..width - 2);
            let y = rng.gen_range(2..height - 2);
            let brightness = rng.gen_range(40000..65535) as u16;
            let size = rng.gen_range(1..=3);

            self.draw_star_16bit(data, width, height, channels, x, y, brightness, size);
        }
    }

    fn draw_star_8bit(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        cx: u32,
        cy: u32,
        brightness: u8,
        size: u32,
    ) {
        for dy in 0..=size * 2 {
            for dx in 0..=size * 2 {
                let x = cx as i32 + dx as i32 - size as i32;
                let y = cy as i32 + dy as i32 - size as i32;

                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }

                let dist = (((dx as i32 - size as i32).pow(2)
                    + (dy as i32 - size as i32).pow(2)) as f64)
                    .sqrt();
                if dist <= size as f64 {
                    let falloff = 1.0 - (dist / (size as f64 + 1.0));
                    let value = (brightness as f64 * falloff) as u8;

                    let idx = ((y as u32 * width + x as u32) * channels) as usize;
                    for c in 0..channels as usize {
                        data[idx + c] = data[idx + c].saturating_add(value);
                    }
                }
            }
        }
    }

    fn draw_star_16bit(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        cx: u32,
        cy: u32,
        brightness: u16,
        size: u32,
    ) {
        for dy in 0..=size * 2 {
            for dx in 0..=size * 2 {
                let x = cx as i32 + dx as i32 - size as i32;
                let y = cy as i32 + dy as i32 - size as i32;

                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }

                let dist = (((dx as i32 - size as i32).pow(2)
                    + (dy as i32 - size as i32).pow(2)) as f64)
                    .sqrt();
                if dist <= size as f64 {
                    let falloff = 1.0 - (dist / (size as f64 + 1.0));
                    let value = (brightness as f64 * falloff) as u16;

                    let idx = ((y as u32 * width + x as u32) * channels) as usize * 2;
                    for c in 0..channels as usize {
                        let current =
                            u16::from_le_bytes([data[idx + c * 2], data[idx + c * 2 + 1]]);
                        let new_value = current.saturating_add(value);
                        let bytes = new_value.to_le_bytes();
                        data[idx + c * 2] = bytes[0];
                        data[idx + c * 2 + 1] = bytes[1];
                    }
                }
            }
        }
    }

    fn generate_flat_8bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let base = (self.base_level >> 8) as u8;
        let noise_range = (255.0 * self.noise_level) as i16;

        for y in 0..height {
            for x in 0..width {
                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value = (base as i16 + noise).clamp(0, 255) as u8;

                let idx = ((y * width + x) * channels) as usize;
                for c in 0..channels as usize {
                    data[idx + c] = value;
                }
            }
        }
    }

    fn generate_flat_16bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let noise_range = (65535.0 * self.noise_level) as i32;

        for y in 0..height {
            for x in 0..width {
                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value =
                    (self.base_level as i32 + noise).clamp(0, 65535) as u16;

                let idx = ((y * width + x) * channels) as usize * 2;
                let bytes = value.to_le_bytes();
                for c in 0..channels as usize {
                    data[idx + c * 2] = bytes[0];
                    data[idx + c * 2 + 1] = bytes[1];
                }
            }
        }
    }

    fn generate_test_pattern_8bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let noise_range = (255.0 * self.noise_level * 0.5) as i16;

        for y in 0..height {
            for x in 0..width {
                // Create a checkerboard with varying intensities
                let block_size = 64;
                let block_x = x / block_size;
                let block_y = y / block_size;
                let is_light = (block_x + block_y) % 2 == 0;

                let base = if is_light { 200u8 } else { 50u8 };

                // Add concentric circles in center
                let cx = width / 2;
                let cy = height / 2;
                let dist = (((x as i32 - cx as i32).pow(2) + (y as i32 - cy as i32).pow(2)) as f64)
                    .sqrt();
                let ring = ((dist / 50.0) as u32) % 2;
                let ring_mod = if ring == 0 { 20i16 } else { -20i16 };

                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value = (base as i16 + ring_mod + noise).clamp(0, 255) as u8;

                let idx = ((y * width + x) * channels) as usize;
                for c in 0..channels as usize {
                    data[idx + c] = value;
                }
            }
        }
    }

    fn generate_test_pattern_16bit<R: Rng>(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        rng: &mut R,
    ) {
        let noise_range = (65535.0 * self.noise_level * 0.5) as i32;

        for y in 0..height {
            for x in 0..width {
                // Create a checkerboard with varying intensities
                let block_size = 64;
                let block_x = x / block_size;
                let block_y = y / block_size;
                let is_light = (block_x + block_y) % 2 == 0;

                let base: u16 = if is_light { 50000 } else { 10000 };

                // Add concentric circles in center
                let cx = width / 2;
                let cy = height / 2;
                let dist = (((x as i32 - cx as i32).pow(2) + (y as i32 - cy as i32).pow(2)) as f64)
                    .sqrt();
                let ring = ((dist / 50.0) as u32) % 2;
                let ring_mod: i32 = if ring == 0 { 5000 } else { -5000 };

                let noise = if noise_range > 0 {
                    rng.gen_range(-noise_range..=noise_range)
                } else {
                    0
                };
                let value = (base as i32 + ring_mod + noise).clamp(0, 65535) as u16;

                let idx = ((y * width + x) * channels) as usize * 2;
                let bytes = value.to_le_bytes();
                for c in 0..channels as usize {
                    data[idx + c * 2] = bytes[0];
                    data[idx + c * 2 + 1] = bytes[1];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
