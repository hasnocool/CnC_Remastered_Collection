//! Deterministic software rasterizer for tile generation.
//!
//! A small PIL-flavored drawing surface (points, lines, rectangles, ellipses,
//! polygons, arcs, alpha compositing, masked paste) over `image::RgbaImage`.
//! All coordinates are in pixels; out-of-bounds writes clip silently, matching
//! Pillow behavior. Line width uses a square stamp centered on each Bresenham
//! step (documented deviation from Pillow's exact rasterization; pixel-exact
//! parity with Pillow is out of scope for this logic-faithful port).

use std::path::Path;

use super::Rgba;

/// Mutable RGBA drawing surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Canvas {
    img: image::RgbaImage,
}

impl Canvas {
    /// Solid-filled canvas.
    pub fn new_filled(w: u32, h: u32, rgba: Rgba) -> Self {
        Self {
            img: image::RgbaImage::from_pixel(w, h, image::Rgba(rgba)),
        }
    }

    /// Fully transparent canvas.
    pub fn transparent(w: u32, h: u32) -> Self {
        Self::new_filled(w, h, [0, 0, 0, 0])
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    /// Pixel at `(x, y)`, or `None` when outside the canvas.
    pub fn pixel(&self, x: i32, y: i32) -> Option<Rgba> {
        u32::try_from(x)
            .ok()
            .filter(|&x| x < self.img.width())
            .zip(u32::try_from(y).ok().filter(|&y| y < self.img.height()))
            .map(|(x, y)| self.img.get_pixel(x, y).0)
    }

    /// Set a pixel; writes outside the canvas are ignored.
    pub fn set_pixel(&mut self, x: i32, y: i32, rgba: Rgba) {
        if let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) {
            if x < self.img.width() && y < self.img.height() {
                self.img.put_pixel(x, y, image::Rgba(rgba));
            }
        }
    }

    /// Borrow the underlying image (atlas packing, previews).
    pub fn as_rgba(&self) -> &image::RgbaImage {
        &self.img
    }

    /// Unwrap into the underlying image.
    pub fn into_rgba(self) -> image::RgbaImage {
        self.img
    }

    /// Fill a closed `(x0, y0, x1, y1)` rectangle (inclusive bounds, PIL
    /// style), then stroke the outline at `outline_width` if requested.
    // Signature mirrors the Python generator's draw calls; splitting it would
    // obscure the 1:1 mapping.
    #[allow(clippy::too_many_arguments)]
    pub fn rect(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        fill: Option<Rgba>,
        outline: Option<Rgba>,
        outline_width: u32,
    ) {
        let (xa, xb) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (ya, yb) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        if let Some(c) = fill {
            for y in ya..=yb {
                for x in xa..=xb {
                    self.set_pixel(x, y, c);
                }
            }
        }
        if let Some(c) = outline {
            let w = outline_width.max(1) as i32;
            for i in 0..w {
                self.line(xa + i, ya, xb - i, ya, c, 1);
                self.line(xa + i, yb, xb - i, yb, c, 1);
                self.line(xa, ya + i, xa, yb - i, c, 1);
                self.line(xb, ya + i, xb, yb - i, c, 1);
            }
        }
    }

    /// Stroked line. Width 1 is Bresenham; wider lines stamp a centered
    /// `width × width` square at every step.
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, rgba: Rgba, width: u32) {
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (sign(x1 - x0), sign(y1 - y0));
        let mut err = dx + dy;
        let (mut x, mut y) = (x0, y0);
        loop {
            self.stamp(x, y, rgba, width);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Filled ellipse inscribed in the closed bounding box, PIL style, with
    /// optional outline of `outline_width` strokes inward from the boundary.
    // Signature mirrors the Python generator's draw calls; splitting it would
    // obscure the 1:1 mapping.
    #[allow(clippy::too_many_arguments)]
    pub fn ellipse(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        fill: Option<Rgba>,
        outline: Option<Rgba>,
        outline_width: u32,
    ) {
        let (xa, xb) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (ya, yb) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let w = (xb - xa + 1) as f64;
        let h = (yb - ya + 1) as f64;
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        let cx = (xa + xb) as f64 / 2.0;
        let cy = (ya + yb) as f64 / 2.0;
        let rx = (w - 1.0) / 2.0;
        let ry = (h - 1.0) / 2.0;
        let ow = outline_width.max(1) as f64;
        for y in ya..=yb {
            for x in xa..=xb {
                let nx = (x as f64 - cx) / rx.max(0.5);
                let ny = (y as f64 - cy) / ry.max(0.5);
                let d = nx * nx + ny * ny;
                if d <= 1.0 {
                    if fill.is_some() {
                        self.set_pixel(x, y, fill.unwrap_or([0, 0, 0, 0]));
                    }
                    if outline.is_some() {
                        // Stroke the rim: distance-from-center band near 1.0.
                        let rim = d.sqrt() >= 1.0 - ow / rx.max(ry).max(1.0);
                        if rim {
                            self.set_pixel(x, y, outline.unwrap_or([0, 0, 0, 0]));
                        }
                    }
                }
            }
        }
    }

    /// Even-odd scanline-filled polygon with optional edge stroke.
    pub fn polygon(&mut self, points: &[(i32, i32)], fill: Option<Rgba>, outline: Option<Rgba>) {
        if let Some(c) = fill {
            let min_y = points.iter().map(|p| p.1).min().unwrap_or(0);
            let max_y = points.iter().map(|p| p.1).max().unwrap_or(0);
            for y in min_y..=max_y {
                // Scanline intersections at the row center.
                let yc = y as f64 + 0.5;
                let mut xs: Vec<f64> = Vec::new();
                for i in 0..points.len() {
                    let (x0, y0) = points[i];
                    let (x1, y1) = points[(i + 1) % points.len()];
                    let (y0f, y1f) = (y0 as f64, y1 as f64);
                    if (y0f <= yc && y1f > yc) || (y1f <= yc && y0f > yc) {
                        let t = (yc - y0f) / (y1f - y0f);
                        xs.push(x0 as f64 + t * (x1 - x0) as f64);
                    }
                }
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                for pair in xs.chunks(2) {
                    if let [a, b] = pair {
                        for x in (a.ceil() as i32)..=(b.floor() as i32) {
                            self.set_pixel(x, y, c);
                        }
                    }
                }
            }
        }
        if let Some(c) = outline {
            for i in 0..points.len() {
                let (x0, y0) = points[i];
                let (x1, y1) = points[(i + 1) % points.len()];
                self.line(x0, y0, x1, y1, c, 1);
            }
        }
    }

    /// Arc of an ellipse inscribed in the closed bounding box, from
    /// `start_deg` to `end_deg`. Angles follow Pillow: 0° is 3 o'clock and
    /// increasing degrees sweep clockwise on screen (y grows downward), so
    /// 90° is the bottom of the ellipse.
    // Signature mirrors the Python generator's draw calls; splitting it would
    // obscure the 1:1 mapping.
    #[allow(clippy::too_many_arguments)]
    pub fn arc(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        start_deg: f64,
        end_deg: f64,
        rgba: Rgba,
        width: u32,
    ) {
        let (xa, xb) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (ya, yb) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let cx = (xa + xb) as f64 / 2.0;
        let cy = (ya + yb) as f64 / 2.0;
        let rx = (xb - xa) as f64 / 2.0;
        let ry = (yb - ya) as f64 / 2.0;
        let mut end = end_deg;
        while end < start_deg {
            end += 360.0;
        }
        // Sample densely enough that the chord step stays under a pixel.
        let r = rx.max(ry).max(1.0);
        let steps = (((end - start_deg) * std::f64::consts::PI / 180.0 * r).ceil() as i32).max(1);
        for i in 0..=steps {
            let t = start_deg + (end - start_deg) * i as f64 / steps as f64;
            let rad = t.to_radians();
            let x = (cx + rx * rad.cos()).round() as i32;
            // Pillow convention: positive sin moves down the screen.
            let y = (cy + ry * rad.sin()).round() as i32;
            self.stamp(x, y, rgba, width);
        }
    }

    /// Source-over composite `src` at `(dx, dy)`.
    pub fn alpha_composite(&mut self, src: &Canvas, at: (i32, i32)) {
        let (dx, dy) = at;
        for y in 0..src.height() as i32 {
            for x in 0..src.width() as i32 {
                let Some(sc) = src.pixel(x, y) else {
                    continue;
                };
                let sa = sc[3] as u32;
                if sa == 0 {
                    continue;
                }
                let (tx, ty) = (dx + x, dy + y);
                let dc = self.pixel(tx, ty).unwrap_or([0, 0, 0, 0]);
                let da = dc[3] as u32;
                let out_a = sa + da * (255 - sa) / 255;
                let mut out = [0u8; 4];
                for i in 0..3 {
                    let top = sc[i] as u32 * sa + dc[i] as u32 * da * (255 - sa) / 255;
                    out[i] = top.checked_div(out_a).unwrap_or(0) as u8;
                }
                out[3] = out_a as u8;
                self.set_pixel(tx, ty, out);
            }
        }
    }

    /// Copy pixels of `src` where `mask` is 255 (binary mask, PIL paste).
    pub fn masked_paste(&mut self, src: &Canvas, mask: &Mask, at: (i32, i32)) {
        let (dx, dy) = at;
        for y in 0..src.height() as i32 {
            for x in 0..src.width() as i32 {
                if mask.get(x, y) == Some(255) {
                    if let Some(c) = src.pixel(x, y) {
                        self.set_pixel(dx + x, dy + y, c);
                    }
                }
            }
        }
    }

    /// Filled rounded rectangle over the closed bbox, PIL
    /// `rounded_rectangle(fill=...)` style: corner quarter-circles of
    /// `radius` (clamped to half the smaller side), straight edges between
    /// them. Radius `0` or a degenerate side falls back to the plain
    /// [`Canvas::rect`] fill.
    pub fn rounded_rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, radius: i32, color: Rgba) {
        let (xa, xb) = sort_axis(x0, x1);
        let (ya, yb) = sort_axis(y0, y1);
        let r = rounded_clamp_radius(xa, ya, xb, yb, radius);
        for y in ya..=yb {
            for x in xa..=xb {
                if rounded_contains(xa, ya, xb, yb, r, x, y) {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    /// Outline form of [`Canvas::rounded_rect`]: strokes the shape boundary
    /// `width` pixels inward (outer edge is the rounded-rect path). When the
    /// stroke thickness consumes the whole shape it fills it, matching what
    /// PIL's outline rendering degrades to for tiny boxes.
    #[allow(clippy::too_many_arguments)] // mirrors PIL's rounded_rectangle signature
    pub fn rounded_rect_outline(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        radius: i32,
        color: Rgba,
        width: u32,
    ) {
        let (xa, xb) = sort_axis(x0, x1);
        let (ya, yb) = sort_axis(y0, y1);
        let r = rounded_clamp_radius(xa, ya, xb, yb, radius);
        if r == 0 {
            self.rect(xa, ya, xb, yb, None, Some(color), width);
            return;
        }
        let w = width.max(1) as i32;
        let (ia0, ia1) = (xa + w, xb - w);
        let (ja0, ja1) = (ya + w, yb - w);
        let inner_valid = ia0 <= ia1 && ja0 <= ja1;
        let ir = rounded_clamp_radius(ia0, ja0, ia1, ja1, r - w);
        for y in ya..=yb {
            for x in xa..=xb {
                if !rounded_contains(xa, ya, xb, yb, r, x, y) {
                    continue;
                }
                let on_inner = inner_valid && rounded_contains(ia0, ja0, ia1, ja1, ir, x, y);
                if !on_inner {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    /// Filled ellipse pieslice over the closed bbox, from `start_deg` to
    /// `end_deg`. Angles follow the same Pillow convention as
    /// [`Canvas::arc`]: 0° is 3 o'clock and degrees sweep clockwise on
    /// screen (y grows downward), so 90° is the bottom of the bbox. A pixel
    /// is drawn when it lies inside the inscribed ellipse and its angle
    /// falls in the clockwise span from `start` to `end` (which wraps past
    /// 360 exactly like [`Canvas::arc`] handles `start > end`); the center
    /// pixel is always included.
    #[allow(clippy::too_many_arguments)] // mirrors PIL's pieslice signature
    pub fn pieslice(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        start_deg: f64,
        end_deg: f64,
        color: Rgba,
    ) {
        let (xa, xb) = sort_axis(x0, x1);
        let (ya, yb) = sort_axis(y0, y1);
        let cx = (xa + xb) as f64 / 2.0;
        let cy = (ya + yb) as f64 / 2.0;
        let rx = ((xb - xa) as f64 / 2.0).max(1e-9);
        let ry = ((yb - ya) as f64 / 2.0).max(1e-9);
        // Mirror arc()'s start>end handling exactly.
        let mut end = end_deg;
        while end < start_deg {
            end += 360.0;
        }
        let span = end - start_deg;
        for y in ya..=yb {
            for x in xa..=xb {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                let nx = dx / rx;
                let ny = dy / ry;
                if nx * nx + ny * ny > 1.0 {
                    continue;
                }
                if dx == 0.0 && dy == 0.0 {
                    self.set_pixel(x, y, color);
                    continue;
                }
                let angle = dy.atan2(dx).to_degrees();
                let rel = (angle - start_deg).rem_euclid(360.0);
                if rel <= span {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    /// Extract the sub-image at `(x, y)` of size `(w, h)`. Source pixels
    /// outside this canvas become transparent (no panic), mirroring PIL
    /// `crop` semantics for out-of-range regions in spirit.
    pub fn crop(&self, x: i32, y: i32, w: u32, h: u32) -> Self {
        let mut img = image::RgbaImage::new(w, h);
        for dy in 0..h {
            for dx in 0..w {
                if let Some(p) = self.pixel(x + dx as i32, y + dy as i32) {
                    img.put_pixel(dx, dy, image::Rgba(p));
                }
            }
        }
        Self { img }
    }

    /// Nearest-neighbour resize: the destination pixel maps to source pixel
    /// `floor(d * src_size / dst_size)` on each axis. Deterministic.
    pub fn resize_nearest(&self, w: u32, h: u32) -> Self {
        let (sw, sh) = (self.img.width(), self.img.height());
        let mut img = image::RgbaImage::new(w, h);
        if sw == 0 || sh == 0 {
            return Self { img };
        }
        for dy in 0..h {
            let sy = (dy as u64 * sh as u64 / h.max(1) as u64) as u32;
            for dx in 0..w {
                let sx = (dx as u64 * sw as u64 / w.max(1) as u64) as u32;
                img.put_pixel(dx, dy, *self.img.get_pixel(sx.min(sw - 1), sy.min(sh - 1)));
            }
        }
        Self { img }
    }

    /// PIL `BILINEAR` resize: half-pixel sample centers on both axes with
    /// clamped edge lookups, computed on premultiplied-alpha channels so
    /// partially transparent pixels cannot bleed color into the result.
    /// (Single-sample kernel, i.e. PIL's behavior for enlargements.)
    pub fn resize_bilinear(&self, w: u32, h: u32) -> Self {
        let (sw, sh) = (self.img.width(), self.img.height());
        let mut img = image::RgbaImage::new(w, h);
        if sw == 0 || sh == 0 || w == 0 || h == 0 {
            return Self { img };
        }
        let scale_x = sw as f64 / w as f64;
        let scale_y = sh as f64 / h as f64;
        for dy in 0..h {
            let sy = (dy as f64 + 0.5) * scale_y - 0.5;
            let iy0 = sy.floor() as i64;
            let fy = sy - iy0 as f64;
            let iy0c = iy0.clamp(0, sh as i64 - 1);
            let iy1c = (iy0 + 1).clamp(0, sh as i64 - 1);
            for dx in 0..w {
                let sx = (dx as f64 + 0.5) * scale_x - 0.5;
                let ix0 = sx.floor() as i64;
                let fx = sx - ix0 as f64;
                let ix0c = ix0.clamp(0, sw as i64 - 1);
                let ix1c = (ix0 + 1).clamp(0, sw as i64 - 1);
                let p00 = to_premult(self.img.get_pixel(ix0c as u32, iy0c as u32).0);
                let p10 = to_premult(self.img.get_pixel(ix1c as u32, iy0c as u32).0);
                let p01 = to_premult(self.img.get_pixel(ix0c as u32, iy1c as u32).0);
                let p11 = to_premult(self.img.get_pixel(ix1c as u32, iy1c as u32).0);
                let mut out = [0.0f64; 4];
                for i in 0..4 {
                    out[i] = p00[i] * (1.0 - fy) * (1.0 - fx)
                        + p10[i] * (1.0 - fy) * fx
                        + p01[i] * fy * (1.0 - fx)
                        + p11[i] * fy * fx;
                }
                img.put_pixel(dx, dy, image::Rgba(from_premult(out)));
            }
        }
        Self { img }
    }

    /// PIL `BICUBIC` resize: Catmull-Rom kernel (a = -0.5, support 2) with
    /// half-pixel centers, clamped edges, normalized taps, computed on
    /// premultiplied-alpha channels. Deterministic.
    pub fn resize_catmullrom(&self, w: u32, h: u32) -> Self {
        let (sw, sh) = (self.img.width(), self.img.height());
        let mut img = image::RgbaImage::new(w, h);
        if sw == 0 || sh == 0 || w == 0 || h == 0 {
            return Self { img };
        }
        let scale_x = sw as f64 / w as f64;
        let scale_y = sh as f64 / h as f64;
        for dy in 0..h {
            let sy = (dy as f64 + 0.5) * scale_y - 0.5;
            let wy = kernel_window(sy);
            for dx in 0..w {
                let sx = (dx as f64 + 0.5) * scale_x - 0.5;
                let wx = kernel_window(sx);
                let mut out = [0.0f64; 4];
                for (j, wj) in &wy {
                    for (i, wi) in &wx {
                        let sxu = (*i).clamp(0, sw as i64 - 1) as u32;
                        let syu = (*j).clamp(0, sh as i64 - 1) as u32;
                        let p = to_premult(self.img.get_pixel(sxu, syu).0);
                        let wgt = wi * wj;
                        for c in 0..4 {
                            out[c] += p[c] * wgt;
                        }
                    }
                }
                img.put_pixel(dx, dy, image::Rgba(from_premult(out)));
            }
        }
        Self { img }
    }

    /// Vertical opaque linear gradient, port of the Python generator's
    /// `gradient()` in the v6.2/6.1 asset-expansion scripts: for each row
    /// `t = y / max(1, h - 1)` and per channel
    /// `int(top[i] * (1 - t) + bottom[i] * t)` (Python `int` truncates).
    pub fn gradient(w: u32, h: u32, top: [u8; 3], bottom: [u8; 3]) -> Self {
        let mut img = image::RgbaImage::new(w, h);
        let denom = h.saturating_sub(1).max(1) as f64;
        for y in 0..h {
            let t = y as f64 / denom;
            let mut col = [0u8, 0, 0, 255];
            for i in 0..3 {
                col[i] = (top[i] as f64 * (1.0 - t) + bottom[i] as f64 * t) as u8;
            }
            for x in 0..w {
                img.put_pixel(x, y, image::Rgba(col));
            }
        }
        Self { img }
    }

    /// Encode as PNG: 8-bit RGB (IHDR color type 2) when every pixel is
    /// fully opaque, 8-bit RGBA (color type 6) otherwise. The pack builder's
    /// `pixel_sha256` hashes the in-memory RGBA buffer, so pixel hashes are
    /// unaffected; only the encoded file bytes change for fully-opaque
    /// canvases (matching how the Python scripts save opaque "RGB" images).
    pub fn encode_png(&self) -> std::io::Result<Vec<u8>> {
        let all_opaque = self.img.pixels().all(|p| p.0[3] == 255);
        let mut buf = Vec::new();
        if all_opaque {
            let rgb = image::RgbImage::from_fn(self.img.width(), self.img.height(), |x, y| {
                let p = self.img.get_pixel(x, y);
                image::Rgb([p[0], p[1], p[2]])
            });
            rgb.write_with_encoder(png_encoder(&mut buf))
                .map_err(encode_error_into_io)?;
        } else {
            self.img
                .write_with_encoder(png_encoder(&mut buf))
                .map_err(encode_error_into_io)?;
        }
        Ok(buf)
    }

    /// Save the canvas at `path` using [`Canvas::encode_png`]'s RGB/RGBA
    /// channel selection (parent directories are created).
    pub fn save_png(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            if !dir.as_os_str().is_empty() {
                std::fs::create_dir_all(dir)?;
            }
        }
        std::fs::write(path, self.encode_png()?)
    }

    fn stamp(&mut self, x: i32, y: i32, rgba: Rgba, width: u32) {
        if width <= 1 {
            self.set_pixel(x, y, rgba);
            return;
        }
        let half = width as i32 / 2;
        for oy in -half..=half {
            for ox in -half..=half {
                self.set_pixel(x + ox, y + oy, rgba);
            }
        }
    }
}

/// 8-bit mask for [`Canvas::masked_paste`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mask {
    w: u32,
    h: u32,
    values: Vec<u8>,
}

impl Mask {
    pub fn new(w: u32, h: u32, fill: u8) -> Self {
        Self {
            w,
            h,
            values: vec![fill; (w * h) as usize],
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: u8) {
        if let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) {
            if x < self.w && y < self.h {
                self.values[(y * self.w + x) as usize] = value;
            }
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        u32::try_from(x)
            .ok()
            .filter(|&x| x < self.w)
            .zip(u32::try_from(y).ok().filter(|&y| y < self.h))
            .map(|(x, y)| self.values[(y * self.w + x) as usize])
    }

    /// Borrow raw row-major values.
    pub fn into_raw(self) -> Vec<u8> {
        self.values
    }
}

fn sign(v: i32) -> i32 {
    if v > 0 {
        1
    } else if v < 0 {
        -1
    } else {
        0
    }
}

/// Sort two coordinates onto an ascending axis.
fn sort_axis(a: i32, b: i32) -> (i32, i32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Radius clamped to half the smaller side of the closed bbox, Pillow
/// `rounded_rectangle` style (a side is measured as `x1 - x0`). Radius 0 or
/// a degenerate side yields 0 (plain rect fallback in the callers).
fn rounded_clamp_radius(xa: i32, ya: i32, xb: i32, yb: i32, radius: i32) -> i32 {
    let side = (xb - xa).min(yb - ya);
    radius.clamp(0, side.max(0) / 2)
}

/// Inside-test of the filled rounded-rect shape over the closed bbox with
/// radius `r` (pre-clamped): straight edge bands joined by corner
/// quarter-circles; radius 0 degenerates to the plain closed rectangle.
fn rounded_contains(xa: i32, ya: i32, xb: i32, yb: i32, r: i32, x: i32, y: i32) -> bool {
    debug_assert!(xa <= xb && ya <= yb);
    if r == 0 {
        return xa <= x && x <= xb && ya <= y && y <= yb;
    }
    // Points outside the bbox are never part of the shape; without this the
    // band tests below would match an unbounded strip through the bbox and
    // the outline's inner-shape test would swallow pixels outside it.
    if x < xa || x > xb || y < ya || y > yb {
        return false;
    }
    let r2 = (r as i64) * (r as i64);
    let in_v_band = y >= ya + r && y <= yb - r;
    let in_h_band = x >= xa + r && x <= xb - r;
    if in_v_band || in_h_band {
        return true;
    }
    // Corner region: nearest corner-center is the quarter-circle center.
    let qx = if x < xa + r { xa + r } else { xb - r };
    let qy = if y < ya + r { ya + r } else { yb - r };
    let dx = (x - qx) as i64;
    let dy = (y - qy) as i64;
    dx * dx + dy * dy <= r2
}

/// Catmull-Rom / PIL BICUBIC kernel (a = -0.5, support 2).
fn catmullrom_kernel(x: f64) -> f64 {
    let a = -0.5;
    let ax = x.abs();
    if ax <= 1.0 {
        let x2 = ax * ax;
        let x3 = x2 * ax;
        (a + 2.0) * x3 - (a + 3.0) * x2 + 1.0
    } else if ax < 2.0 {
        let x2 = ax * ax;
        let x3 = x2 * ax;
        a * x3 - 5.0 * a * x2 + 8.0 * a * ax - 4.0 * a
    } else {
        0.0
    }
}

/// Sampling taps and normalized weights for a windowed convolution at
/// `center` (half-pixel-centered resampling coordinate).
fn kernel_window(center: f64) -> Vec<(i64, f64)> {
    let base = center.floor() as i64;
    let taps: Vec<(i64, f64)> = [base - 1, base, base + 1, base + 2]
        .into_iter()
        .map(|i| (i, catmullrom_kernel(center - i as f64)))
        .collect();
    let sum: f64 = taps.iter().map(|(_, w)| *w).sum();
    let norm = if sum.abs() <= 1e-12 { 1.0 } else { sum };
    taps.into_iter().map(|(i, w)| (i, w / norm)).collect()
}

/// Straight RGBA -> premultiplied f64 channels.
fn to_premult(p: Rgba) -> [f64; 4] {
    let a = p[3] as f64;
    [
        p[0] as f64 * a / 255.0,
        p[1] as f64 * a / 255.0,
        p[2] as f64 * a / 255.0,
        a,
    ]
}

/// Premultiplied f64 channels -> straight RGBA (clamped and rounded).
fn from_premult(p: [f64; 4]) -> Rgba {
    let quantize = |v: f64| -> u8 { v.round().clamp(0.0, 255.0) as u8 };
    let a = p[3].clamp(0.0, 255.0);
    let inv = if a > 0.0 { 255.0 / a } else { 0.0 };
    [
        quantize(p[0] * inv),
        quantize(p[1] * inv),
        quantize(p[2] * inv),
        quantize(a),
    ]
}

/// PIL quality-matched PNG encoder over an in-memory buffer.
fn png_encoder(buf: &mut Vec<u8>) -> image::codecs::png::PngEncoder<std::io::Cursor<&mut Vec<u8>>> {
    use image::codecs::png::{CompressionType, FilterType, PngEncoder};
    PngEncoder::new_with_quality(
        std::io::Cursor::new(buf),
        CompressionType::Best,
        FilterType::Adaptive,
    )
}

/// Map [`image::ImageError`] onto [`std::io::Error`], mirroring the pack
/// builder's save path.
fn encode_error_into_io(e: image::ImageError) -> std::io::Error {
    match e {
        image::ImageError::IoError(io) => io,
        other => std::io::Error::other(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: Rgba = [255, 0, 0, 255];
    const BLUE: Rgba = [0, 0, 255, 255];

    #[test]
    fn line_writes_endpoints() {
        let mut c = Canvas::transparent(32, 32);
        c.line(2, 2, 29, 29, RED, 1);
        assert_eq!(c.pixel(2, 2), Some(RED));
        assert_eq!(c.pixel(29, 29), Some(RED));
        assert_eq!(c.pixel(16, 16), Some(RED));
    }

    #[test]
    fn rect_inclusive_bounds_and_outline() {
        let mut c = Canvas::transparent(32, 32);
        c.rect(4, 4, 10, 10, Some(RED), Some(BLUE), 1);
        // Outline is stroked after the fill, so the border ring (corners
        // included) ends up blue; only the interior stays red.
        assert_eq!(c.pixel(4, 4), Some(BLUE)); // inclusive corner
        assert_eq!(c.pixel(10, 10), Some(BLUE));
        assert_eq!(c.pixel(5, 5), Some(RED)); // interior
        assert_eq!(c.pixel(4, 7), Some(BLUE)); // left border
        assert_eq!(c.pixel(7, 10), Some(BLUE)); // bottom border
                                                // Bounds are inclusive: one step outside is untouched.
        assert_eq!(c.pixel(3, 4), Some([0, 0, 0, 0]));
        assert_eq!(c.pixel(11, 10), Some([0, 0, 0, 0]));
    }

    #[test]
    fn ellipse_fills_center_not_bbox_corners() {
        let mut c = Canvas::transparent(32, 32);
        c.ellipse(6, 6, 25, 25, Some(RED), None, 1);
        assert_eq!(c.pixel(16, 16), Some(RED)); // center
        assert_eq!(c.pixel(6, 6), Some([0, 0, 0, 0])); // bbox corner is outside the circle
        assert_eq!(c.pixel(25, 25), Some([0, 0, 0, 0]));
        assert_eq!(c.pixel(21, 16), Some(RED)); // right edge midpoint
    }

    #[test]
    fn polygon_fills_triangle() {
        let mut c = Canvas::transparent(32, 32);
        c.polygon(&[(4, 4), (27, 4), (16, 27)], Some(RED), None);
        assert_eq!(c.pixel(16, 6), Some(RED)); // near top edge, inside
        assert_eq!(c.pixel(16, 20), Some(RED)); // lower center, inside
        assert_eq!(c.pixel(4, 27), Some([0, 0, 0, 0])); // outside the slanted edges
        assert_eq!(c.pixel(27, 27), Some([0, 0, 0, 0]));
    }

    #[test]
    fn arc_marks_expected_quadrant() {
        let mut c = Canvas::transparent(32, 32);
        // Quarter arc from 0° (3 o'clock) to 90° (12 o'clock): upper right.
        c.arc(2, 2, 29, 29, 0.0, 90.0, RED, 1);
        // Pillow arc convention (y grows down): 0° is 3 o'clock, 90° the
        // bottom. Bbox center (15.5, 15.5), radius 13.5.
        assert_eq!(c.pixel(29, 16), Some(RED)); // start: right middle
        assert_eq!(c.pixel(16, 29), Some(RED)); // end: bottom middle
        assert_eq!(c.pixel(16, 2), Some([0, 0, 0, 0])); // top middle untouched
        assert_eq!(c.pixel(2, 16), Some([0, 0, 0, 0])); // 180° point untouched
    }

    #[test]
    fn alpha_composite_over_transparent_keeps_source() {
        let mut dst = Canvas::transparent(32, 32);
        let mut src = Canvas::transparent(8, 8);
        src.rect(0, 0, 7, 7, Some(RED), None, 0);
        dst.alpha_composite(&src, (4, 4));
        assert_eq!(dst.pixel(4, 4), Some(RED));
        assert_eq!(dst.pixel(3, 3), Some([0, 0, 0, 0]));
        assert_eq!(dst.pixel(11, 11), Some(RED));
    }

    #[test]
    fn masked_paste_honors_mask() {
        let mut dst = Canvas::transparent(32, 32);
        let mut src = Canvas::new_filled(8, 8, RED);
        src.set_pixel(0, 0, BLUE);
        let mut mask = Mask::new(8, 8, 255);
        mask.set(0, 0, 0); // exclude the blue pixel
        dst.masked_paste(&src, &mask, (0, 0));
        assert_eq!(dst.pixel(0, 0), Some([0, 0, 0, 0])); // masked out
        assert_eq!(dst.pixel(1, 1), Some(RED)); // pasted
    }

    #[test]
    fn clipped_writes_are_silent() {
        let mut c = Canvas::new_filled(8, 8, [1, 2, 3, 255]);
        c.set_pixel(-1, 0, RED);
        c.set_pixel(0, 8, RED);
        assert_eq!(c.pixel(-1, 0), None);
        assert_eq!(c.pixel(0, 8), None);
        assert_eq!(c.pixel(0, 0), Some([1, 2, 3, 255]));
    }

    #[test]
    fn rounded_rect_fills_interior_and_clips_corners() {
        let mut c = Canvas::transparent(32, 32);
        // 11x11 box (0,0)-(10,10), radius 4.
        c.rounded_rect(0, 0, 10, 10, 4, RED);
        assert_eq!(c.pixel(5, 5), Some(RED)); // interior
        assert_eq!(c.pixel(4, 0), Some(RED)); // top edge, at the band boundary
        assert_eq!(c.pixel(0, 4), Some(RED)); // left edge
        assert_eq!(c.pixel(0, 0), Some([0, 0, 0, 0])); // outside the corner arc
        assert_eq!(c.pixel(10, 10), Some([0, 0, 0, 0])); // opposite corner too
        assert_eq!(c.pixel(2, 2), Some(RED)); // inside the TL quarter-circle
        assert_eq!(c.pixel(8, 9), Some(RED)); // BR quarter-circle interior
    }

    #[test]
    fn rounded_rect_radius_zero_matches_rect() {
        let mut a = Canvas::transparent(16, 16);
        a.rounded_rect(2, 2, 9, 9, 0, RED);
        let mut b = Canvas::transparent(16, 16);
        b.rect(2, 2, 9, 9, Some(RED), None, 0);
        assert_eq!(a.as_rgba(), b.as_rgba());
        // An oversized radius clamps to half the smaller side and never
        // escapes the bbox; a 19x8 pill keeps every pixel inside.
        let mut c = Canvas::transparent(24, 24);
        c.rounded_rect(2, 2, 20, 9, 999, RED);
        for y in 2..=9 {
            for x in 2..=20 {
                assert!(c.pixel(x, y).is_some());
            }
        }
        assert_eq!(c.pixel(21, 5), Some([0, 0, 0, 0]));
        assert_eq!(c.pixel(2, 1), Some([0, 0, 0, 0]));
    }

    #[test]
    fn rounded_rect_outline_strokes_inward() {
        let mut c = Canvas::transparent(32, 32);
        c.rounded_rect_outline(0, 0, 10, 10, 4, RED, 1);
        assert_eq!(c.pixel(2, 1), Some(RED)); // on the stroke, inside the TL arc
        assert_eq!(c.pixel(0, 0), Some([0, 0, 0, 0])); // outside the arc
        assert_eq!(c.pixel(5, 5), Some([0, 0, 0, 0])); // interior unfilled
        assert_eq!(c.pixel(4, 0), Some(RED)); // top band boundary
                                              // Huge width consumes the shape: full fill.
        let mut d = Canvas::transparent(32, 32);
        d.rounded_rect_outline(0, 0, 10, 10, 4, RED, 100);
        assert_eq!(d.pixel(5, 5), Some(RED));
        assert_eq!(d.pixel(0, 0), Some([0, 0, 0, 0]));
    }

    #[test]
    fn pieslice_marks_quadrants_like_arc_convention() {
        let mut c = Canvas::transparent(32, 32);
        // 0°..90°: 3 o'clock to the bottom of the screen (clockwise sweep).
        // Odd span so the bbox centre (15, 15) is a real pixel.
        c.pieslice(2, 2, 28, 28, 0.0, 90.0, RED);
        assert_eq!(c.pixel(15, 15), Some(RED)); // centre is always included
        assert_eq!(c.pixel(28, 15), Some(RED)); // right middle (angle 0°)
        assert_eq!(c.pixel(15, 28), Some(RED)); // bottom middle (angle 90°)
        assert_eq!(c.pixel(15, 2), Some([0, 0, 0, 0])); // top middle (270°)
        assert_eq!(c.pixel(3, 15), Some([0, 0, 0, 0])); // left middle (180°)
        assert_eq!(c.pixel(28, 28), Some([0, 0, 0, 0])); // bbox corner outside the ellipse
                                                         // Bottom-left quadrant: 90°..180°.
        let mut d = Canvas::transparent(32, 32);
        d.pieslice(2, 2, 28, 28, 90.0, 180.0, BLUE);
        assert_eq!(d.pixel(3, 15), Some(BLUE)); // left middle (180°)
        assert_eq!(d.pixel(15, 28), Some(BLUE)); // bottom middle is the inclusive start
        assert_eq!(d.pixel(28, 15), Some([0, 0, 0, 0]));
        assert_eq!(d.pixel(15, 2), Some([0, 0, 0, 0]));
    }

    #[test]
    fn pieslice_wraps_when_start_exceeds_end() {
        let mut c = Canvas::transparent(32, 32);
        // 270°..0° wraps to 270°..360°: the upper-right quadrant.
        c.pieslice(2, 2, 28, 28, 270.0, 0.0, RED);
        assert_eq!(c.pixel(15, 15), Some(RED)); // centre
        assert_eq!(c.pixel(15, 2), Some(RED)); // top middle (270°)
        assert_eq!(c.pixel(25, 7), Some(RED)); // upper-right interior (~321°)
        assert_eq!(c.pixel(27, 16), Some([0, 0, 0, 0])); // just past the 0° end
        assert_eq!(c.pixel(3, 15), Some([0, 0, 0, 0])); // left middle (180°)
        assert_eq!(c.pixel(7, 7), Some([0, 0, 0, 0])); // inside the ellipse, outside the span
    }

    #[test]
    fn crop_extracts_and_pads_out_of_bounds_transparent() {
        let mut c = Canvas::transparent(8, 8);
        c.rect(2, 2, 6, 6, Some(RED), None, 0);
        let sub = c.crop(2, 2, 3, 3);
        assert_eq!(sub.width(), 3);
        assert_eq!(sub.height(), 3);
        assert_eq!(sub.pixel(0, 0), Some(RED)); // (2,2) is inside the filled rect
        assert_eq!(sub.pixel(1, 1), Some(RED));
        // Out-of-bounds source pixels are transparent, not a panic.
        let wide = c.crop(6, 6, 6, 6);
        assert_eq!(wide.pixel(0, 0), Some(RED)); // (6,6) inside the rect
        assert_eq!(wide.pixel(5, 5), Some([0, 0, 0, 0])); // (11,11) off-canvas
        assert_eq!(wide.width(), 6);
        let far = c.crop(-2, -2, 5, 5);
        assert_eq!(far.pixel(0, 0), Some([0, 0, 0, 0])); // (-2,-2) is OOB
        assert_eq!(far.pixel(2, 2), Some([0, 0, 0, 0])); // (0,0) on-canvas, empty
        assert_eq!(far.pixel(4, 4), Some(RED)); // (2,2) inside the rect
    }

    #[test]
    fn resize_nearest_duplicates_pixels_at_2x() {
        let mut c = Canvas::transparent(2, 2);
        c.rect(0, 0, 0, 1, Some(RED), None, 0); // column 0
        c.rect(1, 0, 1, 1, Some(BLUE), None, 0); // column 1
        let big = c.resize_nearest(4, 4);
        // dst_x * src_w / dst_w = dx * 2 / 4 → columns 0,0,1,1.
        for (x, expected) in [(0, RED), (1, RED), (2, BLUE), (3, BLUE)] {
            assert_eq!(big.pixel(x, 0), Some(expected));
            assert_eq!(big.pixel(x, 1), Some(expected));
            assert_eq!(big.pixel(x, 2), Some(expected));
            assert_eq!(big.pixel(x, 3), Some(expected));
        }
    }

    #[test]
    fn resize_bilinear_matches_hand_computed_centers() {
        // 2x2 opaque with corner values a/b/c/d.
        let mut c = Canvas::transparent(2, 2);
        let a: Rgba = [10, 20, 30, 255];
        let b: Rgba = [50, 60, 70, 255];
        let d: Rgba = [90, 100, 110, 255];
        let e: Rgba = [130, 140, 150, 255];
        c.set_pixel(0, 0, a);
        c.set_pixel(1, 0, b);
        c.set_pixel(0, 1, d);
        c.set_pixel(1, 1, e);
        let big = c.resize_bilinear(4, 4);
        // scale = 0.5; center of dst pixel dx is (dx+.5)*0.5 - .5:
        // -0.25, 0.25, 0.75, 1.25 → clamps on the edges, 25%/75% mixes inside.
        assert_eq!(big.pixel(0, 0), Some(a)); // clamped corner
        assert_eq!(big.pixel(3, 0), Some(b)); // clamped corner
        assert_eq!(big.pixel(3, 3), Some(e));
        // (1,1): row mix a..b then x mix 75/25, y mix 75/25 against d..e.
        // x-mix at .25: [20,30,40]; bottom x-mix: 100,110,120; y 75/25:
        // [40,50,60].
        assert_eq!(big.pixel(1, 1), Some([40, 50, 60, 255]));
        assert_eq!(big.pixel(1, 0), Some([20, 30, 40, 255]));
        // (2,2): fx = fy = .75, so the exact 2D mix is
        // r = 10*.0625 + 50*.1875 + 90*.1875 + 130*.5625 = 100,
        // g = 20*.0625 + 60*.1875 + 100*.1875 + 140*.5625 = 110,
        // b = 30*.0625 + 70*.1875 + 110*.1875 + 150*.5625 = 120.
        assert_eq!(big.pixel(2, 2), Some([100, 110, 120, 255]));
    }

    #[test]
    fn resize_catmullrom_stays_in_range_and_deterministic() {
        let src = Canvas::gradient(4, 4, [10, 20, 30], [200, 100, 60]);
        let a = src.resize_catmullrom(7, 7);
        let b = src.resize_catmullrom(7, 7);
        assert_eq!(a.as_rgba(), b.as_rgba()); // deterministic
        assert_eq!(a.width(), 7);
        for y in 0..7 {
            for x in 0..7 {
                let p = a.pixel(x, y).expect("in bounds");
                assert_eq!(p[3], 255); // opaque in, opaque out
            }
        }
        // Interpolation moved the vertical gradient, not jumped past its ends.
        assert!(a.pixel(0, 0).unwrap()[0] <= 200);
        assert!(a.pixel(0, 6).unwrap()[0] >= 10);
    }

    #[test]
    fn encode_png_selects_rgb_channel_type_for_opaque() {
        let opaque = Canvas::gradient(8, 8, [1, 2, 3], [4, 5, 6]);
        let bytes = opaque.encode_png().expect("encode");
        // PNG signature (8) + length (4) + "IHDR" (4) + width (4) + height (4)
        // puts bit depth at byte 24 and colour type at byte 25: type 2 (RGB)
        // because every pixel has alpha 255.
        assert_eq!(bytes[24], 8);
        assert_eq!(bytes[25], 2);
        // One transparent pixel flips the choice to color type 6 (RGBA).
        let mut either = opaque;
        either.set_pixel(0, 0, [1, 2, 3, 254]);
        let either_bytes = either.encode_png().expect("encode");
        assert_eq!(either_bytes[25], 6);
    }

    #[test]
    fn gradient_rows_match_python_math() {
        let g = Canvas::gradient(4, 3, [0, 0, 0], [90, 60, 30]);
        for x in 0..4 {
            assert_eq!(g.pixel(x, 0), Some([0, 0, 0, 255]));
            // t = 1/2 for the middle row.
            assert_eq!(g.pixel(x, 1), Some([45, 30, 15, 255]));
            assert_eq!(g.pixel(x, 2), Some([90, 60, 30, 255]));
        }
        assert_eq!(&g, &Canvas::gradient(4, 3, [0, 0, 0], [90, 60, 30]));
        // Single-row gradient always renders the top color (t = 0 / max(1, h-1)).
        let one = Canvas::gradient(2, 1, [10, 20, 30], [200, 200, 200]);
        assert_eq!(one.pixel(0, 0), Some([10, 20, 30, 255]));
    }

    static SAVE_SEQ: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    #[test]
    fn save_png_writes_rgb_bytes_for_opaque_canvas() {
        let opaque = Canvas::gradient(4, 4, [1, 2, 3], [4, 5, 6]);
        let tmp = std::env::temp_dir().join(format!(
            "cnc-procgen-save-{}-{}.png",
            std::process::id(),
            SAVE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        opaque.save_png(&tmp).expect("save");
        let bytes = std::fs::read(&tmp).expect("read back");
        std::fs::remove_file(&tmp).ok();
        // IHDR colour type byte sits at 8 (sig) + 4 (len) + 4 ("IHDR") + 9 - 1.
        assert_eq!(bytes[25], 2);
    }
}
