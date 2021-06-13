use amethyst::ecs::Entity;

pub struct FollowedObject {
    pub e: Entity,
    pub hard_lock: bool,
}
