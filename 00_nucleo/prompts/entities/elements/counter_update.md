# Prompt L0 — `entities/elements/counter_update` — `CounterUpdateElem`
Hash do Código: eab67012

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/counter_update.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico (P198C). A mais larga
do lote (largura 39).

> **Fronteira: LOCATÁVEL**. `element_kind` → `ElementKind::CounterUpdate`;
> `to_payload` → `ElementPayload::CounterUpdate`. `from_tags` aplica à
> `CounterRegistry` (`apply_at`/`apply_hierarchical_at`; consumo por payload,
> inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterUpdateElem {
    pub key:    String,
    pub action: CounterAction,   // entities::counter_update (deriva Hash)
}
```

`Content::CounterUpdate { key, action }` → `Content::CounterUpdate(Arc<…Elem>)`.
Construtor: `Content::counter_update(key, action)`.

> **`Hash` por derive** se o tipo de `action` derivar `Hash` (o enum em
> `counter_update.rs:20` deriva `Hash`). **Confirmar em Fase B**: se `action`
> carregar `Value`/`f64`, cair para `Hash` manual via Debug (regra do modelo).

## `impl Element`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1704`) |
| `is_empty` | default `false` |
| `map_*` | **terminais** |
| `element_kind` | `Some(ElementKind::CounterUpdate)` |
| `to_payload` | `Some(ElementPayload::CounterUpdate { key, action })` (absorve `extract_payload.rs:129`) |

## `eq`

`#[derive(PartialEq)]` compara `key + action` (paridade `content.rs:1868`) →
**dispatch** `(CounterUpdate(a), CounterUpdate(b)) => a == b`.

## P1342 — preservação do carrier aninhado da ação Func

### Medição anterior à decisão

`diagnosticos/p1342-topology-audit-r1.md` mede que o elemento e o payload
clonam `key` e `action`; para `CounterUpdate::Func`, o `Func` real é o objeto
que já atravessa Content, mapeamentos e walk. Um campo adicional no elemento
não é necessário para a fixture medida.

### Decisão vigente

Sob `cfg(p1339_observation)`, a ação Func pode conter o carrier privado P1342
definido pelo owner `entities/func`. O derive Clone, `map_content`, `map_text` e
`to_payload` preservam esse mesmo carrier por clone do Func. Não extrair,
recriar ou indexar o carrier por side table; Set/Step não têm carrier.

Struct normal, campos públicos, payload, Eq/Hash, locatability e resultado
permanecem inalterados. A cláusula só legitima o transporte test-only já
aninhado no Func para o fragmento P1342.
