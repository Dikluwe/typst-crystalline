# Sondagem: o que falta em relação ao vanilla 0.15.0 — Passo 531

| Campo | Valor |
|-------|-------|
| Passo | P531 |
| Tipo | Diagnóstico |
| Data | 2026-07-02 |
| Vanilla de referência | **0.15.0** (`/tmp/typst-x86_64-unknown-linux-musl/typst`, release de 15 Jun 2026). Comparável também com 0.14.2 (`/usr/local/bin/typst`) onde relevante. |
| Status | Concluído |

---

## Nota metodológica

Este relatório segue ADR-0108: medição directa antes de classificação. Sempre que possível, os itens foram verificados com comandos corridos no repositório e comparados com o vanilla instalado. Inicialmente o único vanilla disponível era 0.14.2; posteriormente foi descarregado o binário oficial 0.15.0 (release 15 Jun 2026) e todos os testes directos foram repetidos. A secção "Diferenças 0.14.2 → 0.15.0 observadas" resume o que mudou no baseline.

---

## Grupo 1 — Baseline de funcionalidades já fechadas

| Área | Estado | Passo/relatório |
|------|--------|-----------------|
| Paridade de linguagem (sintaxe/semântica/morfologia) | FECHADO | P514 / `paridade-funcional-p514.md` |
| fontdb + system fonts | FECHADO | P515 / `paridade-producao-p515.md` |
| TrueType subsetting | FECHADO | P516 / `paridade-producao-p516.md` |
| System fonts por defeito + marcação de subset | FECHADO | P517 / `paridade-producao-p517.md` |
| Kerning | FECHADO | P520 / `paridade-producao-p520.md` |
| Ligatures + ToUnicode | FECHADO | P520/P521 / `paridade-producao-p521.md` |
| CFF subsetting | FECHADO | P522/P523 / `paridade-producao-p523.md` |
| Variation Fonts (peso) | FECHADO | P530 / `paridade-producao-p530.md` |
| Runtime state/counter | FECHADO | P506 / `paridade-funcional-p506.md` |

Fonte: `cristalino-contexto-handoff.md` (actualizado em P530) e relatórios de paridade em `00_nucleo/diagnosticos/`.

---

## Grupo 2 — Formatos de exportação (reconfirmação de P526)

| Formato | Estado | Verificação |
|---------|--------|-------------|
| HTML export | AUSENTE | `grep -rn "html\|HTML" 01_core/src/ 03_infra/src/` só encontra `MAX_HTML_DEPTH` (protecção de recursão), não exportador. `./target/release/typst --help` só menciona PDF. |
| SVG export | AUSENTE | `grep` só encontra detecção de extensão `.svg` em `image(...)`. Nenhum exportador SVG. |
| PNG/raster export | AUSENTE | `grep` só enconstra referências a carregamento de imagens PNG. Nenhum exportador raster. |
| IDE / LSP | AUSENTE | Nenhuma referência a LSP em `01_core`/`02_shell`/`03_infra`. Fora de escopo confirmado. |

**Conclusão:** estado de P526 mantém-se válido. Nenhum destes formatos foi implementado lateralmente.

---

## Grupo 3 — Fontes: casos fora de TrueType, CFF e Variation Fonts

### 3.1 — Fontes bitmap e a cores (emoji)

| Aspecto | Resultado |
|---------|-----------|
| Código que referencia COLR/CPAL/CBDT/CBLC/sbix | Nenhuma referência em `01_core/src/` ou `03_infra/src/`. |
| Teste directo com Noto Color Emoji | Compila (exit 0), mas o PDF embute `CrystallineFont` (CID TrueType fallback), não `NotoColorEmoji`. `pdftotext` mostra glifos PUA (``, ``) em vez de emoji colorido. |
| Vanilla 0.15.0 | Usa `NotoColorEmoji` Type 3 com emoji colorido. |
| Estado | **AUSENTE** — nenhum suporte a fontes de cor. |

### 3.2 — Fontes Type1/PostScript antigas

| Aspecto | Resultado |
|---------|-----------|
| Fontes Type1 no sistema | Sim: `/usr/share/fonts/X11/Type1/C059-*.pfb`. |
| Teste directo com C059 | Cristalino compila mas usa fallback `CrystallineFont`; não usa C059. |
| Vanilla 0.15.0 | Usa `C059-Roman-Identity-H` (CID Type 0C). |
| Estado | **AUSENTE** — Type1 não é suportado como fonte seleccionável. |

### 3.3 — Qualidade do fallback por caractere

| Aspecto | Resultado |
|---------|-----------|
| Teste directo | Documento com latim + CJK + emoji + árabe na mesma linha. |
| Cristalino | Usa Helvetica; `pdftotext` mostra `Hello ???? ? ?????` — caracteres não-latinos perdem-se. |
| Vanilla 0.15.0 | Selecciona Libertinus (latim), NotoSansCJKjp (CJK), NotoColorEmoji (emoji), FreeMono (árabe) na mesma linha. |
| Estado | **PARCIAL/FRAGIL** — existe fallback para fonte default, mas não fallback por caractere com qualidade vanilla. |

---

## Grupo 4 — PDF avançado

| Sub-área | Estado | Verificação |
|----------|--------|-------------|
| PDF Tagged / PDF/UA | AUSENTE | `grep` por `StructTree`, `/Tagged`, `MarkedContent`, `PDF/UA`, `/Lang` não encontra nada. `pdfinfo` reporta `Tagged: no`. |
| Bookmarks / Outlines | AUSENTE | `grep` por `/Outlines`, `PdfOutline`, `bookmark` não encontra nada. `mutool show ... outline` não mostra entradas para cristalino; vanilla mostra. |
| Object streams / xref streams | AUSENTE | `grep` por `ObjStm`, `XRefStm`, `object stream` não encontra nada. PDF gerado usa objectos individuais, não streams comprimidos. |
| Metadados XMP / Info dict | AUSENTE | `#set document(title: ..., author: ...)` emite warning "target 'document' ainda não suportado". `pdfinfo` não mostra Title/Author. Vanilla mostra Title, Author, Creator, datas e Metadata Stream. |

---

## Grupo 5 — Pacotes (`@preview`)

| Aspecto | Resultado |
|---------|-----------|
| Referências a `@preview`, `package`, `universe` | Nenhuma em `01_core/src/engine/eval/`. |
| Teste directo `#import "@preview/cetz:0.2.2"` | Erro: `import não implementado nesta versão do cristalino`. |
| Estado | **AUSENTE** — não há sequer parsing/resolução de pacotes de repositório. |

---

## Grupo 6 — Bibliografia com estilos CSL

| Aspecto | Resultado |
|---------|-----------|
| `.bib` parsing | **PARCIAL** — `#bibliography("refs.bib", style: "ieee")` compila e lê o ficheiro. |
| Citação inline | **AUSENTE** — `@key1` não é resolvido; o PDF mostra "See ." em vez de "See [1]." |
| Formatação CSL | **PARCIAL** — a bibliografia final é emitida com estilo "ieee", mas com erros de pontuação (`?A Sample Paper,?`, `Bibliograph` truncado). |
| Vanilla 0.15.0 | Citação e bibliografia correctas. |
| Estado | **PARCIAL** — infraestrutura existe, mas citações e formatação têm bugs. |

---

## Grupo 7 — Texto não-latino em profundidade

### 7.1 — Escrita vertical CJK (tategaki)

| Aspecto | Resultado |
|---------|-----------|
| Referências a escrita vertical | `Dir::TTB` existe, mas é usado para `stack()` (empilhamento vertical de blocos), não para escrita vertical de texto CJK. Não há `text.dir: ttb` nem lógica de layout vertical para runs de texto. |
| Estado | **AUSENTE** |

### 7.2 — Quebra de linha CJK / Thai

| Aspecto | Resultado |
|---------|-----------|
| Referências a CJK break / Thai / ICU / word segment | Nenhuma em `01_core/src/engine/layout/`. |
| Teste directo com japonês em coluna estreita | Cristalino não encontrou fonte CJK e usou Helvetica; não foi possível avaliar a quebra. Em termos de infraestrutura, não há segmentador de palavras CJK/Thai. |
| Estado | **AUSENTE** — assume espaços como únicos pontos de quebra. |

### 7.3 — Hifenização

| Aspecto | Resultado |
|---------|-----------|
| Código | Existe `01_core/src/engine/layout/hyphenation.rs` com wrapper `hypher`. |
| Integração | `cursor.rs` chama `hyphenate(word, &lang)` quando `hyphenate: true`. |
| Testes | `cargo test` inclui testes de hyphenation para EN e PT. |
| Estado | **FECHADO** — implementado (Passo 144 / ADR-0057). |

---

## Grupo 8 — Introspecção e layout em escala

### 8.1 — Footnotes em layout multi-coluna

| Aspecto | Resultado |
|---------|-----------|
| Teste directo | `#set page(columns: 2)` + `footnote[...]`. |
| Cristalino | Compila; a nota aparece no final da página, depois de ambas as colunas. Não fica no fundo da coluna onde foi referenciada. |
| Vanilla 0.15.0 | Nota no fundo da coluna correcta. |
| Estado | **PARCIAL** — footnotes funcionam em single-column; em multi-coluna o posicionamento está incorrecto. |

### 8.2 — Numeração de página customizada

| Aspecto | Resultado |
|---------|-----------|
| Teste directo | `#set page(numbering: "i")`. |
| Cristalino | Compila, mas os números de página não aparecem no PDF. |
| Vanilla 0.15.0 | Mostra `i`, `ii`, etc. |
| Estado | **AUSENTE** — a propriedade é reconhecida, mas o render não a aplica. |

---

## Diferenças 0.14.2 → 0.15.0 observadas

Todos os testes directos do Grupo 3–8 foram repetidos com o binário oficial 0.15.0. As diferenças observadas face a 0.14.2:

| Área | 0.14.2 | 0.15.0 | Impacto no gap cristalino |
|------|--------|--------|---------------------------|
| Emoji | NotoColorEmoji Type 3 | Igual | Sem alteração. |
| Fallback multi-script | Libertinus + NotoSansCJKjp + NotoColorEmoji + FreeMono | Igual | Sem alteração. |
| Type1 (C059) | C059-Roman-Identity-H CID Type 0C | Igual | Sem alteração. |
| Bibliografia CSL | `[1]` + formatação correcta | Igual | Sem alteração. |
| Metadados XMP | Title, Author, Creator, Metadata Stream, Tagged | Igual | Sem alteração. |
| Bookmarks | `/Outlines` presente | Igual | Sem alteração. |
| Numeração romana | `i`, `ii` | Igual | Sem alteração. |
| Footnotes multi-coluna | Nota no fundo da coluna correcta | Igual | Sem alteração. |
| Quebra CJK | NotoSansCJKjp + quebras correctas | Igual | Sem alteração. |
| Pacotes `@preview` | Não testado | `@preview/cetz:0.2.2` descarrega e compila com sucesso | 0.15.0 confirma que packages são realidade de produção; cristalino ainda não reconhece sintaxe. |

**Conclusão:** para o conjunto de funcionalidades sondadas, o vanilla 0.15.0 comporta-se de forma idêntica ao 0.14.2. A única novidade confirmada é o ecossistema de packages (`@preview`) a funcionar de forma madura. Os gaps do cristalino mantêm-se os mesmos.

---

## Tabela final

| Área | Estado | Notas |
|------|--------|-------|
| Shaping (rustybuzz) | FECHADO | P515/P520/P521 |
| Kerning | FECHADO | P520/P521 |
| Ligatures + ToUnicode | FECHADO | P520/P521 |
| CFF subsetting | FECHADO | P522/P523 |
| Variation Fonts | FECHADO | P530 |
| HTML export | AUSENTE | Confirmado P526 |
| SVG export | AUSENTE | Confirmado P526 |
| PNG export | AUSENTE | Confirmado P526 |
| IDE/LSP | AUSENTE | Fora de escopo |
| Fontes de cor (emoji) | AUSENTE | Nenhuma referência COLR/CPAL/CBDT/CBLC/sbix |
| Type1/PostScript | AUSENTE | Fontes existem no sistema, cristalino não usa |
| Fallback multi-script | PARCIAL/FRÁGIL | Fallback por caractere não funciona; CJK/emoji/árabe perdem-se |
| PDF Tagged/UA | AUSENTE | Nenhuma estrutura semântica |
| Bookmarks PDF | AUSENTE | Sem `/Outlines` |
| Compressão object streams | AUSENTE | PDF usa objectos individuais |
| Metadados XMP | AUSENTE | `#set document(...)` não suportado |
| Pacotes (`@preview`) | AUSENTE | Import de pacote dá erro directo |
| Bibliografia CSL | PARCIAL | `.bib` lido, mas citações e formatação com bugs |
| Escrita vertical CJK | AUSENTE | `Dir::TTB` só para stacks |
| Quebra de linha CJK/Thai | AUSENTE | Sem segmentador específico |
| Hifenização | FECHADO | P144 / ADR-0057 |
| Footnotes multi-coluna | PARCIAL | Posicionamento incorrecto em 2 colunas |
| Numeração de página customizada | AUSENTE | Propriedade reconhecida, mas não renderizada |

---

## Itens de alto impacto entre os AUSENTE/PARCIAL

Considerando documentos comuns:

1. **Numeração de página customizada** (`#set page(numbering: ...)`): aparece em praticamente todos os documentos formais. **Alto impacto.**
2. **Fallback multi-script / emoji**: documentos com um emoji ou um nome próprio CJK falham visualmente. **Alto impacto.**
3. **Bookmarks PDF**: documentos longos (relatórios, teses) perdem navegabilidade. **Médio-alto impacto.**
4. **Metadados XMP básicos** (title/author): relevante para publicação/arquivamento. **Médio impacto.**
5. **Bibliografia CSL** (citações): essencial para documentos académicos. **Médio-alto impacto.**
6. **Footnotes multi-coluna**: uso menos frequente, mas quando usado fica incorrecto. **Médio impacto.**

Itens de **baixo impacto** para a maioria dos documentos: HTML/SVG/PNG export, IDE/LSP, PDF/UA tagged, object streams, Type1 fonts, escrita vertical CJK, pacotes `@preview`.

---

## Recomendação de prosseguimento

A reorganização (P532+) pode avançar, mas **não deve esconder os itens de alto impacto**. A sugestão é:

- **Antes ou durante a reorganização**, abordar: numeração de página, fallback multi-script/emoji, bookmarks PDF, metadados XMP básicos, e citações CSL.
- **Deixar para pós-reorganização (com débito registado)**: HTML/SVG/PNG export, LSP, PDF/UA, object streams, Type1, escrita vertical CJK, pacotes `@preview`.

---

## Comandos de verificação (para reprodução)

```bash
# Grupo 2
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
./target/release/typst --help

# Grupo 3.1
./target/release/typst /tmp/test-emoji.typ /tmp/emoji-cristalino.pdf
pdffonts /tmp/emoji-cristalino.pdf

# Grupo 3.3
./target/release/typst /tmp/test-mixed.typ /tmp/mixed-cristalino.pdf
pdftotext /tmp/mixed-cristalino.pdf -

# Grupo 4.4
./target/release/typst /tmp/test-meta.typ /tmp/meta-cristalino.pdf
pdfinfo /tmp/meta-cristalino.pdf

# Grupo 5
./target/release/typst /tmp/test-package.typ /tmp/package-cristalino.pdf

# Grupo 6
./target/release/typst /tmp/test-bib.typ /tmp/bib-cristalino.pdf
pdftotext /tmp/bib-cristalino.pdf -

# Grupo 8.1
./target/release/typst /tmp/test-footnote-cols.typ /tmp/footnote-cols.pdf
pdftotext /tmp/footnote-cols.pdf -

# Grupo 8.2
./target/release/typst /tmp/test-page-num.typ /tmp/page-num-cristalino.pdf
pdftotext /tmp/page-num-cristalino.pdf -
```

---

## Referências

- `00_nucleo/materialization/typst-passo-531.md` — especificação do passo.
- `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` — baseline de funcionalidades fechadas.
- `00_nucleo/diagnosticos/sonda-igualdade-saida-p526.md` — sondagem anterior de HTML/SVG/PNG/LSP.
- Typst 0.15.0 release: <https://github.com/typst/typst/releases/tag/v0.15.0> (asset `typst-x86_64-unknown-linux-musl.tar.xz`).
