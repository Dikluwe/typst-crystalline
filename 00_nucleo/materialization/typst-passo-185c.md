# Passo P185C — Layouter integration (`locator` + `current_location`)

Segundo passo de implementação P185 (após P185A diagnóstico
+ P185B trait methods).
Magnitude **M** genuíno — primeira introdução de `Locator`
no Layouter.

Implementa mecanismo M3 da ADR-0068 PROPOSTO: Layouter
ganha `locator: Locator` field e `current_location:
Location` field; gating em `layout_content` (ou método
equivalente) actualiza `current_location` antes de
processar conteúdo locatable, com save/restore para
preservar scoping léxico após children.

Após P185C:
- Layouter expõe `current_location` durante o seu próprio
  walk de layout.
- Locator do Layouter avança em sincronia com Locator do
  walk de introspect (sincronização-por-construção via
  determinismo + `is_locatable` gating).
- Nenhum consumer migra ainda (C1 fica para P187, C2 para
  P188).
- ADR-0068 candidato a transitar PROPOSTO → ACEITE após
  validação P185D.

**Pré-condição**: P185B concluído. Tests workspace 1.779
verdes; zero violations. Trait `Introspector` 18 métodos
(`is_numbering_active_at` + `flat_counter_at`
disponíveis). ADR-0068 PROPOSTO documenta mecanismo M3.

**Restrições**:
- **Não** modificar walk de introspect (P163 walk puro
  preservada).
- **Não** modificar `Locator` em si (apenas usar API
  existente).
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar `StateRegistry`, `CounterRegistry`.
- **Não** migrar C1 ou C2 — P187/P188.
- **Não** promover `Content::Equation` a locatable —
  P186.
- API pública preservada — assinatura de `layout` e
  `layout_with_introspector` não muda.
- Output observable em produção inalterado — `current_location`
  é state interno do Layouter sem consumer ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar Layouter actual:
   - `01_core/src/rules/layout/mod.rs` definição do struct.
   - Inventariar fields existentes (`cursor_x`, `cursor_y`,
     `current_line`, `figure_progress`, `counter`,
     `introspector`, etc.).
   - Identificar onde adicionar `locator` + `current_location`.

2. Confirmar `Locator` API:
   - `01_core/src/entities/locator.rs` (ou similar).
   - Construtor: `Locator::new()` ou similar (verificar
     se é `Default` derivado).
   - Método para avançar: `next()`, `step()`, ou similar.
   - Método para obter location actual (sem avançar):
     `current()`, `peek()`, ou similar.
   - Confirmar que o estado interno é mutável (`&mut self`
     para avançar) e que a sequência é determinística (per
     P185A §3.3 — provado por test).

3. Confirmar `is_locatable`:
   - `01_core/src/rules/introspect/locatable.rs`.
   - Função pública: `is_locatable(&Content) -> bool`.
   - Cobertura per P185A §3.5: Heading, Figure, Cite,
     Metadata, State, StateUpdate, Outline, Bibliography,
     SetHeadingNumbering. **Equation NÃO** (é P186).

4. Confirmar entry point `layout_content` (ou similar):
   - Método principal de despacho do Layouter.
   - Match sobre `Content::*` arms.
   - Onde inserir o gating de Locator (antes do match,
     dentro de cada arm, ou através de wrapper).

5. Confirmar `Layouter::new` e `layout_with_introspector`:
   - Construtor: como `locator` deve ser inicializado?
     Default novo? Recebido como argumento?
   - Decisão: cláusula 1 abaixo.

6. Confirmar L0 actual `rules/layout.md`:
   - Localizar entradas existentes documentando Layouter
     fields e arms.
   - Identificar onde adicionar entrada para mecanismo
     M3 (cf. ADR-0068).

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída**:
- Inventário completo. Decisões de cláusulas abaixo
  informadas por dados empíricos.

### .B Decisão cláusula 1 (inicialização do Locator)

**Opção A** — `Locator::new()` por defeito em `Layouter::new`.
Sem mudança de assinatura de construtor.

**Opção B** — Locator passado como argumento ao construtor
ou ao `layout_with_introspector`. Caller controla
inicialização.

**Opção C** — Locator partilhado com walk de introspect
via referência. Requer mudança de API.

Critério de escolha:
- Determinismo do Locator (per P185A §3.3) garante que
  `Locator::new()` em Layouter produz mesma sequência que
  walk de introspect — Opção A é correcta por
  construção.
- Opção B introduz acoplamento desnecessário.
- Opção C viola atomização e exige mudança de API.

Sugestão: **Opção A**. Sem mudança de assinatura pública.

Output: decisão fixada.

### .C Decisão cláusula 2 (gating em `layout_content`)

Onde colocar o gating "se `is_locatable(content)`,
avança Locator e actualiza `current_location`"?

**Opção α** — gating no topo de `layout_content` (antes
do match). Atómico; uma única chamada por content.

**Opção β** — gating dentro de cada arm que processa
content locatable (Heading, Figure, etc.). Múltiplas
chamadas; mais granular mas duplicado.

**Opção γ** — wrapper função `with_location(content,
fn)` que faz save/advance/run/restore.

Critério: Opção α replica padrão de walk de introspect
(que tem gating uniforme). Opção β duplica código. Opção
γ é Opção α encapsulada.

Sugestão: **Opção α** (com helper `advance_locator_if_locatable`
para clareza).

Output: decisão fixada.

### .D Decisão cláusula 3 (save/restore para scoping)

Após processar content locatable e seus children, deve o
`current_location` voltar ao valor anterior (pai)?

**Opção 1** — Sim, save/restore explícito antes/depois
do processamento.

**Opção 2** — Não, `current_location` avança
monotonicamente. Caller que precisa de scoping faz o seu
próprio save/restore.

Critério: walk de introspect avança monotonicamente
(Locator é cumulativo). Sincronização-por-construção
exige que Layouter faça o mesmo. Opção 2 alinha com walk
de introspect.

Mas para casos de scoping léxico (ex.: heading dentro de
container que tem o seu próprio counter scope), save/restore
local pode ser necessário.

**Decisão pendente** — verificar empiricamente em `.A` se
algum arm do Layouter precisa de scoping local. Se sim:
Opção 1. Se não: Opção 2 (mais simples, menos risco de
desincronização).

Sugestão: **Opção 2** por defeito; revisar se test E2E
em P185D detectar regressão.

Output: decisão fixada.

### .E Adicionar fields ao struct Layouter

1. Em `01_core/src/rules/layout/mod.rs`, struct Layouter:
   - Adicionar `locator: Locator`.
   - Adicionar `current_location: Location`.

2. Decisão sobre inicialização (cláusula 1 = Opção A):
   - `Locator::new()` (ou `Default::default()`) em
     `Layouter::new` e em `layout_with_introspector`.
   - `current_location: Location::default()` ou
     equivalente para "antes do walk começar".

3. Confirmar L0 hash actualiza após edit.

**Critério de saída**:
- `cargo check --workspace` passa.
- Tests existentes não regridem (estado novo sem consumer
  é inerte).
- Linter passa.

### .F Implementar gating em `layout_content`

1. Em `layout_content` (ou método equivalente):
   - No topo, antes do match: chamar
     `is_locatable(content)`.
   - Se `true`: avançar `self.locator` e actualizar
     `self.current_location` para o valor produzido.
   - Se `false`: não avançar.

2. Forma exacta fica para Claude Code conforme convenção
   do projecto. Decisão cláusula 2 = Opção α (gating
   atómico no topo).

3. (Opcional, conforme cláusula 3) Save/restore se
   identificado caso necessário em `.A`.

4. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- `cargo check --workspace` passa.
- Tests existentes não regridem.
- Linter passa.

### .G Sincronização: validação por construção

Tests E2E completos ficam para P185D (passo dedicado).
Em P185C, apenas confirmar que tests existentes não
regridem.

1. `cargo test --workspace --lib` antes do passo: 1.779
   verdes (baseline P185B).

2. `cargo test --workspace --lib` após `.E` + `.F`:
   - Δ esperado: 0 ou +N onde N são testes inerentes ao
     state novo (provavelmente 0 — `current_location`
     sem consumer não afecta nada observable).
   - Se Δ < 0: regressão. Investigar.

3. Cláusula gate substancial: se algum test existente
   regride, indica que mudança ao struct ou ao gating
   tem efeito colateral não-previsto. Reverter `.E`+`.F`
   antes de prosseguir.

**Critério de saída**:
- Δ tests = 0 ou justificado.
- Tests existentes não regridem.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P185B
   baseline (1.779): 0 esperado.
3. `crystalline-lint .` zero violations.
4. Layouter struct ganha `locator` + `current_location`
   fields.
5. `layout_content` faz gating no topo via
   `is_locatable`.
6. Walk de introspect **NÃO modificado**.
7. `Locator` API **NÃO modificada**.
8. Trait `Introspector` **NÃO modificado**.
9. Sub-stores **NÃO modificados**.
10. C1 e C2 **NÃO migrados** (P187/P188).
11. Snapshot tests ADR-0033 verdes.
12. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-185c-relatorio.md`
com:

- Resumo: Layouter ganha `locator` + `current_location`;
  gating no topo de `layout_content` actualiza
  `current_location` para content locatable;
  sincronização-por-construção via determinismo.
- Confirmação `.H` (12 verificações).
- Δ tests vs baseline P185B (esperado 0).
- Hashes finais de L0 modificado (`rules/layout.md`).
- Decisões de execução notáveis.
- Estado actual:
  - P185 série: A ✅ B ✅ C ✅ | D-E pendentes.
  - Layouter location-aware **estruturalmente** (sem
    consumer ainda).
  - 46 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P185D (tests E2E confirmando
  sincronização Locator do Layouter ↔ Locator do walk de
  introspect).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Cláusulas 1-3 fixadas (inicialização, gating,
   save/restore).
3. L0 `rules/layout.md` actualizado.
4. Layouter struct ganha 2 fields.
5. `layout_content` faz gating.
6. Tests existentes não regridem (Δ 0).
7. Verificações `.H` passam (12/12).
8. Relatório `.I` escrito.
9. Output observable em produção inalterado.

---

## O que pode sair errado

- **`Locator` não é `Default` ou requer parâmetros**:
  cláusula gate trivial — ajustar inicialização.
- **Field `Location` exige `Default`**: cláusula gate
  trivial — usar valor sentinel ou `Option<Location>`.
- **`is_locatable` exige import específico**: cláusula
  gate trivial.
- **`layout_content` não é o método principal de
  despacho**: cláusula gate trivial — ajustar nome
  ou identificar método equivalente.
- **Múltiplos métodos de despacho** (ex.: `layout_inline`,
  `layout_block` separados): cláusula gate trivial —
  aplicar gating em cada um, ou refactor para método
  comum.
- **Gating no topo causa regressão em algum arm que já
  avança Locator implicitamente**: cláusula gate
  substancial. Investigar duplicação. **Esperado raro**
  porque P185A confirmou que Layouter não usa Locator.
- **Tests existentes regridem por efeito colateral
  inesperado** (ex.: ordering changed): cláusula gate
  substancial. Reverter e investigar.
- **Save/restore necessário descoberto durante `.F`**:
  cláusula gate trivial — adicionar conforme cláusula 3
  Opção 1.
- **Locator avança mas `current_location` não actualiza
  correctamente** (ex.: API do Locator retorna location
  diferente do esperado): cláusula gate trivial — ler
  documentação do Locator e adaptar.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: M genuíno. ~50-100 LOC produção (struct
  fields + construtor inicialização + gating). Sem tests
  novos em P185C (tests E2E em P185D).
- **Sem dependências externas novas**.
- **Pré-condição P185D**: este passo concluído.
- **Padrão**: M3 da ADR-0068 PROPOSTO. Não tem precedente
  cristalino directo (figure_progress P184 é cursor mas
  não Locator).
- **Cláusula gate trivial**: aplicável a forma de
  inicialização, defaults, imports, edge cases de gating.
- **Cláusula gate substancial**: aplicável apenas se
  gating duplicar com mecanismo existente ou se tests
  regridirem inesperadamente. **Esperado raro** dado
  que P185A confirmou ausência de Locator no Layouter.
- **Risco de escalada M-L**: registado na ADR-0068. Se
  `.F` revelar que gating no topo é insuficiente
  (múltiplos pontos de despacho, save/restore complexo,
  edge cases), passo escala para M-L. Cláusula gate
  trivial neste caso = aceitar magnitude maior; cláusula
  gate substancial = recuar e re-arquitectar.
- **ADR-0068 transição PROPOSTO → ACEITE**: condicional
  a P185D validação. Se P185D confirmar
  sincronização-por-construção empiricamente, ADR-0068
  pode transitar em P185E (encerramento). Se P185D
  detectar dessincronização, ADR-0068 fica PROPOSTO até
  resolução.
- **Test re-update do P185B (caso 4
  `is_numbering_active_at`) não muda em P185C** — caso
  central da ADR-0068 já passou em isolation. P185C
  introduz a infra; P185D valida sincronização end-to-end;
  P187/P188 ligam aos consumers.
