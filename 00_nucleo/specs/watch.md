# Spec: `watch.rs` (Modo de Compilação Contínua)

## 1. Objetivo Central
O arquivo `watch.rs` implementa o modo `typst watch`, que monitora arquivos fonte e recompila automaticamente ao detectar mudanças. Exibe status colorido no terminal (compilando, sucesso, erro) com timestamps e duração.

## 2. Atomização da Lógica Pura (Para L1)

### Formatação de Mensagens de Status
- **`Status::message(&self) -> String`**: Texto puro baseado no estado — "compiling ...", "compiled successfully in {dur}", "compiled with warnings in {dur}", "compiled with errors".
- **`Status::color(&self) -> ColorSpec`**: Mapeamento de estado para cor — Error→header_error, PartialSuccess→header_warning, _→header_note.

### Validação de Modo
- **`validate_watch_output(output: &Output) -> bool`**: Verifica que o output não é stdout (watch mode não suporta stdout).
- **`should_warn_stdin(input: &Input) -> bool`**: Verifica se o input é stdin para emitir warning.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Terminal (Tela + Escrita Colorida)
- `terminal::out()` — Obtenção do handle de saída.
- `out.clear_screen()` — Limpeza do terminal.
- `out.set_color()` / `out.reset()` — Controle de cores ANSI.
- `write!()` / `writeln!()` — Escrita formatada.

### Efeito 2: Relógio Local
- `chrono::offset::Local::now()` — Timestamp para exibição.

### Efeito 3: File System Watcher
- `Watcher::new()` — Criação do observador de arquivos.
- `watcher.update()` / `watcher.wait()` — Controle do loop de observação.

## 4. Estruturas de Dados

```rust
enum Status {
    Compiling,
    Success(Duration),
    PartialSuccess(Duration),
    Error,
}
```
