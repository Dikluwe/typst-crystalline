# Prompt L0 — `rules/introspect/extract_payload`
Hash do Código: 2a0d9c0d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/introspect/extract_payload.rs`
**Criado em**: 2026-04-30 (P162 sub-passo .D)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`extract_payload` é a função pura que mapeia um `&Content` para `Option<ElementPayload>`. `Some(...)` para os 3 kinds locatable em M1 (`Heading`, `Figure`, `Cite`); `None` para todas as outras variantes de Content.

Vanilla resolve via vtable: cada `*Elem` implementa `Locatable` marker trait + lógica de extracção via proc-macro. Cristalino prefere função pura com match exaustivo — o compilador força cobertura, e adicionar um variant novo a `Content` produz erro de compilação aqui até decisão sobre locatability.

P162 sub-passo .D introduz a função; consumida em P162 .E pelo walk modificado.

---

## Restrições Estruturais

- Camada **L1**: função pura, sem efeitos secundários, sem I/O.
- Sem alocação significativa: clones de `Option<String>` e `String` são aceitáveis (são pequenos e raros).
- Match exaustivo (sem `_ => panic!`); fallback é `_ => None` para variantes não-locatable. Adicionar variant locatable nova exige edição explícita aqui.
- Sem dependência em estado (não recebe `CounterStateLegacy` nem walk-context).

## Mapeamento Content → ElementPayload (campos confirmados em P162 .A)

| Content variant | Campos lidos | ElementPayload |
|-----------------|--------------|----------------|
| `Heading { level: u8, body: Box<Content> }` | `level`, `body` (para hash) | `Heading { depth: level, body_hash: hash_content(body), counter_update: CounterUpdate::Step }` |
| `Figure { kind: Option<String>, numbering, caption, .. }` | `kind`, `numbering`, `caption` (P168: predicate `is_counted`) | `Figure { kind: kind.clone(), counter_update: CounterUpdate::Step, is_counted: numbering.is_some() && caption.is_some() }` |
| `Cite { key: String, .. }` | `key` apenas (supplement/form irrelevantes para payload) | `Citation { key: key.clone() }` |
| `Metadata { value }` | `value` (boxed) | `Metadata { value: value.clone() }` (P169 M9) |
| Outras (Text, Sequence, Math*, etc.) | — | `None` |

---

## Interface pública

```rust
use crate::entities::content::Content;
use crate::entities::element_payload::ElementPayload;

pub fn extract_payload(content: &Content) -> Option<ElementPayload>;
```

---

## Semântica

- `extract_payload(c)` é determinístico — mesma entrada produz mesmo output.
- `Some(...)` apenas para os 3 kinds em M1; expandir requer edição explícita.
- Ordem dos arms no match não é semanticamente significativa (cada variante de Content é distinta).

---

## Invariantes

- Função pura sem side-effects.
- Não chama `walk` nem manipula `CounterStateLegacy`.
- Hashing de body via `hash_content` é determinístico (ver `entities/content_hash.md`).
- `_ => None` é literal — nenhum variant fora dos 3 documentados produz `Some`.

---

## Tests obrigatórios (sub-passo .D P162)

- Heading básico → `Some(ElementPayload::Heading { depth, body_hash, counter_update })`.
- Figure básico → `Some(ElementPayload::Figure { kind, counter_update })`.
- Cite básico → `Some(ElementPayload::Citation { key })`.
- Text → `None`.
- Empty / Space → `None`.

---

## Consumers actuais

Nenhum no momento da criação. Imediatamente consumido em P162 sub-passo .E.

## Consumers planeados

- `rules/introspect.rs` walk (P162 .E) — chama `extract_payload(content)` antes da mutação de estado, emite `Tag::Start` se `Some`.
- `Introspector` em M3 — pode reusar para indexar elementos por kind.

---

## Sobre paridade

Vanilla usa marker trait `Locatable` + proc-macro `#[elem(Locatable)]` em `HeadingElem`, `FigureElem`, `CiteElem`. A "extracção" do payload é feita pelo Introspector via `Content::elem()` + `vtable::field()` calls. Cristalino: match exaustivo top-level — sem vtable, sem trait markers per element.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de marker traits vanilla como vtable-driven (scope-out cristalino).

---

## Resultado Esperado

- `01_core/src/rules/introspect/extract_payload.rs` — função + 5 tests unitários.
- `01_core/src/rules/introspect/mod.rs` (criado neste passo) — re-export `extract_payload`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P162 sub-passo .D: função pura Content→ElementPayload para introspecção M1 | `extract_payload.rs`, `extract_payload.md`, `rules/introspect/mod.rs` |
