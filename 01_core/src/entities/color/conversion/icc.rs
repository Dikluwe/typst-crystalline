//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/color/conversion/icc.md
//! @prompt-hash c2e9c026
//! @layer L1
//! @updated 2026-10-21
//!
//! Pure projection through the embedded CMYK profile.
//!
//! Scalar stages and interpolation are adapted from moxcms 0.8.1 by
//! Radzivon Bartoshyk, under Apache-2.0 (dual-licensed BSD-3-Clause OR
//! Apache-2.0): conversions/lut4.rs, conversions/lut_transforms.rs,
//! conversions/prelude_lut_xyz_rgb.rs and conversions/avx/interpolator_q0_15.rs.
//! The immutable profile is the CC0 CGATS TR 001-1995 asset distributed by
//! typst-assets. No profile instances, runtime metadata or global cache exist.

use moxcms::{
    ColorPrimaries, ColorProfile, Hypercube, Lab, Matrix3f, TransferCharacteristics,
    WHITE_POINT_D65,
};
use typst_assets::icc::CMYK_TO_XYZ;

/// Project without changing the stored CMYK channels or other color spaces.
pub(super) fn to_rgba(c: f32, m: f32, y: f32, k: f32) -> (f32, f32, f32, f32) {
    let source = [c, m, y, k].map(|channel| (channel * 255.0).round() as u8);
    let [r, g, b] = Profile::parse(CMYK_TO_XYZ).convert(source);
    (f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0, 1.0)
}

fn u16be(data: &[u8], at: usize) -> u16 {
    u16::from_be_bytes(
        data.get(at..at.checked_add(2).expect("embedded ICC offset overflow"))
            .expect("embedded ICC read exceeds bounds")
            .try_into()
            .expect("two-byte ICC sample"),
    )
}
fn u32be(data: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(
        data.get(at..at.checked_add(4).expect("embedded ICC offset overflow"))
            .expect("embedded ICC read exceeds bounds")
            .try_into()
            .expect("four-byte ICC field"),
    )
}

struct Profile {
    input: [Vec<f32>; 4],
    clut: Vec<f32>,
    output: [Vec<f32>; 3],
    grid: usize,
    lab_v2: bool,
    xyz_to_srgb: Matrix3f,
}

impl Profile {
    fn parse(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 132, "embedded ICC header is truncated");
        assert_eq!(u32be(bytes, 0) as usize, bytes.len());
        assert_eq!(&bytes[36..40], b"acsp");
        assert_eq!(&bytes[12..16], b"scnr", "unsupported embedded ICC class");
        assert_eq!(&bytes[16..20], b"CMYK");
        assert_eq!(&bytes[20..24], b"Lab ");
        let tags = u32be(bytes, 128) as usize;
        assert!(tags.checked_mul(12).unwrap().checked_add(132).unwrap() <= bytes.len());
        let mut a2b = None;
        for n in 0..tags {
            let p = 132 + n * 12;
            let offset = u32be(bytes, p + 4) as usize;
            let length = u32be(bytes, p + 8) as usize;
            let end =
                offset.checked_add(length).expect("embedded ICC tag offset overflow");
            let data = bytes
                .get(offset..end)
                .expect("embedded ICC tag exceeds profile bounds");
            if &bytes[p..p + 4] == b"A2B0" {
                assert!(a2b.is_none(), "embedded ICC has duplicate A2B0 tags");
                a2b = Some(data);
            }
        }
        let b = a2b.expect("embedded ICC has no A2B0 tag");
        assert!(b.len() >= 52, "embedded ICC mft2 header is truncated");
        assert_eq!(&b[0..4], b"mft2");
        assert_eq!((b[8], b[9]), (4, 3));
        let grid = b[10] as usize;
        assert!(grid >= 2);
        // This fixed profile has the identity XYZ matrix. There is no invented
        // matrix interpretation for unsupported/non-identity profile structures.
        for n in 0..9 {
            assert_eq!(
                u32be(b, 12 + n * 4),
                if n % 4 == 0 { 65536 } else { 0 },
                "unsupported embedded ICC matrix"
            );
        }
        let ni = u16be(b, 48) as usize;
        let no = u16be(b, 50) as usize;
        assert!(ni >= 2 && no >= 2);
        let nc = grid.checked_pow(4).unwrap().checked_mul(3).unwrap();
        let entries = ni
            .checked_mul(4)
            .unwrap()
            .checked_add(nc)
            .unwrap()
            .checked_add(no.checked_mul(3).unwrap())
            .unwrap();
        let expected = entries
            .checked_mul(2)
            .and_then(|size| size.checked_add(52))
            .expect("embedded ICC table size overflow");
        assert_eq!(expected, b.len(), "embedded ICC table length mismatch");
        let mut at = 52;
        let mut table = |count: usize| {
            (0..count)
                .map(|_| {
                    let value = u16be(b, at) as f32 * (1.0f32 / 65535.0);
                    at += 2;
                    value
                })
                .collect::<Vec<_>>()
        };
        let input = std::array::from_fn(|_| table(ni));
        let clut = table(nc);
        let output = std::array::from_fn(|_| table(no));
        // Calls only pure matrix construction, not ColorProfile::new/default.
        let xyz_to_srgb =
            ColorProfile::colorants_matrix(WHITE_POINT_D65, ColorPrimaries::BT_709)
                .inverse()
                .to_f32();
        Self {
            input,
            clut,
            output,
            grid,
            lab_v2: bytes[8] < 4,
            xyz_to_srgb,
        }
    }

    fn node(&self, coordinates: [usize; 4]) -> [i32; 3] {
        // Upstream resamples to 17^4 nodes. Compute just the requested node,
        // preserving the input-curve/CLUT/output-curve stage ordering.
        let p: [f32; 4] = std::array::from_fn(|n| {
            curve(coordinates[n] as f32 * (1.0 / 16.0), &self.input[n])
        });
        let clut = Hypercube::new(&self.clut, self.grid, 3)
            .unwrap()
            .quadlinear_vec3(p[0], p[1], p[2], p[3]);
        let mut lab: [f32; 3] =
            std::array::from_fn(|n| curve(clut.v[n], &self.output[n]));
        if self.lab_v2 {
            for x in &mut lab {
                *x *= 65535.0f32 / 65280.0;
            }
        }
        let xyz = Lab::new(lab[0], lab[1], lab[2]).to_pcs_xyz();
        let v = [
            xyz.x * (65535.0f32 / 32768.0),
            xyz.y * (65535.0f32 / 32768.0),
            xyz.z * (65535.0f32 / 32768.0),
        ];
        std::array::from_fn(|n| {
            let m = self.xyz_to_srgb.v[n];
            // Same non-fused generic stage ordering as upstream moxcms.
            let linear = (v[0] * m[0] + v[1] * m[1]) + v[2] * m[2];
            let index = (0.5f32 + linear * 4095.0).clamp(0.0, 4095.0) as u16;
            let encoded =
                TransferCharacteristics::Srgb.gamma(index as f64 * (1.0 / 4095.0));
            ((encoded * 255.0) as f32).round() as i32
        })
    }

    fn convert(&self, source: [u8; 4]) -> [u8; 3] {
        let weights = source.map(weight);
        let mut nodes = [[0; 3]; 16];
        for (mask, node) in nodes.iter_mut().enumerate() {
            *node = self.node(std::array::from_fn(|axis| {
                if mask & (1 << axis) == 0 {
                    weights[axis].0
                } else {
                    weights[axis].1
                }
            }));
        }
        // Exact scalar equivalent of rounded high multiply in each SIMD axis.
        // C then M then Y then K, rounding each product separately.
        for (axis, &(_, _, axis_weight)) in weights.iter().enumerate() {
            for mask in 0..16 {
                if mask & ((1 << (axis + 1)) - 1) == 0 {
                    let b = nodes[mask | (1 << axis)];
                    for (a, b) in nodes[mask].iter_mut().zip(b) {
                        *a = qlerp(*a, b, axis_weight);
                    }
                }
            }
        }
        nodes[0].map(|v| v.clamp(0, 255) as u8)
    }
}

fn curve(x: f32, values: &[f32]) -> f32 {
    let value = x.clamp(0.0, 1.0) * (values.len() - 1) as f32;
    let hi = value.ceil() as usize;
    let lo = value.floor() as usize;
    let d = hi as f32 - value;
    let upper = values[hi];
    (upper + upper * -d) + values[lo] * d
}
fn weight(v: u8) -> (usize, usize, i32) {
    let lo = v as usize * 16 / 255;
    let w =
        ((v as f32 * (16.0f32 * (1.0f32 / 255.0)) - lo as f32) * 32767.0).round() as i32;
    (lo, (lo + 1).min(16), w)
}
fn mulhrs(a: i32, b: i32) -> i32 {
    (a * b + 16384) >> 15
}
fn qlerp(a: i32, b: i32, w: i32) -> i32 {
    mulhrs(a, 32767 - w) + mulhrs(b, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "embedded ICC tag exceeds profile bounds")]
    fn rejects_a_tag_outside_the_embedded_profile() {
        let mut malformed = CMYK_TO_XYZ.to_vec();
        malformed[136..140].copy_from_slice(&u32::MAX.to_be_bytes());
        Profile::parse(&malformed);
    }

    #[test]
    #[should_panic(expected = "unsupported embedded ICC matrix")]
    fn rejects_an_unsupported_profile_matrix() {
        let mut malformed = CMYK_TO_XYZ.to_vec();
        let tags = u32be(&malformed, 128) as usize;
        let directory = (0..tags)
            .map(|index| 132 + 12 * index)
            .find(|offset| &malformed[*offset..*offset + 4] == b"A2B0")
            .expect("the embedded profile supplies A2B0");
        let tag = u32be(&malformed, directory + 4) as usize;
        malformed[tag + 12..tag + 16].copy_from_slice(&0u32.to_be_bytes());
        Profile::parse(&malformed);
    }
}
