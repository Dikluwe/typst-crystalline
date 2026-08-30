use palette::convert::FromColorUnclamped;
use palette::{FromColor, Oklaba, Srgba};

fn main() {
    let mut values = std::env::args().skip(1).map(|value| {
        value
            .parse::<f32>()
            .unwrap_or_else(|error| panic!("invalid f32 {value:?}: {error}"))
    });
    let color = Oklaba::new(
        values.next().expect("l"),
        values.next().expect("a"),
        values.next().expect("b"),
        values.next().expect("alpha"),
    );
    assert!(values.next().is_none(), "expected exactly four components");
    let raw = Srgba::<f32>::from_color_unclamped(color);
    let clamped = Srgba::<f32>::from_color(color);
    println!(
        "{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}",
        raw.red,
        raw.green,
        raw.blue,
        raw.alpha,
        clamped.red,
        clamped.green,
        clamped.blue,
        clamped.alpha
    );
}
