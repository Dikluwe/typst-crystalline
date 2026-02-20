# Spec: `greet.rs` (Boas-vindas da CLI)

## 1. Objetivo Central
O arquivo `greet.rs` executa uma vez durante a primeira utilização da CLI do Typst daquela versão. Imprime uma mensagem de boas-vindas colorida, quebra linhas apropriadamente e sai do processo. Registra que o usuário já foi recebido num arquivo de controle no diretório de dados do sistema.

## 2. Atomização da Lógica Pura (Para L1)

### Mensagem em Si
- **`GREETING`**: A string constante contendo a mensagem de boas-vindas.

### Cálculo do Caminho de Controle
- **`compute_greet_path(data_dir: &Path) -> PathBuf`**: Retorna `data_dir.join("typst").join("greeted")`. Lógica pura.

### Decisão de Greet
- **`should_greet(prev_version: Option<&str>, current_version: &str) -> bool`**: Retorna true se a versão anterior registrada for diferente da atual. Lógica pura.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Sistema de Arquivos (Estado de Instalação)
- `dirs::data_dir()` — Descobre o diretório de dados do usuário no SO.
- `std::fs::read_to_string` / `std::fs::write` — Lê/escreve a string da versão para marcar que foi "greeted".

### Efeito 2: Terminal (Saída e Fluxo)
- Abuso de `clap::Command` para fazer wrapping (quebra de linha visual) no terminal em 80 colunas e colorir.
- `std::process::exit()` — Interrompe o processo imediatamente.

### Efeito 3: Interação (Windows)
- `io::stdin().read()` (no Windows) para pausar antes de fechar o terminal.

## 4. Estruturas de Dados
Apenas strings e manipulação local de PathBuf.
