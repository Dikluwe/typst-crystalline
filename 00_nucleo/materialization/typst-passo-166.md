# Passo P166 — Expor `Introspector` ao caller (M4)

Passo único de M4 do refactor Introspection. Materializa o
mecanismo pelo qual consumers podem aceder ao `TagIntrospector`
construído em paralelo desde P165.

**Decisão pendente registada como sub-passo `.A`**: 3 opções
foram avaliadas em planeamento (M4a quebrar API, M4b entry
point novo, M4c campo no legacy). Decisão adiada para
inventário factual em `.A` — escolha depende de números reais
de call-sites e construtores.

**Paraleliza com `CounterStateLegacy`**: M4 não substitui o
tipo legacy. Walk continua a popular ambos. M5 começa a
migrar consumers; M6 elimina `CounterStateLegacy`.

**Pré-condição**: M3 concluído (P165). `TagIntrospector`
construído em paralelo em `pub fn introspect()` mas
descartado.

**Restrições**:
- Não migrar consumers (M5).
- Não eliminar `CounterStateLegacy` (M6).
- Não tocar `from_tags`, sub-stores, ou walk emission lógica
  (estabelecida em P162-P165).
- Output observable não muda; snapshot tests passam inalterados.
- Decisão M4a/M4b/M4c tomada em `.A` com base em números reais
  do inventário; se decisão for ambígua, gate substancial.

---

## Sub-passos

### .A Inventário e escolha M4a/M4b/M4c

Inventário factual antes de decidir. **A decisão depende
literalmente dos números reportados aqui.**

1. **Call-sites de `introspect()`** (afecta custo de M4a):
   - `grep -rn "introspect(" 01_core/src/` filtrado para
     chamadas reais (não definição).
   - Listar literalmente: ficheiro:linha + contexto curto.
   - Contar.

2. **Construtores de `CounterStateLegacy`** (afecta M4c):
   - `grep -rn "CounterStateLegacy::\|CounterStateLegacy {" 01_core/src/`.
   - Identificar quantos sítios constroem o tipo (vs apenas
     receberem-no).
   - Listar.

3. **Consumers do output de `introspect()`** (afecta M5
   downstream — informa qual mecanismo facilita migração):
   - Para cada call-site identificado em (1), inspeccionar
     o que faz com o retorno.
   - Categorizar:
     - **Read-only**: lê fields de `CounterStateLegacy`,
       não muta.
     - **Mut**: muta o retorno (provavelmente raro; se
       existir, M4c pode complicar).
     - **Composição**: passa o retorno a outras funções.

4. **API pública vs interna**:
   - `introspect()` é `pub` — call-sites externos ao crate
     podem existir. `grep -rn "use ...introspect\|::introspect("
     workspace inteiro`.
   - Se há call-sites externos, M4a (quebrar API) tem
     custo maior.

5. **Padrão cristalino para "output múltiplo"**:
   - Procurar exemplos de função pública que retorna
     tuple. Se há padrão, registar.
   - Procurar exemplos de struct com getters. Se há padrão,
     registar.
   - Cláusula gate trivial aplicável: usar padrão mais
     comum.

6. **Estado de pendências M3**:
   - Confirmar que `TagIntrospector` é construído mas
     descartado em `introspect()` (P165).
   - `_introspector` underscore-prefixed deve estar visível
     no ficheiro `rules/introspect.rs`.

Output: notas internas + matriz de decisão preenchida com
números reais.

**Regras de decisão** (aplicadas após inventário):

| Critério                                    | Favorece M4a   | Favorece M4b      | Favorece M4c              |
|---------------------------------------------|----------------|-------------------|---------------------------|
| Call-sites de `introspect()` ≤ 3            | ✓ baixo custo  |                   |                           |
| Call-sites > 3 e ≤ 10                       |                | ✓ migração gradual|                           |
| Call-sites > 10                             |                | ✓ menos invasivo  |                           |
| Algum consumer já quer Introspector         |                | ✓                 |                           |
| Construtores múltiplos de `CounterStateLegacy` |             |                   | ✗ complica construtores  |
| Construtores apenas em 1 sítio              |                |                   | ✓                        |
| Mut consumer                                | ✗ tuple awkward|                   |                           |
| API externa public                          | ✗              | ✓                 |                           |

**Critério de saída e gate de decisão**:
- Se números são claros e apontam para uma das 3 opções:
  decisão registada, prosseguir.
- Se números são ambíguos (e.g. 5 call-sites, M4a vs M4b
  pode ir para qualquer lado): **parar** e reportar
  inventário, reabrir decisão.
- Se inventário revela algo inesperado (ex. `introspect()`
  é chamado de forma incompatível com qualquer das 3 opções):
  **parar**, reabrir.

### .B Implementar mecanismo escolhido

A implementação concreta depende da escolha em `.A`. Cada
opção tem sub-passos próprios.

#### .B.M4a Quebrar API: tuple retornado

Apenas se `.A` escolheu M4a.

1. Modificar `pub fn introspect()` em
   `01_core/src/rules/introspect.rs`:

   ```rust
   pub fn introspect(content: &Content) -> (CounterStateLegacy, TagIntrospector) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags, None);
       let introspector = from_tags(&tags);
       (state, introspector)
   }
   ```

   Remover `_introspector` underscore + `drop(tags)`.

2. Adaptar todos os call-sites identificados em `.A`:
   - Patch literal: `let state = introspect(&c)` →
     `let (state, _introspector) = introspect(&c)`.
   - Para call-sites que passem retorno a outras funções,
     desempacotar.

3. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Reflectir nova assinatura `-> (CounterStateLegacy, TagIntrospector)`.
   - Documentar que `_introspector` em call-sites externos
     é deliberado em M4 (consumers migram em M5).

#### .B.M4b Entry point novo

Apenas se `.A` escolheu M4b.

1. Adicionar função pública nova em `01_core/src/rules/introspect.rs`:

   ```rust
   pub fn introspect_with_introspector(
       content: &Content,
   ) -> (CounterStateLegacy, TagIntrospector) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags, None);
       let introspector = from_tags(&tags);
       (state, introspector)
   }
   ```

2. Manter `pub fn introspect()` actual exactamente como está
   (continua a descartar `_introspector`).

3. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Documentar duas APIs: `introspect()` (legacy, retorna
     só `CounterStateLegacy`) e `introspect_with_introspector()`
     (nova, retorna ambos).
   - Documentar que M5 migra consumers do antigo para o
     novo gradualmente; M6 elimina o antigo.

4. **Optimização**: se `.A` mostrar que `introspect()`
   ainda é chamado por consumers que NÃO querem
   Introspector, manter as duas funções com walks
   independentes (o walk não é grátis). Senão, considerar
   `introspect()` virar wrapper que descarta:

   ```rust
   pub fn introspect(content: &Content) -> CounterStateLegacy {
       let (state, _introspector) = introspect_with_introspector(content);
       state
   }
   ```

   Decisão local em `.B.M4b` baseada em números de `.A`.

#### .B.M4c Campo no legacy

Apenas se `.A` escolheu M4c.

1. Modificar `01_core/src/entities/counter_state_legacy.rs`:
   - Adicionar field `introspector: Option<TagIntrospector>`.
   - Adicionar getter `pub fn introspector(&self) -> Option<&TagIntrospector>`.
   - Inicializar field como `None` em construtores existentes
     de `CounterStateLegacy::new()` etc.

2. Modificar `pub fn introspect()` em
   `rules/introspect.rs`:
   - Após construir `_introspector` via `from_tags(&tags)`:
     `state.introspector = Some(introspector);`.
   - Retornar state como antes.

3. Update L0 `00_nucleo/prompts/entities/counter_state_legacy.md`:
   - Documentar field novo.
   - Documentar que field é populado em
     `rules/introspect::introspect()` desde M4.

4. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Documentar que `introspect()` agora popula
     `state.introspector`.

**Critério de saída** (qualquer opção):
- `cargo check` passa.
- `cargo test` — todos os tests existentes passam.
- API pública preservada conforme decisão (M4a quebra,
  M4b adiciona, M4c estende sem quebrar).
- Linter passa (sincronização L0↔L1 verificada).

### .C Tests da exposição

Tests verificam que `Introspector` é acessível ao caller
conforme mecanismo escolhido em `.A`/`.B`.

1. Test "Introspector acessível":
   - **M4a**: `let (_, introspector) = introspect(&content);`,
     verificar que `introspector.query_first(Heading)`
     retorna `Some(_)` para Content com heading.
   - **M4b**: `let (_, introspector) = introspect_with_introspector(&content);`,
     verificação idêntica. Adicional: `let state = introspect(&content);`
     retorna apenas `CounterStateLegacy` (assinatura antiga
     preservada).
   - **M4c**: `let state = introspect(&content);
     let introspector = state.introspector().unwrap();`,
     verificar query.

2. Test "Introspector consistente com state":
   - Walk produz `state` E `introspector` que são
     consistentes (já verificado em P165 .G — refazer aqui
     com novo mecanismo de acesso para validar exposição
     correcta).

3. Test "Backward compatibility" (apenas M4b/M4c):
   - Call-sites antigos com assinatura antiga continuam
     a funcionar.

**Critério de saída**:
- 2-3 tests novos passam.
- Tests antigos continuam a passar.
- Linter passa.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs baseline P165 (1590).
3. `crystalline-lint`: zero violations.
4. Mecanismo escolhido em `.A` implementado em `.B`:
   - **M4a**: assinatura de `introspect()` é
     `-> (CounterStateLegacy, TagIntrospector)`; call-sites
     adaptados.
   - **M4b**: existe nova função pública
     `introspect_with_introspector`; antiga preservada.
   - **M4c**: `CounterStateLegacy` tem field
     `introspector: Option<TagIntrospector>` + getter.
5. L0 `rules/introspect.md` reflecte novo estado.
6. (Apenas M4c): L0 `entities/counter_state_legacy.md`
   reflecte field novo.
7. Walk não modificado (lógica de emissão preservada).
8. Snapshot tests de paridade ADR-0033 passam inalterados.
9. Linter passa em verificação final.

### .E Encerramento

Escrever
`00_nucleo/materialization/typst-passo-166-relatorio.md` com:

- Resumo: mecanismo escolhido em `.A` (M4a/M4b/M4c) com
  justificação numérica; Introspector exposto.
- Confirmação de cada verificação .D.
- **Inventário do `.A` em formato literal**: número de
  call-sites, construtores, padrão cristalino, etc. Reportar
  números para auditoria futura.
- Hashes finais de L0 modificados.
- Δ tests vs baseline P165.
- Pendências para M5: migração de consumers para
  `Introspector` (primeiro consumer real virá aqui).
- Pendências cumulativas (lista crescente desde M1).
- Estado pós-passo: M4 concluído. M5 desbloqueado —
  começar migração de consumers do `CounterStateLegacy`
  para `Introspector`.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário completo com números reais; uma
   das 3 opções escolhida com justificação documentada (ou
   gate disparou e foi reaberto).
2. `.B` implementou mecanismo escolhido.
3. `.C` tests verificam exposição correcta.
4. Verificações `.D` 1-9 passam.
5. Relatório `.E` escrito.
6. Output observable não muda.
7. M4 concluído.

---

## O que pode sair errado

- **Inventário ambíguo (.A gate)**: números no meio das
  regras de decisão (e.g. 5 call-sites, alguns externos
  outros não). Parar e reabrir. Conversa com utilizador
  decide entre M4a/M4b/M4c.
- **Call-sites externos ao crate identificados**: M4a fica
  com custo alto (downstream tem que adaptar). Provavelmente
  inclina para M4b. Documentar no inventário.
- **`introspect()` chamado de forma exótica**: ex. via
  ponteiro de função, fn item, ou closure. Pode invalidar
  algumas opções. Reportar e adaptar.
- **Mecanismo escolhido não bate com testes existentes**:
  alguns testes podem assumir assinatura antiga. Tests
  vão ter que adaptar — registar Δ no relatório.
- **(Apenas M4c) `CounterStateLegacy` tem múltiplos
  construtores**: cada um precisa adicionar `introspector:
  None` no init. Pode crescer. Considerar mudar para M4b
  se descobrir tarde.
- **(Apenas M4b) Walks duplicados**: se `introspect()` e
  `introspect_with_introspector()` ambos fazem walk completo,
  consumers que chamarem ambos pagam dupla. Documentar como
  pendência para M5 (consumer migrado para novo entry, antigo
  desuso).
- **L0 modificações criam divergência linter**: especialmente
  M4c (modifica L0 do legacy + introspect). Ajustar conforme
  erro reportado.

---

## Notas operacionais

- **Tamanho**: S-M, depende da escolha:
  - M4a com poucos call-sites: S.
  - M4a com muitos: M.
  - M4b: S (maioritariamente código novo, sem migração).
  - M4c: M (modifica `CounterStateLegacy` + construtores).
- **Pré-condição M5**: mecanismo de exposição existe; M5
  migra primeiro consumer real (provavelmente layout ou
  `materialize_time` de `CounterStateLegacy.resolved_labels`
  → `introspector.query_by_label`).
- **Cláusula gate trivial** (formalizada em P163):
  aplicável. Decisões locais sobre padrão de "output
  múltiplo" no cristalino podem ser resolvidas sem parar.
- **Decisão M4a/M4b/M4c é grande**: não é gate trivial.
  Inventário em `.A` deve ser exaustivo. Se números forem
  ambíguos, parar e reabrir é mais seguro que decidir mal.
- **`CounterStateLegacy` continua a viver**: M4 não toca
  na sua existência. Apenas adiciona acesso paralelo a
  Introspector. M6 elimina o legacy quando todos os
  consumers migrarem (M5).
