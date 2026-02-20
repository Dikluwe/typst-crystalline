# Spec: `fonts.rs` (Descoberta e Listagem de Fontes)

## 1. Objetivo Central
O arquivo `fonts.rs` implementa o comando `typst fonts`, que descobre todas as fontes disponíveis (sistema, embutidas e em caminhos específicos) e as lista no terminal, opcionalmente exibindo detalhes das variantes (estilo, peso, stretch, path).

## 2. Atomização da Lógica Pura (Para L1)

### Formatação de Detalhes da Fonte
- **`format_font_variant(style, weight, stretch, path) -> String`**: Produz a string formatada com os detalhes de uma variante de fonte. Lógica 100% pura: `- Style: {style:?}, Weight: {weight:?}, Stretch: {stretch:?}, Path: {path}`.

> Nota: A maior parte do algoritmo é I/O (iteração sobre resultados do filesystem e impressão iterativa), então a L1 se resume à formatação.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Impressão no Terminal
- `println!("{family}")` e impressão das variantes. O output é para stdout de forma síncrona.

### Efeito 2: Leitura de Sistema de Arquivos / Fontes do Sistema
- `typst_kit::fonts::system()` — Carrega fontes instaladas no SO.
- `typst_kit::fonts::scan(&path)` — Varre diretórios buscando fontes.
- `typst_kit::fonts::embedded()` — Carrega fontes embutidas no binário (I/O embutido em memória).

## 4. Estruturas de Dados Chave
- Usa `typst_kit::fonts::FontStore` (agrega os pacotes de fontes descobertos).
- `FontArgs` (argumentos da CLI para suprimir system/embedded ou incluir caminhos).
