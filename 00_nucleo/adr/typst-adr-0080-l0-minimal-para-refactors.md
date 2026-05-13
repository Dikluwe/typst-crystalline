# ⚖️ ADR-0080: L0 minimal para refactors aditivos pós-M9c

**Status**: `PROPOSTO`
**Data**: 2026-05-13
**Autor**: Humano + IA
**Validado**: 7 aplicações cumulativas pós-M9c
(P217+P218+P219+P220+P222+P223+P224; N=7 patamar empírico
sólido).
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

## 7 aplicações cumulativas (validação empírica)

| Passo | Refactor | L0 acção | Observação |
|-------|----------|----------|------------|
| **P217** | `Content::Columns` variant novo (P224.B/C precedente) | L0 não tocado | Decisão empírica nova |
| **P218** | `native_columns` stdlib | L0 não tocado | **Spec C6 propôs Opção α**; materialização Opção γ |
| **P219** | Layouter arm refactor + Region/Regions | L0 não tocado | **Spec C7 propôs Opção α retroactiva**; materialização Opção γ |
| **P220** | `Content::Colbreak` agregado | L0 não tocado | Convenção consolidada |
| **P222** | `native_measure` stdlib + visibility promotion | L0 não tocado | Pattern N=4 → 5 consolidado |
| **P223** | `Content::Place` +`float`+`clearance` | L0 não tocado | Pattern N=5 → 6 consolidado |
| **P224** | `Content::Grid` refino substantivo + 3 variants + módulo | L0 não tocado | **Spec C6 propôs Opção α**; materialização Opção γ. **Divergência consciente N=7** |

**N=7 cumulativo confirmado** em P226 audit.

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

## Promoção

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

## Próximos passos

ADR-0080 PROPOSTO **não compromete trabalho subsequente**.
Pattern continua estabelecido empíricamente em P226+
(sub-passos Fase 5 Layout candidata identificados em
ADR-0079 PROPOSTO; cada sub-passo materialização decidirá
Opção γ por defeito ou Opção α por excepção).

Promoção formal a `EM VIGOR` candidato Caminho XS
administrativo dedicado se humano priorizar pós-N=8 ou
imediato (passo limpo sem código).
