# Prompt L0 — `emoji` — módulo de emoji Unicode
Hash do Código: ddc90bb5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/emoji.rs`, `01_core/src/rules/eval/mod.rs`
**Origem**: Passo 735 — namespace `emoji` ausente no cristalino ("unknown variable"); vanilla expõe como `module` (medido em P731/P735).
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1).

---

## 1. Contexto e medições

O vanilla expõe `emoji` como módulo (`type(emoji)` → `module`, medido), acessível via `emoji.face` → 😀 (medido). A fonte do vanilla é o crate `codex-0.2.0` (`src/modules/emoji.txt`): **766 entradas top-level** (1464 linhas com variantes indentadas), das quais **529 têm valor de 1 codepoint** (medido — `len == 1` e sem `\vs{emoji}`, sem nomes duplicados).

Decisão registada (passo permite faseado, com razão medida): implementar neste passo **todas as 529 entradas de 1 codepoint** (critério principiado — tudo o que é representável na entidade actual `Symbol { ch: char }`), mais `face` → `'😀'` (default medido: `#emoji.face` → 😀; no codex `face` é pai sem valor bare e `face.grin` = 😀 é a primeira variante) — **530 entradas no módulo**. Fica scope-out (razão: a entidade `Symbol` é `char` único — extensão registada em `achados-adiados-cetz.md`):

- Entradas multi-codepoint (`\vs{emoji}`, sequências) — ~236 top-level.
- Variantes com modificadores (`face.halo`, `airplane.landing`) — mesmo mecanismo scope-out de `sym` (P471: sem modificadores encadeados).
- Pais sem valor bare (exceto `face`, medido).

## 2. Tabela

```rust
// 01_core/src/rules/stdlib/emoji.rs
/// Gerada de `codex-0.2.0/src/modules/emoji.txt` — entradas top-level com
/// valor de 1 codepoint (529 medidas) + `face` (default medido). Ordem do ficheiro.
pub static EMOJI_TABLE: &[(&str, char)] = &[
    ("abacus", '🧮'), ("abc", '🔤'), // ... 529 entradas ...
    ("face", '😀'),
];
```

## 3. Funções

```rust
/// Constrói o Value::Module `emoji` — todas as entradas têm nome simples
/// (sem `.`), logo todas ficam no scope (acessíveis via FieldAccess).
pub fn build_emoji_module() -> Value;
```

## 4. Registo no scope

```rust
// 01_core/src/rules/eval/mod.rs
scope.define("emoji", build_emoji_module());
```

## 5. Eval markup

Sem alteração: `Value::Symbol` já renderiza via o braço de P471 em `eval_markup` (`Content::Text` com o char).

## 6. Critérios de verificação

- `type(emoji)` → `module`.
- `emoji.face` → 😀 (o caso medido do vanilla).
- Amostra de entradas da tabela (`emoji.ant` → 🐜, `emoji.banana` → 🍌) renderiza.
- `cargo test --workspace` verde; `crystalline-lint .` limpo.
