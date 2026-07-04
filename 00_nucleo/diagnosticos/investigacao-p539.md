# Investigação P539 — CSL, diferenças de texto, `lorem()`

**Data:** 2026-07-03  
**Passo:** 539  
**Tipo:** Diagnóstico sem código de produção  
**Dependências:** P533, P538i, P538g

---

## Parte 1 — Formatação CSL da bibliografia

### Teste executado

Ficheiro `/tmp/refs-full.bib` com artigo e livro; quatro estilos testados:
`ieee`, `apa`, `chicago-author-date`, `mla`. Cada documento compilado com
`/home/dikluwe/Documentos/Antigravity/typst-crystalline/target/release/typst` (cristalino)
e `/usr/local/bin/typst` (vanilla).

### Resultados

| Estilo | Cristalino (texto extraído) | Vanilla (texto extraído) | Classificação |
|--------|------------------------------|--------------------------|---------------|
| **ieee** | `[1] J. Silva Maria and Santos , “ A Comprehensive Study of Modern T ypography,” Journal of Design Research , vol. 12 , p. 45––67 , 2023 . [2] A. Costa , The Art of Document Preparation . 2021 . Ver [ 1 ] e [ 2 ] .` | `[1] M. Silva and J. Santos, “A Comprehensive Study of Modern Typography,” Journal of Design Research, vol. 12, pp. 45–67, 2023. [2] A. Costa, The Art of Document Preparation. Academic Press, 2021. Ver [1] e [2].` | **Bug** |
| **apa** | Texto desordenado; autores fragmentados (`Maria and Santos`, `J.,`); publisher ausente na 2ª entrada; citações incompletas. | Texto ordenado alfabeticamente, formatado correctamente, com publisher. | **Bug** |
| **chicago** | `JoÃ£o` em vez de `João`; `Maria and Santos` como autor; `45––67` (double dash); citações truncadas. | `João`; `Maria, and João Santos`; `45–67`; citações correctas. | **Bug** |
| **mla** | `JoÃ£o`; `p. 45––67`; citações `( Silva )` sem ano; publisher ausente. | `João`; `pp. 45–67`; citações `(Silva and Santos)`; publisher presente. | **Bug** |

### Categorias de problema observadas

1. **Parsing de nomes de autor**: cristalino troca ordem e junta nomes
   (`J. Silva Maria and Santos` em vez de `M. Silva and J. Santos`).
2. **Encoding Unicode**: `João` sai como `JoÃ£o` no cristalino.
3. **Hifenização/double dash**: `45––67` em vez de `45–67`.
4. **Espaçamento extra**: espaços antes de vírgulas e pontos.
5. **Dados perdidos**: publisher ausente em algumas entradas; citações incompletas.
6. **Ordenação**: entradas não aparecem ordenadas alfabeticamente.

### Decisão

**Bug a corrigir**, mas fora do scope de P539. Recomenda-se criar passo
próprio com sonda da biblioteca CSL usada (`01_core/src/rules/eval/bibliography.rs`,
`03_infra/src/bib/`).

---

## Parte 2 — As 20 diferenças de texto de P538i

### Levantamento

Para obter o detalhe, o teste `structural_parity.rs` foi temporariamente
alterado para imprimir os textos normalizados cristalino e vanilla lado a
lado. Após coleta, o debug foi removido.

### Lista individual das 20 diferenças

| # | Ficheiro | Cristalino | Vanilla | Categoria | Classificação |
|---|----------|------------|---------|-----------|---------------|
| 1 | `markup/spaces.typ` | `a b c` | `abc` | Espaçamento | Aceitável — normalização de espaços |
| 2 | `math/block.typ` | `n ∑i i =0` | `𝑛 ∑𝑖 𝑖=0` | Math — fonte de variáveis | Aceitável — variáveis não itálicas |
| 3 | `math/simple.typ` | `x2` | `𝑥2` | Math — fonte de variáveis | Aceitável |
| 4 | `math/style_cal_frak.typ` | `ℒ 𝔤` | `ℒ\u{fe00} 𝔤` | Math — variant selector | Aceitável |
| 5 | `math/style_compose_outer_wins.typ` | `𝕩 x` | `𝓍\u{fe00} 𝑥` | Math — fonte/variante | Aceitável |
| 6 | `math/style_size.typ` | `x x x` | `𝑥 𝑥 𝑥` | Math — fonte de variáveis | Aceitável |
| 7 | `p538i/for-basic.typ` | `•um •dois •três` | `• um • dois • três` | Lista — espaço após bullet | Bug menor |
| 8 | `p538i/for-with-counter.typ` | `Item 1 : letra` | `Item 1: letra` | Pontuação — espaço antes de `:` | Bug menor |
| 9 | `visual/cite-bibliography.typ` | Formatação CSL própria truncada | Formatação IEEE/APA-like completa | Bibliografia | **Bug grande** |
| 10 | `visual/counter-heading.typ` | `1. Capítulo Um T exto` | `1 Capítulo Um Texto` | Heading + hifenização | Bug |
| 11 | `visual/equation-ref.typ` | `E=mc 2`, refs truncadas | `𝐸=𝑚𝑐²`, refs completas | Math + referências | **Bug** |
| 12 | `visual/figure-ref.typ` | `Imagemalfa`, `Figura` | `Imagem alfa`, `Figure` | Figuras + i18n | **Bug** |
| 13 | `visual/math-basico.typ` | `x2+y2=z 2` | `𝑥²+𝑦²=𝑧²` | Math | Aceitável |
| 14 | `visual/outline-toc.typ` | Sem leaders/dots | Com leaders/dots | Outline | **Bug grande** |
| 15 | `visual/query-metadata.typ` | `T exto` | `Texto` | Hifenização | Bug |
| 16 | `visual/set-text-bold.typ` | `T exto comweight 700` | `Texto com weight 700` | Hifenização + junção | Bug |
| 17 | `visual/set-text-fill.typ` | `T exto com fill colorido` | `Texto com fill colorido` | Hifenização | Bug |
| 18 | `visual/set-text-size.typ` | `T exto a 16pt` | `Texto a 16pt` | Hifenização | Bug |
| 19 | `visual/set-text-tracking.typ` | `T exto com tracking aumentado` | `Texto com tracking aumentado` | Hifenização | Bug |
| 20 | `visual/show-strong.typ` | `Antes Forte : meio` | `Antes Forte: meio` | Pontuação — espaço antes de `:` | Bug menor |

### Análise de causa comum

- **Hifenização excessiva**: ocorrências 10, 15–19 são todas `T exto` em vez
de `Texto`. Sintoma comum: o justificador/quebrador de linhas está a inserir
uma quebra/hífen no meio de palavras curtas que cabem facilmente na linha.
- **Espaço antes de pontuação**: ocorrências 8 e 20 (` : ` em vez de `:`).
- **Math**: ocorrências 2–6 e 13 são diferenças de fonte/estilo, não bugs
funcionais (aceitáveis segundo ADR-0107, paridade ao nível da linguagem).
- **Bibliografia/Outline**: ocorrências 9, 14 são bugs funcionais já conhecidos
ou relacionados com Parte 1.

### Decisão

- **Math (6 ocorrências)**: aceitável — representação tipográfica diverge de
  propósito; a morfologia (equação presente) é correcta.
- **Hifenização excessiva + junções + pontuação (11 ocorrências)**: **bug** no
  layout/shaping/justificação. Causa provavelmente comum no tratamento de
  glue/kern entre palavras.
- **Bibliografia/Outline (2 ocorrências)**: **bug** funcional, separado.

---

## Parte 3 — Contagem de palavras do `lorem()`

### Definição do cristalino

`01_core/src/rules/stdlib/text.rs:588`:

```rust
pub fn native_lorem(...) -> SourceResult<Value> {
    // ...
    for i in 0..n {
        words.push(LOREM_WORDS[i % len]);
    }
    Ok(Value::Str(words.join(" ").into()))
}
```

A função gera exactamente `n` palavras, repetindo o vocabulário fixo de
69 palavras. A definição é "gerar `n` palavras".

### Testes com vários valores de `n`

| `n` | Cristalino (pdftotext) | Vanilla (pdftotext) |
|-----|------------------------|---------------------|
| 10 | 10 | 10 |
| 50 | 49 | 50 |
| 100 | 99 | 100 |
| 200 | 197 | 200 |
| 500 | 493 | 500 |
| 1200 | 1183 | 1200 |

### Causa da diferença

O `lorem()` do cristalino gera exactamente `n` palavras, mas o layout/shaping
junta pares de palavras. Exemplo concreto para `n=50`:

- Gerado: `...commodo consequat Duis...`
- Extraído do PDF: `...commodoconsequat Duis...`

A junção ocorre sistematicamente em `commodo consequat`, que aparece a cada
69 palavras no vocabulário cíclico. 1200 / 69 ≈ 17,4, o que explica as 17
palavras em falta.

### Decisão

**Não é bug no `lorem()`** — a função cumpre a especificação (gera `n`
palavras). A diferença é efeito do bug de layout/shaping (junção de palavras
vizinhas) identificado na Parte 2. Quando esse bug for corrigido, a contagem
de `pdftotext` deverá aproximar-se de `n`.

---

## Tabela final

| Item | Resultado | Classificação |
|------|-----------|---------------|
| CSL — 4 estilos testados | Bugs em todos os estilos: parsing de nomes, encoding, espaçamento, dados perdidos, ordenação | **Bug a corrigir** (passo próprio) |
| 20 diferenças de texto | 6 de math (aceitáveis); 11 de hifenização/junção/pontuação (bug comum de layout); 2 de bibliografia/outline (bug funcional) | **Bug comum de layout/shaping** + diferenças aceitáveis |
| `lorem()` — causa da diferença | `lorem(n)` gera exactamente `n` palavras; a perda vem do layout que junta palavras | **Efeito do bug de layout**, não bug de `lorem()` |

---

## Decisão de prosseguimento

1. **Bibliografia CSL**: novo passo de correção com sonda da pipeline CSL.
2. **Hifenização/junção de palavras**: novo passo de correção no
   layout/shaping/justificação. A causa comum (glue entre palavras) sugere
   um único ponto de falha.
3. **Math/itálicos**: aceitar como divergência tipográfica documentada.
4. **Outline leaders**: pode ser resolvido no mesmo passo de layout ou
   separadamente, dependendo da causa.

---

## Follow-up — Passo 557 (`visual/outline-toc.typ`)

A sonda P557 confirmou a causa da disparidade de palavras extraídas (524 no
vanilla vs 35 no cristalino): o vanilla gera **leaders/dots e números de
página** no `#outline()`, enquanto o cristalino emite apenas o título e as
entradas do índice. O conteúdo semântico (5 headings + 5 frases) está presente
em ambas as versões.

- O item **"Sem leaders/dots"** permanece aberto.
- A diferença bruta de `pdftotext | wc -w` não deve ser usada como métrica de
  paridade para documentos com outline até que o gap seja fechado.
- Detalhes completos em `00_nucleo/diagnosticos/paridade-producao-p557.md`.
