use std::sync::Arc;

use typst_core::entities::axes::Axes;
use typst_core::entities::gradient::{Gradient, GradientStop};
use typst_core::entities::layout_types::{Angle, Color, ColorSpace, Ratio};

#[path = "../../../../../03_infra/src/export/gradients/adaptive.rs"]
mod adaptive;

fn usage() -> ! {
    eprintln!(
        "usage: p1278-svg-polar-probe <linear|radial> \
         <oklch|hsl|hsv|luma> <true|false> <components...,offset;...>"
    );
    std::process::exit(2);
}

fn parse(field: Option<&&str>) -> f32 {
    field.and_then(|value| value.parse().ok()).unwrap_or_else(|| usage())
}

fn ratio_repr(offset: f64) -> String {
    let percent = ((offset * 100.0) * 100.0).round() / 100.0;
    format!("{percent}%")
}

fn main() {
    let mut args = std::env::args().skip(1);
    let kind = args.next().unwrap_or_else(|| usage());
    let space_name = args.next().unwrap_or_else(|| usage());
    let anti_alias = args
        .next()
        .unwrap_or_else(|| usage())
        .parse::<bool>()
        .unwrap_or_else(|_| usage());
    let encoded = args.next().unwrap_or_else(|| usage());
    if args.next().is_some() {
        usage();
    }

    let space = match space_name.as_str() {
        "oklch" => ColorSpace::Oklch,
        "hsl" => ColorSpace::Hsl,
        "hsv" => ColorSpace::Hsv,
        "luma" => ColorSpace::Luma,
        _ => usage(),
    };
    let stops: Vec<_> = encoded
        .split(';')
        .filter(|row| !row.is_empty())
        .map(|row| {
            let fields: Vec<_> = row.split(',').collect();
            let (color, offset) = match space {
                ColorSpace::Oklch if fields.len() == 5 => (
                    Color::oklch(
                        parse(fields.first()),
                        parse(fields.get(1)),
                        parse(fields.get(2)),
                        parse(fields.get(3)),
                    ),
                    fields[4].parse::<f64>().unwrap_or_else(|_| usage()),
                ),
                ColorSpace::Hsl if fields.len() == 5 => (
                    Color::hsl(
                        parse(fields.first()),
                        parse(fields.get(1)),
                        parse(fields.get(2)),
                        parse(fields.get(3)),
                    ),
                    fields[4].parse::<f64>().unwrap_or_else(|_| usage()),
                ),
                ColorSpace::Hsv if fields.len() == 5 => (
                    Color::hsv(
                        parse(fields.first()),
                        parse(fields.get(1)),
                        parse(fields.get(2)),
                        parse(fields.get(3)),
                    ),
                    fields[4].parse::<f64>().unwrap_or_else(|_| usage()),
                ),
                ColorSpace::Luma if fields.len() == 3 => (
                    Color::Luma { l: parse(fields.first()), a: parse(fields.get(1)) },
                    fields[2].parse::<f64>().unwrap_or_else(|_| usage()),
                ),
                _ => usage(),
            };
            GradientStop::new(color, Ratio(offset))
        })
        .collect();

    let mut gradient = match kind.as_str() {
        "linear" => Gradient::linear_with_space(stops, Angle::deg(0.0), space),
        "radial" => Gradient::radial_with_space(
            stops,
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            space,
        ),
        _ => usage(),
    };
    match &mut gradient {
        Gradient::Linear(value) => Arc::make_mut(value).anti_alias = anti_alias,
        Gradient::Radial(value) => Arc::make_mut(value).anti_alias = anti_alias,
        Gradient::Conic(_) => unreachable!(),
    }

    let sampled = adaptive::svg_adaptive_stops(&gradient);
    println!("index\toffset\toffset_xml\tr\tg\tb\ta");
    for (index, stop) in sampled.iter().enumerate() {
        let (r, g, b, a) = stop.color.to_srgb();
        let offset = stop.offset.expect("adaptive stop offset").0;
        let serialized =
            if matches!(space, ColorSpace::Oklch | ColorSpace::Hsl | ColorSpace::Hsv) {
                ratio_repr(offset)
            } else if index >= sampled.len().saturating_sub(2) {
                let percent =
                    (f64::from(offset as f32) * 10_000.0).round_ties_even() / 100.0;
                format!("{percent}%")
            } else {
                format!("{}", f64::from(offset as f32))
            };
        println!("{index}\t{offset:.17}\t{serialized}\t{r}\t{g}\t{b}\t{a}");
    }
}
