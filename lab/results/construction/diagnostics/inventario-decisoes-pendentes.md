# Inventário de Decisões Pendentes / Pontos em Aberto

**Criado em:** 2026-07-03 (Passo 550)  
**Actualizado em:** 2026-07-08 (Passos 602, 604, 605, 606, P608, P611, P616)  
**Nota:** Não existia um ficheiro `inventario-decisoes-pendentes.md` anterior. Esta é a primeira consolidação formal. A lista informal de ~24 itens referida em P550 foi auditada e integrada na secção 2 deste documento.

---

## 1. Achados do Passo 550 — Auditoria do projecto

### 1.1 Categoria A — Conclusões "fechado" sem número/comando autocontido

Estes relatórios contêm evidência concreta noutras secções, mas a conclusão final é uma afirmação qualitativa. Quem lê só o final não vê a medição.

| Ficheiro:linha | Conclusão exacta | Risco | Acção recomendada |
|---|---|---|---|
| `paridade-producao-p538d.md:99` | "P538d está concluído. [...] **Não houve regressões.**" | Baixo | Incluir na conclusão o comando/número do teste manual (p. ex. `pdftotext` idêntico ao vanilla). |
| `paridade-producao-p540.md:126` | "P540 está concluído. `#for (i, x) in items.enumerate()` **funciona no cristalino**." | Baixo | Referenciar na conclusão o teste de corpus `structural_parity` e o output `pdftotext`. |
| `paridade-producao-p541.md:151` | "P541 está concluído. Padrões compostos de numeração de página **funcionam no cristalino**, usando o total de páginas correcto." | Baixo | Incluir na conclusão a tabela de comparação `pdftotext` com vanilla. |
| `paridade-producao-p543.md:84` | "P543 está concluído. [...] O caso comum [...] **permanece sem regressão**." | Baixo | Juntar à conclusão o teste manual contra vanilla (texto misto sem duplicação). |
| `paridade-producao-p547.md:126` | "**A validação final (`cargo test`, `crystalline-lint`) está verde.** Os artefactos tipográficos restantes são de escopo de fonte/shaper..." | Médio | A conclusão não menciona a comparação vanilla nem os artefactos que desapareceram após P548; deve referenciar a secção 4.2 do próprio relatório. |

### 1.2 Categoria B — Scope-out / fora de escopo sem razão escrita

| Ficheiro:linha | Item | Problema | Razão que deveria constar |
|---|---|---|---|
| `paridade-funcional-p514.md:212` | HTML export | "Fora de escopo" sem razão | Cristalino é PDF-only (decisão arquitectural / ADR-0107). |
| `paridade-funcional-p514.md:213` | SVG export | "Fora de escopo" sem razão | Backend vectorial não é o target do projecto. |
| `paridade-funcional-p514.md:214` | Raster render (PNG) | "Fora de escopo" sem razão | Rasterização é L3 alternativo; projecto prioriza PDF. |
| `paridade-funcional-p514.md:215` | IDE / LSP | "Fora de escopo" sem razão | Fora das camadas cristalinas (L1–L4); seria projeto separado. |
| `paridade-funcional-p514.md:215` | Plugin system (`plugin()`) | "Fora de escopo" sem razão | Requer runtime de plugins não previsto na arquitectura. |
| `paridade-gap-p508.md:152` | HTML/SVG/raster export | "Fora de escopo" sem razão | PDF-only / prioridade de produção. |
| `sondagem-vanilla-p531.md:197` | IDE/LSP | "Fora de escopo" (tabela resumo) | A justificativa completa está na linha 44 do mesmo ficheiro; a tabela resumo deve repeti-la ou referenciá-la. |
| `paridade-funcional-p497.md:123` | `test-page.typ` heading | "scope-out" isolado | Explicar por que heading em test-page é scope-out (falta de layout de heading na altura?). |
| `paridade-funcional-p497.md:124` | `test-place.typ` heading | "scope-out" isolado | Mesmo que acima. |
| `paridade-producao-p515.md:187` | CFF subsetting | "Scope-out do subsetting" (circular) | Justificar: CFF requer parser CFF separado; prioridade baixa face a TrueType/OpenType. |
| `paridade-producao-p515.md:133` | Suporte a CFF adicional | "scope-out adicional" vago | Mesmo que acima. |
| `regressao-subsetting-p519.md:231` | Small caps (`smcp`) | "Scope-out" sem razão | Requer instanciação/variantes de fonte (foco P529/P530). |
| `regressao-subsetting-p519.md:232` | Contextual alternates (`calt`) | "Scope-out" sem razão | Requer suporte a features GSUB avançadas. |
| `paridade-producao-p534.md:64` | Fusão de runs | "A fusão é **scope-out**; foi registada como item técnico futuro." | Explicar que fusão é optimização de export (`BT...ET`), não de correcção de fallback. |
| `paridade-producao-p534.md:143` | Posicionamento pós-layout | "(posicionamento pós-layout não é objectivo deste passo)." | Explicar que o focus é escolha de fonte, não quebra de linha multi-script. |
| `paridade-producao-p537.md:24` | Overflow de nota grande em colunas | "scope-out conhecido." | A explicação completa está na linha 142; a linha 24 deve referenciá-la. |
| `paridade-funcional-p500.md:106` | `numbering:` scope-out/cosmético | "scope-out/cosmético" vago | Explicar que faltava `counter(page).display()` para padrões compostos, tratado em P532/P541. |

### 1.3 Categoria C — Ficheiros com passos próximos no tempo que não se cruzaram

| Ficheiro | Passos próximos | Relatórios | Cruzaram referências? | Risco |
|---|---|---|---|---|
| `01_core/src/engine/layout/mod.rs` | P537, P537b, P538c, P538d, P541, P544, P547 (2026-07-02/03) | Todos lidos | Sim — cadeia coerente de colunas/numeração/CSL. | Baixo |
| `01_core/src/engine/layout/cursor.rs` | P537, P538c, P541, P544, P545 (2026-07-02/03) | Todos lidos | **Parcial** — P545 altera `layout_word` e não referencia directamente P544, que também tinha mexido no mesmo ficheiro. | Médio |
| `01_core/src/engine/layout/text.rs` | P544, P547 (2026-07-03) | P544, P547 | Sim — P547 descreve explicitamente a substituição de `split_whitespace()` de P544. | Baixo |
| `01_core/src/engine/layout/bib_csl.rs` | P533, P547 (2026-07-02/03) | P533, P547 | Sim — focos distintos (citação inline vs formatação CSL). | Baixo |
| `01_core/src/engine/eval/bibliography.rs` | P497, P498, P538f, P540 (2026-06-29 a 07-03) | P497, P498, P538f, P540 | Sim — P540 referencia P538f. | Baixo |
| `03_infra/src/shaper.rs` | P534, P538e, P543, P548 (2026-07-02/03) | P534, P538e, P543, P548 | Sim — cadeia coerente de fallback. | Baixo |
| `03_infra/src/font_metrics.rs` | P544, P548 (2026-07-03) | P544, P546, P548 | Sim — P546 diagnosticou; P548 corrigiu. | Baixo |
| `03_infra/src/export/stream.rs` | P520, P530, P548 (2026-07-01/03) | P520, P530, P548, P549 | Sim — P549 verifica P548 contra P520. | Baixo |
| `03_infra/src/export/builder.rs` | P520, P521, P535, P536, P538b, P548 (2026-07-01/03) | P520, P521, P535, P536, P538b, P548 | **Parcial** — P535 e P536 são adjacentes (mesmo dia), focos separados, não se referenciam directamente, mas não colidem funcionalmente. | Médio |

### 1.4 Itens re-testados em P550 e respectivo veredicto

| # | Passo | Item re-testado | Método | Resultado | Veredicto |
|---|---|---|---|---|---|
| 1 | P540 | Destructuring `#for (i, x) in ...enumerate()` | `pdftotext` vs vanilla | Output idêntico: `1. um2. dois3. três` | Confirmado fechado |
| 2 | P541 | Numeração composta `"1 / 1"` | `pdftotext` vs vanilla | `1/3`, `2/3`, `3/3` em ambos | Confirmado fechado |
| 3 | P543 | Fallback global sem duplicação | Documento com `font: "Helvetica"` + texto latino+CJK | Sem duplicação; texto extraído idêntico | Confirmado fechado |
| 4 | P538d | `style.font` no texto de numeração | Numeração automática simples | Texto extraído idêntico; teste unitário valida `style.font.is_some()` | Confirmado fechado |
| 5 | P545 | Interpolação `#{expr}` em markup | Documento com int/float/array/bool | Funciona; diferença de espaço em listas é gap pré-existente de layout de listas | Confirmado fechado com gap documentado |
| 6 | P534 | Fallback por script (latim+CJK+árabe) | `pdftotext -raw` + `pdffonts` vs vanilla | Texto extraído idêntico; fontes distintas conforme esperado | Confirmado fechado |
| 7 | P537 | Notas de rodapé em `columns(2)` | `pdftotext` + `mutool trace` | Cristalino coloca notas lado a lado no fundo; vanilla 0.14.2 coloca verticalmente | Comportamento cristalino corresponde à semântica esperada; scope-out de overflow documentado |
| 8 | P535 | Bookmarks PDF `/Outlines` | `mutool show ... outline` | Estrutura e hierarquia idênticas; `zoom=nan` vs `100` scope-out conhecido | Confirmado fechado |
| 9 | P536 | Metadados `/Info` | `pdfinfo` | `Title`/`Author` correctos; `Creator` difere (esperado) | Confirmado fechado |
| 10 | P547 | CSL/BibTeX (estilo IEEE) | `pdftotext` vs vanilla | Conteúdo correcto; artefactos tipográficos (`T ypography`, `" A`) desapareceram após P548 | Confirmado fechado (gap CSL/hayagriva permanece fora deste focus) |
| 11 | P533 | Citações inline `@key1` | `pdftotext` vs vanilla | `See [1].` + bibliografia em ambos | Confirmado fechado |
| 12 | P532 | Numeração simples `"i"` | `pdftotext` vs vanilla | `i`, `ii` em ambos | Confirmado fechado |
| 13 | P520/P548 | Sinal do delta `TJ` / kerning | `mutool show` + `mutool trace` (P549) | Posições reais de glifos idênticas ao vanilla | Confirmado fechado |

### 1.5 Itens que continuam abertos após P550

| Item | Passo original | Estado actual | Por que continua aberto |
|---|---|---|---|
| Documento multi-página em `#set page(columns: 2)` | P537/P537b/P553/P554 | **Fechado** | Após P553 (geometria) e P554 (fonte padrão): cristalino **2 páginas**, vanilla 0.14.2/0.15.0 **2 páginas** para `#lorem(1200)`. Ver `paridade-producao-p553.md` e `paridade-producao-p554.md`. |
| Fonte por defeito do cristalino | P554/P555/P556 | **Fechado** | Alterada de `Helvetica`/fallback sans-serif para `FreeSerif` (serif amplamente disponível). P555 adicionou fallback por classe (serif/sans). P556 mediu o corpus completo: 0 documentos mudaram de páginas; variações de palavras extraídas são artefactos de extração, não regressão. Ver `paridade-producao-p554.md`, `paridade-producao-p555.md` e `paridade-producao-p556.md`. |
| Fusão de blocos `BT...ET` consecutivos | P534, P608 | **Scope-out confirmado** | P608 reconfirmou que o sintoma persiste: num documento com fallback de fonte, o cristalino gera 11 blocos `BT...ET` contra 5 do vanilla. Trata-se de optimização de export — o texto renderiza correctamente; a correcção exigiria fundir runs adjacentes com fontes diferentes num único bloco. **Nota:** o tamanho massivo do PDF em P608 (15,7 MB vs 12 KB do vanilla) não era causado pela fusão de blocos, mas por fontes de fallback `.ttc` embutidas inteiras; corrigido em P609. |
| Zoom explícito em destinos `/XYZ` de bookmarks | P535 | **Aberto** (scope-out) | Cristalino usa `null`; vanilla usa `100`. |
| Stream XMP de metadados | P536 | **Fechado em P611** | O vanilla 0.15.0 emite sempre um pacote XMP (`<?xpacket ... ?>`), mesmo sem metadados de utilizador. O cristalino passa a emitir o equivalente com `/Type /Metadata /Subtype /XML` referenciado no catálogo. |
| Ordenação sofisticada de fallback (fontique) | P534 | **Aberto** (scope-out) | Ordem linear do `FontBook` pode não ser ideal para scripts complexos. |
| `/Producer` em `/Info` | P536 | **Fechado** (P600) | Medição em P600 mostra que o vanilla 0.15.0 **não** escreve `/Producer`; escreve `/Creator (Typst 0.15.0)`. O cristalino já preenche `/Creator (typst-crystalline)`. A disparidade era um mal-entendido. |

---

## 2. Lista de ~24 itens materializada em P551

A lista informal referida em P550 foi auditada e integrada nesta secção. Itens já corrigidos por passos posteriores são marcados como tal; itens ainda abertos mantêm o estado original.

### 2.1 Formatos de exportação — nunca implementados

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| HTML export | P526 | Ausente |
| SVG export | P526 | Ausente |
| PNG/raster export | P526 | Ausente |
| IDE / LSP | P526 | Ausente |

### 2.2 Fontes

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| Fontes de cor para emoji (COLR/CPAL/CBDT/CBLC/sbix) | P531 | Ausente |
| Fontes Type1/PostScript | P531 | Ausente |
| Escrita vertical CJK | P531 | **Não é disparidade após P616** — o vanilla 0.15.0 rejeita `dir: ttb/btt` em `#set text(...)`. O layout vertical continua ausente, mas agora com validação de paridade. |
| Quebra de linha para CJK/Thai | P531 | Ausente |
| Quebra de linha errada com fallback de fonte | P534, P538e | **Já corrigido — ver P544/P548** |
| Lista fixa de 5 fontes no fallback, sem alternativa se nenhuma existir | P538e (DEBT-65) | **Já corrigido — ver P543** |
| `y_offset` no emit PDF | P486 (histórico) | Scope-out declarado |
| `smcp` via OpenType real | P446/P486 (histórico) | Scope-out declarado |
| `x_advance` exacto vs `hmtx` | P485 (histórico) | Scope-out declarado |

### 2.3 PDF — estrutura e metadados

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| PDF Tagged / PDF-UA | P531 | Ausente |
| Compressão por object streams | P531 | Ausente |
| Stream XMP | P536 | **Fechado em P611** — o vanilla 0.15.0 emite sempre um pacote XMP (`<?xpacket ... ?>`), mesmo sem metadados de utilizador. O cristalino passa a emitir o equivalente com `/Type /Metadata /Subtype /XML` referenciado no catálogo. |
| `/Producer` | P536 | **Fechado em P600** — o vanilla não usa este campo; usa `/Creator`. O cristalino já preenche `/Creator`. |
| `/CreationDate` sem fuso horário | P536 | Scope-out |
| `subject` em `#set document(...)` | P536 | Não suportado |
| Zoom explícito nos bookmarks | P535 | Scope-out — já em P550 |
| Bookmarks só de headings | P535 | **Fechado em P604** — vanilla 0.15.0 também só gera bookmarks a partir de headings. Labels manuais não entram em `/Outlines`; `#outline(target: ...)` gera TOC/listas no documento, não bookmarks PDF. |
| `/Count` aberto/fechado | P535 | **Fechado em P602** — vanilla 0.15.0 emite `/Count -N` para entradas com filhos (fechadas por defeito) e `/Count N` positivo na raiz `/Outlines`; cristalino replica o comportamento. |
| `bookmarked`/`outlined` em `heading()` | P604/P605/P606 | **Fechado em P606** — `HeadingElem` separa `outlined: bool` (índice do documento) de `bookmarked: Option<bool>` (bookmarks PDF, `None` = auto segue `outlined`). O walk emite `HeadingForToc` e `HeadingForBookmarks` independentemente, replicando o comportamento do vanilla 0.15.0. |

### 2.4 Numeração de página

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| Padrões compostos (`"1 / 1"`, `"I-1"`) | P532 | **Já corrigido — ver P541** |

### 2.5 Colunas

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| Nota de rodapé maior do que o espaço restante | P537 | Scope-out — já em P550 |
| Notas de rodapé lado a lado vs empilhadas | Ver Parte 2 de P551 | **Corrigido em P552** — `#set page(columns: 2)` mantém notas lado a lado (alinhado com vanilla); `#columns(2)` passa a empilhar notas na primeira coluna, como o vanilla |
| Numeração duplicada de notas em `#set page(columns: 2)` | Descoberto em P551 | **Corrigido em P552** — `footnote_counter` deixa de ser restaurado por segmento de coluna; contador avança monotonicamente `[1]`, `[2]`, ... |

### 2.6 Bibliografia

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| Formatação CSL — cinco categorias de bug | P533, P539 | **Já corrigido — ver P547/P548** |

### 2.7 Pacotes

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| `#import "@preview/..."` | P531 | Ausente |

### 2.8 Linguagem

| Item | Onde foi encontrado | Estado |
|------|---------------------|--------|
| `#for` descarta conteúdo | Descoberto em P538c | **Já corrigido — ver P538f** |
| `#for` com destructuring de tuplos | Descoberto em P538i | **Já corrigido — ver P540** |
| `#{expr}` em markup | Descoberto em P540 | **Já corrigido — ver P545** |
| 20 diferenças de texto do corpus | P538i | **Maioria explicada — ver P539/P544** |
| Contagem de `lorem()` | P538c/g | **Explicada — ver P539/P544** |

---

## 3. Notas sobre risco geral

- **Risco documental é maior que risco funcional.** A maioria dos "fechados" sem número na conclusão tem evidência concreta noutras secções; o problema é a conclusão não ser autocontida.
- **Scope-outs do projecto** (HTML/SVG/raster/IDE/plugin) estão bem estabelecidos como decisões arquitecturais, mas a justificativa nos relatórios P514/P508/P531 é mínima. Isso não invalida a decisão, mas enfraquece a trilha de auditoria.
- **Passos recentes cruzam referências de forma geralmente boa.** Os únicos pontos de atenção são `cursor.rs` (P545 vs P544) e `builder.rs` (P535 vs P536), onde a documentação de coordenação podia ser mais explícita.
- **Achado funcional de P551 corrigido em P552:** a numeração duplicada de notas de rodapé em `#set page(columns: 2)` e o posicionamento lado a lado em `#columns(2)` foram corrigidos.
- **Achado de P553/P554:** o documento multi-página em `#set page(columns: 2)` foi fechado. P553 corrigiu a geometria de colunas (5 vs 3 páginas); P554 alterou a fonte padrão do cristalino para `FreeSerif` (3 vs 2 → 2 vs 2 páginas). O item de fonte padrão foi registado separadamente do fallback ausente de P538e.

---

## 4. Próximos passos sugeridos

1. **Corrigir conclusões dos 5 relatórios da Categoria A** para incluir o comando/número/comparação vanilla que sustenta o fecho.
2. **Completar justificativas dos scope-outs em P514** (e referências cruzadas noutros relatórios) com a razão arquitectural PDF-only.
3. **Monitorar o novo default `FreeSerif`** — se em algum ambiente de testes/ci a fonte não estiver instalada, o shaper regredirá para sans-serif e o caso de teste de P553 voltará a 3 páginas. Considerar adicionar `fonts-freefont-ttf` como dependência do ambiente de testes.
