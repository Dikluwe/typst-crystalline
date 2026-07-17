# Passo 398 — Modelagem de Tipos: `Value::Bytes` + `read` binário (S-M)

**Tipo**: Modelagem de tipos primitivos + ativação de consumer (L1 tipo puro + L3 I/O read binário; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + DEBT-62 + P395); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out — `read` modo texto foi graded em P387), DEBT-62 (read binário deferred).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Bytes` ausente, bloqueia `read` binário; DEBT-62 documenta deferimento.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Este passo fecha **DEBT-62** e promove `read` de `implementado⁺` (graded texto-only) → `implementado` (binário real). Último passo da série de tipos primitivos pendentes (Bytes → Decimal → Duration → Version).

---

## 1. Contexto

P387 materializou `read(path)` como **modo texto apenas** (`Value::Str`), com binário deferred até `Value::Bytes`. O motivo era ADR-0017 (enum `Value` fechado) — não se pode adicionar variant sem tipo migrado primeiro.

P395 abriu o portão ADR-0017 com `Value::Tiling`. Agora `Value::Bytes` pode ser modelado, e `read` binário pode ser ativado.

No vanilla:
```typ
#let data = read("logo.png")  // retorna bytes se não-UTF8
#let text = read("hello.txt") // retorna str se UTF8
```

O vanilla usa heurística de encoding: tenta UTF-8 → se falha, retorna `bytes`. No cristalino, P387 implementou `read` como texto-only com erro em binário não-UTF8. Este passo **ativa o fallback binário**.

---

## 2. Decisão de engenharia

### 2.1 — `Value::Bytes` tipo L1

```rust
// entities/bytes.rs — tipo L1 puro
pub struct Bytes(pub Vec<u8>);

// ou, se EcoBytes existir:
pub struct Bytes(pub EcoVec<u8>);
```

**Decisão**: `Vec<u8>` é suficiente para S-M. `EcoVec<u8>` (de `ecow`) é otimização futura (XS). Usar `Vec<u8>` agora, documentar `EcoVec` como refino futuro.

**Derives**: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash` (via vec), `Default` (empty vec).

**Semântica**: bytes opacos — não interpretáveis como texto (diferente de `Value::Str`). Operações: `len`, `at(index)`, `slice(start, end)`, concatenação (future).

### 2.2 — Enum `Value::Bytes`

```rust
// entities/value.rs — novo variant
Bytes(Bytes),  // não Arc — Vec<u8> já é heap-allocated, clone é deep copy
```

**Por que não Arc**: `Bytes` é um `Vec<u8>` — já é um ponteiro + len + cap no heap. Arc adiciona overhead de refcount sem benefício (Vec.clone é deep copy de dados; Arc.clone é shallow mas os dados são os mesmos). Para consistência com `Value::Str(EcoString)` (que é Arc-wrapped internamente), usar `Bytes` struct direto.

**Decisão revisável**: se `Bytes` crescer para incluir shared slices (sub-views), Arc pode ser adicionado depois sem breaking change (o variant é `Bytes(Bytes)`, não `Bytes(Vec<u8>)` direto).

### 2.3 — `read` binário — ativação consumer

P387 `native_read` em `rules/stdlib/loading.rs`:
```rust
// P387 — texto only
pub fn native_read(args: &Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let bytes = read_bytes(path)?;  // L3 I/O
    let text = String::from_utf8(bytes)
        .map_err(|_| eco_format!("arquivo não é UTF-8 válido"))?;
    Ok(Value::Str(text.into()))
}
```

**P398 — ativação binário**:
```rust
pub fn native_read(args: &Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let bytes = read_bytes(path)?;  // L3 I/O — reusa P387

    // Heurística vanilla: tenta UTF-8, fallback para bytes
    match String::from_utf8(bytes.clone()) {
        Ok(text) => Ok(Value::Str(text.into())),
        Err(_) => Ok(Value::Bytes(Bytes(bytes))),
    }
}
```

**Decisão ADR-0107 (língua vs mecânica)**: a paridade é com o **comportamento** do vanilla (heurística UTF-8), não com a mecânica interna (o vanilla usa encoding detection mais sofisticado). A heurística "UTF-8 ou bytes" é suficiente para paridade linguagem.

**Decisão ADR-0054 (graded)**: `read` binário não é graded — é funcionalidade real. O que era graded em P387 era a **ausência de binário**. Agora binário é real.

### 2.4 — `cbor` byte-strings — ativação

P387 materializou `cbor` com graded: byte-strings → `Err` (Value::Bytes ausente). Agora:

```rust
// decode_cbor.rs — ajuste braço byte-string
CborValue::Bytes(b) => Ok(Value::Bytes(Bytes(b.to_vec()))),
// ou, se ciborium retorna Vec<u8>:
CborValue::Bytes(b) => Ok(Value::Bytes(Bytes(b.into()))),
```

**Promoção**: `cbor` graded → `implementado` (ou permanece `implementado⁺` se outros graded persistirem, como datetime rico).

### 2.5 — Impacto cross-module (match exhaustivo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/value.rs` | +1 variant | `Bytes(Bytes)` |
| `eval/repr.rs` | +1 arm | `"bytes({len})"` ou `"bytes(...)"` |
| `eval/cast.rs` | +1 arm | `Bytes → Bytes` (identity); `Str → Bytes` (UTF-8 encode) |
| `eval/ops.rs` | +1 arm | `==` por vec equality |
| `layout/types.rs` | Nenhum | Bytes não é Paint/Fill/Style |
| `export.rs` | Nenhum | Bytes não emite direto (usado em images, etc.) |
| `stdlib/loading.rs` | Ajuste | `native_read` ativa fallback binário |
| `stdlib/loading/decode_cbor.rs` | Ajuste | byte-strings → `Value::Bytes` |

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `bytes.md`

Novo em `00_nucleo/prompts/entities/bytes.md`:

- **Paridade**: `Bytes` ≡ vanilla `bytes` tipo (opaque byte sequence).
- **Substrato**: tipo L1 puro `Bytes(Vec<u8>)`; zero I/O; heap-allocated.
- **Semântica**: opaque — não texto, não array de ints. Operações futuras: len, at, slice.
- **Variant `Value`**: `Bytes(Bytes)` — não Arc (Vec já é heap); revisável para EcoVec futuro.
- **Cast**: `Bytes → Bytes` (identity); `Str → Bytes` (UTF-8 encode, fallible).
- **Repr**: `"bytes({len})"` — não dumpa conteúdo (pode ser binário grande).
- **Teste**: construção `Bytes::new(vec![0x89, 0x50])` → `Value::Bytes` → repr → cast.

### A.2 — Prompt L0 `read-binario.md` (extensão)

Extensão de `00_nucleo/prompts/engine/stdlib/loading.md` (ou `read.md` se existir):

- **Paridade**: `read(path)` ≡ vanilla (heurística UTF-8 → Str, fallback → Bytes).
- **Substrato**: reusa `read_bytes` L3 (P387); adiciona fallback binário.
- **Decisão**: heurística UTF-8 é suficiente (paridade linguagem); não implementa encoding detection sofisticado (scope-out ADR-0054 graded).
- **cbor**: byte-strings → `Value::Bytes` (promove graded P387).
- **Erro**: `read` não retorna mais erro em binário não-UTF8 — retorna `Value::Bytes`.
- **Teste**: `read("texto.txt")` → Str; `read("logo.png")` → Bytes; `read("cbor-dados.cbor")` com byte-strings → Bytes.

### A.3 — CHECKPOINT

Parar. Apresentar `bytes.md` + extensão `read.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Bytes`

Em `01_core/src/entities/bytes.rs`:

```rust
use ecow::EcoVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.0
    }
}
```

**Nota**: `EcoVec<u8>` é refino futuro (XS). Documentar no L0.

### B.2 — Variant `Value::Bytes`

Em `entities/value.rs`:

```rust
Bytes(Bytes),
```

Atualizar:
- `PartialEq` — match arm `Bytes(a) => matches!(other, Bytes(b) if a == b)`.
- `Repr` — `"bytes({})".format(bytes.len())` ou `"bytes(...)"`.
- `Cast` — `Bytes(b) => Ok(b.clone())`; `Str(s) => Ok(Bytes(s.as_bytes().to_vec().into()))` (fallible? UTF-8 encode de Str é sempre válido).
- `type_name` — `"bytes"`.

### B.3 — `native_read` ativação

Em `rules/stdlib/loading.rs` (ajuste P387):

```rust
pub fn native_read(args: &Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let bytes = read_bytes(path)?;  // L3 — reusa P387

    // Heurística vanilla: UTF-8 ou bytes
    match String::from_utf8(bytes.clone()) {
        Ok(text) => Ok(Value::Str(text.into())),
        Err(_) => Ok(Value::Bytes(Bytes::new(bytes))),
    }
}
```

**Decisão**: `bytes.clone()` é necessário porque `String::from_utf8` consome o Vec. Se performance for crítica, tentar `from_utf8` + `into_bytes` em caso de falha (reusa o Vec). Simplificação: clone é aceitável para S-M (arquivos pequenos em testes; arquivos grandes são refino futuro).

### B.4 — `cbor` byte-strings

Em `rules/stdlib/loading/decode_cbor.rs` (ajuste P387):

```rust
// Antes (P387):
// CborValue::Bytes(_) => bail!("byte-strings não suportados — Value::Bytes ausente"),

// Depois (P398):
CborValue::Bytes(b) => Ok(Value::Bytes(Bytes::new(b.to_vec()))),
```

**Nota**: verificar tipo exato que `ciborium` retorna para byte-strings. Pode ser `&[u8]`, `Vec<u8>`, ou tipo wrapper. Adaptar conforme necessário.

### B.5 — Testes

1. **Unit `entities/bytes.rs`** (4-5 tests):
   - `bytes_new` — construção com Vec<u8>.
   - `bytes_len` — `len()` correto.
   - `bytes_empty` — `is_empty()` para vec vazio.
   - `bytes_equality` — PartialEq por conteúdo.
   - `bytes_clone` — clone deep copy.

2. **Unit `entities/value.rs`** (3-4 tests):
   - `value_bytes_variant` — discriminação `Value::Bytes`.
   - `value_bytes_repr` — `"bytes(4)"` para 4 bytes.
   - `value_bytes_cast_identity` — `Bytes → Bytes`.
   - `value_bytes_cast_from_str` — `Str("hi") → Bytes([0x68, 0x69])`.
   - `value_bytes_partial_eq` — equality com outro Bytes.

3. **Unit `stdlib/loading.rs`** (4-5 tests):
   - `read_texto_utf8` — `read("texto.txt")` com conteúdo UTF-8 → `Value::Str`.
   - `read_binario` — `read("logo.png")` com bytes não-UTF8 → `Value::Bytes`.
   - `read_vazio` — arquivo vazio → `Value::Str("")` (empty string é UTF-8 válido).
   - `read_utf8_bom` — UTF-8 com BOM → `Value::Str` (BOM é UTF-8 válido, embora estranho).
   - `read_cbor_bytes` — `cbor` com byte-strings → `Value::Bytes`.

4. **E2E / integration** (2-3 tests):
   - `read_pipeline_texto` — parse + eval + `read("texto.txt")` → Str.
   - `read_pipeline_binario` — parse + eval + `read("logo.png")` → Bytes.
   - `cbor_bytes_pipeline` — `#cbor("dados.cbor")` com byte-strings → Bytes no output.

### B.6 — Linhagem

- `@prompt` aponta para `bytes.md` + extensão `read.md` (P387).
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P387 (read texto), P395 (portão ADR-0017), DEBT-62 (fechamento), ADR-0107 (paridade linguagem), ADR-0054 (graded promotion).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `EcoVec<u8>` — refino futuro XS (documentado no L0).
- **Não** implementar operações em Bytes (len, at, slice, concat) — são stdlib funcs futuras (S cada).
- **Não** implementar encoding detection sofisticado (BOM, ISO-8859-1, etc.) — scope-out ADR-0054 graded; heurística UTF-8 é suficiente.
- **Não** implementar `Value::Decimal`/`Duration`/`Version` — são P399-P401.
- **Não** adicionar `Bytes` como `Paint`/`Fill`/`Style` — não é visual.
- **Não** tocar em export PDF — Bytes não emite direto.
- **Não** quebrar invariantes de camada (L1 puro para tipo, L3 para read).

---

## 6. Critérios de aceitação

1. `Value::Bytes(Bytes)` compila e participa de `match` exhaustivo em eval.
2. `read("texto.txt")` com UTF-8 retorna `Value::Str`.
3. `read("logo.png")` com binário retorna `Value::Bytes`.
4. `cbor` com byte-strings retorna `Value::Bytes` (promove graded P387).
5. Zero tipo novo além de `Bytes`; zero I/O novo (reusa P387 `read_bytes`).
6. Testes verdes (≥15 unit + 2-3 integration); lint zero; hashes propagados.
7. Inventário 148: `Value::Bytes` transita `ausente` → `implementado`; `read` transita `implementado⁺` → `implementado` (binário real); DEBT-62 **fechado**.
8. L0 salvo e hashado antes do código (protocolo de nucleação).
9. `cbor` reclassificado conforme necessário (graded → implementado se byte-strings era único graded).

---

## 7. O que pode sair errado

- **`String::from_utf8` consome o Vec, precisando de clone.** Mitigação: clone é aceitável; refino futuro usa `from_utf8` + `into_bytes` para reusa.
- **`ciborium` retorna tipo inesperado para byte-strings.** Mitigação: inspecionar tipo real em `decode_cbor.rs`; adaptar conversão.
- **Testes de `read` binário precisam de arquivo binário real no filesystem.** Mitigação: criar arquivo temporário em teste (Vec<u8> não-UTF8 escrito em disco) ou mock `read_bytes`.
- **BOM UTF-8 (0xEF 0xBB 0xBF) no início de arquivo texto.** Mitigação: `String::from_utf8` aceita BOM como válido; BOM será parte da string (paridade vanilla — vanilla preserva BOM?).
- **Tentação de já fazer `Value::Decimal` junto.** Mitigação: um passo de cada vez; este é S-M, não M+.

---

## 8. Referências

- P387 — `read` modo texto (graded binário).
- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- DEBT-62 — read binário deferred.
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica (heurística UTF-8).
- ADR-0054 — graded scope-out (encoding detection sofisticado).

---

## 9. Nota sobre o Tekt

Este passo é o **fecho de débito técnico** — DEBT-62 era a última trava real do cluster `loading` (P387). Após P398:
- `read` é **completo** (texto + binário).
- `cbor` é **completo** (sem graded).
- O portão ADR-0017 permanece aberto para os tipos S restantes (Decimal, Duration, Version).

Registar o tempo de ciclo como baseline para **passos de ativação de consumer** (tipo já modelado + consumer L3 ativado) vs **passos de modelagem pura** (P395 — só tipo, sem consumer).
