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
}

pub struct ListEntry;

impl ListEntry {
    pub fn get_list_entry(ent_list: usize, index: usize, process: &Process) -> Result<usize, ProcMemError>{
        process.read_mem::<usize>(ent_list + (8 * (index & 0x7FFF) >> 9) + 16)
    }

    pub fn get_inner(list_entry: usize, index: usize, process: &Process) -> Result<usize, ProcMemError> {
        process.read_mem::<usize>(list_entry + 120 * (index & 0x1FF))
    }

}
