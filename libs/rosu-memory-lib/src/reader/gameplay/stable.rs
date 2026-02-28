//! Module de lecture mémoire gameplay pour le client stable d'osu!
//!
//! Fournit la lecture des données de partie en cours depuis la mémoire du processus osu!stable.

use crate::reader::common::stable::GameStateInfo;
use crate::reader::common::GameState;
use crate::reader::structs::{Hit, State};
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;

/// Informations de gameplay lues depuis la mémoire en temps réel
#[derive(Debug, Clone, ReadMemory)]
pub struct GameplayInfo {
    /// Score actuel du joueur
    #[offset(0x78)]
    pub score: i32,

    /// Mods actifs (décodés via XOR de deux valeurs u64)
    #[computed({
        let mods_xor_base = p.read_i32(__base + 0x1C)?;
        let xor1: u64 = p.read_u64(mods_xor_base + 0xC)?;
        let xor2: u64 = p.read_u64(mods_xor_base + 0x8)?;
        (xor1 ^ xor2) as u32
    })]
    pub mods: u32,

    /// Combo actuel
    #[offset(0x94)]
    pub combo: i16,

    /// Combo maximum atteint durant la partie
    #[offset(0x68)]
    pub max_combo: i16,

    /// Points de vie du joueur (0.0 à 1.0)
    #[ptr_chain(state.addresses.rulesets - 0xb, 0x4, 0x68, 0x40, 0x1C)]
    pub hp: f64,

    /// Nom d'utilisateur du joueur
    #[offset(0x28)]
    pub username: String,

    /// Temps de jeu en millisecondes
    #[computed(crate::reader::common::stable::TimingInfo::read(p, state)?.game_time)]
    pub ig_time: i32,

    /// Nombre de tentatives sur cette map
    #[ptr_chain(state.addresses.base - 0x33, 0x0, 0x8)]
    pub retries: i32,

    /// Détail des notes touchées
    #[nested(0x88)]
    pub hits: Hit,
}

impl GameplayInfo {
    /// Lire les informations de gameplay depuis la mémoire
    ///
    /// Retourne une erreur si le jeu n'est pas en état Playing
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let game_state = GameStateInfo::read(p, state)?;
        if game_state.state != GameState::Playing {
            return Err(Error::NotAvailable("Not in Playing state".to_string()));
        }
        let ruleset_ptr = p.read_i32(state.addresses.rulesets - 0xb)?;
        let ruleset_addr = p.read_i32(ruleset_ptr + 0x4)?;
        let gameplay_base = p.read_i32(ruleset_addr + 0x68)?;
        let score_base = p.read_i32(gameplay_base + 0x38)?;
        Self::read_from_memory(p, state, score_base)
    }
}

pub mod memory {
    use super::*;

    /// Lire toutes les informations de gameplay
    pub fn info(p: &Process, state: &mut State) -> Result<GameplayInfo, Error> {
        GameplayInfo::read(p, state)
    }
}
