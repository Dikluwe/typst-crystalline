use std::sync::Arc;

use typst_core::entities::axes::Axes;
use typst_core::entities::gradient::{Gradient, GradientStop};
use typst_core::entities::layout_types::{Angle, Color, ColorSpace, Ratio};

#[path = "../../../../../03_infra/src/export/gradients/adaptive.rs"]
mod adaptive;

fn usage() -> ! {
    eprintln!(
        "usage: p1266-svg-generalization-probe <linear|radial> \
         <oklab|linear-rgb> <true|false> <c0,c1,c2,a,offset;...>"
    );
    std::process::exit(2);
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
        "oklab" => ColorSpace::Oklab,
        "linear-rgb" => ColorSpace::LinearRgb,
        _ => usage(),
    };
    let stops: Vec<_> = encoded
        .split(';')
        .filter(|row| !row.is_empty())
        .map(|row| {
            let fields: Vec<_> = row.split(',').collect();
            if fields.len() != 5 {
                usage();
            }
            let component =
                |index: usize| fields[index].parse::<f32>().unwrap_or_else(|_| usage());
            let offset = fields[4].parse::<f64>().unwrap_or_else(|_| usage());
            let color = match space {
                ColorSpace::Oklab => {
                    Color::oklab(component(0), component(1), component(2), component(3))
                }
                ColorSpace::LinearRgb => Color::linear_rgb(
                    component(0),
                    component(1),
                    component(2),
                    component(3),
                ),
                _ => unreachable!(),
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

    println!("index\toffset\tr\tg\tb\ta");
    for (index, stop) in adaptive::svg_adaptive_stops(&gradient).iter().enumerate() {
        let (r, g, b, a) = stop.color.to_srgb();
        println!(
            "{}\t{:.17}\t{}\t{}\t{}\t{}",
            index,
            stop.offset.expect("adaptive stop offset").0,
            r,
            g,
            b,
            a
        );
    }
}
