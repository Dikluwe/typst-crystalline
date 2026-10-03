//! Medição intercalada dos protótipos. Os tempos abrangem `run`, inclusive
//! alocações e preparação da grade, mas não a construção da fixture.

use grid_prototypes::{fixture_with_word_repeat, run, Options};
use std::hint::black_box;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rows = 80_usize;
    let mut blocks = 21_usize;
    let mut iterations = 100_usize;
    let mut repeated = true;
    let mut first_page = false;
    let mut word_repeat = 1_usize;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--rows" => rows = args.next().ok_or("--rows exige valor")?.parse()?,
            "--blocks" => blocks = args.next().ok_or("--blocks exige valor")?.parse()?,
            "--iterations" => {
                iterations = args.next().ok_or("--iterations exige valor")?.parse()?
            }
            "--unique" => repeated = false,
            "--first-page" => first_page = true,
            "--word-repeat" => {
                word_repeat = args.next().ok_or("--word-repeat exige valor")?.parse()?
            }
            "--help" | "-h" => {
                println!("Uso: grid-prototypes-measure [--rows N] [--blocks N] [--iterations N] [--unique] [--first-page] [--word-repeat N]");
                return Ok(());
            }
            _ => return Err(format!("argumento desconhecido: {arg}").into()),
        }
    }
    if rows == 0 || blocks == 0 || iterations == 0 || word_repeat == 0 {
        return Err("rows, blocks, iterations e word-repeat devem ser positivos".into());
    }

    let grid = fixture_with_word_repeat(rows, repeated, word_repeat);
    let reference = run(&grid, Options::BASELINE, first_page)?.output;
    let mut timings: [Vec<f64>; 16] = std::array::from_fn(|_| Vec::with_capacity(blocks));
    let mut stats = Vec::with_capacity(16);
    for mask in 0..16 {
        let result = run(&grid, Options::from_mask(mask), first_page)?;
        if result.output != reference {
            return Err(
                format!("saída divergiu: {}", Options::from_mask(mask).label()).into()
            );
        }
        stats.push(result.stats);
    }

    // Aquece cada caminho. Em cada bloco, muda o primeiro modo para distribuir
    // efeitos de temperatura, frequência e caches ao longo das amostras.
    for mask in 0..16 {
        for _ in 0..20 {
            black_box(run(
                black_box(&grid),
                black_box(Options::from_mask(mask)),
                first_page,
            )?);
        }
    }
    for block in 0..blocks {
        for offset in 0..16 {
            let mask = (block + offset) % 16;
            let options = Options::from_mask(mask as u8);
            let start = Instant::now();
            for _ in 0..iterations {
                let result = run(black_box(&grid), black_box(options), first_page)?;
                black_box(result);
            }
            timings[mask].push(start.elapsed().as_nanos() as f64 / iterations as f64);
        }
    }

    for samples in &mut timings {
        samples.sort_by(|a, b| a.total_cmp(b));
    }
    let baseline = median(&timings[0]);
    println!("fixture: rows={rows} cells={} repeated={repeated} first_page={first_page} word_repeat={word_repeat} blocks={blocks} iterations={iterations}", grid.cells.len());
    println!("mode,median_ns,p10_ns,p90_ns,vs_baseline,intrinsic_calls,intrinsic_hits,full_layouts,height_layouts,frame_reuses,frame_fallbacks,peak_saved_lines,measured_rows");
    for mask in 0..16 {
        let samples = &timings[mask];
        let s = &stats[mask];
        println!(
            "{},{:.1},{:.1},{:.1},{:.3},{},{},{},{},{},{},{},{}",
            Options::from_mask(mask as u8).label(),
            median(samples),
            samples[(samples.len() - 1) / 10],
            samples[(samples.len() - 1) * 9 / 10],
            median(samples) / baseline,
            s.intrinsic_calls,
            s.intrinsic_hits,
            s.full_layouts,
            s.height_layouts,
            s.frame_reuses,
            s.frame_fallbacks,
            s.peak_saved_lines,
            s.measured_rows,
        );
    }
    Ok(())
}

fn median(sorted: &[f64]) -> f64 {
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    }
}
