# Prompt L0 — `emoji` — módulo de emoji Unicode
Hash do Código: f4a84c9e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/emoji.rs`, `01_core/src/compiler/eval/mod.rs`
**Origem**: Passo 735 — namespace `emoji` ausente no cristalino ("unknown variable"); vanilla expõe como `module` (medido em P731/P735).
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1).

---

## 1. Contexto e medições

O vanilla expõe `emoji` como módulo (`type(emoji)` → `module`, medido), acessível via `emoji.face` → 😀 (medido). A fonte do vanilla é o crate `codex-0.2.0` (`src/modules/emoji.txt`): **766 entradas top-level** (1464 linhas com variantes indentadas), das quais **529 têm valor de 1 codepoint** (medido — `len == 1` e sem `\vs{emoji}`, sem nomes duplicados).

Decisão histórica: P735 implementou as 529 entradas de um codepoint mais
`face` → 😀 — 530 entradas — porque a entidade então se limitava a `char`.

**P1161 — medição anterior à decisão.** No vanilla `a51e02804`,
`type(emoji.heart)` é `symbol`; o valor base é `❤️` (`U+2764 U+FE0F`) e o
`repr` contém base mais 22 variants: `arrow`, `beat`, `black`, `blue`, `box`,
`broken`, `brown`, `double`, `excl`, `gray`, `green`, `grow`, `lightblue`,
`orange`, `pink`, `purple`, `real`, `revolve`, `ribbon`, `spark`, `white` e
`yellow`. `excl` também contém VS16. Field inválido produz `unknown symbol
modifier` no span do modifier.

Com a representação `EcoString` ratificanda em `entities/symbol.md`, P1162
acrescenta `heart` como grupo complexo sem reabrir toda a tabela multi-codepoint.
Continuam scope-out para passos medidos posteriores:

- restantes entradas multi-codepoint (`\vs{emoji}`, sequências) — inventário
  histórico aproximado de 236, a recontar contra o pin antes de fechamento;
- restantes variantes com modificadores (`face.halo`, `airplane.landing`);
- Pais sem valor bare (exceto `face`, medido).

## 2. Tabela

```rust
// 01_core/src/compiler/stdlib/emoji.rs
/// Gerada de `codex-0.2.0/src/modules/emoji.txt` — entradas top-level com
/// valor de 1 codepoint (529 medidas) + `face` (default medido). Ordem do ficheiro.
pub static EMOJI_TABLE: &[(&str, &str)] = &[
    ("abacus", '🧮'), ("abc", '🔤'), // ... 529 entradas ...
    ("face", '😀'),
];

pub static EMOJI_GROUPS: &[(&str, &str, fn() -> Vec<SymbolVariant>)] = &[
    ("heart", "❤️", heart_variants),
];
```

Todas as entradas simples passam a usar strings uniformemente; converter a
tabela completa de `char` para `&str` é mecânica necessária ao novo contrato,
sem mudança morfológica para as 530 entradas existentes. `heart_variants`
preserva exactamente os clusters medidos, inclusive VS16.

## 3. Funções

```rust
/// Constrói o Value::Module `emoji` — todas as entradas têm nome simples
/// (sem `.`), logo todas ficam no scope (acessíveis via FieldAccess).
pub fn build_emoji_module() -> Value;
```

## 4. Registo no scope

```rust
// 01_core/src/compiler/eval/mod.rs
scope.define("emoji", build_emoji_module());
```

## 5. Eval markup

`Value::Symbol` renderiza via o valor `EcoString` integral. O módulo cria
entradas simples com `Symbol::new` e grupos com `Symbol::with_variants`.

## 6. Critérios de verificação

- `type(emoji)` → `module`.
- `emoji.face` → 😀 (o caso medido do vanilla).
- `emoji.heart` → ❤️, preservando `U+2764 U+FE0F`.
- `emoji.heart.arrow` → 💘; `emoji.heart.excl` → ❣️.
- `repr(emoji.heart)` lista a base e as 22 variants medidas.
- Amostra de entradas da tabela (`emoji.ant` → 🐜, `emoji.banana` → 🍌) renderiza.
- `cargo test --workspace` verde; `crystalline-lint .` limpo.
