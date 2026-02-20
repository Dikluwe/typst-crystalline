# Spec: `deps.rs` (Exportação de Dependências de Compilação)

## 1. Objetivo Central
O arquivo `deps.rs` implementa a exportação de dependências do Typst em três formatos: JSON, Zero (null-separated) e Make (Makefile-style). Usado após compilação para integração com build systems externos.

## 2. Atomização da Lógica Pura (Para L1)

### Escaping de Strings (Make format)
- **`munge(s: &str) -> String`**: Implementa escaping de caracteres especiais para formato Make/GCC. Lida com `\`, `$`, `:`, espaços, tabs e `#`. **100% pura, sem I/O.**

### Relativização de Paths
- **`relativize_path(abs_path: &Path, root: &Path, relative_root: &Path) -> PathBuf`**: Converte um path absoluto de dependência para relativo ao root do projeto. Lógica pura de manipulação de caminhos.

### Formatação de Dependências
- **`format_deps_json(inputs: Vec<String>, outputs: Option<Vec<String>>) -> serde_json::Value`**: Monta a estrutura JSON de dependências sem I/O.
- **`format_deps_make(outputs: &[&str], deps: &[&str]) -> Vec<u8>`**: Formata a regra Make em buffer de bytes.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Escrita em Arquivo/Stdout
- `dest.open()` → Abre o destino para escrita (via `Output::open()`).
- `Write::write_all()` → Escrita de bytes no destino.

### Efeito 2: Diretório de Trabalho
- `std::env::current_dir()` → Para calcular paths relativos.

## 4. Estruturas de Dados

```rust
enum DepsFormat { Json, Zero, Make }
struct Deps { inputs: Vec<String>, outputs: Option<Vec<String>> }
```
