# Passo P162 — `hash_content`, `extract_payload`, e walk em paralelo (M1 sub-passo 2/3)

Segundo de três passos para executar M1 do refactor Introspection
(P161 + P162 + P163). Este passo:
1. Materializa função `hash_content` (pendência herdada de P161).
2. Resolve placeholders `body_hash = 0` em `ElementPayload`
   (P161 .G).
3. Cria `extract_payload` em `rules/introspect/`.
4. Modifica walk em `rules/introspect.rs` para emitir
   `Vec<Tag>` em paralelo a `CounterStateLegacy`. Tags
   descartadas no fim — não consumidas em M1.

P163 verifica via tests E2E que tags capturam informação
consistente.

**Pré-condição**: P161 concluído. 7 tipos novos disponíveis em
`entities/`; `CounterState` renomeado para `CounterStateLegacy`.

**Restrições**:
- Não criar `Introspector` (M3 — passo futuro).
- Não consumir o `Vec<Tag>` produzido — apenas emitir e
  descartar.
- Não extrair `is_locatable` como função pública (M2).
- Não tocar features novas (state, metadata, locate, query).
- API pública de `introspect()` preservada.
- Output observable não muda; snapshot tests passam inalterados.

---

## Sub-passos

### .A Inventário

Reverificar (não confiar em P161 — verificar agora):

1. Tipos criados em P161 estão em `entities/` e re-exportados:
   - `Location`, `Locator`, `ElementKind`, `CounterUpdate`
     (ou alias actual), `ElementPayload`, `ElementInfo`, `Tag`.
   - `CounterStateLegacy` substitui `CounterState`.
2. Hash determinístico sobre `Content` no cristalino:
   - `grep -rn "fn hash_content\|hash_content\b" 01_core/src/`.
   - Se existe, registar localização e assinatura. Se ausente,
     confirmar que .B vai criar.
3. Campos exactos de `Content::Heading`, `Content::Figure`,
   `Content::Citation` (mesma verificação que P161 .A
   número 2 — repetir, não confiar). Registar:
   - Nome + tipo de cada campo relevante para
     `extract_payload`.
   - Se algum campo necessário foi adicionado/removido entre
     P161 e P162.
4. Mecanismo de label no cristalino:
   - Como é que `Content::Heading` (ou outros) tem label?
     Field directo? Wrapper? Selector?
   - `ElementInfo.label: Option<Label>` precisa saber
     extrair label do Content em `extract_payload`.
5. Walk em `rules/introspect.rs`:
   - Assinatura actual de `walk` (parâmetros, return type).
   - Função `introspect()` pública (assinatura).
   - Outras funções públicas em `introspect.rs` que chamam
     walk (precisam adaptar-se à nova assinatura).
6. L0 de `introspect.rs`:
   - Localizar `00_nucleo/prompts/rules/introspect.md` (ou
     equivalente). Confirmar formato L0 igual ao de
     `entities/`.
   - Ler para saber estrutura actual antes de modificar.
7. L0 placeholders de P161:
   - `00_nucleo/prompts/entities/element_payload.md` regista
     `body_hash` como placeholder. Confirmar texto exacto
     da pendência (será actualizado em .C).
   - `00_nucleo/prompts/entities/tag.md` regista `End(Location, u128)`
     onde `u128` é content hash placeholder. Confirmar.

Output: notas internas. Não criar diagnóstico separado a não
ser que campos divirjam significativamente do esperado em P161.

**Critério de saída e gate de decisão**:
- Se `Content::Heading.body` (ou equivalente) não existir
  como referência walkável: **parar**. `body_hash` em
  `ElementPayload::Heading` precisa de algo concreto para
  hashar. Reabrir decisão.
- Se mecanismo de label não for `Option<Label>` directo
  (ex. label vem de outro Content irmão): **parar**. Reabrir
  decisão sobre como `ElementInfo.label` é populado.
- Se L0 de `introspect.rs` não existir: criar L0 retroactivo
  ou registar como pendência separada antes de prosseguir.
- Senão, prosseguir para .B.

### .B Criar L0+L1 de `hash_content`

Função pura `fn hash_content(content: &Content) -> u128`.
Determinística: mesma `Content` produz sempre o mesmo `u128`.

1. L0 em `00_nucleo/prompts/entities/content_hash.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/content_hash.rs`.
   - ADRs: ADR-0033 (paridade), ADR-0066 (Introspection
     contexto).
   - Origem vanilla: vanilla usa `Hash` derive em `Content`
     via proc-macro; cristalino não tem proc-macro, implementa
     manualmente. Documentar como divergência.
   - Restrições estruturais:
     - Função pura, sem efeitos secundários.
     - Determinismo: dois `Content` `Eq` produzem mesmo hash.
     - Sem dependência em ordem de iteração de `HashMap`
       (usar `BTreeMap` se necessário, ou ordenar antes de
       hashar).
     - Sem uso de `f64`/`f32` directos no hash (usar bit
       representation se Content contém floats).
   - Critérios de verificação:
     - Dois Contents iguais produzem mesmo hash.
     - Contents diferentes produzem hashes diferentes (sanity
       check).
     - Determinismo entre runs (chamar 100 vezes, todas
       iguais).
2. L1 em `01_core/src/entities/content_hash.rs`:
   - Cabeçalho `@prompt 00_nucleo/prompts/entities/content_hash.md`.
   - `pub fn hash_content(content: &Content) -> u128`.
   - Implementação concreta: `siphasher::sip128::SipHasher`
     ou equivalente. Walk recursivo sobre estrutura de
     `Content`, hash de cada variant + campos.
   - Tests co-localizados:
     - Igualdade: dois mesmos Contents → mesmo hash.
     - Distinguibilidade: Contents diferentes → hashes
       diferentes (em pelo menos 5 cases construídos
       manualmente).
     - Determinismo: chamar 100 vezes sobre mesmo Content,
       todas iguais.
3. Update `01_core/src/entities/mod.rs`: re-export
   `hash_content`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- L0 e L1 existem com cabeçalhos correctos.
- Linter passa (sincronização L0↔L1 verificada).

### .C Resolver placeholders P161

Update L0s sem mudar L1 (placeholders eram só notas no L0;
construção dos variants em L1 muda em .D ao chamar
`hash_content`).

1. Update L0 `00_nucleo/prompts/entities/element_payload.md`:
   - Remover nota de pendência sobre `body_hash` placeholder.
   - Adicionar nota: "`body_hash` em `Heading` é populado
     pela função `hash_content` em `extract_payload`
     (P162.D)".
2. Update L0 `00_nucleo/prompts/entities/tag.md`:
   - Remover nota de pendência sobre `u128` em `Tag::End`
     placeholder.
   - Adicionar nota: "`u128` em `End` é content hash do nó,
     populado em `walk` via `hash_content` (P162.E)".

**Critério de saída**:
- L0 actualizados sem referências a placeholder.
- Linter passa (hashes recalculados).
- `cargo check` continua a passar (L1 não modificado neste
  sub-passo — alterações reais em .D e .E).

### .D Criar L0+L1 de `extract_payload`

1. L0 em `00_nucleo/prompts/rules/introspect/extract_payload.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/rules/introspect/extract_payload.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla: nenhuma directa. Vanilla resolve via
     vtable de `Locatable` trait. Cristalino prefere função
     pura com match exaustivo sobre `Content`.
   - Restrições:
     - Função pura, sem efeitos secundários.
     - `fn extract_payload(content: &Content) -> Option<ElementPayload>`.
     - `Some(...)` para variants locatable; `None` para
       outros.
     - Match exaustivo sobre `Content` (compilador força
       cobertura — se variant novo for adicionado a
       `Content`, compilação falha aqui até decisão sobre
       locatability).
   - Critérios de verificação:
     - Para cada um dos 3 kinds (Heading, Figure, Citation):
       construir Content mínimo; chamar `extract_payload`;
       verificar `Some(payload)` com payload correcto.
     - Para Content não-locatable (ex. `Content::Text`,
       `Content::Math`): verificar `None`.
2. L1 em `01_core/src/rules/introspect/extract_payload.rs`:
   - Cabeçalho `@prompt`.
   - Implementação:

   ```rust
   use crate::entities::{
       Content, ElementPayload, CounterUpdate, hash_content,
   };

   pub fn extract_payload(content: &Content) -> Option<ElementPayload> {
       match content {
           Content::Heading { depth, body, .. } => Some(
               ElementPayload::Heading {
                   depth: *depth,
                   body_hash: hash_content(body),
                   counter_update: CounterUpdate::Step(*depth),
               }
           ),
           Content::Figure { kind, .. } => Some(
               ElementPayload::Figure {
                   figure_kind: kind.clone(),
                   counter_update: CounterUpdate::Step(1),
               }
           ),
           Content::Citation { key, .. } => Some(
               ElementPayload::Citation {
                   key: key.clone(),
               }
           ),
           _ => None,
       }
   }
   ```

   Adaptar campos exactos aos confirmados em .A número 3. Se
   `Content::Heading.body` for `Vec<Content>` em vez de
   `Box<Content>`, hashar a sequência (`body.iter().map(|c| hash_content(c)).fold(...)`).

   - Tests co-localizados em `#[cfg(test)]`:
     - Heading básico → `Some(ElementPayload::Heading {...})`.
     - Figure básico → `Some(ElementPayload::Figure {...})`.
     - Citation básica → `Some(ElementPayload::Citation {...})`.
     - Text → `None`.
     - Math (ou outro variant não-locatable) → `None`.
3. Update `01_core/src/rules/introspect/mod.rs` (criar se não
   existir): re-export `extract_payload`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- L0 e L1 existem com cabeçalhos correctos.
- Linter passa.

### .E Modificar walk para emitir tags em paralelo

1. Em `01_core/src/rules/introspect.rs`, alterar assinatura
   de `walk`:

   Antes (assumindo forma típica):
   ```rust
   fn walk(content: &Content, state: &mut CounterStateLegacy) { ... }
   ```

   Depois:
   ```rust
   fn walk(
       content: &Content,
       state: &mut CounterStateLegacy,
       locator: &mut Locator,
       tags: &mut Vec<Tag>,
   ) { ... }
   ```

2. Adicionar lógica de emissão **antes** da mutação actual de
   `state`:
   - Chamar `extract_payload(content)`.
   - Se retorna `Some(payload)`:
     - `let location = locator.next();`
     - `let label = extract_label(content);` (helper local
       ou inline conforme mecanismo confirmado em .A número
       4).
     - `let info = ElementInfo { payload, label };`
     - `tags.push(Tag::Start(location, info));`
   - Lógica original de mutação `state` continua exactamente
     como antes.
   - Walks recursivamente filhos passando `state`, `locator`,
     `tags` por mutação.
   - Se emitiu `Tag::Start`:
     - `let hash = hash_content(content);`
     - `tags.push(Tag::End(location, hash));`

3. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Reflectir nova assinatura de `walk`.
   - Documentar emissão de tags em paralelo como
     comportamento adicional.
   - Documentar que tags são descartadas em M1; M2/M3
     começarão a usá-las.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — todos os tests existentes passam (output
  observable não muda).
- Walk aceita 4 parâmetros conforme nova assinatura.
- L0 de `introspect.rs` reflecte nova assinatura.
- Linter passa.

### .F Pontos de entrada

A função pública `introspect()` (e quaisquer outras que chamem
`walk` directamente) precisa de criar `Locator` e `Vec<Tag>`,
chamar walk, e descartar tags.

1. Em `01_core/src/rules/introspect.rs`:

   ```rust
   pub fn introspect(content: &Content) -> CounterStateLegacy {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags);
       // tags descartadas em M1; M2/M3 começarão a consumi-las
       drop(tags);
       state
   }
   ```

   Manter assinatura pública que retorna `CounterStateLegacy`
   — consumers actuais continuam a funcionar sem mudança.

2. Se `materialize_time` ou outras funções públicas chamam
   `walk` directamente, adaptar igual.

3. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Documentar que pontos de entrada criam `Locator` +
     `Vec<Tag>` internamente; tags são descartadas até
     M2/M3.

**Critério de saída**:
- API pública preservada (assinaturas públicas inalteradas).
- `cargo check` passa.
- `cargo test` — todos os tests existentes passam.
- Linter passa.

### .G Tests do walk com tags

Tests novos para verificar que walk emite tags básicas
correctamente. Tests E2E completos ficam para P163; aqui
são tests unitários mínimos.

1. Adicionar helper de teste (`#[cfg(test)]`) em
   `rules/introspect.rs`:

   ```rust
   #[cfg(test)]
   pub(crate) fn introspect_with_tags(
       content: &Content,
   ) -> (CounterStateLegacy, Vec<Tag>) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags);
       (state, tags)
   }
   ```

2. Tests unitários em `rules/introspect.rs` ou módulo de
   tests adjacente:
   - **Test de emissão básica**: walk sobre `Content::Heading`
     simples produz pelo menos um `Tag::Start` + um `Tag::End`
     com a mesma `Location`.
   - **Test de não-emissão**: walk sobre `Content::Text`
     não produz tags.
   - **Test de aninhamento**: walk sobre Heading contendo
     Figure produz 4 tags em ordem: Start(Heading),
     Start(Figure), End(Figure), End(Heading).
   - **Test de paralelismo**: após walk,
     `CounterStateLegacy` tem o conteúdo esperado E `tags`
     tem o número esperado de tags (ambos populados).

**Critério de saída**:
- 4 tests novos passam.
- `cargo test` — todos os tests passam.
- Linter passa.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Contagem de tests aumenta vs baseline P161 (smoke V2 dos
   ficheiros novos + tests do walk em .G). Documentar Δ.
3. `crystalline-lint`: zero violations.
4. Os 2 ficheiros L1 novos existem:
   - `01_core/src/entities/content_hash.rs`.
   - `01_core/src/rules/introspect/extract_payload.rs`.
5. Os 2 L0 novos existem:
   - `00_nucleo/prompts/entities/content_hash.md`.
   - `00_nucleo/prompts/rules/introspect/extract_payload.md`.
6. L0 de `introspect.rs` reflecte nova assinatura de walk.
7. L0 de `element_payload.md` e `tag.md` actualizados (sem
   notas de placeholder).
8. Walk em `introspect.rs` aceita `&mut Locator` e
   `&mut Vec<Tag>`.
9. Função pública `introspect()` retorna `CounterStateLegacy`
   (assinatura preservada).
10. Snapshot tests de paridade ADR-0033 passam inalterados.
11. Linter passa em verificação final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-162-relatorio.md` com:

- Resumo: `hash_content` materializado; placeholders P161
  resolvidos; `extract_payload` criado; walk emite
  `Vec<Tag>` em paralelo; tags descartadas em M1.
- Confirmação de cada verificação .H.
- Hashes finais dos 2 L0 novos (preenchidos pelo linter).
- Hashes actualizados dos 3 L0 modificados:
  `element_payload.md`, `tag.md`, `introspect.md`.
- Decisões registadas em .A:
  - Forma de hash de `Content` (siphasher? alternativo?).
  - Mecanismo de label confirmado.
  - Adaptações em `extract_payload` para campos reais de
    `Content::Heading/Figure/Citation`.
- Pendências para P163: tests E2E de paralelismo +
  consistência por kind + bracketing válido.
- Estado pós-passo: pronto para P163 (verificação completa
  de captura).

---

## Critério de conclusão

Todas em conjunto:

1. .A produziu inventário sem disparar gate.
2. `hash_content` materializado em `entities/content_hash.rs`
   com L0 e L1.
3. Placeholders P161 resolvidos: L0 de `element_payload` e
   `tag` sem notas de pendência sobre body_hash/u128 placeholder.
4. `extract_payload` criado em
   `rules/introspect/extract_payload.rs` com L0 e L1.
5. Walk em `introspect.rs` aceita `Locator` + `Vec<Tag>`,
   emite tags em paralelo. L0 de `introspect.rs` reflecte
   alteração.
6. Pontos de entrada criam e descartam tags. API pública
   preservada.
7. Tests do walk com tags (.G) passam.
8. Verificações .H 1-11 passam.
9. Relatório .I escrito.
10. Output observable não mudou.

---

## O que pode sair errado

- **`Content::Heading.body` não é walkável directamente
  (.A gate)**: pode ser que body seja `Vec<Content>` ou
  `Arc<Content>` ou outra forma. Adaptar `hash_content` e
  `extract_payload` à estrutura real. Se for forma que não
  permite hash determinístico (ex. ponteiro), reportar e
  reabrir decisão.
- **Mecanismo de label divergente (.A gate)**: se label não
  é `Option<Label>` em `Content::Heading` directamente,
  `extract_payload` precisa de lógica adicional (selector,
  busca em irmãos, etc.). Pode crescer para tipo helper
  separado. Aceitar e registar.
- **`hash_content` falha em determinismo**: se Content contém
  `HashMap` que itera em ordem não-determinística, hash varia
  entre runs. Detectar via test "chamar 100 vezes" em .B;
  corrigir antes de prosseguir.
- **Walk recursivo com 4 parâmetros torna assinatura longa**:
  aceitável em M1; refactor para `WalkContext` agregador é
  trabalho de M5/M6.
- **Pontos de entrada múltiplos**: se `walk` é chamado de
  mais que `introspect()` (ex. layout, materialize_time),
  todos precisam adaptar. .A número 5 deve ter inventariado;
  se não foi inventariado completamente, esta secção pode
  inflar inesperadamente.
- **Linter detecta divergência L0↔L1 nos modificados**: ao
  actualizar L0 de `introspect.rs`, `element_payload.md`,
  `tag.md`, o linter pode falhar se mudanças no L0 não
  baterem com L1. Ajustar conforme erro reportado.
- **Tags emitidas mas inconsistentes com `CounterStateLegacy`**:
  detectado via test "paralelismo" em .G. Se walk arm para
  Heading em `extract_payload` produz `CounterUpdate` que
  não bate com mutação real em `CounterStateLegacy`,
  divergência aparece em P163. Tentar antecipar em .G.

---

## Notas operacionais

- **Tamanho**: M-L. 2 L0+L1 novos + actualização de 3 L0
  existentes + modificação de walk + tests é trabalho
  substancial. Cada sub-passo individualmente testável.
- **Pré-condição P163**: walk emitir tags em paralelo é base
  para tests E2E de consistência em P163. Sem P162, P163
  não pode começar.
- **Preservação API pública**: assinaturas de `introspect()`
  e outras funções públicas inalteradas. Consumers externos
  (layout, materialize_time, etc.) continuam a funcionar
  sem alteração. Isto é requisito hard — se uma assinatura
  pública mudar, parar e reabrir.
- **Tags descartadas em M1**: `drop(tags)` no fim de
  `introspect()` é deliberado. M2 (ou M3) vai começar a
  consumir. Não é desperdício; é fase de validação que
  walk produz tags correctas antes de consumir.
