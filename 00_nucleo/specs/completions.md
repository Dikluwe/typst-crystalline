# Spec: `completions.rs` (Geração de Shell Completions)

## 1. Objetivo Central
O arquivo `completions.rs` implementa o comando `typst completions`, que gera scripts de auto-complete para shells (Bash, Zsh, Fish, PowerShell, Elvish) usando `clap_complete`.

## 2. Atomização da Lógica Pura (Para L1)

### Extração de Metadados
- **`get_bin_name(cmd: &Command) -> String`**: Extrai o nome do binário do comando clap. Lógica trivial porém isolável.

> **Nota**: Este módulo é quase 100% I/O. A lógica pura é mínima — a maior parte é delegação para `clap_complete::generate()`.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Escrita em Stdout
- `stdout()` — Handle de saída padrão.
- `generate()` — Geração e escrita direta do script de completions.

## 4. Estruturas de Dados
- Usa `CompletionsCommand { shell: Shell }` do módulo `args`.
