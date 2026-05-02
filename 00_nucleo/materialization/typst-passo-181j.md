# Passo P181J — Relatório consolidado P181

Nono e último passo de materialização P181 (após P181A-P181I).
Magnitude **S**. Trabalho puramente documental.

Consolida materialização completa P181 num relatório único.
Não modifica código, tests, ou diagnósticos. Apenas sintetiza
9 sub-passos cumulativos, métricas agregadas, decisões
arquitecturais, e lições aprendidas.

**Pré-condição**: P181I concluído. Lacuna #6 fechada
formalmente. M9 10/11 features.

**Restrições**:
- Sem código tocado.
- Sem L0/L1 novos.
- Sem alteração de tests.
- Output observable não muda.
- Apenas trabalho documental.

---

## Sub-passos

### .A Sintetizar resultados P181A-P181I

Reunir factos de cada relatório:

| Sub-passo | Output principal | Δ tests | L0 mod | L1 mod |
|-----------|------------------|---------|--------|--------|
| P181A | Decisões + plano | 0 | 0 | 0 |
| P181B | `BibStore` + field | +8 | 1 (novo) + 1 mod | 2 |
| P181C | `ElementKind` + `ElementPayload` Bib | +6 | 2 mod | 3 |
| P181D | `is_locatable` + `extract_payload` | +4 | 2 mod | 2 |
| P181E | `from_tags` arm popula | +4 | 1 mod + 1 | 2 |
| P181F | Trait métodos | +3 | 1 mod | 1 |
| P181G | Layouter cite-arm | +6 | 1 mod | 1 |
| P181H | Walk puro + layout() | +2 | 2 mod | 2 |
| P181I | Tests E2E + lacuna | +5 | 0 | 1 (tests) + 1 (diagnóstico) |
| **Total** | | **+38** | | |

Tests cumulativos: 1700 (P180) → 1738 (P181I).
Δ líquido: +38.

### .B Sintetizar decisões arquitecturais

Lista das 6 cláusulas P181A §3:

1. **`BibStore = Vec<BibEntry> + HashMap<String, u32>`**
   (sem `IndexMap`). Replica shape `CounterStateLegacy`.
2. **`add_bibliography` faz `extend`** (cláusula 2).
3. **`assign_number` usa `or_insert`** (cláusula 3) —
   keys duplicadas preservam primeiro número.
4. **Walk Opção β (puro)**: walk arm `Content::Bibliography`
   não muta state directamente; tag emitida via
   `extract_payload`.
5. **Layouter cite-arm via Introspector** (caminho P168) —
   substitution-with-fallback.
6. **Critério de fecho Opção 3**: infraestrutura + consumer
   migrado. Fields legacy preservados até M6.

### .C Sintetizar achados não-triviais

Cinco achados durante execução:

1. **Bug semântico capturado em P181E.** Snippet sugerido
   na instrução usava `bib_store.len() as u32 + 1` mas
   `add_bibliography` é chamado depois do loop —
   `len()` permaneceria 0. Solução: novo método
   `BibStore::numbers_len()` paralelo a
   `state.bib_numbers.len()`. Snippets de instrução
   tratados como sketch, não código pronto.

2. **Discrepância signature `layout()` em P181H.**
   Instrução assumiu 1 arg; realidade 2 args. Adaptação
   sem reabrir gate — passou a re-correr
   `introspect_with_introspector` internamente, descartar
   state novo, e usar `initial_state` recebido.
   Backward compat 100%.

3. **Test diferencial em P181G.** Adição do 6º teste
   (`cite_consulta_introspector_quando_state_legacy_vazio`)
   prova explicitamente que cite-arm consulta Introspector.
   Sem este, os 5 tests originais passariam mesmo sem
   migração efectiva (state legacy daria mesmo resultado).

4. **Padrão "trait estendido" replicado pela 4ª vez** em
   P181F. Sequência: P175 (query) → P176
   (formatted_counter) → P177 (formatted_counter_at) →
   P181F (bib). Mecânica idêntica.

5. **Padrão substitution-with-fallback P168 replicado** em
   P181G. 2ª vez (figure-ref + cite-arm). Confirma
   reusabilidade para consumers M5 futuros.

### .D Estado final M9 e M5

**M9: 10/11 features**.
1. P169: `metadata(value)`.
2. P170: `CounterKey` hierarquia.
3. P171: `state(key, init)` + `state_update(key, value)`.
4. P172+P173: `state_update_with(key, fn)`.
5. P175: `query(selector)` minimal.
6. P176: `counter.final(key)` minimal.
7. P177: `counter.at(label)` minimal.
8. P178: Outline cascade — lacuna #7.
9. P179: `query` upgrade — `Vec<Location>`.
10. **P181: bib state — lacuna #6**.

Restante: `numbering_active` (lacuna #4) — infraestrutura
pronta P171; consumer aguarda M5 retomar.

**M5: 2/6 consumers migrados**.
- ✅ P168: figure-ref.
- ✅ P181G: cite-arm.
- ⏳ Layouter (5 cite/layout related arms).
- ⏳ `layout_outline`.
- ⏳ `counter_helpers`.
- ⏳ `references.rs::layout_ref` (section-arm).
- ⏳ `layout_equation`.

### .E Estado final P181 lacunas

| # | Lacuna | Estado pós-P181 |
|---|--------|-----------------|
| 1 | figure.kind None vs "image" | Parcial (P168) |
| 2 | Auto-labels | Adiar |
| 3 | Body frozen | Manter (intencional) |
| 4 | numbering_active | Infraestrutura P171; consumer aguarda |
| 5 | format_hierarchical | ✅ P170 |
| 6 | bib_entries / bib_numbers | ✅ **P181** |
| 7 | has_outline | ✅ P178 |

3 resolvidas. 1 com infraestrutura pronta. 3 adiadas/intencionais.

### .F Pendências cumulativas pós-P181

Janela compat encerrada para bib state. M6 elimina:
- `CounterStateLegacy.bib_entries` e `bib_numbers`
  (vazios em produção pós-P181H).
- Copy-sites `state→Layouter` (1397, 1399, 1425, 1427).
- Cite-arm fallback a state legacy.
- Re-walk em `layout()` legacy quando callers adoptarem
  `introspect_with_introspector` directamente.

Pendências pré-existentes inalteradas:
- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

ADR-0062 (`hayagriva` PROPOSTO) **independente** de P181.
P181 trabalha sobre subset minimal cristalino. Promoção
para `IMPLEMENTADO` continua a depender da decisão futura
de adoptar `hayagriva` para CSL parsing.

### .G Escrever relatório consolidado

`00_nucleo/materialization/typst-passo-181-relatorio-consolidado.md`
com 8 secções:

1. **Resumo executivo** — lacuna #6 fechada; M9 10/11;
   pipeline bib state Introspection-style; janela compat
   encerrada.

2. **Sub-passos materializados** — tabela `.A` com
   métricas individuais.

3. **Decisões arquitecturais** — 6 cláusulas `.B`.

4. **Achados durante execução** — 5 itens `.C`.

5. **Estado M9 e M5** — `.D`.

6. **Estado lacunas** — `.E`.

7. **Pendências cumulativas** — `.F`.

8. **Próximos passos sugeridos**:
   - **`numbering_active` (lacuna #4)** — fecha M9 11/11.
     Infraestrutura pronta P171.
   - **M5 retomar** — 4 consumers restantes ainda
     bloqueados por lacunas (#3 outline body) ou padrões
     mutação inerentes.
   - **M6 cleanup** — eliminar fields legacy quando
     M5 saturar.
   - **`here()` ou `locate(callback)`** — features M9
     adicionais com pré-requisitos arquitecturais.

### .H Verificação estrutural

1. `cargo check --workspace` passa (sem código tocado).
2. `cargo test --workspace --lib` passa sem mudança de
   contagem (1738).
3. `crystalline-lint .`: zero violations.
4. Relatório consolidado existe com 8 secções.
5. Dados sintetizados são consistentes com relatórios
   individuais P181A-P181I.
6. Sem código de produção tocado.

### .I Encerramento

Relatório consolidado é o output principal. Encerra
formalmente a série P181.

Adicionalmente, breve relatório
`00_nucleo/materialization/typst-passo-181j-relatorio.md`
com:

- Resumo: relatório consolidado escrito; série P181
  encerrada; lacuna #6 fechada; M9 10/11.
- Confirmação `.H` 1-6.
- Estado pós-passo: P181J concluído. Série P181
  inteiramente fechada.
- Pendências inalteradas.
- Caminho à frente: `numbering_active` (P182?) ou M5
  retomar ou M6 cleanup.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` sintetizou métricas dos 9 sub-passos.
2. `.B` listou 6 cláusulas arquitecturais.
3. `.C` identificou 5 achados.
4. `.D` documentou estado M9 (10/11) e M5 (2/6).
5. `.E` actualizou tabela de lacunas.
6. `.F` documentou pendências e janela compat.
7. `.G` escreveu relatório consolidado em 8 secções.
8. Verificações `.H` 1-6 passam.
9. `.I` relatório curto que encerra formalmente.

---

## O que pode sair errado

- **Métricas inconsistentes entre relatórios individuais
  e consolidado**: re-conferir somas. Cláusula gate
  trivial.
- **Localização do consolidado em pasta materialization
  causa duplicação com J relatório**: J pode ser apenas
  ponteiro para o consolidado. Decisão local.
- **Dados de relatórios individuais conflitam**:
  improvável; relatórios são sequenciais e cumulativos.
- **L0 actualização para `m1-lacunas-captura.md` já
  feito em P181I**: confirmar que P181J não tenta
  re-actualizar.

---

## Notas operacionais

- **Tamanho**: S. Trabalho documental puro. Sem código.
- **Output principal**: `typst-passo-181-relatorio-consolidado.md`
  como referência única para futuras consultas.
- **Pré-condição P182**: P181J encerra formalmente a
  série. P182 pode ser feature seguinte M9 (`numbering_active`),
  M5 retomar, M6 cleanup, ou outra estratégia.
- **Cláusula gate trivial**: aplicável a localização
  do consolidado, formato exacto.
- **Padrão "passo final consolidador"**: P167 fez
  similar para inventário; P175/P179 não fizeram
  porque eram features individuais. P181J consolida
  série de 9 sub-passos — caso especial.
- **Lacuna #6 já fechada formalmente em P181I**: P181J
  não toca `m1-lacunas-captura.md`. Apenas referencia.
- **M9 10/11 marca momento simbólico**: 10 features
  M9 materializadas. Lacuna #4 fecha M9 11/11.
