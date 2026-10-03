use grid_prototypes::{fixture, run, Options};
use std::hint::black_box;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rows = 80_usize;
    let mut repetitions = 15_usize;
    let mut repeated = true;
    let mut first_page = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--rows" => rows = args.next().ok_or("--rows exige valor")?.parse()?,
            "--repetitions" => {
                repetitions = args.next().ok_or("--repetitions exige valor")?.parse()?
            }
            "--unique" => repeated = false,
            "--first-page" => first_page = true,
            "--help" | "-h" => {
                println!("Uso: grid-prototypes [--rows N] [--repetitions N] [--unique] [--first-page]");
                return Ok(());
            }
            _ => return Err(format!("argumento desconhecido: {arg}").into()),
        }
    }
    if repetitions == 0 {
        return Err("--repetitions deve ser maior que zero".into());
    }
    let grid = fixture(rows, repeated);
    let reference = run(&grid, Options::BASELINE, first_page)?.output;
    println!(
        "fixture: rows={rows} cells={} repeated={repeated} first_page={first_page}",
        grid.cells.len()
    );
    println!("mode,median_us,intrinsic_calls,intrinsic_hits,full_layouts,height_layouts,frame_reuses,frame_fallbacks,peak_saved_lines,measured_rows,equal_output");
    for mask in 0..16 {
        let options = Options::from_mask(mask);
        let mut timings = Vec::with_capacity(repetitions);
        let mut last = None;
        for _ in 0..repetitions {
            let start = Instant::now();
            let result = run(black_box(&grid), black_box(options), first_page)?;
            let micros = start.elapsed().as_micros();
            black_box(&result.output);
            timings.push(micros);
            last = Some(result);
        }
        timings.sort_unstable();
        let result = last.expect("repetitions > 0");
        let equal = result.output == reference;
        let stats = &result.stats;
        println!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            options.label(),
            timings[timings.len() / 2],
            stats.intrinsic_calls,
            stats.intrinsic_hits,
            stats.full_layouts,
            stats.height_layouts,
            stats.frame_reuses,
            stats.frame_fallbacks,
            stats.peak_saved_lines,
            stats.measured_rows,
            equal
        );
        if !equal {
            return Err(format!("saída divergiu: {}", options.label()).into());
        }
    }
    Ok(())
}
