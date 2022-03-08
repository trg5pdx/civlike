//! Code for this comes from the perlin-fractal example from the bracket-noise crate
//!
//! Link: https://github.com/amethyst/bracket-lib/tree/master/bracket-noise

use crate::{xy_idx, MAPCOUNT, MAPHEIGHT, MAPWIDTH};
use bracket_lib::prelude::RandomNumberGenerator;
use bracket_noise::prelude::*;

/// Generates a 2d heightmap populated with f32 values ranging from -1 to +1
/// Used for mapping terrain tiles to positions in the map with the terrain tile being
/// determined by the value generated at the same place in the heightmap
pub fn generate_heightmap() -> Vec<f32> {
    let mut rng = RandomNumberGenerator::new();

    let mut noise = FastNoise::seeded(rng.next_u64());
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(8);
    noise.set_fractal_gain(1.0);
    noise.set_fractal_lacunarity(3.0);
    noise.set_frequency(3.0);

    let mut generated_heightmap = vec![0.0; MAPCOUNT];

    let map_height: i32 = MAPHEIGHT as i32 - 1;
    let map_width: i32 = MAPWIDTH as i32 - 1;

    for y in 0..=map_height {
        for x in 0..=map_width {
            let idx = xy_idx(x, y);
            generated_heightmap[idx] = noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);
        }
    }

    generated_heightmap
}
