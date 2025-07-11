use crate::Error;
use rayon::prelude::*;
use rosu_mem::{
    process::{Process, ProcessTraits},
    signature::Signature,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

#[derive(Default, Clone)]
pub struct StaticAddresses {
    pub base: i32,
    pub status: i32,
    pub menu_mods: i32,
    pub rulesets: i32,
    pub playtime: i32,
    pub skin: i32,
    pub chat_checker: i32,
    pub audio_time_base: i32,
    pub ig_time_base: i32,
    pub settings: i32,
    pub user_profile: i32,
}

#[derive(Debug, Default, Clone)]
pub struct Hit {
    pub _geki: i16,
    pub _300: i16,
    pub _katu: i16,
    pub _100: i16,
    pub _50: i16,
    pub _miss: i16,
}

#[derive(Debug, Clone)]
pub(crate) struct SignatureBase {
    base_sig: &'static str,
    status_sig: &'static str,
    menu_mods_sig: &'static str,
    rulesets_sig: &'static str,
    playtime_sig: &'static str,
    skin_sig: &'static str,
    chat_checker_sig: &'static str,
    audio_time_base_sig: &'static str,
    ig_time_base_sig: &'static str,
    settings_sig: &'static str,
    user_profile_sig: &'static str,
}

pub(crate) const SIGNATURES: SignatureBase = SignatureBase {
    base_sig: "F8 01 74 04 83 65",
    status_sig: "48 83 F8 04 73 1E",
    menu_mods_sig: "C8 FF ?? ?? ?? ?? ?? 81 0D ?? ?? ?? ?? 00 08 00 00",
    rulesets_sig: "7D 15 A1 ?? ?? ?? ?? 85 C0",
    playtime_sig: "5E 5F 5D C3 A1 ?? ?? ?? ?? 89 ?? 04",
    skin_sig: "75 21 8B 1D",
    chat_checker_sig: "0A D7 23 3C 00 00 ?? 01",
    audio_time_base_sig: "DB 5C 24 34 8B 44 24 34",
    ig_time_base_sig: "EB 0A A1 ?? ?? ?? ?? A3",
    settings_sig: "83 E0 20 85 C0 7E 2F",
    user_profile_sig: "FF 15 ?? ?? ?? ?? A1 ?? ?? ?? ?? 8B 48 54 33 D2",
};

impl StaticAddresses {
    pub fn new(p: &Process) -> Result<Self, Error> {
        let start = Instant::now();
        println!("Reading signatures in parallel with rayon...");

        let signatures = [
            ("base", SIGNATURES.base_sig),
            ("status", SIGNATURES.status_sig),
            ("menu_mods", SIGNATURES.menu_mods_sig),
            ("rulesets", SIGNATURES.rulesets_sig),
            ("playtime", SIGNATURES.playtime_sig),
            ("skin", SIGNATURES.skin_sig),
            ("chat_checker", SIGNATURES.chat_checker_sig),
            ("audio_time_base", SIGNATURES.audio_time_base_sig),
            ("ig_time_base", SIGNATURES.ig_time_base_sig),
            ("settings", SIGNATURES.settings_sig),
            ("user_profile", SIGNATURES.user_profile_sig),
        ];

        let results: HashMap<&str, i32> = signatures
            .par_iter()
            .map(|(name, sig)| {
                let signature = Signature::from_str(sig)?;
                let addr = p.read_signature(&signature)?;
                Ok::<_, Error>((*name, addr))
            })
            .collect::<Result<_, Error>>()?;

        println!("Rayon time taken: {:?}", start.elapsed());

        Ok(Self {
            base: results["base"],
            status: results["status"],
            menu_mods: results["menu_mods"],
            rulesets: results["rulesets"],
            playtime: results["playtime"],
            skin: results["skin"],
            chat_checker: results["chat_checker"],
            audio_time_base: results["audio_time_base"],
            ig_time_base: results["ig_time_base"],
            settings: results["settings"],
            user_profile: results["user_profile"],
        })
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub addresses: StaticAddresses,
}
