# Paridade de Produção — Passo 559

**Data:** 2026-07-04  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino antes de P558: `/tmp/typst-crystalline-p558-antes/target/release/typst` (commit `fc6695f11`, P557)
- Cristalino depois de P558: `./target/release/typst` (commit `1cbb37767`, P558)
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `cargo test`, `pdfinfo`, `pdftotext`, `pdftoppm`, `diff`

---

## 1. Objetivo

P555 estabeleceu a regra de bateria completa após mudança de fonte por defeito.
P558 foi a terceira mudança consecutiva de fonte (Helvetica → FreeSerif →
Liberation Serif) e introduziu a exclusão de mark glyphs (`x_advance == 0`) em
`collect_shaped_glyph_mappings`. Este passo executa as duas verificações que a
regra exigia e que P558 não tinha feito:

1. Bateria completa (não só `cargo test --workspace`) depois de Liberation Serif.
2. Confirmação de que a exclusão de mark glyphs não destrói conteúdo semântico
   real em texto árabe/hebraico, onde diacríticos com `x_advance == 0` são
   informação, não decoração.

---

## 2. Parte 1 — Bateria completa depois de Liberation Serif

### 2.1 `cargo test` no `lab/parity`

```bash
cd lab/parity && cargo test
```

Resultado: **99 testes passaram, 0 falharam**.

- `consolidado_p206d`: 11 passed
- `eval_parity`: 1 passed
- `layout_parity`: 1 passed
- `parse_parity`: 50 passed
- `structural_parity`: 34 passed
- `vanilla_cli_smoke`: 2 passed

### 2.2 Bateria de corpus com o binário release

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ \
         lab/parity/corpus/p520/*.typ lab/parity/corpus/p523/*.typ \
         lab/parity/corpus/p538i/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && \
    echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Resultado: **42 ficheiros compilados, 0 falhas**.

### 2.3 Comparação de páginas/palavras (antes vs. depois de P558)

Método idêntico ao de P556: 86 ficheiros do corpus sem `#set text(font:)`
explicitamente.

| Métrica | Valor |
|---|---|
| Documentos medidos | 86 |
| Documentos que mudaram de páginas | **0** |
| Documentos que mudaram de palavras extraídas | **3** |

Documentos com variação de palavras:

| Documento | Palavras antes (P557) | Palavras depois (P558) | Observação |
|---|---|---|---|
| `p500/test-outline-advanced.typ` | 14 | 13 | "Secç o" → "Secção" |
| `visual/figure-ref.typ` | 34 | 33 | "Referencias :" → "Referências:" |
| `visual/outline-toc.typ` | 35 | 31 | Acentos trocados corrigidos (`Índicé` → `Índice`, `Résultãdos` → `Resultados`, etc.) |

Todas as variações são **melhorias** resultantes da correção de acentos de P558
(mudança para `Liberation Serif` + ignorar mark glyphs no mapeamento do
subsetter). Nenhuma indica perda de conteúdo.

---

## 3. Parte 2 — Corpus RTL depois da exclusão de mark glyphs

### 3.1 Corpus básico de P488

Documentos compilados sem erro nos dois estados (antes e depois de P558):

- `lab/parity/corpus/rtl/arabic_basic.typ`
- `lab/parity/corpus/rtl/hebrew_basic.typ`

Contagem de palavras extraídas (`pdftotext`):

| Documento | Antes (P557) | Depois (P558) |
|---|---|---|
| `arabic_basic.typ` | 16 | 16 |
| `hebrew_basic.typ` | 13 | 13 |

O hebrew não apresenta diferença na extração. O árabe apresenta diferença na
**formatação da extração** (menos fragmentação em linhas no estado depois), mas
mantém o mesmo número de palavras e o mesmo conteúdo semântico.

### 3.2 Teste explícito com diacríticos árabes (tashkeel)

Documento de teste:

```typst
#set text(lang: "ar", size: 72pt)
#align(center + horizon)[مُحَمَّد]
```

Extração `pdftotext`:

| Estado | Resultado |
|---|---|
| Antes (P557) | `ُم َح َّمد` |
| Depois (P558) | `ُم َح َّمد` |
| Vanilla 0.15.0 | `َّمد` / `ُم` / `َح َّم` / `ُم َح` |

A extração do cristalino é idêntica antes e depois de P558: os diacríticos
(fatha, damma, shadda) continuam presentes. A separação entre base e marcas é um
artefato do `pdftotext`, não uma perda de conteúdo.

### 3.3 Verificação visual

Tentativa de renderização das páginas geradas com `pdftoppm`/`mutool draw`:

- Vanilla 0.15.0: renderiza corretamente; os diacríticos são visíveis.
- Cristalino (antes e depois de P558): ambos produzem uma imagem em branco.
  A ferramenta de renderização reporta `Couldn't create a font for
  'AAAAAA+CrystallineFont'`.

Conclusão: o problema de renderização visual de RTL é **pré-existente a P558**.
Não foi introduzido nem agravado pela exclusão de mark glyphs.

---

## 4. Decisão

1. **Não há regressão de paginação** com a mudança para `Liberation Serif`.
2. **Não há perda de conteúdo semântico** no corpus RTL após a exclusão de mark
   glyphs.
3. **Não é necessário refinar a condição de exclusão** de `x_advance == 0`: o
   teste de tashkeel confirma que os diacríticos árabes não desaparecem.
4. O problema de renderização visual de RTL (imagem em branco, fonte embutida
   não carregável pelos renderizadores testados) é um achado fora do âmbito
   deste passo e pré-existente a P558. Fica registado para investigação futura.

---

## 5. Validação

```bash
cargo test --workspace
# passa
cd lab/parity && cargo test
# 99 passed, 0 failed
crystalline-lint .
# ✅ 0 violations
```

A validação final do linter foi executada e confirma zero violações.

---

## 6. Conclusão

- A bateria completa depois de `Liberation Serif` passou sem falhas.
- A comparação de páginas/palavras mostra **0 mudanças de paginação** e 3
  melhorias de extração causadas pela correção de acentos de P558.
- O corpus RTL de P488 continua a compilar sem panic e sem perda de conteúdo.
- O teste de tashkeel confirma que diacríticos árabes (`x_advance == 0`) não
  são removidos pela exclusão de mark glyphs.
- Não há ação corretiva a tomar relativamente a P558.

---

## 7. Critério de fecho do passo

- [x] Parte 1: bateria completa corrida; comparação de páginas/palavras feita.
- [x] Parte 2: corpus RTL e teste de tashkeel confirmados.
- [x] Nenhuma regressão RTL detetada; decisão registada.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p559.md`.
- [x] Inventário não requer alteração (nenhum item aberto afetado).

---

## 8. Achado fora de âmbito — renderização visual RTL

O PDF cristalino para texto árabe/hebraico não é renderizável por `pdftoppm`/
`mutool draw` devido a falha no carregamento da fonte embutida
(`AAAAAA+CrystallineFont`). O vanilla 0.15.0 renderiza o mesmo documento
corretamente. Este problema é **pré-existente a P558** e não está relacionado
com a exclusão de mark glyphs. Recomenda-se um passo dedicado para investigar a
embebedação/subsetting de fontes em contexto RTL/bidi.
