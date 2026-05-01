# Passo P161 — Preparação Introspection refactor: renomear `CounterState`, criar 7 tipos novos (M1 sub-passo 1/3)

Primeiro de três passos para executar M1 do desenho de
arquitectura Introspection com refactor (embebido em §"Desenho
arquitectural" abaixo). Este passo prepara o terreno: renomeia
o tipo agregador actual e cria L0+L1 para os tipos novos.
**Não toca walk em `introspect.rs`**. **Não emite tags ainda**.

P162 e P163 completam M1: P162 cria `extract_payload` e
modifica walk para emitir `Vec<Tag>` em paralelo; P163 verifica
captura via tests E2E.

**Continuidade**: segue directamente P160A (último passo
executado). P160B (instrução de `state(key, init)` por Opção C
field aditivo) **descartado** por decisão arquitectural pós-
debate: refactor antes de features. Trabalho futuro registado
em P160B (refactor de CounterState para tipos isolados) é
agora trabalho actual deste passo e dos seguintes.

**Restrições**:
- Não modificar walk em `introspect.rs`.
- Não criar `Introspector` (M3 — passos futuros).
- Não extrair `is_locatable` como função pública (M2 — passo
  futuro).
- Não tocar features novas (state, metadata, locate, query).
- Output observable não muda; snapshot tests passam inalterados.

---

## Desenho arquitectural (embebido)

Decisões fixadas em discussão prévia que P161-P163 executam:

### Modelo

Pipeline futuro com iteração até fixpoint (paridade vanilla
multi-pass via `comemo`/`convergence`). Loop não vive dentro
de `introspect`; vive no caller. Walk emite `Vec<Tag>` puro;
`Introspector::from_tags` constrói snapshot consultável.

### Tipos a criar em P161

Sete tipos novos em `01_core/src/entities/`:

- **`Location`** — `pub struct Location(u128)`. Identidade
  estável de elemento entre iterações. Forma `u128` em
  paridade vanilla (`lab/typst-original/.../introspection/location.rs`).
- **`Locator`** — gerador determinístico de `Location`s
  durante walk. Vanilla `Locator` mais helpers
  `SplitLocator`/`LocatorLink`; cristalino simplifica.
- **`ElementKind`** — enum dos kinds locatable
  (Heading/Figure/Citation em P161; outros adicionados em
  passos futuros).
- **`CounterUpdate`** — operação sobre counter (Set/Step;
  Func adiado). Verificar primeiro se já existe.
- **`ElementPayload`** — enum por kind, payload type-safe.
  Decisão tomada: payload por kind (não inline fields, não
  variants em Tag). Compilador força exaustividade no fold de
  `Introspector::from_tags`.
- **`ElementInfo`** — wrapper minimalista
  `{ payload: ElementPayload, label: Option<Label> }`.
- **`Tag`** — `enum Tag { Start(Location, ElementInfo),
  End(Location, u128) }`. Minimalista (2 variants como
  vanilla), mas com payload estruturado em vez de Content
  opaco.

### Renomeação `CounterState` → `CounterStateLegacy`

O tipo agregador actual cristalino tem 14-16 fields
acumulados — auditoria classifica como "pior que vanilla".
Será decomposto em M2-M6. Para evitar colisão com vanilla
(onde `CounterState` é tipo diferente: `SmallVec<[u64; 3]>`
em `Counter`), renomeia-se agora para `CounterStateLegacy`.
Eliminado em M6.

### O que P161 NÃO decide

Adiado para passos futuros:
- Forma exacta do hash de `Content` para `body_hash` em
  Heading e `u128` em `Tag::End`. P162 usa hash existente se
  houver; cria se não.
- Memoização (`comemo`) — adiado para M8.
- `Introspector` como trait ou struct — M3.
- Loop fixpoint — M7.
- Features novas (state, metadata, etc.) — M9+.

---

## Sub-passos

### .A Inventário

Inventário **antes** de tocar código. Gate de decisão depende
dos números reportados aqui.

1. Localizar `01_core/src/entities/counter_state.rs`. Ler.
   Registar:
   - Número exacto de fields públicos.
   - Lista literal dos fields (nome + tipo).
   - Número de call-sites de `CounterState` no workspace
     (grep em `01_core/src/`).
   - Profundidade máxima da cadeia de uso (quantos níveis
     entre alocador e consumidor mais profundo).
2. Localizar `01_core/src/entities/content.rs`. Identificar:
   - Campos exactos de `Content::Heading` (nome + tipo).
   - Campos exactos de `Content::Figure` (nome + tipo).
   - Campos exactos de `Content::Citation` (se existir como
     variant directo; senão, registar como ausente).
   - Existência ou não de `Content::Label` ou equivalente
     mecanismo de label.
3. Verificar se `CounterUpdate` existe no cristalino:
   - `grep -rn "enum CounterUpdate" 01_core/src/`.
   - `grep -rn "struct CounterUpdate" 01_core/src/`.
   - Se existe, registar localização e forma actual.
4. Verificar se função de hash determinístico sobre `Content`
   existe:
   - `grep -rn "fn hash_content\|hash_content" 01_core/src/`.
   - Se existe, registar assinatura. Se não, registar como
     pendência para P162 (não criar agora).
5. Localizar `00_nucleo/prompts/entities/`. Confirmar:
   - Existe L0 para `counter_state` (esperado:
     `00_nucleo/prompts/entities/counter_state.md`). Registar
     hash actual.
   - Não existe L0 para os 7 tipos novos (esperado: ausentes).
6. Verificar `00_nucleo/adr/README.md` para identificar ADRs
   aplicáveis a `entities/*`:
   - ADR-0033 (paridade observable) — esperado aplicável.
   - ADR-0036 (atomização progressiva) — verificar
     aplicabilidade a entities novos.
   - ADR-0066 (Introspection runtime adiada) — referenciar
     como contexto.
   - Outros ADRs sobre estrutura L1 — registar.
7. Verificar último relatório executado:
   `00_nucleo/materialization/typst-passo-160a-relatorio.md`
   existe → P160A foi último. Confirmar que
   `typst-passo-160b-relatorio.md` **não** existe.

Output do inventário: documento
`00_nucleo/diagnosticos/inventario-entities-passo-161.md`
com:
- Fields actuais de `CounterStateLegacy` (literal).
- Campos exactos de `Content::Heading/Figure/Citation`.
- Existência ou ausência de `CounterUpdate` e
  `hash_content`.
- Lista de ADRs aplicáveis com nome curto cada.
- Confirmação P160A último relatório, P160B sem relatório.

**Critério de saída e gate de decisão**:
- Se `Content::Citation` não existir como variant directo: **parar**
  e reportar. Reabrir decisão sobre kinds a cobrir em M1
  (talvez restringir a Heading/Figure, deixar Citation para
  passo posterior).
- Se `CounterUpdate` existir com forma incompatível com o
  desenho (ex. variants completamente diferentes): **parar**
  e reportar. Reabrir decisão sobre nome ou estrutura.
- Senão, prosseguir para .B.

### .B Renomear `CounterState` → `CounterStateLegacy`

Pré-condição: .A concluído sem gate disparado.

1. Renomear `struct CounterState` → `struct CounterStateLegacy`
   em `01_core/src/entities/counter_state.rs`.
2. Renomear ficheiro:
   `01_core/src/entities/counter_state.rs` →
   `01_core/src/entities/counter_state_legacy.rs`.
3. Update `01_core/src/entities/mod.rs`:
   `pub mod counter_state;` → `pub mod counter_state_legacy;`
   e re-export `CounterStateLegacy`.
4. Find-and-replace em todos os call-sites (de .A número 1):
   `CounterState` → `CounterStateLegacy`.
5. Renomear L0 correspondente:
   `00_nucleo/prompts/entities/counter_state.md` →
   `00_nucleo/prompts/entities/counter_state_legacy.md`.
   Update conteúdo: nome do tipo + nome do ficheiro alvo.
   Adicionar nota no L0: "tipo agregador transitório, será
   eliminado em M6 do refactor Introspection (ver passos
   P161-P166)".

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — todos os tests passam.
- Nenhuma referência a `CounterState` (não-Legacy) no workspace
  excepto em comentários históricos ou em `lab/typst-original/`.
- Linter passa (sincronização L0↔L1 verificada automaticamente).

### .C Criar L0+L1 de `Location`

1. Escrever L0 em `00_nucleo/prompts/entities/location.md`:
   - Cabeçalho com campo "Hash do Código" em branco (linter
     preenche).
   - Camada L1, ficheiro alvo
     `01_core/src/entities/location.rs`.
   - ADRs: ADR-0033 (paridade), ADR-0066 (Introspection
     runtime adiada — contexto).
   - Origem vanilla:
     `lab/typst-original/crates/typst-library/src/introspection/location.rs`.
   - Restrições estruturais: `Copy + Clone + Eq + Hash`,
     `pub struct Location(u128)`, construtor não público.
   - Critérios de verificação: `Location` é `Copy`; dois
     `Location`s com mesmo `u128` são iguais; construtor
     directo não é acessível fora do módulo.
2. Escrever L1 em `01_core/src/entities/location.rs`:
   - Cabeçalho `@prompt 00_nucleo/prompts/entities/location.md`.
   - `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct Location(u128);`.
   - Construtor `pub(crate) fn new(hash: u128) -> Self` — só
     visível dentro do crate (para `Locator` em .D).
   - Tests co-localizados em `#[cfg(test)]`: dois Locations
     iguais comparam iguais; cópia preserva valor.
3. Update `01_core/src/entities/mod.rs`: re-export `Location`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- L0 e L1 existem com cabeçalhos correctos.
- Linter passa (sincronização L0↔L1 verificada
  automaticamente; hash registado pelo linter).

### .D Criar L0+L1 de `Locator`

1. L0 em `00_nucleo/prompts/entities/locator.md`:
   - Cabeçalho com campo "Hash do Código" em branco (linter
     preenche).
   - Camada L1, ficheiro alvo
     `01_core/src/entities/locator.rs`.
   - ADRs aplicáveis (mesmas que .C).
   - Origem vanilla:
     `lab/typst-original/crates/typst-library/src/introspection/locator.rs`
     (apenas tipo `Locator` principal; `SplitLocator` e
     `LocatorLink` adiados — relevantes para optimização
     `comemo` em M8).
   - Restrições: gerador determinístico (mesma sequência de
     `next()` produz mesmas Locations); sem dependências
     externas além de `Location` e tipos básicos.
   - Critérios de verificação: `Locator::new()` + duas
     chamadas a `next()` retorna Locations diferentes;
     dois Locators independentes com a mesma sequência de
     chamadas produzem Locations iguais (determinismo).
2. L1 em `01_core/src/entities/locator.rs`:
   - Cabeçalho `@prompt`.
   - `pub struct Locator { /* implementação determinística */ }`.
   - Implementação concreta a critério: contador incremental
     hashado, ou hash de path. Documentar a escolha em
     comentário.
   - `pub fn new() -> Self`.
   - `pub fn next(&mut self) -> Location`.
   - Tests co-localizados: determinismo, distinguibilidade,
     não-determinismo entre Locators distintos só se inputs
     diferirem.
3. Update `mod.rs`: re-export.

**Critério de saída**: igual a .C.

### .E Criar L0+L1 de `ElementKind`

1. L0 em `00_nucleo/prompts/entities/element_kind.md`:
   - Cabeçalho com campo "Hash do Código" em branco (linter
     preenche).
   - Camada L1.
   - Sem origem vanilla directa (vanilla usa marker traits
     `Locatable`, não enum). Documentar como divergência
     consciente: cristalino prefere enum exhaustivo.
   - Restrições: `Copy + Clone + Eq + Hash`; subset estrito
     dos kinds que `extract_payload` cobre em P162.
2. L1 em `01_core/src/entities/element_kind.rs`:
   - Cabeçalho.
   - `#[derive(...)] pub enum ElementKind { Heading, Figure,
     Citation }`.
   - Tests co-localizados: igualdade, hash.
3. Update `mod.rs`.

**Critério de saída**: igual a .C.

### .F Criar L0+L1 de `CounterUpdate` (se ausente)

Se .A número 3 confirmou que `CounterUpdate` já existe com
forma compatível, **saltar este sub-passo**. Anotar no
relatório que tipo já existia.

Se ausente:

1. L0 em `00_nucleo/prompts/entities/counter_update.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1.
   - Origem vanilla:
     `lab/typst-original/.../introspection/counter.rs`
     (tipo `CounterUpdate`).
   - Restrições: `Clone`; `Func` variant adiado para passo
     futuro (cristalino actual sem funcs em counter updates).
2. L1 em `01_core/src/entities/counter_update.rs`:
   - Cabeçalho.
   - `pub enum CounterUpdate { Set(usize), Step(usize) }`.
   - Tests co-localizados.
3. Update `mod.rs`.

**Critério de saída**: igual a .C.

### .G Criar L0+L1 de `ElementPayload`

1. L0 em `00_nucleo/prompts/entities/element_payload.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1.
   - Sem origem vanilla directa (vanilla usa Content opaco
     em `Tag::Start`; cristalino prefere payload tipado por
     kind).
   - Restrições: `Clone`; um variant por `ElementKind`;
     compilador força exaustividade no fold em
     `Introspector::from_tags` (M3).
   - Decisão de payload por kind documentada no L0 (não
     usar inline fields opcionais).
2. L1 em `01_core/src/entities/element_payload.rs`:
   - Cabeçalho.
   - Enum com variants `Heading { depth, body_hash,
     counter_update }`, `Figure { figure_kind, counter_update
     }`, `Citation { key }`. Adaptar campos exactos aos de
     `Content::Heading/Figure/Citation` confirmados em .A
     número 2.
   - Se `body_hash` em Heading depende de função de hash
     ausente (.A número 4): usar `body_hash: u128 = 0`
     placeholder e registar como pendência P162.
   - Tests co-localizados: construir cada variant, igualdade.
3. Update `mod.rs`.

**Critério de saída**: igual a .C.

### .H Criar L0+L1 de `ElementInfo`

1. L0 em `00_nucleo/prompts/entities/element_info.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1.
   - Sem origem vanilla directa (vanilla `TagFlags` é forma
     diferente; cristalino prefere wrapper minimalista de
     payload + label).
   - Restrições: `Clone`; struct simples sem lógica.
2. L1 em `01_core/src/entities/element_info.rs`:
   - Cabeçalho.
   - `pub struct ElementInfo { pub payload: ElementPayload,
     pub label: Option<Label> }`.
   - `Label` já existe no cristalino — referenciar.
   - Tests co-localizados.
3. Update `mod.rs`.

**Critério de saída**: igual a .C.

### .I Criar L0+L1 de `Tag`

1. L0 em `00_nucleo/prompts/entities/tag.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1.
   - Origem vanilla:
     `lab/typst-original/.../introspection/tag.rs`.
   - Decisão de divergência consciente: cristalino usa
     `Start(Location, ElementInfo)` em vez de
     `Start(Content, TagFlags)` — payload estruturado em
     vez de Content opaco. Manter 2 variants como vanilla.
   - Restrições: `Clone`; ambas variants têm `Location`
     identificadora.
2. L1 em `01_core/src/entities/tag.rs`:
   - Cabeçalho.
   - `pub enum Tag { Start(Location, ElementInfo),
     End(Location, u128) }`.
   - O `u128` em End é content hash — paridade vanilla.
   - Tests co-localizados: construir cada variant, igualdade.
3. Update `mod.rs`.

**Critério de saída**: igual a .C.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Contagem de tests aumenta vs baseline (smoke V2 dos
   ficheiros novos). Documentar Δ.
3. `crystalline-lint`: zero violations.
4. `grep -rn "struct CounterState\b" 01_core/src/` retorna
   apenas `CounterStateLegacy` (zero `CounterState` puro).
5. Os 7 ficheiros L1 novos existem em `01_core/src/entities/`
   com cabeçalho `@prompt` correcto.
6. Os 7 L0 novos (ou 6 se `CounterUpdate` já existia) existem
   em `00_nucleo/prompts/entities/`.
7. Linter (`crystalline-lint`) confirma sincronização L0↔L1
   automática para todos os tipos novos e para
   `CounterStateLegacy` renomeado. Hashes preenchidos pelo
   linter nos campos "Hash do Código" dos L0.
8. Snapshot tests de paridade ADR-0033 passam inalterados
   (output observable não muda — walk não foi tocado).

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-161-relatorio.md` com:

- Resumo: 7 tipos novos criados (ou 6 se CounterUpdate já
  existia); CounterState renomeado.
- Confirmação de cada verificação .J.
- Hashes finais de cada L0 (preenchidos pelo linter; copiar
  para o relatório como referência).
- Decisões registadas: kinds em ElementKind (Heading, Figure,
  Citation); payload por kind (não inline fields); Tag
  minimalista 2 variants.
- Pendências para P162: hash determinístico de Content (se
  ausente em .A); placeholders (e.g. body_hash = 0) a resolver.
- Pendências para P163: tests E2E de paralelismo só após
  walk emitir tags.
- Estado pós-passo: pronto para P162 (criar `extract_payload`
  e modificar walk para emitir `Vec<Tag>` em paralelo).

---

## Critério de conclusão

Todos em conjunto:

1. .A produziu inventário completo sem disparar gate.
2. `CounterState` renomeado para `CounterStateLegacy`
   (struct, ficheiro L1, ficheiro L0, call-sites).
3. 7 tipos L1 novos criados (ou 6 se CounterUpdate
   pré-existente) com cabeçalhos `@prompt`.
4. 7 L0 novos criados (ou 6) com hashes registados.
5. Walk em `introspect.rs` **não modificado**.
6. Verificações .J 1-8 passam.
7. Relatório .K escrito.

---

## O que pode sair errado

- **`Content::Citation` ausente como variant directo (.A
  número 2)**: gate de decisão dispara. Parar e reportar.
  Pode ser que citations sejam tratadas via outro mecanismo
  (sub-tipo de paragraph, selector). Reabrir escolha de kinds
  para M1.
- **`CounterUpdate` existente com forma incompatível (.A
  número 3)**: gate dispara. Reabrir nome (`CounterDelta`?
  `IntrospectionCounterUpdate`?) ou estender forma existente.
- **L0 `counter_state.md` não existe (.A número 5)**: significa
  que cristalino actual tem L1 sem L0 correspondente (lacuna
  histórica). .B precisa de criar L0 retroactivo antes de
  renomear, ou registar como pendência separada. Decidir
  caso a caso.
- **Função de hash determinístico de Content ausente (.A
  número 4)**: P162 vai precisar criar; P161 usa placeholder
  em ElementPayload. Verificar que placeholder não regride
  testes.
- **Hash determinístico do Locator falha em walk real**: tests
  unitários em .D só verificam `next()` directo. Determinismo
  no walk completo só se verifica em P163. Se aqui já houver
  suspeita (e.g. iteração sobre HashMap dentro de Locator),
  corrigir agora.
- **Renomeação de ficheiro `counter_state.rs` →
  `counter_state_legacy.rs`**: alguns sistemas de build
  cacheiam paths. Se `cargo check` falhar com erro de path,
  fazer `cargo clean` antes de prosseguir.
- **Linter detecta divergência L0↔L1 inicial**: ao escrever
  L0 e L1 de tipo novo, o linter pode falhar na primeira
  verificação se a estrutura do L0 não bate exactamente com
  o L1 (e.g. derive trait declarado no L0 mas omitido no
  L1). Corrigir L0 ou L1 conforme erro reportado, não
  ignorar.

---

## Notas operacionais

- **Tamanho**: M-L. 7 tipos novos + renomeação + 8 L0s
  (incluindo o de `CounterStateLegacy`) é trabalho substancial,
  mas cada sub-passo .C-.I é pequeno e testável isoladamente.
- **Passo de preparação puro**: não toca walk nem features.
  Output observable preservado por construção.
- **Precondição P162**: este passo entrega tipos prontos para
  serem usados em `extract_payload` (P162 sub-passo .3). Sem
  P161, P162 não pode começar.
- **Precondição M2-M6**: o nome `CounterState` libertado por
  esta renomeação fica disponível para uso futuro alinhado
  com vanilla, se necessário.
