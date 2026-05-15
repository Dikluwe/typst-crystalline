# ⚖️ ADR-0080: L0 minimal para refactors aditivos pós-M9c

**Status**: `EM VIGOR`
**Data**: 2026-05-13 (PROPOSTO P226; **EM VIGOR P229**)
**Autor**: Humano + IA
**Validado**: 9 aplicações cumulativas pós-M9c
(P217+P218+P219+P220+P222+P223+P224+**P227+P228**; N=9
patamar empírico extremamente sólido).
**Reservado para**: meta-documental codifica prática
empírica emergente Fase 3 sub-fase b + Fase 4 candidata
Layout pós-M9c.

**Nota numeração**: spec P226 hipótese previa ADR-0067 mas
ADR-0067 já estava ocupado (`attribute-grammar-scoping`).
`P226.div-1` registado; ADR-0080 escolhido como próximo
slot disponível após ADR-0079 (Layout Fase 5 roadmap).

---

## Contexto

Entre P217 e P224 (cumulativamente 7 sub-passos pós-M9c da
série Layout Fase 3 sub-fase b + Fase 4 candidata), emergiu
prática empírica não-formalizada: refactors aditivos a
variants Content existentes ou novas stdlib funcs aditivas
**NÃO actualizam L0 prompts** em `00_nucleo/prompts/`; em
vez disso, documentam decisões em:

- **Inline-doc no código Rust** (`/// ...` sobre fields,
  functions, variants).
- **Footnote em inventário 148**
  (`typst-cobertura-vanilla-vs-cristalino.md`).
- **Anotação em ADR relevante** (PROPOSTO ou IMPLEMENTADO).
- **Marca cirúrgica em blueprint §3.0...** se for fecho
  de série/Fase.

Vários passos divergiram conscientemente face às specs em
favor da prática empírica emergente:
- **P218 spec C6** propôs Opção α "linha minimal em tabela";
  materialização escolheu Opção γ.
- **P219 spec C7** propôs Opção α "secção dedicada retroactiva
  refinada"; materialização escolheu Opção γ.
- **P224 spec C6** propôs Opção α "secção dedicada para
  variants substantivos novos"; materialização escolheu
  Opção γ (divergência consciente documentada em §"Decisão
  4" P224.relatorio).

Pattern N=7 atinge limiar formalização (N=3-4 mínimo)
**amplamente ultrapassado**.

---

## Decisão

**Refactors aditivos pós-M9c NÃO actualizam L0 prompts** por
defeito. Documentação fica distribuída em código + inventário
+ ADRs + blueprint per estrutura emergente.

## Escopo

### "Refactor aditivo" qualifica para Opção γ se:

- **Variant Content novo** (sem alterar variants existentes).
- **Field novo** a variant Content existente (refino aditivo
  paridade P223 `Place +float +clearance`).
- **Stdlib func nova aditiva** (sem alterar stdlib funcs
  existentes).
- **Stdlib func refinada com named args novos** (sem alterar
  semantic de args existentes; paridade P224 `native_grid`
  +5 named args).
- **Helper privado novo** ou **promoção de visibility**
  (paridade P222 `measure_content` `pub(super)` →
  `pub(crate)`).
- **Módulo L1 novo** (paridade P224 `grid_placement.rs`).
- **Refactor de arms exhaustivos** compiler-driven cascade
  (paridade P217+P220+P223+P224).

### "Refactor não-aditivo" requer L0 actualizada
(retroactividade explícita):

- Mudança de signature de função L0-documentada.
- Mudança de variant existente sem manter paridade
  observable.
- Refactor estrutural de tipo (e.g. enum → struct;
  `Vec<T>` → `Arc<[T]>`).
- Novas regras de validação que mudem rejeição de input
  em features pre-existentes.
- Reabertura de decisões arquitecturais maiores (paridade
  ADR-0079 Fase 5 Categoria C reabertura Opção B P219;
  P216B; DEBT-56 ENCERRADA).

---

## 9 aplicações cumulativas (validação empírica)

| Passo | Refactor | L0 acção | Observação |
|-------|----------|----------|------------|
| **P217** | `Content::Columns` variant novo (P224.B/C precedente) | L0 não tocado | Decisão empírica nova |
| **P218** | `native_columns` stdlib | L0 não tocado | **Spec C6 propôs Opção α**; materialização Opção γ |
| **P219** | Layouter arm refactor + Region/Regions | L0 não tocado | **Spec C7 propôs Opção α retroactiva**; materialização Opção γ |
| **P220** | `Content::Colbreak` agregado | L0 não tocado | Convenção consolidada |
| **P222** | `native_measure` stdlib + visibility promotion | L0 não tocado | Pattern N=4 → 5 consolidado |
| **P223** | `Content::Place` +`float`+`clearance` | L0 não tocado | Pattern N=5 → 6 consolidado |
| **P224** | `Content::Grid` refino substantivo + 3 variants + módulo | L0 não tocado | **Spec C6 propôs Opção α**; materialização Opção γ. **Divergência consciente N=7** |
| **P227** | Grid+Table +`stroke` field; `Value::Stroke` variant novo; `native_stroke` constructor; renderização Opção β | L0 não tocado | **N=7 → 8 — primeira validação real pós-formalização ADR-0080** |
| **P228** | Grid+Table +`fill` field; sem `Value` variant novo + sem constructor stdlib (anti-inflação) | L0 não tocado | **N=8 → 9 — segunda validação real; promoção EM VIGOR P229** |

**N=9 cumulativo confirmado** em P229 audit. Pattern
empírico **extremamente sólido**; ultrapassa critério N=8+
de §"Promoção".

---

## Consequências

### Positivas:
- **Reduz overhead documental** de refactors aditivos
  frequentes.
- **Mantém L0 como documentação semantic estável** (não
  histórico de evolução).
- **Acelera materialização pós-M9c** (precedente N=7;
  média ~30min por refactor sem L0 update vs ~1-2h com
  L0 update + hash propagation).
- **Preserva ADR-0033 paridade observable** (L0 documenta
  semantic; refactors aditivos preservam semantic
  observable).

### Negativas:
- L0 **não reflecte estado real cumulativo** de variants/
  stdlib (que cresceram +5 variants Content + ~6 stdlib
  funcs pós-M9c sem L0 actualização correspondente).
- **Auditor externo precisa cruzar L0 + inventário 148 +
  ADRs** para estado completo.

**Trade-off aceite**: documentação distribuída vs overhead
actualização cumulativa.

---

## Alternativas consideradas

### Alt A — Actualizar L0 em todos refactors
- **Pro**: L0 sempre reflecte estado real.
- **Con**: overhead inflacionário; pattern empírico N=7
  contraria (precedente forte 7 sub-passos sequenciais
  sem reformulação).
- **Rejeitada**: precedente empírico forte (N=7
  ultrapassa limiar formalização N=3-4 amplamente).

### Alt B — Actualizar L0 apenas para variants Content novos
- **Pro**: refinos aditivos a variants existentes não
  inflacionam.
- **Con**: P217+P220+P224 introduziram 5 variants Content
  novas (`Columns`, `Colbreak`, `GridHeader`, `GridFooter`,
  `GridCell`) sem L0 → pattern N=4 viola Alt B.
- **Rejeitada**: pattern empírico N=4 já viola.

### Alt C — Actualizar L0 apenas para módulos L1 novos
- **Pro**: estrutura modular fica documentada formalmente.
- **Con**: P224 `grid_placement.rs` introduzido sem L0 →
  N=1 já viola.
- **Rejeitada**: pattern empírico N=1 já viola.

---

## Cross-references

- **P217 + P218 + P219 + P220 + P222 + P223 + P224** — 7
  aplicações cumulativas.
- **P224 spec C6** — divergência consciente Opção α → γ
  (terceira divergência cumulativa em spec; pattern N=3
  divergências conscientes cumulativas em P218+P219+P224).
- **§3.0terdecies P225** — pattern N=7 registado pré-promoção
  formal.
- **§3.0quaterdecies P226** — promoção formal a ADR meta
  documental.
- **ADR-0033** — paridade observable preservada (L0
  documenta semantic; refactors aditivos preservam).
- **ADR-0034** — diagnóstico obrigatório precedente
  metadocumental.
- **ADR-0065** — inventariar primeiro precedente
  metadocumental.

---

## Promoção [HISTÓRICO P226 — preservado per padrão P204H+]

ADR-0080 transita PROPOSTO → **EM VIGOR** quando:

1. **N=8+ aplicação cumulativa** sem decisão explícita
   contrária (i.e., humano não fixou Opção α em sub-passo
   futuro materializado).
2. **OU passo administrativo XS dedicado** para promoção
   (humano fixa).

ADR-0080 transita PROPOSTO → **REJEITADA** se:
- Humano fixa decisão explícita de actualizar L0 em
  sub-passos futuros pós-N=7.

**Status PROPOSTO** — autorização arquitectural concedida
em princípio; promoção EM VIGOR em passo futuro.

---

## Promoção executada — P229 (2026-05-13)

**Status**: PROPOSTO → **EM VIGOR**.

**Critério satisfeito** (dupla satisfação dos critérios
§"Promoção" histórica P226):

- ✓ **N=8+ aplicação cumulativa** atingido: **N=9**
  (P217+P218+P219+P220+P222+P223+P224 pre-formalização +
  **P227+P228** pós-formalização). N=9 ultrapassa N=8+
  critério.
- ✓ **Sem decisão explícita contrária** em 9 aplicações
  cumulativas (zero Opção α humana fixada em sub-passo
  materializado pós-P217).
- ✓ **Passo administrativo XS dedicado** fixado humano
  (decisão literal pós-P228 §8 "P229 administrativo").

**Pattern emergente "L0 minimal para refactors aditivos
pós-M9c" formalizado como regra metodológica EM VIGOR**.
Refactors aditivos pós-P229 seguirão automaticamente este
padrão (Opção γ "L0 não tocado por defeito"); divergências
exigem decisão explícita humana fixada em spec individual.

**Trajectória cumulativa**:
- P226 — formalização PROPOSTO (N=7 validação cumulativa
  pre-formal).
- P227 — primeira aplicação real pós-formalização (N=7 → 8).
- P228 — segunda aplicação real pós-formalização (N=8 → 9).
- **P229 — promoção EM VIGOR formal**.

**Patterns emergentes secundários** preservados como
candidatos a ADRs meta separadas (passos administrativos
XS dedicados futuros se humano priorizar; anti-inflação
via singularidade P229):
- "Field armazenado semantic adiada" N=5.
- "Refino aditivo paralelo entre variants irmãos" N=2.
- "Anti-inflação por aproveitamento de tipos existentes"
  N=1.
- "Divergência factual material `Pxxx.div-N`" N=3.
- "Encerramento Fase Layout pós-M9c" N=2.
- "Abertura Fase Layout pós-M9c" N=1.
- "ADR PROPOSTO com materialização parcial graded" N=1
  estendido (pós-P229: 2 ADRs PROPOSTAS — ADR-0066 +
  ADR-0079).

**Status pós-P229**: `EM VIGOR` — regra metodológica
formal cristalina para refactors aditivos pós-M9c.

---

## Próximos passos

ADR-0080 PROPOSTO **não compromete trabalho subsequente**.
Pattern continua estabelecido empíricamente em P226+
(sub-passos Fase 5 Layout candidata identificados em
ADR-0079 PROPOSTO; cada sub-passo materialização decidirá
Opção γ por defeito ou Opção α por excepção).

Promoção formal a `EM VIGOR` candidato Caminho XS
administrativo dedicado se humano priorizar pós-N=8 ou
imediato (passo limpo sem código).

---

## Excepção P240 — features runtime + walk integration

**Data**: 2026-05-14.

**Primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-promoção P229**. ADR-0080 §"Escopo"
aplica-se a refactors aditivos a variants/fields existentes.
**Features runtime + walk integration** (novos Content
variants + novos ElementPayload variants + funções fixpoint
novas + Introspector trait methods novos) merecem L0 tocado
partial.

**P240 materializa M7+1 (M9d primeiro sub-passo; ADR-0081
PROPOSTO P239 Opção γ)**:
- `Content::StateDisplay { key, callback: Option<Func> }`
  variant novo.
- `ElementPayload::StateDisplay { key, callback }` variant
  novo.
- `ElementKind::StateDisplay` variant novo.
- `apply_state_displays` fixpoint function nova (paralelo
  absoluto `apply_state_funcs` P191B).
- `Introspector::state_display_value` trait method novo +
  impl em `TagIntrospector` + `CountingIntrospector`.
- `TagIntrospector.state_displays:
  HashMap<(String, Location), Content>` storage novo.
- `native_state_display(key, [callback])` stdlib func nova.
- Walk integration layout-time arm
  `Content::StateDisplay`.

**4-5 entidades novas cumulativas** — não-trivial estructural;
paridade conceitual hipótese P236 spec original (que foi
rejeitada empíricamente pós-`P236.div-1`; mas pattern
proposto naquele spec aplica-se aqui: features runtime
novas merecem L0 tocado).

**L0 tocado partial P240** (3 ficheiros):
- `00_nucleo/prompts/entities/content.md` — bloco
  `Content::StateDisplay` documentado.
- `00_nucleo/prompts/rules/stdlib.md` — bloco
  `state_display(key, [callback])` documentado.
- `00_nucleo/prompts/rules/introspect.md` — bloco
  `apply_state_displays` + `Introspector::state_display_value`
  documented.

**Pattern emergente "L0 tocado para features runtime novas
+ walk integration" N=1 inaugurado P240** — primeira
aplicação real (P236 spec original hipotetizou; rejeitada
empíricamente pós-divergência).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
preservado** mas **não-incrementa P240** (excepção
justificada documentada formalmente acima).

**Critério para excepções futuras** (cristalizado P240):
sub-passo merece L0 tocado partial se introduz:
- 4+ entidades novas cumulativas (Content variant +
  ElementPayload variant + fixpoint function + trait
  method + storage + stdlib func + walk arm).
- Walk integration arquitectural (não apenas eval-time
  wrapper).
- Pipeline restructuring (ainda que paridade pattern
  existente).
- Feature M-fase nova (M7+/M9d+ etc).

Sub-passo NÃO merece L0 tocado se é:
- Refactor aditivo +1 field/variant cosmético.
- Eval-time wrapper trivial paralelo existing.
- Renderização Z-order extension paridade existing.

---

## Excepção P241 — segunda aplicação cumulativa (N=2)

**Data**: 2026-05-14.

**Segunda excepção justificada à aplicação automática ADR-0080
EM VIGOR pós-P229** (paralela absoluta P240). Pattern emergente
"L0 tocado para features runtime novas + walk integration"
N=1 → **2 cumulativo** (P240 + P241).

P241 materializa M7+2 (M9d segundo sub-passo; ADR-0081
IMPLEMENTADO parcial 1/5 → 2/5) — paralelo absoluto P240 M7+1
substituindo `state_display` por `counter_display`:

- `Content::CounterDisplayCallback { key, callback }` variant
  novo (distinto de `Content::CounterDisplay { kind }` legacy
  single-pass que coexiste).
- `ElementPayload::CounterDisplay { key, callback }` variant
  novo.
- `ElementKind::CounterDisplay` variant novo.
- `apply_counter_displays` fixpoint function nova (paralelo
  absoluto `apply_state_displays` P240).
- `Introspector::counter_display_value` trait method novo +
  impl em `TagIntrospector` + `CountingIntrospector` adapter.
- `TagIntrospector.counter_displays:
  HashMap<(String, Location), Content>` storage novo.
- `native_counter_display(key, [callback])` stdlib func nova.
- Walk integration layout-time arm
  `Content::CounterDisplayCallback`.

**~6 entidades novas cumulativas** — não-trivial estructural;
paridade absoluta P240; pattern de excepção justificada
cristalizado. **L0 partial tocado** (3 ficheiros paralelos P240):
- `00_nucleo/prompts/entities/content.md` — bloco
  `Content::CounterDisplayCallback` documentado.
- `00_nucleo/prompts/rules/stdlib.md` — bloco
  `counter_display(key, [callback])` documentado.
- `00_nucleo/prompts/rules/introspect.md` — bloco
  `apply_counter_displays` + `Introspector::counter_display_value`
  documentado.

**Pattern "L0 tocado para features runtime novas + walk
integration" N=1 → 2 cumulativo** (P240 + P241). N=2 atinge
limiar formalização N=3-4 marginal; promoção a sub-categoria
ADR-0080 candidata se N=3 atinge em sub-passo M7+ futuro.

**Critério §"Excepção P240" preservado literal**: ambas as
aplicações N=2 satisfazem critério "4+ entidades novas cumulativas
+ walk integration arquitectural + pipeline restructuring +
feature M-fase nova". Pattern cristalizado.

---

## Excepção P242 — terceira aplicação cumulativa (N=3, sub-categoria geometry/exporter)

**Data**: 2026-05-14.

**Terceira excepção justificada à aplicação automática ADR-0080
EM VIGOR pós-P229**. **Sub-categoria diferente** de P240/P241
(walk-time runtime integration): P242 é **"L0 tocado para
geometry/exporter infrastructure"** — distinta semânticamente
mas justificada pelo mesmo critério estructural (4+ entidades
novas + cross-camada L1/L3).

P242 materializa M7+5 (M9d terceiro sub-passo; ADR-0081
IMPLEMENTADO parcial 2/5 → 3/5):

- `Corners<T>` tipo entity novo em `01_core/src/entities/corners.rs`
  (paralelo `Sides<T>`; sub-padrão #14 "Tipo entity em ficheiro
  próprio" N=5 → 6 cumulativo).
- `ShapeKind::RoundedRect { radii: Corners<Length> }` variant
  novo em `entities/geometry.rs`.
- **Refino tipo** `Content::Block.radius` + `Content::Boxed.radius`
  `Option<Length>` → `Corners<Length>` (per-corner; promoção real
  de scope-out P231 graded).
- `extract_corners_length_value` helper novo em
  `rules/stdlib/layout.rs` (paralelo `extract_sides_lengths`).
- stdlib `block(radius:)` + `box(radius:)` aceitam Length uniforme
  OR Dict por canto.
- Layouter Block arm emite `FrameItem::Group { clip_mask:
  Some(ShapeKind::RoundedRect { radii }) }` quando `clip == true`.
- PDF exporter `emit_rounded_rect_ops` helper novo desenha
  Bezier 4 corners path em 5 sítios cross-arm.

**~6 entidades novas cumulativas + cross-camada L1/L3** —
estrutural; promoção real graded de scope-out P156G/H P231.

**L0 partial tocado P242** (4 ficheiros — sub-categoria
geometry/exporter distinta de P240/P241):
- `00_nucleo/prompts/entities/corners.md` (**ficheiro novo**).
- `00_nucleo/prompts/entities/geometry.md` — secção
  `ShapeKind::RoundedRect`.
- `00_nucleo/prompts/entities/content.md` — refino
  `Block.radius` + `Boxed.radius` + materialização clip semantic.
- `00_nucleo/prompts/infra/export.md` — secção rounded-rect
  clip path.

**Sub-padrão emergente "promoção real scope-out ADR-0054 graded"
N=1 inaugurado P242** — distinto de:
- Refino qualitativo (P156L Pad sides Length → Option<Length>).
- Refactor cosmético (P158C Figure.kind String → Option<String>).
- **Sub-categoria nova**: scope-out P156G/H P231 "semantic adiada"
  → semantic concreta P242 + render PDF real.

**Pattern "L0 tocado para features runtime + walk integration"**
N=2 preservado (P240+P241) — **NÃO incrementa P242** por ser
sub-categoria diferente. Pattern emergente cumulativo total
"L0 tocado pós-P229 (sub-categorias)": **N=3 cumulativo** (P240,
P241, P242) com 2 sub-categorias formalizadas.

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8 preservado**
mas **não-incrementa P242** (excepção justificada documentada
formalmente acima).

---

## Excepção P243 — quarta aplicação cumulativa (N=4, sub-categoria nova "Layouter internal refactor")

**Data**: 2026-05-14.

**Quarta excepção justificada à aplicação automática ADR-0080
EM VIGOR pós-P229**. **Sub-categoria diferente** de P240/P241
(walk-time runtime) E de P242 (geometry/exporter): P243 é
**"Layouter internal refactor"** — distinta semanticamente por
tocar estrutura interna do Layouter L1 sem walk-time integration
nem cross-camada L1/L3.

P243 materializa M7+3 fase (a) (M9d quarto sub-passo; ADR-0081
IMPLEMENTADO parcial 3/5 → 4/5):

- Extensão `Regions` struct: `backlog: Vec<Region>` + `last:
  Option<Region>` fields novos (paridade vanilla literal).
- `Regions::advance` method novo (fase (a): retorna None;
  fase (b) consumirá backlog).
- Promoção real ≥3 scope-outs via `regions.current.width`
  save/restore:
  - `Pad.right` scope-out P156C → semantic real P243.
  - `Block.width` semantic adiada P156G → semantic real P243.
  - `Boxed.width` semantic adiada P156H → semantic real P243.

**~5 entidades novas cumulativas em L1 interno** (2 fields novos
+ 1 method novo + 3 scope-out promoções).

**Audit C1 P243 finding material**: spec hipotetizou refactor
profundo cross-module L+ (5-7 fields migrar + ~30-50 sítios
adaptação). Reality empírica: refactor field-agregation já feito
em P216A/P216B; P243 reduz para extensão de `Regions` existente.
Magnitude real M (~2-3h) face L+ hipotetizado. **Sem `P243.div-N`**
— paridade lição N=6 cumulativo precedente.

**L0 partial tocado P243** (2 ficheiros — sub-categoria nova):
- `00_nucleo/prompts/entities/region.md` — extensão `Regions`
  backlog/last/advance + sub-padrão promoção real scope-out N=2.
- `00_nucleo/prompts/entities/content.md` — secção promoção
  scope-outs Pad.right + Block.width + Boxed.width.

**Pattern emergente total "L0 tocado pós-P229 (sub-categorias)"**
N=3 → **4 cumulativo** com **3 sub-categorias formalizadas**:
- walk-time runtime (N=2 P240+P241).
- geometry/exporter (N=1 P242).
- **Layouter internal refactor (N=1 P243)** ← inaugurada.

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=1 →
**2 cumulativo** (P242 radius/clip + **P243 multi-region attrs
Pad.right + Block.width + Boxed.width**). Atinge limiar
formalização N=2 — candidato a ADR meta passo administrativo XS.

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8 preservado**
mas **não-incrementa P243** (excepção justificada documentada
formalmente acima).

---

## Lição refinada P244 — Audit C1 deve grep variants `Content::*` candidatas antes de assumir ausência (2026-05-14)

**Refinamento procedural** do padrão "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=6 → **7 cumulativo** (P237 + P238
reescrito + P240 + P241 + P242 + P243 + **P244**). Distintamente
em P244, audit C1 **é** o substantivo material do passo (não
preâmbulo a materialização subsequente).

**Origem da lição**: P243 audit C1 capturou parcialmente o
estado factual:
- ✓ **Detectou** que `Regions` já existia (P216A/P216B) →
  Decisão 2 spec P243 corrigida: "Migração field-by-field já
  feita P216A/P216B".
- ✗ **Não detectou** que `Content::Columns` (P217) +
  `Content::Colbreak` (P220) + `native_columns` (P218) +
  `native_colbreak` (P220) também já existiam, com ADR-0078
  IMPLEMENTADO + ADR-0061 IMPLEMENTADO + DEBT-56 ENCERRADO em
  P221.

Resultado: Decisão 5 spec P243 ("Sem `Content::Columns`/
`Colbreak` em P243 — fase (b) DEBT-56 pendente") foi
internamente coerente com o spec mas assumiu base factual
incorrecta. P244 reconcilia via reconhecimento Linha A
pré-existente.

**Procedimento recomendado pós-P244** (refino procedural lição
N=7 cumulativo):

1. **Identificar variants candidatas mencionadas no spec**
   (e.g. `Content::Columns`, `Content::Colbreak`,
   `Content::StateDisplay`, etc.).
2. **`grep -n "VariantName" 01_core/src/entities/content.rs`**
   antes de assumir ausência. Pattern recomendado:
   ```
   grep -n "Content::FOO\|Content::BAR" 01_core/src/entities/content.rs
   grep -rn "native_foo\|native_bar" 01_core/src/rules/stdlib/
   grep "DEBT-XX\|ADR-XXXX" 00_nucleo/DEBT.md 00_nucleo/adr/
   ```
3. **Se variant existe** → ajustar spec ou criar `Pxxx.div-N`
   conforme magnitude da divergência:
   - Ajuste trivial signature/naming: sem div-N (paridade lição
     N=5-6 cumulativo precedente).
   - Divergência estructural material: `Pxxx.div-N` formal
     bloqueando para decisão humana.
4. **Se variant ausente** → prosseguir com materialização.
5. **Audit C1 deve grep DEBT.md + ADRs** para confirmar status
   factual cumulativo (ENCERRADO/IMPLEMENTADO) — não confiar
   no spec sobre estado de DEBTs/ADRs sem verificação empírica.

**Sub-padrão "Reconciliação documental pós-divergência factual
planeamento vs materialização" N=1 inaugurado P244**.
Candidato a formalização N=3-4 futuro.

**Pattern "Passo administrativo XS"** N=5 → **6 cumulativo**
(P156A + P156K + ADR-0062-create + P160A + P238 + **P244**).
**Atinge limiar formalização sólido N≥4-6**; candidato a ADR
meta dedicada em passo administrativo XS futuro
(não-reservado per política P158).

**Aplicação da lição refinada**: passos administrativos
subsequentes que envolvem audit C1 sobre material pré-existente
devem aplicar passos 1-5 acima literalmente. Patamar empírico
N=7 cumulativo pós-P244 ultrapassa largamente limiar N=4
sólido; lição refinada é metodológicamente robusta.

---

## Lição refinada P245 — Audit C1 deve grep fields/arms já implementados antes de assumir trabalho original (2026-05-14)

**Refinamento procedural** do padrão "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=7 → **8 cumulativo** (P237 +
P238 reescrito + P240 + P241 + P242 + P243 + P244 + **P245**).
Extensão da lição P244 ("grep variants `Content::*` candidatas
antes de assumir ausência") para **"grep fields/arms já
implementados antes de assumir trabalho original"**.

**Origem da lição P245**: P245 audit C1 capturou que P223 já
tinha armazenado `Content::Place.float` + `Content::Place.clearance`
**mas** o Layouter consumer em `mod.rs:916` ainda **ignorava
literal** os fields (`float: _, clearance: _`). Sem este audit,
P245 poderia ter assumido implementação original (criar tudo
do zero) ou que tudo já estava feito (skip total). Audit C1
identificou o **estado intermediário**: storage P223 ✓ + consumer
P223 graded (ignorado) → P245 promove consumer a real.

**Sub-categoria nova ADR-0080 "Layouter internal refactor
(semantic activation)"** N=1 → **2 cumulativo** (P243 extensão
Regions + scope-outs promovidos; **P245 Place float semantic
activa**). Distinta de:
- Walk-time runtime (P240+P241).
- Geometry/exporter (P242).
- Layouter internal refactor (P243 — extensão + scope-outs).
- **Layouter internal refactor (semantic activation) — P245**
  consumer real de field graded pré-existente.

**Procedimento recomendado pós-P245** (refino procedural
lição N=8 cumulativo):

1. Identificar fields/methods candidatos mencionados no spec.
2. `grep -n "Content::FOO { ... field: _" 01_core/src/rules/layout/`
   para detectar consumer graded (field ignorado).
3. `grep -n "Field-name armazenado mas\|semantic adiada" 01_core/src/entities/`
   para detectar storage graded P223-style.
4. Se field existe + consumer ignora → P245-style "promoção
   graded → real semantic activação consumer".
5. Se field ausente → trabalho original.
6. Se field existe + consumer activo → P244-style reconciliação.

**Sub-padrão "Promoção graded → real semantic activação
consumer" N=1 inaugurado P245**. Candidato a formalização
N=3-4 futuro.

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=4 cumulativo
preservado P245 (sem novo L0 partial tocado em P245 — paridade
P243 Layouter internal refactor; sub-categoria 4ª distinta
mas L0 não-incrementado).

**Aplicação da lição refinada**: passos materialização
subsequentes que envolvem field graded pré-existente devem
aplicar passos 1-6 acima. Patamar empírico N=8 cumulativo
pós-P245 sólido — lição refinada metodológicamente robusta.

---

## Lição refinada P246 — Audit C1 deve mapear empíricamente distribuição de usos por sub-módulo antes de fixar arquitectura de migração (2026-05-14)

**Refinamento procedural** do padrão "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=8 → **9 cumulativo** (P237 + P238
reescrito + P240 + P241 + P242 + P243 + P244 + P245 + **P246**).
Extensão da lição P245 ("grep fields/arms já implementados
antes de assumir trabalho original") para **"mapear
empíricamente distribuição de usos por sub-módulo antes de
fixar arquitectura de migração"**.

**Origem da lição P246**: P246 spec previa 4 fields a migrar
com distribuição cross-módulo desconhecida. Audit C1 P246
revelou empíricamente:
- **Save/restore**: único em `grid.rs:361-382` (8 sítios; 4
  save + 4 write).
- **Reads**: apenas em `placement.rs` (4 read sites).
- **Sem `push`/`pop` API** em Regions pré-existente.
- **Sem `regions.cell`** pré-existente.

Escopo reduzido vs hipótese spec (~12-15 sítios; trivial
migração) — Decisão 1 fixada Opção B (snapshot `cell: Option<Region>`)
pós-audit empírico em vez de Opção A (push/pop stack) ou Opção
C (preservar legacy + API paralela).

**Sub-categoria nova ADR-0080 "Layouter consumer migration via
API wrapper"** N=1 inaugurada P246 — distinta de:
- Walk-time runtime (P240+P241).
- Geometry/exporter (P242).
- Layouter internal refactor (P243 — extensão + scope-outs).
- Layouter internal refactor (semantic activation) (P245).
- **Layouter consumer migration via API wrapper (P246)** —
  migração field-by-field Layouter privado → API entity-side
  para reduzir acoplamento.

**Procedimento recomendado pós-P246** (refino procedural
lição N=9 cumulativo):

1. Identificar fields/methods candidatos mencionados no spec.
2. `grep -rn "field_name" 01_core/src/rules/module_dir/` para
   mapear distribuição cross-submodule.
3. Classificar usos por categoria:
   - Save/restore (entrada/saída de contexto).
   - Write (atribuições durante contexto activo).
   - Read (consumo do contexto).
4. Contar sítios por categoria + sub-módulo.
5. Se ≤10-15 usos cumulativos → migração trivial (Opção
   minimal entity-side API).
6. Se >20 usos cumulativos → considerar migração dual
   (preserve fields legacy + add API paralela; deprecação
   gradual passos futuros).
7. Fixar Decisão arquitectural pós-audit (não pré-audit).

**Sub-padrão "Layouter consumer migration via API wrapper"
N=1 inaugurado P246**. Candidato a formalização N=3-4
futuro.

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=4 cumulativo
preservado P246 + **+1 sub-categoria N=1** ("Layouter consumer
migration via API wrapper") = **5 sub-categorias formalizadas
cumulativo** (mas L0 tocado N=4 preservado — paridade P243+
sub-categoria; P245 não tocou L0; **P246 toca L0 partial via
extensão `region.md`** N=4 → **5 cumulativo**).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
preservado** mas **não-incrementa P246** (excepção justificada
sub-categoria nova documentada formalmente acima).

**Aplicação da lição refinada**: passos materialização
subsequentes que envolvem refactor cross-submodule devem
aplicar passos 1-7 acima. Patamar empírico N=9 cumulativo
pós-P246 sólido — lição refinada metodológicamente robusta.

---

## Lição refinada P247 — N=9 → N=10 cumulativo

P247 refina ainda mais o pattern N=9 P246 ("mapear empíricamente
distribuição de usos por sub-módulo antes de fixar arquitectura
de migração"):

**Lição N=10 cumulativa P247**: "mapear scope-outs declarados
historicamente vs estado real materializado antes de assumir
ausência" (refino directo do pattern P243→P244 onde scope-outs
declarados estavam factualmente materializados).

**Conteúdo refinado**:

- Scope-outs declarados em L0 ou ADR não são garantia de
  ausência real — código pode tê-los materializado em passos
  intermédios sem actualizar L0.
- Audit C1 §2 deve **mapear empíricamente** (grep, sed, leitura
  de arms layouter) o estado actual antes de assumir ausência
  pré-implementação.
- Cenários A/B/C documentados pré-audit em spec são preliminares;
  decisão final fixa **pós-audit** §2.9.
- Sub-padrão "verificação empírica refuta hipótese spec" N=2
  cumulativo (P243→P244 outset declarado-ausente factualmente
  materializado parcialmente; **P247 outset declarado-armazenado
  factualmente zero-uso em Layouter** — paridade inversa do
  pattern P244 mas mesmo lemma metodológico).

**Sub-padrão emergente "promoção real scope-out ADR-0054 graded"
N=2 → N=3 cumulativo P247**:

- N=1 inaugurado P242 (radius + clip materializados).
- N=2 cumulativo P242 agregado (radius + clip num passo único).
- **N=3 cumulativo P247** (outset semantic + fill + stroke num
  passo único — pattern "agregar promoções" N=1 inaugurado).
- Contando granular: **5 promoções reais cumulativas** (radius +
  clip + outset + fill + stroke). Limiar conceptual sólido para
  ADR meta candidata futura (XS admin; "Promoções reais
  scope-outs ADR-0054 graded" formalização N=5).

**Sub-padrão "agregar promoções scope-outs cosméticos visuais"
N=1 inaugurado P247**:

- Cumprimento dos critérios:
  - Magnitude controlada M-L (não L+).
  - Coesão semantic forte (3 atributos visuais ortogonais).
  - Tests cross-multiplicados naturalmente.
- Candidato a formalização N=3-4 se outras agregações ocorrerem
  futuro (hipóteses: 4 scope-outs Block restantes — spacing +
  above + below + sticky — agregar em passo único S-M paridade
  P247).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=5
cumulativo P246 → **N=6 cumulativo P247** (P247 toca L0 via
extensão `entities/content.md` §"Promoção scope-outs Block/Boxed
fill+stroke+outset — Passo 247"; hash propagado).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
preservado** **incrementa P247** N=8 → **N=9 cumulativo** (P247
toca L0 mas refino aditivo paralelo paridade P242 + sub-padrão
emergente; L0 minimal preservado).

---

## Lição refinada P248 — N=10 → N=11 cumulativo

P248 refina o pattern N=10 P247 ("mapear scope-outs declarados
historicamente vs estado real materializado antes de assumir
ausência"):

**Lição N=11 cumulativa P248**: "mapear pontos de check overflow
existentes antes de adicionar novos checks duplicados" (audit
C1 §2.1 — 9 sítios de `new_page()` mapeados antes de adicionar
medição antecipada em arms específicos).

**Conteúdo refinado**:

- Mecanismo de page break já existente (`cursor.rs:127` + 8
  sítios adicionais) é o canónico; P248 acrescenta checks
  antecipados em arms específicos (Block) sem substituir.
- Audit C1 §2.4 confirmou `measure_content_constrained` puro
  (sem side-effects); reusado directamente em Block/Boxed/cell.
- Audit C1 §2.6 confirmou cell layout sem mecanismo embrionário
  de overflow (zero refactor necessário; activação primeira vez).

**Sub-padrão emergente "Activação semantic real multi-consumer
via mecanismo comum" N=1 inaugurado P248**:

- 3 activações granulares (Block.breakable + Boxed.height +
  TableCell overflow) usam mecanismo partilhado
  (`measure_content_constrained` puro pré-existente).
- Magnitude L controlada (não L+) porque mecanismo comum reduz
  custo per-activação.
- Tests cross-multiplicados naturalmente (cross-attribute).
- Candidato a formalização N=3-4 futuro.

**Sub-padrão "promoção graded → real semantic activação consumer"
N=1 → N=2 cumulativo P248**:

- N=1 inaugurado P245 (Place float real).
- **N=2 cumulativo P248** (3 sub-activações graded → real em
  agregação).
- Granular: N=4 contando 3 sub-activações P248 + 1 P245.

**Sub-padrão emergente "promoção real scope-out ADR-0054 graded"
granular N=5 → **N=8 cumulativo** P248**:

- N=5 cumulativo P247 (radius + clip + outset + fill + stroke).
- **N=8 cumulativo P248** (+ breakable + height + cell_overflow).
- Limiar conceptual sólido para ADR meta candidata futura XS
  admin (N≥6 patamar atingido P248).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=6 cumulativo
P247 → **N=7 cumulativo P248** (P248 toca L0 via extensão
`entities/content.md` §"Promoção graded → real semantic
Block.breakable + Boxed.height + TableCell overflow — Passo
248"; hash propagado `9f03e1a8`).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=9
preservado** **incrementa P248** N=9 → **N=10 cumulativo** (P248
toca L0 mas refino documentar 3 activações via secção dedicada;
L0 minimal preservado per Opção β).

---

## Lição refinada P249 — N=11 → N=12 cumulativo

P249 refina o pattern N=11 P248 ("mapear pontos de check
overflow existentes antes de adicionar novos checks duplicados"):

**Lição N=12 cumulativa P249**: "ADR meta administrativo XS
exige audit empírico das N≥4 aplicações concretas antes de
formalizar pattern".

**Conteúdo refinado**:

- Antes de criar ADR meta (ex: ADR-0082 P249 promoções reais),
  audit C1 deve confirmar empíricamente cada uma das N
  aplicações cumulativas (audit §2.1 P249 confirmou 8
  aplicações: P242 ×2 + P247 ×3 + P248 ×3).
- Audit C1 deve confirmar próximo número ADR livre (audit §2.3
  P249 revelou ADR-0067 ocupada → `P249.div-2` formal +
  ADR-0082 escolhida como próximo slot).
- Audit C1 deve confirmar precedentes ADR meta estruturais
  (template canónico ADR-0065/ADR-0080).
- Audit C1 deve confirmar ADR-0054 estado actual antes de
  anotar refino interno secção nova.

**Sub-padrão emergente "ADR meta formalizar pattern N≥4
cumulativo" N=2 → N=3 cumulativo P249**:

- N=1 (P156K Smart→Option N=6 → ADR-0064).
- N=1 (P156K inventariar primeiro N=5 → ADR-0065).
- N=2 cumulativo (P234 L0 minimal P217-P224 N=7 → ADR-0080).
- **N=3 cumulativo P249** (promoções reais scope-outs ADR-0054
  graded N=8 → ADR-0082 PROPOSTO).

Limiar formalização interno N=3 atingido P249. Pattern
metodológico sólido.

**Pattern "Passo administrativo XS"** N=6 → **N=7 cumulativo
P249** (P156A historiograma + P156K ADRs meta + ADR-0062-create
+ P160A + P238 + P244 + **P249**). Limiar formalização N=6
ultrapassado; pattern sólido reforçado.

**Promoções reais scope-outs ADR-0054 graded** granular **N=8
preservado P249** (P249 administrativo XS não materializa nova
promoção; apenas formaliza pattern via ADR-0082).

**Cross-reference ADR-0082 PROPOSTO** (P249 administrativo XS).

---

## Lição refinada P250 — N=12 → N=13 cumulativo

P250 refina o pattern N=12 P249 ("ADR meta administrativo XS
exige audit empírico das N≥4 aplicações concretas antes de
formalizar pattern"):

**Lição N=13 cumulativa P250**: "refactor cross-arm Sequence
consumer exige audit de todos os patterns de iteração existentes
antes de migrar a peekable".

**Conteúdo refinado**:

- Audit C1 §2.2 P250 confirmou empíricamente 2 Sequence consumers
  em mod.rs (layout + measure) + 2 sítios em helpers.rs (não-
  bloqueantes; traversal puro). Decisão pós-audit: refactor
  apenas mod.rs:478 layout consumer (measure não precisa de
  neighbour context — spacing colapsa para zero em medição
  estática per ADR-0054 graded).
- Audit C1 §2.3 confirmou zero peekable usage prévio no Layouter
  — P250 inaugura pattern.
- Audit C1 §2.4 confirmou vanilla algorithm reference
  (`lab/typst-original/crates/typst-library/src/layout/
  container.rs`): `above`/`below` fallback `spacing`; `max(prev.
  below, curr.above)` collapse; sticky default false.

**Sub-padrão emergente "Refactor Sequence consumer cross-arm
via peekable + neighbour context" N=1 inaugurado P250**:

- Pattern novo (look-ahead 1-block via `iter.peek()` no
  Layouter).
- Magnitude L controlada (1 arm consumer + Layouter fields +
  arm Block consume).
- Candidato a formalização N=3-4 futuro (hipóteses: pagebreak
  weak collapse com weak adjacent; HSpace/VSpace weak collapse
  genérico P156D refino).

**Sub-padrão "Aplicação citante ADR-0082 PROPOSTO" N=0 → N=1
P250**:

- **Primeira aplicação concreta a citar ADR-0082 explicitamente**
  (criada P249 administrativo XS).
- Os 4 critérios operacionais ADR-0082 verificados:
  1. Storage prévio ✓ (4 fields scope-out P156G).
  2. Consumer Layouter pre-promoção graded ✓ (rejeitados
     `native_block` erro hard).
  3. Paridade vanilla referência empírica ✓ (audit §2.4).
  4. Backward compat literal ✓ (sentinela
     `p250_block_defaults_preserva_output_pre_p250`).
- Validação ADR-0082 N=1 citante — primeiro passo dum sequente
  N=3 para promoção EM VIGOR (paridade ADR-0065 P156K validada
  pós-P156J + P157A + P157B sequente).

**Sub-padrão "promoção real scope-out ADR-0054 graded" granular
N=8 → N=12 cumulativo P250** (P242 ×2 + P247 ×3 + P248 ×3 +
P250 ×4). Marco interno: primeiro variant Content com **100%
dos scope-outs originais P156G fechados** — Block A.4
COMPLETO 10/10.

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=7
cumulativo P248 → **N=8 cumulativo P250** (P250 toca L0 via
extensão `entities/content.md` §"Promoção Block spacing +
above + below + sticky — Passo 250"; hash propagado
`418bbbfb`).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=10
preservado** **incrementa P250** N=10 → **N=11 cumulativo**
(P250 toca L0 mas refino documentar 4 fields + algoritmo
collapse + sticky lookahead; L0 minimal preservado per Opção β).

**Cross-reference ADR-0082 PROPOSTO** (citado primeira vez P250
explicitamente).

---

## Lição refinada P251 — N=13 → N=14 cumulativo

P251 refina o pattern N=13 P250 ("refactor cross-arm Sequence
consumer exige audit de todos os patterns de iteração existentes
antes de migrar a peekable"):

**Lição N=14 cumulativa P251**: "audit C1 deve confirmar
localidade pos.y antes de fixar abordagem γ-Items vs γ-Content
para slicing".

**Conteúdo refinado**:

- Audit C1 §2.1 P251 confirmou empíricamente que
  `layout_sub_frame_with_width` retorna items com `pos.y` **local
  ao sub-frame** (comentário literal mod.rs). Decisão fixa pós-
  audit: **γ-Items** (slice por threshold trivial via filter +
  rebase) face γ-Content (re-layout tail Content, magnitude L+
  ~10-12h).
- Audit C1 §2.4 confirmou 6 variants FrameItem
  (Text/Line/Glyph/Image/Shape/Group); `Line` usa `start`/`end`
  (não `pos`) — rebase trata simétricamente.
- Audit C1 §2.6 confirmou pattern P245 `DeferredFloat` reusável
  directo para `DeferredCellTail`.

**Sub-padrão emergente "Slice frame items at height via filter
+ rebase pos.y" N=1 inaugurado P251**:

- Pattern novo (primeira aplicação γ-Items no Layouter).
- Função pura `slice_frame_items_at_height(items, threshold)
  -> (head, tail)` em módulo dedicado.
- Magnitude L controlada (não L+).
- Candidato a formalização N=3-4 futuro (hipóteses: column flow
  DEBT-56 multi-region; pagination overflow generic outros
  variants Content).

**Sub-padrão "DeferredX buffer + flush em new_page" N=1 → N=2
cumulativo P251**:

- N=1 inaugurado P245 (`DeferredFloat` + `floats_pending` +
  `flush_pending_floats`).
- **N=2 cumulativo P251** (`DeferredCellTail` +
  `pending_cell_tails` + `flush_pending_cell_tails`).
- Paridade arquitectural directa; pattern emergente consolidado.

**Sub-padrão "Aplicação citante ADR-0082 PROPOSTO" N=1 → N=2
cumulativo P251**:

- N=1 P250 (Block.spacing/above/below/sticky).
- **N=2 P251** (TableCell row break real cell-level).
- N=3 candidato pós-P252 (Boxed stroke-overhang) → **promoção
  ADR-0082 → EM VIGOR humana possível**.

**Sub-padrão "promoção real scope-out ADR-0054 graded" granular
N=12 → N=13 cumulativo P251** (P242 ×2 + P247 ×3 + P248 ×3 +
P250 ×4 + **P251 ×1**: TableCell.body overflow row break real).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=8 cumulativo
P250 → **N=9 cumulativo P251** (P251 toca L0 via extensão
`entities/region.md` §"Anotação cumulativa P251 — `pending_cell_
tails` buffer + flush em new_page"; hash propagado `6eec928d`).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=11
preservado** **incrementa P251** N=11 → **N=12 cumulativo**
(P251 toca L0 mas refino documentar `pending_cell_tails` field
+ `DeferredCellTail` struct + flush method; L0 minimal preservado
per Opção β).

**Cross-reference ADR-0082 PROPOSTO** (citado segunda vez P251).

---

## Lição refinada P252 — N=14 → N=15 cumulativo

P252 refina o pattern N=14 P251 ("audit C1 deve confirmar
localidade pos.y antes de fixar abordagem γ-Items vs γ-Content
para slicing"):

**Lição N=15 cumulativa P252**: "refactor cross-cutting de
entity primitivo exige mapa empírico exhaustive de todos os
construtores literais antes de modificar struct".

**Conteúdo refinado**:

- Audit C1 §2.1 P252 mapeou empíricamente ~42 construtores
  literais `Stroke` cross-cutting (entities/geometry + entities/
  content + rules/layout/mod + rules/layout/tests + rules/
  stdlib/shapes + rules/stdlib/mod + rules/stdlib/layout).
  Pre-spec audit como primeira aplicação cumulativa onde audit
  C1 antecede spec writing.
- Cascade replace_all guiado via `sed` pattern
  `Stroke { paint: ..., thickness: <num> } → Stroke { paint:
  ..., thickness: <num>, overhang: false }` cobriu ~38 sítios
  automaticamente; 4 sítios manualmente fixados (formatting
  multilinha + variable shorthand).
- Audit C1 §2.5 confirmou vanilla `overhang: true` default;
  divergência consciente cristalina default `false` no construtor
  Rust + `true` via stdlib `extract_stroke` (paridade user-
  facing preservada).

**Sub-padrão emergente "Refactor cross-cutting entity primitivo
com cascade replace_all guiado" N=1 inaugurado P252**:

- Pattern novo (entity primitivo cross-cutting; ~42 sítios
  cascade replace).
- Magnitude M controlada via sed pattern.
- Candidato a formalização N=3-4 futuro (hipóteses: `Color`
  extensão alpha channel; `Length` extensão font-relative
  units; `Sides<T>` refactor multi-arg).

**Sub-padrão "Aplicação citante ADR-0082 PROPOSTO" N=2 → N=3
cumulativo P252**:

- N=1 P250 (Block.spacing/above/below/sticky).
- N=2 P251 (TableCell row break real cell-level).
- **N=3 P252** (Boxed.stroke-overhang refactor cross-cutting).
- **N=3 limiar interno atingido** — paridade ADR-0065 P156K
  validada via P156J/P157A/P157B sequente. **Promoção ADR-0082
  PROPOSTO → EM VIGOR humana possível pós-P252** (decisão humana
  directa via passo administrativo XS).

**Sub-padrão "Backward compat literal estrita" N=1 → N=2
cumulativo P252**:

- N=1 P251 (cell tails preservam P248 clip para Fixed rows).
- **N=2 P252** (stroke overhang preserva bounds via default
  construtor Rust `false`).
- Pattern emergente: defaults zero-impact em construtor Rust
  low-level + paridade vanilla restaurada via stdlib parse.

**Sub-padrão "promoção real scope-out ADR-0054 graded" granular
N=13 → N=14 cumulativo P252** (P242 ×2 + P247 ×3 + P248 ×3 +
P250 ×4 + P251 ×1 + **P252 ×1**: Boxed.stroke-overhang real).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=9 cumulativo
P251 → **N=10 cumulativo P252** (P252 toca L0 via extensão
`entities/geometry.md` §"Default cristalino divergente P252";
hash propagado `7c1ba7a4`).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=12
preservado** **incrementa P252** N=12 → **N=13 cumulativo**
(P252 toca L0 refino documentar overhang field + divergência
consciente; L0 minimal preservado per Opção β).

**Cross-reference ADR-0082 PROPOSTO** (citado terceira vez P252
— N=3 limiar interno atingido).

---

## Lição refinada P253 — N=15 → N=16 cumulativo

P253 refina o pattern N=15 P252 ("refactor cross-cutting de
entity primitivo exige mapa empírico exhaustive de todos os
construtores literais antes de modificar struct"):

**Lição N=16 cumulativa P253**: "promoção ADR roadmap →
IMPLEMENTADO exige audit empírico cumulativo de sub-passos
materializados antes de declarar critério satisfeito".

**Conteúdo refinado**:

- Audit C1 §2.2 P253 confirmou empíricamente ~14 sub-passos
  granulares materializados P227-P252 + mapping para categorias
  A/B/C/D ADR-0079.
- Audit C1 §2.7 fixou Cenário A pós-empírico (contagem 11-13
  com gaps documentáveis → IMPLEMENTADO com scope-out formal
  C.2 multi-region completo + D.2-D.6 restantes).
- Paridade directa pattern ADR-0061 P221 IMPLEMENTADO precedente
  + ADR-0060 P155 IMPLEMENTADO sequente.

**Sub-padrão emergente "ADR Fase X roadmap → IMPLEMENTADO via
scope-out formal humano" N=2 → N=3 cumulativo P253**:

- N=1 ADR-0060 P155 (Fase 1 fechada; Fase 2/3 roadmap).
- N=2 ADR-0061 P221 (Fase 1+2+3 cumpridas; columns/colbreak
  roadmap).
- **N=3 ADR-0079 P253** (~14 sub-passos cumpridos cumulativamente;
  C.2 multi-region completo + D.2-D.6 roadmap).
- **Limiar formalização interno N=3 atingido** — candidato a
  ADR meta formalizar pattern futuro (paridade ADR-0065 →
  EM VIGOR sequente; paridade ADR-0080 → EM VIGOR sequente).

**Sub-padrão "Passo administrativo XS" N=7 → N=8 cumulativo P253**:

- N=1 P156A historiograma.
- N=2 P156K ADRs meta.
- N=3 ADR-0062-create.
- N=4 P160A.
- N=5 P238.
- N=6 P244.
- N=7 P249.
- **N=8 P253** (promoção ADR-0079 PROPOSTO → IMPLEMENTADO).
- Limiar formalização N=6 ultrapassado amplamente; pattern
  metodológico extremamente sólido.

**Sub-padrão "promoção real scope-out ADR-0054 graded" granular
N=14 preservado P253** (P253 administrativo XS não materializa
nova promoção; apenas formaliza promoção ADR-0079 roadmap →
IMPLEMENTADO).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=10
preservado P253 (P253 zero L0 tocado; paridade administrativo
XS estricto).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=13
preservado** P253 (P253 zero código + zero L0; nem aplica nem
viola ADR-0080).

**Cross-reference ADR-0079 IMPLEMENTADO** (citada terceira ADR
Fase X roadmap a transitar IMPLEMENTADO via scope-out formal
humano).

---

## Lição refinada P254 — N=16 → N=17 cumulativo

P254 refina o pattern N=16 P253 ("promoção ADR roadmap →
IMPLEMENTADO exige audit empírico cumulativo de sub-passos
materializados antes de declarar critério satisfeito"):

**Lição N=17 cumulativa P254**: "promoção ADR meta PROPOSTO →
EM VIGOR exige confirmação empírica de critério N=3 citantes
documentado literal no próprio ADR meta antes de promover".

**Conteúdo refinado**:

- Audit C1 §2.2 P254 confirmou empíricamente 3 entradas N=1/2/3
  documentadas literal em ADR-0082 §"Aplicações citantes":
  - N=1 P250 (Block.spacing/above/below/sticky).
  - N=2 P251 (TableCell row break real cell-level).
  - N=3 P252 (Boxed.stroke-overhang refactor cross-cutting).
- Sequente consecutivo P250 → P251 → P252 → P254 paridade
  ADR-0065 P156K via P156J/P157A/P157B sequente (validação
  retroactiva).
- Paridade arquitectural directa P229 ADR-0080 PROPOSTO→EM
  VIGOR (2026-05-13) — passo administrativo XS dedicado
  pós-validação empírica cumulativa.

**Sub-padrão emergente "ADR meta PROPOSTO → EM VIGOR via passo
admin XS dedicado" N=1 → N=2 cumulativo P254**:

- N=1 P229 ADR-0080 (pós-N=9 validação cumulativa applicações
  L0 minimal).
- **N=2 P254 ADR-0082** (pós-N=3 citantes consecutivos P250+
  P251+P252).
- **Limiar interno N=2 atingido** — candidato a sub-padrão
  emergente formalizar quando N=3-4 cumulativo futuro.

**Sub-padrão "Passo administrativo XS" N=8 → N=9 cumulativo
P254**:

- N=1 P156A historiograma.
- N=2 P156K ADRs meta.
- N=3 ADR-0062-create.
- N=4 P160A.
- N=5 P229 ADR-0080 PROPOSTO→EM VIGOR.
- N=6 P238.
- N=7 P244.
- N=8 P249.
- N=9 P253.
- **N=9+1=10 cumulativo** se contarmos P254... wait
  re-counting: P156A(1) + P156K(2) + ADR-0062-create(3) +
  P160A(4) + P229(5) + P238(6) + P244(7) + P249(8) + P253(9)
  + **P254(10)** = **N=10 cumulativo P254**.
  (Spec hipotetizou N=9; contagem empírica revela N=10
  cumulativo incluindo P229 retroactivo).
- Limiar formalização N=6 ultrapassado amplamente.

**Sub-padrão "promoção real scope-out ADR-0054 graded" granular
N=14 preservado P254** (P254 administrativo XS não materializa
nova promoção; apenas formaliza ADR-0082 PROPOSTO→EM VIGOR).

**Pattern "L0 tocado pós-P229 (sub-categorias)"** N=10
preservado P254.

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=13
preservado** P254 (P254 zero código + zero L0; nem aplica nem
viola ADR-0080).

**Paridade pattern P229 ADR-0080 PROPOSTO→EM VIGOR precedente
directo para ADR-0082 P254 PROPOSTO→EM VIGOR** — mesmo template
arquitectural: passo administrativo XS dedicado pós-validação
empírica cumulativa.

**Cross-reference ADR-0082 EM VIGOR** (segunda ADR meta a
transitar PROPOSTO → EM VIGOR via passo admin XS dedicado).
