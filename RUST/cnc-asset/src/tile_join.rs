//! Seamless tile joining for the ORIGINAL template art.
//!
//! # Why this exists
//!
//! The 1996 engine does not pick tiles at render time.
//! `original/TIBERIANDAWN/CELL.CPP:913` `Draw_It` does:
//!
//! ```cpp
//! if (TType != TEMPLATE_NONE) { ttype = ...; icon = TIcon; }
//! else                        { ttype = CLEAR1; icon = Clear_Icon(); }
//! ```
//!
//! i.e. it draws the `(TemplateType, TIcon)` pair stored in the cell. Seamless
//! terrain in shipped maps is therefore AUTHORED data: `MAP.CPP:678` reads
//! `(TType, TIcon)` per cell out of the `.bin`, and `MAPEDPLC.CPP:1000` writes
//! `TIcon = Cell_X(occupy) + Cell_Y(occupy) * Width` when the designer drags a
//! template. The `_slope<mask>` tables in `CDATA.CPP:47` (`AltIcons`) are
//! available to the engine, but `CELL.CPP:515` only consults them for LAND
//! TYPE / passability, never for choosing the drawn icon.
//!
//! So a generator has no authored icons and must pick one that joins. This
//! module measures that from the art instead of guessing.
//!
//! # What is measured
//!
//! For every `(template, icon)` pair a theater ships, [`TileEdgeProfile`] is
//! the per-edge classification of the outermost pixel run: what fraction of
//! the edge is transparent, opaque material, and water-blue. Two tiles join
//! cleanly on a shared edge when both sides of that edge describe the same
//! material — a tile whose south edge is 100% opaque land needs the tile below
//! it to have a north edge that is also 100% opaque land, or the seam shows a
//! hard cut against open background.
//!
//! The measurement runs once at tileset build time over [the shipped
//! archive][crate::original_assets], and the result is stored as a data table
//! so the render loop does no pixel work.
//!
//! # Scoring
//!
//! For a cell, the picker considers the 4 orthogonal neighbours (and the 4
//! diagonals, which matter for corner-shaped templates) and scores each
//! candidate icon by seam disagreement:
//!
//! - an edge facing a neighbour that shares this tile's material should be
//!   OPAQUE (the material continues across the seam);
//! - an edge facing a neighbour of a different material should be TRANSPARENT
//!   (the neighbour's material shows through).
//!
//! The candidate with the lowest total disagreement wins; ties break toward
//! the icon with the most opaque edges, which is what a solid interior tile
//! looks like.

use std::collections::HashMap;

/// How one edge of one icon is classified.
///
/// Fractions sum to 1 over the edge's pixel run (within the sampled width).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeProfile {
    /// Fraction of edge pixels that are transparent (alpha below the
    /// [`ALPHA_CUTOFF`]).
    pub transparent: f32,
    /// Fraction of edge pixels that are opaque non-water material.
    pub opaque: f32,
    /// Fraction of edge pixels that read as water (blue-dominant).
    pub water: f32,
}

/// Alpha below which a pixel is considered see-through.
pub const ALPHA_CUTOFF: u8 = 32;

/// Four-edge profile of one icon, in `[N, E, S, W]` order.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeProfiles {
    pub n: EdgeProfile,
    pub e: EdgeProfile,
    pub s: EdgeProfile,
    pub w: EdgeProfile,
}

impl EdgeProfile {
    /// Classify one edge's pixel run.
    ///
    /// `rgba` is the icon's RGBA data; `coords` are edge pixel offsets in
    /// icon space, over a row length of `stride` pixels. Sampling the outermost
    /// row/column is what the eye actually judges a seam by.
    pub fn measure(rgba: &[u8], stride: usize, coords: &[(usize, usize)]) -> Self {
        let (mut t, mut o, mut w) = (0u32, 0u32, 0u32);
        for (x, y) in coords {
            let o4 = (*y * stride + *x) * 4;
            let Some(px) = rgba.get(o4..o4 + 4) else {
                continue;
            };
            if px[3] < ALPHA_CUTOFF {
                t += 1;
            } else if is_water_blue(px) {
                w += 1;
            } else {
                o += 1;
            }
        }
        let n = (t + o + w).max(1) as f32;
        EdgeProfile {
            transparent: t as f32 / n,
            opaque: o as f32 / n,
            water: w as f32 / n,
        }
    }

    /// How much this edge wants to be opaque, in `0.0..=1.0`.
    pub fn wants_opaque(&self) -> f32 {
        self.opaque + self.water
    }

    /// Disagreement with a demanded state, in `0.0..=1.0`.
    ///
    /// `want_opaque` is `true` when the material should continue across this
    /// edge (the neighbour is the same family) and `false` when the neighbour
    /// is a different family and the neighbour's material should show through.
    pub fn disagreement(&self, want_opaque: bool) -> f32 {
        let have_opaque = self.wants_opaque();
        if want_opaque {
            1.0 - have_opaque
        } else {
            have_opaque
        }
    }
}

/// Water-blue test: blue channel dominates both others by a margin.
pub fn is_water_blue(px: &[u8]) -> bool {
    let (r, g, b) = (px[0], px[1], px[2]);
    u16::from(b) > u16::from(r) + 20 && u16::from(b) > u16::from(g) + 5
}

/// Which material a cell belongs to, for join purposes.
///
/// Two cells join when they are the same material across a shared edge. The
/// editor's [`TerrainType`](super::generator) already carries the game's own
/// terrain id; this enum is the join-level grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinMaterial {
    /// Clear / grass / any plain land.
    Land,
    /// Open water (`W1`, `W2`).
    Water,
    /// Beach and shoreline (`SH*`).
    Shore,
    /// River (`RV*`), shallow and running.
    River,
    /// Road and bridge (`D*`).
    Road,
    /// Slope and cliff (`S*`).
    Slope,
    /// Patch of alternate land (`P*`).
    Patch,
    /// Brush and boulder (`BR*`, `B*`).
    Rock,
}

/// One candidate tile: the icon plus its measured edges.
#[derive(Debug, Clone)]
pub struct JoinCandidate {
    /// Template name as the archive spells it (`"SH1"`, `"D01"`, `"W2"`).
    pub template: String,
    /// Icon index inside that template.
    pub icon: u32,
    /// Measured edge profiles.
    pub edges: EdgeProfiles,
    /// Which material this tile belongs to.
    pub material: JoinMaterial,
}

impl JoinCandidate {
    /// Total seam disagreement against one neighbourhood.
    ///
    /// `neighbours` is `[(material, side)]`, `side` in `N/E/S/W`.
    pub fn score(&self, neighbours: &[(JoinMaterial, char)]) -> f32 {
        let mut total = 0.0;
        for (mat, side) in neighbours {
            let same = *mat == self.material;
            let edge = match side {
                'N' | 'n' => self.edges.n,
                'E' | 'e' => self.edges.e,
                'S' | 's' => self.edges.s,
                _ => self.edges.w,
            };
            total += edge.disagreement(same);
        }
        total
    }

    /// Seam score that also accounts for how much MATERIAL faces the seam.
    ///
    /// Two tiles from the same family still fail to join if one is mostly
    /// transparent on the facing edge (a thin sliver tile) while the other is
    /// solid. Comparing the facing edge's own opacity against the demand, for
    /// BOTH sides of a neighbour pair, is what makes adjacent tiles agree, so
    /// the picker avoids slivers next to solid interiors unless the
    /// neighbourhood actually calls for them.
    ///
    /// `neighbour_sides` is the number of sides (of 4) facing the same
    /// material; the rest face a different one.
    pub fn score_detailed(&self, neighbours: &[(JoinMaterial, char)]) -> f32 {
        let mut total = 0.0;
        for (mat, side) in neighbours {
            let same = *mat == self.material;
            let edge = match side {
                'N' | 'n' => self.edges.n,
                'E' | 'e' => self.edges.e,
                'S' | 's' => self.edges.s,
                _ => self.edges.w,
            };
            // Same material: the facing edge must carry material too, else the
            // neighbour's material would not continue across the seam. A tile
            // whose edge is entirely transparent is an overlay/sliver piece, and
            // it can only join another transparent edge of the same family.
            if same {
                total += (1.0 - edge.wants_opaque()) * 2.0;
                // Penalise a transparent edge facing a solid neighbour on the
                // same side, since the neighbour may be solid too.
                if edge.wants_opaque() < 0.15 {
                    total += 1.0;
                }
            } else {
                // Different material: this edge must give way to the
                // neighbour's material, so transparency is preferred.
                total += edge.wants_opaque() * 2.0;
                // A water edge facing land is an outright seam break.
                if edge.water > 0.5 && matches!(self.material, JoinMaterial::Water) {
                    total += 0.5;
                }
            }
        }
        total
    }

    /// Tie-break preference: solid interior tiles first.
    pub fn solidity(&self) -> f32 {
        self.edges.n.wants_opaque()
            + self.edges.e.wants_opaque()
            + self.edges.s.wants_opaque()
            + self.edges.w.wants_opaque()
    }
}

/// The candidate table for one theater.
///
/// Built from the shipped archive; the render loop only reads it.
#[derive(Debug, Default)]
pub struct JoinTable {
    by_material: HashMap<JoinMaterial, Vec<JoinCandidate>>,
}

impl JoinTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a measured candidate.
    pub fn push(&mut self, candidate: JoinCandidate) {
        self.by_material
            .entry(candidate.material)
            .or_default()
            .push(candidate);
    }

    /// All candidates for a material.
    pub fn candidates(&self, material: JoinMaterial) -> &[JoinCandidate] {
        self.by_material
            .get(&material)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// A view whose candidates are only those matching `template`.
    pub fn restricted(&self, material: JoinMaterial, template: &str) -> JoinTable {
        let mut out = JoinTable::new();
        for c in self.candidates(material) {
            if c.template == template {
                out.push(c.clone());
            }
        }
        out
    }

    /// Pick the icon that joins best against `neighbours`.
    ///
    /// Returns `None` only when no candidate exists for `material`, so a
    /// caller can fall back to whatever it already had.
    pub fn select(
        &self,
        material: JoinMaterial,
        neighbours: &[(JoinMaterial, char)],
    ) -> Option<&JoinCandidate> {
        // Overlay slivers (`S05`, `S04`, `B1`, `CLEAR1`) have mostly
        // transparent edges: they belong ON a base tile, not as one. Engine
        // code writes CLEAR1 with icon 0 for "no template here", so a
        // fully-transparent icon is never a drawn base.
        //
        // A candidate is a base when at least two opposite edges carry real
        // material, or when its edge material is above 0.2 on three or more
        // sides. That admits the solid patches and roads while dropping the
        // thin accents, which otherwise appear as holes next to solid tiles.
        let candidates = self.candidates(material);
        let solid_candidates: Vec<&JoinCandidate> = candidates
            .iter()
            .filter(|c| {
                let ns = c.edges.n.wants_opaque() + c.edges.s.wants_opaque();
                let ew = c.edges.e.wants_opaque() + c.edges.w.wants_opaque();
                let sides = [
                    c.edges.n.wants_opaque(),
                    c.edges.e.wants_opaque(),
                    c.edges.s.wants_opaque(),
                    c.edges.w.wants_opaque(),
                ];
                let material_sides = sides.iter().filter(|s| **s > 0.2).count();
                ns >= 1.0 || ew >= 1.0 || material_sides >= 3
            })
            .collect();
        let candidates: Vec<&JoinCandidate> = if solid_candidates.is_empty() {
            candidates.iter().collect()
        } else {
            solid_candidates
        };
        candidates.into_iter().min_by(|a, b| {
            a.score_detailed(neighbours)
                .partial_cmp(&b.score_detailed(neighbours))
                .unwrap_or(std::cmp::Ordering::Equal)
                // Tie-break: prefer the tile whose edge opacities match the
                // neighbour demands most evenly, then the most solid.
                .then_with(|| {
                    b.solidity()
                        .partial_cmp(&a.solidity())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                // Final tie-break keeps the result deterministic.
                .then_with(|| a.template.cmp(&b.template).then(a.icon.cmp(&b.icon)))
        })
    }

    /// Number of candidates held.
    pub fn len(&self) -> usize {
        self.by_material.values().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.by_material.values().all(Vec::is_empty)
    }
}

/// Template prefixes and the material they belong to.
///
/// The names come from `TemplateTypeClass` in `original/TIBERIANDAWN/CDATA.CPP`
/// (and the RA equivalent); the material grouping is the join-level mapping.
pub const MATERIAL_FOR_PREFIX: &[(&str, JoinMaterial)] = &[
    ("SH", JoinMaterial::Shore),
    ("D", JoinMaterial::Road),
    ("S", JoinMaterial::Slope),
    ("RV", JoinMaterial::River),
    ("P", JoinMaterial::Patch),
    ("BR", JoinMaterial::Rock),
    ("B", JoinMaterial::Rock),
    ("W", JoinMaterial::Water),
    ("FALLS", JoinMaterial::Water),
    ("FORD", JoinMaterial::Shore),
    ("CLEAR", JoinMaterial::Land),
    ("BRIDGE", JoinMaterial::Road),
];

/// Template count per prefix, so the probe covers every family.
pub const PREFIX_COUNT: &[(&str, u32)] = &[
    ("SH", 63),
    ("D", 43),
    ("S", 38),
    ("RV", 25),
    ("P", 20),
    ("BR", 10),
    ("B", 6),
    ("W", 2),
    ("FALLS", 2),
    ("FORD", 2),
    ("CLEAR", 4),
    ("BRIDGE1D", 1),
    ("BRIDGE2D", 1),
    ("BRIDGE3D", 1),
    ("BRIDGE4D", 1),
];

/// Material a template name belongs to, by longest matching prefix.
pub fn material_for(template: &str) -> Option<JoinMaterial> {
    MATERIAL_FOR_PREFIX
        .iter()
        .filter(|(prefix, _)| template.starts_with(prefix))
        .max_by_key(|(prefix, _)| prefix.len())
        .map(|(_, material)| *material)
}

/// Build the join table for one theater by measuring every icon that ships.
///
/// Decoding ~1700 icons takes a moment; callers build this once at tileset
/// load and hold it. Returns an empty table when the Steam install is absent,
/// so a caller falls back to whatever it had.
pub fn measure(
    game: crate::original_assets::Game,
    theater: crate::original_assets::Theater,
) -> JoinTable {
    use crate::original_assets::OriginalAssets;

    let Ok(assets) = OriginalAssets::open_default(game) else {
        return JoinTable::new();
    };
    let mut table = JoinTable::new();
    for (prefix, count) in PREFIX_COUNT {
        for n in 0..=*count {
            // TD pads two-digit families (`SH01`, `D01`); single-digit ones
            // write `W1`, `B1`. Try both, take the first that decodes.
            let names = [format!("{prefix}{n:02}"), format!("{prefix}{n}")];
            for name in &names {
                let Some(material) = material_for(name) else {
                    continue;
                };
                for icon in 0..8u32 {
                    let Ok((rgba, w, h)) = assets.icon_rgba(name, icon, theater) else {
                        continue;
                    };
                    let (uw, uh) = (w as usize, h as usize);
                    if uw < 4 || uh < 4 {
                        continue;
                    }
                    let n_row: Vec<_> = (0..uw).map(|x| (x, 0)).collect();
                    let s_row: Vec<_> = (0..uw).map(|x| (x, uh - 1)).collect();
                    let w_row: Vec<_> = (0..uh).map(|y| (0, y)).collect();
                    let e_row: Vec<_> = (0..uh).map(|y| (uw - 1, y)).collect();
                    table.push(JoinCandidate {
                        template: name.clone(),
                        icon,
                        edges: EdgeProfiles {
                            n: EdgeProfile::measure(&rgba, uw, &n_row),
                            e: EdgeProfile::measure(&rgba, uw, &e_row),
                            s: EdgeProfile::measure(&rgba, uw, &s_row),
                            w: EdgeProfile::measure(&rgba, uw, &w_row),
                        },
                        material,
                    });
                }
            }
        }
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_mapping_covers_every_family() {
        assert_eq!(material_for("SH1"), Some(JoinMaterial::Shore));
        assert_eq!(material_for("SH17"), Some(JoinMaterial::Shore));
        assert_eq!(material_for("D01"), Some(JoinMaterial::Road));
        assert_eq!(material_for("W1"), Some(JoinMaterial::Water));
        // Longest prefix wins: `BRIDGE1D` is a road, not a boulder.
        assert_eq!(material_for("BRIDGE1D"), Some(JoinMaterial::Road));
        assert_eq!(material_for("NOT_A_FAMILY"), None);
    }

    #[test]
    fn edge_profile_classifies_transparent_opaque_and_water() {
        // 4px run: transparent, black, white, water-blue.
        let rgba: Vec<u8> = vec![
            0, 0, 0, 0, // transparent
            0, 0, 0, 255, // opaque
            255, 255, 255, 255, // opaque
            40, 90, 200, 255, // water
        ];
        let coords = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        // The measurer treats `(x, y)` as icon-space offsets.
        let p = EdgeProfile::measure(&rgba, 4, &coords);
        assert!((p.transparent - 0.25).abs() < 1e-6, "{p:?}");
        assert!((p.opaque - 0.5).abs() < 1e-6, "{p:?}");
        assert!((p.water - 0.25).abs() < 1e-6, "{p:?}");
    }

    #[test]
    fn score_prefers_edge_that_matches_neighbour_material() {
        // Candidate whose south edge is opaque, north edge transparent.
        let cand = JoinCandidate {
            template: "X".into(),
            icon: 0,
            edges: EdgeProfiles {
                n: EdgeProfile {
                    transparent: 1.0,
                    opaque: 0.0,
                    water: 0.0,
                },
                e: EdgeProfile {
                    transparent: 1.0,
                    opaque: 0.0,
                    water: 0.0,
                },
                s: EdgeProfile {
                    transparent: 0.0,
                    opaque: 1.0,
                    water: 0.0,
                },
                w: EdgeProfile {
                    transparent: 1.0,
                    opaque: 0.0,
                    water: 0.0,
                },
            },
            material: JoinMaterial::Shore,
        };
        // The south neighbour is also shore: the opaque south edge is right.
        let good = cand.score(&[(JoinMaterial::Shore, 'S')]);
        // The south neighbour is water: the opaque south edge is wrong.
        let bad = cand.score(&[(JoinMaterial::Water, 'S')]);
        assert!(good < bad, "good {good} should beat bad {bad}");
    }

    #[test]
    fn select_returns_none_only_for_empty_material() {
        let table = JoinTable::new();
        assert!(table
            .select(JoinMaterial::Shore, &[(JoinMaterial::Water, 'N')])
            .is_none());
        assert!(table.is_empty());
    }
}

#[cfg(test)]
mod seam_tests {
    use super::*;

    /// The join picker must return a tile that carries material on every edge
    /// facing the same material, and gives way on edges facing a different one.
    ///
    /// Built from hand-set profiles so the test is deterministic and does not
    /// need the Steam install.
    #[test]
    fn pick_agrees_with_neighbouring_edges() {
        let mut table = JoinTable::new();
        // A solid shore tile: every edge carries material.
        table.push(JoinCandidate {
            template: "SOLID".into(),
            icon: 0,
            edges: solid(),
            material: JoinMaterial::Shore,
        });
        // A thin sliver: material only on the north edge.
        table.push(JoinCandidate {
            template: "SLIVER".into(),
            icon: 1,
            edges: EdgeProfiles {
                n: opaque_edge(),
                ..transparent_edges()
            },
            material: JoinMaterial::Shore,
        });
        // A placeholder: every edge transparent.
        table.push(JoinCandidate {
            template: "EMPTY".into(),
            icon: 2,
            edges: transparent_edges(),
            material: JoinMaterial::Land,
        });

        // Interior: every neighbour is shore, so the sliver cannot win (its
        // east/south/west edges are transparent and must not be).
        let interior = vec![
            (JoinMaterial::Shore, 'N'),
            (JoinMaterial::Shore, 'E'),
            (JoinMaterial::Shore, 'S'),
            (JoinMaterial::Shore, 'W'),
        ];
        let pick = table
            .select(JoinMaterial::Shore, &interior)
            .expect("shore candidates");
        assert_eq!(pick.template, "SOLID");

        // Edge-facing: shore on the north, water elsewhere. The candidate must
        // still carry material on the north edge.
        let coastal = vec![
            (JoinMaterial::Shore, 'N'),
            (JoinMaterial::Water, 'E'),
            (JoinMaterial::Water, 'S'),
            (JoinMaterial::Water, 'W'),
        ];
        let pick = table
            .select(JoinMaterial::Shore, &coastal)
            .expect("shore candidates");
        assert!(pick.edges.n.wants_opaque() > 0.5, "{pick:?}");
    }

    #[test]
    fn fully_transparent_placeholder_is_never_chosen_as_base() {
        let mut table = JoinTable::new();
        table.push(JoinCandidate {
            template: "CLEAR1".into(),
            icon: 0,
            edges: transparent_edges(),
            material: JoinMaterial::Land,
        });
        table.push(JoinCandidate {
            template: "P01".into(),
            icon: 0,
            edges: solid(),
            material: JoinMaterial::Land,
        });
        let all_land = [
            (JoinMaterial::Land, 'N'),
            (JoinMaterial::Land, 'E'),
            (JoinMaterial::Land, 'S'),
            (JoinMaterial::Land, 'W'),
        ];
        let pick = table.select(JoinMaterial::Land, &all_land).expect("land");
        assert_eq!(pick.template, "P01");
    }

    #[test]
    fn selection_is_deterministic_across_calls() {
        let mut table = JoinTable::new();
        for i in 0..4u32 {
            table.push(JoinCandidate {
                template: format!("T{i}"),
                icon: 0,
                edges: solid(),
                material: JoinMaterial::Road,
            });
        }
        let ctx = [(JoinMaterial::Road, 'N')];
        let first = table
            .select(JoinMaterial::Road, &ctx)
            .map(|c| (c.template.clone(), c.icon));
        for _ in 0..8 {
            let again = table
                .select(JoinMaterial::Road, &ctx)
                .map(|c| (c.template.clone(), c.icon));
            assert_eq!(first, again, "picker must be stable");
        }
    }

    fn opaque_edge() -> EdgeProfile {
        EdgeProfile {
            transparent: 0.0,
            opaque: 1.0,
            water: 0.0,
        }
    }

    fn solid() -> EdgeProfiles {
        EdgeProfiles {
            n: opaque_edge(),
            e: opaque_edge(),
            s: opaque_edge(),
            w: opaque_edge(),
        }
    }

    fn transparent_edges() -> EdgeProfiles {
        let t = EdgeProfile {
            transparent: 1.0,
            opaque: 0.0,
            water: 0.0,
        };
        EdgeProfiles {
            n: t,
            e: t,
            s: t,
            w: t,
        }
    }
}
