# Proposta de pacote: gradiente no preenchimento de texto

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo o pacote”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. Os L0s são a fonte do código.

## Medição

Crystalline compilado em release do HEAD `88672ddc7d4e50019564ba14b7a6476e4cd35209`, com alterações não commitadas apenas em `00_nucleo/adr/ORDER.md`, ADR-0109, ADR-0127, `CLAUDE.md` e `CONTRIBUTING.md`. SHA-256 `6caac5f18e8a1ea5c6cb342f82a1e45b7cac29f89069f18992f28db808d3a261`. Vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`, igual ao manifesto. Saída SVG, 16:30 de Brasília. Página 120 × 60 pt.

| Caso | Vanilla | Cristalino |
|---|---|---|
| `#text(fill: gradient.linear(red, blue))[ok]` | gradiente | erro `text(fill:) espera color, recebeu gradient` |
| `#set text(fill: gradient.linear(red, blue))`, idem radial e conic | gradiente | compila, texto preto |
| `#set text(gradient.linear(red, blue))` posicional | gradiente | erro `unexpected argument` |
| `set` dentro de bloco, `show strong: set text(…)`, texto sólido aninhado | gradiente só no escopo | preto no lugar do gradiente; o sólido aninhado funciona |
| `rotate` e `box` sob `set` | gradiente | preto |
| `relative: "self"`, construtor | erro `gradients and tilings on text must be relative to the parent` | erro `text(fill:) espera color…` |
| `relative: "self"`, `set` | mesmo erro | compila, texto preto |
| `relative: "parent"`, construtor | gradiente | erro `text(fill:) espera color…` |
| `$x^2$`, `$a/b$` | glifos e barra com gradiente | preto |
| `underline` | glifos com gradiente, traço com `stroke="url(…)"` | preto |
| Tiling em texto | tiling | `set` em silêncio, construtor com erro |
| Controles: `red`, `3`, `none` | sólido, erro, erro | iguais nos três |

Uma sonda de PNG com o binário anterior (`69eede6e…`, de 19/11) mostrou pixels coloridos só no vanilla. Deve ser remedida no RED. PDF também: uma contagem preliminar não distinguiu streams comprimidos.

## Causa

- O construtor rejeita `Gradient` e `Tiling`: `01_core/src/compiler/stdlib/text/constructor.rs:75`.
- O `set` os descarta: `01_core/src/compiler/eval/rules/set/text.rs:99`. O L0 registra esse corte como parcial. A seleção posicional só aceita `Value::Color` (linha 32).
- `Style::Fill(Color)` (`01_core/src/entities/style.rs:45`), `StyleDelta.fill` (`style_chain.rs:55`) e `TextStyle.fill: Option<Color>` (`layout_types.rs:180`) não transportam gradiente. `FrameItem::Line.color` (`layout_types.rs:435`) também não.
- O vanilla resolve `relative: auto` em texto como `Parent` (`lab/typst-original/crates/typst-library/src/visualize/gradient.rs:1046`) e rejeita `self` no parse (`text/mod.rs:303`).
- Os exportadores PDF, SVG e raster já pintam gradiente em `Shape` e já acompanham `paint_root`. Inferência: essa raiz basta para resolver `Parent` em glifos, sem campo novo. Refutaria: um caso em `box`, `rotate` ou grid cuja raiz em glifos difira da do vanilla.

## Escopo proposto

**Dentro:** gradientes linear, radial e conic em `text.fill`, por construtor, `set` nomeado, `set` posicional e `show … set`.

| Portador | Mudança |
|---|---|
| `Style::Fill` | `Color` → `Paint` |
| `StyleDelta.fill`, `StyleChain::fill()` | `Option<Color>` → `Option<Paint>` |
| `TextStyle.fill` | `Option<Color>` → `Option<Paint>` |
| `FrameItem::Line.color` | `Option<Color>` → `Option<Paint>`, para decoração e barras de matemática |
| `Value`, `Color`, `Gradient`, `FramePaint`, `Shape` | sem mudança |

A rota `set` transporta `Value::Gradient` pelo canal `text.fill`, como já faz com cor. `Paint` conserva a igualdade de `TextStyle`; `FramePaint` não tem `PartialEq`.

**Efeitos de compatibilidade.**
- `set` com `relative: "self"` passa de preto silencioso a erro, como no vanilla. Deixa de aceitar uma entrada que hoje compila (§2.4).
- O gradiente silencioso passa a ser pintado. Documentos com ele mudam de aparência.
- A mensagem de `relative: "self"` no construtor muda para a do vanilla.

**Fora:**
- Tiling em texto: exige célula preparada pelo layout (`FramePaint`). Mantém o comportamento atual nas duas rotas. Se `Paint::Tiling` chegar a um portador de texto por construção interna, deve falhar em teste.
- Ler `text.fill` em `context`: hoje falha também com cor sólida.
- `stroke` de texto, `highlight`, `underline(evade:)`, gradiente em HTML semântico.
- Novas capacidades dos exportadores: valem os espaços de cor e fallbacks já contratados para `Shape`, inclusive a marca de fallback do SVG.

Não há mudança de fase, de default nem de traits.

## Owners a atualizar

| L0 | Motivo |
|---|---|
| `00_nucleo/prompts/core/entities/paint.md` | **Primeiro.** Declara `Paint` só com `Solid` e `Copy`; o Rust tem três variantes e não é `Copy`. |
| `00_nucleo/prompts/core/entities/gradient.md` | Declara que `TextStyle.fill` permanece `Option<Color>`. |
| `00_nucleo/prompts/core/entities/style.md`, `00_nucleo/prompts/core/entities/style_chain.md`, `00_nucleo/prompts/core/entities/layout_types.md` | Portadores. |
| `00_nucleo/prompts/core/compiler/stdlib/text/constructor.md`, `00_nucleo/prompts/core/compiler/eval/rules/set/text.md` | Entrada, seleção posicional e erro de `self`. |
| `00_nucleo/prompts/core/compiler/layout/`: `decorations.md`, `mod.md`, `paragraph_paint.md`, `paragraph_session.md`, `raw.md`, `text/style.md` | Leitores de fill. |
| `00_nucleo/prompts/core/compiler/math/layout/`: `callbacks.md`, `cancel.md`, `external_bridge.md`, `line_decorations.md`, `underline.md` | Leitores de fill e linhas. |
| `00_nucleo/prompts/infra/export/`: `builder/resources.md`, `render.md`, `stream.md`, `stream/shape.md`, `stream/text.md`, `stream/text/products.md`, `stream/text/runs.md`, `svg.md`, `svg/glyphs.md`, `svg/paints.md` | Emissão e recursos. |
| `00_nucleo/prompts/infra/shaper/complete_session.md` | Chave: gradiente fica fora da chave limitada (frio), como Spot hoje. |

A lista vem da busca por leitores diretos. O inventário final é o que o compilador apontar ao mudar os tipos; um owner a mais com o mesmo papel é ajuste interno do pacote. Um novo contrato protegido reabre a aprovação. Owners de teste acompanham seus módulos. Um núcleo compartilhado para a raiz `Parent` em texto só se dois owners o consumirem de fato.

## Validação

1. RED antes do código: os casos da tabela, com asserção de pintura. `rc=0` não basta.
2. Morfologia e estilo: escopo aninhado, restauração, `show … set`, sólido sobre gradiente e vice-versa.
3. Geometria: `Parent` no nível da página, em `box`, `rotate` e grid; raiz e coordenadas comparadas ao vanilla, não só a presença de `linearGradient`.
4. Shaping: ligaduras, runs mistos e coalescência. Posições e avanços de glifo iguais aos do controle sólido.
5. PDF verbose contra vanilla (`/Pattern`, `cs/scn`, `Tr`, matrizes) em PDF descomprimido; `--compact` contra verbose, por sobreposição.
6. SVG e PNG: cor por posição contra vanilla. Conic segue o orçamento de erro já contratado para `Shape`.
7. Reúso: gradientes distintos não compartilham produto de shaping, memo físico ou recurso PDF.
8. Controles: cor sólida, Spot, `raw`, `highlight` e tiling sem mudança. PDF sólido sem novos recursos de padrão.
9. Suíte completa, clippy, fmt, doc e linhagem nas duas direções.

## Ordem

`paint.md` → demais L0s e selos → RED → entidades → eval → layout e matemática → PDF, SVG e raster → validação. Se a inferência sobre a raiz `Parent` falhar, parar e voltar com a evidência: um campo novo em `TextStyle` ou nos itens de glifo muda o escopo.

## Execução

**24 de novembro de 2026, 17:19 de Brasília.** Árvore não commitada sobre `88672ddc7d4e50019564ba14b7a6476e4cd35209`. Binário de debug medido: SHA-256 `06bb6a34527a94c4f2b1d6187360e52ee9f42ef3c5c95606a666c2c82bd231e5`. Lista de arquivos em `git diff HEAD --stat`, mais este documento e `00_nucleo/prompts/_nuclei/text-gradient-paint.toml`.

- **RED:** 5 casos de CLI falharam antes do código. **GREEN:** 7 de 7 depois, com os dois de geometria/PNG e identidade acrescentados no fim.
- **Hipótese da raiz `Parent`:** confirmada nos casos medidos. Nenhum campo novo foi criado. O estado de raiz é privado de cada exportador.
- **Suíte:** 10.913 testes passaram, zero falhas, sete ignorados.
- **Contra o vanilla, por pixel a 144 dpi:** linear, radial, ângulos, stops, box, rotate, grid, table, place, RTL, ligaduras, fração e decoração. PDF verbose com diferença máxima de 1/255 em todos. PNG e SVG iguais ou dentro do antialias, salvo os resíduos abaixo.

### Ajustes internos ao escopo

Owners além da lista: `subtree_session`, `footnote_flush`, `physical_memo`, `equation`, `frac`, `root`, `matrix`, `stdlib/mod`, `stdlib/text/mod`, `gradients/mod`, `pdf_tiling`, `stream/draw`, `stream/page` e `svg/geometry`. O papel é o mesmo: transportar, ler ou emitir a pintura. O teste de CLI pertence a `language_and_export_contract`.

A matemática ignorava até cor sólida declarada por `set text(fill:)`, a barra de fração e o traço da raiz eram sempre pretos, e `strike`/`overline` não herdavam a cor do `set`. O pacote exigia esses casos com gradiente, e o mesmo conserto vale para cor sólida. É convergência com o vanilla.

### Resíduos

- **Conic em PDF:** com `mutool` o resultado é igual ao do vanilla (diferença máxima 2). Com `poppler`, a malha Type 6 perde antialias nas bordas. O controle de forma tem a mesma causa.
- **Conic em SVG:** 84 pixels acima de 24/255, nenhum acima de 64.
- **Luma e CMYK em SVG:** usam o fallback já contratado para formas.
- **PNG de raiz e glifos extensíveis, texto em `box` e espessura de `strike`:** os resíduos aparecem também no controle sólido.
- **Tiling com texto dentro e caixa de página com bleed em PDF:** divergem igualmente no controle sólido.
- **`--compact`:** a cor de um run sólido vaza para o texto seguinte sem cor. A saída do HEAD é idêntica na sonda sólida, portanto é anterior a este pacote.

`cargo clippy -D warnings` e `cargo fmt --check` já falham no HEAD em arquivos que este pacote não altera ou em trechos que ele não toca. Os arquivos alterados não acrescentam falhas. A sonda de controle sólido produz SVG e PNG idênticos aos do HEAD.

## Correções da revisão

Em 24/11/2026, das 20:05 às 20:14 de Brasília, os binários antes e depois
da correção foram medidos a partir de versões não commitadas do pacote com base em
`88672ddc7d4e50019564ba14b7a6476e4cd35209`. Os recibos em
`lab/results/text-gradient-review-fix/` registram o `git diff HEAD --stat`,
o estado dos arquivos, os horários, as entradas e os SHA-256 dos binários.

- O `text(fill:)` usa a declaração mais próxima no construtor ou no `set`.
  Ao sair do escopo interno, os glifos voltam a receber a tinta externa.
- `strike` e `overline` herdam a tinta do texto que decoram. Runs inline de
  tintas distintas recebem traços separados, e o `stroke` explícito tem
  prioridade. A geometria de tinta única segue pelo coletor existente.
- As máscaras PNG de glifos transformados e de traços em gradiente cobrem
  apenas os limites do contorno no dispositivo. O gradiente e o clip continuam
  definidos nas coordenadas da página.

As dez sondas SVG em `red.json` e `green.json` comparam a tinta de cada glifo
ou traço com o Typst fixado. Antes, oito casos divergiam. Depois, os dez
coincidiram. Em `raster.json`, o texto rotacionado com gradiente em debug
passou de 8,169 s para 0,553 s a 72 ppi, pela mediana de três execuções.
A 144 ppi passou de um timeout de 30 s para 1,720 s. Os PNGs sólidos foram
idênticos. O PNG rotacionado com gradiente a 72 ppi diferiu em um pixel, com
delta máximo de 9/255. Os PNGs rotacionados com sublinhado em gradiente
coincidiram pixel a pixel a 72 e 144 ppi.

O build do workspace passou. A suíte executável e os doctests somaram
10.917 testes aprovados e sete ignorados. Os testes de `watch` passaram
com acesso a loopback, e as integrações que exigem `/dev/shm` usaram
o diretório temporário e o oráculo do manifesto, conforme a CI.
A linhagem reversa validou os 1.030 pares. O linter passou no checkout
principal com uma configuração temporária que exclui a outra worktree
local sob `.claude/`. O estado e os logs estão em `validation.json`.
