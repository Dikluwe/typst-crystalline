# L0 — Motor de Introspecção (`rules/introspect.rs`)
Hash do Código: 264f58c8

## Módulo
`01_core/src/rules/introspect.rs`

## Propósito
Pré-passagem analítica sobre `Content`. Constrói o `CounterState`
completo (incluindo `resolved_labels`) antes do layout físico arrancar.
Permite resolver referências para a frente (forward refs).

## Regras de negócio

### O que a introspecção faz
- Percorre `Content` recursivamente via `walk()`.
- Avança contadores (`step_hierarchical`, `step_flat`) nos mesmos
  nós onde o Layouter o faria.
- Regista `resolved_labels` para cada `Labelled` encontrado.
- Intercede em `SetHeadingNumbering` e `CounterUpdate` para replicar
  os side-effects de estado.

### O que a introspecção NÃO faz
- Não acede a `FontMetrics`.
- Não aloca `Frame`, `FrameItem`, ou `PagedDocument`.
- Não produz output visual de nenhum tipo.

### Isolamento
A função pública `introspect(content: &Content) -> CounterState`
é pura: dado o mesmo `Content`, retorna sempre o mesmo `CounterState`.
Não tem estado global.

### Integração com o layout físico
A função `layout(content)` executa automaticamente:
1. `introspect(content)` → obtém `resolved_labels` populado.
2. Inicia o Layouter com `resolved_labels` injectados.
3. Reconstrói `hierarchical`, `flat` e `numbering_active` nó a nó
   durante o layout — NÃO copia estes campos da introspecção, para que
   os prefixos visuais sejam gerados na ordem correcta.

## Interface pública
```rust
pub fn introspect(content: &Content) -> CounterState;
```

## Critérios de verificação
- `Labelled` após `Heading` → `resolved_labels` contém a chave.
- `Labelled` antes de `Heading` (forward ref) → `resolved_labels` contém
  a chave (porque `walk` percorre o `target` antes de registar).
- `CounterUpdate { action: Update(5) }` → `flat["equation"] == 5`.
- `SetHeadingNumbering { active: true }` → `is_numbering_active("heading") == true`.
- Dois documentos independentes → estados independentes (sem partilha).
- `layout(content)` com forward ref → texto resolvido (não `@nome`).
