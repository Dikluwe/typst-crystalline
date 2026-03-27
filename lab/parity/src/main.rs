mod compact;
use compact::{compact_cristalino, compact_original};

fn main() {
    let arg = std::env::args().nth(1)
        .unwrap_or_else(|| "Hello *world*".to_string());

    // Aceita path de ficheiro ou string inline
    let input = if std::path::Path::new(&arg).exists() {
        std::fs::read_to_string(&arg)
            .unwrap_or_else(|e| { eprintln!("Erro ao ler {}: {}", arg, e); std::process::exit(1); })
    } else {
        arg.replace("\\n", "\n")  // permitir \n na linha de comandos
    };

    let orig  = typst_syntax::parse(&input);
    let crist = typst_core::rules::parse::parse(&input);

    let co = compact_original(&orig);
    let cc = compact_cristalino(&crist);

    if co == cc {
        println!("✓ Paridade confirmada ({} bytes)", input.len());
        return;
    }

    // Gravar para diff visual
    let expected = format!("{:#?}", co);
    let actual   = format!("{:#?}", cc);

    std::fs::write("/tmp/parity_expected.txt", &expected).unwrap();
    std::fs::write("/tmp/parity_actual.txt",   &actual).unwrap();

    eprintln!("✗ Divergência detectada");
    eprintln!();
    eprintln!("  Inspecionar com:");
    eprintln!("    delta /tmp/parity_expected.txt /tmp/parity_actual.txt");
    eprintln!("    diff  /tmp/parity_expected.txt /tmp/parity_actual.txt | head -40");
    eprintln!("    code --diff /tmp/parity_expected.txt /tmp/parity_actual.txt");
    eprintln!();

    // Mostrar primeira divergência inline
    for (i, (a, b)) in expected.lines().zip(actual.lines()).enumerate() {
        if a != b {
            eprintln!("  Primeira divergência na linha {}:", i + 1);
            eprintln!("    expected: {}", a.trim());
            eprintln!("    actual:   {}", b.trim());
            break;
        }
    }

    std::process::exit(1);
}
