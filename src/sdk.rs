use proc_mem::{Process, ProcMemError};
use crate::*;

pub struct PlayerController;

impl PlayerController {
    pub fn get_team(player_controller: usize, process: &Process) -> Result<i32, ProcMemError> {
        process.read_mem::<i32>(player_controller + M_I_TEAM_NUM)
    }
}

pub struct BasePlayerPawn;

impl BasePlayerPawn {
    pub fn get_pos(pawn_base: usize, process: &Process) -> Result<Vector, ProcMemError> {
        process.read_mem::<Vector>(pawn_base + M_V_OLD_ORIGIN)
    }

    pub fn get_ent_index(pawn_base: usize, process: &Process) -> Result<i32, ProcMemError> {
        process.read_mem::<i32>(pawn_base + M_I_IDENT_INDEX)
    }

    pub fn on_ground(pawn_base: usize, process: &Process) -> Result<bool, ProcMemError> {
        process.read_mem::<bool>(pawn_base + M_B_ON_GROUND_LAST_TICK)
    }

    pub fn flags(pawn_base: usize, process: &Process) -> Result<i32, ProcMemError> {
        process.read_mem::<i32>(pawn_base + FLAGS)
    }

}
