# Passo 186A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.783 verdes; zero violations.
- M9 ✅ 11/11 (slot 11 livre).
- M5/M4 progresso: 6/12 read-sites migrados.
- DEBT M4-residual cobre apenas C1 + C2.
- ADR-0068 ACEITE. Layouter location-aware via
  `current_location: Option<Location>` populado em
  `layout_content` gating. Trait `Introspector` 18
  métodos.
- P186 é o último pedaço de infraestrutura M4-residual:
  promove `Content::Equation` a locatable (eixo 2 do
  bloqueio P183C). Após P186, C2 fica pronto para
  migração em P188.

P186 não migra consumer. Apenas:
- Adiciona variant `ElementPayload::Equation { ... }`.
- Marca `Content::Equation` como locatable em
  `is_locatable`.
- Adiciona arm em `extract_payload` para produzir
  payload.
- Adiciona arm em `from_tags` para popular sub-store
  apropriado para chave `"equation"`.

Padrão replicado: P181 (Bibliography promoção) + P178
(Outline) + P182C (SetHeadingNumbering promoção). Os
três precedentes oferecem template literal.

Material de partida verificado:

- `00_nucleo/diagnosticos/diagnostico-p183c-bloqueio.md`
  — P183C identificou eixo 2 falha para C2: variant
  `ElementPayload::Equation` ausente; arm em `from_tags`
  ausente; `CounterRegistry` nunca recebe entry para
  chave `"equation"`. Promoção a locatable é o caminho
  de desbloqueio.
- `00_nucleo/materialization/typst-passo-185-relatorio-consolidado.md`
  §8 — P186 listado como pré-requisito para C2 com
  magnitude S esperada.
- `01_core/src/rules/introspect/locatable.rs:11` —
  invariante explícito `is_locatable(c) ↔
  extract_payload(c).is_some()`. Promoção exige edits
  em ambos.
- `00_nucleo/adr/typst-adr-0026-content-enum-fechado.md`
  — Content como enum fechado; promover variant a
  locatable não muda enum mas muda cobertura.

P186A é o passo de diagnóstico que precede a
implementação. Sem decisões fixadas, P186B+ herda
problema do plano monolítico.

---

## Postura do auditor / executor

P186A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P181A/P182A/P183A/P184A/P185A.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  o exigir — improvável (replicação de padrão).
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** `extract_payload`, `is_locatable`,
  `from_tags`, `Content` enum, sub-stores — P186B+.

**Magnitude diagnóstico**: S. Decisões esperadas são
locais (forma do payload, sub-store alvo, convenção de
chave). Sem ADR substancial.

---

## Escopo

**Primário**: desenhar promoção de `Content::Equation` a
locatable kind no Introspector.

**Confirmação**: validar inventário factual — forma actual
do variant `Content::Equation`, ausência de
`ElementPayload::Equation`, ausência de arm em
`from_tags`, sub-store apropriado para counter equation.

**Decisões a tomar** — 6 cláusulas:

1. **Forma do `ElementPayload::Equation`** — que campos
   carrega o payload? (`numbering: Option<...>`,
   `block: bool`, `body: ?`, `label: Option<Label>`,
   etc.)

2. **Sub-store alvo** — onde popular dados para chave
   `"equation"`? (`CounterRegistry` provavelmente, em
   simetria com `figure:{kind}` per P184B; mas
   confirmar.)

3. **Convenção de chave** — `equation` simples vs
   `equation:numbered` per-mode vs alternativas. Equations
   em vanilla típicamente têm um único contador global,
   diferente de figures (que têm contador per-kind). Mas
   confirmar.

4. **Auto-init em `from_tags::StateUpdate`** — semelhante
   ao caso de P182C (`SetHeadingNumbering`)? Se equations
   incluem state update (improvável mas possível),
   confirmar comportamento.

5. **Forma de migração de consumer C2 (P188)** —
   substitution-with-fallback (replica P184D) com
   `flat_counter_at("equation", current_location)` per
   P185B. Confirmar.

6. **Critério de fecho de P186** — Opção 3 simétrica com
   P181/P182/P184 (infra pronta + walk popula sub-store;
   consumer migra em P188).

**Fora de escopo**:

- Migração consumer C1 (P187 — heading prefix usa
  `formatted_counter_at`).
- Migração consumer C2 (P188 — equation counter usa
  `flat_counter_at`).
- Walk puro M5 (P189).
- Eliminação `CounterStateLegacy` M6 (P190).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "Content::Equation\|EquationElem" 01_core/src/`.
Para cláusula 1, confirmar forma actual do variant
`Content::Equation` em `01_core/src/entities/content.rs`.
Para cláusula 2, inventariar sub-stores existentes e os
seus métodos de populate.

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusula 3
(convenção), 2 alternativas viáveis (chave simples vs
sufixo). Para cláusula 1 (forma do payload), depende da
forma actual do variant — pode ser determinada por
campos disponíveis.

### O3 — Critério de escolha

Padrão estabelecido em P181/P182C/P184B. Simetria com
figures (`figure:{kind}` em P184B) sugere `equation:?`
mas equations não têm kind. Convenção em vanilla typst
para chave de counter equation.

### O4 — Magnitude

Trivial vs substancial. Cada cláusula é independente.
Cláusula 1 é potencialmente substancial se variant tem
muitos campos e payload precisa de subset cuidadoso.

### O5 — Reversibilidade

Reversível ou fixa direção cara mudar.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Variant locatable replica padrão P181F (Bibliography),
P178 (Outline), P182C (SetHeadingNumbering)? Promoção
em 4 sítios uniformes (`is_locatable`, `extract_payload`,
`from_tags`, sub-store).

### Q2 — Honestidade de magnitude

P186A diagnóstico é S. P186B+ implementação:
- P186B: refinar `is_locatable` + L0 (S).
- P186C: adicionar variant `ElementPayload::Equation` +
  L0 (S).
- P186D: arm em `extract_payload` (S).
- P186E: arm em `from_tags` (S).
- P186F: tests integration + relatório (S).

Total agregado P186B–F: ~80-150 LOC produção + ~80 LOC
tests ≈ S. Magnitude S esperada para a série inteira.

### Q3 — Cobertura sem regressão

Promover Equation a locatable adiciona Tag emitida no
walk. Consumers existentes que iteravam `kind_index`
podem agora ver Equations onde antes não viam. Confirmar
empiricamente que nenhum consumer assume "Equations não
estão em kind_index". Per P185D auditor já confirmou
empiricamente que Equation não dispara gating actual —
P186 muda isto.

### Q4 — Fechamento de C2

Após P186, C2 (`equation.rs:97`) está pronto para
migração em P188:
- Eixo 1 (semântica temporal): resolvido por P185
  (location-aware).
- Eixo 2 (dados em sub-store): resolvido por P186
  (sub-store populado).

Cláusula 6 fecha quando ambos eixos atendidos para C2.
Migração concreta é P188.

### Q5 — Granularidade

5 sub-passos típicos para passo S agregado: variant +
locatable + extract_payload + from_tags + tests/relatório.
Pode ser comprimido em menos se cada peça for trivial.

---

## Sub-passos de P186A

### Sub-passo 186A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Forma do variant `Content::Equation` actual:
   - `01_core/src/entities/content.rs` — localizar
     `Equation { ... }` ou similar.
   - Inventariar campos: `body`, `block`, `numbering`,
     `label`, etc.

2. `ElementPayload` actual:
   - `01_core/src/entities/element_payload.rs` (ou
     similar).
   - Confirmar **ausência** de variant `Equation`.
   - Listar variants existentes (referência: per P184A,
     há `Figure { kind: Option<String>, counter_update,
     is_counted, .. }`, etc.).

3. `is_locatable` cobertura actual:
   - `01_core/src/rules/introspect/locatable.rs`.
   - Confirmar arm `Content::Equation { .. } => false`
     (per P185A §3.5 e P185D `.C` validado
     empiricamente).

4. `extract_payload` cobertura actual:
   - `01_core/src/rules/introspect/extract_payload.rs`.
   - Confirmar **ausência** de arm para
     `Content::Equation`.

5. `from_tags` cobertura actual:
   - `01_core/src/rules/introspect/from_tags.rs`.
   - Confirmar ausência de arm para `ElementPayload::Equation`
     (que ainda não existe).

6. Sub-store apropriado:
   - `CounterRegistry` (per P184B padrão para counter
     keys) é o candidato natural.
   - Verificar se há outros sub-stores que possam ser
     mais apropriados.
   - Confirmar API de populate (`apply_at(key, update,
     loc)` per P184B).

7. Walk legacy actual para equations:
   - `grep -rn "equation" 01_core/src/rules/introspect.rs`.
   - Walk legacy popula `state.numbering_active["equation"]`
     ou similar? Confirmar.

8. Vanilla typst — solução equivalente:
   - `grep -rn "EquationElem\|Equation" lab/typst-original/crates/`.
   - Como vanilla resolve consultas de contador equation?

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída**:
- Inventário completo. Decisões cláusulas 1-3 informadas
  por dados empíricos.

### Sub-passo 186A.B — Decisão cláusula 1 (forma do `ElementPayload::Equation`)

Avaliar campos a incluir no payload baseado no inventário
`.A.1`.

**Opção A** — Payload mínimo:
```
ElementPayload::Equation {
    block: bool,
    label: Option<Label>,
}
```
Suficiente para counter populate (block decide se conta;
label opcional para resolved labels futuros).

**Opção B** — Payload completo:
```
ElementPayload::Equation {
    block: bool,
    label: Option<Label>,
    counter_update: CounterUpdate,
    is_counted: bool,
}
```
Replica padrão P184B `ElementPayload::Figure` com
`counter_update` + `is_counted` explícitos.

**Opção C** — Payload paralelo a Heading:
```
ElementPayload::Equation {
    block: bool,
    counter_update: Option<CounterUpdate>,
    label: Option<Label>,
}
```

Critério: simetria com `Figure` (P184B) sugere Opção B.
Mas equations podem não precisar de `is_counted` se
"contar" é equivalente a "block: true && numbering:
Some(_)". Confirmar empiricamente.

Output: decisão fixada com justificação literal.

### Sub-passo 186A.C — Decisão cláusula 2 (sub-store alvo)

Avaliar onde popular dados para chave `"equation"`.

**Opção 1** — `CounterRegistry` simples (per P184B
padrão): chave `"equation"` ou `"equation:counter"`.

**Opção 2** — Sub-store dedicado `EquationStore` (análogo
a `BibStore` P181 ou novo).

Critério: Equations têm counter incremental simples (1,
2, 3...) sem hierarquia per-kind como figures. Opção 1
é suficiente. Sub-store dedicado seria overhead.

Output: Opção 1 esperado. Confirmar.

### Sub-passo 186A.D — Decisão cláusula 3 (convenção de chave)

**Opção A** — Chave `"equation"` simples (sem sufixo).

**Opção B** — Chave `"equation:counter"` ou
`"equation:numbered"` (com sufixo per convenção
P182A `<feature>:<sub-feature>`).

**Opção C** — Chave alinhada com legacy
`state.numbering_active["equation"]` se essa chave já
está em uso.

Critério: P182A padrão `<feature>:<sub-feature>` é para
distinguir variantes. Equations não têm variantes
(diferente de figures que têm kind). Sufixo seria
redundante.

Sugestão: **Opção A** (`"equation"` simples). Confirmar
que não colide com chave legacy ou outra convenção.

Output: decisão fixada.

### Sub-passo 186A.E — Decisão cláusula 4 (auto-init em `from_tags`)

Equation precisa de auto-init similar a P182C (per
`SetHeadingNumbering`)? Improvável — equations não são
state updates, são counter steps.

Confirmar empiricamente:
- `from_tags` arm para `ElementPayload::Equation` chamará
  `counters.apply_at(key, update, loc)`.
- `apply_at` em `CounterRegistry` per P171/P184B é
  defensivo (init na primeira ocorrência) — sem
  necessidade de auto-init explícito como P182C.

Output: confirmar que arm de equation não precisa de
tratamento especial. Padrão `apply_at` cobre.

### Sub-passo 186A.F — Decisão cláusula 5 (forma migração C2 em P188)

Confirmar que P188 vai usar:
```
self.introspector
    .flat_counter_at("equation", self.current_location.unwrap())
    .or_else(|| self.counter.get_flat("equation"))
    .unwrap_or(0)  // ou heurística adequada
```

Substitution-with-fallback per P184D padrão. Trait method
`flat_counter_at` existe per P185B.

Output: decisão registada para P188 (não implementada
em P186).

### Sub-passo 186A.G — Decisão cláusula 6 (critério de fecho de P186)

P186 fecha quando:

- **Opção 1** — Variant `ElementPayload::Equation`
  adicionado + `is_locatable` retorna true para
  `Content::Equation` + `extract_payload` produz payload
  + `from_tags` popula sub-store + tests integration
  confirmam Tag emitida + sub-store populado.

- **Opção 2** — Opção 1 + tests E2E confirmando
  `flat_counter_at("equation", loc)` retorna valor
  correcto via pipeline real.

Critério: Opção 2 dá rigor e prepara P188. Equivale ao
padrão P184E (tests E2E pré-migração consumer).

Sugestão: **Opção 2**. Replica padrão.

Output: critério literal verificável.

### Sub-passo 186A.H — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Adicionar variant `ElementPayload::Equation` + L0 | S | — |
| `.C` | Modificar `is_locatable` para `Content::Equation` + L0 | trivial | — |
| `.D` | Adicionar arm em `extract_payload` + L0 | S | `.B`, `.C` |
| `.E` | Adicionar arm em `from_tags` para popular `CounterRegistry` + L0 | S | `.B`, `.D` |
| `.F` | Tests integration + E2E paridade `flat_counter_at("equation", loc)` + relatório consolidado | S | `.B`-`.E` |

Sequência fixa B → C → D → E → F. Sem cláusulas
condicionais.

Output: tabela final.

### Sub-passo 186A.I — ADR

Avaliar:

- Promoção de variant a locatable é refino dentro de
  Content enum fechado (ADR-0026) — **não ADR**.
- Forma do payload replica P184B (Figure) + P181F
  (Bibliography) — **não ADR**.
- Sub-store alvo `CounterRegistry` é reuso — **não ADR**.

Conclusão esperada: **não cria ADR**.

### Sub-passo 186A.J — Outputs

Produzir 3 ficheiros (padrão P181A–P185A):

1. **`00_nucleo/diagnosticos/diagnostico-equation-locatable-passo-186a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–6 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Relação com P183C bloqueio (eixo 2 desbloqueado
     após P186).
   - §8 Próximo sub-passo (P186B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-186a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/P182A/etc.).

3. **Sem ADR e sem DEBT esperados** (replicação de
   padrão).

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar `is_locatable`** — P186C.
- **Não adicionar variant ao `ElementPayload`** — P186B.
- **Não adicionar arm em `extract_payload`** — P186D.
- **Não adicionar arm em `from_tags`** — P186E.
- **Não migrar consumer C2** — P188.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: se variant `Content::Equation`
  tem campos inesperados que dificultem payload mínimo,
  registar como tal.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-equation-locatable-passo-186a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-186a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de 5 sub-passos sem condicionais.
- Magnitude S agregada confirmada.
- Critério de fecho C2-eixo-2 fixado.
- ADR avaliada (esperado: não criada).
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.783 inalterados.
- `crystalline-lint .` zero violations.

P186A é instrumento. Promoção concreta de
`Content::Equation` a locatable começa em P186B.
