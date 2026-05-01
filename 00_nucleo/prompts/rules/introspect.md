# L0 — Motor de Introspecção (`rules/introspect.rs`)
Hash do Código: b988ff3d

## Módulo
`01_core/src/rules/introspect.rs`

**Histórico relevante**:
- 2026-04-30 (P161): renomeação `CounterState` → `CounterStateLegacy`.
- 2026-04-30 (P162): walk passa a aceitar `&mut Locator` + `&mut Vec<Tag>` + `Option<&Label>`; emite `Tag::Start`/`Tag::End` em paralelo a mutação de estado.
- 2026-04-30 (P163): este L0 refinado para reflectir P162.
- 2026-04-30 (P165): `from_tags` constrói `TagIntrospector` em paralelo; resultado descartado em M3.
- 2026-04-30 (P166 / M4): adicionado entry point `introspect_with_introspector`; `introspect()` passa a ser wrapper que descarta `TagIntrospector`.

## Propósito
Pré-passagem analítica sobre `Content`. Constrói o `CounterStateLegacy`
completo (incluindo `resolved_labels`) antes do layout físico arrancar.
Permite resolver referências para a frente (forward refs).

A partir de P162, em paralelo, produz uma sequência `Vec<Tag>` que captura
Heading/Figure/Cite com `Location` única (gerada por `Locator`) e
`ElementInfo` com payload + label opcional. As tags são **descartadas em
M1** — consumo real começa em M2/M3 quando `Introspector` for materializado.

## Regras de negócio

### O que a introspecção faz
- Percorre `Content` recursivamente via `walk()`.
- Avança contadores (`step_hierarchical`, `step_flat`) nos mesmos
  nós onde o Layouter o faria.
- Regista `resolved_labels` para cada `Labelled` encontrado.
- Intercede em `SetHeadingNumbering` e `CounterUpdate` para replicar
  os side-effects de estado.
- **P162**: emite `Tag::Start(Location, ElementInfo)` quando o nó é
  payload-yielder (Heading/Figure/Cite) e `Tag::End(Location, u128)`
  ao subir. Emparelhamento garantido por construção.

### O que a introspecção NÃO faz
- Não acede a `FontMetrics`.
- Não aloca `Frame`, `FrameItem`, ou `PagedDocument`.
- Não produz output visual de nenhum tipo.
- **P162**: não consome o `Vec<Tag>` produzido — descarta no fim de
  `introspect()`. M2/M3 começarão a consumir.

### Isolamento
A função pública `introspect(content: &Content) -> CounterStateLegacy`
é pura: dado o mesmo `Content`, retorna sempre o mesmo
`CounterStateLegacy`. Não tem estado global.

`Locator` e `Vec<Tag>` são internos a cada chamada — instanciados em
`introspect()`, propagados por `walk` recursivo, e descartados no fim.
Sem partilha entre chamadas.

### Integração com o layout físico
A função `layout(content)` executa automaticamente:
1. `introspect(content)` → obtém `resolved_labels` populado.
2. Inicia o Layouter com `resolved_labels` injectados.
3. Reconstrói `hierarchical`, `flat` e `numbering_active` nó a nó
   durante o layout — NÃO copia estes campos da introspecção, para que
   os prefixos visuais sejam gerados na ordem correcta.

## Assinaturas internas (P162)

```rust
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
);
```

`label_from_parent` carrega a label de um wrapper `Content::Labelled`
para o `target` recursivo (P162 .A decisão sobre mecanismo de label).
No arm `Content::Labelled`, walk recursa com `Some(label)`; nos outros
arms, recursa com `None`.

### Lógica de emissão de tags

No topo de `walk`:
1. Chamar `extract_payload(content)` (em `rules/introspect/extract_payload.rs`).
2. Se `Some(payload)`:
   - `let location = locator.next();`
   - `let info = ElementInfo { payload, label: label_from_parent.cloned() };`
   - `tags.push(Tag::Start(location, info));`
   - Guardar `location` para emissão do `End`.
3. Mutação de estado existente prossegue (não alterada).
4. Recursão para filhos (passando `state`, `locator`, `tags` por mutação;
   `label_from_parent` é `None` excepto no arm `Content::Labelled`).
5. Se emitiu `Start`, no fim de walk:
   - `tags.push(Tag::End(location, hash_content(content)));`

Pareamento Start↔End garantido: cada `Start(loc, _)` é seguido pelo seu
`End(loc, _)` correspondente após a recursão dos filhos. Bracketing
válido por construção (verificado por tests E2E em P163.C.2).

## Interface pública

Duas funções pub a partir de M4 (P166):

```rust
/// Entry point legacy — wrapper que descarta TagIntrospector.
pub fn introspect(content: &Content) -> CounterStateLegacy;

/// Entry point novo (M4 / P166) — produz state + introspector
/// num único walk.
pub fn introspect_with_introspector(
    content: &Content,
) -> (CounterStateLegacy, TagIntrospector);
```

Forma interna (P166):
```rust
pub fn introspect(content: &Content) -> CounterStateLegacy {
    let (state, _introspector) = introspect_with_introspector(content);
    state
}

pub fn introspect_with_introspector(
    content: &Content,
) -> (CounterStateLegacy, TagIntrospector) {
    let mut state = CounterStateLegacy::new();
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    walk(content, &mut state, &mut locator, &mut tags, None);
    let introspector = self::from_tags::from_tags(&tags);
    (state, introspector)
}
```

**Walk único**: state + introspector vêm da mesma passagem — não há
duplicação. `introspect()` é wrapper que descarta o introspector,
preservando assinatura legada para os ~38 call-sites identificados em
P166 .A.

Padrão de migração M5+: caller que actualmente faz `let state =
introspect(&c)` e quer queries via Introspector adopta
`let (state, intr) = introspect_with_introspector(&c)` sem custo
adicional. M6 eliminará o wrapper + `CounterStateLegacy` quando todos
os consumers tiverem migrado.

Helper de teste em `#[cfg(test)]`:
```rust
fn introspect_with_tags(content: &Content) -> (CounterStateLegacy, Vec<Tag>);
```
Disponível só em testes para verificar a captura de tags em paralelo
(P162.G + P163.C/.D). API pública não muda.

## Sobre paridade

Vanilla não tem walk explícito sobre `Content`. Usa `comemo` + `convergence::analyze` para fixpoint multi-iteração com type-erased `Introspect` ops. Cristalino diverge: walk single-pass directo + tags em paralelo (P162). Quando M2/M3 introduzirem consumo real, a divergência pode estreitar mas não inverter — cristalino não vai usar `comemo` para introspecção runtime (decisão herdada de ADR-0066 PROPOSTO).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) para o mapa completo de tipos vanilla e quais cristalino materializa.

## Critérios de verificação
- `Labelled` após `Heading` → `resolved_labels` contém a chave.
- `Labelled` antes de `Heading` (forward ref) → `resolved_labels` contém
  a chave (porque `walk` percorre o `target` antes de registar).
- `CounterUpdate { action: Update(5) }` → `flat["equation"] == 5`.
- `SetHeadingNumbering { active: true }` → `is_numbering_active("heading") == true`.
- Dois documentos independentes → estados independentes (sem partilha).
- `layout(content)` com forward ref → texto resolvido (não `@nome`).
- **P162/P163**:
  - Walk sobre `Content::Heading` produz `Tag::Start` + `Tag::End`
    com `Location` igual.
  - Walk sobre `Content::Text` não produz tags.
  - Walk sobre `Content::Labelled { target: Heading, label }` emite
    `Tag::Start` para Heading com `info.label = Some(label)`.
  - Walk duas vezes sobre o mesmo Content produz `Vec<Tag>` idêntico
    (determinismo).
  - Bracketing válido em qualquer aninhamento (todo Start tem o seu End,
    sem overlapping).
  - Hash em `Tag::End` distingue Contents diferentes.
  - Número de `ElementPayload::Heading` em tags == número de headings no
    input.
