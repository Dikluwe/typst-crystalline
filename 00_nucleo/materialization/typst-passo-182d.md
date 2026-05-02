# Passo P182D — Layouter consumers via `Introspector` (substitution-with-fallback)

Terceiro passo de materialização P182 (após P182A diagnóstico,
P182B trait method, P182C extract_payload + auto-init).
Magnitude **S**.

Migra os 2 consumers Layouter de leitura legacy directa para
`Introspector::is_numbering_active(key)` com fallback para
`self.counter.is_numbering_active(legacy_key)`. Padrão
substitution-with-fallback P168/P181G replicado.

Após P182D:
- Layouter heading-arm (`layout/mod.rs:301`) consulta
  Introspector primeiro; fallback legacy se
  `numbering_active:heading` ausente em `StateRegistry`.
- Layouter equation-arm (`layout/equation.rs:24`) consulta
  Introspector primeiro; fallback legacy se
  `numbering_active:equation` ausente.
- Walk arm canonical e write-sites legacy continuam intocados
  (M6 elimina).
- Output observable em produção inalterado — fallback
  preserva paridade enquanto Introspector ainda não tem
  emitter para `numbering_active:equation` (P182C cobriu só
  heading; equation continua inteiramente legacy via
  fallback).

**Pré-condição**: P182C concluído. Tests workspace 1.748
verdes; zero violations. `Introspector::is_numbering_active`
operacional para chave `numbering_active:heading` (populada
via tag `Content::SetHeadingNumbering`); para `numbering_active:equation`
sempre retorna `false` (sem emitter — P182C apenas cobriu
heading variant) — fallback legacy compensa.

**Restrições**:
- **Não** modificar walk arm `introspect.rs:455–457` —
  continua write canonical legacy.
- **Não** modificar write paralelo `layout/counters.rs:11–13`
  — continua.
- **Não** modificar copy-sites `layout/mod.rs:1414, 1442` —
  continuam.
- **Não** modificar trait `Introspector` (P182B fechou).
- **Não** modificar `extract_payload`, `is_locatable`,
  `from_tags` (P182C fechou).
- **Não** adicionar emitter para `numbering_active:equation`
  — fora de escopo P182.
- API pública preservada.
- Output observable em produção inalterado — fallback
  garante paridade.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar Layouter heading-arm consumer:
   - `01_core/src/rules/layout/mod.rs:301` (per P182A/C).
   - Localizar a leitura actual: padrão esperado
     `self.counter.is_numbering_active("heading")`.
   - Identificar contexto exacto (arm `Content::Heading`,
     função/método específico, escopo das variáveis
     locais).
   - Confirmar acesso a `self.introspector` no escopo
     (campo, ref local, Tracked).

2. Confirmar Layouter equation-arm consumer:
   - `01_core/src/rules/layout/equation.rs:24` (per P182A/C).
   - Localizar leitura actual: padrão esperado
     `self.counter.is_numbering_active("equation")`.
   - Identificar contexto + acesso a `self.introspector`.

3. Confirmar L0s relevantes:
   - `00_nucleo/prompts/rules/layout/mod.md` ou
     `layout.md` (verificar nome real).
   - `00_nucleo/prompts/rules/layout/equation.md` (se
     existir).
   - Identificar entradas que documentam comportamento
     destes arms.

4. Confirmar disponibilidade de `Introspector` no Layouter:
   - P181G migrou cite-arm via `self.introspector.bib_*`
     — confirmar que mesmo field/ref existe.
   - P168 figure-ref migrou similar — confirmar padrão.
   - Se field não acessível (Layouter ainda não tem
     `introspector` no construtor): cláusula gate
     **substancial** — recuar e investigar antes de
     continuar.

5. Confirmar convenção de chave para fallback legacy:
   - `CounterStateLegacy::is_numbering_active(key: &str)`
     usa key sem prefixo (`"heading"`, `"equation"`).
   - `Introspector::is_numbering_active(key: &str)` usa
     key com prefixo (`"numbering_active:heading"`).
   - Fallback respeita as duas convenções.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `self.introspector` não acessível no Layouter:
  cláusula gate substancial — investigar P181G/P168 antes
  de prosseguir (improvável; ambos estabeleceram acesso).
- Se contexto da leitura legacy revela mais que apenas
  `is_numbering_active(...)` (e.g. ler também
  `numbering_pattern` ou similar): cláusula gate trivial —
  migração cobre apenas `is_numbering_active`; resto
  continua legacy.
- Senão prosseguir.

### .B Actualizar L0s relevantes

1. Em L0 do Layouter heading (ou ficheiro equivalente):
   - Documentar que arm consulta Introspector com
     fallback legacy.
   - Justificação: padrão P168 figure-ref / P181G
     cite-arm.

2. Em L0 do Layouter equation:
   - Mesma documentação para equation-arm.
   - Nota explícita: `numbering_active:equation` em
     Introspector retorna sempre `false` em P182 (sem
     emitter); fallback é o caminho real até passo
     dedicado equation-set-rule (fora P182).

3. Hashes em branco aguardam recálculo.

**Critério de saída**:
- L0s contêm entradas novas/actualizadas.
- Convenção de chave documentada (`numbering_active:<feature>`
  prefixo apenas para Introspector).

### .C Migrar Layouter heading-arm

1. Em `01_core/src/rules/layout/mod.rs:301`:
   - Substituir leitura `self.counter.is_numbering_active("heading")`
     por:
     ```
     self.introspector.is_numbering_active("numbering_active:heading")
         || self.counter.is_numbering_active("heading")
     ```
   - Forma exacta fica para Claude Code conforme convenção
     do projeto (variável intermédia `numbering_on` vs
     inline; nome de ref `self.introspector` vs
     `introspector` se já bound).

2. Confirmar cabeçalho de linhagem `@prompt-hash`
   actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Output observable inalterado em tests existentes
  (Introspector retorna real ou false; fallback compensa
  caso false).
- Linter passa.

### .D Migrar Layouter equation-arm

1. Em `01_core/src/rules/layout/equation.rs:24`:
   - Substituir leitura `self.counter.is_numbering_active("equation")`
     por padrão simétrico com chave
     `"numbering_active:equation"`.

2. Confirmar cabeçalho de linhagem actualiza.

**Critério de saída**:
- `cargo check --workspace` passa.
- Output observable inalterado.
- Linter passa.

### .E Tests unitários ou ajustes

1. Verificar tests existentes que cobrem heading numbering:
   - `grep -rn "numbering_active\|is_numbering_active" 01_core/src/rules/layout/`.
   - Se tests existentes usam apenas `state.numbering_active`
     directo: continuam a passar (fallback cobre).
   - Se tests usam pipeline completo (walk + Introspector +
     Layouter): podem ganhar paridade automática (Introspector
     populado via P182C).

2. Adicionar 1-2 tests novos cobrindo migração:
   - Test 1: documento com `Content::SetHeadingNumbering
     { active: true }`. Após walk + `from_tags` + Layouter
     heading-arm, prefixo de heading é gerado **via
     Introspector** (não fallback). Confirmar via assertion
     no output do Layouter ou via instrumentação.
   - Test 2: documento sem `SetHeadingNumbering`.
     Layouter heading-arm cai em fallback legacy. Output
     idêntico.
   - (Opcional) Test 3: Layouter equation-arm sempre cai
     em fallback (P182 não emite `numbering_active:equation`).

3. Tests E2E pipeline completo ficam para P182E (passo
   dedicado).

**Critério de saída**:
- 1-3 tests novos passam.
- Tests existentes não regridem.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P182C
   baseline (1.748): +1 a +3 dependendo de cobertura `.E`.
3. `crystalline-lint .` zero violations.
4. Layouter heading-arm consulta `self.introspector.is_numbering_active`
   primeiro, fallback legacy.
5. Layouter equation-arm consulta `self.introspector.is_numbering_active`
   primeiro, fallback legacy.
6. Walk arm `introspect.rs:455–457` **NÃO modificado**.
7. Write paralelo `layout/counters.rs:11–13` **NÃO
   modificado**.
8. Copy-sites `layout/mod.rs:1414, 1442` **NÃO
   modificados**.
9. Snapshot tests ADR-0033 verdes (output observable
   inalterado — fallback preserva paridade).
10. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-182d-relatorio.md`
com:

- Resumo: 2 consumers Layouter migrados; substitution-with-fallback
  padrão; output observable inalterado.
- Confirmação `.F` (10 verificações).
- Δ tests vs baseline P182C (esperado +1 a +3).
- Hashes finais de L0s modificados.
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P182 série: A ✅ B ✅ C ✅ D ✅ | E-F pendentes.
  - M9: 10/11 features (inalterado).
  - **M5 progresso**: 3/6 consumers migrados (figure-ref
    P168 + cite-arm P181G + heading-arm/equation-arm
    P182D). Nota: contagem assume "consumer" = call-site
    distinto; auditar se P182D conta como 1 ou 2.
  - 34 passos executados.
- Pendências cumulativas: inalteradas (legacy continua).
- Próximo passo: P182E (tests E2E pipeline completo
  confirmando paridade Introspector vs legacy).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0s actualizados (heading + equation Layouter docs).
3. Layouter heading-arm migrado.
4. Layouter equation-arm migrado.
5. Tests novos passam (1-3 esperado).
6. Tests existentes não regridem (paridade via fallback).
7. Verificações `.F` passam (10/10).
8. Relatório `.G` escrito.
9. Output observable em produção inalterado.

---

## O que pode sair errado

- **`self.introspector` não acessível em Layouter**:
  cláusula gate substancial. P181G estabeleceu acesso;
  improvável regressão. Se sim, recuar.
- **`is_numbering_active` em legacy tem assinatura
  diferente** (`(&self, &str) -> bool` vs `(&self, key:
  String) -> bool` ou similar): cláusula gate trivial —
  adaptar.
- **Layouter heading-arm tem dependências adicionais não
  inventariadas** (lê outros fields além de
  `numbering_active`): cláusula gate trivial — migrar
  apenas `is_numbering_active`; resto legacy.
- **Tests existentes regridem**: não esperado se fallback
  está correcto. Se regridir, fallback tem bug —
  investigar antes de prosseguir.
- **Snapshot tests divergem**: indica que Introspector
  retorna `true` quando legacy retornava `false` (ou
  vice-versa). Investigar — divergência genuína bloqueia
  P182D até resolução. Provável causa: `from_tags::StateUpdate`
  auto-init (P182C 5.1) entrou em conflito com legacy
  `or_insert` semantics. Cláusula gate substancial.
- **Linter divergência V13/V14 por edits**: cláusula gate
  trivial — `--fix-hashes`.
- **Forma exacta da chamada `||` vs `match` vs variável
  intermédia**: detalhe de implementação; não afecta
  semântica. Claude Code escolhe convenção do projeto.

---

## Notas operacionais

- **Tamanho**: S. ~30-60 LOC (2 edits inline + 1-3 tests
  + edits L0).
- **Sem dependências externas novas**.
- **Pré-condição P182E**: este passo concluído.
- **Padrão replicado**: P168 figure-ref + P181G cite-arm
  (substitution-with-fallback).
- **Cláusula gate trivial**: aplicável a forma exacta da
  expressão, nome de variável, assinatura legacy.
- **Cláusula gate substancial**: aplicável apenas se
  Introspector inacessível ou se snapshot tests divergirem
  (regressão real).
- **`numbering_active:equation` continua sem emitter** —
  fallback é o caminho real para equation. Documentado
  explicitamente em L0 `.B.2` para evitar interpretação
  errada futura.
- **Equation set rule** (variant `Content::SetEquationNumbering`
  ou similar) ficaria fora de P182 — se algum dia for
  introduzido, replicaria P182C para `numbering_active:equation`.
  Sem decisão activa em P182.
