# Relatório P328 — Lote 13 (`Figure`, o último) + carona C2 + gatilho DEBT-58

**Pré-condição**: Lote 12 (P327) fechado — lint 0, suíte verde (typst-core 2678).
✅ Verificado.
**Commits**: `Passo 328 — carona de registro` (C2, tree limpo, só modelo) e
`Passo 328 — lote 13`.

---

## Carona C2 — Arc-wrap no passo mecânico (gravada agora)

O achado Arc-wrap do P327 só estava na Contabilidade (história), **não** no
passo mecânico (a regra). Como o transformador é efémero (script `/tmp` por
lote), a adaptação perde-se se não morar no modelo. Gravado como **item 7** do
passo mecânico: o transformador emite `Content::x(…)` **só** quando o construtor
cobre todos os campos; senão, `Content::X(Arc::new(Elem{…}))` (e struct-update
`..(**e).clone()` em `materialize_time`). Commit; zero código.

---

## Lote 13 — `Figure` (o último element-shaped)

**Composição: `Figure`(89)** — variante única, a mais larga.

| Aspeto | Resultado |
|--------|-----------|
| Campos | `body: Content`, `caption: Option<Content>`, `kind`/`numbering: Option<String>` (4; Box removido de body/caption) |
| **Locatável** | **SIM** (M1) — absorve `element_kind`→`Figure`, `to_payload`→`Figure{kind, counter_update:Step, is_counted: numbering.is_some() && caption.is_some()}` |
| `is_empty` | `body.is_empty() && caption vazio/ausente` |
| `map_content`/`map_text` | recursam body+caption, **simétricos** (precedente Quote L8; sem assimetria) |
| `Hash` | **derive** (`Content` manual Hash + `Option<String>`; sem floats, sem dependência) |
| Construtor | `figure(body, caption, kind, numbering)` — cobre os 4 campos → transformador usa `Content::figure(…)`, **não Arc-wrap** (regra C2 não dispara) |

### Toque em `figure_image.rs` e `layout/figure.rs`

- **`infer_kind_from_body`** (tocado no L7): matcheia o **body**
  (`Content::Image(_)`/`Content::Raw(_)`, já migrados) — **inalterado**.
- **`native_figure`** (construção): `Content::figure(…)`; `caption` deixou de
  embrulhar em `Box`.
- **`layout_figure`**: assinatura `caption: &Option<Box<Content>>` →
  `&Option<Content>` (`*cap.clone()` → `cap.clone()`).
- **`get_field` "body"** fica no hub (arm próprio `(Content::Figure(e), "body")
  => Some(Value::Content(e.body.clone()))`) — **não absorvido** ao trait
  (content-preserving; o `FigureElem::get_field` é o default).

### C1…C1-quater + C2 — sites tratados à mão (zero conversões indevidas)

Skip-list: **22 sites** registados, entregue aos dois transformadores. Tratados
à mão:

- **Arms de produção** (`introspect.rs` materialize/walk/computa-numeração ×3;
  `layout/mod.rs`; `figure_image.rs` construção) — por mão/aliases/construtor.
- **`matches!` com guarda de campo** (`stdlib/mod.rs` ×3,
  `caption: Some(_)`/`None`): o transformador dropou a guarda → restaurada à mão
  (`if e.caption.is_some()`/`is_none()`); `matches!(&result, …)` para não mover.
- **`matches!` em or-pattern** (`eval/rules.rs`): o transformador bindou `(e)`
  num arm `|`-combinado (`e` não bound em todos) → `(_)`. (Nota: `{ .. }` é
  válido em variante-tuplo — `Raw`/`Equation` já o usavam; só `Figure(e)`
  quebrava.)
- **`Box` em `if`-expr** (`layout/tests.rs` caption) → unbox à mão.
- **Verificação pós-passada**: ✓ **interseção vazia** (zero `Content::figure(`
  em posição de padrão).

---

## Medições do lote (ADR-0104)

- **`cargo build`**: limpo. **Suíte**: typst-core **2685** (era 2678; **+7**),
  0 failed; `typst-infra` 472, restantes verdes.
- **`content.rs`**: **5157 → 5135** (−22). **Parte atómica**: `figure.rs` =
  **164 linhas**.
- **`crystalline-lint`**: 0 drift; **✓ No violations found**.

---

## Balanço da fase de lotes (piloto P316 + Lotes 2–13) — consolidação ADR-0104

### Trajetória do hub (`content.rs`, linhas por commit de lote)

| Passo | Lote | `content.rs` | Δ |
|-------|------|-------------|---|
| P313 | baseline | ~5782 | — |
| P316 | piloto (3) | 5785 | +setup do trait |
| P317 | L2 math (11) | 5735 | −50 |
| P318 | L3 lista/termos (5) | 5700 | −35 |
| P319 | L4 decorações (3) | 5639 | −61 |
| P320 | L5 quebras/grid-hdr (9) | 5603 | −36 |
| P321 | L6 state/counter (7) | 5573 | −30 |
| P322 | L7 largura (5) | 5557 | −16 |
| P323 | L8 largura (4) | 5495 | −62 |
| P324 | L9 largura (5) | 5419 | −76 |
| P325 | L10 largura (3) | 5385 | −34 |
| P326 | L11 largura (2) | 5395 | +10 (2 construtores novos) |
| P327 | L12 bloco grid/table (4) | 5157 | **−238** (o maior) |
| P328 | L13 `Figure` (1) | **5135** | −22 |

**Hub: 5782 → 5135 = −647 acumulado** (−11.2%), apesar de 15 variantes ainda
terem arms próprios (não migradas; ver dossiê). O setup do trait (P316) foi pago
uma vez; cada lote depois encolheu.

### Parte atómica e suíte

- **Parte atómica**: **62 módulos** `entities/elements/*.rs` (um por variante
  migrada) — a lógica saiu dos 6 matches gigantes para módulos testáveis
  isoladamente.
- **Suíte**: typst-core **2521 (P313) → 2685 (P328)** = **+164 testes**
  unitários (cada `…Elem` trouxe os seus); 0 asserções existentes alteradas
  (só sintaxe de construção).
- **Variantes migradas**: **62** (3 piloto + 11+5+3+9+7+5+4+5+3+2+4+1 nos Lotes
  2–13).

### O método (caronas C1→C2)

A fase produziu, além do código, a **maturação do passo mecânico do
transformador** — a saga das reincidências e os seus consertos, todos no modelo:
- **C1** (P322 nota → P324 regra): padrões são manuais.
- **C1-bis** (P325): verificação pós-passada (apanha).
- **C1-ter** (P326): skip-list por linha (previne).
- **C1-quater** (P327): skip-list aos **dois** transformadores + aninhados por
  classe.
- **C2** (P328): construtor-vs-Arc-wrap por aridade.
Resultado: P327 e P328 tiveram **interseção pós-passada vazia** — a prevenção
funciona.

---

## Contabilidade FINAL da fase (item obrigatório)

- Migradas **61 → 62**; **element-shaped restantes: 0**.
- Conta de fecho: **62 migradas** + 4 `Set*` (→ F/99.E) + 11 (DEBT-58) = **77** ✓.
- **Gatilho do DEBT-58 DISPARADO** — os element-shaped esgotaram. Registrado.

## Dossiê da triagem DEBT-58

Mora em **`00_nucleo/materialization/dossie-triagem-debt-58.md`**. Resumo:
inventário das **15 não migradas** (4 `Set*` fora da triagem + 11 DEBT-58:
primitivos AST/math `MathIdent`/`MathText`/`MathSequence`, cola estrutural
`Sequence`/`Empty`/`Block`, cola de texto `Space`/`Text`, wrappers
`Styled`/`Boxed`/`Labelled`) com largura refeita hoje × forma no hub; as
perguntas que a triagem deve responder (primitivo vs `Elem`? wrapper vs lote
tardio? cola segue a guideline?); e a fotografia do hub final (62 dispatch +
~15 arms próprios remanescentes) para o F / DEBT 99.E. **Zero decisão de desenho
neste passo** — é o material com que a triagem começa.

## Fora de escopo (confirmado)

A **triagem do DEBT-58 em si** (passo seguinte, com o dossiê na mão); F /
`Set*` / 99.E; qualquer migração além de `Figure`; otimizações sugeridas por
medição (medir ≠ mexer). Caveat conhecida: stack default em `recursao_infinita_*`
— não é regressão (`RUST_MIN_STACK=33554432`).
