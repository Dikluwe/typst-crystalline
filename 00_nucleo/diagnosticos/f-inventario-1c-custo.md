# F — Inventário 1c: custo do estado atual

> Diagnóstico **read-only** ("o F" / StyleChain), Front 1c.
> Zero alterações de código. Cada número traz o comando exato que o produziu.
> Medir, não opinar. Sem otimização.
>
> Alvo: `01_core/src/entities/content.rs` (enum `Content`, 65 variantes
> migradas a "Modelo D"; 12 arms permanecem não-migrados por design/F-scope).
>
> Data de medição: 2026-06-12.

---

## 0. Métodos-hub localizados

Comando:

```bash
grep -nE "fn plain_text|fn is_empty|fn get_field|fn map_content|fn map_text|fn eq|impl PartialEq" 01_core/src/entities/content.rs
```

Resultado (apenas as 6 entradas-hub relevantes):

| Método            | Linha de início | Bloco lido            |
|-------------------|-----------------|------------------------|
| `fn is_empty`     | 1480            | 1480–1559              |
| `fn plain_text`   | 1562            | 1562–1691              |
| `impl PartialEq` / `fn eq` | 1694 / 1695 | 1695–1822        |
| `fn get_field`    | 1842            | 1842–1850              |
| `fn map_content`  | 1859            | 1859–2026              |
| `fn map_text`     | 2034            | 2038–2189              |

Os arms foram lidos integralmente (Read das linhas 1480–1692, 1709–1843,
1842–2061, 2061–2261).

---

## 1. Matriz das 12 arms remanescentes nos 6 matches

As 12 variantes não-migradas (por design/F-scope):
**7 primitivos** (`Sequence`, `MathSequence`, `Empty`, `Space`, `Text`,
`MathText`, `MathIdent`) + `Styled` + **4 `Set*`**
(`SetHeadingNumbering`, `SetEquationNumbering`, `SetPage`, `SetFigureNumbering`).

Legenda das células: número = linhas próprias (não-dispatch) que a variante
ocupa naquele match. Em branco = a variante **não tem arm próprio** naquele
match (cai num bloco genérico `_ =>` / terminal `=> self.clone()` agrupado —
ver notas). "term." = está incluída num bloco terminal agrupado (clona em
bloco; sem arm individual).

| Variante              | is_empty | plain_text | eq | get_field | map_content | map_text | **Σ próprias** |
|-----------------------|:--------:|:----------:|:--:|:---------:|:-----------:|:--------:|:-------------:|
| `Sequence`            | 1 (1483) | 1 (1567)   | 1 (1700) | — | 4 (1866–1870) | 4 (2044–2048) | **11** |
| `MathSequence`        | —        | 1 (1593)   | 1 (1710) | — | 4 (1885–1889) | term.    | **6** |
| `Empty`               | 1 (1482) | 1 (1564)   | 1 (1697) | — | term.        | term.    | **3** |
| `Space`               | — (`_`)  | 1 (1566)   | 1 (1699) | — | term.        | term.    | **2** |
| `Text`                | — (`_`)  | 1 (1565)   | 1 (1698) | — | term.        | 1 (2040) | **3** |
| `MathText`            | — (`_`)  | 1 (1595)   | 1 (1712) | — | term.        | term.    | **2** |
| `MathIdent`           | — (`_`)  | 1 (1594)   | 1 (1711) | — | term.        | term.    | **2** |
| `Styled`              | — (`_`)  | 1 (1657)   | 1 (1780) | — | 4 (2015–2018) | 4 (2184–2187) | **11** |
| `SetHeadingNumbering` | — (`_`)  | 1 (1613)   | 1 (1732) | — | term.        | term.    | **2** |
| `SetEquationNumbering`| — (`_`)  | 1 (1614)   | 1 (1733) | — | term.        | term.    | **2** |
| `SetPage`             | — (`_`)  | 1 (1654)   | 3 (1773–1775) | — | term.   | term.    | **4** |
| `SetFigureNumbering`  | — (`_`)  | 1 (1619)   | 1 (1740) | — | term.        | term.    | **2** |
| **Σ por match**       | **3**    | **12**     | **14** | **0** | **12** | **13** | **Σ=50** |

### Notas sobre blocos agrupados

- **`is_empty` (1480–1559):** só `Empty` (1482) e `Sequence` (1483) têm arm
  próprio entre as 12; as outras 10 caem no `_ => false` (1557). Contadas em
  branco (não "term.") porque o catch-all `_` não as nomeia.
- **`get_field` (1842–1850):** match minúsculo; só `Heading` (1846) e `Figure`
  (1847) têm arm; as 12 caem em `_ => None` (1848). **Zero** arms próprios.
- **`map_content` — bloco terminal `=> self.clone()` (1949–1984):** nomeia
  explicitamente, entre as 12: `Text`, `Space`, `Empty`, `MathIdent`,
  `MathText`, `SetHeadingNumbering`, `SetEquationNumbering`,
  `SetFigureNumbering`, `SetPage`. (`Sequence`, `MathSequence`, `Styled` têm
  arm dedicado fora do bloco.) Marcadas "term." — fazem parte de um bloco
  partilhado de ~36 linhas, não atribuível por variante.
- **`map_text` — bloco terminal `=> self.clone()` (2101–2151):** nomeia, entre
  as 12: `Empty`, `Space`, `MathIdent`, `MathText`, `MathSequence`,
  `SetHeadingNumbering`, `SetEquationNumbering`, `SetFigureNumbering`,
  `SetPage`. (`Text`, `Sequence`, `Styled` têm arm dedicado.) Marcadas "term.".

### Total de linhas remanescentes ("custo do estado misto" que o F herda)

**Σ de linhas próprias (não-dispatch) atribuíveis às 12 variantes = 50 linhas.**

Caveat de contagem: "term." conta 0 por variante porque a linha pertence a um
bloco `|`-agrupado partilhado e não é fatiável por variante. Se se contar a
*menção* da variante dentro do bloco terminal como 1 linha cada:
- `map_content`: +9 menções terminais → +9
- `map_text`: +9 menções terminais → +9
- (`is_empty` catch-all `_`: 0 menções nominais)

Logo, **limite superior alternativo = 50 + 18 = 68 linhas** se cada menção
nominal num bloco agrupado contar como 1. As métricas oficiais deste relatório
usam o número conservador **50** (linhas exclusivamente próprias) e reportam
**68** como teto.

---

## 2. Larguras refeitas hoje (preditor do custo de qualquer opção Fase-3)

Comando (executado para cada `<V>`):

```bash
for V in Sequence MathSequence Empty Space Text MathText MathIdent Styled \
         SetHeadingNumbering SetEquationNumbering SetPage SetFigureNumbering; do
  N=$(grep -rnE "Content::${V}\b|Self::${V}\b" 01_core/src/ | wc -l)
  printf "%-22s %s\n" "$V" "$N"
done
```

Resultado (re-verificado hoje):

| Variante              | Largura (grep `Content::V\|Self::V` em `01_core/src/`) | Aprox. prévio do prompt |
|-----------------------|:------:|:------:|
| `Sequence`            | **220** | (novo) |
| `MathSequence`        | **27**  | (novo) |
| `Empty`               | **164** | (novo) |
| `Space`               | **28**  | (novo) |
| `Text`                | **93**  | 93 ✓ |
| `MathText`            | **47**  | 47 ✓ |
| `MathIdent`           | **108** | 108 ✓ |
| `Styled`              | **82**  | 82 ✓ |
| `SetHeadingNumbering` | **66**  | 66 ✓ |
| `SetEquationNumbering`| **20**  | 20 ✓ |
| `SetPage`             | **13**  | 13 ✓ |
| `SetFigureNumbering`  | **9**   | 9 ✓  |
| **Σ (12 variantes)**  | **877** | — |

Todos os aprox. prévios fornecidos no prompt **conferem exatamente**.
Novos (não fornecidos): `Sequence`=220, `Empty`=164, `MathSequence`=27,
`Space`=28. **`Sequence` (220) e `Empty` (164) são as duas maiores larguras
do conjunto** — qualquer opção que reescreva essas duas paga o preço mais alto.

---

## 3. Propriedades de estilo distintas que o layouter consome

### 3.1 Definição canónica — `TextStyle`

Comando:

```bash
grep -rn "pub struct TextStyle" 01_core/src/
# → 01_core/src/entities/layout_types.rs:123
```

`TextStyle` (lido em `layout_types.rs:123–139`) declara **10 propriedades**:

| # | Propriedade     | Tipo                       | Linha (decl.) |
|---|-----------------|----------------------------|---------------|
| 1 | `bold`          | `bool`                     | 124 |
| 2 | `italic`        | `bool`                     | 125 |
| 3 | `size`          | `Pt`                       | 126 |
| 4 | `fill`          | `Option<Color>`            | 128 |
| 5 | `heading_level` | `Option<u8>`               | 130 |
| 6 | `weight`        | `Option<u16>`              | 134 |
| 7 | `tracking`      | `Option<Length>`           | 135 |
| 8 | `leading`       | `Option<Length>`           | 136 |
| 9 | `lang`          | `Option<Lang>`             | 137 |
| 10| `font`          | `Option<FontList>`         | 138 |

Espelhadas 1:1 pelo enum `Style` (delta) e pelos getters de `StyleChain`:

```bash
grep -nE "^\s+(Bold|Italic|Size|Fill|HeadingLevel|Weight|Tracking|Leading|Lang|Font)\(" 01_core/src/entities/style.rs
```
→ `Bold(bool)`(35), `Italic(bool)`(37), `Size(Pt)`(39), `Fill(Color)`(41),
`HeadingLevel(u8)`(43), `Lang(Lang)`(50), `Weight(u16)`(61),
`Tracking(Length)`(74), `Leading(Length)`(94), `Font(FontList)`(122) — **10**.

```bash
grep -nE "pub fn (fill|heading_level|bold|italic|size|weight|tracking|leading|lang|font)" 01_core/src/entities/style_chain.rs
```
→ getters em 196/210/222/227/232/241/251/261/271/281 — **10 getters**, um por
propriedade.

**Contagem de propriedades de estilo distintas = 10** (alinhamento perfeito
`TextStyle` ↔ `Style` ↔ `StyleChain`). Isto dimensiona qualquer opção PropMap.

### 3.2 Onde cada propriedade é *lida* no layouter (`file:line`)

Sítios de leitura não-teste. Comando-base por propriedade:

```bash
grep -rnE "self\.style\.<P>\b" 01_core/src/rules/layout/   # + self.chain.<P>, local style.<P>
```

Contagem de sítios de leitura não-teste por propriedade
(`self.style.<P>` ∪ `self.chain.<P>` ∪ `style.<P>` local, excluindo
`tests.rs`, `node_style.*` e comentários):

```bash
for P in size bold italic fill heading_level weight tracking leading lang font; do
  C=$(grep -rnE "self\.style\.$P\b|self\.chain\.$P\b|\bstyle\.$P\b" 01_core/src/rules/layout/ \
      | grep -v 'tests.rs' | grep -v 'node_style' | grep -v '//' | wc -l)
  printf "%-14s %s\n" "$P" "$C"
done
```

Resultado:

| Propriedade     | # sítios de leitura | Sítios `file:line` (não-teste)                         |
|-----------------|:-------------------:|--------------------------------------------------------|
| `size`          | 6 | equation.rs:62; equation.rs:72 (`style.size`); cursor.rs:29; cursor.rs:32; cursor.rs:41; mod.rs:589 |
| `bold`          | 0* | (só lida via merge `node_style.bold` mod.rs:587 + testes) |
| `italic`        | 0* | (só via merge `node_style.italic` mod.rs:588 + testes) |
| `fill`          | 1 | mod.rs:2124 (`self.style.fill`); + merge mod.rs:594    |
| `heading_level` | 0* | (só via merge mod.rs:595)                              |
| `weight`        | 0* | (só via merge mod.rs:598; consumer faux-bold em `faux_bold_stroke_pt`) |
| `tracking`      | 1 | cursor.rs:30 (`self.style.tracking`); + merge mod.rs:599 |
| `leading`       | 1 | cursor.rs:124 (`style.leading` local); + merge mod.rs:600 |
| `lang`          | 3 | cursor.rs:56 (`self.style.lang`); mod.rs:2202 (`&self.style.lang`); mod.rs:2227 (`self.chain.lang()`) |
| `font`          | 0* | (só via merge mod.rs:602)                              |

\* As 10 propriedades são **todas** consumidas no sítio de *merge*
`mod.rs:587–602` (cada `self.style.<P>.or(node_style.<P>)`), que constrói o
`TextStyle` propagado. As marcadas "0" não têm leitura *adicional* fora desse
merge (algumas, como `weight`, são consumidas indiretamente via
`TextStyle::faux_bold_stroke_pt` em `layout_types.rs:160`).

Sítio de merge canónico (constrói o `TextStyle` efetivo):

```bash
grep -rnoE "node_style\.(size|bold|italic|fill|heading_level|weight|tracking|leading|lang|font)\b" 01_core/src/rules/layout/ | sort | uniq -c
```
→ exatamente 1 ocorrência de cada uma das 10 props, todas em
`mod.rs:587–602`. Confirma: **as 10 props são lidas/mescladas num único
ponto** (mod.rs:587–602) e algumas têm leituras adicionais em
cursor.rs / equation.rs / mod.rs:2124/2202/2227.

Campo `chain: StyleChain` está no struct do layouter:
```bash
grep -nE "chain:\s+StyleChain|font_size_pt:" 01_core/src/rules/layout/mod.rs
# → mod.rs:87 (font_size_pt: Pt), mod.rs:97 (chain: StyleChain)
```

---

## 4. Tamanho atual de `content.rs`

Comando:

```bash
wc -l 01_core/src/entities/content.rs
```

Resultado:

```
4960 01_core/src/entities/content.rs
```

**`content.rs` = 4960 linhas** (confere com o ~4960 esperado).

Trajetória de referência (fornecida no briefing): **5782 → 4960** ao longo dos
lotes do Modelo D — redução de **822 linhas** (≈14,2%). Não re-medido aqui
(5782 é histórico; só 4960 é verificável no estado atual).

---

## Resumo dos números-chave

- **Linhas de arms remanescentes (custo misto que o F herda):** **50**
  (conservador, linhas exclusivamente próprias) · **68** (teto, contando
  menções nominais em blocos terminais agrupados).
- **Distribuição por match:** is_empty=3 · plain_text=12 · eq=14 ·
  get_field=0 · map_content=12 · map_text=13.
- **Σ larguras das 12 variantes não-migradas = 877**; maiores:
  `Sequence`=220, `Empty`=164, `MathIdent`=108, `Text`=93, `Styled`=82.
- **Propriedades de estilo distintas que o layouter consome = 10**
  (`TextStyle` ↔ `Style` ↔ `StyleChain`, alinhamento 1:1), todas mescladas
  no ponto único `mod.rs:587–602`.
- **`content.rs` = 4960 linhas** (trajetória 5782 → 4960).

---

## Dúvidas (para o dossiê §perguntas)

1. **Contagem de arms "term." (blocos `|`-agrupados):** adotei a convenção
   conservadora de **0 linhas próprias** para variantes dentro de
   `=> self.clone()` agrupado (map_content 1949–1984, map_text 2101–2151),
   reportando **68** como teto alternativo. Confirmar qual número (50 vs 68)
   é o canónico para a decisão do F.

2. **`get_field` tem 0 arms para as 12** mas só 2 arms no total (Heading,
   Figure) — todas as outras 65 variantes também caem em `_ => None`. Logo
   `get_field` é quase-vazio para *todo* o enum, não só para as 12. Isto é
   esperado/aceite (campos só expostos onde show-rules precisam) ou é dívida
   à parte do F?

3. **Larguras `Sequence`=220 e `Empty`=164** dominam o custo de qualquer
   reescrita. O regex `Content::V\b|Self::V\b` apanha *todas* as ocorrências
   em `01_core/src/` incluindo testes e construtores; não separei
   produção-vs-teste. Querem a largura desagregada (excluindo `*tests*` e
   `#[cfg(test)]`) para `Sequence`/`Empty`/`Styled`/`Text`?

4. **`weight`/`bold`/`italic`/`heading_level`/`font` = "0 leituras"** fora do
   merge `mod.rs:587–602`. Isto significa que 5 das 10 props só existem para
   serem propagadas (consumer parcial/diferido — ex.: `weight` via
   `faux_bold_stroke_pt`). Confirmar se uma opção PropMap deve dimensionar
   pelas **10** props (declaradas) ou pelas **~5** efetivamente lidas fora do
   merge.
