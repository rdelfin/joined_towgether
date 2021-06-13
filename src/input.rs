use std::fmt::{self, Display};

use amethyst::{
    core::{geometry::Plane, Transform},
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, WriteStorage},
    input::{BindingTypes, InputHandler},
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
};
use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Forwards,
    Sideways,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Fire,
    Activate,
    Place,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct GameBindingTypes;

impl BindingTypes for GameBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

// Useful input utils

pub fn get_mouse_projection<'s>(
    entities: &Entities<'s>,
    input: &Read<'s, InputHandler<GameBindingTypes>>,
    transforms: &WriteStorage<'s, Transform>,
    cameras: &ReadStorage<'s, Camera>,
    active_camera: &Read<'s, ActiveCamera>,
    screen_dimensions: &ReadExpect<'s, ScreenDimensions>,
) -> Option<Point2<f32>> {
    let mouse = match input.mouse_position() {
        Some((x, y)) => Point2::new(x, y),
        None => Point2::new(0.0, 0.0),
    };
    let mut camera_join = (cameras, transforms).join();

    match active_camera
        .entity
        .and_then(|a| camera_join.get(a, &entities))
        .or_else(|| camera_join.next())
    {
        Some((camera, camera_transform)) => {
            let ray = camera.screen_ray(
                mouse,
                Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                camera_transform,
            );
            let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
            let point_intersection = ray.at_distance(distance);
            Some(Point2::new(point_intersection.x, point_intersection.y))
        }
        None => None,
    }
}
