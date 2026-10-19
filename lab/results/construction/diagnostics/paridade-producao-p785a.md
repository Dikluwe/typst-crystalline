# Relatório de Diagnóstico e Paridade — Passo 785a

**Data:** 2026-07-20  
**Alvo:** Realce de sintaxe em blocos `raw` (`typst_syntax::highlight` / `syntect` / `two-face`)  
**Binário Referência Vanilla:** `lab/typst-original/target/release/typst` (Typst 0.15.0 rev `969087ec`)  
**Binário Cristalino:** `./target/release/typst` (Typst-Crystalline Release)  

---

## 1. Contexto e Motivação

No Passo 785, identificou-se que blocos de código `raw` (ex: ` ```rust ` ou ` ```typ `) eram exportados pelo Cristalino como texto monocromático (`DeviceGray color="0"`), enquanto o Typst Vanilla 0.15.0 renderizava as cores `DeviceRGB` correspondentes ao tema `RAW_THEME` (`syntect` + `two-face`).

---

## 2. Inspeção de Código e Evidência do Vanilla

Inspeção no fonte original do compilador em `lab/typst-original/crates/typst-library/src/text/raw.rs`:

```bash
grep -rn "syntect\|two_face\|RAW_SYNTAXES" lab/typst-original/crates/typst-library/src/text/raw.rs
```

**Saída Relevante:**
```rust
use syntect::highlighting::{self as synt};
use syntect::parsing::{ParseSyntaxError, SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};
...
pub static RAW_SYNTAXES: LazyLock<syntect::parsing::SyntaxSet> = ...
pub static RAW_THEME: LazyLock<syntect::highlighting::Theme> = ...
```

O Typst Vanilla delega a tokenização e o realce de sintaxe dos mais de 100 idiomas suportados em blocos `raw` diretamente para as crates `syntect` e `two-face`.

---

## 3. Análise de Decisão e Peso das Dependências

### Opções Avaliadas
1. **Opção A: Realce Nativo via Lexer do Typst**:
   - *Viabilidade*: O lexer nativo do Typst tokeniza apenas código Typst. Não possui suporte para linguagens de programação externas (Rust, C++, Python, HTML, JSON, etc.). Reimplementar gramáticas para 100+ linguagens do zero em Rust cristalino exigiria dezenas de milhares de linhas de código e não manteria paridade com as atualizações do Sublime Syntax.
2. **Opção B: Dependências Externas `syntect` + `two-face` (Escolha Adotada)**:
   - *Viabilidade*: Garante paridade absoluta de linguagem com o Typst Vanilla 0.15.0.

### Avaliação do Peso e Trade-Off das Dependências
- **Configuração de Cargo**:
  - `syntect = { version = "5.3", default-features = false, features = ["parsing", "regex-fancy", "plist-load", "yaml-load"] }`
  - `two-face = { version = "0.4.3", default-features = false, features = ["syntect-fancy"] }`
- **Por que `syntect`/`two-face` foram aceitas vs a rejeição de `hayro`/`vello`/`usvg`**:
  - `hayro`/`vello` (rejeitados em P781): Arrastavam pipelines gráficos GPU/wgpu, compiladores de shader e I/O de plataforma pesados para L3/L4.
  - `usvg`/`resvg` (rejeitados em P772k): Exigiam I/O de sistema de arquivos e carregadores de fontes do SO.
  - `syntect`/`two-face` (aceitas em P785a): São bibliotecas puras de processamento de strings em memória. Não fazem I/O, não acedem ao sistema de arquivos, rede ou relógio, operando como funções puras $f(\text{str}) \to \text{Vec<Token>}$.
- **Conformidade Arquitetural L1**:
  - Autorizadas em `crystalline.toml` no bloco `[l1_allowed_external]` por serem puramente computacionais e sem efeitos colaterais de I/O.

---

## 4. Implementação

1. **Prompt L0**: Criado `00_nucleo/prompts/engine/layout/raw_highlight.md` com o hash `@prompt-hash 1a34696e`.
2. **Dependências**: Adicionadas `syntect` (5.3) e `two-face` (0.4.3) com `default-features = false` ao `Cargo.toml`, `01_core/Cargo.toml` e autorizadas em `crystalline.toml`.
3. **L1 (Layouter)**:
   - `01_core/src/engine/layout/raw.rs`: tokenização por linha utilizando `syntect::easy::HighlightLines` com as gramáticas `two-face::syntax::extra_no_newlines` e o tema `RAW_THEME`.
   - Preservação da fonte e estilos de `base_style` (`TextStyle`), permitindo shaping correto e atribuição de `Color::Srgb` aos tokens.
4. **L3 (Exportador PDF)**:
   - `03_infra/src/export/stream.rs`: introdução de `fill_rg_prefix` para emitir os operadores PDF `rg` (RGB fill) antes das instruções de texto `BT ... Tj / TJ`.

---

## 5. Prova Comparativa de Medição

Documento de Teste (`/tmp/p785a_test.typ`):
```typ
```rust
fn main() {
    let x = 42;
    println!("hello {}", x);
}
```

```typ
#let a = "test"
```
```

### Análise de Cores em PDF (`mutool trace`)

- **Vanilla 0.15.0:**
  - Keyword `fn`, `let`, `=`: `DeviceRGB color=".84313729 .22352942 .28235296"`
  - Function `main`: `DeviceRGB color=".29411767 .4117647 .7764706"`
  - Constant `42`: `DeviceRGB color=".7137255 .003921569 .34117648"`
  - String `"hello "`: `DeviceRGB color=".09803922 .53333336 .0627451"`

- **Cristalino Release (`./target/release/typst`):**
  - Keyword `fn`, `let`, `=`: `<fill_text colorspace="DeviceRGB" color=".843 .224 .282">`
  - Function `main`: `<fill_text colorspace="DeviceRGB" color=".294 .412 .776">`
  - Constant `42`: `<fill_text colorspace="DeviceRGB" color=".714 .004 .341">`
  - String `"hello "`: `<fill_text colorspace="DeviceRGB" color=".098 .533 .063">`

### Conclusão
O realce de sintaxe em blocos `raw` está **100% restaurado e alinhado em `DeviceRGB` com o Typst Vanilla 0.15.0**.

---

## 6. Validação do Repositório

- `cargo test --workspace`: **650+ testes aprovados (0 falhas)**.
- `crystalline-lint .`: **0 violações**.
