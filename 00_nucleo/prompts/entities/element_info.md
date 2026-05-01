# Prompt L0 — `entities/element_info`
Hash do Código: d36b7190

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/element_info.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .8)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`ElementInfo` agrupa o `ElementPayload` (dados específicos do elemento por kind) com a label opcional atribuída por sintaxe `<label>`. É a unidade que viaja dentro de `Tag::Start(Location, ElementInfo)` durante a passagem de introspecção.

Separação `ElementPayload` (kind-específico) + `Option<Label>` (geral) reflecte que **qualquer** elemento indexado pode ter uma label, independentemente do kind. Vanilla embute label dentro de cada `*Elem` via vtable; cristalino externaliza para evitar duplicar `Option<Label>` em cada variante de `ElementPayload`.

`Label` já existe em `entities/label.rs` (`pub struct Label(pub String)` derivando `Clone, PartialEq, Eq, Hash`).

---

## Restrições Estruturais

- Camada **L1**: struct puro.
- `Clone` derivado para passar por valor para `Tag`.
- Sem alocação extra além do que `ElementPayload` e `Label` já comportam.

---

## Interface pública

```rust
use crate::entities::element_payload::ElementPayload;
use crate::entities::label::Label;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementInfo {
    pub payload: ElementPayload,
    pub label:   Option<Label>,
}

impl ElementInfo {
    /// Construtor sem label.
    pub fn new(payload: ElementPayload) -> Self;

    /// Construtor com label.
    pub fn with_label(payload: ElementPayload, label: Label) -> Self;
}
```

---

## Semântica

- `new(payload)`: constrói `ElementInfo { payload, label: None }`.
- `with_label(payload, label)`: constrói `ElementInfo { payload, label: Some(label) }`.
- Os campos são públicos — consumers podem aceder directamente para pattern matching ou construção literal.

---

## Invariantes

- `payload` é sempre presente (sem `Option<ElementPayload>` — cada `ElementInfo` é construído porque há um elemento concreto a indexar).
- `label` é opcional — reflecte fielmente a sintaxe Typst (`<intro>` aparece como sufixo opcional, não obrigatório).
- `Hash`/`Eq` derivados — duas `ElementInfo` são iguais sse `payload` e `label` forem iguais.

---

## Consumers actuais

Nenhum em P161.

## Consumers planeados

- `entities/tag.rs::Tag::Start(Location, ElementInfo)` (P161 sub-passo .9).
- `rules/introspect.rs` walk em P162 — branches Heading/Figure/Cite construindo `ElementInfo::new(payload)` ou `ElementInfo::with_label(payload, label)` consoante o nó tenha sido envolvido em `Content::Labelled { ... }`.
- Registry de elementos (M3+) — chave `Location`, valor `ElementInfo`.

---

## Sobre paridade

Vanilla não tem `ElementInfo` separado. Cada `*Elem` (HeadingElem, FigureElem, CiteElem) carrega o seu próprio campo `label` via macro `#[elem]`. O equivalente combinado vanilla é o `Content` indexado pelo `Introspector`, com a label acessível via `Content::label()`.

Cristalino externaliza para um struct simples porque:
1. Sem vtable — não há `*Elem` por feature; há um enum `ElementPayload` estreito.
2. A separação payload/label expressa que label é ortogonal ao kind.
3. Permite que registries futuros indexem por label (`HashMap<Label, Location>`) sem precisar de pattern-match no payload.

---

## Resultado Esperado

- `01_core/src/entities/element_info.rs` — struct + 2 construtores + tests unitários (new, with_label, igualdade, hash).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .8: agregador payload + label opcional para Introspection M1 | `element_info.rs`, `element_info.md` |
