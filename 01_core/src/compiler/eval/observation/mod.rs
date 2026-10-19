//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/observation/mod.md
//! @prompt-hash 92917382
//! @layer L1
//! @updated 2026-09-26
//!
//! Relação observacional causal — família `mod`.

use super::*;

mod content;
mod elements;
mod geometry;
mod math;
mod page;
mod style;
mod value;

#[cfg(test)]
pub(crate) use math::{
    p1436_math_lr_observation_differs, p1437_math_mid_observation_differs,
    p1439_math_overbrace_observation_differs, p1440_math_overbracket_observation_differs,
    p1441_math_overline_observation_differs, p1442_math_overparen_observation_differs,
    p1443_math_overshell_observation_differs, p1444_math_primes_observation_differs,
    p1448_math_underbrace_observation_differs,
    p1449_math_underbracket_observation_differs,
    p1450_math_underparen_observation_differs, p1451_math_undershell_observation_differs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ObservationRelation {
    Same,
    Different,
    Unproven,
}
impl ObservationRelation {
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Different, _) | (_, Self::Different) => Self::Different,
            (Self::Unproven, _) | (_, Self::Unproven) => Self::Unproven,
            (Self::Same, Self::Same) => Self::Same,
        }
    }
    fn exact(same: bool) -> Self {
        if same {
            Self::Same
        } else {
            Self::Different
        }
    }
}
pub(super) trait ObservationEq {
    fn observation_relation(&self, other: &Self) -> ObservationRelation;
}
macro_rules! observation_scalar {
    ($($ty:ty),* $(,)?) => { $(impl ObservationEq for $ty {
        fn observation_relation(&self, other: &Self) -> ObservationRelation {
            ObservationRelation::exact(self == other)
        }
    })* };
}
// Audited leaves: no hidden Value, callback or floating-point payload.
observation_scalar!(
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    str,
    String,
    EcoString,
    crate::entities::span::Span,
    crate::entities::file_id::FileId,
    std::num::NonZeroUsize,
    crate::entities::label::Label,
    crate::entities::world_types::Datetime,
    // P1473 — enum fechado de dois variantes, `PartialEq` derivado.
    crate::entities::elements::figure_caption::CaptionPosition
);
impl ObservationEq for f64 {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_bits().observation_relation(&other.to_bits())
    }
}
impl ObservationEq for f32 {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_bits().observation_relation(&other.to_bits())
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for &T {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(*other)
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for Box<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(&**other)
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for Arc<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(&**other)
    }
}
impl<T: ObservationEq> ObservationEq for [T] {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter()
            .zip(other)
            .fold(ObservationRelation::Same, |r, (a, b)| r.and(a.observation_relation(b)))
    }
}
impl<T: ObservationEq, const N: usize> ObservationEq for [T; N] {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq> ObservationEq for Vec<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq + Clone> ObservationEq for ecow::EcoVec<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq> ObservationEq for Option<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (None, None) => ObservationRelation::Same,
            (Some(a), Some(b)) => a.observation_relation(b),
            _ => ObservationRelation::Different,
        }
    }
}
impl<T: ObservationEq, E: ObservationEq> ObservationEq for Result<T, E> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (Ok(a), Ok(b)) => a.observation_relation(b),
            (Err(a), Err(b)) => a.observation_relation(b),
            _ => ObservationRelation::Different,
        }
    }
}
impl<A: ObservationEq, B: ObservationEq> ObservationEq for (A, B) {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.0
            .observation_relation(&other.0)
            .and(self.1.observation_relation(&other.1))
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::Criterion;
    use std::hint::black_box;
    use std::time::Duration;

    use crate::entities::content::Content;
    use crate::entities::dir::Dir;
    use crate::entities::elements::math_root::MathRootElem;
    use crate::entities::elements::strong::StrongElem;
    use crate::entities::layout_types::Abs;
    use crate::entities::page_geometry::PageBinding;
    use crate::entities::style::{Style, Styles};
    use crate::entities::value::Value;

    fn criterion() -> Criterion {
        Criterion::default()
            .without_plots()
            .sample_size(30)
            .warm_up_time(Duration::from_millis(150))
            .measurement_time(Duration::from_millis(300))
    }

    #[test]
    #[ignore = "microbenchmark Criterion; executar explicitamente em perfil release"]
    fn observation_microbenchmarks() {
        let mut criterion = criterion();

        let equal_values = [7_u64; 256];
        let mut early_values = equal_values;
        early_values[0] = 8;
        let mut late_values = equal_values;
        late_values[255] = 8;
        let mut values = criterion.benchmark_group("observation/values");
        values.bench_function("equal", |b| {
            b.iter(|| {
                black_box(&equal_values).observation_relation(black_box(&equal_values))
            })
        });
        values.bench_function("early_difference", |b| {
            b.iter(|| {
                black_box(&equal_values).observation_relation(black_box(&early_values))
            })
        });
        values.bench_function("late_difference", |b| {
            b.iter(|| {
                black_box(&equal_values).observation_relation(black_box(&late_values))
            })
        });
        values.finish();

        let content_equal = Content::sequence(
            (0..128).map(|index| Content::text(format!("node-{index}"))).collect(),
        );
        let content_same = content_equal.clone();
        let content_late = Content::sequence(
            (0..128)
                .map(|index| {
                    Content::text(if index == 127 {
                        "changed".into()
                    } else {
                        format!("node-{index}")
                    })
                })
                .collect(),
        );
        let mut content = criterion.benchmark_group("observation/content");
        content.bench_function("tree_equal", |b| {
            b.iter(|| {
                black_box(&content_equal).observation_relation(black_box(&content_same))
            })
        });
        content.bench_function("tree_late_difference", |b| {
            b.iter(|| {
                black_box(&content_equal).observation_relation(black_box(&content_late))
            })
        });
        content.finish();

        let style_equal =
            Styles::from_iter([Style::Bold { value: true, from_strong: true }]);
        let style_same = style_equal.clone();
        let style_different =
            Styles::from_iter([Style::Bold { value: false, from_strong: true }]);
        let mut style = criterion.benchmark_group("observation/style");
        style.bench_function("equal", |b| {
            b.iter(|| {
                black_box(&style_equal).observation_relation(black_box(&style_same))
            })
        });
        style.bench_function("different", |b| {
            b.iter(|| {
                black_box(&style_equal).observation_relation(black_box(&style_different))
            })
        });
        style.bench_function("direction", |b| {
            b.iter(|| black_box(&Dir::LTR).observation_relation(black_box(&Dir::RTL)))
        });
        style.finish();

        let mut geometry = criterion.benchmark_group("observation/geometry");
        geometry.bench_function("equal", |b| {
            b.iter(|| {
                black_box(&Abs::pt(12.0)).observation_relation(black_box(&Abs::pt(12.0)))
            })
        });
        geometry.bench_function("different", |b| {
            b.iter(|| {
                black_box(&Abs::pt(12.0)).observation_relation(black_box(&Abs::pt(13.0)))
            })
        });
        geometry.finish();

        let math_equal = MathRootElem {
            index: Some(Content::text("3")),
            radicand: Content::text("x"),
            index_explicit: true,
        };
        let math_same = math_equal.clone();
        let math_different =
            MathRootElem { radicand: Content::text("y"), ..math_equal.clone() };
        let mut math = criterion.benchmark_group("observation/math");
        math.bench_function("equal", |b| {
            b.iter(|| black_box(&math_equal).observation_relation(black_box(&math_same)))
        });
        math.bench_function("different", |b| {
            b.iter(|| {
                black_box(&math_equal).observation_relation(black_box(&math_different))
            })
        });
        math.finish();

        let mut page = criterion.benchmark_group("observation/page");
        page.bench_function("equal", |b| {
            b.iter(|| {
                black_box(&PageBinding::Left)
                    .observation_relation(black_box(&PageBinding::Left))
            })
        });
        page.bench_function("different", |b| {
            b.iter(|| {
                black_box(&PageBinding::Left)
                    .observation_relation(black_box(&PageBinding::Right))
            })
        });
        page.finish();

        let element_equal = StrongElem::new(Content::text("body"));
        let element_same = element_equal.clone();
        let element_different = StrongElem::new(Content::text("other"));
        let mut elements = criterion.benchmark_group("observation/elements");
        elements.bench_function("equal", |b| {
            b.iter(|| {
                black_box(&element_equal).observation_relation(black_box(&element_same))
            })
        });
        elements.bench_function("different", |b| {
            b.iter(|| {
                black_box(&element_equal)
                    .observation_relation(black_box(&element_different))
            })
        });
        elements.finish();

        let value = Value::Content(content_equal);
        criterion.bench_function("observation/value_content", |b| {
            b.iter(|| black_box(&value).observation_relation(black_box(&value)))
        });
        criterion.final_summary();
    }
}
