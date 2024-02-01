use rand::prelude::*;
use std::time::Instant;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

fn random_walk(num_steps: usize, step_size: f32) -> Point {
    let mut rng = rand::thread_rng();
    let mut current: Point = Point::new(0.0, 0.0, 0.0);
    

    for _ in 0..num_steps {
        let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let phi = rng.gen::<f32>() * std::f32::consts::PI;
        let dx: f32 = step_size * theta.cos() * phi.sin();
        let dy: f32 = step_size * theta.sin() * phi.sin();
        let dz: f32 = step_size * phi.cos();
        
        current = Point::new(
            current.x + dx as f32,
            current.y + dy as f32,
            current.z + dz as f32,
        );

    }
    current
}

fn mean_squared_displacement(k: usize, n: usize, l: f32) -> f32 {
    let mut total_msd: f32 = 0.0;

    for _ in 0..k {
        let final_point = random_walk(n, l);
        let dx: f32 = final_point.x;
        let dy: f32 = final_point.y;
        let dz: f32 = final_point.z;

        let msd: f32 = dx * dx + dy * dy + dz * dz;
        total_msd += msd;
    }

    total_msd / k as f32
}

fn parallel_mean_squared_displacement(k: usize, n: usize, l: f32) -> f32 {
    let total_msd: f32 = (0..k).into_par_iter().map(|_| {
        let final_point = random_walk(n, l);
        let dx: f32 = final_point.x;
        let dy: f32 = final_point.y;
        let dz: f32 = final_point.z;
        dx * dx + dy * dy + dz * dz
    }).sum();

    total_msd / k as f32
}

fn main() {
    let k_values = vec![10, 100, 1000];
    let n_values = vec![10000, 20000, 50000];
    let l: f32  = 1.0;

    for n in &n_values {
        for k in &k_values {
            // Record the start time
            let start_time = Instant::now();
            let msd = parallel_mean_squared_displacement(*k, *n, l);
            
            // Record the end time
            let end_time = Instant::now();

            // Calculate the elapsed time
            let elapsed_time = end_time - start_time;
        
            println!("Mean Square Displacement for k = {} replicates, num_steps = {}: {}, elapsed_time = {}",k, n, msd, elapsed_time.as_secs_f64());
        }
    }
    
}

