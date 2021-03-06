use super::utils::*;
use crate::{
    comp::StateUpdate,
    sys::character_behavior::{CharacterBehavior, JoinData},
};

pub struct Data;

impl CharacterBehavior for Data {
    fn behavior(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);

        handle_move(data, &mut update, 1.0);
        handle_jump(data, &mut update);
        handle_wield(data, &mut update);
        handle_climb(data, &mut update);
        handle_dodge_input(data, &mut update);

        update
    }

    fn wield(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);
        attempt_wield(data, &mut update);
        update
    }

    fn sit(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);
        attempt_sit(data, &mut update);
        update
    }

    fn dance(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);
        attempt_dance(data, &mut update);
        update
    }

    fn glide_wield(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);
        attempt_glide_wield(data, &mut update);
        update
    }

    fn swap_loadout(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);
        attempt_swap_loadout(data, &mut update);
        update
    }
}
