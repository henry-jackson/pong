extern crate amethyst;
use amethyst::prelude::*;
#[derive(PartialEq)]
pub enum CurrentState {
   Running,
   Paused,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Paused
    }
}

struct GameplayState;

impl SimpleState for GameplayState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
      let my_condition = true;
        if (my_condition) {
            *data.world.write_resource::<CurrentState>() = CurrentState::Paused;
        }
        
        Trans::None
    }
}
