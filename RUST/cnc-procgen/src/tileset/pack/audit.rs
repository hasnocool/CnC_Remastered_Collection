//! Contrast auditing for palette pairs (WCAG relative luminance).
//!
//! Logic-faithful port of `luminance`, `contrast_ratio`, `color_hex`,
//! `CONTRAST_PAIRS` and `contrast_audit` from the Python generator. The
//! report shape matches the pack's `validation_report.json` `contrast` key.

use serde::Serialize;

/// Semantic foreground/background pairs to audit, in pack order.
pub const CONTRAST_PAIRS: [(&str, &str, &str); 12] = [
    ("unit_gdi_body", "gdi", "grass_dark"),
    ("unit_gdi_body_vs_water", "gdi", "water_dark"),
    ("unit_nod_body", "nod", "dirt_dark"),
    ("unit_nod_body_vs_grass", "nod", "grass_dark"),
    ("road_surface_vs_grass", "road", "grass"),
    ("road_marking_vs_road", "white", "road"),
    ("shore_sand_vs_water", "sand", "water"),
    ("shore_foam_vs_water", "white", "water"),
    ("metal_detail_vs_body", "metal", "metal_dark"),
    ("infantry_vs_grass", "infantry", "grass_dark"),
    ("muzzle_flash_vs_unit", "muzzle", "metal_dark"),
    ("tiberium_vs_dirt", "tiberium", "dirt_dark"),
];

/// WCAG relative luminance of an sRGB color.
pub fn luminance(rgb: [u8; 3]) -> f64 {
    fn channel(c: u8) -> f64 {
        let x = f64::from(c) / 255.0;
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    }
    let [r, g, b] = rgb;
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

/// WCAG contrast ratio between two colors (always >= 1.0).
pub fn contrast_ratio(a: [u8; 3], b: [u8; 3]) -> f64 {
    let (hi, lo) = if luminance(a) > luminance(b) {
        (luminance(a), luminance(b))
    } else {
        (luminance(b), luminance(a))
    };
    (hi + 0.05) / (lo + 0.05)
}

/// `#RRGGBB` hex string, matching Python's format.
pub fn color_hex(c: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2])
}

/// One audited pair in the report.
///
/// Key sets match Python exactly per context: audit pairs carry the
/// foreground/background palette names; [`check_contrast_pair`] rows instead
/// carry `threshold` and omit the names; `suggestion` appears only where
/// Python sets it.
#[derive(Serialize)]
pub struct ContrastPair {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    pub foreground_hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    pub background_hex: String,
    pub ratio: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    pub pass: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// The `contrast` section of `validation_report.json`.
#[derive(Serialize)]
pub struct ContrastReport {
    pub threshold: f64,
    pub pairs_checked: usize,
    pub failures: Vec<ContrastPair>,
    pub pairs: Vec<ContrastPair>,
    pub note: String,
}

/// One audited pair with an actionable suggestion (Python's
/// `check_contrast_pair`, kept for API parity — audits and QA scripts can
/// call it directly).
pub fn check_contrast_pair(
    name: &str,
    foreground: [u8; 3],
    background: [u8; 3],
    threshold: f64,
) -> ContrastPair {
    let raw_ratio = contrast_ratio(foreground, background);
    let ratio = (raw_ratio * 1000.0).round() / 1000.0;
    let pass = raw_ratio >= threshold;
    ContrastPair {
        name: name.to_string(),
        foreground: None,
        foreground_hex: color_hex(foreground),
        background: None,
        background_hex: color_hex(background),
        ratio,
        threshold: Some(threshold),
        pass,
        suggestion: if pass {
            None
        } else {
            Some(
                "Use a more distinct foreground/background color or add a contrasting outline."
                    .to_string(),
            )
        },
    }
}

/// Audit every semantic palette pair against `threshold`.
pub fn contrast_audit(threshold: f64) -> ContrastReport {
    let mut pairs = Vec::with_capacity(CONTRAST_PAIRS.len());
    for (name, fg_name, bg_name) in CONTRAST_PAIRS {
        let fg = super::super::palette_rgb(fg_name);
        let bg = super::super::palette_rgb(bg_name);
        let ratio = (contrast_ratio(fg, bg) * 1000.0).round() / 1000.0;
        pairs.push(ContrastPair {
            name: name.to_string(),
            foreground: Some(fg_name.to_string()),
            foreground_hex: color_hex(fg),
            background: Some(bg_name.to_string()),
            background_hex: color_hex(bg),
            ratio,
            threshold: None,
            pass: ratio >= threshold,
            suggestion: None,
        });
    }
    // Python appends a distinct suggestion to each audit failure row.
    let failures = pairs
        .iter()
        .filter(|p| !p.pass)
        .map(|p| ContrastPair {
            name: p.name.clone(),
            foreground: p.foreground.clone(),
            foreground_hex: p.foreground_hex.clone(),
            background: p.background.clone(),
            background_hex: p.background_hex.clone(),
            ratio: p.ratio,
            threshold: p.threshold,
            pass: p.pass,
            suggestion: Some(
                "Choose a lighter foreground or darker background; increase separation; 1.5:1 is the default material-separation floor.".to_string(),
            ),
        })
        .collect();
    ContrastReport {
        threshold,
        pairs_checked: pairs.len(),
        failures,
        pairs,
        note: "The configured threshold is a practical material-separation floor, not a universal WCAG requirement. Check actual in-game scaling and palette.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_endpoints() {
        assert!((luminance([255, 255, 255]) - 1.0).abs() < 1e-9);
        assert!((luminance([0, 0, 0])).abs() < 1e-9);
    }

    #[test]
    fn ratio_is_symmetric_and_min_one() {
        let a = [82, 126, 57];
        let b = [39, 91, 139];
        assert_eq!(contrast_ratio(a, b), contrast_ratio(b, a));
        assert!(contrast_ratio(a, b) >= 1.0);
        assert_eq!(contrast_ratio(a, a), 1.0);
    }

    #[test]
    fn hex_formatting_matches_python() {
        assert_eq!(color_hex([211, 184, 88]), "#D3B858");
        assert_eq!(color_hex([0, 0, 0]), "#000000");
    }

    #[test]
    fn audit_covers_all_pairs_and_passes_at_default() {
        let report = contrast_audit(1.5);
        assert_eq!(report.pairs_checked, 12);
        assert_eq!(report.pairs.len(), 12);
        assert!(report.failures.is_empty(), "unexpected failures at 1.5:1");
        // The pack's own gate: every pair clears the 1.5 floor.
        assert!(report.pairs.iter().all(|p| p.ratio >= 1.5));
    }

    #[test]
    fn impossible_threshold_reports_failures() {
        let report = contrast_audit(99.0);
        assert_eq!(report.failures.len(), 12);
        assert!(report.pairs.iter().all(|p| !p.pass));
    }

    #[test]
    fn contrast_helper_threshold_parity() {
        // Python semantics: pass uses the UNROUNDED ratio, the reported ratio is ROUNDED to 3 decimals.
        // Boundary case: true ratio 1.499889... rounds to 1.5 at 3 decimals.
        // Python: pass = (1.499889... >= 1.5) = false, reported ratio = round(1.499889..., 3) = 1.5
        let pair = check_contrast_pair("test", [0, 0, 0], [0, 3, 157], 1.5);
        // The reported ratio must still be the 3-decimal rounded value (1.5), mirroring Python.
        assert!(
            (pair.ratio - 1.5).abs() < 1e-6,
            "ratio should be 1.5 (rounded), got {}",
            pair.ratio
        );
        // pass must be computed from the UNROUNDED ratio, matching Python.
        assert!(
            (pair.pass) == (1.4998892100478367 >= 1.5),
            "pass should be false (unrounded < threshold), got {}",
            pair.pass
        );
    }
}
