mod universe {
    use octree::Octree;
    use std::time::Instant;

    pub struct Universe {
        pub positions: Vec<[f64;3]>,
        pub masses: Vec<f32>,
        pub velocities: Vec<[f64;3]>
    }

    impl Universe {
        pub fn setup(&mut self) {
            for _ in 0..1000 {
                let x = (rand::random::<f64>()-0.5) * 2.0;
                let y = (rand::random::<f64>()-0.5) * 2.0;
                let z = (rand::random::<f64>()-0.5) * 2.0;
                self.new_body(1.0, [x,y,z], [0.0,0.0,0.0])
            }
        }

        fn new_body(&mut self, mass: f32, position: [f64; 3], initial_velocity: [f64; 3]) {
            self.masses.push(mass);
            self.positions.push(position);
            self.velocities.push(initial_velocity);
        }

        pub fn update(&self) {
            let start_time = Instant::now();

            let mut tree = Octree::new(self.positions.clone());
            tree.build(10);

            for x in 0..999 {
                println!("{}", self.positions[x][0]==tree.points[x][0]);
            }

            let end_time = Instant::now();
            println!("Time taken: {}ms", (end_time - start_time).as_millis());

            std::process::exit(0);
        }
    }
}

mod renderer {
    extern crate kiss3d;
    extern crate nalgebra as na;

    use kiss3d::camera::{ArcBall, FirstPerson};
    use kiss3d::event::{Action, Key, WindowEvent};
    use kiss3d::light::Light;
    use kiss3d::window::Window;

    use na::Point3;

    use crate::universe;

    pub fn setup() {
        let eye = Point3::new(5.0f32, 5.0, 5.0);
        let at = Point3::origin();
        let mut first_person = FirstPerson::new(eye, at);
        let mut arc_ball = ArcBall::new(eye, at);
        let mut use_arc_ball = true;
        let mut window = Window::new("Universal Operator v2.0.0");
        window.set_light(Light::StickToCamera);

        let mut universe = universe::Universe {
            positions: Vec::new(),
            masses: Vec::new(),
            velocities: Vec::new()
        };
        universe.setup();

        while !window.should_close() {
            {
                for event in window.events().iter() {
                    match event.value {
                        WindowEvent::Key(key, Action::Release, _) => {
                            if key == Key::Numpad1 {
                                use_arc_ball = true
                            } else if key == Key::Numpad2 {
                                use_arc_ball = false
                            }
                        }
                        _ => {}
                    }
                }

                if use_arc_ball {
                    window.render_with_camera(&mut arc_ball);
                } else {
                    window.render_with_camera(&mut first_person);
                }

                window.draw_line(
                    &Point3::new(-1.0, 0.0, 0.0),
                    &Point3::new(1.0, 0.0, 0.0),
                    &Point3::new(1.0, 0.0, 0.0),
                );
                window.draw_line(
                    &Point3::new(0.0, -1.0, 0.0),
                    &Point3::new(0.0, 1.0, 0.0),
                    &Point3::new(0.0, 1.0, 0.0),
                );
                window.draw_line(
                    &Point3::new(0.0, 0.0, -1.0),
                    &Point3::new(0.0, 0.0, 1.0),
                    &Point3::new(0.0, 0.0, 1.0),
                );
            }

            universe.update();

            for body in universe.positions.iter() {
                window.draw_point(
                    &Point3::new(body[0] as f32, body[1] as f32, body[2] as f32),
                    &Point3::new(1.0, 1.0, 1.0),
                );
            }
        }
    }
}

fn main() {
    renderer::setup();
}