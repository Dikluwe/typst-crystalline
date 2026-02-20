//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Deps
//! Responsabilidade: Escaping de strings para Make, relativização de paths e formatação de dependências.

use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;

/// Escaping de caracteres especiais para formato Make/GCC.
///
/// Baseado em `munge` do `libcpp/mkdeps.cc` do GCC.
/// Escapa `\`, `$`, `:`, espaços, tabs e `#`.
pub fn munge(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    let mut slashes = 0;
    for c in s.chars() {
        match c {
            '\\' => slashes += 1,
            '$' => {
                res.push('$');
                slashes = 0;
            }
            ':' => {
                res.push('\\');
                slashes = 0;
            }
            ' ' | '\t' => {
                // "A space or tab preceded by 2N+1 backslashes represents
                // N backslashes followed by space..."
                for _ in 0..slashes + 1 {
                    res.push('\\');
                }
                slashes = 0;
            }
            '#' => {
                res.push('\\');
                slashes = 0;
            }
            _ => slashes = 0,
        };
        res.push(c);
    }
    res
}

/// Converte um path absoluto de dependência para relativo ao root do projeto.
pub fn relativize_dependency(
    dependency: &Path,
    root: &Path,
    relative_root: &Path,
) -> PathBuf {
    dependency
        .strip_prefix(root)
        .map_or_else(|_| dependency.to_path_buf(), |x| relative_root.join(x))
}

/// Calcula o root relativo ao diretório de trabalho atual.
pub fn compute_relative_root(root: &Path, current_dir: &Path) -> PathBuf {
    pathdiff::diff_paths(root, current_dir).unwrap_or_else(|| root.to_path_buf())
}

/// Estrutura de dependências para serialização JSON.
#[derive(Debug, Serialize, PartialEq)]
pub struct DepsJson {
    pub inputs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
}

/// Formata dependências no formato Make.
///
/// Gera uma regra no formato: `output1 output2: dep1 dep2\n`
pub fn format_make_rule(outputs: &[&str], deps: &[&str]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for (i, output) in outputs.iter().enumerate() {
        if i != 0 {
            buffer.write_all(b" ").unwrap();
        }
        buffer.write_all(munge(output).as_bytes()).unwrap();
    }
    buffer.write_all(b":").unwrap();
    for dep in deps {
        buffer.write_all(b" ").unwrap();
        buffer.write_all(munge(dep).as_bytes()).unwrap();
    }
    buffer.write_all(b"\n").unwrap();
    buffer
}

/// Formata dependências no formato Zero (null-separated).
pub fn format_zero_deps(deps: &[&str]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for dep in deps {
        buffer.write_all(dep.as_bytes()).unwrap();
        buffer.write_all(b"\0").unwrap();
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_munge_plain() {
        assert_eq!(munge("main.typ"), "main.typ");
    }

    #[test]
    fn test_munge_space() {
        assert_eq!(munge("my file.typ"), "my\\ file.typ");
    }

    #[test]
    fn test_munge_dollar() {
        assert_eq!(munge("price$5"), "price$$5");
    }

    #[test]
    fn test_munge_colon() {
        assert_eq!(munge("C:file"), "C\\:file");
    }

    #[test]
    fn test_munge_hash() {
        assert_eq!(munge("file#1"), "file\\#1");
    }

    #[test]
    fn test_munge_tab() {
        assert_eq!(munge("a\tb"), "a\\\tb");
    }

    #[test]
    fn test_munge_backslash_space() {
        // Backslash followed by space: 1 slash => 2 slashes + space
        assert_eq!(munge("\\ "), "\\\\\\ ");
    }

    #[test]
    fn test_relativize_within_root() {
        let dep = Path::new("/project/src/main.typ");
        let root = Path::new("/project");
        let relative_root = Path::new(".");
        assert_eq!(
            relativize_dependency(dep, root, relative_root),
            PathBuf::from("./src/main.typ")
        );
    }

    #[test]
    fn test_relativize_outside_root() {
        let dep = Path::new("/other/lib.typ");
        let root = Path::new("/project");
        let relative_root = Path::new(".");
        assert_eq!(
            relativize_dependency(dep, root, relative_root),
            PathBuf::from("/other/lib.typ")
        );
    }

    #[test]
    fn test_compute_relative_root() {
        let root = Path::new("/home/user/project");
        let cwd = Path::new("/home/user");
        assert_eq!(compute_relative_root(root, cwd), PathBuf::from("project"));
    }

    #[test]
    fn test_format_make_rule() {
        let result = format_make_rule(&["output.pdf"], &["main.typ", "lib.typ"]);
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "output.pdf: main.typ lib.typ\n"
        );
    }

    #[test]
    fn test_format_make_rule_with_special_chars() {
        let result = format_make_rule(&["my output.pdf"], &["my file.typ"]);
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "my\\ output.pdf: my\\ file.typ\n"
        );
    }

    #[test]
    fn test_format_zero_deps() {
        let result = format_zero_deps(&["a.typ", "b.typ"]);
        assert_eq!(result, b"a.typ\0b.typ\0");
    }

    #[test]
    fn test_deps_json_serialize() {
        let deps = DepsJson {
            inputs: vec!["main.typ".into(), "lib.typ".into()],
            outputs: Some(vec!["output.pdf".into()]),
        };
        let json = serde_json::to_string(&deps).unwrap();
        assert!(json.contains("\"inputs\""));
        assert!(json.contains("main.typ"));
        assert!(json.contains("\"outputs\""));
    }

    #[test]
    fn test_deps_json_no_outputs() {
        let deps = DepsJson {
            inputs: vec!["main.typ".into()],
            outputs: None,
        };
        let json = serde_json::to_string(&deps).unwrap();
        assert!(!json.contains("outputs"));
    }
}
