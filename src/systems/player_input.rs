use crate::prelude::*;


// Legion systems are quite complicated internally. It uses Procedural Macros (commonly
// referred to as "proc macros") to save you from writing a lot of repetitive boilerplate
// code for each system.
// #[system] is a procedural macro

// The #[system] line annotates the player_input function with a procedural macro
// named system . This macro transforms your function name into player_input_system ,
// and wraps it with all of the extra code Legion requires to construct a system.
// #[write_component] requests writable access to a component type
// #[read_component] requests read-only access to a component type
// #[resource] requests access to types you stored in Legion’s Resource handler
// (see main.rs)
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
)
{
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()
                .filter(components::Player());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
