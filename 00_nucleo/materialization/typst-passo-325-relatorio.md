# Relatório P325 — Lote 10 (largura: Pad, Bibliography, Equation) + carona C1-bis

**Pré-condição**: Lote 9 (P324) fechado — lint 0, suíte verde (typst-core 2637).
✅ Verificado.
**Commits**: `Passo 325 — carona de registro (C1-bis)` (tree limpo, só modelo) e
`Passo 325 — lote 10`.

---

## Carona C1-bis — o elo que falhou (confirmada)

**Diagnóstico (qual elo):** **(ii)**. No P324, o grep do passo 1 *capturou* os
padrões aninhados `Value::Content(Content::{Stack,Cite} {…})` (verificado no
output do P324); a exclusão do passo 2 confiou na heurística interna do
transformador, que os converteu na mesma (E0164). O elo (i) — o grep — funcionou.

**Correção (só o elo ii):** a exclusão deixa de ser promessa e vira
**verificável** — após a passada, intersetar os sites tocados com a lista do
grep; interseção não-vazia ⇒ parar e reverter **antes de compilar**. Gravado no
modelo (Fase B, passo mecânico, item 4). Commit `86fe9f5a1`; zero código.

**A C1-bis estreou e apanhou a 4ª reincidência** (P321/P323/P324/P325). Neste
lote, o transformador de construções voltou a converter 2 padrões aninhados
(`if let Value::Content(Content::pad(…))` e `…bibliography(…)`). A verificação
pós-passada (grep de construtor-em-posição-de-padrão) **apanhou-os antes de
compilar**; revertidos à mão. A regra cumpriu o objetivo: tornar o erro
*apanhado*, não *enviado*. **Honestidade epistémica**: a heurística do
transformador continua o elo fraco — a 4ª reincidência diz que a prevenção real
seria excluir por **linha-do-grep** (não por heurística). Candidato a **C1-ter**.

---

## Lote 10 — largura crescente (3 variantes)

**Composição confirmada: `Pad`(39) · `Bibliography`(40) · `Equation`(45)** =
~124 sites largura-modelo (132 raw) — dentro da faixa-guia.

### Forma de cada `…Elem`

| Variante | Campos | Locatável | `is_empty` | `map_content`/`map_text` | Hash |
|----------|--------|-----------|------------|--------------------------|------|
| `PadElem` | `body`, `sides` | não | `body.is_empty()` | recurse / recurse | **manual** (`Sides<Length>`/f64) |
| `BibliographyElem` | `entries`, `title?` | **SIM** (P181C) | `entries.empty && title.none` | recurse title / recurse title | **derive** (`BibEntry: Eq+Hash`) |
| `EquationElem` | `body`, `block` | **SIM** (P186B) | **default false** | **recurse / TERMINAL** | **derive** (`Content`+bool) |

### Locatabilidade (o achado do lote)

**2 de 3 locatáveis** — `Bibliography` (P181C) e `Equation` (P186B). Ambas
absorvem `element_kind`/`to_payload` no trait (precedente Heading/Lote 6/Cite-L9);
`extract_payload.rs` passou a `Content::{Bibliography,Equation}(e) =>
e.to_payload()`; `locatable.rs` a `(_)  => true`. O consumo por
`ElementPayload::{Bibliography,Equation}` (`from_tags`→`BibStore` / gate
`block && numbering_active:equation`) é **inalterado**. `Pad` é não-locatável.

### Achado — `Equation` assimétrico (inédito no roteiro)

`Equation` é o **primeiro contentor com `map_content ≠ map_text`**: `map_content`
recursa no body (arm próprio no hub), mas `map_text` é **terminal** (math
structural não desce em texto — `Equation` fica no arm `|`-combinado de
`map_text`). O `EquationElem` materializa ambos (`map_text` devolve `self.clone()`
por contrato); o hub mantém `Equation(_)` no `|`-terminal de `map_text` (sem
split). `Equation`/`Transform`/`Place` `is_empty` no default `false`.

### Hash — sem dependência este lote

`BibEntry` já deriva `Eq + Hash` (`bib_entry.rs:80`) → `Bibliography` deriva.
`Equation` deriva (`Content` tem `impl Hash` manual; `bool`). `Pad` é manual por
`Length`. Nenhum análogo-`Parity`/`CitationForm`.

### C1+C1-bis — sites de padrão tratados à mão

Grep prévio: **96 sites** das 3 variantes registados. Tratados manualmente:

- **2 padrões mal-convertidos pelo transformador** (a 4ª reincidência, apanhada
  pela verificação C1-bis): `stdlib/mod.rs` `if let Value::Content(Content::Pad
  (…))` e `…Bibliography(…)` → revertidos para `(e)` + campos `e.`.
- **Construções `&Content::Equation { … }`** (7 em `export/tests.rs`):
  falso-positivo de padrão no transformador (`&`-prefix) — convertidas à mão.
- **Match-arms** `Content::Equation { body, .. } => f(body)` (5 em
  `eval/tests.rs`): o transformador de padrões não cobre arms — manuais.
- **Arms de produção** (`layout/mod.rs`, `introspect.rs`): destruturações →
  `(e)`/aliases por mão (incl. colisão `for e in entries` → bind `b`).

---

## Verificação e medições (ADR-0104)

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2653 passed** (era 2637; **+16**), 0 failed. `typst-infra` 472,
  restantes verdes.
- **`content.rs`**: **5419 → 5385 linhas** (−34). Trajetória desde P313 (5782):
  −397 acumulado.
- **Parte atómica**: 3 módulos novos = **384 linhas** (`elements/{pad,
  bibliography,equation}.rs`).
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.
- Ressalva conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Contabilidade (modelo atualizado)

- Migradas **52 → 55**; restantes element-shaped **~10 → ~7**.
- Conta de fecho: 55 + 4 `Set*` + 11 (DEBT-58) + 7 restantes = **77** ✓.

## Proposta do Lote 11 (decisão humana)

Restam **~7**: o **bloco grid/table cell** (`TableCell`32 · `GridCell`47 ·
`Table`41 · `Grid`73, ~193 sites) é lote próprio (L12, penúltimo) e `Figure`(89)
é a mais larga (L13, fecha os element-shaped). Fora do bloco e de Figure sobram
**`Footnote`(46) · `Shape`(57)** (~103 sites).

- **Lote 11 = `Footnote` · `Shape`** (~103, dentro da faixa) — confirmado contra
  o mapa; mantém o ritmo por largura. Depois: L12 bloco grid/table cell, L13
  `Figure` (dispara o gatilho do DEBT-58).

## Fora de escopo (confirmado)

Lotes 11+ (`Footnote`/`Shape`), bloco grid/table cell (L12), `Figure` (L13),
DEBT-58 (gatilho no fim dos element-shaped — ainda não), F / `Set*` / 99.E,
otimizações sugeridas por medição (medir ≠ mexer).
