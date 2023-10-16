#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use log::debug;
use proc_mem::Process;
use stopwatch::Stopwatch;
use crate::sdk::{BasePlayerPawn, PlayerController};
use crate::types::Vector;
use winsafe::{self as w};
use winsafe::co::VK;

// offsets.rs
const DW_LOCAL_PLAYER_CONTROLLER: usize = 0x17DE508;
const DW_LOCAL_PLAYER_PAWN: usize = 0x187CFC8;
const DW_ENTITY_LIST: usize = 0x178FC88;
const DW_VIEW_MATRIX: usize = 0x187DAB0;
const DW_FORCE_JUMP: usize = 0x1697300;

// CCSPlayerController
const M_H_PLAYER_PAWN: usize = 0x7BC;

// C_BaseEntity
const M_I_TEAM_NUM: usize = 0x3BF;

// C_BasePlayerPawn
const M_V_OLD_ORIGIN: usize = 0x1214;
const M_I_IDENT_INDEX: usize = 0x152C;
const M_B_ON_GROUND_LAST_TICK: usize = 0x2298;
const FLAGS: usize = 0x3C8;

mod types;
mod sdk;

fn main() {
    let mut enigo = Enigo::new();

    let mut stopwatch = Stopwatch::new();

    let logger = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let process = Process::with_name("cs2.exe").expect("couldn't find cs2.");

    // modules
    let client = process.module("client.dll").expect("couldn't get client.dll");

    // offsets
    let local_player_controller = process.read_mem::<usize>(client.base_address() + DW_LOCAL_PLAYER_CONTROLLER).expect("couldn't get local player controller.");
    let local_player_pawn = process.read_mem::<usize>(client.base_address() + DW_LOCAL_PLAYER_PAWN).expect("couldn't get local player pawn.");
    let entity_list = process.read_mem::<usize>(client.base_address() + DW_ENTITY_LIST).expect("couldn't get ent list.");

    loop {
        sleep(Duration::from_millis(1));
        let local_team = PlayerController::get_team(local_player_controller, &process);

        if local_team.is_err() {
            continue;
        }

        let local_team = local_team.unwrap();


        // Auto B-Hop.
        let flags = BasePlayerPawn::flags(local_player_pawn, &process);

        if flags.is_err() {
                continue;
        }

        let flags = flags.unwrap() & (1 >> 0);
        let in_air = (flags == 1);
            if w::GetAsyncKeyState(VK::SPACE) {
                // so we jump if we are on ground.
                if(!in_air) {
                    process.write_mem::<i32>(client.base_address() + DW_FORCE_JUMP, 256);
                    process.write_mem::<i32>(client.base_address() + DW_FORCE_JUMP, 65537);
                    process.write_mem::<i32>(client.base_address() + DW_FORCE_JUMP, 256);
                } else {
                    process.write_mem::<i32>(client.base_address() + DW_FORCE_JUMP, 65537);
                }
        }


        // Triggerbot

        let mouse_over_ent_index = BasePlayerPawn::get_ent_index(local_player_pawn, &process);

        if mouse_over_ent_index.is_err() {
            continue;
        }

        let mouse_over_ent_index = mouse_over_ent_index.unwrap();

        if mouse_over_ent_index < 0 {
            continue;
        }

        let mouse_over_ent_index = mouse_over_ent_index as usize;

        let list_entry = process.read_mem::<usize>(entity_list + 16);

        if list_entry.is_err() {
            continue;
        }

        let list_entry = list_entry.unwrap();

        let crosshair_pawn = process.read_mem::<usize>(list_entry + 120 * (mouse_over_ent_index & 0x1FF));

        if crosshair_pawn.is_err() {
            continue;
        }

        let crosshair_pawn = crosshair_pawn.unwrap();

        let team = PlayerController::get_team(crosshair_pawn, &process);

        if team.is_err() {
            continue;
        }

        let team = team.unwrap();

        if team != local_team {
            // ensure first shot isnt instant.
            if !stopwatch.is_running() {
                stopwatch.start();
            }
            if stopwatch.elapsed_ms() > 150 {
                enigo.mouse_click(MouseButton::Left);
                stopwatch.reset();
            }
        }
    }
}