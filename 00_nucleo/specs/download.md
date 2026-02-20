# Spec: `download.rs` (Fábrica de Downloaders com Progresso)

## 1. Objetivo Central
O arquivo `download.rs` cria a fábrica de downloaders HTTP do Typst. Constrói um `ProgressDownloader` que envolve um `SystemDownloader` com um reporter de progresso que exibe status colorido no terminal.

## 2. Atomização da Lógica Pura (Para L1)

### Construção do User-Agent
- **`build_user_agent(version: &str) -> String`**: Formata `typst/{version}` como header HTTP.

### Classificação de Download
- **`classify_download_key(key) -> Option<String>`**: Determina o rótulo de exibição baseado no tipo de recurso: `PackageSpec` → nome do pacote, `"release"` → "release", outro → `None`.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: HTTP Download
- `SystemDownloader::new()` / `::with_cert_path()` — Criação do downloader com ou sem certificado customizado.
- `ProgressDownloader::new()` — Envolvimento com reporter de progresso.

### Efeito 2: Terminal (Progresso Colorido)
- `terminal::out()` — Handle de saída com cores.
- `out.set_color()` / `out.reset()` — Cores para "downloading".
- `out.clear_last_line()` — Atualização in-place da barra de progresso.

### Efeito 3: Estado Global
- `ARGS.cert` — Leitura do certificado customizado a partir dos argumentos globais.

## 4. Estruturas de Dados
- `PrintProgress(Option<EcoString>)` — Reporter de progresso com nome opcional.
