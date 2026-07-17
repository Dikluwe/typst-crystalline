# Paridade Produção — P689 — `str.codepoints()`, `str.position()`, `str.match()`

**Estado:** fechado (os três métodos implementados com paridade de valor confirmada;
o bloqueio de `codepoints` em `oxifmt` desapareceu; próximo bloqueio medido é `sys`,
fora de escopo).

**Commit do trabalho:** `6fff77ef42f9b8632c71800bbb5ad2d9ce75ce1b`
**Commit base (HEAD antes deste passo):** `1b23fd450c4e1d58ab613944b32721a1a525ca5b`
**Hora da medição:** `2026-07-10T17:59:17-03:00` (saída de `date -Is`)
**Árvore:** detached HEAD; working tree com apenas 4 ficheiros tracked alterados
(untracked pré-existentes em `materialization/`, `adr/`, `diagnosticos/`, `temp_p*`
não foram tocados).

---

## Proveniência da medição (regra de P569/P574)

- **Código medido:** este commit `6fff77ef42f9b8632c71800bbb5ad2d9ce75ce1b`, sobre a base `1b23fd450`.
- `git diff HEAD --stat` no momento da medição:

```
 00_nucleo/prompts/entities/regex.md           |  16 ++-
 00_nucleo/prompts/engine/stdlib/collections.md |  12 ++
 01_core/src/entities/regex.rs                 |  29 ++++-
 01_core/src/engine/stdlib/collections.rs       | 171 ++++++++++++++++++++++++++
 4 files changed, 226 insertions(+), 2 deletions(-)
```

- `cargo test --workspace`: **4363 passed, 0 failed** (3694 + 610 + 28 + 2 + 27 + 2;
  3 doc-tests ignorados). P688 tinha 4359; **+4** = os 4 testes novos deste passo.
- `crystalline-lint .`: **0 violations**.
- Binários: cristalino `target/debug/typst`; vanilla
  `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)` (`compile`).

---

## Sonda (medida antes de decidir — ADR-0108)

Confirmado contra o vanilla (`#repr`):

| método | caso | vanilla |
|--------|------|---------|
| `codepoints()` | `("abc")` | `("a","b","c")` |
| | `("café")` | `("c","a","f","é")` (chars/scalars) |
| | `("")` | `()` |
| `position(hay)` | `("abc").position("b")` | `1` |
| | `("abc").position("z")` | `none` |
| | `("abc").position(regex("b"))` | `1` (aceita regex) |
| | `("xéy").position("y")` | `3` (**byte**, x=1+é=2) |
| `match(re)` | `("abc").match(regex("b"))` | `(start:1, end:2, text:"b", captures:())` |
| | `("abc").match(regex("z"))` | `none` |
| | `("abc").match(regex("(a)(b)(c)"))` | `captures: ("a","b","c")` |
| | `("xéy").match(regex("é"))` | `(start:1, end:3)` (**bytes**) |
| | `(…).match(regex("(?<y>\\d{4})-…"))` | `captures: ("2024","01","02")` (nomeados → ordem) |

**Decisões medidas:**
- `codepoints` = itera **chars** (scalar values); coincide com o `clusters`
  simplificado do cristalino.
- `position` e `match` devolvem índices em **bytes** (confirmado por multibyte) — isto
  é semântica da linguagem (ADR-0107) e difere de `str.at`/`str.slice` do cristalino
  (que indexam por **char**). Divergência interna pré-existente; ver Débitos.
- `match` → `dict {start, end, text, captures}` ou `none`; `position` → `int | none`;
  `position` aceita `str` **ou** `regex`; `match` aceita só `regex`.

---

## O que foi feito

**L0 (Trava Arquitetural — antes de código):**
- `00_nucleo/prompts/entities/regex.md` — adicionados `RegexMatch` e
  `Regex::captures_first` (índices em bytes); ajustado o scope-out (P689 cobre
  `match`/`position`; `replace`/flags continuam fora).
- `00_nucleo/prompts/engine/stdlib/collections.md` — 3 linhas na tabela de `str` +
  nota P689 (bytes vs chars).
- `crystalline-lint --fix-hashes .` → `01_core/src/entities/regex.rs` `@prompt-hash`
  actualizado para `2d267947`. (`collections.rs` não tem linha `@prompt-hash` —
  pré-existente, tolerado pelo lint.)

**Código:**
- `01_core/src/entities/regex.rs` — `struct RegexMatch { start, end, text, captures }`
  e `Regex::captures_first(&self, text) -> Option<RegexMatch>` (delega a
  `regex::Regex::captures`; grupos não-participantes → `""`; nomeados em ordem).
- `01_core/src/engine/stdlib/collections.rs` — 3 braços no dispatcher de `str` e os
  helpers `str_codepoints`, `str_position` (str **ou** regex), `str_match` (regex →
  `Value::Dict` com ordem de campos `start,end,text,captures`).

**Testes (`#[cfg(test)]` em `collections.rs`, +4):**
- `p689_str_codepoints_chars` — ascii + `café` (4) + vazio.
- `p689_str_position_str_e_regex_byte_indices` — str/regex, encontrado/none, byte 3 em `xéy`.
- `p689_str_match_dict_e_captures` — none, dict simples, byte 1–3 em `xéy`, captures `(a)(b)(c)`.
- `p689_str_position_match_tipo_errado` — `position(Int)` e `match(Str)` → erro.

---

## Validação cristalino vs vanilla

**Documento de 6+ expressões (`metodos.typ`) — ambos exit 0.** Os **valores** são
idênticos; a única diferença é a **formatação** do `repr` de dict multi-campo: o
vanilla pretty-printa em várias linhas com vírgula final
`( start: 0, end: 3, …, captures: (…), )`, enquanto o cristalino imprime inline
`(start: 0, end: 3, …, captures: (…))`. `start/end/text/captures` e a ordem dos campos
coincidem → diferença **mecânica** (pretty-printer), ADR-0107; não afecta o valor.

**Import de `oxifmt` (via cetz) — avança para além de `codepoints`:**
- Antes (P688): `field access não suportado em str` em `oxifmt.typ:12` (`.codepoints()`).
- Depois (P689): o erro de `codepoints` **desapareceu**. `#import "@preview/oxifmt:1.0.0"`
  agora falha em `unknown variable: sys` — `oxifmt.typ:13-14` usa `sys.version >= version(0,11,0)`.
  `sys` é um módulo builtin do vanilla (`sys.version == version(0,15,0)`), ausente no
  scope global do cristalino. É o **próximo bloqueio real**, independente de `codepoints`.

**`cetz` 0.5.2 (documento correcto de P688):** mesmo bloqueio `sys` (cetz importa
oxifmt). Ainda sem PDF — esperado: `sys` e, mais à frente (inferência de P688 ainda
não confirmada), o plugin WASM `cetz_core.wasm`. Registado com honestidade.

---

## Débitos (fora de escopo)

- `sys` (módulo builtin: `sys.version`, `sys.platform`, …) — ausente no scope global;
  bloqueia oxifmt/cetz no import. Passo próprio.
- Plugin WASM (`cetz_core.wasm`) — provável bloqueio posterior de cetz 0.5.2; só se
  confirma após `sys` (inferência, refutável).
- Formatação do `repr` de dict (inline vs multiline + vírgula final) — mecânica; sem
  impacto de valor.
- `str.len()` do cristalino conta **chars** (`"café".len()==4`), enquanto o vanilla
  conta **bytes** (`==5`) — divergência pré-existente descoberta nesta sonda;
  `str.at`/`str.slice` indexam por char enquanto `position`/`match` (agora) indexam
  por byte. Não introduzida por P689; unificar a convenção de índices é trabalho
  separado.

## Conclusão

`str.codepoints()`, `str.position()` e `str.match()` estão implementados com **paridade
de valor** confirmada contra o vanilla (ao nível da língua, ADR-0107), medidos com
proveniência (ADR-0108). O bloqueio de `codepoints` em `oxifmt` (P688) está removido;
o próximo bloqueio medido (`sys`) é independente e fica registado.
