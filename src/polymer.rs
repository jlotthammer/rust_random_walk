use pyo3::prelude::*;
use rand::prelude::*;
// use rand::seq::index::sample;
use rayon::prelude::*;
use std::sync::Mutex;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufWriter, Write};
use physical_constants::BOLTZMANN_CONSTANT;
use physical_constants::AVOGADRO_CONSTANT;
use rand::distributions::WeightedIndex;
// use hdf5::File;
// use hdf5::types::VarLenArray;
// use hdf5::errors::Error;

// Define a struct for a 3D coordinate
#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[pymethods]
impl Point {
    #[new]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    #[getter]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[getter]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[getter]
    pub fn z(&self) -> f32 {
        self.z
    }

}


// Define a trait for points that can compute distances
pub trait Distance {
    fn distance(&self, other: &Self) -> f32;
}

impl Distance for Point {
    fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}



// Define a struct for a polymer
#[pyclass]
#[derive(Debug)]
pub struct Polymer {
    monomer_positions: Vec<Point>,
}

#[pymethods]
impl Polymer {
    #[new]
    pub fn new() -> Self {
        Polymer { monomer_positions: Vec::new() }
    }

    pub fn add_point(&mut self, point: Point) {
        self.monomer_positions.push(point);
    }

    // WORKING CODE!!!

    // pub fn random_walk_step(&self, radius: f32, previous_position: Point) -> Point {
    //     let mut rng: ThreadRng = rand::thread_rng();
    //     loop {
    //         let phi: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    //         let theta: f32 = rng.gen_range(0.0..std::f32::consts::PI);
    //         // Calculate displacement from the previous position
    //         let dx: f32 = 2.0 * radius * theta.sin() * phi.cos();
    //         let dy: f32 = 2.0 * radius * theta.sin() * phi.sin();
    //         let dz: f32 = 2.0 * radius * theta.cos();
            
    //         // Calculate new position by adding displacement to the previous position
    //         let mut new_position: Point = Point::new(
    //             previous_position.x() + dx,
    //             previous_position.y() + dy,
    //             previous_position.z() + dz,
    //         );
            
    //         // let overlap: bool = self.monomer_positions.iter().any(|&p| new_position.distance(&p) < radius);
    //         let overlap = self.monomer_positions.par_iter().any(|&p| new_position.distance(&p) < radius);

    //         // Collision resolution
    //         if overlap {
    //         // Adjust the position slightly
    //         let adjustment_factor: f32 = 0.1; // Adjust as needed
    //         let adjustment_vector: Point = Point::new(rng.gen_range(-adjustment_factor..adjustment_factor), 
    //                                            rng.gen_range(-adjustment_factor..adjustment_factor), 
    //                                            rng.gen_range(-adjustment_factor..adjustment_factor));

    //         new_position = Point::new(new_position.x() + adjustment_vector.x(),
    //                                   new_position.y() + adjustment_vector.y(),
    //                                   new_position.z() + adjustment_vector.z());
    //         }

    //         if !overlap {
    //             return new_position;
    //         }
    //         // else {
    //         //     println!("Overlap detected, retrying")
    //         // }
    //     }
    // }
        
    // WORKING-ish, stalls at seq length ~200
    // pub fn random_walk_step(&self, radius: f32, previous_position: Point) -> Point {
    //     let mut rng: ThreadRng = rand::thread_rng();
    //     let bond_length: f32 = 2.0 * radius;
    //     loop {
    //         let phi: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    //         let theta: f32 = rng.gen_range(0.0..std::f32::consts::PI);
    //         let dx: f32 = bond_length * theta.sin() * phi.cos();
    //         let dy: f32 = bond_length * theta.sin() * phi.sin();
    //         let dz: f32 = bond_length * theta.cos();
            
    //         let proposed_position: Point = Point::new(
    //             previous_position.x() + dx,
    //             previous_position.y() + dy,
    //             previous_position.z() + dz,
    //         );
    
    //         // Check for overlap
    //         let mut overlap_point: Option<Point> = None;
    //         for &p in &self.monomer_positions {
    //             if proposed_position.distance(&p) < bond_length {
    //                 overlap_point = Some(p);
    //                 break;
    //             }
    //         }
    
    //         if let Some(overlap_point) = overlap_point {
    //             // Calculate the direction vector from the previous position to the proposed position
    //             let direction_vector: Point = Point::new(
    //                 proposed_position.x() - previous_position.x(),
    //                 proposed_position.y() - previous_position.y(),
    //                 proposed_position.z() - previous_position.z(),
    //             );
    
    //             // Normalize the direction vector
    //             let norm: f32 = (direction_vector.x().powi(2) + direction_vector.y().powi(2) + direction_vector.z().powi(2)).sqrt();
    //             let normalized_direction: Point = Point::new(
    //                 direction_vector.x() / norm,
    //                 direction_vector.y() / norm,
    //                 direction_vector.z() / norm,
    //             );
    
    //             // Adjust the proposed position along the normalized direction vector
    //             let adjusted_position: Point = Point::new(
    //                 previous_position.x() + normalized_direction.x() * bond_length,
    //                 previous_position.y() + normalized_direction.y() * bond_length,
    //                 previous_position.z() + normalized_direction.z() * bond_length,
    //             );
    
    //             // If the adjusted position is free from collisions, return it
    //             if !self.monomer_positions.iter().any(|&p| adjusted_position.distance(&p) < bond_length) {
    //                 return adjusted_position;
    //             }
    //         } else {
    //             // If there's no collision, return the proposed position
    //             return proposed_position;
    //         }
    //     }
    // }
    
    pub fn random_walk_step(&self, radius: f32, previous_position: Point) -> Point {
        let mut rng: ThreadRng = rand::thread_rng();
        let bond_length: f32 = 2.0 * radius;
        loop {
            let phi: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
            let theta: f32 = rng.gen_range(0.0..std::f32::consts::PI);
            let dx: f32 = bond_length * theta.sin() * phi.cos();
            let dy: f32 = bond_length * theta.sin() * phi.sin();
            let dz: f32 = bond_length * theta.cos();
            
            let proposed_position: Point = Point::new(
                previous_position.x() + dx,
                previous_position.y() + dy,
                previous_position.z() + dz,
            );
    
            // Check for overlap directly with the monomer positions
            let overlap: bool = self.monomer_positions.par_iter().any(|&p| proposed_position.distance(&p) < bond_length);
    
            // If there's no overlap, return the proposed position
            if !overlap {
                return proposed_position;
            }
        }
    }
    



    // pub fn propose_position(&self, previous_position: Point, bond_length: f32) -> Point {
    //     let mut rng: ThreadRng = rand::thread_rng();
    //     let phi: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    //     let theta: f32 = rng.gen_range(0.0..std::f32::consts::PI);
    //     let dx: f32 = bond_length * theta.sin() * phi.cos();
    //     let dy: f32 = bond_length * theta.sin() * phi.sin();
    //     let dz: f32 = bond_length * theta.cos();
        
    //     let proposed_position: Point = Point::new(
    //         previous_position.x() + dx,
    //         previous_position.y() + dy,
    //         previous_position.z() + dz,
    //     );

    //     proposed_position
    // }

    // pub fn random_walk_step(&self, previous_position: Point, radius: f32) -> Point {
    //     // hardcode the average bond distance between beads, 3.81 angstroms
    //     // hardcore force constant [9.6 kcal/mol/A^2] and temperature [310 K
    //     // all data comes from MpipiGG forcefield
        
    //     loop {
    //         let bond_length: f32 = sample_step_size(3.81, 9.6, 310.0);
    //         let proposed_position: Point = self.propose_position(previous_position, bond_length);
    //         let hard_sphere: f32 = 2.0 * radius;

    //         let overlap_point: Option<&Point> = self.monomer_positions.par_iter()
    //         .find_any(|&p| proposed_position.distance(p) < hard_sphere);
    
    //         if let Some(overlap_point) = overlap_point {
    //             // Calculate the direction vector from the previous position to the proposed position
    //             let direction_vector: Point = Point::new(
    //                 proposed_position.x() - previous_position.x(),
    //                 proposed_position.y() - previous_position.y(),
    //                 proposed_position.z() - previous_position.z(),
    //             );
    
    //             // Normalize the direction vector
    //             let norm: f32 = (direction_vector.x().powi(2) + direction_vector.y().powi(2) + direction_vector.z().powi(2)).sqrt();
    //             let normalized_direction: Point = Point::new(
    //                 direction_vector.x() / norm,
    //                 direction_vector.y() / norm,
    //                 direction_vector.z() / norm,
    //             );
    
    //             // Adjust the proposed position along the normalized direction vector
    //             let adjusted_position: Point = Point::new(
    //                 previous_position.x() + normalized_direction.x() * bond_length,
    //                 previous_position.y() + normalized_direction.y() * bond_length,
    //                 previous_position.z() + normalized_direction.z() * bond_length,
    //             );
    
    //             // If the adjusted position is free from collisions, return it
    //             // 6.27 is the average wang-frenkel sigma value between all monomer pairs in mpipiGG forcefield
    //             if !self.monomer_positions.par_iter().any(|&p| adjusted_position.distance(&p) < hard_sphere) {
    //                 return adjusted_position;
    //             }
    //         } else {
    //             // If there's no collision, return the proposed position
    //             return proposed_position;
    //         }
    //     }
    // }

    pub fn random_walk(&mut self, num_steps: usize, radius: f32) {
        let mut previous_position: Point = Point::new(0.0, 0.0, 0.0);
    
        (0..num_steps).for_each(|_| {
            let step: Point = self.random_walk_step(radius, previous_position);
            self.add_point(step);
            previous_position = step;
        });
    }
    
    pub fn get_points(&self) -> Vec<Point> {
        self.monomer_positions.clone()
    }

    pub fn get_coordinates(&self) -> Vec<(f32, f32, f32)> {
        self.monomer_positions
            .par_iter()
            .map(|point: &Point| (point.x(), point.y(), point.z()))
            .collect()
    }

    // I want to optimize this function! 
    // In principle, i should be able to use ndarray and rayon to parallelize the computation
    // I will need to ensure that the appropriate types are return as this is a struct that will interface with python
    // pub fn pairwise_distances(&self) -> Vec<Vec<f32>> {
    //     let mut distances: Vec<Vec<f32>> = vec![vec![0.0; self.monomer_positions.len()]; self.monomer_positions.len()];

    //     for i in 0..self.monomer_positions.len() {
    //         distances[i][i] = 0.0; // Diagonal elements
    //         for j in (i + 1)..self.monomer_positions.len() {
    //             let distance: f32 = self.monomer_positions[i].distance(&self.monomer_positions[j]);
    //             distances[i][j] = distance;
    //             distances[j][i] = distance; // Symmetric matrix
    //         }
    //     }

    //     distances
    // }

    pub fn pairwise_distances(&self) -> Vec<Vec<f32>> {
        let num_points: usize = self.monomer_positions.len();
        let distances: Mutex<Vec<Vec<f32>>> = Mutex::new(vec![vec![0.0; num_points]; num_points]);

        (0..num_points).into_par_iter().for_each(|i: usize| {
            let mut distances = distances.lock().unwrap();
            distances[i][i] = 0.0;
            for j in (i + 1)..num_points {
                let distance: f32 = self.monomer_positions[i].distance(&self.monomer_positions[j]);
                distances[i][j] = distance;
                distances[j][i] = distance;
            }
        });

        distances.into_inner().unwrap()
    }

}


#[pyfunction]
pub fn save_pairwise_distances(pairwise_distances: Vec<Vec<f32>>, filename: &str) -> PyResult<()> {
    let file: File = File::create(filename).map_err(|e: std::io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    let encoder: GzEncoder<BufWriter<File>> = GzEncoder::new(BufWriter::new(file), Compression::default());

    let mut writer: BufWriter<GzEncoder<BufWriter<File>>> = BufWriter::new(encoder);
    for row in pairwise_distances.iter() {
        writeln!(
            &mut writer,
            "{}",
            row.iter()
                .map(|&val| val.to_string())
                .collect::<Vec<String>>()
                .join("\t")
        ).map_err(|e: std::io::Error| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    }

    Ok(())
}


// #[pyfunction]
// pub fn save_conformers_as_h5(conformers: Vec<Vec<f32>>, filename: &str) -> PyResult<()> {
//     let file: File = File::create(filename).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

//     // Create a dataset to store conformers
//     let dataset = file.new_dataset::<f32>()
//         .create("conformers", (conformers.len(), conformers[0].len()))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

//     // Write conformers to the dataset
//     dataset.write(&conformers).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

//     Ok(())
// }


fn harmonic_to_probability(x: f64, x0: f64, k: f64, temperature: f64) -> f64 {
    let kb: f64 = (BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT / 1000.0) * 0.239006;
    (-1.0 / (kb * temperature)) * (0.5 * k * (x0 - x).powi(2)).exp()
}
pub fn sample_step_size(x0: f32, k: f32, temperature: f32) -> f32 {
    let num_points: i32 = 100;
    let start_length: f32 = 3.135;
    let end_length: f32 = 4.615;
    let bond_lengths: Vec<f32> = (0..num_points)
        .map(|i: i32| start_length + (i as f32) * (end_length - start_length) / (num_points - 1) as f32)
        .map(|x: f32| x as f32)
        .collect();

    // maybe doesnt need parallelization
    let gaussian_pdf: Vec<f32> = bond_lengths
    .par_iter() // Parallel iterator
    .map(|&length| harmonic_to_probability(length as f64, x0 as f64, k as f64, temperature as f64) as f32)
    .collect();

    // Normalize to get the weights
    let sum: f32 = gaussian_pdf.iter().sum();
    let normalized_pdf: Vec<f32> = gaussian_pdf.iter().map(|&prob| prob / sum).collect();

    let weights: Vec<_> = normalized_pdf.iter().map(|&x| x).collect();
    let dist: WeightedIndex<f32> = WeightedIndex::new(&weights).unwrap();
    let mut rng: ThreadRng = rand::thread_rng();

    let sampled_index: usize = dist.sample(&mut rng);
    let sampled_bond_length: f32 = bond_lengths[sampled_index];

    sampled_bond_length
}        

