use libnoise::*;
use super::libnoise_gens::*;
use splines::{Interpolation, Key, Spline};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;


pub fn named_seed(useed: u64, name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    return useed ^ hasher.finish();
}

#[derive(Clone)]
pub struct DimensionNoise {
    gen_cont : Arc<dyn Noise2 + Send + Sync>,
    gen_density : Arc<dyn Noise3 + Send + Sync>,

    spline_cont : Arc<Spline<f64, f64>>
}

const SMOOTHNESS_FACTOR : f64 = 1.0 / 20.0;
const DENSITY_SQUASH : f64 = 0.5;
const HEIGHT_SCALE : f64 = 100.0;
impl DimensionNoise {
    pub fn new(useed: u64) -> DimensionNoise {
        let continentalness_generator = Source::simplex(named_seed(useed, "continentalness"))
            .fbm(5, 0.013, 2.0, 0.5)
            .scale([SMOOTHNESS_FACTOR; 2]);

        // uncomment these two lines to make an image of the noise
        // let path = "output.png";
        // Visualizer::<2>::new([1024, 1024], &continentalness_generator).write_to_file(path).unwrap();

        
        let density_generator = Source::simplex(named_seed(useed, "density"))
            .lambda_point(|p, d| { d - (DENSITY_SQUASH * p[1]) })
            .scale([SMOOTHNESS_FACTOR; 3]);
        
        
        let spline_cont = Spline::from_vec(vec![
            Key::new(-1.0, -1.0, Interpolation::Linear),
            Key::new(-0.5, -0.9, Interpolation::Linear),
            Key::new(-0.3, -0.2, Interpolation::Linear),
            Key::new( 0.0,  0.2, Interpolation::Linear),
            Key::new( 0.3,  0.4, Interpolation::Linear),
            Key::new( 0.7,  0.9, Interpolation::Linear),
            Key::new( 1.0,  1.0, Interpolation::Linear)
        ]);

        DimensionNoise {
            gen_cont: Arc::new(continentalness_generator),
            gen_density: Arc::new(density_generator),

            spline_cont: Arc::new(spline_cont)
        }
    }

    pub fn get_density(&self, x : i32, y: i32, z: i32) -> f64 {
        self.gen_density.sample([x as f64, y as f64, z as f64])
    }

    pub fn get_raw_cont(&self, x : i32, z: i32) -> f64 {
        self.gen_cont.sample([x as f64, z as f64])
    }

    pub fn get_splined_cont(&self, x : i32, z: i32) -> f64 {
        self.spline_cont.sample(self.get_raw_cont(x, z)).expect("Raw Continentalness outside [-1, 1]") * HEIGHT_SCALE
    }
}