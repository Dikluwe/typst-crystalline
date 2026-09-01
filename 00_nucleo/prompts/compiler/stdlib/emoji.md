# Prompt L0 — `emoji` — módulo de emoji Unicode
Hash do Código: c4ecf7d3

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/emoji.rs`
**Origem**: Passo 735 — namespace `emoji` ausente no cristalino ("unknown variable"); vanilla expõe como `module` (medido em P731/P735). **P1283**: catálogo integral do vanilla ratificado a partir de `codex = 0.3.0`.
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1), ADR-0129 (ownership 1:1).

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

Com a representação `EcoString` ratificada em `entities/symbol.md`, P1162 acrescentou
`heart` como primeiro grupo complexo. P1283 fecha o scope-out histórico.

**Medição P1283 antes da decisão (2026-08-30, vanilla `a51e02804`):** o workspace
vanilla usa `codex = 0.3.0`, checksum crates.io
`0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`. A fonte
`codex-0.3.0/src/modules/emoji.txt:1-1472`, SHA-256
`8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`, declara
772 símbolos, 1.386 registos de valor/variante e 86 parents sem bare. São contagens
de inventário, não percentagem de paridade.

## 2. Tabela

P1283 substitui as tabelas manuais por um adaptador puro sobre `codex::EMOJI`, pinado
exactamente em `0.3.0`. Entradas simples usam `Symbol::new`; grupos usam
`Symbol::with_variants`. A sequência Unicode integral é morfologia da linguagem:
VS15/U+FE0E, VS16/U+FE0F e ZWJ não podem ser truncados nem normalizados.

Parents sem bare usam o mesmo best-match vanilla: maior coincidência, menos modifiers
extra e primeira variante em empate. Exemplos focais: `emoji.apple` resolve para 🍏 e
`emoji.arrow` para ↙️; escolher simplesmente a primeira variante da fonte é incorreto
quando ela tem mais modifiers.

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
- `emoji.apple` → 🍏 e preserva as variants `green`/`red`.
- `emoji.arrow` → ↙️ pelo algoritmo de parent sem bare.
- Todos os 772 símbolos e 1.386 registos de valor/variant do pin resolvem com o mesmo
  kind, valor, `repr` e modifier do vanilla.
- Amostra de entradas da tabela (`emoji.ant` → 🐜, `emoji.banana` → 🍌) renderiza.
- `cargo test --workspace` verde; `crystalline-lint .` limpo.
