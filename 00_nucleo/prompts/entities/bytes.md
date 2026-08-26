# Prompt L0 — `Bytes` — sequência binária opaca
Hash do Código: 374a56fc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/bytes.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 398 — modelagem de `Value::Bytes` (S-M); fecha DEBT-62 e activa `read` binário + byte-strings CBOR.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1).

---

## 1. Contexto

O Typst vanilla expõe `bytes` como uma sequência binária. Em cristalino, `read(path)` foi materializado em P387 apenas como texto (modo UTF-8); binário foi deferido porque o enum `Value` era fechado (ADR-0017). Com o portão aberto em P395 (`Value::Tiling`), `Value::Bytes` pode ser modelado.

> **Fonte de paridade (P1031)** — doc comment `#[ty]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/foundations/bytes.rs:16-28`, publicado em
> `typst.app/docs/reference/foundations/bytes/`:
>
> *"A sequence of bytes. This is conceptually similar to an array of integers between `{0}`
> and `{255}`, but represented much more efficiently. You can iterate over it using a for
> loop. You can convert - a string or an array of integers to bytes with the `bytes`
> constructor - bytes to a string with the `str` constructor, with UTF-8 encoding - bytes to
> an array of integers with the `array` constructor. When reading data from a file, you can
> decide whether to load it as a […]"*
>
> **Correcção de redacção**: a documentação **não** caracteriza `bytes` como *opaco* — pelo
> contrário, enuncia-o como equivalente conceptual a um array de inteiros 0–255, iterável e
> convertível nos dois sentidos com `str` e `array`. A palavra "opaco" foi removida do
> contexto acima, por ser afirmação sobre a linguagem que a fonte não sustenta.
>
> **Natureza**: literal para a definição do tipo e para as três conversões. O que este L0
> descreve como `Bytes(pub Vec<u8>)` é representação interna — mecânica, diverge de
> propósito (ADR-0107/ADR-0030).

## 2. Tipo L1

```rust
// 01_core/src/entities/bytes.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self { Self(data) }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
}

impl From<Vec<u8>> for Bytes { ... }
impl From<Bytes> for Vec<u8> { ... }
```

- `Vec<u8>` é suficiente para S-M; `EcoVec<u8>` é refino futuro XS.
- Semântica opaca — bytes não são texto nem array de ints.

## 3. Variant `Value::Bytes`

```rust
// 01_core/src/entities/value.rs
Bytes(crate::entities::bytes::Bytes),
```

Atualizações necessárias:
- `type_name()` → `"bytes"`.
- `PartialEq` via derive (a struct já implementa).
- `Hash` via derive/Debug formatting (padrão existente).
- Opcional: `cast_bytes()` para extrair `&Bytes`.

## 4. Repr

Não existe `native_repr` em L1. Quando existir, a representação deverá ser `"bytes({len})"` — nunca dump do conteúdo binário.

## 5. Cast

- `Value::Bytes(b) -> Bytes`: identidade.
- `Value::Str(s) -> Bytes`: encode UTF-8 (sempre válido).
- Outros tipos → erro de tipo.

## 6. Scope-out

- Operações `len`, `at`, `slice`, concatenação — stdlib futura (S).
- `EcoVec<u8>` — refino XS.
- Encoding detection sofisticado (BOM, ISO, etc.) — ADR-0054 graded.

## 7. Testes

- Construção, `len`, `is_empty`, `PartialEq`, `clone`, `From`.
- `Value::Bytes` discriminação, `type_name`, `Hash`.
