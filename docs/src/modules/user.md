# User

Reads the currently logged-in user profile. Always available once the osu! process is running.

## Struct: `UserInfo`

```rust
pub struct UserInfo {
    pub accuracy: f64,
    pub rankedscore: i64,
    pub username: String,
    pub id: i32,
    pub level: f32,
    pub playcount: i32,
    pub playmode: i32,
    pub rank: i32,
    pub pp: i32,
    pub bancho_status: i32,
    pub country_code: i32,
}
```

| Field | Offset (from user_base) | Notes |
|-------|-------------------------|-------|
| `accuracy` | `0x4` | f64, batched with rankedscore |
| `rankedscore` | `0xC` | i64 |
| `username` | `0x30` | String |
| `id` | `0x70` | batched with level |
| `level` | `0x74` | f32 |
| `playcount` | `0x7C` | 5-field batch: 0x7C–0x8C |
| `playmode` | `0x80` | |
| `rank` | `0x84` | |
| `pp` | `0x88` | |
| `bancho_status` | `0x8C` | |
| `country_code` | `0x9C` | separate read |

## Pointer Chain

```
state.addresses.user_profile + 0x7
  → read_i32       (ptr1)
  → read_i32 + 0x0 (user_base)  ← struct base
```

## Usage

```rust
let mut reader = UserReader::new(&p, &mut state, OsuClientKind::Stable);
let info = reader.info()?;
println!("Player: {} | Rank: #{} | PP: {}", info.username, info.rank, info.pp);
```
