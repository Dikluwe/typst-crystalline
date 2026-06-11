# Relatório P324 — Lote 9 (largura: SmartQuote, Stack, Cite, Transform, Place) + carona C1

**Pré-condição**: Lote 8 (P323) fechado — lint 0, suíte verde (typst-core 2615).
✅ Verificado.
**Commits**: `Passo 324 — carona de registro (C1)` (tree limpo, só modelo) e
`Passo 324 — lote 9`.

---

## Carona C1 — transformador × posições de padrão (confirmada)

O erro do transformador (converter posição de padrão em construtor) reincidiu
P321 → P323 apesar da nota. **Convertido em passo mecânico obrigatório** na Fase
B do modelo (`modelo-lote-migracao-d.md`): antes de rodar o transformador,
`grep` de `matches!`/struct-literal pelas variantes do lote, exclusão da passada
automática, regra "qualquer posição de padrão é manual". Diff: 14 linhas no
modelo, zero código. Commit `6949c08e0`.

**E reincidiu de novo neste lote** — prova de que a regra era necessária: o
transformador converteu 3 padrões `if let Value::Content(Content::{Stack,Cite}
{…})` em chamadas de construtor (E0164), corrigidos à mão. Registado abaixo.

---

## Lote 9 — largura crescente (5 variantes)

**Composição confirmada no checkpoint: 5 variantes**, as mais estreitas a partir
de `SmartQuote`, pulando o bloco grid/table cell e `Figure`(89):
`SmartQuote`(28) · `Stack`(30) · `Cite`(32) · `Transform`(32) · `Place`(34).
Soma largura-modelo **~156** (raw `grep Content::X`: 194). Maior lote por sites
até agora; excedente marginal sobre a faixa-guia (~110–150) aceite pelo dono —
registado para calibrar próximos.

### Forma de cada `…Elem`

| Variante | Campos | Locatável | `plain_text` | `is_empty` | `map_*` | Hash |
|----------|--------|-----------|--------------|------------|---------|------|
| `SmartQuoteElem` | `double` | não | `"`/`'` | default false | terminal (leaf, arm `\|`-combinado) | **derive** |
| `StackElem` | `children`, `dir`, `spacing` | não | join children | `all empty` | recurse children | **manual** (`Length`) |
| `CiteElem` | `key`, `supplement?`, `form?` | **SIM** (M1) | `[key]`+supplement | false | recurse supplement | **derive*** |
| `TransformElem` | `matrix`, `body` | não | `body` | **default false** | recurse body | **manual** (`TransformMatrix` 6×f64) |
| `PlaceElem` | `alignment`,`dx`,`dy`,`scope`,`float`,`clearance?`,`body` | não | `body` | **default false** | recurse body | **manual** (f64/`Align2D`) |

### Decisão do dono — `Cite` Hash (opção a, análogo-`Parity`)

`CiteElem` carrega `Option<CitationForm>`. `CitationForm` é `Copy + Eq` com 4
variantes unit (sem floats) → seguro para `Hash` por derive (regra do modelo).
**Ação (dependência do lote, não conserto oportunista)**: `Hash` adicionado ao
derive de `CitationForm` (`citation_form.rs`) + L0 `citation_form.md`
atualizado (o L0 **pina** os derives — diferença para `parity.md` do P320, que
não pinava). `CiteElem` deriva `Hash`.

### Achados content-preserving

- **`Transform`/`Place` `is_empty`**: caem em `_ => false` no hub — **não
  delegam** a `body.is_empty()`. Preservado (default, sem override). Precedente
  `Align` (Lote 7).
- **`SmartQuote`** fica no arm `|`-terminal combinado de `map_*` (leaf, sem
  binding) → `{ .. }` → `(_)`, **sem split**.
- **Construtor `cite`** já tomava `Option<Content>`; o `supplement` em
  `structural.rs` deixou de embrulhar em `Box`. `.into()` redundante removido
  num call-site (`cite` toma `impl Into<String>`; gerava E0283).

### Passo mecânico C1 — sites de padrão tratados à mão

O grep prévio (`matches!`/struct-literal pelas 5 variantes) listou os sites de
padrão. Os tratados manualmente (excluídos da passada automática ou corrigidos
após ela):

- **3 padrões mal-convertidos pelo transformador** (E0164, a reincidência):
  `stdlib/mod.rs` if-let `Content::Stack` (×2) e `Content::Cite` (×1) →
  revertidos para `Content::{Stack,Cite}(e)` + campos `e.`.
- **`matches!` com valor de campo**: `content.rs` SmartQuote (×2,
  `matches!(&sd, …(e) if e.double)`), `stdlib`/`layout` análogos.
- **Construções com comentário inline** (`float: true, // P223`): 6 `Place` em
  `layout/tests.rs` que o parser de campos do transformador rejeitou — manuais.
- **`&Content::X { … }`** (referência-a-construção): falso-positivo de padrão no
  transformador — corrigido.
- **Arms de produção** (`layout/mod.rs`, `introspect.rs`): destruturações →
  `(e)`/aliases `let f = &e.f;`, por mão.

---

## Verificação e medições (ADR-0104)

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2637 passed** (era 2615; **+22**), 0 failed. `typst-infra` 472,
  restantes verdes.
- **`content.rs`**: **5495 → 5419 linhas** (−76; o hub encolhe, como esperado —
  6 matches viram dispatch). Trajetória desde P313 (5782): −363 acumulado.
- **Parte atómica**: 5 módulos novos = **571 linhas** (`elements/{smartquote,
  stack,cite,transform,place}.rs`).
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.
- Ressalva conhecida: stack default estoura em `recursao_infinita_*` — não é
  regressão (`RUST_MIN_STACK=33554432`).

## Contabilidade (modelo atualizado)

- Migradas **47 → 52**; restantes element-shaped **~15 → ~10**.
- Conta de fecho: 52 + 4 `Set*` + 11 (DEBT-58) + 10 restantes = **77** ✓.

## Proposta do Lote 10 (decisão humana)

Restam **~10** element-shaped, dos quais o **bloco grid/table cell** (`TableCell`
32 · `GridCell` 47 · `Table` 41 · `Grid` 73, ~193 sites) é lote próprio
(penúltimo, sequência do dono) e `Figure`(89) é a mais larga. Fora do bloco e de
Figure sobram: `Pad`(39) · `Bibliography`(40) · `Equation`(45) · `Footnote`(46)
· `Shape`(57). Opções:

- **(A)** Lote 10 = as 5 fora-do-bloco-e-Figure (`Pad`/`Bibliography`/`Equation`/
  `Footnote`/`Shape`), deixando bloco grid/table + `Figure` para o fim.
- **(B)** Lote 10 = bloco grid/table cell (o lote pesado), adiando as avulsas.

Recomendação: **(A)** — mantém o ritmo por largura e isola o bloco pesado para um
lote dedicado, como já planeado. `Figure` fecha sozinha ou com as sobras.

## Fora de escopo (confirmado)

Lotes 10+, bloco grid/table cell (lote próprio), DEBT-58 (gatilho ainda não
disparou — restam element-shaped), F / `Set*` / 99.E, otimizações sugeridas por
medição (medir ≠ mexer).
