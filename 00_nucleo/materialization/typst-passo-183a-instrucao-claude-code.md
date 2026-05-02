# Passo 183A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace verdes (1.738 ou superior).
- `crystalline-lint .` zero violations.
- M1, M2, M3 ✅ concluídos (sub-stores + locatable kind).
- M9 ✅ 10/11 (P181 fechou lacuna #6); P182A fixou decisões
  para fechar lacuna #4 (M9 11/11).
- M4 (per desenho original em P156k–P175): "migrar consumers
  para Introspector" — **incompleto**. 2/6 consumers migrados
  (cite-arm em P181G; figure-ref em P168). 4 consumers
  restantes lêem `CounterStateLegacy` directamente: outline,
  counter_helpers, section-arm, layout_equation.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-181j-relatorio.md` §5
  — confirma "M5: 2/6 consumers migrados" (terminologia do
  projecto; corresponde a M4 do desenho original).
- `00_nucleo/diagnosticos/auditoria-fresh-projecto.md` (F1)
  — `CounterStateLegacy` 18 fields ainda lidos por consumers
  não-migrados.
- Padrão de migração estabelecido em P168 (figure-ref) e
  P181G (cite-arm): substitution-with-fallback via
  `Introspector` trait method.

P183A é o passo de diagnóstico que precede a migração dos 4
consumers restantes. Sem decisões fixadas em P183A, P183B+
herda o problema do plano monolítico que padrão P181A/P182A
evita.

---

## Postura do auditor / executor

P183A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P154A, P181A, P182A. Aplicam-se as restrições
padrão:

- **Zero código tocado** em `01_core/`, `02_shell/`,
  `03_infra/`, `04_wiring/`.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural o
  exigir.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não migra consumers** — esse é P183B em diante.

O executor lê material como contexto factual já validado.
Não re-inventaria estruturas estabelecidas. P183A consome
inventários existentes e produz **decisões + plano
executável**.

**Sem cláusulas condicionais nos sub-passos `.B`+ do plano.**
P183A fixa direcção; P183B+ não tem "FULL vs INVENTORY_ONLY".

---

## Escopo

**Primário**: migração dos 4 consumers restantes de
`CounterStateLegacy` para `Introspector` trait.

**Confirmação**: validar que os 4 consumers continuam a
ser:
- `layout_outline` (presumivelmente em
  `01_core/src/rules/layout/outline.rs` ou similar).
- `counter_helpers` (presumivelmente em
  `01_core/src/rules/layout/counters.rs`).
- `section-arm` (presumivelmente em
  `01_core/src/rules/layout/references.rs` ou similar).
- `layout_equation` (em
  `01_core/src/rules/layout/equation.rs`).

**Decisões a tomar** — 6 cláusulas:

1. **Lista exacta de consumers** (confirmar 4 ou ajustar
   contagem; localização ficheiro:linha).
2. **Bloqueios por consumer** (lacunas abertas que impedem
   migração — auditoria fresh menciona lacuna #3 outline body
   bloqueando `layout_outline`).
3. **Métodos trait necessários** (que métodos novos
   `Introspector` trait precisa para servir cada consumer;
   alguns podem reusar `state_value`, `final_value`, etc. de
   P171/P175/P176/P177).
4. **Ordem de migração** (consumers triviais primeiro? ou
   bloqueados primeiro para destrancar?).
5. **Forma de migração** (substitution-with-fallback per P168
   ou substituição directa).
6. **Critério de fecho de M4** (todos os 4 migrados, ou
   apenas os não-bloqueados; lacuna #3 fica para passo
   próprio).

**Fora de escopo**:

- Migração concreta dos consumers (P183B+).
- Adição de métodos novos ao trait `Introspector` (P183B+
  conforme decisão da cláusula 3).
- Walk puro (P184A — M5).
- Eliminação de `CounterStateLegacy` (P185A — M6).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "CounterStateLegacy\|counter_state_legacy"
01_core/src/rules/layout/` para identificar todos os
call-sites. Ficheiro:linha por consumer.

### O2 — Alternativas consideradas

Mínimo 2 quando há margem real. Para cláusula 4 (ordem),
mínimo 2 (triviais primeiro / bloqueados primeiro).

### O3 — Critério de escolha

ADR existente, padrão estabelecido (P168, P181G), invariante,
custo de implementação.

### O4 — Magnitude da decisão

Trivial vs substancial.

### O5 — Reversibilidade

Reversível ou fixa direção cara mudar depois.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Plano replica P168 (figure-ref) e P181G (cite-arm)?

### Q2 — Honestidade de magnitude

4 consumers × magnitude S = magnitude S agregada se padrão
trivial. Se algum consumer revelar trabalho L+, registar
revisão.

### Q3 — Bloqueios identificados

Lacuna #3 outline body é bloqueio real ou rastreio
desactualizado? Confirmar empiricamente.

### Q4 — Fechamento de M4

Critério é "4/4 migrados" ou "4/4 ou explicitamente bloqueado
com DEBT"? Auditor seguinte deve poder marcar M4 como ✅ sem
ambiguidade.

### Q5 — Granularidade dos sub-passos

Cada consumer = 1 sub-passo, ou agrupar consumers triviais
num só? Decisão de granularidade.

---

## Sub-passos de P183A

### Sub-passo 183A.A — Validação do estado actual

Auditor confirma empiricamente:

- `grep -rn "CounterStateLegacy" 01_core/src/rules/layout/`
  e registar todos os call-sites.
- Para cada call-site, identificar a função/arm e o que é
  lido (`state.bib_*`, `state.numbering_active`,
  `state.heading_*`, etc.).
- Confirmar que P181G (cite-arm) e P168 (figure-ref) já
  migraram os seus call-sites.
- Identificar o que sobra após excluir os já migrados.

Output: tabela com linha por call-site não-migrado.

### Sub-passo 183A.B — Decisão cláusula 1 (lista exacta)

Confirmar 4 consumers. Pode ser mais ou menos do que
relatado em P181J §5. Ajustar contagem com base em .A.

Output: tabela com nome, ficheiro, linha, função, fields
lidos.

### Sub-passo 183A.C — Decisão cláusula 2 (bloqueios)

Para cada consumer, verificar se há lacuna aberta em
`m1-lacunas-captura.md` que o referencia. Lacuna #3
(`outline body em state vs hash em tags`) é candidato
óbvio para `layout_outline`.

Output: tabela com consumer + bloqueio (sim/não/qual lacuna).

### Sub-passo 183A.D — Decisão cláusula 3 (métodos trait)

Para cada consumer, identificar o que `Introspector` trait
precisa expor. Comparar com métodos já existentes:

- `state_value`, `final_value` (P171).
- `bib_entry_for_key`, `bib_number_for_key` (P181F).
- `is_numbering_active` (P182B se já materializado).
- Outros documentados no trait actual.

Se algum consumer pede método novo, decidir nome + assinatura.

Output: tabela com consumer + método consumido (existente ou
novo).

### Sub-passo 183A.E — Decisão cláusula 4 (ordem)

Duas opções:

**Opção A** — triviais primeiro (consumers sem bloqueio,
método trait já existe). Valida o padrão de migração; cada
sub-passo é S puro.

**Opção B** — bloqueados primeiro (resolver lacuna #3 antes;
desbloqueia outline). Risco: lacuna #3 pode ser L+.

Critério: P168 e P181G estabeleceram que consumers triviais
migram fácil. Sugestão: Opção A.

Output: ordem fixada.

### Sub-passo 183A.F — Decisão cláusula 5 (forma de migração)

Substitution-with-fallback (P168) vs substituição directa.

Substitution-with-fallback:
```rust
let val = self.introspector.bib_number_for_key(key)
    .or_else(|| self.counter.bib_numbers.get(key).copied());
```

Substituição directa:
```rust
let val = self.introspector.bib_number_for_key(key);
```

Critério: durante M4 (consumer migrado, walk ainda muta
state), substitution-with-fallback dá segurança. Substituição
directa só após M5 (walk puro).

Output: decisão = substitution-with-fallback (replica P168
e P181G).

### Sub-passo 183A.G — Decisão cláusula 6 (critério de fecho M4)

**Opção 1** — 4/4 migrados.

**Opção 2** — 4/4 ou bloqueado-com-DEBT. Permite fechar M4
mesmo que `layout_outline` continue legacy se lacuna #3 for
L+.

**Opção 3** — definir M4 como "consumers não-bloqueados
migrados; consumers bloqueados ficam como DEBT M4-residual".

Critério: P181 fechou lacuna #6 com Opção 3 análoga
(infraestrutura + consumer migrado; legacy preservado até
M6). Simétrico.

Output: critério literal verificável.

### Sub-passo 183A.H — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Consumer | Magnitude | Depende |
|-----------|----------|-----------|---------|
| `.B` | counter_helpers | S | — |
| `.C` | section-arm | S | — |
| `.D` | layout_equation | S | — |
| `.E` | layout_outline (ou DEBT se bloqueado) | S ou DEBT | lacuna #3 |
| `.F` | tests E2E paridade | S | `.B`–`.E` |
| `.G` | M4 fechado em registo + relatório | S | `.F` |

Output: tabela final.

### Sub-passo 183A.I — ADR

Avaliar se decisões justificam ADR nova:

- Cláusulas replicam P168/P181G/P171/P181F: **não ADR**.
- Cláusula 6 com Opção 3 replica P181 simetricamente: **não
  ADR**.

Conclusão esperada: **não cria ADR**.

### Sub-passo 183A.J — Outputs

Produzir 3 ficheiros:

1. **`00_nucleo/diagnosticos/diagnostico-m4-consumers-passo-183a.md`**
   — diagnóstico com 8 secções.

2. **`00_nucleo/materialization/typst-passo-183a-relatorio.md`**
   — relatório de fecho com 14 secções (padrão P181A/P182A).

3. **Actualização de auditoria fresh F1** — registar que P183
   ataca metade do problema (consumers); F1 só fecha
   completamente após M5+M6.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não migrar consumers** — esse é P183B+.
- **Não modificar walk** — esse é P184A.
- **Não eliminar `CounterStateLegacy`** — esse é P185A.
- **Sem cláusulas condicionais nos sub-passos `.B`+**.
- **Honestidade obrigatória**: se consumer revelar trabalho
  L+, registar como tal.

---

## Critério de conclusão

- Diagnóstico produzido (8 secções).
- Relatório produzido (14 secções).
- 6 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais.
- Magnitude consolidada.
- Critério de fecho de M4 fixado.
- ADR avaliada (esperado: não criada).
- Nenhum ficheiro de cristalino tocado.
- Tests inalterados.

P183A é instrumento. Migração concreta começa em P183B.
