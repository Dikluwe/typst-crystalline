//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Greet
//! Responsabilidade: Lógica de decisão de saudação, geração do path e strings constantes.

use std::path::{Path, PathBuf};

/// Mensagem de boas-vindas exibida na primeira execução.
#[rustfmt::skip]
pub const GREETING: &str = color_print::cstr!("\
<s>Welcome to Typst, we are glad to have you here!</> ❤️

If you are new to Typst, <s>start with the tutorial</> at \
<u>https://typst.app/docs/tutorial/</>. To get a quick start with your first \
project, <s>choose a template</> on <u>https://typst.app/universe/</>.

Here are the <s>most important commands</> you will be using:

- Compile a file once: <c!>typst compile file.typ</>
- Compile a file on every change: <c!>typst watch file.typ</>
- Set up a project from a template: <c!>typst init @preview/<<TEMPLATE>></>

Learn more about these commands by running <c!>typst help</>.

If you have a question, we and our community would be glad to help you out on \
the <s>Typst Forum</> at <u>https://forum.typst.app/</>.

Happy Typsting!
");

/// Calcula o caminho do arquivo de controle 'greeted'
/// baseado no diretório de dados do sistema.
pub fn compute_greet_path(data_dir: &Path) -> PathBuf {
    data_dir.join("typst").join("greeted")
}

/// Determina se devemos saudar o usuário.
/// Retorna `true` se a versão não estava registrada ou é diferente da atual.
pub fn should_greet(prev_version: Option<&str>, current_version: &str) -> bool {
    prev_version != Some(current_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_greet_path() {
        let dir = Path::new("/var/data");
        let path = compute_greet_path(dir);
        assert_eq!(path, PathBuf::from("/var/data/typst/greeted"));
    }

    #[test]
    fn test_should_greet_when_no_prev_version() {
        assert!(should_greet(None, "0.12.0"));
    }

    #[test]
    fn test_should_greet_when_different_version() {
        assert!(should_greet(Some("0.11.0"), "0.12.0"));
    }

    #[test]
    fn test_should_not_greet_when_same_version() {
        assert!(!should_greet(Some("0.12.0"), "0.12.0"));
    }
}
