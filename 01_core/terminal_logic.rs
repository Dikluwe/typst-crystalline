//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Terminal
//! Responsabilidade: Constantes ANSI e resolução pura de ColorChoice.

use codespan_reporting::term::termcolor::ColorChoice;

/// Sequência ANSI para limpar a tela inteira e reposicionar o cursor no topo-esquerdo.
pub const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

/// Sequência ANSI para mover o cursor uma linha acima e limpar do cursor até o fim da tela.
pub const ANSI_CLEAR_LAST_LINE: &str = "\x1B[1F\x1B[0J";

/// Resolve a escolha de cor basead nas opções da CLI e na presença de TTY.
pub fn resolve_color_choice(clap_color: clap::ColorChoice, is_tty: bool) -> ColorChoice {
    match clap_color {
        clap::ColorChoice::Auto if is_tty => ColorChoice::Auto,
        clap::ColorChoice::Always => ColorChoice::Always,
        _ => ColorChoice::Never,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_clear_screen_constant() {
        assert_eq!(ANSI_CLEAR_SCREEN, "\x1B[2J\x1B[1;1H");
    }

    #[test]
    fn test_ansi_clear_last_line_constant() {
        assert_eq!(ANSI_CLEAR_LAST_LINE, "\x1B[1F\x1B[0J");
    }

    #[test]
    fn test_resolve_color_auto_with_tty() {
        assert_eq!(
            resolve_color_choice(clap::ColorChoice::Auto, true),
            ColorChoice::Auto,
        );
    }

    #[test]
    fn test_resolve_color_auto_without_tty() {
        assert_eq!(
            resolve_color_choice(clap::ColorChoice::Auto, false),
            ColorChoice::Never,
        );
    }

    #[test]
    fn test_resolve_color_always() {
        assert_eq!(
            resolve_color_choice(clap::ColorChoice::Always, false),
            ColorChoice::Always,
        );
    }

    #[test]
    fn test_resolve_color_never() {
        assert_eq!(
            resolve_color_choice(clap::ColorChoice::Never, true),
            ColorChoice::Never,
        );
    }
}
