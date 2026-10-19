# Passo 288 — `Style::Lang(Lang)` variant

**Frente**: `P-style-lang-variant` (rank 3 do relatório P287 §9).
**Origem**: P287 §4 — "testes que requerem injectar `Lang::EN/PT`
via `Style::Lang(...)` foram adiados porque o enum `Style` (em
`entities/style.rs`) ainda **não tem variant `Lang`** — feature
ortogonal cuja activação será passo futuro condicional (paralelo
arquitectural à activação P285 do `stroke` em P284)".
**Pré-requisitos**: nenhum bloqueador.
**Origem secundária**: Tabela B.3 lista 5 variants de `Style`
(Bold/Italic/Size/Fill/HeadingLevel) — sem `Lang`. Mas Tabela B.4
lista `StyleDelta.lang` como `implementado⁺` desde P144
(hyphenation). Activar o variant `Style::Lang(Lang)` fecha a
assimetria.

---

## §1 — Objectivo

Adicionar `Style::Lang(Lang)` ao enum `Style` em
`01_core/src/entities/style.rs`, integrar com a cascata de
styles existente (parse `#set text(lang: "de")` → `Style::Lang(Lang::DE)`
→ acumula em `StyleDelta.lang`), e desbloquear os 3 testes
lang-aware adiados em P287.

Razão de ser:

1. **Fecha assimetria registada** entre Tabela B.3 (`Style`
   variants) e Tabela B.4 (`StyleDelta` fields). `StyleDelta.lang`
   existe e é preenchido — mas o caminho actual passa por algum
   mecanismo distinto do enum `Style` (verificar A.1).
2. **Desbloqueia testes adiados P287 §4** — 3 testes lang-aware
   já especificados mas inactivos.
3. **Reaplicação do padrão P285 §8.3** ("activação posterior de
   feature parseada-mas-inerte") para `Style` enum em vez de
   `FrameItem`/Layouter — paralelo arquitectural directo,
   confirmado no relatório P287 §9 ponto 3.
4. **Candidato a disparar N=5 do padrão "single source of truth
   como invariante anti-bug"** — se A.4 confirmar que `FrameItem::Text`
   continua a consultar `Lang` via mecanismo existente sem alteração
   em L3, hash `export.rs 66cb8ac3` preservado pelo **5º passo
   consecutivo** → formalização ADR meta natural.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Quatro ambiguidades factuais antes de materializar:

### A.1 — Inventário do caminho actual `lang` na cascade

Listar literalmente em
`00_nucleo/diagnosticos/diagnostico-style-lang-passo-288.md`:

1. **Como `#set text(lang: "de")` é processado actualmente** — se
   alguma vez é. Tabela A.2 linha 69 lista `#set text(...)` com
   nota "weight/tracking/leading/lang/font activos pós-DEBT-52".
   Verificar literalmente como.
2. **De onde vem `StyleDelta.lang`** — se há algum caminho indirecto
   (e.g. via `Style::Other(Lang)` catchall? via parse directo no
   eval sem passar pelo enum `Style`?). `grep -rn "StyleDelta.*lang\|delta\.lang"`.
3. **Como `Layouter.style.lang` é populado** — referência directa
   em P287 §2.3 `match &self.style.lang`. Identificar onde é
   inicializado.
4. **Crate `hypher` integration P144** — `text/lang.rs`. Como
   o `Lang` é consultado durante hyphenation? Via `StyleDelta` agregada
   ou directamente do `Style` enum? P144 (ADR-0057) deve clarificar.

Output: diagrama de fluxo (texto) `#set text(lang: "de")` →
parse → ??? → `StyleDelta.lang: Some(Lang::DE)` →
`Layouter.style.lang` consumo.

### A.2 — Estrutura do variant `Style::Lang`

Decisão arquitectural (per ADR-0065 critério #1 — extensão de
enum core L1):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Lang(Lang)` — paralelo aos existentes (`Bold(bool)`, `Size(Pt)`) | Simétrico; mínimo | Acopla `Style` ao tipo `Lang` que vive noutro módulo |
| **(b)** `Lang(Option<Lang>)` — permite "unset" explícito | Captura `set text(lang: none)` | Vanilla typst não suporta `lang: none`; over-engineering |
| **(c)** `LangCode(EcoString)` — preserva string raw, conversão tardia | Sem dependência directa em `Lang` enum | Acopla representação string ao runtime; replica parsing |

Default sugerido: **(a)** — paridade com os 5 variants existentes
da Tabela B.3 (todos guardam tipo concreto, não Option nem string
raw). **(b)** rejeitada salvo se A.1 revelar uso de `lang: none`.
**(c)** rejeitada — adicionaria parsing redundante.

### A.3 — Integração com `StyleDelta`

Quando `Style::Lang(Lang::DE)` for processado pelo cascade
acumulator (provavelmente `StyleDelta::apply` ou método análogo;
verificar em A.1):

| Opção | Comportamento |
|---|---|
| **(α)** `delta.lang = Some(lang)` — semântica "last write wins" (paralelo aos outros campos da `StyleDelta`) | Padrão dos 9 fields existentes em `StyleDelta` |
| **(β)** Merge com algum mecanismo herdado (lang do parent persiste salvo override explícito) | Vanilla typst aplica scoping de blocos — verificar A.1 |

Default sugerido: **(α)** salvo se A.1 mostrar que P144 já
implementa (β) e o restante código depende disso — caso em que
A.3 reusa a infraestrutura sem alterar. **(β)** é o comportamento
esperado de qualquer cascade — A.1 deve confirmar a mecânica
actual.

### A.4 — Impacto em `FrameItem::Text` e emit

Decisão crítica para o gatilho N=5 do padrão "single source of
truth":

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** `FrameItem::Text` já consulta `Lang` indirectamente via `StyleDelta` herdada do Layouter (P144 paradigm) — P288 apenas estende o caminho de entrada | `export.rs` **preservado bit-exact** (5º passo consecutivo) → dispara N=5 |
| **(ii)** `FrameItem::Text` precisa de novo campo `lang: Option<Lang>` para o emit consultar (e.g. para shaping per-lang) | `export.rs` **muda** — não dispara N=5 |
| **(iii)** Híbrido — emit não muda mas algum reflector toca o ficheiro | `export.rs` muda **mas** trivialmente |

Default sugerido: **(i)** se A.1 confirmar P144 paradigm (hyphenation
funciona consultando `Layouter.style.lang` sem ter que registar `Lang`
em `FrameItem`). **(ii)** apenas se necessidade concreta emergir
durante materialização — improvável dado escopo P288 limitado.

**Implicação gatilho N=5**: se A.4 → (i), este passo cita o padrão
"single source of truth" e atinge N=5 — formalização ADR meta
**dispara naturalmente neste passo** (não no seguinte). Relatório
deve registar a transição como evento histórico. **Promoção formal
do ADR meta passa a ser sub-objectivo legítimo deste passo**
(P273.17 §0 anti-padrão over-formalização: só promover se o gatilho
genuinamente disparar; se A.4 → (ii)/(iii), não promover).

---

## §3 — Materialização

Após Fase A produzir inventário + estrutura + integração + impacto
emit:

1. Adicionar `Lang(Lang)` ao enum `Style` em
   `01_core/src/entities/style.rs`.
2. Implementar visitor pattern parallelo aos 5 variants existentes:
   `Display`, `PartialEq`, `Hash` (se aplicável).
3. Estender o acumulator/aplicador `StyleDelta` (nome exacto via
   A.1) — adicionar arm para `Style::Lang(l) => delta.lang = Some(l)`
   ou equivalente per A.3.
4. Estender o parser de `#set text(lang: "de")` (caminho identificado
   em A.1) para produzir `Style::Lang(...)` em vez do mecanismo
   actual indirecto, **se** A.1 mostrar que existe um caminho
   indirecto distinto. Se P144 já produz `Style::Lang(...)`
   internamente (improvável dado Tabela B.3), apenas expor.
5. Activar os 3 testes adiados em P287 §4:
   - Localizar no código P287 (referência relatório §5.2).
   - Remover marcador `#[ignore]` ou condicional.
   - Confirmar que `Style::Lang(Lang::EN)` / `Style::Lang(Lang::PT)`
     injectado num Layouter de teste produz output lang-aware
     esperado (curly quotes EN vs aspas PT em SmartQuote, por
     exemplo).
6. Visitors L1:
   - Se `Style` tem visitors equivalentes ao `Content` (pouco
     provável dado Tabela B.3 lista 5 variants planos sem
     visitor pattern), estender.
7. Promoção condicional ADR meta:
   - Se A.4 → (i): este passo legitimamente atinge N=5 e formaliza
     "Single source of truth como invariante anti-bug" como
     ADR-008X (numeração via A.1). ADR contém: definição do padrão,
     5 aplicações cumulativas (P282/P285/P286/P287/P288),
     consequências, alternativas, status `IMPLEMENTADO`.
   - Se A.4 → (ii)/(iii): **não promover**. Registar em §8 do
     relatório que o gatilho não disparou neste passo.
8. Actualizar L0:
   - `00_nucleo/prompts/engine/style.md` (ou caminho equivalente).
   - Se ADR meta promovida, `00_nucleo/adrs/index.md` + ADR nova.
   - Propagar hashes via `crystalline-lint --fix-hashes`.
9. Actualizar Tabela B.3 — adicionar `Lang(Lang)` como 6º variant
   (estado `implementado`, referência P288).
10. Actualizar Tabela B.4 — nota cruzada P288 na linha 353 (path
    actual indirecto → variant directo).
11. Testes:
   - L1 unitário variant: construção; PartialEq; serialização se
     aplicável.
   - L1 unitário cascade: `StyleDelta::apply(Style::Lang(Lang::DE))`
     produz `delta.lang = Some(Lang::DE)`.
   - L1 unitário parse: `#set text(lang: "de")` em parser produz
     `Style::Lang(Lang::DE)`.
   - L1 unitário regression: testes pré-P288 que dependem de
     `StyleDelta.lang` continuam verdes byte-by-byte.
   - 3 testes lang-aware reactivados de P287 (smartquote en/pt).
   - L3 integração PDF: `#set text(lang: "en"); ...quotes...`
     produz curly quotes EN; `#set text(lang: "pt")` produz aspas PT.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-12 (modesto;
trabalho de extensão simétrica de enum existente).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P287: 2 748 testes.
  Esperado: ~2 756 a ~2 763.
- `crystalline-lint` zero violations.
- Hash L0 `style.md` muda (+1 variant em B.3).
- Hash L0 `content.md` **preserved** salvo se A.1 revelar acoplamento
  inesperado.
- Hash L0 `stdlib.md` **preserved** (sem nova função).
- Hash L0 `export.rs` **condicional** — preserved se A.4 → (i);
  registar mudança se A.4 → (ii). **Critério estrito**: A.4
  decide.
- **Regressão bit-exact validada** — todos os testes lang-aware
  pré-P288 (P144 hyphenation, P158B figure supplement, P287
  smartquote markup) continuam verdes byte-exact.
- Tabela B.3 actualizada com `Lang(Lang)` (6º variant).
- Tabela B.4 linha 353 com nota cruzada P288.
- 3 testes adiados de P287 §4 activados e verdes.
- Diagnóstico A.1+A.2+A.3+A.4 produzido.
- **Condicional**: se A.4 → (i), ADR meta "Single source of truth"
  promovida como ADR-008X com referência aos 5 passos cumulativos.

---

## §5 — Não-objectivos

- **Não** materializar `text.region`/`text.script`/`text.dir`
  (Tabela A.3 linhas 96-98). Continuam bloqueados por rustybuzz
  (DEBT-53).
- **Não** estender o conjunto de 6 idiomas suportados em
  `rules/lang/quotes.rs` (P155) ou `text/lang.rs` (P144). Adições
  são passos próprios.
- **Não** activar shaping per-lang (kern/lig/bidi). DEBT-53 XL,
  fora de scope.
- **Não** materializar `text.smartquotes` (atributo `set text`).
  Continua ADR-0054 graded; passo distinto candidato.
- **Não** promover ADR meta se A.4 → (ii)/(iii). A promoção é
  **condicional** ao gatilho disparar genuinamente neste passo —
  P273.17 §0 proíbe promoção mecânica.
- **Não** alterar `StyleDelta.lang` semanticamente. Se já é
  `Option<Lang>` (Tabela B.4 sugere que sim, dado `implementado⁺`),
  apenas adicionar caminho de entrada via `Style::Lang(...)`.
- **Não** materializar `Style::Region`/`Style::Script`/etc.
  Apenas `Lang` neste passo. Outros campos da `StyleDelta` que
  ainda não têm variant correspondente em `Style` enum (verificar
  A.1) ficam para passos próprios.

---

## §6 — Pendências relacionadas

Resolve:
- **P287 §4 / §5.2** — 3 testes lang-aware adiados.
- **Assimetria Tabela B.3 vs B.4** — `lang` é o único `StyleDelta`
  field sem variant `Style` correspondente (verificar A.1).

Não resolve (continua aberto):
- Outros `StyleDelta` fields sem variant `Style` correspondente
  — se A.1 revelar `weight`/`tracking`/`leading`/`font`
  ausentes do enum `Style`, registar mas passos próprios.
- Show rules sobre lang (`#show "de": ...`) — bloqueado por regex
  em L1.
- `text.smartquotes` atribute disable — passo candidato P287.1.

---

## §7 — Risco residual

Risco principal: A.1 pode revelar que **não há caminho indirecto**
para `StyleDelta.lang` — i.e., Tabela B.4 marca `implementado⁺`
mas `lang` está dead code (parseado mas nunca consultado em emit
real). Mitigação: A.1 traça o fluxo literalmente; se for dead code,
P288 activa pela primeira vez — descoberta inesperada mas não
bloqueante (paralelo a P285 que activou `stroke` parseado mas
inerte).

Risco secundário: A.4 → (ii) (precisa novo campo em `FrameItem::Text`).
Mitigação: §5 não-objectivo "não promover ADR meta se A.4 →
(ii)"; relatório regista honestamente que gatilho não disparou.

Risco terciário: assimetria oculta entre `Style` enum e `StyleDelta`
fields é maior do que 1 (lang). Se A.1 revelar 3-5 fields sem
variant correspondente (e.g. `weight`, `tracking`, `leading`,
`font`), considerar sub-passos P288.1, P288.2 — **não materializar
todos neste passo**. P288 é escopo XS por design; alargamento
quebra a sequência fechada P282-P285-P286-P287 de passos cirúrgicos
focados.

Risco quaternário: promoção ADR meta indevida. Mitigação: §5
explicita condicional; relatório do passo distinguirá citação de
gatilho real. Falha mesmo subtil em A.4 → (i) (e.g. um reflector
trivial em export.rs) **desclassifica** o gatilho — relatório
regista N=4 sustained e adia formalização.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/style.rs` (`Style` enum,
  5 variants pré-P288).
- Tipo relacionado: `01_core/src/entities/style.rs` ou módulo
  vizinho (`StyleDelta` struct, 10 fields).
- Caminho cascade actual: identificar em A.1 — provavelmente
  `apply` ou método análogo em `StyleDelta`.
- Caminho parse `#set text(...)`: P102 (ADR-0040) introduziu o
  mecanismo `#set text(lang: ...)` — ver
  `01_core/src/engine/eval/set.rs` (ou caminho equivalente).
- Precedente "activação posterior de feature parseada-mas-inerte":
  P285 (FrameItem::Line.color) + P286 (decorations wrap-aware) +
  P287 (smartquote markup-paralelo) — N=3 cumulativo do padrão.
- P144 (ADR-0057) — `text.lang` via crate `hypher`; precedente
  directo da consulta lang em runtime.
- P158B — `figure_supplement_for_lang(kind, lang)`; reusa o
  paradigma "consultar `Lang` em runtime sem persistir em emit".
- ADR processual: ADR-0065 (inventariar-primeiro; 4 critérios
  cobertos por A.1/A.2/A.3/A.4).
- ADR cultural: P273.17 §0 anti-padrão over-formalização (justifica
  **condicional** sobre promoção ADR meta).
- ADR estilo: ADR-0038 (justifica `Style` enum em vez de vtable
  vanilla); ADR-0040 (`#set text` activation).

---

*Spec P288 produzida 2026-05-19 pós-P287 (smartquote fechado;
testes lang-aware adiados explicitamente para este passo). Frente
`P-style-lang-variant` — extensão simétrica de enum `Style` com
variant `Lang(Lang)`, paridade `StyleDelta.lang` existente (P144).
Cluster decorações P284-P285-P286 + smartquote P287 fechados;
P288 é passo de consolidação arquitectural. Fase A obrigatória
(4 ambiguidades: inventário caminho actual, estrutura variant,
integração cascade, impacto emit). Critério de fecho condicional
em A.4: se A.4 → (i), hash `export.rs 66cb8ac3` preservado pelo
5º passo consecutivo → **dispara N=5 do padrão "single source of
truth como invariante anti-bug" → promoção ADR meta natural** (não
acidental — gatilho registado em P282 §1.1, P285 §8.2, P286 §5.2,
P287 §5.1 + §8.1 explicitamente). Se A.4 → (ii)/(iii), não promover.
Honestidade epistémica explícita §5 e §7 risco quaternário. Sem
caps LOC ou magnitude (P282 §7).*
