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
