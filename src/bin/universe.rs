use std::{time::Instant};

#[derive(Clone)]
pub struct Body {
    pub mass: f32,
    pub position: [f32; 3],
    pub velocity: [f32; 3]
}

pub struct Octree {
    pub b_count: i32,
    pub octants: Vec<(i8, Octree)>, // (octant_index, octree object)
    pub children: Vec<Body>,
}

impl Octree {
    pub fn new(bodies: Vec<Body>, max_per_octant: i32) -> Octree {

        let b_count = bodies.len() as i32;

        if b_count <= max_per_octant {
            return Octree {
                b_count: b_count,
                octants: Vec::new(),
                children: bodies
            };
        }

        let max_point: [f32; 3]= [
            bodies.iter().map(|body| body.position[0]).max_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap(),
            bodies.iter().map(|body| body.position[1]).max_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap(),
            bodies.iter().map(|body| body.position[2]).max_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap()
        ];
        let min_point :[f32; 3]= [
            bodies.iter().map(|body| body.position[0]).min_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap(),
            bodies.iter().map(|body| body.position[1]).min_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap(),
            bodies.iter().map(|body| body.position[2]).min_by(
                |a, b| a.partial_cmp(b).unwrap()
            ).unwrap()
        ];

        let mut sorted: Vec<(i32, Vec<Body>)> = Vec::from([
            (0, Vec::new()),
            (1, Vec::new()),
            (2, Vec::new()),
            (3, Vec::new()),
            (4, Vec::new()),
            (5, Vec::new()),
            (6, Vec::new()),
            (7, Vec::new())
        ]);

        for body in bodies.iter() {
            let octant: [i8; 3] = [
                if body.position[0] > (max_point[0]+min_point[0])/2.0 { 1 } else { 0 },
                if body.position[1] > (max_point[1]+min_point[1])/2.0 { 1 } else { 0 },
                if body.position[2] > (max_point[2]+min_point[2])/2.0 { 1 } else { 0 }
            ];
            let octant_index: i8 = (octant[0]*4 + octant[1]*2 + octant[2]) as i8;
            sorted[octant_index as usize].1.push(body.clone());
        }

        let octree: Octree = Octree {
            b_count: bodies.len() as i32,
            octants: Vec::from([
                (0, Octree::new(sorted[0].1.clone(), max_per_octant)),
                (1, Octree::new(sorted[1].1.clone(), max_per_octant)),
                (2, Octree::new(sorted[2].1.clone(), max_per_octant)),
                (3, Octree::new(sorted[3].1.clone(), max_per_octant)),
                (4, Octree::new(sorted[4].1.clone(), max_per_octant)),
                (5, Octree::new(sorted[5].1.clone(), max_per_octant)),
                (6, Octree::new(sorted[6].1.clone(), max_per_octant)),
                (7, Octree::new(sorted[7].1.clone(), max_per_octant))
            ]),
            children: bodies
        };

        octree
    }
}

pub struct Universe {
    pub bodies: Vec<Body>
}

impl Universe {
    pub fn new() -> Universe {
        let mut universe = Universe {
            bodies: Vec::new()
        };

        for _ in 0..100000 {
            universe.bodies.push(Body {
                mass: rand::random::<f32>() * 100.0,
                position: [rand::random::<f32>()-0.5, rand::random::<f32>()-0.5, rand::random::<f32>()-0.5],
                velocity: [0.0, 0.0, 0.0]
            });
        }

        universe
    }

    pub fn update(&self) {
        let start_time = Instant::now();

        let _octree = Octree::new(self.bodies.clone(), 5);

        let end_time = Instant::now();
        println!("Time taken: {}ms", (end_time - start_time).as_millis());

        std::process::exit(0);
    }
}

fn main() {
    let universe = Universe::new();
    universe.update();
}