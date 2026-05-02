# Passo P182E — Tests E2E pipeline confirmando paridade

Quarto passo de materialização P182 (após P182A diagnóstico,
P182B trait method, P182C extract_payload + locatable +
auto-init, P182D Layouter consumers).
Magnitude **S**.

Adiciona tests E2E que cobrem o pipeline completo `eval →
walk → from_tags → layout_with_introspector` para documentos
típicos com numeração de heading. Confirma paridade
observable entre path Introspector e path legacy
(substitution-with-fallback do P182D). Cobre caso de
regressão (re-update da chave) e caso de documento complexo
(headings + equation).

Após P182E:
- Tests E2E confirmam que migração P182B–D não quebra
  output observable.
- Auto-init em `from_tags::StateUpdate` (P182C 5.1)
  validado em pipeline real (não só em test unitário).
- Caminho de re-update (Set após Set) validado.
- Paridade snapshot legacy vs Introspector confirmada.

**Pré-condição**: P182D concluído. Tests workspace 1.751
verdes; zero violations. 4 consumers migrados (figure-ref,
cite-arm, heading-arm, equation-arm) com fallback
preservado. Output observable inalterado em produção.

**Restrições**:
- **Não** modificar código de produção em
  `01_core/src/rules/`, `01_core/src/entities/`,
  `02_shell/`, `03_infra/`, `04_wiring/`.
- **Não** modificar walk arm, write-sites, copy-sites
  legacy.
- **Não** modificar trait `Introspector`,
  `extract_payload`, `is_locatable`, `from_tags`,
  Layouter consumers.
- **Não** remover fallback (M6).
- API pública preservada.
- Output observable em produção inalterado — P182E é
  apenas tests novos.

---

## Sub-passos

### .A Auditoria de tests existentes

1. Inventariar tests existentes que cobrem o caminho:
   - `grep -rn "layout_with_introspector\|walk.*from_tags"
     01_core/src/`.
   - Localizar tests P181 série que correm pipeline E2E
     (`p181_*` em `01_core/src/rules/layout/tests.rs` ou
     similar). Padrão de helper de pipeline a replicar.
   - Localizar test P182D `p182d_heading_numbering_paridade_legacy_vs_migrated`
     (cobre paridade básica; P182E estende).

2. Inventariar helpers disponíveis:
   - `eval_for_test`, `world_from_str`, `compile_to_document`,
     ou similar — qual existe e como é chamado.
   - Padrão estabelecido nos tests P181 / P182D.

3. Confirmar tests pré-existentes que cobrem
   `Content::SetHeadingNumbering`:
   - `tests.rs:899` (per P182C) — usa injecção directa
     em `state.numbering_active["equation"]`.
   - `layout_set_heading_numbering_activa_contador` (per
     P182D) — cobre heading legacy.
   - Identificar se cobertura existente já inclui pipeline
     completo via `eval` ou apenas construção manual de
     `Content`.

Output: tabela com tests existentes + cobertura + lacuna
identificada que P182E preenche.

**Critério de saída e gate de decisão**:
- Se helpers de pipeline E2E ausentes (improvável per
  P181/P182D que os usaram): cláusula gate substancial —
  recuar e investigar.
- Se cobertura existente já cobre 80%+ do que P182E
  pretende: cláusula gate trivial — P182E reduz a 1-2
  tests dedicados que cobrem a lacuna específica (re-update,
  por exemplo).
- Senão prosseguir com 3-4 tests novos.

### .B Test E2E pipeline completo (caso típico)

1. Adicionar test em `01_core/src/rules/layout/tests.rs`
   (ou ficheiro de tests E2E existente) que:
   - Constrói documento via `eval` ou markup string
     equivalente: `#set heading(numbering: "1.1")` seguido
     de 2-3 headings.
   - Corre pipeline `eval → walk → from_tags →
     layout_with_introspector`.
   - Extrai `plain_text` do `PagedDocument` resultante.
   - Confirma assertion: prefixos `"1"`, `"1.1"`, `"2"` (ou
     padrão equivalente conforme nesting) presentes.

2. Nome sugerido: `p182e_pipeline_e2e_heading_numbering`.

**Critério de saída**:
- Test passa.
- Padrão dos helpers replicado correctamente.

### .C Test de re-update (regressão auto-init)

1. Adicionar test cobrindo o caminho re-update validado em
   P182C 5.1 (auto-init na primeira ocorrência + update
   normal na segunda):
   - Documento: `#set heading(numbering: "1.1")` seguido
     de 1 heading; depois `#set heading(numbering: none)`
     seguido de outro heading.
   - **Cristalino actual**: variant é `Content::SetHeadingNumbering
     { active: bool }` (não tem campo `pattern`). O caso
     "numbering: none" pode mapear para `active: false`.
   - Confirmar primeiro heading com prefixo, segundo
     heading sem prefixo.

2. Se forma exacta da `set rule` em cristalino diverge do
   padrão vanilla (cristalino não suporta `numbering:
   "1.1"` — apenas `active: bool`): adaptar test para usar
   2x `Content::SetHeadingNumbering { active: true }` ou
   `true → false` directamente, conforme o que cristalino
   suporta.

3. Nome sugerido: `p182e_pipeline_e2e_re_update_numbering`.

**Critério de saída**:
- Test passa.
- Regressão auto-init coberta no pipeline E2E (não só
  unitário em P182C).

### .D Test de paridade snapshot (documento complexo)

1. Adicionar test que constrói documento com:
   - `Content::SetHeadingNumbering { active: true }`.
   - 2-3 headings.
   - 1+ equation block.
   - 1+ parágrafo de texto entre headings.

2. Comparar output:
   - Path A: `layout(content)` legacy.
   - Path B: `walk + from_tags + layout_with_introspector(content,
     introspector)`.
   - Confirmar `plain_text` (ou outro accessor observable)
     idêntico entre A e B.

3. Nome sugerido: `p182e_pipeline_e2e_paridade_documento_complexo`.

**Critério de saída**:
- Test passa.
- Paridade observable confirmada para documento não-trivial.

### .E (Opcional) Test que walk continua a popular legacy

1. Adicionar test cobrindo regressão evitada (walk legacy
   continua intocado):
   - Documento com `Content::SetHeadingNumbering { active:
     true }`.
   - Após `walk`, inspeccionar `state.numbering_active`:
     deve conter `("heading", true)`.
   - Confirma que P182C/D não regrediram o caminho legacy
     paralelo.

2. Nome sugerido: `p182e_walk_popula_legacy_apos_p182cd`.

**Critério de saída**:
- Test passa.
- Sentinela contra regressão de janela compat M6.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P182D
   baseline (1.751): +3 a +4 dependendo de cobertura
   `.E`.
3. `crystalline-lint .` zero violations.
4. Tests E2E `.B`–`.E` passam isoladamente
   (`cargo test --workspace --lib p182e`).
5. Tests existentes não regridem.
6. Output observable em produção inalterado (P182E não
   toca produção).
7. Snapshot tests ADR-0033 verdes.
8. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-182e-relatorio.md`
com:

- Resumo: 3-4 tests E2E adicionados; paridade Introspector
  vs legacy confirmada; auto-init validado em pipeline
  real; sentinela de regressão legacy.
- Confirmação `.F` (8 verificações).
- Δ tests vs baseline P182D.
- Hashes finais de L0s modificados (se algum) — esperado
  zero edits L0 (apenas tests).
- Decisões de execução notáveis.
- Estado actual:
  - P182 série: A ✅ B ✅ C ✅ D ✅ E ✅ | F pendente.
  - M9: 10/11 features (inalterado — fechamento formal
    em P182F).
  - 35 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P182F (fecho lacuna #4 + relatório
  consolidado série P182).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Test E2E caso típico passa (`.B`).
3. Test E2E re-update passa (`.C`).
4. Test E2E paridade documento complexo passa (`.D`).
5. (Opcional) Test sentinela legacy passa (`.E`).
6. Tests existentes não regridem.
7. Verificações `.F` passam (8/8).
8. Relatório `.G` escrito.
9. Output observable em produção inalterado (P182E não
   toca produção).

---

## O que pode sair errado

- **Helpers de pipeline E2E ausentes**: cláusula gate
  substancial. Improvável (P181/P182D usaram). Se sim,
  recuar.
- **`#set heading(numbering: ...)` em cristalino diverge
  de vanilla**: auditoria `.A` confirma forma exacta do
  variant `Content::SetHeadingNumbering`. Test adapta.
- **`eval` de markup não disponível em test context**:
  cláusula gate trivial — construir `Content` directamente
  via construtor (per padrão P181 / P182D que evitam
  `eval` em testes).
- **Snapshot tests divergem entre paths A e B no `.D`**:
  cláusula gate substancial — investigar. Causa provável:
  algum field do `state` legacy popula via path diferente
  do Introspector (ex.: `headings_for_toc` carregando
  body frozen vs hash em tags — lacuna #3). P182E tests
  cobrem apenas `numbering_active`; outros fields que
  divirjam são fora de escopo. Se divergência só afeta
  `plain_text`, é P182 issue. Se divergência afeta outros
  fields, é cobertura para passos futuros.
- **`active: false` em `Content::SetHeadingNumbering` não
  produz comportamento simétrico** (Introspector retorna
  `false`, mas legacy mantém `true` da escrita anterior):
  improvável dado que P182C 5.1 documentou `Bool(false)`
  é registado e propagado. Test `.C` confirma.
- **Linter divergência V13/V14**: cláusula gate trivial —
  `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 LOC (3-4 tests + helpers
  partilhados se necessário).
- **Sem código de produção tocado** — tests apenas.
- **Sem dependências externas novas**.
- **Pré-condição P182F**: este passo concluído.
- **Padrão replicado**: P181I tests E2E (paridade
  legacy vs Introspector via pipeline completo).
- **Cláusula gate trivial**: aplicável a forma da
  construção de `Content`, helpers, nomes de tests.
- **Cláusula gate substancial**: aplicável apenas se
  helpers ausentes ou se snapshot tests divergirem
  inesperadamente em campos fora de `numbering_active`
  (registo do achado mas não bloqueio P182E).
- **`.E` é opcional** — só accionado se inventário `.A`
  identificar que sentinela vale o custo. Default é
  incluir.
