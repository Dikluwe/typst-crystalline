# Paridade de produção — P614: Sonda de escrita vertical CJK

**Data:** 2026-07-08  
**Tipo:** Sonda pura (sem código).  
**Materialization:** `00_nucleo/materialization/typst-passo-614.md`  

---

## 1. Descoberta principal

**O vanilla 0.15.0 não suporta escrita vertical CJK.**

A tentativa de usar `#set text(lang: "ja", dir: ttb)` no vanilla 0.15.0 produz o erro:

```text
error: text direction must be horizontal
```

Isto altera o enquadramento de P614: não se trata de uma disparidade de implementação entre cristalino e vanilla, porque **ambos não implementam escrita vertical para texto** nesta versão. O trabalho, se avançar, é uma funcionalidade nova para ambos, não uma correção de paridade.

---

## 2. Sondas realizadas

### 2.1 Japonês simples sem direcção explícita

```typst
#set text(lang: "ja")
これは縦書きのテストです。日本語の文章は上から下に読みます。
```

| Ferramenta | Resultado |
|------------|-----------|
| Vanilla 0.15.0 | Renderiza horizontalmente, da esquerda para a direita. |
| Cristalino | Renderiza horizontalmente, da esquerda para a direita. |

Ambos ignoram a implicação tipográfica de `lang: "ja"` e não activam escrita vertical automaticamente.

### 2.2 Direcção explícita `dir: ttb`

```typst
#set text(lang: "ja", dir: ttb)
これは縦書きのテストです。
```

| Ferramenta | Resultado |
|------------|-----------|
| Vanilla 0.15.0 | **Erro:** `text direction must be horizontal`. |
| Cristalino | Aceita silenciosamente, mas renderiza horizontalmente. |

O cristalino tem aqui um bug de validação: `dir: ttb` não é rejeitado nem aplicado. O vanilla rejeita explicitamente.

### 2.3 Texto misto (latim/números) em japonês horizontal

```typst
#set text(lang: "ja")
これはテスト123です。ABC も含みます。
```

O vanilla renderiza tudo na mesma linha horizontal. Não é possível testar o comportamento vertical de latim/números porque o vanilla não suporta escrita vertical.

---

## 3. Estado do código cristalino

### 3.1 Leitura de métricas verticais da fonte

Pesquisa por `vmtx`, `vhea` e `VerticalFontMetric` em `01_core/src` e `03_infra/src`:

- **Não existe leitura das tabelas OpenType `vmtx`/`vhea`.**
- O que existe com o nome "vertical metrics" refere-se a ascender/descender horizontais para cálculo de line height (`vertical_metrics` em `03_infra/src/font_metrics.rs`).
- O shaper e o export PDF trabalham exclusivamente com métricas horizontais (`hmtx`, advance horizontal, kerning horizontal).

### 3.2 Direcções conhecidas pelo cristalino

O tipo `Dir` do cristalino conhece `LTR`, `RTL`, `TTB` e `BTT`, mas:
- `TTB`/`BTT` são usados em contextos de layout de bloco (por exemplo, `stack`), não de texto.
- O layout de texto (`layout_word`, `flush_line`, `cursor.rs`, etc.) assume sempre o eixo horizontal.
- Não há troca de eixo "linha" ↔ "quebra de linha" para texto vertical.

---

## 4. Mapeamento da cascata vertical

Usando a sequência horizontal já resolvida (P484–P592 / P593) como referência, o equivalente vertical exigiria reescrever cada nível:

| Nível | Horizontal (resolvido) | Vertical (ainda por fazer) |
|-------|------------------------|----------------------------|
| **Letra/glifo** | Largura do glifo vinda de `hmtx`; posicionamento X. | Altura do glifo vinda de `vmtx`/`vhea`; posicionamento Y. As fontes CJK que suportam vertical têm métricas verticais separadas. |
| **Palavra** | Largura da palavra, com forma de escrita aplicada (ligatures RTL). | CJK não tem "palavras" no sentido ocidental — cada carácter é uma unidade. Não há ligação de formas, mas o posicionamento e o kerning vertical existem em algumas fontes. |
| **Linha** | Quebra por largura disponível; linha = faixa horizontal. | Quebra por **altura** disponível; "linha" passa a ser uma coluna vertical. |
| **Parágrafo** | Alinhamento esquerda/direita; fluxo de cima para baixo. | Alinhamento topo/fundo; fluxo da **direita para a esquerda** entre colunas. A primeira coluna vertical começa no topo direito da página. |
| **Coluna/página** | Largura útil; colunas fluem de cima para baixo. | Altura útil; "colunas" verticais fluem da direita para a esquerda. |
| **Mistura latim/números** | Horizontal por defeito. | Em tipografia vertical CJK, latim e números podem: (a) manter-se horizontais dentro da coluna (tate-chū-yoko), (b) rodar 90°, ou (c) usar fonte vertical. O vanilla 0.15.0 não chega a este ponto. |

Conclusão do mapeamento: não é uma extensão do RTL. É uma reescrita do eixo de referência de todo o motor de composição de texto.

---

## 5. Estimativa de tamanho

A implementação completa de escrita vertical CJK no cristalino é **XL**, não L ou M. Divide-se naturalmente em passos menores:

1. **Métricas verticais da fonte (S–M)** — ler `vmtx`/`vhea`, expor altura de glifo e kerning vertical.
2. **Layout de texto vertical básico (L)** — trocar eixo de medida e quebra de "linha" (coluna vertical) para texto puro CJK, sem mistura latim.
3. **Fluxo de parágrafo e página vertical (M–L)** — alinhamento e colunas da direita para a esquerda.
4. **Mistura de scripts em vertical (L–XL)** — decidir e implementar tate-chū-yoko, rotação, ou fallback horizontal para latim/números.
5. **Export PDF vertical (M)** — posicionar e rotar glifos; ajustar `ToUnicode` e seleção de texto.
6. **Validação de paridade (M)** — quando o vanilla tiver escrita vertical; enquanto não, validar contra referências tipográficas standard.

Apenas os passos 1–2 já constituem um trabalho substancial, comparável à sequência RTL completa.

---

## 6. Decisão

Dado que o vanilla 0.15.0 **também não implementa** escrita vertical CJK, esta funcionalidade deixa de ser uma "disparidade a corrigir" e passa a ser:

- **Ausente em ambos** — funcionalidade nova para uma versão futura.
- **Não priorizada nesta conversa** — o esforço é XL e não há base de paridade para justificá-lo agora.
- **Registada como escopo futuro**, com o mapa de passos menores acima.

O bug imediato do cristalino (aceitar `dir: ttb` em texto sem rejeitar nem aplicar) pode ser tratado num passo pequeno separado, se desejado.

---

## 7. Ligações

- Materialization: `00_nucleo/materialization/typst-passo-614.md`
- Arquivos inspecionados:
  - `lab/typst-original/crates/typst-library/src/layout/dir.rs`
  - `lab/typst-original/crates/typst-library/src/text/mod.rs`
  - `03_infra/src/font_metrics.rs`
  - `01_core/src/rules/layout/text.rs` (referência indirecta)
