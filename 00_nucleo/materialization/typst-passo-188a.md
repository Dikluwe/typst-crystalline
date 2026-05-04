# Passo 188A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.805 verdes; zero violations.
- M9 ✅ 11/11 (slot 11 livre).
- M5/M4 progresso: 7/12 read-sites migrados (C1 fechado em
  P187B; C3 fechado em P184D).
- DEBT M4-residual cobre apenas C2 (cenário B per P187A
  §8 — sem DEBT formal aberto; nota preventiva).
- ADR-0068 ACEITE. Layouter location-aware via
  `current_location: Option<Location>` (P185C).
- Trait `Introspector` 18 métodos incluindo
  `flat_counter_at(key, location)` (P185B).
- Equation locatable estruturalmente — P186 série fechada
  com gate dormente em produção.

P188 fecha **C2 equation counter**. **Última peça
funcional de M4-residual**. Após P188, M4-residual
fechado; DEBT M4-residual vazio em prática; segue M5
(P189).

C2 tem 2 diferenças importantes face a C1 (P187):

1. **Primitiva diferente**: `flat_counter_at(key,
   location) -> Option<usize>` (P185B) em vez de
   `formatted_counter_at(key, location) -> Option<String>`
   (P177). Counter equation é flat (1, 2, 3...), não
   hierárquico.

2. **Introspector dormente em produção**: per P186A §11.2,
   `Content::SetEquationNumbering` não existe em
   cristalino. State `numbering_active:equation` nunca é
   populado em produção real → gate em `from_tags`
   bloqueia → counter introspector permanece vazio. P188
   resulta em **fallback legacy permanente** como caminho
   funcional, até equation set rule materializar.

P188 é replicação de padrão P187B com primitiva
location-aware adaptada e honestidade sobre estado
dormente.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-186-relatorio-consolidado.md`
  §8 — P188 listado como próximo após P186F; blueprint
  similar a P187B com Introspector dormente.
- `00_nucleo/materialization/typst-passo-187-relatorio-consolidado.md`
  §8 — P188 confirmado como independente; blueprint
  para C2.
- `01_core/src/rules/layout/equation.rs:97` — site C2
  actual (ou similar; confirmar empiricamente em `.A`).

P188A é o passo de diagnóstico que precede a
implementação. Magnitude S esperada — replicação de
padrão P187B com primitiva diferente.

---

## Postura do auditor / executor

P188A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P181A/P182A/P183A/P184A/P185A/P186A/P187A.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  exigir — improvável (replicação de padrão P187B).
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** trait `Introspector`, Layouter consumer,
  `Locator` — P188B+.

**Magnitude diagnóstico**: S. Decisões esperadas são
locais (forma exacta de migração, tratamento do `None` do
Introspector, documentação honesta do estado dormente).

---

## Escopo

**Primário**: desenhar migração de C2 equation counter
(`equation.rs:97`) para `flat_counter_at(key,
current_location)` com fallback legacy.

**Confirmação**: validar que infraestrutura P185+P186 está
acessível no site exacto + que padrão P187B é aplicável +
que estado dormente é correctamente caracterizado.

**Decisões a tomar** — 7 cláusulas:

1. **Forma exacta da expressão de migração** — variante
   da substitution-with-fallback, paralela a P187B mas
   com primitiva `flat_counter_at`.

2. **Tratamento do `None` do Introspector** — em produção
   real, este é o caso **central** (não defensivo como em
   C1). Counter equation introspector permanece sempre
   vazio até equation set rule materializar. Fallback
   legacy é caminho funcional **permanente**.

3. **Tratamento do `None` do `current_location`** —
   simétrico a P187B Opção B (`and_then` defensivo).
   Confirmar que site C2 tem `current_location` populado
   (Equation é locatable após P186D).

4. **Acesso a `self.introspector` e `self.current_location`
   no site C2** — `equation.rs` é módulo separado de
   `mod.rs`. Confirmar se `Layouter` é o `self` ou se
   acesso é diferente.

5. **Forma de retorno** — `flat_counter_at` retorna
   `Option<usize>`. `format_hierarchical` legacy retorna
   `Option<String>`. Para C2, qual é o tipo final
   esperado? `usize` para contador? `String` formatado?
   Confirmar conversão.

6. **Documentação honesta do estado dormente** — P188 é
   o primeiro caso onde Introspector path migra mas é
   **permanentemente dormente** em produção. Documentar:
   - Comentário inline explicando que fallback legacy é
     caminho funcional.
   - Cross-reference a P186A §11.2.
   - Apontar trabalho futuro (`Content::SetEquationNumbering`
     materialização).

7. **Critério de fecho de P188** — Opção 3 (consumer
   migrado + tests E2E + DEBT M4-residual fecha). Após
   P188, M4-residual fechado; DEBT vazio em prática.

**Fora de escopo**:

- `Content::SetEquationNumbering` materialização (passo
  fora série).
- Walk puro M5 (P189).
- Eliminação `CounterStateLegacy` M6 (P190).

---

## Critérios objectivos

Para cada decisão das 7 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "get_flat\|flat_counter_at\|equation" 01_core/src/rules/layout/`.
Para cláusula 1, confirmar contexto exacto da leitura em
`equation.rs:97`. Para cláusula 4, confirmar acesso a
`self` (Layouter) ou outro receptor.

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusula 5 (forma
de retorno), 2 alternativas viáveis (manter String legacy
vs converter `usize` → `String`).

### O3 — Critério de escolha

Padrão P187B replicado para C2. Sem decisão arquitectural
nova esperada.

### O4 — Magnitude

Trivial vs substancial. Cláusulas 1-5 são triviais;
cláusula 6 (documentação) é S; cláusula 7 é confirmação.

### O5 — Reversibilidade

Substitution-with-fallback é reversível. Documentação
inline também.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Migração replica P187B literalmente? Diferença esperada:
primitiva `flat_counter_at` em vez de
`formatted_counter_at` + tipo `usize` em vez de `String`.

### Q2 — Honestidade de magnitude

P188A diagnóstico é S. P188B+ implementação:
- Provável série de **2 sub-passos** (B + C agregado, ou
  B único agregado como P187B).

Total agregado: ~10 LOC produção + ~80 LOC tests +
documentação inline ≈ S.

### Q3 — Cobertura sem regressão

Tests existentes que cobrem equation counter: identificar
em `.A`. Migração não deve regredir nenhum.

### Q4 — Honestidade sobre estado dormente

C2 é o primeiro consumer onde Introspector path migra mas
**não é caminho funcional em produção**. Diferente de:
- P184D Figure: Introspector funcional.
- P187B C1 heading: Introspector funcional.
- P186 Equation locatable: infra existe; sem consumer.
- **P188 C2 equation counter**: Introspector path migra
  mas dormente; fallback legacy é caminho funcional
  permanente.

Documentação inline obrigatória — sem isto, leitores
futuros podem assumir que P188 fecha completamente o
caminho Introspector para C2 (não fecha — apenas estrutura).

### Q5 — Granularidade dos sub-passos P188B+

Provável passo único agregado P188B (similar a P187B):
- Migração consumer.
- Tests E2E (incluindo paridade legacy + caso central de
  estado dormente).
- L0 actualizado.
- Documentação inline.
- Relatório consolidado P188.
- Actualização nota DEBT M4-residual.

---

## Sub-passos de P188A

### Sub-passo 188A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar consumer C2 actual:
   - `01_core/src/rules/layout/equation.rs:97` (per
     P186A §2). Confirmar empiricamente.
   - Localizar leitura: padrão esperado
     `state.get_flat("equation")` ou
     `self.counter.get_flat("equation")` ou similar.
   - Identificar contexto exacto (função/método;
     escopo das variáveis locais).

2. Confirmar receptor (`self` é Layouter ou outro):
   - `equation.rs` é submódulo de `rules/layout/`?
     Imports? Tipo de `self`?
   - Se `self` é Layouter: acesso a `self.introspector`
     e `self.current_location` directo.
   - Se não: identificar como aceder.

3. Confirmar API `flat_counter_at`:
   - Trait method P185B: `flat_counter_at(&self, key:
     &str, location: Location) -> Option<usize>`.
   - Confirmar empiricamente.

4. Confirmar `current_location` no site:
   - Equation é locatable após P186D. Layouter avança
     `current_location` antes do arm Equation em
     `layout_content`.
   - Site C2 (`equation.rs:97`) é dentro do arm Equation?
   - Confirmar empiricamente.

5. Confirmar tipo de retorno legacy:
   - `state.get_flat("equation")` retorna `Option<usize>`
     ou `Option<Vec<usize>>` ou outro?
   - Confirmar API legacy em
     `counter_state_legacy.rs` ou similar.

6. Confirmar como counter equation é usado downstream:
   - `equation.rs:97` retorna o valor para onde?
   - Como é formatado para display?
   - String final tem forma `"(1)"`, `"1"`, ou outra?

7. Confirmar tests existentes que cobrem equation counter:
   - `grep -rn "equation.*counter\|equation_number\|fn .*equation" 01_core/src/rules/layout/`.
   - Identificar tests que devem manter-se inalterados
     após P188B.

8. Confirmar nota DEBT M4-residual em relatórios:
   - Per P187 §7: notas em P184F/P185-consolidado/
     P186-consolidado mencionam C1+C2; P187 actualizou
     para "apenas C2"; P188 actualiza para "vazio em
     prática" (C2 fechado estruturalmente; fallback legacy
     funcional).

9. Confirmar achado P186A §11.2:
   - `Content::SetEquationNumbering` ausente em
     cristalino. Confirmar empiricamente:
     `grep -rn "SetEquationNumbering" 01_core/src/`.
     Esperado: zero hits.
   - Confirmar comentário inline em `equation.rs:25-29`
     que P186A documentou.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída**:
- Site C2 localizado.
- Receptor identificado.
- `flat_counter_at` API confirmada.
- Tipo de retorno determinado.
- Estado dormente confirmado empiricamente.

### Sub-passo 188A.B — Decisão cláusula 1 (forma da expressão)

Avaliar a forma exacta da migração.

**Opção A** — Inline directo (paralelo P187B):
```
self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .or_else(|| self.counter.get_flat("equation"))
```

**Opção B** — Variável intermédia.

**Opção C** — Match explícito.

Critério: P187B usou Opção A combinação `and_then` +
`or_else`. Replica padrão.

Sugestão: **Opção A** com forma idêntica a P187B
substituindo apenas primitiva (`formatted_counter_at` →
`flat_counter_at`) e chave (`"heading"` → `"equation"`).

Output: decisão fixada com base em `.A`.

### Sub-passo 188A.C — Decisão cláusula 2 (tratamento `None` do Introspector)

`flat_counter_at("equation", location)` retorna `None`:
- Em produção real: **sempre**, porque gate dormente
  bloqueia populate.
- Em tests com state injectado: `Some(n)` quando counter
  populado.

**Opção A** — Cair em fallback legacy (replica P187B):
```
.or_else(|| self.counter.get_flat("equation"))
```

**Opção B** — Default value (`unwrap_or(0)` ou similar).

**Opção C** — Mistura.

Critério: P187B usou Opção A. Replica padrão. **Diferença
honesta face a C1**: em C1, fallback é defensivo (raramente
disparado em produção); em C2, fallback é o **caminho
funcional permanente** (sempre disparado em produção).
Decisão é a mesma; semântica documentada é diferente.

Sugestão: **Opção A**.

Output: decisão fixada com nota sobre semântica dormente.

### Sub-passo 188A.D — Decisão cláusula 3 (tratamento `None` do `current_location`)

Simétrico a P187B Opção B. `and_then` defensivo:
```
self.current_location.and_then(|loc| ...)
```

Sem panic. Cai em fallback legacy se `current_location`
for `None`.

Confirmar empiricamente em `.A.4` que site C2 tem
`current_location` populado (Equation locatable; gating
precede arm).

Output: decisão fixada (Opção B).

### Sub-passo 188A.E — Decisão cláusula 4 (acesso ao receptor)

Depende de `.A.2`. 3 cenários:

**Cenário 1** — `self` é Layouter directamente:
- Acesso `self.introspector` + `self.current_location`
  funciona inline. Sem mudança.

**Cenário 2** — `self` é outro tipo (ex.: `EquationLayouter`,
helper struct):
- Receber `&Layouter` como argumento adicional ou
  reorganizar acesso.

**Cenário 3** — Função livre que recebe `Layouter`:
- Aceder via `layouter.introspector` + `layouter.current_location`.

Sugestão: dependente de `.A`. Esperado **Cenário 1** (per
P184D Figure que migrou no mesmo módulo `mod.rs`).

Output: cenário identificado e plano de acesso fixado.

### Sub-passo 188A.F — Decisão cláusula 5 (forma de retorno)

`flat_counter_at` retorna `Option<usize>`.
`format_hierarchical` legacy retorna `Option<String>` —
**não compatível directamente**.

`get_flat` legacy retorna `Option<usize>` — **compatível**.

Confirmar em `.A.5` qual é o legacy real.

**Opção A** — Se legacy é `get_flat -> Option<usize>`:
substituição directa preserva tipo.

**Opção B** — Se legacy é diferente: conversão necessária.

Sugestão: **Opção A** esperado. Se Opção B aplicar,
adicionar conversão na expressão.

Output: tipo final fixado.

### Sub-passo 188A.G — Decisão cláusula 6 (documentação inline)

P188 é o primeiro consumer onde Introspector path migra
mas **não é caminho funcional em produção**. Documentação
inline obrigatória.

**Opção A** — Comentário simples:
```
// Path Introspector dormente em produção até
// `Content::SetEquationNumbering` materializar (vide
// P186A §11.2). Fallback legacy é caminho funcional
// permanente.
```

**Opção B** — Documentação extensiva com cross-references.

**Opção C** — Documentação no L0 apenas (sem inline).

Sugestão: **Opção A** (comentário curto inline +
cross-reference).

Output: forma de documentação fixada.

### Sub-passo 188A.H — Decisão cláusula 7 (critério de fecho)

P188 fecha quando:
- Consumer C2 migrado.
- Tests E2E confirmam paridade observable (output
  Layouter idêntico legacy vs Introspector path quando
  state injectado).
- Tests E2E confirmam comportamento dormente em produção
  (sem state → Introspector retorna `None` → fallback
  legacy fornece valor).
- DEBT M4-residual actualizado: vazio em prática (era C2
  apenas após P187B).

Output: critério literal verificável.

### Sub-passo 188A.I — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Migrar consumer C2 + L0 + tests E2E + documentação inline + actualização DEBT M4-residual + relatório consolidado P188 | S | — |

Sub-passo único agregado (similar a P187B). Razão:
migração + tests + actualização DEBT é trabalho coeso e
pequeno.

**Alternativa**: dividir em 2 sub-passos (B = migração;
C = tests + DEBT). Decisão depende de magnitude empírica
de cada peça em `.A`.

Sugestão: **passo único P188B** se `.A` confirmar que
trabalho cabe em ~80-150 LOC total.

Output: tabela final.

### Sub-passo 188A.J — ADR

Avaliar:

- Substitution-with-fallback é padrão P187B — não ADR.
- `flat_counter_at` já existe (P185B) — não ADR.
- `current_location` já existe (P185C) — não ADR.
- Semântica dormente em produção é honestidade
  documental, não decisão arquitectural — não ADR.

Conclusão esperada: **não cria ADR**.

### Sub-passo 188A.K — DEBT

P188 fecha o último caso coberto pelo DEBT M4-residual
(C2). Cenário B per P187A §8: sem DEBT formal aberto.

P188B actualiza nota preventiva no relatório consolidado
P188 indicando:
- Após P188: DEBT M4-residual **vazio em prática**.
- C1 fechado em P187B (Introspector funcional).
- C2 fechado em P188B (Introspector dormente; fallback
  legacy funcional permanente).
- P183F formal pode ser dispensado (DEBT vazio antes de
  abrir formalmente).

Output: cenário identificado.

### Sub-passo 188A.L — Outputs

Produzir 3 ficheiros (padrão P181A–P187A):

1. **`00_nucleo/diagnosticos/diagnostico-c2-equation-counter-passo-188a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação (DEBT M4-residual vazio em
     prática após P188).
   - §7 Estado dormente honestamente documentado.
   - §8 Próximo sub-passo (P188B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-188a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT novo esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não migrar consumer C2** — P188B.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar Layouter struct** — P185C fechou.
- **Não modificar P186 (Equation locatable)** — P186F
  fechou.
- **Não materializar `Content::SetEquationNumbering`** —
  passo dedicado fora da série.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória sobre estado dormente**:
  P188 não fecha C2 funcionalmente como P187B fechou C1.
  P188 fecha C2 estruturalmente (Introspector path
  presente); fallback legacy é caminho funcional
  permanente em produção. Esta diferença deve ser
  registada explicitamente em todos os outputs.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-c2-equation-counter-passo-188a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-188a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de 1 sub-passo (B agregado) sem condicionais.
- Magnitude S agregada confirmada.
- Critério de fecho C2 fixado (incluindo estado dormente).
- ADR avaliada (esperado: não criada).
- DEBT M4-residual cenário identificado (vazio em prática
  após P188).
- Estado dormente honestamente documentado em todas as
  secções relevantes.
- Achado P186A §11.2 confirmado empiricamente
  (`SetEquationNumbering` ausente).
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.805 inalterados.
- `crystalline-lint .` zero violations.

P188A é instrumento. Migração concreta de C2 começa em
P188B.
