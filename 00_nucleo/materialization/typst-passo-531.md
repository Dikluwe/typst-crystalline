---
# P531 — Sondagem: o que falta em relação ao vanilla 0.15.0, antes da reorganização

> **Passo:** 531
> **Data:** 2026-07-02
> **Foco:** Levantar, com medição directa (não com memória do que já foi discutido), tudo o que o vanilla 0.15.0 tem e o cristalino ainda não tem. Cobre áreas já sondadas antes (HTML/SVG/PNG/LSP em P526) para as reconfirmar, e áreas ainda não sondadas neste projecto (fontes especiais, PDF avançado, pacotes, bibliografia com estilos, texto não-latino em profundidade). Zero código de produção. O resultado decide se avança para a reorganização (P532+) ou se há mais trabalho de paridade a fazer primeiro.
> **Tipo:** Diagnóstico.
> **Tamanho:** L (~90 min — é uma sondagem ampla, não um passo pequeno).
> **ADR-0108 EM VIGOR** — medir antes de decidir. **ADR-0114 EM VIGOR** — sonda antes de qualquer especificação de implementação.
> **Dependências:** P514 (paridade de linguagem fechada), P515–P530 (paridade de produção: shaping, kerning, ligatures, CFF, Variation Fonts), P526 (sonda anterior de HTML/SVG/PNG/LSP).

---

## Contexto

Antes de decidir avançar para uma reorganização do projecto (ADRs, prompts, débitos, tradução), faz sentido confirmar que não falta nada de relevante por implementar. Este passo não assume que a lista de P526 (HTML, SVG, PNG, LSP) é a lista completa — é revista aqui, e alargada a áreas que nunca foram sondadas neste projecto.

Cada grupo abaixo termina com uma classificação: `FECHADO` (confirmado por passo anterior, com número), `AUSENTE` (zero código, não sondado antes), ou `PARCIAL` (existe algo, mas incompleto).

---

## Grupo 1 — Confirmar o baseline exacto, sem depender de memória

Antes de sondar o que falta, confirmar o que já está fechado, lendo directamente os relatórios, não recontando de memória.

```bash
ls 00_nucleo/diagnosticos/*.md | sort
grep -l "FECHADO\|Fechado em" 00_nucleo/diagnosticos/*.md
```

Produzir uma tabela simples: passo, área, estado, ficheiro de relatório. Esta tabela é a base de comparação para todos os grupos seguintes — evita reabrir sondas sobre coisas já resolvidas.

### Critério de fecho

- [ ] Tabela de baseline produzida, com referência ao ficheiro de relatório de cada item.

---

## Grupo 2 — Formatos de exportação (reconfirmar P526)

P526 sondou HTML, SVG, PNG, LSP e classificou por complexidade, mas não implementou nada. Confirmar se esse estado mudou entretanto (por exemplo, se algum passo posterior tocou nisto de forma lateral).

```bash
grep -rn "html\|HTML" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "//\|# " | head -20
grep -rn "svg\|SVG" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "//\|# " | head -20
grep -rn "png\|PNG\|raster" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "//\|# " | head -20
./target/release/typst compile --help
```

### Critério de fecho

- [ ] Confirmado se o estado de P526 continua válido ou mudou.
- [ ] Se mudou, registar o quê e quando.

---

## Grupo 3 — Fontes: casos fora de TrueType, CFF, e Variation Fonts

O projecto fechou TrueType (P516), CFF (P522/P523), e Variation Fonts (P530). Não sondou:

### 3.1 — Fontes bitmap e a cores (emoji)

```bash
grep -rn "COLR\|CPAL\|CBDT\|CBLC\|sbix" 01_core/src/ 03_infra/src/ --include="*.rs"
```

O vanilla suporta fontes de emoji a cores (tabelas `COLR`/`CPAL` ou `CBDT`/`CBLC`). Confirmar se o cristalino sequer reconhece essas tabelas, ou se um documento com emoji produz glifo em preto-e-branco, `.notdef`, ou falha.

Teste directo:

```bash
cat > /tmp/test-emoji.typ <<'EOF'
#set text(font: "Noto Color Emoji", size: 40pt)
🎉 Hello 🌍
EOF
./target/release/typst /tmp/test-emoji.typ /tmp/emoji-cristalino.pdf
echo "Exit code: $?"
```

### 3.2 — Fontes Type1/PostScript antigas

```bash
find / -iname "*.pfa" -o -iname "*.pfb" 2>/dev/null | head -5
```

Type1 é um formato antigo, cada vez mais raro, mas ainda presente nalguns sistemas (LaTeX legacy). Confirmar se existe no sistema de testes e, se existir, o que o cristalino faz com ele.

### 3.3 — Qualidade do font fallback

O vanilla tem lógica de fallback por carácter bastante refinada (troca de fonte a meio da linha quando um carácter não está coberto pela fonte pedida, preservando o resto dos atributos). Confirmar se o cristalino faz o mesmo com a mesma qualidade, com um teste directo de texto misto (latim + CJK + emoji na mesma linha).

### Critério de fecho

- [ ] Fontes de cor (COLR/CPAL): classificado.
- [ ] Type1: classificado (mesmo que "não testável, ausente do sistema").
- [ ] Fallback por carácter em texto misto: comparado com vanilla.

---

## Grupo 4 — PDF avançado

Nunca sondado neste projecto.

### 4.1 — PDF/UA e Tagged PDF (acessibilidade)

```bash
grep -rn "StructTree\|/Tagged\|MarkedContent\|PDF/UA\|/Lang " 01_core/src/ 03_infra/src/ --include="*.rs"
```

O vanilla 0.15.0 tem suporte a PDF taggeado (estrutura semântica para leitores de ecrã). Confirmar se o cristalino gera algum tipo de marcação estrutural, ou se o PDF é "achatado" (só visual, sem estrutura para acessibilidade).

### 4.2 — Bookmarks / painel de navegação do PDF

Distinto de `outline()` do documento (que é conteúdo visível na página). Isto é a árvore `/Outlines` do catálogo PDF, que aparece no painel lateral de um leitor de PDF.

```bash
grep -rn "/Outlines\|PdfOutline\|bookmark" 03_infra/src/export/ --include="*.rs"
```

Testar visualmente: abrir um PDF gerado pelo cristalino num leitor e verificar se o painel de marcadores tem entradas.

### 4.3 — Compressão: object streams e cross-reference streams

```bash
grep -rn "ObjStm\|XRefStm\|object stream\|compressed xref" 03_infra/src/export/ --include="*.rs"
```

PDF 1.5+ permite comprimir objectos em streams, reduzindo significativamente o tamanho do ficheiro. Confirmar se o cristalino usa isto ou gera PDF "não comprimido" na estrutura (mesmo que o conteúdo de fontes/imagens esteja comprimido à parte).

### 4.4 — Metadados XMP e Info dict customizado

```bash
grep -rn "XMP\|DocumentInfo\|/Title\|/Author\|/Keywords" 03_infra/src/export/ --include="*.rs"
```

O Typst permite definir `#set document(title: ..., author: ...)`. Confirmar se isto chega ao PDF final como metadados reais.

### Critério de fecho

- [ ] Cada um dos 4 sub-itens classificado: `FECHADO`, `AUSENTE`, ou `PARCIAL`.

---

## Grupo 5 — Pacotes (`#import` de repositório, Typst Universe)

Área nunca discutida neste projecto até agora.

```bash
grep -rn "@preview\|package\|universe\|import.*@" 01_core/src/rules/eval/ --include="*.rs" | head -20
```

O Typst vanilla tem um sistema de pacotes (`#import "@preview/nome:versão"`) que descarrega pacotes de um repositório central. Confirmar se o cristalino reconhece esta sintaxe, mesmo que não tenha acesso de rede para descarregar de facto — a pergunta é se a linguagem reconhece a forma da importação.

### Critério de fecho

- [ ] Sintaxe de importação de pacote: reconhecida ou erro de parse.
- [ ] Se reconhecida: o que acontece ao tentar resolver (erro claro de "sem rede", ou falha confusa).

---

## Grupo 6 — Bibliografia com estilos (CSL)

```bash
grep -rn "csl\|CSL\|citation.style\|\.bib\b" 01_core/src/ --include="*.rs" | head -20
```

O Typst vanilla suporta ficheiros `.bib` (BibTeX) e estilos de citação via CSL (Citation Style Language) — permite escolher entre milhares de estilos de citação diferentes (APA, IEEE, Chicago, etc.). Confirmar se o cristalino tem `#bibliography()` a funcionar minimamente, e se aceita `.bib`, e se algum estilo CSL está implementado, ou se só tem um formato fixo.

### Critério de fecho

- [ ] `.bib` parsing: confirmado ou ausente.
- [ ] Estilos CSL: quantos (0, 1, ou vários) implementados.

---

## Grupo 7 — Texto não-latino em profundidade

O corpus RTL (P488) e o shaper com bidi (P484) cobrem árabe/hebraico horizontal. Não cobrem:

### 7.1 — Escrita vertical CJK (tategaki)

```bash
grep -rn "vertical\|tategaki\|writing.mode\|TTB\|top.to.bottom" 01_core/src/ --include="*.rs"
```

### 7.2 — Quebra de linha para scripts sem espaços (CJK, Thai)

```bash
grep -rn "line.break\|CJK.*break\|Thai\|ICU\|word.segment" 01_core/src/rules/layout/ --include="*.rs"
```

O japonês, chinês, e tailandês não usam espaços entre palavras. A quebra de linha correcta nestes scripts exige regras específicas (ou um segmentador de palavras, no caso do tailandês). Confirmar se o cristalino trata isto correctamente ou quebra a meio de palavras/caracteres sem critério.

### 7.3 — Hifenização

```bash
grep -rn "hyphenat\|hyphen" 01_core/src/ --include="*.rs"
```

### Critério de fecho

- [ ] Escrita vertical: classificado.
- [ ] Quebra de linha CJK/Thai: testada com um documento real, comparada com vanilla.
- [ ] Hifenização: classificada.

---

## Grupo 8 — Introspecção e layout em escala

### 8.1 — Footnotes em layout multi-coluna

```bash
cat > /tmp/test-footnote-cols.typ <<'EOF'
#set page(columns: 2)
#lorem(50)#footnote[Nota numa coluna.]
#lorem(50)
EOF
./target/release/typst /tmp/test-footnote-cols.typ /tmp/footnote-cols.pdf
echo "Exit code: $?"
```

Confirmar visualmente onde a nota de rodapé aparece (fundo da coluna, fundo da página, ou incorrecto).

### 8.2 — Numeração de página com padrões customizados

```bash
grep -rn "numbering.*page\|page.*numbering\|roman\|\"i\"\|\"I\"" 01_core/src/rules/stdlib/ --include="*.rs" | head -10
```

O Typst permite `#set page(numbering: "i")` (numeração romana) ou padrões customizados. Confirmar cobertura.

### Critério de fecho

- [ ] Footnotes multi-coluna: testado, resultado registado.
- [ ] Numeração de página customizada: classificada.

---

## Tabela final

| Área | Estado | Passo/relatório de referência |
|------|--------|-------------------------------|
| Shaping (rustybuzz) | FECHADO | P515/P520/P521 |
| Kerning | FECHADO | P520/P521 |
| Ligatures + ToUnicode | FECHADO | P520/P521 |
| CFF subsetting | FECHADO | P522/P523 |
| Variation Fonts | FECHADO | P530 |
| HTML export | _a preencher_ | P526 (reconfirmar) |
| SVG export | _a preencher_ | P526 (reconfirmar) |
| PNG export | _a preencher_ | P526 (reconfirmar) |
| IDE/LSP | _a preencher_ | P526 (reconfirmar) |
| Fontes de cor (emoji) | _a preencher_ | novo |
| Type1/PostScript | _a preencher_ | novo |
| Fallback multi-script | _a preencher_ | novo |
| PDF Tagged/UA | _a preencher_ | novo |
| Bookmarks PDF | _a preencher_ | novo |
| Compressão object streams | _a preencher_ | novo |
| Metadados XMP | _a preencher_ | novo |
| Pacotes (`@preview`) | _a preencher_ | novo |
| Bibliografia CSL | _a preencher_ | novo |
| Escrita vertical CJK | _a preencher_ | novo |
| Quebra de linha CJK/Thai | _a preencher_ | novo |
| Hifenização | _a preencher_ | novo |
| Footnotes multi-coluna | _a preencher_ | novo |
| Numeração de página customizada | _a preencher_ | novo |

---

## Decisão de prosseguimento

Depois de preencher a tabela:

- **Se tudo relevante estiver `FECHADO` ou os `AUSENTE`/`PARCIAL` forem de baixo impacto** (ex.: pacotes, PDF/UA, escrita vertical — usos específicos, não centrais para a maioria dos documentos): a reorganização (P532+) pode avançar.
- **Se houver `AUSENTE`/`PARCIAL` de alto impacto** (ex.: metadados XMP básicos, numeração de página customizada, footnotes multi-coluna — coisas que aparecem em documentos comuns): decidir se entram antes da reorganização ou se ficam registados como trabalho futuro pós-reorganização, mas não escondidos.

Este passo não decide sozinho — produz os dados para a decisão.

---

## Relatório de execução

`00_nucleo/diagnosticos/sondagem-vanilla-p531.md` — tabela final preenchida, com nota por cada item de como foi verificado (comando corrido, resultado, ficheiro de referência se já sondado antes).
