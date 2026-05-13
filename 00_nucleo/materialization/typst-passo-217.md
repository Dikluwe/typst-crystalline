# Passo 217 — `Content::Columns` variant + arms exhaustivos

**Série**: 217 (terceiro sub-passo materialização Layout
Fase 3; primeiro sub-fase (b) DEBT-56; aditivo puro).
**Marco**: nenhum (sexto passo pós-M9c; primeira adição
de variant pós-M9c).
**Tipo**: aditivo puro ao `Content` enum + arms exhaustivos
em ~7 sítios L1 (paridade P156C-J série).
**Magnitude**: S+ (~1.5-2h).
**Pré-condição**: P216B concluído (`Regions` minimal em
`entities/region.rs`; Layouter usa `regions: Regions`; 1946
tests verdes; sub-fase (a) DEBT-56 fechada estruturalmente);
ADR-0078 PROPOSTO anotada com P216A+P216B; humano fixou
Caminho 1 (continuação Fase 3 imediatamente). Pattern
"refactor stacking" N=1; cohabitação L0 N=2; anti-inflação
11 aplicações cumulativas.
**Output**: 1 ficheiro relatório curto + código alterado +
extensão L0 `entities/content.md` (sem ficheiro novo;
secção `Columns` adicionada) + ADR-0078 anotada (sem
transição de status).

---

## §1 Trabalho

P216A+P216B agregaram state geométrico do Layouter em
`Regions`. P217 adiciona o **primeiro consumer estrutural**
da multi-region: variant `Content::Columns { count, gutter,
body }` paridade vanilla `ColumnsElem`. **Aditivo puro** —
sem consumer real ainda (consumer multi-column em P219
sub-fase b).

**Decisão central de P217**: variant rico (não Styled) +
arms exhaustivos em ~7 sítios L1 (paridade P156G `Block`,
P156H `Boxed`, P156I `Stack`). Layouter arm em P217 é
**stub transparente** — `Content::Columns { body, .. }`
delega a `layout_content(body)` ignorando count/gutter.
Consumer multi-region real em P219.

**Justificação stub transparente em P217**:
- Variant existe mas sem semantic real até P219.
- Permite tests E2E `#columns(2)[texto]` parsearem sem
  erro.
- `Content::Columns` decorre como contêiner transparente
  para introspect (counters/labels dentro contam normal).
- Mantém pattern P156J `Repeat` (single-render diferido
  per ADR-0054 graded) — variant + stdlib materializados,
  consumer real diferido.

**Decisão alternativa rejeitada**: combinar P217 (variant)
+ P218 (stdlib) + P219 (consumer) num único sub-passo
"big-bang". Rejeitada porque:
- Atomização ADR-0036 — features distintas em sub-passos
  distintos.
- Variant + arms é trabalho aditivo puro (sem riscos
  semantic real).
- Stdlib + consumer multi-region é trabalho com risco
  semantic (P219 introduz consumer real; isolamento
  permite tests específicos).
- Precedente literal P156J: `Repeat` variant + arms
  materializados separadamente do refino lazy semantic.

Reuso de dados (sem recolha nova):

- Tabela B.2 inventário 148 — Content cristalino 56
  variants pós-P159A; P217 leva a 57.
- ADR-0078 PROPOSTO §"Decisão" `Columns` forma proposta.
- Precedente P156J `Content::Repeat` (3 fields: body,
  gap: Option<Length>, justify: bool) — pattern
  estrutural similar.
- Precedente P156G `Content::Block` (5 fields incluindo
  body + 4 attrs com scope-outs documentados).
- ADR-0054 graded — scope-outs autorizados.

---

## §2 Cláusulas (10)

### C1 — Confirmar contagem `Content` enum pós-P159A

Auditoria empírica:

```
grep -c "^    [A-Z][A-Za-z]\+\(\s\|{\|(\)" 01_core/src/entities/content.rs
```

Hipótese: **56 variants** (P155 41 → P156C-J +9 → 50 → P157A/B/C
+4 → 54 → P159A +2 → 56). Inventário 148 Tabela B.2 lista 41
(desactualizada pós-P155); contagem real via grep confirma 56.

Se contagem divergir: registar `P217.div-1` antes de C2.

### C2 — Adicionar `Content::Columns` variant

Editar `01_core/src/entities/content.rs` adicionando variant:

```rust
/// Multi-column container — Fase 3 Layout (DEBT-56).
///
/// Adicionado em P217 (variant + arms exhaustivos);
/// stdlib `native_columns` em P218; consumer multi-region
/// real em P219 (sub-fase b DEBT-56).
///
/// P217 layouter arm é transparente — delega a body
/// ignorando count/gutter. Permite parsing e introspect
/// sem semantic real até P219.
///
/// Paridade vanilla `ColumnsElem { count, gutter, body }`
/// per ADR-0078 PROPOSTO + ADR-0054 graded.
Columns {
    /// Número de colunas. `count >= 1` (validado em
    /// `native_columns` P218).
    count: usize,
    /// Espaço entre colunas. `Option<Length>`: `None` ↔
    /// default vanilla (~4% width per ADR-0064 Caso C).
    /// Default resolvido em uso no consumer P219.
    gutter: Option<crate::entities::length::Length>,
    /// Body — content a fluir entre N colunas.
    body: Box<Content>,
},
```

**Decisão atributos**:
- **`count: usize`** vs `count: u32`/`u16`: `usize` paridade
  Rust convencional para contagens. Validação `>= 1`
  diferida a P218 (`native_columns`).
- **`gutter: Option<Length>`** per **ADR-0064 Caso C**:
  `None` ↔ default vanilla. Default resolvido em uso
  (P219 `gutter.unwrap_or(default_gutter(width))`).
  Patamar Caso C cumulativo N=cresce.
- **`body: Box<Content>`** paridade `Pad`/`Hide`/`Block`/
  `Boxed`. Boxed para evitar tamanho recursivo de `Content`.
- **Scope-outs declarados** (per ADR-0054 graded; vanilla
  `ColumnsElem` tem só estes 3 fields — sem extras).

Variant count Content: **56 → 57**.

### C3 — Arms exhaustivos em ~7 sítios L1

Cobertura mecânica de `match content` em todos os sítios
exhaustivos (paridade P156G-J).

**`entities/content.rs`** (5 arms):
- `is_empty` — proxy para `body.is_empty()`.
- `plain_text` — recurse em body (transparente).
- `PartialEq::eq` — comparação 3-fields (count, gutter,
  body deep eq).
- `map_content` — recurse em body; count/gutter preservados
  via `Copy`/`Clone`.
- `map_text` — recurse em body; idem.

**`rules/introspect.rs`** (2 arms):
- `materialize_time` — recurse em body (transparente;
  counter/labels descendentes contam normal).
- `walk` — recurse em body (sem `Tag::Start/End` próprio;
  columns não é locatable).

**`rules/layout/mod.rs::layout_content`** (1 arm):
- **Stub transparente P217**: `Content::Columns { body, .. } =>
  { self.layout_content(body); }`.
- Count/gutter armazenados mas ignorados (consumer real
  P219).

Total: **8 arms** em 3 ficheiros.

Helpers de construção:
- `Content::columns(body, count, gutter) -> Content`
  construtor Rust.

### C4 — `native_columns` em stdlib (DIFERIDO P218)

P217 **NÃO** adiciona stdlib. Diferido a P218 (atomização).

Justificação:
- P156C precedente: variant Pad/Hide + stdlib `native_pad`/
  `native_hide` num mesmo passo. Mas P156C era agregado
  granularidade "tudo num passo" (5 features em P156C
  per ADR-0061 Decisão 1).
- P217 + P218 separados por atomização individual.
  Alternativa: agregação P217.2 (variant) + P217.4 (stdlib)
  num mesmo passo se humano preferir granularidade reduzida.

**Decisão fixada em C4**: separação P217 (variant) +
P218 (stdlib). Anti-inflação 12ª aplicação cumulativa —
P217 isolado é S+ tractável; agregação seria S+ ligeiramente
maior.

### C5 — Sentinelas P217

Tests unitários P217 em `content.rs::tests` (paridade
P156G/H/I/J):

- `p217_columns_variant_existe` — instancia
  `Content::columns(...)` + verifica fields.
- `p217_columns_plain_text_recurse` — `plain_text`
  retorna `body.plain_text()`.
- `p217_columns_is_empty_proxy` — `is_empty` proxy para
  `body.is_empty()`.
- `p217_columns_map_content_recurse` — `map_content`
  recurse em body preservando count/gutter.
- `p217_columns_partial_eq_3_fields` — `eq` compara 3
  fields.

Total sentinelas P217: **5 unit tests**.

Layout E2E test (1 test em `tests.rs`):
- `p217_columns_arm_transparente_renderiza_body` —
  `layout(Content::columns(Content::text("hello"), 2,
  None))` produz PagedDocument com `plain_text() ==
  "hello"`. Count/gutter ignorados (stub P217).

Total tests P217: **5 unit + 1 E2E = 6 tests**.
Esperado pós-P217: **1946 + 6 = 1952 verdes**.

### C6 — L0 `entities/content.md` extensão

Editar `00_nucleo/prompts/entities/content.md` adicionando
secção `## Variant `Content::Columns` (Passo 217)` antes
de "Variants conscious limitations Fase X" se existir.

Estrutura paridade P156J (Repeat) + P156I (Stack):
- Forma struct (count/gutter/body).
- Distinção material face a Block/Boxed/Stack/Repeat.
- Comportamento `is_empty` / `plain_text` / `map_*`.
- Renderização (layouter): **stub transparente em P217;
  consumer real em P219**.
- Validação em `native_columns` (diferido P218 — secção
  vazia em P217 com nota).
- Construtores: stdlib **diferido P218**; construtor Rust
  `Content::columns(body, count, gutter)`.
- Limitações conscientes (P217):
  - Consumer multi-column é stub em P217 (transparente).
  - `gutter: Option<Length>` per ADR-0064 Caso C.
  - Sem show rules `#show columns: ...`.

Hash propagado via `crystalline-lint --fix-hashes`.

### C7 — Verificação tests workspace

Critério: 1946 verdes pré-P217 + 6 sentinelas P217 = **1952
verdes**.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: zero. Qualquer test pre-existente red
indica regressão (P217 é aditivo puro — não deveria
quebrar nada).

Hipótese provável: 1952 verdes. Variant aditivo puro
com arm transparente não tem mudança observable em
features existentes.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em
`entities/content.md` (L0) + `entities/content.rs` (L1).

Esperado: 1-2 hashes drift initial (content.md + content.rs
mudaram); zero pós-`--fix-hashes`.

### C9 — Inventário 148 actualização

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Tabela B.2** linha nova:
  ```
  | `Columns {count, gutter, body}` | ColumnsElem | `parcial` | Passo 217 | variant + arms exhaustivos; stdlib P218; consumer multi-region P219 |
  ```
- **§A.5 Layout** linha `columns(n)`: **NÃO actualiza
  estado** ainda (`ausente` mantido; reclassificação a
  `implementado` ocorre só pós-P219+P220 com consumer
  real). Anotação inline: "P217 adicionou variant +
  arms; consumer real diferido P219".

Footnote 40 P217 documenta:
- Variant adicionado (56 → 57 Content variants).
- Stdlib diferido P218.
- Consumer real diferido P219.
- 6 tests adicionados (5 unit + 1 E2E).
- Sem reclassificação §A.5 — variant não é "feature
  user-facing implementada" até stdlib + consumer
  existirem.

### C10 — ADR-0078 anotação P217

`00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
§"Plano de materialização" anotado com bloco
**`### P217 materializado 2026-05-12`**:

```markdown
Sub-fase (b) DEBT-56 — primeiro sub-passo aditivo:
- Variant `Content::Columns { count: usize, gutter:
  Option<Length>, body: Box<Content> }` adicionado
  (Content variants 56 → 57).
- 8 arms exhaustivos em 3 ficheiros L1 (5 content.rs
  + 2 introspect.rs + 1 layout/mod.rs).
- Layout arm é stub transparente — delega a body
  ignorando count/gutter (consumer real P219).
- Stdlib `native_columns` diferido P218 (atomização
  ADR-0036).
- ADR-0064 Caso C aplicado a `gutter` (None ↔ default
  vanilla).
- 6 tests adicionados (5 unit + 1 E2E). Tests
  workspace: 1946 → 1952 verdes.

Status ADR-0078: PROPOSTO mantido. 4 sub-passos restantes
(P218 stdlib, P219 consumer, P220 colbreak, P221 fecho).
```

**Status**: PROPOSTO mantido. Não transita ainda.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-217-relatorio.md`.

Estrutura (~5-7 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Confirmação contagem Content (C1).
- §3 Variant `Columns` adicionado (forma + atributos +
  ADR-0064 Caso C cumulativo) (C2).
- §4 Arms exhaustivos cobertura (C3; 8 arms / 3 ficheiros).
- §5 Decisões substantivas (variant rico vs Styled; stub
  transparente vs stdlib + consumer; separação P217/P218/
  P219 atomização; anti-inflação 12ª aplicação).
- §6 Resultados verificação (tests + lint).
- §7 Inventário 148 + ADR-0078 anotação (C9 + C10).
- §8 Próximo sub-passo (P218 stdlib; Caminho 1 continuação).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (+ ~5-10
  LOC variant `Columns` + ~30-50 LOC arms exhaustivos em
  5 sítios + 5 sentinelas).
- **Editado**: `01_core/src/rules/introspect.rs` (+ 2 arms:
  `materialize_time` + `walk`).
- **Editado**: `01_core/src/rules/layout/mod.rs` (+ 1 arm
  stub transparente em `layout_content`).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+ 1
  E2E test).
- **Editado**: `00_nucleo/prompts/entities/content.md` (+
  secção `Variant Content::Columns`).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (+ linha Tabela B.2 + footnote 40 P217).
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P217).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Materializar `native_columns` stdlib — diferido P218.
- Adicionar campo `Content::Columns.count` validação
  `>= 1` — diferida a P218 stdlib validation.
- Consumer multi-region real no Layouter — diferido P219.
- Reclassificar §A.5 inventário 148 `columns` de `ausente`
  → `implementado` — só pós-P219+P220 (consumer real
  + colbreak).
- `Content::Colbreak` — diferido P220.
- Tests com count > 1 que verifiquem multi-column real —
  diferidos P219.
- Show rules `#show columns: ...` — fora de escopo Fase
  3 cristalino (mesmo padrão P156I Stack).
- Mudança observable em features existentes (P217 é
  aditivo puro).
- Promover ADR-0078 → IMPLEMENTADO — só P221.
- Fechar DEBT-56 — só P221.

---

## §5 Riscos a evitar

1. **Esquecer arm em sítio exhaustivo**: 7-8 sítios L1
   onde `match content` é exhaustivo (Rust não permite
   omissão). Mitigação: compiler errors detectam
   imediatamente; iterar até zero errors.
2. **Stub transparente que afecta introspect**: `walk`
   recurse no body sem emitir `Tag::Start/End` próprio
   (columns não é locatable). Cuidado para não emitir
   tag espúria que conte como elemento extra.
3. **Box<Content> recursão**: `body: Box<Content>` segue
   pattern Pad/Hide/Block/Boxed. Não tentar `Arc<Content>`
   ou outra forma — paridade preservada.
4. **`count: usize` validação prematura em P217**: tentação
   óbvia é validar `count >= 1` no construtor Rust
   `Content::columns(body, count, gutter)`. Rejeitada —
   validação é da stdlib `native_columns` (P218); construtor
   Rust aceita `count = 0` (caso degenerate; consumer P219
   pode tratar como passthrough).
5. **`gutter` validação prematura**: idem — validação
   negativa diferida a P218 (`native_columns`). Construtor
   Rust aceita `Length::ZERO` ou `None`.
6. **L0 separado para Columns**: tentação de criar
   `entities/columns.md`. Rejeitada — secção em
   `entities/content.md` paridade P156G/H/I/J. Mantém
   single-source-of-truth para Content variants.
7. **Reclassificar §A.5 prematuramente**: tentação óbvia
   é marcar `columns` como `implementado` em P217. Rejeitada
   — `implementado` requer stdlib + consumer real (P218 +
   P219). P217 deixa `ausente` com anotação progresso.
8. **Tests P217 que dependem de consumer real**: cuidado
   em `p217_columns_arm_transparente_renderiza_body` —
   teste verifica **transparência** (plain_text de body
   preservado) não comportamento multi-column. Se test
   falha, indica que stub não é transparente (erro).
9. **Anti-inflação 12ª aplicação**: separação P217/P218
   pode parecer pedantria. Defesa: atomização ADR-0036 é
   princípio consolidado; P217 + P218 são features
   distintas (variant declaration vs stdlib function).
10. **Hash drift inicial**: editar 2 L0s
    (`content.md` + ADR-0078) + 2 L1s (`content.rs` +
    layout `mod.rs`) provoca drift. Mitigação:
    `crystalline-lint --fix-hashes` resolve uniformemente.

---

## §6 Hipótese provável

C1 confirmará Content enum em 56 variants (paridade
cumulativo P155+P156+P157+P159A).

C2 adicionará `Columns { count: usize, gutter:
Option<Length>, body: Box<Content> }`; ADR-0064 Caso C
aplicado a `gutter` (cumulativo N=cresce).

C3 cobrirá 8 arms em 3 ficheiros (compiler errors guiam
até zero).

C4 fixará separação P217/P218 (anti-inflação 12ª aplicação).

C5 criará 5 unit tests + 1 E2E test (paridade P156G/H/I/J).

C6 estenderá `content.md` com secção `Columns`; hashes
propagados.

C7 reportará 1952 tests verdes (1946 + 6).

C8 reportará 0 violations pós-fix-hashes.

C9 actualizará Tabela B.2 inventário 148 (linha nova) +
footnote 40 P217. §A.5 Layout linha `columns` permanece
`ausente` (anotação progresso).

C10 anotará ADR-0078 cumulativo.

Custo real: S+ (~1.5-2h). Maior parcela em C3 (arms
exhaustivos em 3 ficheiros) + C5/C6 (tests + L0).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P217

P217 é estruturalmente distinto na trajectória pós-M9c:

- **Primeira adição de variant pós-M9c** — séries P156C-J
  (Layout Fase 1+2+3 sub1) e P157A/B/C (Model Fase 2)
  e P159A (Model Fase 2 par) adicionaram variants pré-M9c.
  P217 retoma cadência aditiva.
- **Primeiro sub-passo sub-fase (b) DEBT-56** — P216A+B
  fecharam sub-fase (a). P217 começa sub-fase (b) com
  arquitectura preparada.
- **Anti-inflação 12ª aplicação cumulativa** pós-P205D —
  separação P217 (variant) + P218 (stdlib) por atomização.
- **Pattern "refactor stacking" N=1 → ?** — P217 acede
  `self.regions.current.X` no arm Layout (3 níveis
  stacking pós-P216A+B). Pattern preservado mas estável;
  não cresce a N=2 (P217 não refactora P216B output;
  apenas adiciona arm que usa nomenclatura P216B).
- **Stub transparente pattern** — Arm Layout transparente
  é precedente novo: variant materializado sem semantic
  específica esperando consumer real em sub-passo
  posterior. Pattern possivelmente reusável (P220
  Colbreak terá situação similar). Promoção a meta
  diferida (precisa N=2-3).
- **ADR-0064 Caso C cumulativo N=cresce** — `gutter:
  Option<Length>` é aplicação adicional. Precedentes
  P156L (Sides<Option<Length>>) e P156I (Stack.spacing:
  Option<Length>).

Por isso §5 risco 1 (arms exhaustivos esquecidos) é o mais
provável. Compiler errors são a defesa principal — Rust
detecta `match` non-exhaustive imediatamente. Mitigação
operacional: iterar `cargo build` até zero errors antes
de `cargo test`.

**Critério de aceitação P217**:
- 6 tests novos verdes (5 unit + 1 E2E).
- 1946 tests pre-existentes preservados verdes.
- 0 violations.
- Content variants 56 → 57.
- Sub-fase (b) DEBT-56: 0/4 → 1/4 (primeira atomização).
