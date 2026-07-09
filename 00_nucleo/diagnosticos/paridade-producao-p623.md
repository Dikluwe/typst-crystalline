# Relatório de Paridade — Passo 623

**Data:** 2026-07-09  
**Commit de implementação:** `ece949251`  
**Hash do L0 `rules/layout.md`:** `fbed936a`  
**Fonte:** Noto Sans Devanagari-Regular.ttf (`/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf`)  
**Tamanho de teste:** 40 pt

---

## 1. Objetivo

Verificar se a decisão de P622 — não incluir `Devanagari` em `needs_shaped_width` —
continua válida para texto denso em consoantes conjuntas, ou se a suposição
"provavelmente as ligaduras não reduzem a largura da mesma forma" falha quando
o input tem muitas conjuntas.

---

## 2. Documento denso em conjuntas

Input (`/tmp/p623-conjuntas.typ`):

```typst
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 40pt)
धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय
```

Este verso contém conjuntas complexas: `र्म`, `क्ष`, `र्व`, `ण्ड`, `ञ्ज`.

### 2.1 Páginas

| Implementação | Páginas |
|---------------|---------|
| Cristalino (antes da correção) | 1 |
| Vanilla | 1 |

Ambos produzem 1 página, mas isso não decide a questão: o texto completo é
longo demais para caber numa linha A4 mesmo com shaping aplicado.

### 2.2 Medição directa de largura (fontTools + hb-shape)

Método: somar advances individuais de cada codepoint (`fontTools`) vs largura
produzida por `hb-shape` com formas contextuais aplicadas.

| Palavra | Não-shaped (pt) | Shaped (pt) | Redução (pt) | Redução (%) |
|---------|-----------------|-------------|--------------|-------------|
| `धर्मक्षेत्रे` | 157.64 | 99.08 | 58.56 | 37.1% |
| `कुरुक्षेत्रे` | 139.60 | 103.56 | 36.04 | 25.8% |
| `समवेता` | 106.36 | 106.36 | 0.00 | 0.0% |
| `युयुत्सवः` | 129.36 | 119.36 | 10.00 | 7.7% |
| `मामकाः` | 109.92 | 109.92 | 0.00 | 0.0% |
| `पाण्डवाश्चैव` | 192.48 | 156.04 | 36.44 | 18.9% |
| `किमकुर्वत` | 156.64 | 140.28 | 16.36 | 10.4% |
| `सञ्जय` | 109.60 | 100.16 | 9.44 | 8.6% |

**Total palavras:**

- Não-shaped: **1101.60 pt**
- Shaped: **934.76 pt**
- Redução total: **166.84 pt (15.1%)**

A redução média é significativa; palavras individuais chegam a 37% mais
estreitas com shaping.

---

## 3. Teste de fronteira

Para forçar uma decisão de quebra sensível à largura, usou-se um documento
curto com largura de página ajustada para que o texto **shaped** caiba numa
linha e o texto **não-shaped** não caiba.

Input (`/tmp/p623-fronteira.typ`):

```typst
#set page(width: 240pt, margin: 10pt)
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 40pt)
धर्मक्षेत्रे कुरुक्षेत्रे
```

Larguras:

- Não-shaped: `157.64 + 10.40 (espaço) + 139.60 = 307.64 pt`
- Shaped: `99.08 + 10.40 + 103.56 = 213.04 pt`
- Largura útil: `240 - 2×10 = 220 pt`

### 3.1 Antes da correção

| Implementação | Linhas | Observação |
|---------------|--------|------------|
| Cristalino | 2 | Quebra prematura: `धर्मक्षेत्रे` / `कु-` `रुक्षेत्रे` |
| Vanilla | 1 | `धर्मक्षेत्रे कुरुक्षेत्रे` numa linha (213.04 pt) |

### 3.2 Após a correção

| Implementação | Linhas | Largura da linha |
|---------------|--------|------------------|
| Cristalino | 1 | 204.20 pt |
| Vanilla | 1 | 213.04 pt |

O cristalino passou a tomar a mesma decisão de quebra que o vanilla para este
input de fronteira.

---

## 4. Decisão

**Adicionar `Script::Devanagari` a `needs_shaped_width`.**

A suposição de P622 foi refutada por medição: texto devanágari denso em
conjuntas reduz de largura de forma significativa quando o shaping é aplicado,
e essa diferença é suficiente para alterar a decisão de quebra de linha em
documentos realistas.

---

## 5. Implementação

Ficheiro alterado: `01_core/src/rules/layout/metrics.rs:150`

```rust
pub fn needs_shaped_width(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c.script(),
            Script::Arabic
                | Script::Syriac
                | Script::Mongolian
                | Script::Nko
                | Script::Mandaic
                | Script::Devanagari
        )
    })
}
```

Foram adicionados testes unitários no mesmo ficheiro:

- `devanagari_precisa_shaped_width`
- `latim_nao_precisa_shaped_width`
- `arabic_precisa_shaped_width`

O Prompt L0 `00_nucleo/prompts/rules/layout.md` foi actualizado para documentar
`metrics.rs` e a lista de scripts contextuais, com hash `fbed936a`.

---

## 6. Validação

```bash
cargo build --release --workspace   # ok
cargo test --workspace              # ok — todos os testes passam
crystalline-lint .                  # ✓ No violations found
```

---

## 7. Notas e limitações

- O teste com o documento completo ainda mostra diferenças visuais menores
  entre cristalino e vanilla (hifenização, posicionamento de visarga,
  `pdftotext` extrai algumas palavras de forma diferente). Estas diferenças
  estão fora do escopo de P623, que se limitou à decisão de quebra de linha
  via `needs_shaped_width`.
- A correção não altera o caminho de renderização final; apenas faz com que o
  Layouter use `advance_shaped` para texto devanágari, ao mesmo nível de
  precisão já usado para árabe desde P591.
