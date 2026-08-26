# Prompt L0 — `entities/counter_format`
Hash do Código: 25914603

**Camada**: L1 · **Alvo**: `01_core/src/entities/counter_format.rs`
**Criado em**: 2026-06-24 (P451 — heading numbering patterns)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0109 (counters hierárquicos)

---

## Contexto

`CounterRegistry` fornece valores hierárquicos de counters (ex.: `[1, 2, 1]` para headings). Este módulo aplica **patterns de formatação** a esses valores, permitindo renderizar prefixos como `"1."`, `"I."`, `"(a)"` ou `"A."`.

---

## Interface pública

```rust
/// Formata um vector de valores hierárquicos segundo um pattern.
pub fn format_counter(values: &[usize], pattern: &str) -> Option<String>;
```

## Semântica

- Cada caractere do pattern que for um **token** (`1`, `I`, `a`, `A`) é substituído pela representação do valor do nível correspondente.
- Caracteres não-token são copiados literalmente (prefixos, sufixos, separadores).

> **Fonte de paridade (P1031)** — doc comment `#[func]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/model/numbering.rs`, publicado em
> `typst.app/docs/reference/model/numbering/`:
>
> - **Estrutura do pattern** — `numbering.rs:19-21`: *"A numbering pattern consists of
>   counting symbols, for which the actual number is substituted, their prefixes, and one
>   suffix. The prefixes and the suffix are displayed as-is."* ✅ sustenta as duas regras
>   acima.
> - **Prefixo/sufixo definidos** — `numbering.rs:70-75`: *"**Suffixes** are all characters
>   after the last counting symbol. They are displayed as-is at the end of any rendered
>   number. **Prefixes** are all characters that are neither counting symbols nor suffixes."*
>   Nota: o vanilla distingue **um único sufixo** (tudo o que vem depois do último símbolo)
>   de prefixos por símbolo — distinção mais forte do que o "copiados literalmente" acima.
> - **Repetição do último símbolo** — `numbering.rs:89-90`: *"If `numbering` is a pattern and
>   more numbers than counting symbols are given, the last counting symbol with its prefix is
>   repeated."* Regra que este L0 não regista — **lacuna documentada**.
> - **Contagem a partir de um** — `numbering.rs:84-87`: *"In general, numbers are counted from
>   one. A number of zero indicates that the first element has not yet appeared."*
>
> **A lista de tokens do cristalino é um subconjunto declarado.** O vanilla enumera 24
> símbolos de contagem (`numbering.rs:61-64`, citação literal): `1`, `a`, `A`, `i`, `I`, `α`,
> `Α`, `一`, `壹`, `あ`, `い`, `ア`, `イ`, `א`, `가`, `ㄱ`, `*`, `١`, `۱`, `१`, `১`, `ক`, `①`,
> `⓵` — *"They are replaced by the number in the sequence, preserving the original case."* —
> e documenta ainda que `*` conta pela sequência `*`, `†`, `‡`, `§`, `¶`, `‖`, com repetição
> acima de seis (`numbering.rs:66-68`). O cristalino suporta quatro (`1`, `I`, `a`, `A`).
> Os quatro estão correctos; os restantes vinte são **lacuna documentada**, não afirmação
> incorrecta.
- Tokens são consumidos em ordem: o primeiro token usa `values[0]`, o segundo `values[1]`, etc.
- Se `values` estiver vazio, ou se o pattern não contiver nenhum token reconhecido, retorna `None`.
- **Sufixo único** — tudo o que vem depois do **último** token é o sufixo; é emitido uma só
  vez, no fim. Os restantes literais são **prefixos** do token que os segue.
- **Mais `values` do que tokens** (P1036): o último token **com o seu prefixo** repete-se
  por cada valor excedente. `[1,1,1]` + `"1."` → `"1.1.1."`.
- **Mais tokens do que `values`** (P1036): a formatação **pára** no último valor
  disponível; os tokens excedentes (e os seus prefixos) são descartados, e o sufixo é
  emitido a seguir. `[1]` + `"1.1"` → `"1"`. **Não** retorna `None`.

## Tokens suportados

| Token | Significado | Exemplos |
|-------|-------------|----------|
| `1` | Algarismos arábicos | `1`, `2`, `10` |
| `I` | Romanos maiúsculos (até 3999; fallback arábico fora do range) | `I`, `IV`, `XLIX` |
| `a` | Letras minúsculas (a=1, ..., z=26, aa=27, ...) | `a`, `z`, `aa` |
| `A` | Letras maiúsculas | `A`, `Z`, `AA` |

## Exemplos

```
[1] + "1."      → "1."
[1, 2] + "1.1"  → "1.2"
[4] + "I."      → "IV."
[2] + "(a)"     → "(b)"
[3] + "A."      → "C."
[2, 3] + "I.1"  → "II.3"
[1, 1, 1] + "1."     → "1.1.1."      (P1036 — repetição do último token)
[1] + "1.1"          → "1"           (P1036 — tokens excedentes descartados)
[1, 1, 1, 1] + "A.1.a" → "A.1.a.a"   (P1036 — repete `.a`, o último token com prefixo)
```

## Scope-outs

- Círculos numerados (`"①"`), kanji (`"一"`), etc.
- Patterns com repetição/posicionamento não-linear dos tokens.

## Tests obrigatórios

- Pattern `"1."` com nível 1 e 2.
- Pattern `"1.1"` hierárquico.
- Pattern `"I."` para romanos.
- Pattern `"(a)"` para letras minúsculas (incluindo wrap `aa`).
- Pattern `"A."` para letras maiúsculas.
- Valores em excesso repetem o último token com o seu prefixo (P1036).
- Tokens em excesso são descartados, com o sufixo preservado (P1036).

---

## Resultado esperado

- `01_core/src/entities/counter_format.rs` — função pura + tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## P1036 — a lacuna documentada em P1031, fechada

**Data:** 2026-08-13 · **Proveniência:** `HEAD = 0c8b64a41` (P1033), árvore de trabalho
com edições só em `00_nucleo/prompts/**`; vanilla `/usr/local/bin/typst`
(md5 `36da18895eeb5e0136c068a7634e3f82`, idêntico a
`lab/typst-original/target/release/typst`, baseline ratificado `a51e02804`);
cristalino `target/release/typst` reconstruído de `0c8b64a41`. Medições 17:30–17:45 -03:00.

A regra que a secção "Fonte de paridade (P1031)" acima já registava como **lacuna
documentada** — *"If `numbering` is a pattern and more numbers than counting symbols are
given, the last counting symbol with its prefix is repeated"* (`numbering.rs:89-90`) —
não estava implementada. Consequência medida (`#set heading(numbering: …)`, corpo do
documento, níveis 1..5):

| pattern | vanilla | cristalino (antes) |
|---|---|---|
| `1.` | `1.` / `1.1.` / `1.1.1.` / `1.1.1.1.` | `1.` / `1.` / `1.` / `1.` |
| `I.` | `I.` / `I.I.` / `I.I.I.` | `I.` / `I.` / `I.` |
| `1.1` | `1` / `1.1` / `1.1.1` | `1.` / `1.1` / `1.1` |
| `A.1.a` | `A` / `A.1` / `A.1.a` / `A.1.a.a` / `A.1.a.a.a` | `1.` / `1.1.` / `A.1.a` / … |

A causa é dupla e ambas as metades vivem nesta função: (i) o loop percorria o **pattern**
e não os **valores**, logo um pattern de um token consumia um só valor; (ii) `values.get(level)?`
devolvia `None` quando o pattern tinha mais tokens do que valores, empurrando o caller para
um fallback com formatação diferente.

**Débito registado, não fechado aqui:** existe uma segunda implementação da mesma regra da
linguagem — `format_pattern` (`compiler/stdlib/numbering.rs`, `pub(crate)`), que já estava
correcta (medido: `#numbering("1.", 1, 2, 3)` → `1.2.3.` nos dois binários) e que suporta
mais tokens (`i`, `א`, `①`). As duas passam a concordar na semântica, mas continuam a ser
dois corpos de código. Unificar exige mover `format_pattern` para esta camada e resolver a
sua dependência de `Engine` (usada só para os warnings de `א`/`①`) — **passo próprio**,
não tentado aqui.
