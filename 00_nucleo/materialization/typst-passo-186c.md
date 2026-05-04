# Passo P186C — `extract_payload` arm `Content::Equation`

Segundo passo de implementação P186 (após P186A diagnóstico,
P186B variants).
Magnitude **S**.

> **Nota de ordem invertida**: este passo era originalmente
> P186D (após `is_locatable` activado em P186C). Após P186B
> revelar que `from_tags.rs` já tem stub no-op para
> `ElementPayload::Equation` (cláusula gate trivial
> documentada em P186B §"Decisões"), a ordem foi invertida:
> **P186C agora adiciona arm em `extract_payload` antes de
> activar `is_locatable` (P186D)**. Isto elimina a janela
> de invariante quebrada que existiria entre os dois passos.
> Análise da inversão em P186C `.A.6`.

Adiciona arm em `extract_payload` para `Content::Equation`
produzir `ElementPayload::Equation { block, counter_update:
CounterUpdate::Step }`. Mantém invariante `is_locatable(c)
↔ extract_payload(c).is_some()` em estado **seguro**:
ambos lados continuam falsos para Equation até P186D
activar `is_locatable`.

Após P186C:
- `extract_payload(Content::Equation)` retorna `Some(...)`.
- `is_locatable(Content::Equation)` ainda `false` (P186D
  activa).
- Walk **não** chama `extract_payload` para Equation
  (porque `is_locatable=false`) — payload é declarado mas
  não emitido.
- Layouter **não** avança Locator para Equation (mesma
  razão).
- `from_tags` stub no-op (P186B) continua intocado.
- Sub-store não recebe entries.
- **Invariante preservada**: `is_locatable=false ∧
  extract_payload≠None` — não viola a invariante porque
  a invariante é `is_locatable ↔ extract_payload.is_some()`,
  que é satisfeita apenas quando ambos lados coincidem.
  Aqui ambos diferem mas em direcção segura: `extract_payload`
  está pronto para quando `is_locatable` activar, sem
  efeito enquanto `is_locatable=false`.

**Pré-condição**: P186B concluído. Tests workspace 1.790
verdes; zero violations. Variants `ElementPayload::Equation`
e `ElementKind::Equation` declarados.

**Restrições**:
- **Não** modificar `is_locatable` — P186D.
- **Não** modificar `from_tags` — P186E.
- **Não** modificar walk arm legacy.
- **Não** migrar consumer C2 — P188.
- API pública preservada.
- Output observable em produção inalterado — `extract_payload`
  arm é declarativo; sem efeito até `is_locatable` activar.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `extract_payload` actual:
   - `01_core/src/rules/introspect/extract_payload.rs:83`
     (per P186A §2 — catch-all `_ => None`).
   - Localizar arms existentes (`Content::Heading`,
     `Content::Figure`, `Content::Bibliography`,
     `Content::SetHeadingNumbering`, etc.).
   - Identificar onde inserir arm para `Content::Equation`
     (provavelmente próximo de outros math elements ou
     em ordem alfabética conforme convenção empírica).

2. Confirmar campos do `Content::Equation` actual:
   - `01_core/src/entities/content.rs:84-87` (per P186A
     §2): `{ body: Box<Content>, block: bool }`.
   - **Sem campo `numbering`** (descoberta P186A §11.1).
   - **Sem campo `label`** — diferente de `Figure`.

3. Confirmar variant `ElementPayload::Equation`:
   - Adicionado em P186B: `{ block: bool, counter_update:
     CounterUpdate }`.

4. Confirmar variant `CounterUpdate::Step`:
   - `01_core/src/entities/counter_update.rs` (ou
     similar).
   - Confirmar que `Step` é variant válido sem campos.

5. Confirmar L0 `extract_payload.md`:
   - Localizar entrada actual sobre arms existentes.
   - Identificar onde adicionar arm para `Content::Equation`.

6. Confirmar análise de ordem invertida:
   - **Antes da inversão**: P186C activava `is_locatable=true`
     (1 LOC mudança); Layouter avançava Locator para
     Equation; walk **não** emitia tag (porque
     `extract_payload` retornava `None`).
   - Resultado: Locations dessincronizadas entre
     Layouter e walk durante intervalo P186C↔D. Quebra
     invariante de sincronização que ADR-0068 garante
     por construção.
   - **Após inversão**: P186C adiciona arm em
     `extract_payload`; `is_locatable` continua `false`,
     logo walk **não** chama `extract_payload` para
     Equation; arm fica latente. P186D activa
     `is_locatable=true` e nesse momento walk começa a
     chamar `extract_payload` para Equation, que já tem
     arm. Sincronização preservada.

7. Confirmar tests existentes em `extract_payload.rs`:
   - `mod tests` — padrão de tests dos arms existentes.
   - Replicar para Equation.

Output: tabela com item + estado + linha actual.

**Critério de saída**:
- `extract_payload` localizado.
- Variants confirmados.
- Análise de ordem invertida confirmada.

### .B Actualizar L0 `extract_payload.md`

1. Adicionar entrada para arm novo:
   - Variant: `Content::Equation { body, block }` →
     `Some(ElementPayload::Equation { block: *block,
     counter_update: CounterUpdate::Step })`.
   - Justificação: replica padrão `Figure` (P184B) com
     campos disponíveis no variant. `body` ignorado
     (não relevante para counter); `block` propagado
     para gate em `from_tags` (P186E).
   - **Nota de ordem invertida**: arm é adicionado
     **antes** de `is_locatable` ser activado em P186D.
     Padrão "infra antes de gate" para preservar
     sincronização-por-construção da ADR-0068.
   - Cross-reference: P184B arm Figure como template.

2. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 contém entrada nova.
- Coerente com convenção dos arms existentes.

### .C Adicionar arm a `extract_payload`

1. Em `01_core/src/rules/introspect/extract_payload.rs`:
   - Adicionar arm `Content::Equation { block, .. } =>
     Some(ElementPayload::Equation { block: *block,
     counter_update: CounterUpdate::Step })`.
   - Posicionar conforme `.A.1`.

2. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.

### .D Tests unit do arm

2-3 tests obrigatórios. Padrão dos arms existentes.

1. **`Content::Equation { block: true }` produz `Some`**:
   - Input: `Content::Equation { body: Box::new(Content::Empty),
     block: true }`.
   - Esperado: `Some(ElementPayload::Equation { block: true,
     counter_update: CounterUpdate::Step })`.

2. **`Content::Equation { block: false }` produz `Some`
   com `block: false`**:
   - Input: bloco inline.
   - Esperado: `Some(ElementPayload::Equation { block: false,
     counter_update: CounterUpdate::Step })`.
   - Confirma propagação do flag para gate downstream
     (P186E).

3. (Opcional) **`Content::Equation` body é ignorado**:
   - Input: variant com body distinto.
   - Esperado: payload idêntico a teste 1 — body não
     afecta extract_payload.

Tests co-localizados em `mod tests` de
`extract_payload.rs`. Padrão dos tests P184B replicado.

**Critério de saída**:
- 2-3 tests passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P186B
   baseline (1.790): +2 a +3.
3. `crystalline-lint .` zero violations.
4. `extract_payload(&Content::Equation { .. })` retorna
   `Some(...)`.
5. `is_locatable(Content::Equation)` ainda `false`
   (P186D activa).
6. **Estado intermédio seguro**: arm em
   `extract_payload` está latente; walk não o invoca
   porque `is_locatable=false`. Sem efeito em produção;
   sem dessincronização Locator.
7. Walk arm legacy intocado.
8. `from_tags` stub no-op (P186B) intocado.
9. Snapshot tests ADR-0033 verdes.
10. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-186c-relatorio.md`
com:

- Resumo: arm `extract_payload` materializado em ordem
  invertida (antes de `is_locatable` activar);
  invariante preservada; payload latente até P186D.
- Confirmação `.E` (10 verificações).
- Δ tests vs baseline P186B (esperado +2 a +3).
- Hashes finais L0 (`extract_payload.md`).
- **Decisão notável**: ordem invertida face à spec
  original (era P186D). Justificação em `.A.6`.
- Estado actual:
  - P186 série: A ✅ B ✅ C ✅ | D-F pendentes.
  - **Invariante preservada** — sem janela quebrada.
  - 50 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P186D (activar `is_locatable(Content::Equation)
  = true` + ajustar test P185D `.C`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria + confirmou análise de ordem
   invertida.
2. L0 `extract_payload.md` actualizado.
3. Arm adicionado.
4. 2-3 tests passam.
5. Tests existentes não regridem.
6. Verificações `.E` passam (10/10).
7. **Invariante de sincronização preservada** —
   `is_locatable=false` continua, logo walk não invoca
   `extract_payload` para Equation; arm latente.
8. Relatório `.F` escrito.

---

## O que pode sair errado

- **Forma exacta de `Content::Equation` muda** (campo
  novo aparece): cláusula gate trivial — adaptar arm.
- **`CounterUpdate::Step` exige parâmetros** (em vez de
  variant unit): cláusula gate trivial — adaptar.
- **Tests existentes regridem por adição de arm específico**
  (em vez de cair em catch-all): cláusula gate trivial —
  ajustar.
- **`extract_payload` não tem catch-all** (match
  exaustivo) e adição de arm exige ordem específica:
  cláusula gate trivial — verificar empiricamente.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~5 LOC arm + ~30 LOC tests +
  edit L0.
- **Sem dependências externas novas**.
- **Pré-condição P186D**: este passo concluído.
- **Padrão replicado**: P182C
  (`SetHeadingNumbering` arm — também replicou
  Figure pattern).
- **Cláusula gate trivial**: aplicável a forma exacta
  do payload, ordem de arms, estrutura de tests.
- **Sem cláusula gate substancial esperada**.
- **Ordem invertida face à spec original**: ver `.A.6`
  para justificação. Padrão "infra antes de gate" —
  payload pronto antes de `is_locatable` activar
  preserva sincronização-por-construção da ADR-0068.
  Replica espírito de P181D que adicionou
  `extract_payload` antes de `is_locatable` para
  Bibliography (verificar empiricamente em P181 série).
- **Sub-store ainda não populado**: walk não emite Tag
  (porque `is_locatable=false`); `from_tags` stub no-op
  intocado. Sub-store permanece vazio até P186D + P186E.
- **Output observable em produção inalterado**: arm é
  declarativo; sem efeito até P186D activar
  `is_locatable`.
