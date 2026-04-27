# Passo P158C — Refactor `kind: String → Option<String>` em `Content::Figure`

Refactor cosmético de tipo de field em variant `Content::Figure`
existente. Aplicação **ADR-0064 Caso A estrito** — vanilla
`Smart<String>` (`Auto = computa do contexto`) → cristalino
`Option<String>` (None ↔ Auto). Consistência com pattern já
estabelecido em outros variants Content (P156G/H/I + P157B +
P159A/C aplicaram Caso A).

**Décima oitava aplicação consecutiva de materialização** desde
início da série granular P156C. **Tamanho XS** declarado em
diagnóstico P159B §3.4 — refactor cosmético sem alteração
funcional.

**Patamar Caso A cresce N=6 → 7** com primeiro Caso A "estrito"
em refactor (não em variant novo). Subpadrão emergente
"refactor para Option" candidato a registo.

---

## Estado actual antes de começar

- 63 ADRs após P159D (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em **13 passos consecutivos** P156L → P159D via L0-baseline).
- Hash `entities/bib_entry.rs`: `5a2c0ebd` (P159A; preservado
  P159D).
- Hash `entities/citation_form.rs`: `677849cb` (P159C).
- 1451 tests (lib+integ+diagnostic; workspace 1472); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159D: granularidade N=17;
  inventariar N=19; Smart→Option Caso A patamar N=6 com
  equilíbrio cross-domínio 50/50; §análise risco N=19;
  estabilidade hash L0 content.rs N=13; tipo entity em
  ficheiro próprio N=5; infraestrutura state lookup N=2;
  P155 cross-feature N=1; refino tipo entity sem alteração
  Content N=1.

**Diagnóstico P159B** §3.4 (esboço P158C):
- Refactor cosmético: `kind: String → Option<String>` em
  `Content::Figure` per ADR-0064 Caso A estrito.
- Sem dependência cruzada hard.
- Hash `content.rs`: **L0-baseline preserva** (variant Content
  já documentado a alto nível em prompt; refactor de tipo
  interno sem alteração de doc-comment do prompt L0). Lição
  metodológica P159A/C/D internalizada — não prever quebra.
- Tests Δ: +2-4.
- Granularidade: XS preservada.

**Política "sem novas reservas" preservada** — P158C não
cria reservas para passos pós-P158C.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3.4 esboço P158C.
- `00_nucleo/materialization/typst-passo-158a-relatorio.md` —
  precedente directo (auto-detect kind; padrão `kind: String`
  actual).
- `00_nucleo/materialization/typst-passo-158b-relatorio.md` —
  precedente i18n (supplement por lang usa `kind: &str`).
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso A definição.
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para preservação de output.
- `01_core/src/entities/content.rs` — variant
  `Content::Figure { body, caption, kind: String, numbering }`
  actual.
- `01_core/src/rules/stdlib/figure_image.rs` — `native_figure`
  actual + `infer_kind_from_body` (P158A).
- `01_core/src/rules/lang/figure_supplement.rs` —
  `figure_supplement_for_lang(kind: &str, lang: ...)` actual
  (P158B).
- `01_core/src/rules/introspect.rs` — uso de
  `Content::Figure.kind` em counters por kind.
- `lab/typst-original/crates/typst-library/src/model/figure.rs`
  (vanilla, quarentena) — confirmar `Smart<Str>` semântica.

---

## Natureza do passo

**Tamanho**: XS.

**Justificação**: Refactor de tipo de field único em variant
existente. Cascade de modificações em sítios que acedem ao
field. Sem nova feature. Sem nova decisão arquitectural-chave
além da já formalizada em ADR-0064 Caso A.

Granularidade preservada: 0 features (refactor cosmético) →
não desafia padrão N=17. Patamar empírico granularidade
mantém-se.

**Risco baixo**:
- Refactor cosmético com pattern bem estabelecido.
- ADR-0064 Caso A já validado em N=6 aplicações.
- Tests pré-existentes (P158A/B) podem ser preservados via
  `unwrap_or("image".to_string())` no caller ou refactor
  de helpers para aceitar `&Option<String>`.
- Sem alteração observável (None ↔ Auto produz default
  `"image"` — paridade P158A).

---

## Decisões já tomadas

- **Refactor de tipo**:
  ```rust
  Figure {
      body:      Box<Content>,
      caption:   Option<Box<Content>>,
      kind:      Option<String>,  // P158C: era String
      numbering: Option<String>,
  }
  ```

  **ADR-0064 Caso A estrito**: `Smart<String>` vanilla → `Option<String>`
  cristalino; `None` ↔ Auto (default `"image"` resolvido em uso).

- **Comportamento default preservado**: quando `kind = None`,
  caller resolve para `"image"` (paridade P158A `infer_kind_from_body`
  fallback). **Sem alteração observável** vs P158B.

- **Caller adaptations**:
  - `infer_kind_from_body(body)` (P158A) **continua a retornar
    `Option<String>`** — sem alteração.
  - `figure_supplement_for_lang(kind: &str, ...)` (P158B)
    continua a aceitar `&str` — caller usa `kind.as_deref().unwrap_or("image")`.
  - `introspect.rs` counters por kind: `kind.as_deref().unwrap_or("image")`
    para indexação em `local_figure_counters`.
  - `native_figure` (stdlib): aceitação de `kind: auto/none/Str`
    explícita como `Option<String>` directo (sem unwrap).

- **`infer_kind_from_body` retorna `Option<String>` directamente**
  em vez de String com fallback hardcoded — alinha com novo
  tipo do field. Caller que constrói Figure passa `Option<String>`
  directamente.

- **Sem alteração ao prompt L0 `content.md`**: variant Content
  já documentado a alto nível; refactor de tipo interno preserva
  hash via L0-baseline interpretation (lição P159A/C/D).

## Decisões diferidas

- **Refactor análogo de outros fields String em Content
  variants**: NÃO reservado. Candidato a refactor futuro se
  prioritário.

- **Helper público `extract_optional_string`**: NÃO criado
  neste passo (extracção é trivial inline). Promoção diferida
  per política consistente N=3-4.

- **Documentação completa de variants no L0 prompt content.md**:
  diferida per política — passo administrativo XS futuro NÃO
  reservado.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-figure-kind-refactor-passo-158c.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
refactor cascading + sítios callers:

1. Assinatura vanilla `Figure.kind` — confirmar `Smart<Str>`
   com semântica "Auto = computa do contexto".
2. Comportamento observável (None ↔ Auto produz default
   `"image"` per fallback caller; output idêntico a P158B).
3. ADR-0064 Caso A confirmado: refactor estrito de String
   directo para `Option<String>`.
4. Variants Content existentes a estender: `Content::Figure`
   refactor de field — não expansão.
5. Helpers stdlib reusáveis: nenhum directo (refactor
   cascading inline).
6. Limitações aceites: nenhuma (refactor cosmético sem perda
   funcional).
7. Tests planeados (refactor cascading não-regressivo + 1
   teste novo `figure_kind_none_default_image` — range 2-4
   per esboço P159B §3.4).
8. **(Específico cascading)** Identificar **todos os sítios
   callers** de `Content::Figure.kind` via grep. Esperados:
   - `entities/content.rs` (variant declaration + impls).
   - `rules/stdlib/figure_image.rs` (`native_figure` +
     `infer_kind_from_body`).
   - `rules/introspect.rs` (counters por kind).
   - `rules/layout/mod.rs` (se aplicável).
   - Tests pré-existentes (P157A/P158A/B + tests específicos
     de Figure).
9. **(Específico backwards compat)** Confirmar que tests
   pré-existentes preservam-se via `kind.as_deref().unwrap_or("image")`
   ou similares — sem mudança de assinatura externa stdlib
   `native_figure` (continua a aceitar `auto/none/Str`).

### .2 Refactor variant `Content::Figure`

`01_core/src/entities/content.rs`:
- Modificar field declaration: `kind: String → kind: Option<String>`.
- Cobrir todos os sítios pattern-match Content existentes
  (paridade P157A/B/C + P159A/C):
  - Variant declaration + construtor.
  - `is_empty()`: comportamento inalterado (ignora kind).
  - `plain_text()`: continua a usar caption + body; kind
    irrelevante.
  - `PartialEq`: cobre kind agora `Option<String>` —
    `Some("image") ≠ None` distinção semântica nova.
  - `map_content`/`map_text`: kind preservado tal como recebido.
  - `introspect.rs::materialize_time`/walk: usa
    `kind.as_deref().unwrap_or("image")` para counters.
  - `layout/mod.rs::layout_content`: arm Figure
    inalterado (não acede kind directamente).

- Construtor `Content::figure(body, caption, kind, numbering)`
  agora aceita `kind: Option<String>` directo.

### .3 Adaptar `infer_kind_from_body`

`01_core/src/rules/stdlib/figure_image.rs`:
- Helper `infer_kind_from_body(body) -> Option<String>` já
  retorna `Option<String>` (P158A).
- **Sem alteração à assinatura**.
- Caller `native_figure`: refactor para usar `Option<String>`
  directamente em vez de `unwrap_or_else(|| "image")`.

### .4 Adaptar `native_figure` stdlib

`01_core/src/rules/stdlib/figure_image.rs`:
- Aceitação de `kind: auto/none/Str` — refactor para retornar
  `Option<String>` directamente:
  ```rust
  let kind = args.named.get("kind")
      .and_then(|v| match v {
          Value::Str(s) => Some(Some(s.to_string())),
          Value::Auto | Value::None => Some(None),
          _ => None,  // tipo inválido
      })
      .unwrap_or_else(|| infer_kind_from_body(&body));
  ```
- Se ainda `None` após fallback chain, **não aplicar default
  `"image"` no momento da construção** — passa `None` directo
  ao construtor `Content::figure(..., None, ...)`.
- **Default `"image"` resolvido em uso** (counters, supplement,
  etc.) per ADR-0064 Caso A — não em construção.

### .5 Adaptar `figure_supplement_for_lang`

`01_core/src/rules/lang/figure_supplement.rs`:
- Helper continua a aceitar `kind: &str` (não muda assinatura).
- Caller em `introspect.rs` ou layout passa
  `kind.as_deref().unwrap_or("image")` para garantir &str.

### .6 Adaptar `introspect.rs`

`01_core/src/rules/introspect.rs`:
- Counters por kind: indexação usa
  `kind.as_deref().unwrap_or("image")` para preservar
  comportamento P157A/P158A.
- Format de label: idem.

### .7 Tests

- **Test novo** em `entities/content.rs` ou `stdlib/mod.rs`
  (~1):
  - `figure_kind_none_resolve_para_image_default` —
    `figure(...)` sem kind explícito + body não-detectável
    (e.g. Text) produz `kind == None`; counter usa "image"
    via fallback caller.

- **Tests existentes adaptação** (~1-3):
  - Tests P157A/P158A/B que verificam `kind == "image"` em
    output: ajustar para verificar saída final (counter,
    label) em vez de field directo, OU adaptar para `kind
    == Some("image")` se test inspeccionar Content directamente.

- **Δ esperado**: +2 a +4 tests (alinhado com esboço P159B
  §3.4; refactor cascading + 1 teste novo).

### .8 Propagação de hashes

`crystalline-lint --fix-hashes .`:
- `content.rs` hash: **L0-baseline preserva** `ec58d849` —
  prompt `content.md` não modificado.
- `figure_image.rs`: pode quebrar via L0-baseline se prompt
  `figure_image.md` for actualizado — esperado preservar
  per refactor interno cosmético.
- `introspect.rs`/`figure_supplement.rs`: idem.

**Verificação esperada**: "Nothing to fix" se interpretação
L0 mantém — hashes preservam-se em todos os ficheiros.
**Lição P159A/C/D internalizada**: refactor de tipo interno
sem alteração de prompt L0 preserva hash.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1451 + Δ** tests, zero falhas
   (Δ esperado +2-4).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refactor
   de field).
4. Contagem stdlib funcs: **48** (inalterada — `native_figure`
   modificada via parsing).
5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **14º passo consecutivo** se interpretação L0-baseline
   mantém (lição P159A/C/D internalizada — preservação é
   regra default).
6. ADR-0064 Caso A patamar **N=6 → 7** com primeiro Caso A
   estrito em refactor (não em variant novo).
7. Tests pré-existentes Figure (P157A/P158A/B) passam
   inalterados ou adaptados via fallback `kind.as_deref().unwrap_or("image")`
   nos callers.
8. Cascading sítios callers identificados em .1 e cobertos
   integralmente em .2-.6.
9. **Sem novas reservas** criadas (paridade política P158).
10. ADR-0017 não afectada (refactor não toca counters
    cross-document).
11. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A/C/D).
12. Comportamento observable: idêntico vs P158B (None ↔
    Auto produz default "image" per caller fallback).

---

## Critério de conclusão

- Verificações 1-12 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-158c-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=19 → 20).
  - Slope cumulativo Model (mesa P155-P158C).
  - ADR-0061 §"Aplicações cumulativas" anotada com P158C.
  - **Confirmação**: ADR-0064 Caso A patamar N=7;
    estabilidade hash L0 content.rs N=13 → 14 (se preservado).
  - **Subpadrão emergente** "refactor de field para Option"
    (precedente novo se primeiro do tipo).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela sítio caller não previsto (e.g. test
  helper ou módulo non-óbvio) → adicionar à lista de cascading;
  documentar.
- Inventário .1 revela que vanilla `Figure.kind` é `Smart<Content>`
  em vez de `Smart<Str>` (e.g. accept Content type para custom
  kinds) → ajustar refactor para `Option<Content>` em vez
  de `Option<String>`; expandir scope levemente.

**Cenários específicos**:
- Tests pré-existentes verificarem `kind == "image"` directo
  vs `kind == Some("image".to_string())` — adaptar tests para
  consumir output final ou usar `Some(...)` literal.
- `figure_supplement_for_lang(kind: &str)` ter callers em
  múltiplos sítios além dos identificados — grep adicional;
  documentar.
- Counters em `introspect.rs` usarem `&String` em vez de
  `&str` — `as_deref()` resolve.
- L0-baseline NÃO preservar hash `content.rs` (e.g. doc-comment
  do field foi modificado por inadvertência) → reconhecer e
  documentar; quebra excepcional não bloqueante.

---

## Notas operacionais

- **Décima oitava aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso baixo (refactor
  cosmético; pattern já validado N=6). Vigésima aplicação
  consecutiva — preserva precedente.
- **ADR-0064 Caso A patamar cresce N=6 → 7**: primeiro Caso A
  "estrito" em refactor (não em variant novo). Distribuição
  cross-domínio passa a 4 Layout (P156G/H/I + ?) + 4 Model
  (P157B/P159A/C + **P158C**) — equilíbrio mantido se
  P156G/H/I = 3 Layout vs aplicações.
- **Lição metodológica internalizada**: hash `content.rs`
  preserva-se via L0-baseline a menos que prompt L0 seja
  explicitamente modificado. Refactor de tipo interno
  cosmético cabe na regra. Primeira aplicação consciente
  desta lição em enunciado.
- **Subpadrão emergente NOVO**: "refactor de field para Option"
  — primeiro do tipo se confirmado. Distinto de:
  - Variant novo com `Option<T>` field (P156G/H/I/P157B/P159A/C
    — patamar normal Caso A em variant aditivo).
  - Refino de tipo entity (P159D — não tocou Content).
  - Expansão de variant existente com novo field (P156L Pad;
    P159C Cite — Option foi para field novo, não refactor).
  P158C é o primeiro **refactor de field existente** (era
  String) **para Option**. Subpadrão pode crescer se outros
  fields String forem similarmente refactored.
- **Política "sem novas reservas" preservada** — refactor
  análogo de outros fields String NÃO reservado.

---

## Pós-passo

Após conclusão de P158C:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refactor cosmético sem mover counts).
**Hash `entities/content.rs` provavelmente preservado** (14º
passo consecutivo via L0-baseline).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **P159F** — Numbering numérico Bibliography (M; counter
  local). Único candidato Bloco A restante após P158C.
- ADR-0062-create (XS administrativo; ainda pendente).
- Mudança de módulo (Introspection P160).
- Bloco B (com ADR-0062 PROPOSTO).
- Outras direcções pendentes.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=17 com P158C se
fechar sem reformulação) **NÃO** é formalizado em ADR.
Continua candidato.

**Pausa natural após P158C — Figure.kind refactored para
Option<String>; ADR-0064 Caso A patamar N=7; estabilidade
hash content.rs N=14 (esperado). Decisão humana sobre próxima
direcção tem máxima informação. Bloco A do diagnóstico P159B
fica reduzido a 1 candidato (P159F) após P158C.**
