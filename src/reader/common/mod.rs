pub mod stable;
use std::path::PathBuf;

use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum OsuClientKind {
    #[default]
    Stable,
    Lazer,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameMode {
    Osu,
    Taiko,
    Catch,
    Mania,
    #[default]
    Unknown,
}

impl From<u32> for GameMode {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Osu,
            1 => Self::Taiko,
            2 => Self::Catch,
            3 => Self::Mania,
            _ => Self::Unknown,
        }
    }
}

impl From<i32> for GameMode {
    fn from(value: i32) -> Self {
        Self::from(value as u32)
    }
}

impl GameMode {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            GameMode::Osu => "std".to_string(),
            GameMode::Taiko => "taiko".to_string(),
            GameMode::Catch => "catch".to_string(),
            GameMode::Mania => "mania".to_string(),
            GameMode::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum GameState {
    MainMenu = 0,
    Editor = 1,
    Playing = 2,
    Exit = 3,
    EditorSongSelect = 4,
    SongSelect = 5,
    SelectDrawing = 6,
    ResultScreen = 7,
    Update = 8,
    Busy = 9,
    MultiplayerLobbySelect = 11,
    MultiplayerLobby = 12,
    MultiplayerSongSelect = 13,
    MultiplayerResultScreen = 14,
    OffsetWizard = 16,
    MultiplayerResultScreenTagCoop = 17,
    MultiplayerResultScreenTeamVs = 18,
    SongImport = 19,
    #[default]
    Unknown,
}

impl From<u32> for GameState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::MainMenu,
            1 => Self::Editor,
            2 => Self::Playing,
            3 => Self::Exit, // Very not useful :)
            4 => Self::EditorSongSelect,
            5 => Self::SongSelect,
            6 => Self::SelectDrawing, // Idk wwhat this shit is but tosu said its that lol
            7 => Self::ResultScreen,
            8 => Self::Update, // Idk what this shit is but tosu said its that lol
            9 => Self::Busy,   // Idk what this shit is but tosu said its that lol
            10 => Self::Unknown, // if tosu doesnt know i will not too
            11 => Self::MultiplayerLobbySelect,
            12 => Self::MultiplayerLobby,
            13 => Self::MultiplayerSongSelect,
            14 => Self::MultiplayerResultScreen,
            16 => Self::OffsetWizard,
            17 => Self::MultiplayerResultScreenTagCoop,
            18 => Self::MultiplayerResultScreenTeamVs,
            19 => Self::SongImport,
            _ => Self::Unknown,
        }
    }
}

pub struct CommonReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> CommonReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }

    pub fn get_game_state(&mut self) -> Result<GameState, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::get_game_state(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_menu_mods(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::get_menu_mods(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_path_folder(&mut self) -> Result<PathBuf, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::get_path_folder(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn check_game_state(&mut self, g_state: GameState) -> Result<bool, Error> {
        match self.osu_type {
            OsuClientKind::Stable => {
                stable::memory::check_game_state(self.process, self.state, g_state)
            }
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
