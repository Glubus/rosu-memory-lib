//! Module de lecture mémoire profil utilisateur pour le client stable d'osu!
//!
//! Fournit la lecture des données du profil utilisateur connecté depuis la mémoire du processus osu!stable.

use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;

/// Informations du profil utilisateur lues depuis la mémoire
#[derive(Debug, Clone, ReadMemory)]
pub struct UserInfo {
    /// Précision globale du joueur (en pourcentage)
    #[offset(0x4)]
    pub accuracy: f64,

    /// Score classé total
    #[offset(0xC)]
    pub rankedscore: i64,

    /// Nom d'utilisateur
    #[offset(0x30)]
    pub username: String,

    /// Identifiant numérique du joueur
    #[offset(0x70)]
    pub id: i32,

    /// Niveau actuel du joueur
    #[offset(0x74)]
    pub level: f32,

    /// Nombre total de parties jouées
    #[offset(0x7C)]
    pub playcount: i32,

    /// Mode de jeu principal du joueur
    #[offset(0x80)]
    pub playmode: i32,

    /// Rang mondial du joueur
    #[offset(0x84)]
    pub rank: i32,

    /// Points de performance (pp)
    #[offset(0x88)]
    pub pp: i32,

    /// Statut Bancho (connecté, absent, etc.)
    #[offset(0x8C)]
    pub bancho_status: i32,

    /// Code pays du joueur
    #[offset(0x9C)]
    pub country_code: i32,
}

impl UserInfo {
    /// Lire les informations du profil utilisateur depuis la mémoire
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let base = p.read_i32(p.read_i32(state.addresses.user_profile + 0x7)?)?;
        Self::read_from_memory(p, state, base)
    }
}

pub mod memory {
    use super::*;

    /// Lire toutes les informations du profil utilisateur
    pub fn info(p: &Process, state: &mut State) -> Result<UserInfo, Error> {
        UserInfo::read(p, state)
    }

    /// Lire uniquement le mode de jeu principal (utilisé par l'overlay)
    pub fn playmode(p: &Process, state: &mut State) -> Result<i32, Error> {
        Ok(UserInfo::read(p, state)?.playmode)
    }
}
