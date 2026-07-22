# Estado do projecto typst-crystalline — handoff para novo chat (pós-P798)

**Data:** 2026-07-21
**Último passo fechado:** P807 (fila de achados de P798 encerrada — P799–P807, ver tabela abaixo)
**Handoff anterior:** `00_nucleo/handoff-novo-chat-p762.md` (cobre até P762 — este documento cobre P763 em diante, não substitui o anterior)
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla Typst 0.15.0, rev `969087ec`)

---

## Como esta linha de trabalho funciona (igual ao handoff anterior, reforçado)

Mesmo fluxo: sonda → implementação → validação → relatório. Claude (eu) escrevo o prompt de cada passo; o utilizador leva para execução (Claude Code e, por vezes, outras ferramentas — ver aviso abaixo); o relatório volta para revisão.

**Aviso importante, novo nesta fase:** o utilizador confirmou que ferramentas diferentes (Gemini, Kimi Code) às vezes executam passos fora desta conversa, sem eu ter escrito o prompt. Isso já causou confusão real uma vez (P786/P786a — um relatório de reverificação apareceu sem eu saber da sua origem, e dois relatórios anteriores foram eliminados por decisão do utilizador fora desta conversa). **Se um novo chat receber um relatório sem ter enviado o prompt correspondente, perguntar a origem antes de assumir que segue a mesma disciplina desta conversa.**

### Regra nova reforçada nesta fase: nunca aceitar "corrigido"/"mecanicamente correto" sem execução mostrada

Ao longo de P763–P798, pelo menos **seis** relatórios inicialmente aceitos como fechados continham problemas que só apareceram ao exigir prova concreta:
- P763d/P774/P777/P783: "corrigido"/"correto mecanicamente" sem medição real — todos revertidos ao exigir evidência.
- P792 (1ª versão): implementação descrita sem nenhum comando/saída — duas suspeitas de hardcode confirmadas (uma real, uma refutada) só depois de exigir teste com casos não-triviais.
- P798 (1ª versão): 7 de 15 módulos "classificados" com testes genéricos (`Hello World`) que não exercitavam o módulo real — taxa de sinal subiu de 27% para 60% ao corrigir a metodologia.

**Lição operacional:** todo relatório de execução deve trazer comando exacto + saída literal (vanilla vs cristalino) para cada afirmação, e a contagem de testes da suíte `typst-core` deve ser mostrada e bater com o número de testes novos declarados. Relatórios sem isso devem ser questionados antes de aceitar.

---

## Linha do tempo resumida: P763 → P798

### P763–P771 — Download de pacotes → cadeia de bugs de layout (fechada)
Começou como "confirmar suporte a download de pacotes `@preview`". Validar `cetz` (pacote de desenho) revelou uma cadeia de nove passos de investigação: um bug de coordenadas em `place()` dentro de `align`/sub-frames (P763c–P763f), que por sua vez revelou que `Content::Shape` nunca tinha sido decidido como bloco ou inline desde a Fase 1 do projecto (Passo 76, arqueologia em P767), e que a mesma lacuna afectava `Content::Image` (P769). Terminou com correcção de clip de imagem (P771) e AE=0 confirmado por coordenadas em todos os casos testados. **Lição da cadeia:** nunca implementar um elemento novo sem confirmar contra o código-fonte do vanilla se é bloco ou inline — a pergunta é barata, o custo de não fazer foi de nove passos.

### P772 (série "a" até "y") — Varredura sistemática da stdlib
Classificação item a item de `lacuna-inventario` (lista congelada da lente, 400 itens), módulo a módulo. Achados maiores, todos fechados:
- `layout::grid::resolve`: `layout_place` duplicava origem quando aninhado em wrappers (P772g); `grid.header`/`grid.footer` nunca implementados como row-groups reais, corrigido com `split_header_footer` (P772i); alinhamento per-célula usava `Content::Place` num código órfão nunca commitado — revertido e reimplementado com `Content::Align`, o mecanismo real do vanilla (P772j).
- `foundations::scope`: fuga de âmbito em `CodeBlock`/`ContentBlock` (P772l); `cannot_mutate_constant` — nomes da stdlib podiam ser reatribuídos silenciosamente, corrompendo `calc`/`image`/etc. sem erro (P772n); mensagem de mutação de variável capturada (P772q); hint de subtracção em `unknown_variable` (P772r).
- `image::raster`/`svg`/`pdf`: DPI real de metadados EXIF/JFIF/PNG (P773); rotação EXIF via matriz PDF, não recodificação de pixels (P774, P776); formato inválido deixa de ser omissão silenciosa (P772p); suporte a SVG e a PDF-como-imagem avaliados e **rejeitados** por peso de dependência (`usvg`/`resvg`, `hayro`/`vello` — P772k, P781).
- `text::font`: colapso de espaço em fontes variáveis de peso alto — `advance()` não aplicava variação de eixo (P772o, Ubuntu Sans); causa distinta em Cantarell-VF — `glyph_to_nominal` usava face pré-instanciação enquanto `/W` usava pós-instanciação (P772u).
- `layout::frame`: decoração (sublinhado/tachado/overline) não propagava através de `layout_sub_frame` — 7 call-sites, corrigido (P772x); `place(float: true)` continua best-effort, não garantido (débito).
- `math`: espaçamento automático por `MathClass` nunca existia — implementado do zero, tabela THIN/MEDIUM/THICK (P772y); `math.class()` implementado; achados adjacentes: `MathIdent` bare não resolvia variável de utilizador (P780, fechava debt de P301); splice de `#expr`/field-access bare em modo math (P782).
- `table()`: header/footer estendido do mecanismo já validado em `grid()` (P772v).

### P785–P798 — Triagem em lote (15 módulos por vez) + achados de severidade "silêncio quando deveria errar"
P785 (lote 1): taxa de sinal real 27% (corrigida de uma primeira tentativa que classificou "100% mecânica" sem testar — 4 achados reais escondidos: highlighting de blocos raw, campos nativos de tipos, offset de coluna UTF-16). P786 (lote 2): taxa de sinal 80%, 12 achados reais, todos da categoria "aceita comportamento errado em silêncio" ou "função ausente". Todos os 10 candidatos de agrupamento de P786 foram corrigidos em P787–P797:
- P787: CSV rejeita linha malformada; `row-type:` aceita tipo, não string.
- P788: referências/citações inválidas erram (não mais "?" silencioso); bug genérico de posicionamento de `#link` corrigido (flip Y ausente em filhos de `FrameItem::Link`).
- P789: conflito de célula com header de tabela detectado; repeat-across-páginas continua deferido (mesmo débito de P772i); achado novo: `table.cell(x:,y:)` explícito é ignorado no placement (não corrigido, registado).
- P790: show-by-string (`#show "texto": ...`) implementado via splice; `#show page`/`#show par` com warnings correctos em vez de erro fatal.
- P791: `#show <label>: ...` implementado (`Selector::Label`); achado adjacente: label em nó de texto não indexado pelo introspector (não corrigido).
- P792/P792a: `layout()`, `text.lang`, `here().position()` implementados; suspeita de margem hardcoded em `layout()` **confirmada e corrigida**; suspeita de idioma fixo em `text.lang` **refutada**.
- P793: `#numbering()`, `enum` com `+`, warning hebrew-zero.
- P794: aspas duplas curvas por padrão; `#set smartquote(...)` conectado; validação de `quotes:`.
- P795: modificador `neq` em símbolos; resolução de símbolos bare (`arrow`, `dif`) em modo math.
- P796: `sys.version` Display correcto; `.at()`; `--version` do CLI mostra versão de paridade + hash de commit próprio (decisão do dono registada).
- P797: bold/italic sem efeito visual — causa real era CFF1 subsetting incompatível com `/CIDFontType0`+`Identity-H`; corrigido com fallback para embutimento integral + subtipo `/OpenType`.

P798 (lote 3, corrigido): taxa de sinal real 60% (corrigida de uma primeira tentativa com testes genéricos tipo "Hello World" para 7 de 15 módulos). 9 achados reais + 1 código morto confirmado, **nenhum corrigido ainda** — ficaram na fila.

---

## Estado actual — trabalho em aberto

### Achados de P798 (lote 3) — **TODOS FECHADOS em P799–P807 (2026-07-21)**

| # | Módulo | Achado | Estado |
|---|---|---|---|
| 4 | `utils::protected` | Representação de array de 1 elemento diverge (`(0,)` vanilla vs `(0)` cristalino) | **FECHADO P801** — vírgula final em `repr_value` |
| 5 | `utils::listset` | Falta warning de label não-anexada (`query(<lbl>)` sobre label órfã) | **FECHADO P802** — warning via Sink |
| 7 | `syntax::kind` | `#if true [Hello $x^2$]` — ordem/conteúdo do output completamente diferente do vanilla | **FECHADO P800** — causa real: baseline do math inline desalinhada (não era `#if`); residual `𝑥` vs `x` = itálico matemático (ver abaixo) |
| 8 | `visualize::curve` | Mensagem de erro da API diverge | **FECHADO P803** — `expected content, found {type}` |
| 9 | `visualize` | `#line(length: ...)` — argumento nomeado rejeitado | **FECHADO P804** — `length:`/`angle:` implementados |
| 10 | `text::lorem_` | Falta ponto final no output de `#lorem(n)` | **FECHADO P805** — byte-parity total via crate `lipsum` (nova dependência L1, whitelist); sub-passo **P805a** corrigiu ligaduras fi/ffi sem ToUnicode no embed integral CFF (bug pré-existente grave descoberto na validação) |
| 11 | `pdf::attach` | `#pdf.attach()` rejeitado, decisão de escopo pendente | **FECHADO P807 — Opção B (dono)**: scope-out mantido e formalizado como **DEBT-66** em `00_nucleo/diagnosticos/debt/DEBT.md` |
| 12 | `model` | `#par[...]` como função dá `unknown variable: par` | **FECHADO P806** — `native_par` registada |
| 13 | `math::attach` | Posicionamento de sub/superscript com `_`/`^` completamente quebrado | **FECHADO P799** — sub+sup empilham na mesma origem x |
| 6 | `utils::deferred` | Código morto confirmado — sem acção | mantido |

Relatórios (convenção — vivem em `diagnosticos/`): `00_nucleo/diagnosticos/paridade-producao-p799.md` … `paridade-producao-p807.md` + `paridade-producao-p805a.md` (P807 produziu só a entrada DEBT-66, por decisão do dono). Os duplicados em `materialization/` foram removidos por essa convenção (conteúdo preservado no commit `c98ffc8ac`).

**Em aberto a partir destes passos**: itálico matemático (P786 §7 — `x` vs `𝑥`, `αβ` vs `𝛼𝛽`) confirmado como causa distinta e **não corrigido** (candidato a passo próprio); warning vanilla "content labelled multiple times" (registado no L0 eval.md); `type_name()` "int" vs vanilla "integer" em mensagens de erro (registado em P806); nomes de tipo `str`/`integer` idem.

### Débitos grandes, decisão consciente de não implementar (peso de dependência ou escopo maior)
- Suporte real a SVG (`image::svg`) — nunca implementado, scope-out desde P772k.
- `#image()` com PDF como fonte — rejeitado por peso de dependência (`hayro`/`vello`), P781.
- Repeat-across-páginas de `grid.header`/`grid.footer`/`table.header`/`table.footer` — débito reafirmado pelo menos três vezes (P772i, P789).
- `#pdf.attach()` — scope-out mantido por decisão do dono em P807 (Opção B), formalizado como **DEBT-66** (`00_nucleo/diagnosticos/debt/DEBT.md`).

### Achados menores registados, sem correcção
- `table.cell(x:,y:,colspan:,rowspan:)` — posições explícitas ignoradas no placement (P789).
- Conflito de célula com **footer** (só header foi coberto).
- Avisos de depreciação de símbolos (`Deprecation`) — falta fonte de dados (P772l §2.6).
- Span em `Args` para 326 pontos em funções auxiliares + span por-argumento completo (~1200 pontos, débito desde P740C) — P772s.
- `place(float: true)` com decoração — best-effort, não garantido (P772x §5).
- `#label()` como função não intercepta show rules (só sintaxe `<lbl>` cobre) — P791 §6.
- Grupos de símbolos sem uso confirmado no corpus (`face`, `harpoon`, `triangle`, `clock`, `heart`, `person`, emoji) — scope-out desde P766.

### Observações nunca viraram passo (P786 §7)
- `#text(size:)` não aceito como argumento nomeado.
- Fontes embutidas falham em poppler (`Couldn't create a font`) — pode estar relacionado ao achado de P797 (CFF subsetting), confirmar se já resolvido incidentalmente.
- Itálico matemático extrai texto plano em vez de estilizado (`αβ` vs `𝛼𝛽`) — pode sobrepor-se ao achado #13 de P798 (`math::attach`), confirmar.
- Mensagens de erro misturam português e inglês.
- Paginação diverge em teste de header de tabela (5 vs 7 páginas) — provavelmente o mesmo débito de repeat-across-páginas.

### Varredura sistemática — progresso
Lista original (P772t): ~178 itens em ~66 módulos. P785 (15) + P786 (15) + P798 (15) = 45 módulos triados. **Restam aproximadamente 21 módulos não triados.**

---

## Numeração

P590 a P762 no handoff anterior. P763 a P798 nesta fase, com sub-numeração por letra quando o achado nasce de dentro de outro passo (ex: P772a–P772y nasceram da varredura iniciada em P772; P786a nasceu de uma reverificação de P786). Números "soltos" (P780, P781, ...) foram usados quando o utilizador pediu explicitamente para sair da cadeia de letras por ela estar longa demais — decisão registada na altura, não espontânea.

---

## Ficheiros/mecanismos centrais mencionados com frequência nesta fase

- `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` — lista congelada de `lacuna-inventario` (TSV), fonte de toda a série de triagem P765–P798.
- `01_core/src/rules/layout/sub_frame.rs`, `placement.rs` — mecanismo de composição de coordenadas entre sub-frames e o frame pai (P763f, P772g, P772x).
- `01_core/src/rules/eval/math.rs` — resolução de identificadores/callees em modo matemático (P780, P782, P795).
- `01_core/src/entities/scope.rs`, `rules/scopes.rs` — `Library` materializada com `global: Scope`, `base` real; distinção entre exclusão estrutural (`is_constant`) e flag por-binding (`captured_by`) — dois mecanismos irmãos, não intercambiáveis (P772n, P772q).
- `03_infra/src/export/builder.rs`, `images.rs` — exportação PDF; subsetting CFF1/CID-keyed (P797); clip de imagem (P771); matriz de orientação EXIF (P776).
- `03_infra/src/font_metrics.rs`, `shaper.rs` — aplicação de variação de eixo em fontes variáveis (P772o, P772u).

---

## Recomendação para o próximo chat

1. **A fila de achados de P798 está encerrada (P799–P807, todos tratados em 2026-07-21).** O próximo trabalho de fundo é **continuar a triagem (lote 4, ~21 módulos restantes)** da lista congelada.
2. **Itálico matemático (P786 §7) é o maior residual aberto desta fase**: confirmado em P799/P800 como causa distinta (selecção de fonte/estilo math — `x` vs `𝑥`, `αβ` vs `𝛼𝛽`); afecta todo o output math. Candidato a passo dedicado, provavelmente antes ou durante o lote 4.
3. **Manter a disciplina de execução mostrada** (comando + saída real, contagem de testes batendo) — não relaxar mesmo que o volume pareça convidar a atalhos.
4. **Perguntar a origem de qualquer relatório inesperado** antes de aceitar, dado o padrão já confirmado de execuções paralelas fora desta conversa.
