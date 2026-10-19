# Passo 189A — Instrução Claude Code

> **Nota de origem**: este ficheiro foi inicialmente
> redigido como "P184A" antes de P183B/C/D revelarem que
> M4 não fechava em P183 isolado. P184–P188 ficaram
> reservados para M4-residual. P189A é o trabalho M5,
> agora com pré-condições actualizadas e aprendizados
> incorporados (M4-residual fase completa em
> 2026-05-04 com 13 aplicações do padrão
> diagnóstico-primeiro).

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.808 verdes; zero violations.
- M1, M2, M3 ✅ concluídos.
- M9 ✅ 11/11 (slot 11 livre — fechado em P182F).
- **M4-residual fechado funcionalmente** após série
  P181–P188 (8 séries; ~30 passos materializados):
  - C1 fechado funcionalmente (P187B — Introspector
    funcional).
  - C2 fechado estruturalmente (P188B — Introspector
    **dormente em produção**; fallback legacy permanente
    até `Content::SetEquationNumbering` materializar).
  - C3 fechado funcionalmente (P184D — Introspector
    funcional).
  - 5 outros via P181G/P182D/P184D/P186 estabelecidos.
- M5/M4 progresso: **8/12 read-sites migrados**.
- DEBT M4-residual: vazio em prática.
- ADR-0068 ACEITE — Layouter location-aware.

**Objectivo M5** (per desenho original P163):
"remover mutação directa do walk". `CounterStateLegacy`
deixa de ser mutado durante walk; passa a ser construído
a partir de tags via `from_tags` ou eliminado.

P163 invariante actual: walk puro em arms locatable. M5
estende o invariante a **todos** os arms — incluindo
non-locatables que mutam state.

---

## Pré-condição rígida (com 2 reservas)

P189A só pode correr após M4-residual fechar. **M4-residual
fechado funcionalmente em P188B.** Mas existem 2 reservas
herdadas que P189 não fecha por si:

### Reserva 1 — `Content::SetEquationNumbering` ausente

Per P186A §11.2 + P188 §5: cristalino não tem variant
`Content::SetEquationNumbering`. Walk arm legacy em
`introspect.rs:377-382` é a **única fonte** que popula
`numbering_active:equation` em produção. Se P189 tornar
walk puro para essa mutação:
- State `numbering_active:equation` permanece `None`.
- Gate em P186E nunca dispara → counter introspector
  vazio.
- Fallback legacy P188B (`get_flat`) também retorna 0
  (sem mutação no walk → sem counter populado em legacy
  também).
- **Equation count em produção fica zero** — regressão
  observable.

P189 **não pode tornar walk puro para Equation** sem
materializar `SetEquationNumbering` primeiro. Equation
walk arm pode ficar **explicitamente excepcionado** de
M5 com nota cruzada para passo dedicado.

### Reserva 2 — C4 (resolved label) não migrado

Per P183E não corrido (P183 série fechou após D em falha
gate substancial; E nunca executou). C4 (resolved label
TOC) ainda lê legacy em runtime.

P189 não pode tornar walk puro para mutações que
populam `state.resolved_labels` se C4 ainda lê
directamente. Duas opções:
- Excepcionar C4 de M5 (bloqueio explícito).
- Migrar C4 antes de P189 (pequeno passo P183E retomado
  ou P189 estendido).

### Implicação para P189

P189 **não fecha M5 universalmente**. Fecha M5 para arms
onde infra está pronta (Heading numbering, Figure,
Bibliography, Outline) e **excepciona explicitamente**:
- Equation arm (depende `SetEquationNumbering`).
- Resolved label arm (depende C4 migration).

Estas excepções podem ser:
- (a) abertas como DEBT M5-residual com nota
  arquitectural; ou
- (b) executadas como pré-trabalho integrado em P189A `.A`.

Decisão fica para cláusula nova em `.A`.

---

## Postura do auditor / executor

P189A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 13 aplicações (P131A/132A/140A/148/154A/
181A/182A/183A/184A/185A/186A/187A/188A).

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Não modifica walk** — P189B+.
- **Não toca `from_tags`** — P189B+.
- **Sem cláusulas condicionais** nos sub-passos `.B`+.

**Regra dos 2 eixos aplicável** (P183C §6 consolidado em
M4-residual): para qualquer arm a auditar, validar
empiricamente:
- **Eixo 1 — semântica temporal**: o consumer downstream
  precisa do valor "durante walk" (Layouter mutável) ou
  "snapshot final" (TagIntrospector)?
- **Eixo 2 — existência de dados**: sub-store
  correspondente é populado para a chave em produção?

Se qualquer eixo falhar para um arm, declarar excepção
(reserva 1 ou 2 acima, ou nova).

**3 padrões pragmáticos** descobertos em M4-residual,
úteis para P189B+ se necessário:
- Ajustar fixture de teste em vez de violar restrição
  (P186C).
- Violar restrição justificadamente quando spec é
  internamente inconsistente (P186D).
- Inlining em vez de chamar trait method para evitar
  circularidade estilística (P186E).

---

## Escopo

**Primário**: identificar todos os walk arms que ainda
mutam `CounterStateLegacy` directamente; decidir como
cada um migra para "walk puro + from_tags popula state",
ou se fica excepcionado (reservas 1 ou 2).

**Decisões a tomar** — 7 cláusulas (era 6; +1 reserva):

1. **Lista exacta de arms não-puros** (`grep` em
   `01_core/src/engine/introspect.rs` por mutações de
   `state.*`).
2. **Estratégia por arm** — promoção a locatable (já
   feita para Bibliography/Outline/Equation), ou
   ElementPayload::StateUpdate (P171 padrão), ou
   excepção declarada (reserva 1 ou 2).
3. **Compatibilidade com sub-stores** — `LabelRegistry`,
   `CounterRegistry`, `MetadataStore`, `StateRegistry`.
   Mapear cada field do legacy para sub-store ou
   mecanismo.
4. **Backward compatibility durante transição** — walk
   puro mas legacy ainda existe. Quem popula legacy?
   Bridge em `from_tags`?
5. **Walk arm de `Content::Styled`** (caso especial — não
   é locatable mas pode afectar state via deltas).
6. **Excepções declaradas** (NOVO) — quais arms ficam
   fora de M5 com nota explícita?
   - Equation arm (Reserva 1).
   - Resolved label arm (Reserva 2).
   - Outros descobertos em `.A`?
7. **Critério de fecho de M5** — `grep` zero matches em
   arms (excluindo excepções declaradas) + tests E2E
   paridade.

**Fora de escopo**:

- Eliminação de `CounterStateLegacy` (P190 — M6).
- Loop fixpoint (P191 — M7).
- Memoização (P192 — M8).
- `Content::SetEquationNumbering` materialização (passo
  fora série).
- C4 (resolved label) migração (P183E retomado, ou
  passo dedicado).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "state\." 01_core/src/engine/introspect.rs`
filtrado por mutações (`state.x = ...`,
`state.x.push(...)`, `state.x.insert(...)`, etc.).

Para cada match, validar **empiricamente** com regra
dos 2 eixos: o consumer downstream precisa do valor
durante walk ou no snapshot final?

### O2 — Alternativas consideradas

Por arm: emitir Tag e popular via `from_tags` (Opção α
ou β); excepcionar (Opção γ). Mínimo 2 quando há
margem real.

### O3 — Critério de escolha

Padrão estabelecido (P162 extract_payload + from_tags;
P171 StateUpdate; P181 Bibliography).

### O4 — Magnitude

Trivial vs substancial. Cada arm é independente. Excepções
são triviais (sem trabalho de migração — apenas
documentação).

### O5 — Reversibilidade

Walk puro reversível (basta voltar a mutação directa).
Excepções reversíveis (basta migrar quando reserva
fechar).

---

## Critérios qualitativos

### Q1 — Consistência com padrão

Plano replica P162/P165/P169/P171/P177/P181E (locatable
kind + extract_payload + from_tags arm) ou P171
(StateUpdate puro)?

### Q2 — Honestidade de magnitude

Cada arm = sub-passo S. Se algum revelar L+, registar.

### Q3 — Cobertura completa com excepções declaradas

Plano cobre todos os arms identificados em `.A`? Cada arm
tem decisão (migrar ou excepcionar). Sem "resolveremos os
outros depois" — excepções são decisão consciente, não
adiamento.

### Q4 — Fechamento de M5

Critério verificável sem julgamento subjectivo:

```
grep -E "state\.\w+\s*[=.]" 01_core/src/engine/introspect.rs
```

Retorna zero matches em arms **excepto**:
- Inicialização.
- Arms excepcionados explicitamente (reservas 1, 2, ou
  outras descobertas em `.A`).

### Q5 — Granularidade

Cada arm = 1 sub-passo, ou agrupar arms similares?
Provável: agrupar StateUpdate-style arms (P171 cobre); 1
sub-passo por excepção declarada.

### Q6 — Honestidade sobre M5 não-universal

P189 fecha M5 **com excepções declaradas**, não
universalmente. Documentação obrigatória em 4 pontos
(replica padrão P188B):
1. Comentário inline em arms excepcionados.
2. Secção em L0 `rules/introspect.md`.
3. Test sentinela que valida excepção (counter retorna
   esperado em produção).
4. Relatório consolidado P189 §"Excepções M5".

---

## Sub-passos de P189A

### Sub-passo 189A.A — Validação do estado actual

Confirmar pré-condições:
- M4-residual fechado (P188B ✅).
- Tests workspace 1.808.

Inventariar mutações:
- `grep -rn "state\." 01_core/src/engine/introspect.rs`.
- Filtrar mutações vs leituras.
- Para cada mutação, identificar arm.

Aplicar regra dos 2 eixos a cada mutação:
- Eixo 1: consumer precisa de valor "durante walk" ou
  snapshot final?
- Eixo 2: sub-store correspondente populado em produção?
- Falha em qualquer eixo → candidato a excepção.

Confirmar Reservas 1 e 2:
- Reserva 1 (`SetEquationNumbering` ausente):
  `grep -rn "SetEquationNumbering" 01_core/src/` →
  esperado zero hits em produção (per P188 §5).
- Reserva 2 (C4 não migrado): localizar consumer C4
  resolved label em `01_core/src/engine/layout/`. Confirmar
  que ainda lê legacy directamente.

Confirmar estado real de Bibliography/Outline arms:
- Per P181 (Bibliography promoção) e P178 (Outline):
  arms emitem Tag, mutação centralizada em `from_tags`.
- **Verificar empiricamente** — auditor M4-residual
  descobriu várias vezes que specs antigas não
  reflectiam realidade (vide P186C `.A.6`).

Output: tabela com:
- Arm + linha + field mutado.
- Eixo 1 / Eixo 2 status.
- Decisão preliminar: migrar / excepcionar / já puro.

### Sub-passo 189A.B — Decisão cláusula 1 (lista de arms)

Confirmar lista empírica. Esperado (per relatórios M4-residual):

| Arm | Mutação | Estado esperado |
|-----|---------|-----------------|
| `Content::SetHeadingNumbering` | `state.numbering_active.insert("heading", ...)` | activo (P182C populates Tag StateUpdate; verificar se walk legacy ainda muta directamente) |
| `Content::Equation` block + numbering | `state.flat["equation"]` step | activo (Reserva 1) |
| `Content::Heading` | `state.headings_for_toc` push? auto-labels? | a verificar (lacuna #2 ainda?) |
| `Content::Bibliography` | (esperado puro pós-P181) | confirmar |
| `Content::Outline` | (esperado puro pós-P178) | confirmar |
| `Content::Figure` | (esperado puro pós-P184) | confirmar |
| `Content::Styled` | depende de deltas | a verificar |

**Aviso de honestidade**: a lista pode divergir
empiricamente. Auditor deve ler `introspect.rs` e
listar **o que está lá**, não o que specs antigas
prevêem. Vide P186C aprendizado sobre divergência spec
vs realidade.

Output: tabela com arms a migrar / excepcionar / já
puro.

### Sub-passo 189A.C — Decisão cláusula 2 (estratégia por arm)

Para cada arm não-puro:

**Opção α** — promover a locatable (se ainda não é):
adicionar `ElementKind` + `ElementPayload`;
`extract_payload` arm; `from_tags` arm popula sub-store.
Padrão P181/P186.

**Opção β** — não-locatable mas emite Tag para
state-update: arm emite `ElementPayload::StateUpdate
{ key, update }` (já existente per P171); `from_tags`
actualiza `StateRegistry`. Padrão P171/P182C.

**Opção γ** — caso especial (`Content::Styled`):
investigar empiricamente.

**Opção δ (NOVO)** — excepcionar do M5: arm permanece
não-puro com nota explícita. Aplicável a Reservas 1 e
2 (e quaisquer outras descobertas).

Output: tabela com arm + opção α/β/γ/δ + sub-store alvo
ou justificação de excepção.

### Sub-passo 189A.D — Decisão cláusula 3 (sub-store por field)

Mapear cada field de `CounterStateLegacy` para sub-store
ou mecanismo:

| Field legacy | Sub-store / mecanismo | Status |
|-------------|----------------------|--------|
| `numbering_active` | `StateRegistry` (P182) | activo |
| `figure_numbers`, `equation_numbers` | `CounterRegistry` (P165 + P177 + P185B `flat_counter_at`) | activo (Equation Reserva 1) |
| `bib_entries`, `bib_numbers` | `BibStore` (P181) | activo |
| `resolved_labels` | `LabelRegistry` (P165) | a confirmar; Reserva 2 |
| `headings_for_toc` | (lacuna #3 outline body) | pode ficar para passo dedicado |
| `auto_labels` | (lacuna #2) | pode ficar |
| `has_outline` | `query_by_kind(Outline)` (P178) | activo |
| Outros fields | a auditar empiricamente | — |

Output: tabela completa.

### Sub-passo 189A.E — Decisão cláusula 4 (backward compat)

Walk puro durante transição: legacy ainda existe
(consumers M4-fallback usam-no, especialmente C2 com
fallback permanente). Quem popula legacy?

**Opção A** — `from_tags` popula sub-store **e** copia
para legacy via bridge. Walk fica puro de verdade.

**Opção B** — walk arm emite Tag + adicionalmente popula
legacy directamente (não-puro durante transição).

**Opção C** — `from_tags` popula só sub-store; consumers
que ainda lêem legacy passam a ler do sub-store via
getter compat (legacy é facade em vez de storage).

**Implicação para Reserva 1**: para Equation, mesmo com
Opção A (bridge `from_tags` → legacy), o problema
permanece — `from_tags` arm Equation tem gate dormente
(P186E). Sem state activo, gate nunca dispara, bridge
nunca popula legacy. Equation precisa de **excepção
explícita** (Opção δ em cláusula 2), não é coberta pela
Opção A da cláusula 4.

Sugestão: **Opção A para arms migráveis**; excepção δ
para Reservas 1 e 2.

Output: decisão fixada.

### Sub-passo 189A.F — Decisão cláusula 5 (`Content::Styled`)

`Content::Styled([deltas], body)` aplica style deltas
durante walk. Não é locatable. Pode mutar state se delta
toca counter ou similar.

Verificar empiricamente: arm em walk muta state? Se sim,
qual? Pode ser que P171 já cobre via StateUpdate (cada
delta emite Tag).

Decisão depende: se muta, escolher opção α/β/γ/δ
(cláusula 2). Se não muta, é puro por design — sem
trabalho.

Output: decisão por confirmação empírica.

### Sub-passo 189A.G — Decisão cláusula 6 (excepções declaradas)

**Reserva 1 — Equation walk arm**:
- Justificação: `Content::SetEquationNumbering` ausente
  (P186A §11.2; confirmado P188 §5).
- Excepção: walk arm Equation **continua a mutar**
  `state.flat["equation"]` directamente.
- Documentação obrigatória (4 pontos per Q6):
  1. Comentário inline em `introspect.rs:377-382`.
  2. Secção em L0 com cross-reference.
  3. Test sentinela que valida produção (Equation
     count > 0 quando walk corre — paridade preservada).
  4. Secção em P189 consolidado.
- Quando reserva fechar (`SetEquationNumbering`
  materializado): excepção é removida; walk arm migra
  para Opção β.

**Reserva 2 — Resolved label walk arm**:
- Justificação: C4 consumer ainda lê legacy directamente
  (P183E não corrido).
- Excepção: walk arm que popula `state.resolved_labels`
  continua a mutar directamente.
- Documentação obrigatória idêntica.
- Quando reserva fechar (C4 migrado): excepção removida.

**Outras excepções**: depende de `.A`. Se algum arm não
for migrável por razão estrutural não-prevista, declarar
empiricamente.

Output: lista de excepções declaradas + justificação
literal + plano de fechamento.

### Sub-passo 189A.H — Decisão cláusula 7 (critério de fecho M5)

**Opção 1** — `grep "state\." 01_core/src/engine/introspect.rs`
em arms retorna zero (apenas leituras em headers/imports
e inicialização e excepções declaradas).

**Opção 2** — Opção 1 + auditor confirma manualmente que
arms são puros.

**Opção 3** — Opção 1 + tests E2E confirmam paridade
introspect-only vs walk-only para arms migrados.

Critério: Opção 3 dá maior segurança. Padrão P181I.

Output: critério literal verificável, com lista
explícita de excepções aceitáveis.

### Sub-passo 189A.I — Validação do plano de sub-passos

Tabela esperada (depende de `.B`):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar arm 1 (mais simples) + L0 | S |
| `.C` | Migrar arm 2 + L0 | S |
| `.D` | Bridge legacy (Opção A) | S |
| `.E` | Documentar excepções (Reservas 1+2) | S |
| `.F` | Tests E2E paridade + sentinelas | S |
| `.G` | Relatório consolidado P189 | S |
| `.H` | M5 fechado com excepções | trivial |

Magnitude consolidada: **S agregada para arms migráveis;
trivial para excepções**.

**Alternativa**: agrupar arms StateUpdate-style num só
sub-passo. Decisão depende de magnitude empírica de cada
arm em `.A`.

Output: tabela final.

### Sub-passo 189A.J — ADR

Esperado: **não cria**. Replica padrão estabelecido
(P162/P165/P169/P171/P177/P181E).

**Excepção**: se decisão sobre Reservas 1 e 2 exigir
ADR formal (improvável — são honestidade documental,
não decisão arquitectural nova), criar `PROPOSTO`.

Conclusão esperada: **não cria ADR**.

### Sub-passo 189A.K — DEBT

P189 abre **DEBT M5-residual** se Reservas 1 e 2 forem
honradas como excepções:
- C cobre Equation walk arm (até `SetEquationNumbering`).
- C cobre resolved_label walk arm (até C4 migrado).

Cenário a confirmar em `.A`:
- **Cenário A**: abrir DEBT M5-residual formal em
  `00_nucleo/`.
- **Cenário B**: nota preventiva no relatório
  consolidado P189 (paralelo a M4-residual cenário B).

Sugestão: **Cenário B** (paralelo a M4-residual). Sem
DEBT formal; nota preventiva.

Output: cenário identificado.

### Sub-passo 189A.L — Outputs

3 ficheiros padrão:

1. **`00_nucleo/diagnosticos/diagnostico-walk-puro-passo-189a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual + regra dos 2 eixos.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação (M5-residual cenário A/B).
   - §7 **Excepções declaradas** (Reservas 1+2 +
     descobertas).
   - §8 Próximo sub-passo (P189B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-189a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados** (Cenário B).

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P189B+.
- **Não tocar `from_tags`** — P189B+.
- **Não tocar sub-stores** — P189B+.
- **Não materializar `SetEquationNumbering`** — passo
  dedicado fora série.
- **Não migrar C4** — passo dedicado fora série (ou
  P183E retomado).
- **Sem cláusulas condicionais** nos sub-passos `.B`+.
- **Honestidade obrigatória sobre Reservas 1 e 2**: M5
  não fecha universalmente; excepções declaradas com
  documentação em 4 pontos.
- **Sem inflação retórica**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a cada mutação inventariada
  em `.A`.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walk-puro-passo-189a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-189a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais.
- Magnitude consolidada (S agregada para arms migráveis;
  trivial para excepções).
- Critério de fecho M5 verificável (com excepções
  declaradas).
- ADR avaliada (esperado: não criada).
- DEBT M5-residual cenário identificado (A ou B).
- Reservas 1 e 2 confirmadas empiricamente em `.A`.
- Lista empírica de mutações em `introspect.rs` produzida
  (não suposta a partir de specs antigas).
- Regra dos 2 eixos aplicada a cada mutação.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.808 inalterados.
- `crystalline-lint .` zero violations.

P189A é instrumento. Walk puro materializado em P189B+.
