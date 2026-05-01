# Prompt L0 — `entities/tag`
Hash do Código: 9af0a44d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/tag.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`Tag` marca início e fim de um elemento indexável durante a passagem de introspecção. Forma análoga a `lab/typst-original/crates/typst-library/src/introspection/tag.rs::Tag` (linha 12), com simplificação: apenas duas variantes `Start` e `End`.

Walk em P162 percorrerá `Content` e, para cada `Content::Heading`/`Content::Figure`/`Content::Cite`, emitirá `Tag::Start(loc, ElementInfo)` antes de descer para o body e `Tag::End(loc, content_hash)` ao subir. A sequência de tags é o registo cronológico do walk.

P161 cria apenas a definição do tipo. Walk não emite tags ainda — isso é P162. Em P161 nenhum consumer ergue `Tag`.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone` derivado para que stream de tags possa ser clonado/passado por valor entre fases.
- Sem `Arc` interno — `ElementInfo` é clonado por valor (custo aceitável: payload + Option<Label>, sem Content).

---

## Interface pública

```rust
use crate::entities::element_info::ElementInfo;
use crate::entities::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Início de um elemento indexável.
    /// Carrega Location única (gerada por Locator) e o payload+label do elemento.
    Start(Location, ElementInfo),

    /// Fim do elemento. Carrega a Location correspondente ao Start
    /// (para emparelhamento) e um content_hash u128 — paridade vanilla
    /// para detecção de duplicação cross-iteration.
    End(Location, u128),
}
```

---

## Semântica

- `Tag::Start(loc, info)`: marca o ponto onde o walk entra num elemento indexável. `loc` é única na sequência (gerada por `Locator::next()`). `info` carrega o payload e a label opcional.
- `Tag::End(loc, content_hash)`: marca a saída do elemento. `loc` é a mesma do `Start` correspondente (emparelhamento Start↔End). `content_hash` é o hash do conteúdo (Content) do elemento, populado em `walk` via `hash_content` (`entities/content_hash.rs`, P162 .B+.E). Paridade com vanilla, onde o hash é guardado no End para optimização de queries.

---

## Invariantes

- Cada `Start(loc, _)` é seguido (eventualmente) por exactamente um `End(loc, _)` na sequência produzida por um walk completo. Sub-walks aninhados produzem Start/End correctamente aninhados.
- A `Location` em `Start` e `End` correspondentes é exactamente a mesma (igualdade `Eq` no `u128` interno).
- Fora de uma sequência de walk, `Tag`s podem ser comparados por valor (`Eq`/`Hash` derivados) — útil para tests de regressão.

---

## Consumers actuais

Nenhum em P161 — walk não emite tags ainda. `Tag` existe como definição passiva.

## Consumers planeados

- `rules/introspect.rs` walk em P162 — emite `Vec<Tag>` em paralelo ao `CounterStateLegacy` actual. Walk continua a fazer o que faz hoje (avançar contadores, popular labels) e adicionalmente passa a coleccionar tags.
- `Introspector` (M3) — consome a sequência de tags para construir o índice de elementos.
- Snapshot tests em P163 — comparação determinística de sequências de tags entre execuções equivalentes.

---

## Sobre paridade

Vanilla `Tag` em `tag.rs` linha 12:

```rust
pub enum Tag {
    Start(Content, TagFlags),
    End(Location, u128),  // u128 = key hash
}
```

Diferenças cristalino P161:

- Vanilla `Start(Content, TagFlags)`: carrega o `Content` inteiro do elemento e flags (`introspectable`, `tagged`). Cristalino `Start(Location, ElementInfo)`: separa Location explícita (vanilla extrai-a do Content) e usa `ElementInfo` (subset focado) em vez de Content completo.
- Vanilla `TagFlags` (struct com 2 bools): scope-out em P161. Não há tagged-PDF nem distinção introspectable/non-introspectable porque todos os elementos no enum cristalino `ElementKind` são introspectable.
- Vanilla `TagElem` (`#[elem]` wrapper): scope-out — cristalino não usa vtable elements; tags fluem como `Vec<Tag>` do walk para o consumer.
- `End` é estruturalmente idêntico (`(Location, u128)`).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de `TagFlags` (concept needed; forma vanilla aplicável quando relevante) e `TagElem` (vtable — scope-out).

---

## Resultado Esperado

- `01_core/src/entities/tag.rs` — enum + tests unitários (construção Start/End, igualdade entre tags equivalentes).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .9: marcadores Start/End para introspecção fixpoint M1 | `tag.rs`, `tag.md` |
