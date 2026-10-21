//! Unit, turret and effect frames.
//!
//! Logic-faithful port of `unit_frame`, `unit_condition_frame`,
//! `independent_turret_frame` and `effect_frame` from
//! `cnc32_asset_generator_v5.py` (v5.5.0). These are pure deterministic
//! geometry — no randomness. Facings are eight-way (N, NE, E, SE, S, SW, W,
//! NW); bodies are drawn per-facing with direction vectors instead of
//! rotating finished sprites.
//!
//! Faithfulness notes: Python's floor division (`stride // 2`) maps to
//! [`i32::div_euclid`] and Python's always-non-negative remainder
//! (`(t + phase) % 3`) maps to [`i32::rem_euclid`]; flat-coordinate
//! polylines are split into line segments; PIL replaces pixels outright when
//! drawing (even with alpha), matching [`Canvas::set_pixel`].

use super::canvas::Canvas;
use super::{color, Rgba};

/// Rendered factions. Python treats any non-`"gdi"` faction as `nod`.
pub const FACTIONS: [&str; 2] = ["gdi", "nod"];

/// Damage conditions for full-frame condition sprites, in pack order.
pub const CONDITIONS: [&str; 4] = ["intact", "damaged", "critical", "destroyed"];

/// Unit kinds and their supported animation states, in pack order (the
/// manifest's `unit_state_definitions` and review-sheet rows use this).
pub const UNIT_STATES: &[(&str, &[&str])] = &[
    ("tank", &["idle", "move", "turn", "attack", "death"]),
    (
        "harvester",
        &["idle", "move", "turn", "harvest", "unload", "death"],
    ),
    (
        "artillery",
        &["idle", "move", "turn", "deploy", "attack", "death"],
    ),
    ("infantry", &["idle", "move", "attack", "death"]),
    ("engineer", &["idle", "move", "repair", "death"]),
    ("rocket_soldier", &["idle", "move", "attack", "death"]),
    ("buggy", &["idle", "move", "turn", "attack", "death"]),
];

/// Effect kinds and their frame counts, in pack order.
pub const EFFECT_KINDS: [(&str, u8); 4] = [
    ("explosion", 6),
    ("smoke", 6),
    ("muzzle_flash", 3),
    ("destruction", 8),
];

/// Frames per animation state (Python `STATE_FRAMES`).
pub fn state_frames(state: &str) -> u8 {
    match state {
        "idle" => 2,
        "move" | "turn" | "attack" | "harvest" | "unload" | "deploy" | "repair" => 4,
        "death" => 6,
        _ => 0,
    }
}

/// Per-frame duration in milliseconds for a state.
pub fn frame_duration_ms(state: &str) -> u32 {
    match state {
        "idle" => 180,
        "attack" | "turn" => 110,
        "death" => 140,
        _ => 120,
    }
}

/// Whether a state's animation loops.
pub fn state_loops(state: &str) -> bool {
    matches!(state, "idle" | "move" | "harvest")
}

/// Per-frame duration in milliseconds for an effect kind.
pub fn effect_duration_ms(kind: &str) -> u32 {
    if kind == "smoke" {
        140
    } else {
        90
    }
}

/// Error for an unknown unit condition name.
#[derive(thiserror::Error, Debug)]
#[error("unknown unit condition: {0}")]
pub struct UnknownCondition(pub String);

const DIRS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn faction_colors(faction: &str) -> (Rgba, Rgba) {
    if faction == "gdi" {
        (color("gdi"), color("gdi_dark"))
    } else {
        (color("nod"), color("nod_dark"))
    }
}

/// Stroke a flat-coordinate polyline (PIL's `d.line((x0,y0,x1,y1,...))`):
/// consecutive segments through every point, including overlapping ones.
fn polyline(c: &mut Canvas, coords: &[i32], rgba: Rgba, width: u32) {
    let (pairs, _trail) = coords.as_chunks::<2>();
    for pair in pairs.windows(2) {
        let [a, b] = pair else {
            return;
        };
        c.line(a[0], a[1], b[0], b[1], rgba, width);
    }
}

/// Draw a direction-aware unit pose without rotating a finished sprite.
///
/// `facing` is an eight-way direction; body axis, weapon direction, limbs,
/// track marks, recoil, harvesting and death poses are drawn directly per
/// facing/state. Unknown factions render as `nod`; unknown units still get
/// state accents (matching Python's if/elif fall-through).
pub fn unit_frame(faction: &str, unit: &str, facing: u8, frame: u8, state: &str) -> Canvas {
    let mut im = Canvas::transparent(32, 32);
    let (body, dark) = faction_colors(faction);
    let (dx, dy) = DIRS[(facing % 8) as usize];
    let (cx, cy) = (16, 16);
    let frame = frame as i32;

    if state == "death" {
        let spread = 5.min(frame + 1);
        if matches!(unit, "tank" | "harvester" | "artillery" | "buggy") {
            im.polygon(
                &[
                    (cx - spread, cy - 5),
                    (cx + 5, cy - spread),
                    (cx + spread, cy + 4),
                    (cx - 4, cy + spread),
                ],
                Some(dark),
                None,
            );
            im.rect(
                cx - 5,
                cy - 2,
                cx + 5,
                cy + 2,
                Some(color("metal_dark")),
                None,
                0,
            );
        } else {
            im.ellipse(
                cx - 7 - spread / 2,
                cy - 3,
                cx + 7 + spread / 2,
                cy + 3 + spread / 2,
                Some(dark),
                None,
                1,
            );
            let head_x = cx - 3 + frame % 3;
            im.ellipse(
                head_x - 2,
                cy - 6 - spread / 2,
                head_x + 2,
                cy - 2,
                Some(color("skin")),
                None,
                1,
            );
            im.line(
                cx - 4,
                cy + 2,
                cx + 4 + frame % 2,
                cy + 3 + spread / 2,
                body,
                2,
            );
        }
        if frame >= 3 {
            let r = frame - 2;
            let alpha = 40.max(190 - frame * 22) as u8;
            let smoke: Rgba = [
                color("smoke")[0],
                color("smoke")[1],
                color("smoke")[2],
                alpha,
            ];
            im.ellipse(
                cx - r - 2,
                cy - r - 5,
                cx + r + 2,
                cy + r - 1,
                Some(smoke),
                None,
                1,
            );
        }
        return im;
    }

    const BOB: [i32; 4] = [0, 1, 0, -1];
    let bob = BOB[(frame % 4) as usize];
    let recoil = if state == "attack" && frame == 2 {
        -2
    } else {
        0
    };

    if matches!(unit, "tank" | "harvester" | "artillery") {
        // Vehicle hull oriented by direction vectors, not image rotation.
        let (px, py) = (-dy, dx);
        let length = if unit == "artillery" { 12 } else { 10 };
        let width = 7;
        let xy = |along: i32, across: i32| -> (i32, i32) {
            (
                cx + dx * along + px * across,
                cy + dy * along + py * across + bob,
            )
        };
        let corners = [
            xy(-length, -width),
            xy(length, -width),
            xy(length, width),
            xy(-length, width),
        ];
        im.polygon(&corners, Some(color("track")), Some(color("metal_dark")));
        // Track pads shift with the animation phase.
        let phase = if state == "move" || state == "turn" {
            frame % 4
        } else {
            0
        };
        for side in [-1, 1] {
            let mut t = -length + 2;
            while t < length {
                let a = xy(t, side * (width - 1));
                let b = xy(length.min(t + 2), side * (width - 1));
                let pad = if (t + phase).rem_euclid(3) == 0 {
                    color("metal")
                } else {
                    color("metal_dark")
                };
                im.line(a.0, a.1, b.0, b.1, pad, 1);
                t += 4;
            }
        }
        let inner = [
            xy(-length + 2, -width + 2),
            xy(length - 2, -width + 2),
            xy(length - 2, width - 2),
            xy(-length + 2, width - 2),
        ];
        im.polygon(&inner, Some(body), Some(dark));
        // Raised engine deck and highlight panels.
        let rear1 = xy(-length + 4, -width + 3);
        let rear2 = xy(-length + 4, width - 3);
        im.line(rear1.0, rear1.1, rear2.0, rear2.1, dark, 2);
        let front1 = xy(length - 4, -width + 3);
        let front2 = xy(length - 4, width - 3);
        im.line(front1.0, front1.1, front2.0, front2.1, body, 1);
        // Faction mark on the rear deck.
        let mark = if faction == "gdi" {
            color("white")
        } else {
            color("fire")
        };
        let (mx, my) = xy(-length + 1, 0);
        im.set_pixel(mx, my, mark);
        // Turret ring, highlight arc and directional barrel.
        let tcx = cx + dx * recoil;
        let tcy = cy + dy * recoil + bob;
        im.ellipse(
            tcx - 5,
            tcy - 5,
            tcx + 5,
            tcy + 5,
            Some(dark),
            Some(color("metal_dark")),
            1,
        );
        im.arc(
            tcx - 4,
            tcy - 4,
            tcx + 4,
            tcy + 4,
            195.0,
            330.0,
            color("metal"),
            1,
        );
        im.ellipse(
            tcx - 2,
            tcy - 2,
            tcx + 2,
            tcy + 2,
            Some(body),
            Some(dark),
            1,
        );
        let blen = if unit == "artillery" { 12 } else { 8 };
        let ex = tcx + dx * blen;
        let ey = tcy + dy * blen;
        im.line(tcx, tcy, ex, ey, color("metal_dark"), 4);
        im.line(tcx, tcy, ex, ey, color("metal"), 2);
        if state == "attack" && frame == 2 {
            im.polygon(
                &[
                    (ex + dy * 3, ey - dx * 3),
                    (ex + dx * 4, ey + dy * 4),
                    (ex - dy * 3, ey + dx * 3),
                ],
                Some(color("muzzle")),
                None,
            );
        }
        if unit == "harvester" {
            if state == "harvest" || state == "unload" {
                let cargo = if state == "harvest" {
                    color("tiberium")
                } else {
                    color("sand")
                };
                im.rect(
                    cx + dx * 7 - 3,
                    cy + dy * 7 - 3,
                    cx + dx * 7 + 3,
                    cy + dy * 7 + 3,
                    Some(cargo),
                    None,
                    0,
                );
            }
            im.rect(
                cx - 3,
                cy - 3 + bob,
                cx + 3,
                cy + 3 + bob,
                Some(color("metal")),
                None,
                0,
            );
        }
        if state == "deploy" && unit == "artillery" {
            im.line(
                cx - dy * 8,
                cy + dx * 8,
                cx + dy * 8,
                cy - dx * 8,
                color("metal"),
                2,
            );
        }
    } else if matches!(unit, "infantry" | "engineer" | "rocket_soldier") {
        let stride_table: [i32; 4] = if state == "move" {
            [-3, 1, 3, -1]
        } else {
            [0, 1, 0, -1]
        };
        let stride = stride_table[(frame % 4) as usize];
        let hx = cx + dx * 3;
        let hy = cy + dy * 3 + bob;
        im.ellipse(hx - 3, hy - 3, hx + 3, hy + 3, Some(color("skin")), None, 1);
        im.polygon(
            &[
                (cx - dy * 4, cy + dx * 4 + bob),
                (cx + dy * 4, cy - dx * 4 + bob),
                (cx - dx * 3, cy - dy * 3 + bob),
            ],
            Some(body),
            None,
        );
        for side in [-1, 1] {
            let sx = cx + -dy * side * 2;
            let sy = cy + dx * side * 2 + bob;
            // Python: sx+dx*4-dy*side*stride//2 — floor division applies to
            // the whole product, not just stride.
            let ex = sx + dx * 4 - (dy * side * stride).div_euclid(2);
            let ey = sy + dy * 4 + (dx * side * stride).div_euclid(2);
            im.line(sx, sy, ex, ey, dark, 2);
            let lx = cx - dy * side * 2;
            let ly = cy + dy * 3 + bob;
            let fx = lx + dx * 4 - dy * side * stride;
            let fy = ly + dy * 4 + dx * side * stride;
            im.line(lx, ly, fx, fy, dark, 2);
        }
        let wx = cx + dx * (8 + recoil);
        let wy = cy + dy * (8 + recoil) + bob;
        if unit == "rocket_soldier" {
            im.line(cx + dx * 2, cy + dy * 2, wx, wy, color("metal"), 3);
        } else if unit == "engineer" {
            im.rect(
                wx - 2,
                wy - 2,
                wx + 2,
                wy + 2,
                Some(color("metal")),
                None,
                0,
            );
            if state == "repair" && (frame == 1 || frame == 2) {
                im.set_pixel(wx + dx * 2, wy + dy * 2, color("muzzle"));
            }
        } else {
            im.line(cx + dx * 2, cy + dy * 2, wx, wy, color("metal"), 2);
        }
        if state == "attack" && frame == 2 {
            im.line(wx, wy, wx + dx * 5, wy + dy * 5, color("muzzle"), 3);
        }
    } else if unit == "buggy" {
        let (px, py) = (-dy, dx);
        let pts = [
            (cx - dx * 8 - px * 5, cy - dy * 8 - py * 5 + bob),
            (cx + dx * 8 - px * 5, cy + dy * 8 - py * 5 + bob),
            (cx + dx * 8 + px * 5, cy + dy * 8 + py * 5 + bob),
            (cx - dx * 8 + px * 5, cy - dy * 8 + py * 5 + bob),
        ];
        im.polygon(&pts, Some(body), Some(dark));
        for side in [-1, 1] {
            let wx = cx + px * side * 5;
            let wy = cy + py * side * 5 + bob;
            im.ellipse(
                wx - 2,
                wy - 3,
                wx + 2,
                wy + 3,
                Some(color("track")),
                None,
                1,
            );
            if state == "move" || state == "turn" {
                let phase = frame % 4;
                im.set_pixel(wx + (phase - 1), wy + (phase % 3) - 1, color("metal"));
            }
        }
        let ex = cx + dx * (9 + recoil);
        let ey = cy + dy * (9 + recoil) + bob;
        im.line(cx, cy, ex, ey, color("metal"), 3);
        if state == "attack" && frame == 2 {
            im.ellipse(
                ex - 2,
                ey - 2,
                ex + 2,
                ey + 2,
                Some(color("muzzle")),
                None,
                1,
            );
        }
    }

    // State timing accents: engine light, cargo, tools.
    if state == "idle" && frame % 2 == 1 {
        im.set_pixel(cx + dx * 5, cy + dy * 5, color("muzzle"));
    } else if state == "harvest" {
        im.set_pixel(
            cx + dx * (11 + frame),
            cy + dy * (11 + frame),
            color("tiberium"),
        );
    } else if state == "unload" {
        for j in 0..=frame {
            im.set_pixel(cx + 5 + j, cy + 10, color("sand"));
            im.set_pixel(2 + j * 2, 29, color("sand_light"));
        }
    } else if state == "deploy" {
        im.set_pixel(cx + frame - 1, cy + 8, color("metal"));
    } else if state == "repair" && frame % 2 == 1 {
        im.set_pixel(cx + dx * 10, cy + dy * 10, color("muzzle"));
    }
    im
}

/// Complete directional unit sprite at a damage condition.
///
/// Conditions are engine-ready full frames, not compositor decals: damage
/// marks are deterministic and attached relative to the hull facing.
pub fn unit_condition_frame(
    faction: &str,
    unit: &str,
    facing: u8,
    condition: &str,
) -> Result<Canvas, UnknownCondition> {
    if !CONDITIONS.contains(&condition) {
        return Err(UnknownCondition(condition.to_string()));
    }
    if condition == "destroyed" {
        // Wreck pose from early in the death animation, plus charred cracks.
        let mut im = unit_frame(faction, unit, facing, 2, "death");
        polyline(
            &mut im,
            &[7, 21, 12, 17, 16, 23, 22, 18],
            color("metal_dark"),
            2,
        );
        polyline(&mut im, &[11, 13, 15, 15, 18, 11], color("rock_light"), 1);
        im.set_pixel(8, 25, color("fire"));
        im.set_pixel(23, 20, color("rock_light"));
        return Ok(im);
    }

    let mut im = unit_frame(faction, unit, facing, 0, "idle");
    if condition == "intact" {
        return Ok(im);
    }

    let (dx, dy) = DIRS[(facing % 8) as usize];
    // Impact point sits forward-right of the facing, not at a fixed pixel.
    let (px, py) = (-dy, dx);
    let hx = (16 + dx * 5 + px * 3).clamp(4, 27);
    let hy = (16 + dy * 5 + py * 3).clamp(4, 27);
    if condition == "damaged" {
        im.ellipse(
            hx - 2,
            hy - 2,
            hx + 2,
            hy + 2,
            Some(color("rock_dark")),
            None,
            1,
        );
        polyline(
            &mut im,
            &[hx - 3, hy, hx, hy + 2, hx + 3, hy - 2],
            color("metal_dark"),
            1,
        );
        im.set_pixel(hx + 2, hy + 1, color("rock_light"));
    } else {
        // Critical: heavy scorch plus fire and smoke accents.
        let scorch: Rgba = [0, 0, 0, 235];
        im.ellipse(hx - 4, hy - 3, hx + 4, hy + 3, Some(scorch), None, 1);
        polyline(
            &mut im,
            &[
                hx - 4,
                hy - 1,
                hx - 1,
                hy + 2,
                hx + 2,
                hy - 2,
                hx + 4,
                hy + 1,
            ],
            color("rock_light"),
            1,
        );
        im.set_pixel(
            (hx + dx).clamp(1, 30),
            (hy + dy).clamp(1, 30),
            color("fire"),
        );
        im.set_pixel(
            (hx + dx * 2).clamp(1, 30),
            (hy + dy * 2).clamp(1, 30),
            color("fire_light"),
        );
        im.set_pixel(
            (hx - dx * 2).clamp(2, 29),
            (hy - dy * 2).clamp(2, 29),
            color("smoke"),
        );
    }
    Ok(im)
}

/// Transparent turret overlay: aim direction independent of hull facing.
///
/// `recoil_frame`: 0 = ready, 1 = kickback (+muzzle), 2 = recovery.
pub fn independent_turret_frame(
    faction: &str,
    unit: &str,
    turret_facing: u8,
    recoil_frame: u8,
) -> Canvas {
    let mut im = Canvas::transparent(32, 32);
    let (dx, dy) = DIRS[(turret_facing % 8) as usize];
    let (body, dark) = faction_colors(faction);
    let kick = match recoil_frame {
        1 => 2,
        2 => 1,
        _ => 0,
    };
    let cx = 16 - dx * kick;
    let cy = 16 - dy * kick;
    im.ellipse(
        cx - 4,
        cy - 4,
        cx + 4,
        cy + 4,
        Some(dark),
        Some(color("metal_dark")),
        1,
    );
    im.ellipse(
        cx - 2,
        cy - 2,
        cx + 2,
        cy + 2,
        Some(body),
        Some(color("metal")),
        1,
    );
    let length = if unit == "artillery" { 12 } else { 9 };
    let ex = cx + dx * length;
    let ey = cy + dy * length;
    im.line(cx, cy, ex, ey, color("metal_dark"), 4);
    im.line(cx, cy, ex, ey, color("metal"), 2);
    if recoil_frame == 1 {
        im.polygon(
            &[
                (ex + dy * 3, ey - dx * 3),
                (ex + dx * 4, ey + dy * 4),
                (ex - dy * 3, ey + dx * 3),
            ],
            Some(color("muzzle")),
            None,
        );
    }
    im
}

/// One frame of an effect animation (explosion, smoke, muzzle flash,
/// multi-stage destruction).
pub fn effect_frame(kind: &str, frame: u8) -> Canvas {
    let mut im = Canvas::transparent(32, 32);
    let frame = frame as i32;
    match kind {
        "explosion" => {
            const RADII: [i32; 6] = [4, 8, 13, 10, 6, 3];
            let r = RADII[(frame % 6) as usize];
            let cx = 16 + (frame % 3) - 1;
            let cy = 16 - (frame % 2);
            let fire: Rgba = [color("fire")[0], color("fire")[1], color("fire")[2], 230];
            im.ellipse(cx - r, cy - r, cx + r, cy + r, Some(fire), None, 1);
            if r > 3 {
                im.ellipse(
                    cx - r / 2,
                    cy - r / 2,
                    cx + r / 2,
                    cy + r / 2,
                    Some(color("fire_light")),
                    None,
                    1,
                );
            }
            if frame % 2 == 1 {
                im.polygon(
                    &[(cx, cy - r - 2), (cx + 3, cy - 2), (cx - 2, cy + 2)],
                    Some(color("muzzle")),
                    None,
                );
            }
        }
        "smoke" => {
            let r = 3 + frame % 5;
            let cx = 14 + frame % 4;
            let cy = 22 - frame * 2;
            let alpha = 60.max(210 - frame * 22) as u8;
            let smoke: Rgba = [
                color("smoke")[0],
                color("smoke")[1],
                color("smoke")[2],
                alpha,
            ];
            im.ellipse(cx - r, cy - r, cx + r, cy + r, Some(smoke), None, 1);
        }
        "muzzle_flash" => {
            let col = if frame % 2 == 1 {
                color("muzzle")
            } else {
                color("fire")
            };
            im.polygon(
                &[
                    (5, 16),
                    (13, 12 + frame % 3),
                    (19, 6),
                    (18, 15),
                    (28, 16),
                    (18, 19),
                    (20, 26),
                    (12, 20),
                ],
                Some(col),
                None,
            );
        }
        "destruction" => {
            if frame <= 2 {
                const RADII: [i32; 3] = [5, 10, 14];
                let r = RADII[frame as usize];
                let alpha = 100.max(240 - frame * 35) as u8;
                let fire: Rgba = [color("fire")[0], color("fire")[1], color("fire")[2], alpha];
                im.ellipse(16 - r, 16 - r, 16 + r, 16 + r, Some(fire), None, 1);
                im.ellipse(
                    16 - r / 2,
                    16 - r / 2,
                    16 + r / 2,
                    16 + r / 2,
                    Some(color("muzzle")),
                    None,
                    1,
                );
                for i in 0..(4 + frame * 2) {
                    let angle = (i * 137 + frame * 29) as f64 * std::f64::consts::PI / 180.0;
                    let x = (16.0 + angle.cos() * (r + 2) as f64) as i32;
                    let y = (16.0 + angle.sin() * (r + 2) as f64) as i32;
                    im.rect(
                        x - 1,
                        y - 1,
                        x + 1,
                        y + 1,
                        Some(color("metal_dark")),
                        None,
                        0,
                    );
                }
            } else {
                let r = 3 + (frame - 3) * 2;
                let cx = 13 + frame % 3;
                let cy = 22 - (frame - 3) * 2;
                let alpha = 35.max(190 - (frame - 3) * 28) as u8;
                let smoke: Rgba = [
                    color("smoke")[0],
                    color("smoke")[1],
                    color("smoke")[2],
                    alpha,
                ];
                im.ellipse(cx - r, cy - r, cx + r, cy + r, Some(smoke), None, 1);
                if frame < 6 {
                    let debris: Rgba = [
                        color("metal_dark")[0],
                        color("metal_dark")[1],
                        color("metal_dark")[2],
                        220,
                    ];
                    im.line(7 + frame, 25, 11 + frame, 22, debris, 2);
                }
            }
        }
        _ => {}
    }
    im
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(im: &Canvas) -> Vec<u8> {
        im.as_rgba().as_raw().clone()
    }

    #[test]
    fn state_tables_match_python() {
        assert_eq!(state_frames("idle"), 2);
        assert_eq!(state_frames("death"), 6);
        assert_eq!(state_frames("harvest"), 4);
        assert_eq!(frame_duration_ms("idle"), 180);
        assert_eq!(frame_duration_ms("attack"), 110);
        assert_eq!(frame_duration_ms("death"), 140);
        assert_eq!(frame_duration_ms("move"), 120);
        assert!(state_loops("idle") && state_loops("move") && state_loops("harvest"));
        assert!(!state_loops("attack") && !state_loops("death"));
        assert_eq!(effect_duration_ms("smoke"), 140);
        assert_eq!(effect_duration_ms("explosion"), 90);
        let total: u32 = UNIT_STATES
            .iter()
            .flat_map(|(_, states)| states.iter())
            .map(|s| state_frames(s) as u32)
            .sum();
        // Python: planned_unit_frames = 2 factions * 8 facings * sum of
        // STATE_FRAMES across every unit's states = 16 * 136 = 2176.
        assert_eq!(total, 136);
        assert_eq!(2 * 8 * total, 2176);
    }

    #[test]
    fn unit_frames_are_deterministic() {
        let a = unit_frame("gdi", "tank", 1, 2, "move");
        let b = unit_frame("gdi", "tank", 1, 2, "move");
        assert_eq!(hash(&a), hash(&b));
        assert_eq!((a.width(), a.height()), (32, 32));
    }

    #[test]
    fn every_unit_and_facing_renders_nonempty_on_move() {
        for (unit, _) in UNIT_STATES {
            for facing in 0..8u8 {
                let im = unit_frame("gdi", unit, facing, 1, "move");
                let empty = im.as_rgba().as_raw().iter().all(|&b| b == 0);
                assert!(!empty, "{unit} facing {facing} rendered empty");
            }
        }
    }

    #[test]
    fn tank_facings_differ() {
        let n = hash(&unit_frame("gdi", "tank", 0, 1, "move"));
        let e = hash(&unit_frame("gdi", "tank", 2, 1, "move"));
        assert_ne!(n, e);
    }

    #[test]
    fn adjacent_animation_frames_differ() {
        for unit in ["tank", "infantry"] {
            let states = UNIT_STATES.iter().find(|(u, _)| *u == unit).unwrap().1;
            for state in states {
                let count = state_frames(state);
                for facing in 0..8u8 {
                    for frame in 0..count - 1 {
                        let a = hash(&unit_frame("gdi", unit, facing, frame, state));
                        let b = hash(&unit_frame("gdi", unit, facing, frame + 1, state));
                        assert_ne!(
                            a,
                            b,
                            "{unit}/{state}/facing {facing}: frames {frame}=={}",
                            frame + 1
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn factions_render_differently() {
        let gdi = hash(&unit_frame("gdi", "tank", 2, 1, "move"));
        let nod = hash(&unit_frame("nod", "tank", 2, 1, "move"));
        assert_ne!(gdi, nod);
    }

    #[test]
    fn unknown_faction_renders_as_nod() {
        let nod = hash(&unit_frame("nod", "tank", 3, 1, "move"));
        let other = hash(&unit_frame("some_mod", "tank", 3, 1, "move"));
        assert_eq!(nod, other);
    }

    #[test]
    fn conditions_are_deterministic_and_distinct() {
        let intact = hash(&unit_condition_frame("gdi", "tank", 3, "intact").unwrap());
        let again = hash(&unit_condition_frame("gdi", "tank", 3, "intact").unwrap());
        assert_eq!(intact, again);
        for condition in ["damaged", "critical", "destroyed"] {
            let im = unit_condition_frame("nod", "infantry", 5, condition).unwrap();
            assert_ne!(hash(&im), intact, "{condition} must differ from intact");
            assert_eq!((im.width(), im.height()), (32, 32));
        }
        assert!(unit_condition_frame("gdi", "tank", 0, "on_fire").is_err());
    }

    #[test]
    fn turrets_recoil_and_facing_change_output() {
        let ready = hash(&independent_turret_frame("gdi", "tank", 0, 0));
        let kick = hash(&independent_turret_frame("gdi", "tank", 0, 1));
        let recover = hash(&independent_turret_frame("gdi", "tank", 0, 2));
        assert_ne!(ready, kick);
        assert_ne!(kick, recover);
        let south = hash(&independent_turret_frame("gdi", "tank", 4, 0));
        assert_ne!(ready, south);
        assert_eq!(ready, hash(&independent_turret_frame("gdi", "tank", 0, 0)));
    }

    #[test]
    fn effect_frames_render_and_vary() {
        for (kind, count) in EFFECT_KINDS {
            let mut seen = std::collections::HashSet::new();
            for frame in 0..count {
                let im = effect_frame(kind, frame);
                assert_eq!((im.width(), im.height()), (32, 32));
                let empty = im.as_rgba().as_raw().iter().all(|&b| b == 0);
                assert!(!empty, "{kind} frame {frame} rendered empty");
                seen.insert(hash(&im));
            }
            assert_eq!(seen.len() as u8, count, "{kind} frames must all differ");
        }
    }
}
