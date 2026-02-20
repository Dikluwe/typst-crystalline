# Spec: `terminal.rs` (Abstração de Terminal Colorido)

## 1. Objetivo Central
O arquivo `terminal.rs` fornece uma abstração centralizada para escrita colorida no terminal (`TermOut`). Atua como singleton global que encapsula `termcolor::StandardStream` e decide se cores são suportadas com base nos argumentos da CLI (`--color`) e na presença de um TTY. Todo o CLI usa `terminal::out()` para imprimir mensagens coloridas e limpar linhas.

## 2. Atomização da Lógica Pura (Para L1)

### Escape Codes ANSI
- **`ANSI_CLEAR_SCREEN`**: Sequência `\x1B[2J\x1B[1;1H` (limpa tela + cursor top-left).
- **`ANSI_CLEAR_LAST_LINE`**: Sequência `\x1B[1F\x1B[0J` (move cursor para cima + limpa até o fim).

### Resolução de Cor
- **`resolve_color_choice(clap_color: clap::ColorChoice, is_tty: bool) -> ColorChoice`**: 
  - `Auto + TTY → Auto`
  - `Always → Always`
  - `_ → Never`
  
  Lógica 100% pura, isenta de I/O.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Stream Global Singleton
- `typst::utils::singleton!` — Cria um `TermOutInner` como singleton lazily-initialized.
- `termcolor::StandardStream::stderr(color_choice)` — Abre stream com detecção de cor.

### Efeito 2: Operações de Escrita no Terminal
- `write!`, `flush()` — Escrita e limpeza do buffer no `stderr`.
- `set_color()`, `reset()` — Manipulação de cores via `WriteColor`.

### Efeito 3: Detecção de TTY
- `std::io::stderr().is_terminal()` — Verifica se stderr é um TTY.

## 4. Estruturas de Dados Chave
- `TermOut`: Handle público para escrita colorida. Impl `Write` + `WriteColor`.
- `TermOutInner`: Struct interna que contém o `StandardStream` singleton.
