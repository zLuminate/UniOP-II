// mod universe;

// extern crate kiss3d;
//     use kiss3d::camera::{ArcBall, FirstPerson};
//     use kiss3d::event::{Action, Key, WindowEvent};
//     use kiss3d::light::Light;
//     use kiss3d::window::Window;

//     extern crate nalgebra as na;
//     use na::Point3;

// fn main() {
//     let mut universe = universe::Universe {
//         positions: Vec::new(),
//         masses: Vec::new(),
//         velocities: Vec::new()
//     };
//     universe.setup();


//     let eye = Point3::new(5.0f32, 5.0, 5.0);
//     let at = Point3::origin();
//     let mut first_person = FirstPerson::new(eye, at);
//     let mut arc_ball = ArcBall::new(eye, at);
//     let mut use_arc_ball = true;
//     let mut window = Window::new("Universal Operator v2.0.0");
//     window.set_light(Light::StickToCamera);

//     while !window.should_close() {
//         universe.update();

//         for body in universe.bodies.iter() {
//             window.draw_point(
//                 &Point3::new(body.position[0], body.position[1], body.position[2]),
//                 &Point3::new(1.0, 1.0, 1.0),
//             );
//         }

//         for event in window.events().iter() {
//             match event.value {
//                 WindowEvent::Key(key, Action::Release, _) => {
//                     if key == Key::Numpad1 {
//                         use_arc_ball = true
//                     } else if key == Key::Numpad2 {
//                         use_arc_ball = false
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         if use_arc_ball {
//             window.render_with_camera(&mut arc_ball);
//         } else {
//             window.render_with_camera(&mut first_person);
//         }

//         window.draw_line(
//             &Point3::new(-1.0, 0.0, 0.0),
//             &Point3::new(1.0, 0.0, 0.0),
//             &Point3::new(1.0, 0.0, 0.0),
//         );
//         window.draw_line(
//             &Point3::new(0.0, -1.0, 0.0),
//             &Point3::new(0.0, 1.0, 0.0),
//             &Point3::new(0.0, 1.0, 0.0),
//         );
//         window.draw_line(
//             &Point3::new(0.0, 0.0, -1.0),
//             &Point3::new(0.0, 0.0, 1.0),
//             &Point3::new(0.0, 0.0, 1.0),
//         );
//     }
// }