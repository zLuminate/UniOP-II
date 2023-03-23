mod universe {
    pub struct Body {
        pub position: (f32, f32, f32),
        velocity: (f32, f32, f32),
        mass: f32
    }

    pub struct Universe {
        pub celestial_bodies: Vec<Body>
    }

    impl Universe {
        pub fn setup(&mut self) {
            for _ in 0..1300 {
                let x = (rand::random::<f32>()-0.5) * 2.0;
                let y = (rand::random::<f32>()-0.5) * 2.0;
                let z = (rand::random::<f32>()-0.5) * 2.0;

                let body = Body {
                    position: (x, y, z),
                    velocity: (0.0, 0.0, 0.0),
                    mass: 1.0 // rand::random::<f32>() * 4.0
                };

                self.celestial_bodies.push(body);
            }

            // self.celestial_bodies.push(Body {
            //     position: (0.0, 0.0, 0.0),
            //     velocity: (0.0, 0.0, 0.0),
            //     mass: 1000.0
            // });
        }

        fn duplicate_celestial_bodies(&self) -> Vec<Body> {
            let mut bodies = Vec::new();
            for body in self.celestial_bodies.iter() {
                bodies.push(Body {
                    position: body.position.clone(),
                    velocity: body.velocity.clone(),
                    mass: body.mass.clone()
                });
            }
            bodies
        }

        pub fn update(&mut self) {
            let mut other_bodies = self.duplicate_celestial_bodies();

            for i in 0..self.celestial_bodies.len() {
                other_bodies.remove(0);

                for other_body in other_bodies.iter() {
                    let x = self.celestial_bodies[i].position.0 - other_body.position.0;
                    let y = self.celestial_bodies[i].position.1 - other_body.position.1;
                    let z = self.celestial_bodies[i].position.2 - other_body.position.2;

                    let distance = (x*x + y*y + z*z).sqrt();
                    let force = 6.67408e-11 * self.celestial_bodies[i].mass * other_body.mass / (distance * distance);

                    let force_x = force * x / distance;
                    let force_y = force * y / distance;
                    let force_z = force * z / distance;

                    self.celestial_bodies[i].velocity.0 -= force_x;
                    self.celestial_bodies[i].velocity.1 -= force_y;
                    self.celestial_bodies[i].velocity.2 -= force_z;

                    self.celestial_bodies[i].position.0 += self.celestial_bodies[i].velocity.0;
                    self.celestial_bodies[i].position.1 += self.celestial_bodies[i].velocity.1;
                    self.celestial_bodies[i].position.2 += self.celestial_bodies[i].velocity.2;

                    // if i == 0 {
                    //     println!("{}", force);
                    //     println!("{}", distance);
                    //     println!("{} {} {}", force_x, force_y, force_z);
                    //     std::process::exit(0);
                    // }
                }
            }
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

        let mut universe = universe::Universe { celestial_bodies: Vec::new() };
        universe.setup();

        while !window.should_close() {
            // let curr_yaw = arc_ball.yaw();
            // arc_ball.set_yaw(curr_yaw + 0.05);

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

            for body in universe.celestial_bodies.iter() {
                window.draw_point(
                    &Point3::new(body.position.0, body.position.1, body.position.2),
                    &Point3::new(1.0, 1.0, 1.0),
                );
            }
        }
    }
}

fn main() {
    renderer::setup();
}