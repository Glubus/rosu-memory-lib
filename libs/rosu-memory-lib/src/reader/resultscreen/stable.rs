//! Module de lecture mémoire écran de résultats pour le client stable d'osu!
//!
//! Fournit la lecture des données de l'écran de résultats depuis la mémoire du processus osu!stable.

use crate::reader::common::stable::GameStateInfo;
use crate::reader::common::{GameMode, GameState};
use crate::reader::structs::{Hit, State};
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;

/// Informations de l'écran de résultats lues depuis la mémoire
#[derive(Debug, Clone, ReadMemory)]
pub struct ResultScreenInfo {
    /// Nom d'utilisateur du joueur
    #[offset(0x28)]
    pub username: String,

    /// Mode de jeu utilisé
    #[offset(0x64, via = i32)]
    pub mode: GameMode,

    /// Combo maximum atteint durant la partie
    #[offset(0x68)]
    pub max_combo: i16,

    /// Score final
    #[offset(0x78)]
    pub score: i32,

    /// Détail des notes touchées
    #[nested(0x88)]
    pub hits: Hit,

    /// Précision calculée selon le mode de jeu
    #[computed(crate::reader::helpers::calculate_accuracy(&mode, &hits)?)]
    pub accuracy: f64,
}

impl ResultScreenInfo {
    /// Lire les informations de l'écran de résultats depuis la mémoire
    ///
    /// Retourne une erreur si le jeu n'est pas en état ResultScreen
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let game_state = GameStateInfo::read(p, state)?;
        if game_state.state != GameState::ResultScreen {
            return Err(Error::NotAvailable("Not in ResultScreen state".to_string()));
        }
        let ruleset_ptr = p.read_i32(state.addresses.rulesets - 0xb)?;
        let ruleset_addr = p.read_i32(ruleset_ptr + 0x4)?;
        let base = p.read_i32(ruleset_addr + 0x38)?;
        Self::read_from_memory(p, state, base)
    }
}

pub mod memory {
    use super::*;

    /// Lire toutes les informations de l'écran de résultats
    pub fn info(p: &Process, state: &mut State) -> Result<ResultScreenInfo, Error> {
        ResultScreenInfo::read(p, state)
    }
}
