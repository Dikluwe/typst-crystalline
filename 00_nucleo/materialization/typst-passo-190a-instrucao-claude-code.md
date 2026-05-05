# Passo 190A — Instrução Claude Code

> **Nota de origem**: este ficheiro **substitui** o
> P190A original (em
> `typst-passo-185a-relatorio.md` renomeado em série
> P185). Original declarado obsoleto em P200
> consolidado §10. P190A reescrita do zero baseada
> no estado consolidado pós-P200B.

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.869 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- **M5 universal completo** (P200B):
  - 0 excepções activas + 0 residuos + 0 pré-requisitos.
  - 9 séries materializadas P189B-P200.
  - 5 variantes operacionais ADR-0069 consolidadas.
  - 7 aplicações ADR-0069 stylesheet.
  - 4 helpers privados família ADR-0069.
- Trait `Introspector`: 20 métodos.
- `TagIntrospector`: 9 sub-stores.
- `ElementPayload`: 13 variants.
- `ElementKind`: 10.
- `Content` enum: + 1 variant em P199B
  (`SetEquationNumbering`).

P190 inicia **M6 — eliminação `CounterStateLegacy`**.
Cleanup do write paralelo M5 + struct + dependências.
**Magnitude L cross-modular** esperada.

**Material de partida** verificado:

- `00_nucleo/materialization/typst-passo-200-relatorio-consolidado.md`
  §10 marco M5 universal completo; §8 DEBT M6
  documentação.
- `00_nucleo/auditoria-fresh-projecto.md` F1
  (`CounterStateLegacy` 18 fields públicos + 12+
  conceitos ortogonais; 330 linhas; 25 métodos);
  F3 (Layouter 19 fields herdados).
- `00_nucleo/m1-lacunas-captura.md` — lacunas #1,
  #1b, #2 ortogonais a M6 (não bloqueiam).

P190A é o passo de diagnóstico. **Magnitude esperada
S-M** (não S puro como passos anteriores) —
diagnóstico M6 exige inventário detalhado de 18
campos + 4 helpers + Layouter consumers + walk arms
+ tests.

P190B+ implementação depende criticamente de:
- Cláusula 1: estratégia de eliminação (incremental
  por categoria vs big-bang).
- Cláusula 2: ordem das categorias (dependências
  cruzadas).
- Cláusula 3: forma final do struct (eliminação
  total vs façade vs rename).

---

## Postura do auditor / executor

P190A é passo **L0-puro / diagnóstico-primeiro**,
padrão estabelecido em 22 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` se decisão
  arquitectural emergir (provável — eliminação de
  struct é decisão substancial; precedente ADR-0068
  PROPOSTO em P185A).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, Layouter,
  consumers — P190B+.

**Magnitude diagnóstico**: S-M (mais que passos
anteriores). Decisões expandidas porque:
- 18 campos a inventariar individualmente.
- 4 helpers a auditar.
- Múltiplos consumers Layouter.
- Estratégia de eliminação não tem precedente
  directo no projecto.

**Particularidade de P190**: trabalho cross-modular
sem precedente directo. Diferente de P195-P200 que
seguiam padrões testados, P190 é **categoria nova
de trabalho** — eliminação de struct + cleanup
paralelo.

---

## Escopo

**Primário**: planear eliminação completa de
`CounterStateLegacy` struct + cleanup do write
paralelo M5 + migração final de helpers e consumers
que ainda dependem de legacy.

**Confirmação**: validar inventário factual:
- Campos exactos do `CounterStateLegacy` (esperado:
  18 públicos + 2 privados per F1).
- Cada campo: consumer? Já tem caminho Introspector?
  Layouter directo?
- 4 helpers `compute_*`: cada um lê quais campos?
  Substituível por trait method location-aware?
- Walk arms: mutações legacy preservadas (write
  paralelo M5) — quais ficheiros, quantas mutações.
- Layouter assignments: `mod.rs:1490, 1521` + outros.
- Tests workspace: quais dependem de
  `CounterStateLegacy` directamente? Quais via
  Layouter `counter` field?

**Decisões a tomar** — 9 cláusulas:

1. **Estratégia de eliminação**:
   - **α** (incremental por campo): cada campo migra
     independentemente; struct progressivamente
     esvazia. Magnitude: 18+ sub-passos pequenos.
   - **β** (incremental por categoria): agrupar
     campos por tema; cada categoria fecha em
     sub-passo M. Magnitude: 5-8 sub-passos médios.
   - **γ** (big-bang): toda a eliminação numa série
     única. Magnitude: L+ num único passo; alta
     probabilidade de gate substancial.

   **Sugestão preliminar**: β.

2. **Ordem das categorias** (dependências cruzadas):
   - Quais categorias têm consumers que já lêem via
     Introspector path (com fallback)?
   - Quais categorias precisam de migração consumer
     primeiro?
   - Identificar cadeias de dependência (helpers
     `compute_*` lêem state durante walk).

3. **Forma final do struct**:
   - **α** (eliminação total): `CounterStateLegacy`
     deixa de existir.
   - **β** (façade temporário): struct vazio
     implementando `Deref` para `TagIntrospector`.
   - **γ** (rename): renomear para `IntrospectionState`
     ou similar.

   **Sugestão preliminar**: α — padrão sub-store
   cristalino é eliminação. Façade engana; rename
   esconde problema.

4. **API pública**:
   - `CounterStateLegacy` é `pub` em `entities/mod.rs`?
   - Re-exportado em `lib.rs`?
   - Consumer externo (CLI, tests integração)
     usa-o?
   - Se API pública: eliminação é breaking change —
     ADR de breaking change.
   - Se API privada/interna: eliminação livre.

5. **4 helpers `compute_*` família ADR-0069**:
   - Cada helper lê state legacy durante walk.
   - **α** (eliminação total): walk arms passam a
     popular Tags directamente sem helpers privados.
   - **β** (migração para Introspector
     location-aware): helpers passam a ler
     `intr.flat_counter_at(...)` etc. via API
     trait.
   - **γ** (renomear como helpers públicos):
     improvável.

   Sugestão preliminar: α se write paralelo se torna
   desnecessário; β se algum consumer downstream
   ainda precisa de `compute_*` semantics.

6. **Layouter dependências**:
   - `Layouter<M, S>::counter: CounterStateLegacy`
     campo embebido (per F3).
   - Layouter consumers fazem `self.counter.X`.
   - **Decisão obrigatória**: Layouter migra para
     consumer Introspector path completo. Substitui
     `self.counter.X` por `self.introspector.X` ou
     equivalente.
   - Magnitude potencialmente significativa
     (Layouter 19 fields per F3).

7. **Walk arms**:
   - Cada walk arm migrado em M5 ainda muta
     `state.X` durante walk (write paralelo).
   - **Decisão obrigatória**: eliminar mutações
     legacy. Walk torna-se puro (apenas emite Tags;
     não muta state).
   - Walk arms a tocar:
     - Heading (4 mutações).
     - Figure (2 mutações).
     - SetHeadingNumbering (1 mutação).
     - SetEquationNumbering (1 mutação).
     - CounterUpdate (3 caminhos).
     - Labelled (1 mutação).
     - Outline + Bibliography + outros.

8. **Tests workspace**:
   - 1.869 tests verdes.
   - Quais dependem de `CounterStateLegacy`
     directamente? Quais via Layouter `counter`
     field?
   - **Decisão**: tests adaptados conforme padrão
     pragmático auditor #1. Tests existentes não
     regridem.
   - Tests sentinela legacy (E1-E6 + E2-residuo):
     preservar como histórico ou remover?

9. **Critério de fecho de M6**:
   - `grep -rn "CounterStateLegacy" 01_core/src/`
     retorna **zero**.
   - `grep -rn "CounterStateLegacy" 02_shell/
     03_infra/ 04_wiring/` retorna zero.
   - 4 helpers `compute_*` eliminados ou migrados.
   - Walk arms puros.
   - Layouter usa Introspector path completo.
   - Tests workspace verdes.
   - F1 marcado como fechado.
   - F3 parcialmente resolvido.

**Fora de escopo**:

- Lacunas residuais #1, #1b (ortogonais a M6).
- Lacuna #2 (auto-labels — pode permanecer
  divergência intencional).
- M7 (loop fixpoint), M8 (memoização comemo) —
  passos futuros.
- F11 (`export.rs` 2090 linhas — refactor separado).
- F2-F10 outros achados — fora de escopo.

---

## Critérios objectivos

### O1 — Inputs verificáveis

- `grep -rn "CounterStateLegacy" 01_core/src/`.
- `grep -rn "counter_state_legacy" 01_core/src/`.
- `grep -rn "compute_labelled\|compute_heading_auto_toc\|compute_figure\|compute_heading_for_toc"
  01_core/src/`.
- `grep -rn "self.counter\." 01_core/src/`.
- `grep -rn "state\." 01_core/src/rules/introspect.rs`.

### O2 — Alternativas

Cláusula 1 (3 opções estratégia), cláusula 3 (3
opções forma final), cláusula 5 (3 opções helpers).
Demais cláusulas têm caminho preferido claro.

### O3 — Critério de escolha

Padrão sub-store cristalino é eliminação. Façade
adia; rename engana. Eliminação por categoria (β)
é menos arriscada que big-bang (γ) e mais coerente
que por campo (α).

### O4 — Magnitude

P190 implementação **L cross-modular**:
- α (incremental por campo): 18+ sub-passos × S/M.
  Total: L genuíno em agregado.
- β (incremental por categoria): 5-8 sub-passos × M.
  Total: L em agregado; granularidade média.
- γ (big-bang): 1 sub-passo × L+. Risco alto de
  cláusula gate substancial.

P190A diagnóstico: S-M (mais que passos anteriores).
P190B+ implementação total: L cross-modular.

### O5 — Reversibilidade

Eliminação reversível mas cara. Reverter exige
restaurar struct + bridge legacy + helpers
removidos.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P190 elimina o que séries P181-P200 acumularam como
write paralelo M5. **Sem precedente directo**.
Padrão sub-store cristalino sugere eliminação total
(Opção 3α).

### Q2 — Honestidade de magnitude

P190A diagnóstico é S-M. P190 implementação é L
cross-modular. **Não disfarçar** — eliminar struct
de 18 campos com 4 helpers + Layouter consumers é
trabalho substancial. Cláusulas gate substanciais
prováveis.

### Q3 — Cobertura sem regressão

Output observable preservado por construção:
- Sub-stores Introspector populated via Tag pipeline.
- Consumers Layouter migram para Introspector path
  com fallback temporário até toda migração concluir.
- Tests existentes adaptados conforme necessário.

**Risco**: regressão em casos edge não cobertos por
sub-stores actuais. Mitigação: tests E2E exhaustivos
+ snapshot tests + auditoria empírica em cada
sub-passo.

### Q4 — F1 + F3 fecham

Após M6:
- F1 fecha (`CounterStateLegacy` eliminado).
- F3 parcialmente fecha (Layouter perde campo
  `counter`; resta lidar com outros 18 fields).

### Q5 — Granularidade

P190 série tem **5-8 sub-passos** (β recomendada).
Cada sub-passo magnitude M; total agregado L.

---

## Sub-passos de P190A

### Sub-passo 190A.A — Validação do estado actual + inventário

Auditor confirma empiricamente:

#### Estado consolidado pós-P200B

1. Tests workspace 1.869 verdes.
2. Linter zero violations.
3. M5 universal completo confirmado (per P200
   consolidado §6 Marco M5).

#### Inventário `CounterStateLegacy`

4. Localizar
   `01_core/src/entities/counter_state_legacy.rs`
   (per F1: 330 linhas, 18 fields públicos + 2
   privados, 25 métodos).

5. Listar **todos os campos** com type:
   - Esperado: 18 públicos + 2 privados.
   - Forma: nome, type, função.
   - Categorias prováveis (per F1):
     - **Counters**: `flat`, `hierarchical`,
       `figure_numbers`, `local_figure_counters`,
       `figure_label_numbers`.
     - **Labels**: `resolved_labels`,
       `headings_for_toc`, `auto_label_counter`,
       `label_pages`, `known_page_numbers`.
     - **Numbering active**: `numbering_active`.
     - **Bibliography**: `bib_entries`, `bib_numbers`.
     - **Document metadata**: `lang`, `has_outline`,
       `is_readonly`.

6. Listar **todos os métodos** com escopo:
   - 25 métodos esperados.
   - Cada método: signatura + função + caller(s).

#### Inventário consumers

7. Para cada campo, identificar consumers:
   - Walk arms que mutam (write paralelo M5).
   - Layouter consumers (`self.counter.X`).
   - 4 helpers `compute_*` que leem.
   - Tests que dependem.

8. Identificar consumers Layouter directos:
   - `grep -rn "self.counter\."
     01_core/src/rules/layout/`.

9. Identificar Layouter assignments
   (`mod.rs:1490, 1521` + possíveis outros).

#### Inventário 4 helpers `compute_*`

10. Para cada helper, identificar:
    - Localização: `introspect.rs` (assumido).
    - Campos legacy lidos: quais.
    - Caller(s): qual walk arm chama.
    - Pode ser substituído por Introspector path
      location-aware?

11. Confirmar trait `Introspector` métodos
    location-aware existentes:
    - `is_numbering_active_at(key, location) -> bool`.
    - `flat_counter_at(key, location) -> Option<usize>`.
    - `formatted_counter_at(key, location) ->
      Option<String>`.
    - `headings_for_toc()` (P200B — não
      location-aware, retorna full vec).
    - `resolved_label_for(label) -> Option<&str>`.
    - `figure_number_at_index(kind, idx) ->
      Option<usize>`.
    - Outros 14 métodos.

#### Inventário walk arms com mutações legacy

12. Para cada walk arm migrado em M5, listar
    mutações legacy preservadas:
    - Heading: 4 mutações.
    - Figure: 2 mutações.
    - SetHeadingNumbering: 1 mutação.
    - SetEquationNumbering: 1 mutação.
    - CounterUpdate: 3 caminhos.
    - Labelled: 1 mutação.
    - Outline + Bibliography + outros.

#### Inventário tests workspace

13. Identificar tests que dependem de
    `CounterStateLegacy` directamente:
    - Tests sentinela E1-E6 + E2-residuo.
    - Tests `walk_arm_*_le_X_legacy`.
    - Outros.

14. Identificar tests que dependem via Layouter
    `counter` field:
    - `grep -rn "layouter.counter\."
      01_core/src/`.

#### API pública

15. Confirmar visibility de `CounterStateLegacy`:
    - `pub` em `entities/mod.rs`?
    - Re-exportado em `lib.rs`?
    - `grep -rn "pub.*CounterStateLegacy\|use.*CounterStateLegacy"
      01_core/src/`.

#### L0 alvos

16. Identificar L0s a tocar em P190B+:
    - `entities/mod.md`.
    - `entities/counter_state_legacy.md`
      (eliminado ou actualizado).
    - `rules/introspect.md` (walk arms purificados).
    - `rules/layout/*.md` (consumers migrados).
    - Possivelmente outros.

Output: tabela inventário com:
- 18 campos × consumer × cobertura Introspector.
- 4 helpers × campos lidos × substituição.
- Walk arms × mutações × ficheiro.
- Layouter consumers × ficheiro/linha.
- Tests dependentes.

**Critério de saída**:
- Inventário completo produzido.
- Cada campo categorizado: "tem caminho Introspector
  activo" / "precisa migração consumer primeiro" /
  "blocked".
- 4 helpers categorizados: "eliminável" /
  "migrável para Introspector path" / "blocked".
- Walk arms × mutações listadas.

### Sub-passo 190A.B — Decisão cláusula 1 (estratégia)

Conforme `.A`:

3 opções:
- α (incremental por campo): 18+ sub-passos.
- β (incremental por categoria): 5-8 sub-passos.
- γ (big-bang): 1 sub-passo L+.

**Sugestão preliminar**: β — granularidade óptima.

**Categorias prováveis** (per F1):
1. Counters (flat/hierarchical/figure_numbers/
   local_figure_counters/figure_label_numbers).
2. Labels (resolved_labels/headings_for_toc/
   auto_label_counter).
3. Numbering active.
4. Bibliography.
5. Document metadata (lang/has_outline/is_readonly).
6. Page tracking (known_page_numbers/label_pages).

Output: estratégia + categorias fixadas.

### Sub-passo 190A.C — Decisão cláusula 2 (ordem)

Conforme `.A` + cláusula 1:

Ordem provável (dependências cruzadas):
1. **Categorias com caminho Introspector já activo**
   primeiro.
2. **Categorias com helpers `compute_*` para
   eliminar/migrar** próximas.
3. **Layouter consumer migration** depois (mais
   arriscado).
4. **Eliminação struct + dependências finais**
   último.

Output: ordem fixada per `.A`.

### Sub-passo 190A.D — Decisão cláusula 3 (forma final)

Conforme `.A.15`:

Sugestão preliminar: **α** (eliminação total).

Decisão dependente de visibilidade pública (cláusula
4).

Output: forma fixada.

### Sub-passo 190A.E — Decisão cláusula 4 (API pública)

Conforme `.A.15`:

Se `CounterStateLegacy` é pub: ADR breaking change.
Se privada: livre.

Output: decisão baseada em factualidade.

### Sub-passo 190A.F — Decisão cláusula 5 (helpers)

Conforme `.A.10`:

Sugestão preliminar:
- Helpers eliminados se write paralelo se torna
  desnecessário.
- Helpers migrados para Introspector path se
  consumer ainda precisa de semantics.

Decisão por helper:
- `compute_labelled` (P195D): ?
- `compute_heading_auto_toc` (P196B): ?
- `compute_figure` (P197B): ?
- `compute_heading_for_toc` (P200B): ?

Output: decisão por helper.

### Sub-passo 190A.G — Decisão cláusula 6 (Layouter)

Conforme `.A.8` + `.A.9`:

Layouter perde campo `counter`. Migração para
`introspector` field directo. Magnitude
significativa (Layouter 19 fields → 18 fields per
F3).

Output: estratégia Layouter migration fixada.

### Sub-passo 190A.H — Decisão cláusula 7 (walk arms)

Conforme `.A.12`:

Eliminar mutações legacy em walk arms migrados.
Walk torna-se puro.

**Cláusula gate substancial**: walk arms ainda
referenciados por `compute_*` helpers. Eliminação
simultânea ou sequenciamento cuidado.

Output: estratégia walk purification fixada.

### Sub-passo 190A.I — Decisão cláusula 8 (tests)

Conforme `.A.13` + `.A.14`:

- Tests sentinela legacy: preservar arqueológicos
  ou remover?
- Tests Layouter: adaptar conforme necessário.
- Tests workspace: padrão pragmático auditor #1.

Output: estratégia tests fixada.

### Sub-passo 190A.J — Decisão cláusula 9 (critério de fecho)

Conforme `.A`:

M6 fecha quando:
- `grep -rn "CounterStateLegacy"` retorna zero em
  código produção.
- 4 helpers eliminados ou migrados.
- Walk arms puros.
- Layouter sem campo `counter`.
- Tests workspace verdes.
- F1 fechado.

Output: critério literal verificável.

### Sub-passo 190A.K — Validação do plano de sub-passos

Sub-passos esperados (per cláusula 1 β):

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Categoria 1 (counters) — eliminar mutações + migrar `compute_figure` + eliminar campos correspondentes | M |
| `.C` | Categoria 2 (labels) — eliminar mutações + migrar `compute_labelled` + `compute_heading_auto_toc` + `compute_heading_for_toc` + eliminar campos | M |
| `.D` | Categoria 3 (numbering active) — eliminar mutações + migrar consumers + eliminar campo | M |
| `.E` | Categoria 4 (bibliography) — eliminar mutações + migrar consumers + eliminar campos | M |
| `.F` | Categoria 5 (document metadata) — migrar para sub-store dedicado ou eliminar | M |
| `.G` | Categoria 6 (page tracking) — migrar para sub-store ou eliminar | M |
| `.H` | Layouter migration — remover campo `counter`; migrar consumers Layouter directos | M+ |
| `.I` | Eliminação final struct `CounterStateLegacy` + L0 cleanup + relatório consolidado P190 | M |

Total agregado: **~9 sub-passos × M = L
cross-modular**.

Output: plano detalhado fixado per `.A`.

### Sub-passo 190A.L — ADR

Avaliar:

- Decisão arquitectural substancial: **eliminar
  struct de 18 campos é mudança maior**.
- Forma final (α/β/γ) afecta API.
- ADR provável **PROPOSTO**.

**Opção 1**: ADR-0070 PROPOSTO em P190A; ACEITE
após P190 série fechar.
**Opção 2**: sem ADR (replica P181-P200 sem ADR
para passos M5).

Sugestão preliminar: **Opção 1** — eliminação de
struct é decisão substancial análoga a ADR-0068
P185A para mecanismo arquitectural.

Output: decisão ADR fixada.

### Sub-passo 190A.M — DEBT

P190 fecha **DEBT M6 documentação** registada em
P200C §8.

Após P190B+:
- Antes: write paralelo M5 activo; struct existe;
  helpers leem legacy.
- Após: struct eliminado; walk puro; Layouter
  migrado; helpers eliminados ou migrados.

**F1 fecha**. **F3 parcialmente fecha**.

**Cenário B continua** (sem DEBT formal aberto;
DEBT M6 documentação fecha por execução).

Output: estado actualizado.

### Sub-passo 190A.N — Outputs

Produzir 3 ficheiros:

1. **`00_nucleo/diagnosticos/diagnostico-eliminacao-counter-state-legacy-passo-190a.md`**
   — diagnóstico com 8-10 secções:
   - §1 Validação estado actual.
   - §2 Inventário 18 campos × consumer × cobertura.
   - §3 Inventário 4 helpers × campos lidos ×
     substituição.
   - §4 Inventário walk arms × mutações.
   - §5 Inventário Layouter consumers.
   - §6 Inventário tests dependentes.
   - §7 Decisões cláusula 1–9 (formato O1–O5).
   - §8 Plano de sub-passos sem condicionais.
   - §9 ADR + DEBT avaliação.
   - §10 Próximo sub-passo (P190B com escopo
     concreto).

2. **`00_nucleo/materialization/typst-passo-190a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **(Eventualmente) ADR-0070 PROPOSTO** —
   `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md`
   se cláusula L decidir Opção 1.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora
  de `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P190B+.
- **Não tocar `from_tags`** — P190B+.
- **Não modificar trait `Introspector`** — P190B+.
- **Não modificar `TagIntrospector`** — P190B+.
- **Não modificar Layouter** — P190B+.
- **Não eliminar `CounterStateLegacy`** — P190B+.
- **Não modificar 4 helpers `compute_*`** — P190B+.
- **Não materializar lacunas residuais** (#1, #1b,
  #2) — ortogonais.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a cada campo
  empiricamente.
- **Reaproveitar pattern ADR-0069 + 5 variantes
  operacionais** se aplicável a sub-passos
  individuais.
- **Sem cláusulas condicionais nos sub-passos `.B`+
  do plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-eliminacao-counter-state-legacy-passo-190a.md`
  com 8-10 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-190a-relatorio.md`
  com 14 secções produzido.
- Inventário completo produzido.
- 9 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (esperado:
  9 sub-passos B-J).
- Magnitude consolidada confirmada (L
  cross-modular).
- Critério de fecho M6 fixado.
- ADR avaliada (esperado: ADR-0070 PROPOSTO).
- DEBT M6 documentação estado registado.
- Estratégia de eliminação fixada (β recomendada).
- Forma final fixada (α recomendada).
- API pública analisada.
- Regra dos 2 eixos aplicada empiricamente.
- Pattern ADR-0069 reaproveitado se aplicável.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.869 inalterados.
- `crystalline-lint .` zero violations.

P190A é instrumento. Eliminação concreta de
`CounterStateLegacy` começa em P190B (categoria 1)
e segue até P190I (eliminação final + relatório).

**Após P190 série fechar**: F1 fecha; F3
parcialmente fecha; struct eliminado; walk puro;
Layouter migrado; 4 helpers eliminados ou migrados.
Desbloqueia M7 (loop fixpoint) e M8 (memoização
comemo).

**Risco arquitectural moderado-alto**: P190 é
trabalho cross-modular sem precedente directo no
projecto. Cláusulas gate substanciais prováveis
durante implementação P190B+. Auditoria
diagnóstico-primeiro em P190A reduz incerteza mas
não a elimina.
