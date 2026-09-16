# Prompt L0 — `entities/elements/math_root` — `MathRootElem`
Hash do Código: 8a7994c7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_root.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathRootElem {
    pub index:    Option<Content>,   // era Option<Box<Content>>; None = raiz quadrada
    pub radicand: Content,           // era Box<Content>
    pub index_explicit: bool,        // P1426: presença causal do primeiro argumento
}
```

`Content::MathRoot { index, radicand }` → `Content::MathRoot(Arc<MathRootElem>)`.
Construtor ergonómico: `Content::math_root(index: Option<Content>, radicand: Content,
index_explicit: bool)`.

## `impl Element for MathRootElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `None => format!("sqrt({})", radicand…)`; `Some(i) => format!("root({}, {})", i…, radicand…)` (`content.rs:1625`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** (`content.rs:2107`): `index` via `.as_ref().map(map_content).transpose()?`; `radicand.map_content(f)?`; preserva `index_explicit` nominalmente |
| `map_text` | **terminal** (math structural; `content.rs:2623`): `Content::MathRoot(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `index + radicand + index_explicit`.

## Critério

`plain_text` `sqrt(...)`/`root(i, ...)`; `map_content` recurse preservando `None`
no index; `map_text` terminal; igualdade estrutural.

## P1140.3-A — produtor público `math.sqrt`

Além da sintaxe em modo math, `math.sqrt(radicand: Content)` é função pública
e produz o mesmo `Content::MathRoot` com `index: None`. A função pertence à
stdlib math e não a esta entidade: `MathRootElem` continua sendo dado puro,
sem dependência inversa para avaliação (ADR-0109, forma B). `calc.sqrt` é uma
função numérica distinta.

## P1426 — presença causal do índice (PROPOSTA; gate ADR-0127 pendente)

O índice tem três estados públicos: ausente (`None/false`, caso `sqrt`),
explicitamente `none` (`None/true`) e conteúdo presente (`Some(_)/true`, que
inclui conteúdo vazio). O bit não altera `plain_text` nem geometria, mas é
morfologia causal e deve sobreviver a clone, hash, igualdade e walkers.

A implementação desta secção exige confirmação explícita do dono por alterar
campo e assinatura públicos.
