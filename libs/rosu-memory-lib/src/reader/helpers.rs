use crate::reader::common::GameMode;
use crate::reader::structs::Hit;
use crate::Error;

/// Génère des méthodes d'accès mémoire osu! avec dispatch automatique selon le type de client.
///
/// Élimine le boilerplate en générant des méthodes qui délèguent à l'implémentation stable.
#[macro_export]
macro_rules! impl_osu_accessor {
    ($(fn $name:ident() -> $ret:ty => $call:path),* $(,)?) => {
        $(
            pub fn $name(&mut self) -> Result<$ret, Error> {
                match self.osu_type {
                    OsuClientKind::Stable => $call(self.process, self.state),
                    _ => Err(Error::Unsupported(
                        "Unsupported osu type for now".to_string(),
                    )),
                }
            }
        )*
    };
}

/// Calcule la précision selon le mode de jeu et les notes touchées
#[inline]
pub fn calculate_accuracy(gamemode: &GameMode, hit: &Hit) -> Result<f64, Error> {
    let acc = match gamemode {
        GameMode::Osu => {
            let total = (hit._300 + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64;
            (score / (total * 6.0)) * 100.0
        }
        GameMode::Taiko => {
            let total = (hit._300 + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = hit._300 as f64 * 2.0 + hit._100 as f64;
            (score / (total * 2.0)) * 100.0
        }
        GameMode::Catch => {
            let caught = (hit._300 + hit._100 + hit._50) as f64;
            let total = (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            (caught / total) * 100.0
        }
        GameMode::Mania => {
            let total = (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = (hit._geki + hit._300) as f64 * 6.0
                + hit._katu as f64 * 4.0
                + hit._100 as f64 * 2.0
                + hit._50 as f64;
            (score / (total * 6.0)) * 100.0
        }
        _ => return Ok(0.0),
    };

    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(geki: i16, n300: i16, katu: i16, n100: i16, n50: i16, miss: i16) -> Hit {
        Hit {
            _geki: geki,
            _300: n300,
            _katu: katu,
            _100: n100,
            _50: n50,
            _miss: miss,
        }
    }

    #[test]
    fn accuracy_empty_hit_returns_zero() {
        let h = hit(0, 0, 0, 0, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Osu, &h).unwrap(), 0.0);
        assert_eq!(calculate_accuracy(&GameMode::Taiko, &h).unwrap(), 0.0);
        assert_eq!(calculate_accuracy(&GameMode::Catch, &h).unwrap(), 0.0);
        assert_eq!(calculate_accuracy(&GameMode::Mania, &h).unwrap(), 0.0);
    }

    #[test]
    fn accuracy_osu_full_300s() {
        // All 300s → 100% accuracy
        let h = hit(0, 100, 0, 0, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Osu, &h).unwrap(), 100.0);
    }

    #[test]
    fn accuracy_osu_all_miss() {
        // All misses → 0%
        let h = hit(0, 0, 0, 0, 0, 100);
        assert_eq!(calculate_accuracy(&GameMode::Osu, &h).unwrap(), 0.0);
    }

    #[test]
    fn accuracy_osu_mixed() {
        // 50x300, 50x100 → (50*6 + 50*2) / (100*6) * 100 = 400/600*100 = 66.666...
        let h = hit(0, 50, 0, 50, 0, 0);
        let acc = calculate_accuracy(&GameMode::Osu, &h).unwrap();
        assert!((acc - 66.666_666_666_666_67).abs() < 1e-10);
    }

    #[test]
    fn accuracy_taiko_full_300s() {
        let h = hit(0, 100, 0, 0, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Taiko, &h).unwrap(), 100.0);
    }

    #[test]
    fn accuracy_taiko_mixed() {
        // 50x300, 50x100 → (50*2 + 50) / (100*2) * 100 = 150/200*100 = 75.0
        let h = hit(0, 50, 0, 50, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Taiko, &h).unwrap(), 75.0);
    }

    #[test]
    fn accuracy_catch_full() {
        // All caught (300+100+50), no katu/miss → 100%
        let h = hit(0, 50, 0, 30, 20, 0);
        assert_eq!(calculate_accuracy(&GameMode::Catch, &h).unwrap(), 100.0);
    }

    #[test]
    fn accuracy_catch_with_miss() {
        // 90 caught, 10 miss → 90%
        let h = hit(0, 90, 0, 0, 0, 10);
        assert_eq!(calculate_accuracy(&GameMode::Catch, &h).unwrap(), 90.0);
    }

    #[test]
    fn accuracy_mania_full_geki() {
        // All geki (MAX/rainbow 300) → 100%
        let h = hit(100, 0, 0, 0, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Mania, &h).unwrap(), 100.0);
    }

    #[test]
    fn accuracy_unknown_mode_returns_zero() {
        let h = hit(0, 100, 0, 0, 0, 0);
        assert_eq!(calculate_accuracy(&GameMode::Unknown, &h).unwrap(), 0.0);
    }
}
