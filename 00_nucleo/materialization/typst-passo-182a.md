# Passo 182A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- 1.738 tests workspace verdes (per P181J consolidado).
- `crystalline-lint .` zero violations.
- Série P181 fechada; lacuna #6 ✅ resolvida; M9 10/11.
- Lacuna #4 (`is_numbering_active` / `numbering_active`)
  ainda aberta. P181A §estado lacunas confirmou
  StateRegistry (P171) disponível como infraestrutura
  base candidata.

Material de partida verificado:

- `00_nucleo/diagnosticos/m1-lacunas-captura.md` — registo
  da lacuna #4 com texto:

  > `CounterStateLegacy.numbering_active: HashMap<String, bool>`
  > controla por chave se a numeração está activada
  > (populado pelo walk arm `Content::SetHeadingNumbering`).
  > `TagIntrospector` não captura este estado. Consumer
  > típico: `Layouter` consulta `is_numbering_active("heading")`
  > antes de formatar prefixo de heading.
  >
  > Possíveis caminhos:
  > - Adicionar variant locatable `SetHeadingNumbering` a
  >   `ElementPayload`.
  > - Adicionar campo `numbering_state: HashMap<String, bool>`
  >   a `TagIntrospector` populado por extracção paralela
  >   em `from_tags`.

- `00_nucleo/materialization/typst-passo-181j-relatorio.md`
  §5 — confirma `numbering_active` como opção 1 do
  caminho à frente; magnitude estimada S-M; padrão P181
  a replicar.

- `00_nucleo/diagnosticos/auditoria-fresh-projecto.md`
  (2026-04-29) — F1 (`CounterStateLegacy` 18 fields)
  ainda em aberto; `numbering_active` é um dos 18.

P182A é o passo de diagnóstico que precede a
implementação. Sem decisões fixadas em P182A, P182B
herda o problema do plano monolítico que P181 evitou.

---

## Postura do auditor / executor

P182A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P154A e P181A. Aplicam-se as restrições padrão:

- **Zero código tocado** em `01_core/`, `02_shell/`,
  `03_infra/`, `04_wiring/`.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural o
  exigir.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não materializa** consulta de state em Layouter, não
  adiciona método trait, não toca walk arm. Esse trabalho
  é P182B em diante.

O executor lê material de partida como **contexto factual
já validado**. Não re-inventaria estruturas estabelecidas
(StateRegistry P171, padrão P175/P176/P177/P181F).
P182A consome inventários existentes e produz **decisões +
plano executável**.

**Decisão arquitectural ausente em P181A — P182A toma-a
explicitamente**: não há gate "FULL vs INVENTORY_ONLY".
P182A fecha o mecanismo (M1/M2/M3) com base em inventário
empírico. Se M3 for o caminho, P182A regista magnitude
maior e plano com mais sub-passos — mas o plano é
executável e P182B+ não tem cláusulas condicionais.

---

## Escopo

**Primário**: lacuna #4 conforme delimitada em
`m1-lacunas-captura.md`.

**Confirmação**: validar que `numbering_active` em
`CounterStateLegacy` continua na forma documentada
(`HashMap<String, bool>`); que walk arm
`Content::SetHeadingNumbering` continua a ser quem o
popula; que Layouter heading-arm é o consumer único
identificado.

**Decisões a tomar** — 6 cláusulas:

1. **Mecanismo** (M1 state via P171 / M2 sub-store
   dedicado / M3 document-level config).
2. **Default value** quando state ausente em local.
3. **Lista de consumers** (Layouter heading-arm apenas
   ou múltiplos).
4. **Localização exacta do consumer** (ficheiro:linha
   actual, arm específico).
5. **Forma da API** (`state_value` directo vs helper
   `Introspector::is_numbering_active(key, location)`).
6. **Critério de fecho** da lacuna #4.

**Fora de escopo**:

- Modificação de Layouter heading-arm (P182B+).
- Adição de stdlib func nova se necessária (P182B+).
- Modificação de walk arm `Content::SetHeadingNumbering`
  (P182B+ se mecanismo escolhido o exigir).
- Eliminação do field `numbering_active` legacy de
  `CounterStateLegacy` (M6).
- Outras lacunas (#1, #2, #3 — não pertencem a P182).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

Que ficheiros / linhas / fields foram inspeccionados para
confirmar que a decisão é aplicável. Citação directa
(ficheiro:linha) onde aplicável. Para cláusula 1
(mecanismo), inputs incluem inventário vanilla
(`grep -rn "numbering_active\|is_numbering_active"` em
`lab/typst-original/`) e inventário cristalino.

### O2 — Alternativas consideradas

Listar as opções pesadas. Para cláusula 1: M1, M2, M3
(três obrigatórias por já estarem no P182 anterior).
Para outras: mínimo 2 quando há margem real.

### O3 — Critério de escolha

Que regra arquitectural justifica a opção escolhida.
ADR existente, invariante de walk puro, simetria com
sub-store anterior, custo de implementação, ou simetria
com vanilla.

### O4 — Magnitude da decisão

Trivial (sem efeito a montante / a jusante) vs
substancial (afecta API pública, exige ADR nova,
muda invariante).

### O5 — Reversibilidade

Decisão é reversível em sub-passo posterior sem custo
prático? Ou fixa direcção que será cara mudar depois?

---

## Critérios qualitativos

Para o plano agregado:

### Q1 — Consistência com padrão estabelecido

Plano replica padrão "feature consulta state via
Introspector" (P175/P176/P177/P181F) ou inventa estrutura
nova?

### Q2 — Honestidade de magnitude

Estimativa S-M de P181J §5 continua válida após decisões
fixadas? Se mecanismo escolhido for M3, registar revisão
explícita para M+.

### Q3 — Simetria com vanilla

Onde cristalino diverge de vanilla, divergência é
intencional e documentada?

### Q4 — Fechamento de lacuna

Critério de fecho da lacuna #4 (cláusula 6) é
verificável? Auditor seguinte consegue confirmar
"fechada" sem julgamento subjectivo?

### Q5 — Granularidade dos sub-passos

Sub-passos `.B`+ propostos são S-M individualmente? Ou
algum esconde trabalho L+?

---

## Sub-passos de P182A

P182A é trabalho de leitura + decisão. Sequência
sugerida:

### Sub-passo 182A.A — Validação do estado actual

Auditor confirma empiricamente:

- `01_core/src/entities/counter_state_legacy.rs` contém
  `numbering_active: HashMap<String, bool>` (linha a
  registar).
- Walk arm `Content::SetHeadingNumbering` em
  `01_core/src/rules/introspect.rs` continua a ser quem
  popula `state.numbering_active` (linha actual).
- `01_core/src/entities/content.rs` tem variant
  `Content::SetHeadingNumbering { key, value }` ou
  similar (forma exacta a registar).
- Layouter consumer: localizar leitura de
  `is_numbering_active` ou `numbering_active` em
  `01_core/src/rules/layout/mod.rs`. Registar
  ficheiro:linha. Se não existir leitura activa,
  registar — significa que a lacuna #4 nunca chegou a
  ter consumer real e P182 muda de natureza.

Output: tabela com linha por item, "confirmado" /
"desviado" / "ausente". Se houver desvio, registar
forma actual e verificar se afecta decisões P182A.

### Sub-passo 182A.B — Inventário vanilla

Auditor procura em `lab/typst-original/`:

- `grep -rn "numbering_active\|is_numbering_active"`
  em `lab/typst-original/crates/`.
- Para cada match, registar ficheiro, linha, contexto
  (1-3 linhas).
- Determinar:
  - **Forma de armazenamento** (campo de struct,
    state via mecanismo equivalente, document config).
  - **Quem lê** (qual rule/elem/layouter consulta).
  - **Quem escreve** (qual rule/heading set rule muta).
  - **Default value** (ON ou OFF quando ausente).
  - **Granularidade** (booleano global, por key
    "heading"/"figure"/etc, hierárquico por nível).

Output: notas estruturadas em prosa curta (5-15 linhas)
por mecanismo identificado. Se vanilla tem múltiplos
mecanismos para `numbering_active` em diferentes
contextos, registar cada um.

### Sub-passo 182A.C — Decisão cláusula 1 (mecanismo)

Avaliar M1 / M2 / M3 contra critérios O1–O5.

Inputs verificáveis: outputs de .A e .B. Padrão
estabelecido em P171 (StateRegistry) e P175/P176/P177/
P181F (consumer consulta via Introspector).

Considerar:

- **M1** (state via P171) é compatível se vanilla usa
  state-style (cristalino capitaliza P171, sem cascade
  novo).
- **M2** (sub-store dedicado) replica P165/P169/P171 mas
  sem benefício se M1 viável; rejeitar excepto se
  razão técnica força.
- **M3** (document-level config) exige mecanismo novo
  em cristalino — sem precedente; magnitude M+.

Critério de escolha esperado: se .B confirma vanilla
state-style, escolher M1; se .B revela document-level
config sem state, escolher M3 e registar magnitude
revisada.

Output: decisão fixada com justificação literal +
referência ao padrão replicado.

### Sub-passo 182A.D — Decisão cláusula 2 (default value)

Vanilla ON ou OFF quando state ausente?

Inputs: .B inventário vanilla deve ter resposta directa.
Se vanilla testa explicitamente `state.is_some() &&
state.unwrap() == true`, default é OFF. Se testa
`state.unwrap_or(true)`, default é ON.

Cristalino actual: `CounterStateLegacy.numbering_active`
inicializa como `HashMap::new()`; `is_numbering_active`
hipotético lê com `.get(key).copied().unwrap_or(?)` —
auditor confirma o `?` empiricamente se método existir,
ou regista que método não existe e default deve ser
fixado em P182.

Output: decisão (ON / OFF) com referência a vanilla.

Magnitude: trivial.

### Sub-passo 182A.E — Decisão cláusula 3 (lista de consumers)

Quantos consumers reais existem?

Inputs: .A.4 já localizou Layouter consumer. Auditor
adicionalmente verifica:

- `grep -rn "numbering_active\|is_numbering_active"
  01_core/src/rules/`.
- Cada match: é leitura activa, comentário,
  identificador parcial?

Se múltiplos consumers reais (heading + outline +
section + ...), magnitude cresce — cada um precisa
migração no padrão P168/P181G.

Output: lista numerada de consumers com ficheiro:linha.

Magnitude: substancial se >1 consumer; trivial se
apenas 1 (esperado).

### Sub-passo 182A.F — Decisão cláusula 4 (localização exacta)

Para cada consumer identificado em .E, registar:

- Ficheiro.
- Linha.
- Função ou arm (ex.: `Content::Heading` arm em
  `Layouter::layout_content`).
- Forma actual da leitura (`state.is_numbering_active(key)`
  ou `state.numbering_active.get(key)` ou outra).

Output: tabela. Esta informação alimenta P182B+
implementação directa sem re-inventário.

### Sub-passo 182A.G — Decisão cláusula 5 (forma da API)

Duas opções:

**Opção A1** — `state_value` directo:
```rust
let on = self.introspector
    .state_value("numbering_active:heading", location)
    .map(|v| matches!(v, Value::Bool(true)))
    .unwrap_or(<default>);
```

**Opção A2** — helper trait method:
```rust
let on = self.introspector
    .is_numbering_active("heading", location);
```

Critério de escolha: P181F adicionou `bib_entry_for_key`
+ `bib_number_for_key` ao trait — precedente para A2.
P175/P176 usaram state directo — precedente para A1.

Considerar:
- A1 tem zero cascade de trait (sem ADR de extensão).
- A2 encapsula `Value::Bool` matching, mais legível em
  consumer.
- Magnitude trabalho: A1 = trivial; A2 = trivial mas
  toca trait.

Output: decisão com justificação. Magnitude trivial.

### Sub-passo 182A.H — Decisão cláusula 6 (critério de fecho)

Lacuna #4 fecha quando:

**Opção 1** — infraestrutura pronta (state populado +
método disponível) **mesmo que** consumer não migrado.

**Opção 2** — infraestrutura pronta **e** consumer
migrado **e** `numbering_active` legacy redundante.

**Opção 3** — infraestrutura pronta + consumer migrado;
fields legacy permanecem até M6.

Critério de escolha: P181A fixou Opção 3 para lacuna #6.
Simetria sugere Opção 3 para lacuna #4. Se mecanismo
escolhido em .C for M1, simetria mantém-se;
se M3, critério pode mudar.

Output: critério literal verificável.

### Sub-passo 182A.I — Validação do plano de sub-passos

P182 anterior propunha estrutura mas com cláusulas
condicionais ("apenas se .A escolheu prosseguir").
P182A elimina condicionais — produz plano executável.

Estrutura esperada para mecanismo M1:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | (se A2) trait method `is_numbering_active` | S | — |
| `.C` | walk arm `SetHeadingNumbering` adapta para também emitir state via P171 (se necessário) | S | `.B` |
| `.D` | Layouter consumer migra para Introspector | S-M | `.B`, `.C` |
| `.E` | tests E2E | S | `.D` |
| `.F` | lacuna #4 marcada fechada + relatório | S | `.E` |

Para mecanismo M3 (improvável), plano cresce — auditor
produz tabela apropriada.

Output: tabela final com sub-passo, escopo, magnitude,
dependência. Sem cláusulas "apenas se".

### Sub-passo 182A.J — ADR

Avaliar se decisões justificam ADR nova:

- Se cláusula 1 escolheu M1 e replica P171/P175/P176/
  P177/P181F: **não ADR**.
- Se cláusula 5 escolheu A2 e replica P181F: **não
  ADR**.
- Se cláusula 1 escolheu M3 (mecanismo novo): **ADR
  PROPOSTO** com justificação.

Conclusão esperada: P182A **não** cria ADR (caminho M1
+ A2). Se auditor identificar excepção, criar
`PROPOSTO`.

### Sub-passo 182A.K — Outputs

Produzir 3 ficheiros (mesmo padrão de P181A):

1. **`00_nucleo/diagnosticos/diagnostico-numbering-active-passo-182a.md`**
   — diagnóstico com 8 secções:

   - §1 Validação estado actual.
   - §2 Inventário vanilla.
   - §3 Decisões cláusula 1–6 (uma sub-secção cada,
     formato O1–O5 + opção escolhida + justificação
     literal).
   - §4 Plano de sub-passos sem condicionais (tabela).
   - §5 Magnitude consolidada.
   - §6 ADR avaliação (se sim, link; se não, justificar).
   - §7 DEBT avaliação.
   - §8 Próximo sub-passo (P182B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-182a-relatorio.md`**
   — relatório de fecho do passo, com 14 secções
   numeradas no padrão P154A/P181A:

   - §1 Sumário.
   - §2 Validação estado actual.
   - §3 Inventário vanilla.
   - §4 Decisões cláusula 1–6 (síntese).
   - §5 Plano de sub-passos.
   - §6 Magnitude consolidada.
   - §7 ADR avaliação.
   - §8 DEBT avaliação.
   - §9 Plano de materialização (P182B+).
   - §10 ADR (se houver, senão "não produzido").
   - §11 DEBTs (se houver, senão "não aberto").
   - §12 `m1-lacunas-captura.md` actualizado.
   - §13 Próximo passo (P182B com escopo concreto).
   - §14 Verificação final (tabela com itens cumpridos).

3. **Actualização de
   `00_nucleo/diagnosticos/m1-lacunas-captura.md`** —
   linha lacuna #4 evolui de:

   ```
   **Decisão**: adiar para passo dedicado (M9 ou similar)
   que adicione mecanismo de numbering-active ao
   Introspector. Possíveis caminhos: ...
   ```

   para:

   ```
   **P182A decisões fixadas**: 6 cláusulas resolvidas
   (link diagnóstico). Mecanismo: {M1/M2/M3}. Default:
   {ON/OFF}. Consumers: {N}. API: {A1/A2}. Critério de
   fecho: {Opção 1/2/3}. Plano de N sub-passos
   .B–.{F/G/...} validado. Próximo: P182B (link).
   ```

---

## Restrições

- **Zero código tocado** em qualquer ficheiro de
  cristalino fora de `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não materializar** consulta de state em Layouter —
  esse é P182B+.
- **Não adicionar trait method** — esse é P182B se
  cláusula 5 escolher A2.
- **Não modificar walk arm** — esse é P182 sub-passo
  posterior.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: se decisão for trivial,
  registar como trivial; se for substancial, registar
  como substancial. Sem "no entanto isto é compensado
  por..." que apaga magnitude.
- **Não pedir confirmação ao humano antes de fixar
  decisões**: P182A toma-as com critérios; humano lê
  depois.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**. P182A fixa direcção; P182B+ não tem
  "FULL vs INVENTORY_ONLY". Se P182A não consegue
  fixar a direcção (input ausente em vanilla, por
  exemplo), regista a falta como bloqueio e abre DEBT —
  não como gate condicional.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-numbering-active-passo-182a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-182a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem cláusulas condicionais —
  tabela com escopo + magnitude + dependência.
- `m1-lacunas-captura.md` actualizado.
- Magnitude consolidada (S / S-M / M+ / M conforme
  decisões).
- Critério de fecho lacuna #4 fixado em palavras
  verificáveis.
- ADR avaliada (criada com `PROPOSTO` se necessária; ou
  justificação literal de "não necessária").
- Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
  `04_wiring/` tocado.
- `cargo test --workspace --lib`: 1.738 inalterados.
- `crystalline-lint .`: zero violations.

P182A é instrumento. Implementação concreta da lacuna #4
fica para P182B em diante.
