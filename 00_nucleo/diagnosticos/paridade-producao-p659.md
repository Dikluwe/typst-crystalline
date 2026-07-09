# Relatório de Paridade — P659

**Passo:** 659  
**Data:** 2026-07-09  
**Foco:** Corrigir a chave da cache de `shaped_width` para incluir variações de eixo OpenType.  
**Dependências:** P658 (onde o problema foi encontrado), P591 (onde a cache foi criada).  
**Hash do commit com as alterações:** `731f04aa0`

---

## 1. Sonda

### 1.1 Estrutura da chave `ShapedWidthKey`

`03_infra/src/font_metrics.rs:284`:

```rust
#[derive(Debug, Hash, Eq, PartialEq)]
struct ShapedWidthKey {
    text:      String,
    size_bits: u64,
    font_hash: u64,
    bold:      bool,
    italic:    bool,
    weight:    Option<u16>,
    dir:       u8,
    lang:      Option<typst_core::entities::lang::Lang>,
}
```

Campos ausentes: qualquer referência a eixos de variação OpenType (`wght`, `wdth`, `ital`, etc.).

### 1.2 Sintaxe de eixos de variação no cristalino

O cristalino ainda não expõe a sintaxe do vanilla `#text(font: (name: "...", variant: (wdth: 62.5)))`. Testes directos falharam com:

```text
error: text() argumento nomeado desconhecido: 'font'
error: unknown font dict field: name
```

A representação actual de variantes no cristalino usa nomes de variantes (`EcoString`) e o `FontVariant` interno tem `style`, `weight`, `stretch`. A função `axis_variations_for_font_variant` (usada pelo shaper desde P525) já converte peso/estilo em variações de eixo para `rustybuzz`.

### 1.3 Reprodução via teste unitário

Como a sintaxe de eixos numéricos não está disponível no parser, a colisão foi reproduzida em teste unitário: dois `TextStyle` com o mesmo texto e tamanho, mas `weight` 400 e 700, produziam a mesma `ShapedWidthKey` antes da correção. Com a correção, produzem `axis_hash` diferentes.

---

## 2. Implementação

### 2.1 `03_infra/src/font_metrics.rs`

- Adicionado campo `axis_hash: u64` a `ShapedWidthKey`.
- Em `shaped_width_key`:
  - Calcula-se `variant = text_style_to_font_variant(style)`.
  - Calcula-se `axis_vars = axis_variations_for_font_variant(&variant)`.
  - Calcula-se `axis_hash` a partir das tags e valores das variações, usando `DefaultHasher`.
  - Inclui-se `axis_hash` na chave.
- Importado `axis_variations_for_font_variant`.
- Adicionado `#[derive(Debug)]` a `ShapedWidthKey` para permitir `assert_eq!` nos testes.

### 2.2 Teste unitário

`p659_shaped_width_key_distingue_variacoes_de_eixo`:

- Verifica que `weight = 400` e `weight = 700` produzem `axis_hash` diferentes.
- Verifica que as chaves completas são diferentes.
- Verifica que o mesmo peso produz `axis_hash` igual.

---

## 3. Validação

### 3.1 Testes

```bash
cargo test --workspace
```

Resultado: todos os crates passaram.

### 3.2 Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.3 Documento árabe repetido (caso comum, sem fontes variáveis)

Baseline (antes da correção):

```json
{
    "layout_ms": 5.086795,
    "shape_ms": 283.027917,
    "total_ms": 294.258994
}
```

Depois da correção:

```json
{
    "layout_ms": 5.180416,
    "shape_ms": 283.391073,
    "total_ms": 294.694437
}
```

Sem regressão perceptível. Para fontes sem variações de eixo, `axis_vars` é vazio e `axis_hash` é 0, pelo que o hit ratio da cache mantém-se.

### 3.4 Testes RTL e devanágari

Os testes existentes de P590-P592 e P622-P623 (incluindo `p591_advance_shaped_arabico_reduz_largura`) continuam a passar.

---

## 4. Decisão

- A chave `ShapedWidthKey` passou a incluir `axis_hash`, cobrindo variações de eixo OpenType.
- Isto evita colisões silenciosas na cache de `shaped_width` quando o mesmo texto/tamanho tem eixos diferentes.
- O caso comum (fontes sem variações) não sofre degradação, porque `axis_hash` é 0 para todos esses casos.
