# Estado do projecto typst-crystalline — handoff para novo chat (pós-P974)

**Data:** 2026-08-05
**Último passo fechado:** P974 (altura do símbolo `√` escalando com o radicando) + confirmação de
auditoria externa independente (8 rodadas, 2026-08-03 a 2026-08-05) fechando zero divergência de
conteúdo nova.
**Handoff anterior:** `00_nucleo/handoff-novo-chat-p927.md` (cobre até P927 — este documento cobre
P928 em diante).
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/
release/typst` (vanilla, 0.15.0/0.15.1) — **sempre confirmar identidade por string distintiva
(`Typst compiler (crystalline)` vs `The Typst compiler`), nunca só pelo nome do caminho** — lição
cara de P934 (uma investigação inteira baseada num binário mal identificado).

---

## Como esta linha de trabalho funciona (reforçado nesta fase)

Três frentes sequenciais, cada uma fechando com prova mensurável, não impressão:

1. **P928-943 — performance de fallback de fontes**: distância ao vanilla caiu de ~25× para
   paridade prática (~1.2-1.3×), causa raiz de cada camada (I/O, coverage, glyph verification
   redundante) identificada e corrigida uma a uma.
2. **P944-974 — paridade de geometria matemática e exportação PDF**: motivada por relato directo
   do dono ("os glifos saem mal escritos"), depois validada por **auditoria externa independente**
   em 8 rodadas — cada achado corrigido com causa medida no vanilla real, nunca por suposição.

### Regras e ferramentas novas desta fase

- **`ADR-0126`** (emendada em P956) — modo verboso (`Tm`/`q`/`cm`/`Q`/`cs`/`scn`/`Tr`, espelhando o
  vanilla) é o **caminho de produção padrão**; modo compacto (formato original do Passo 20) é
  **flag `--compact`** opcional, validado por decalque contra o verboso, não descartado.
- **`ADR-0127`** — critério formal de quando parar para confirmação do dono: mudança de contrato
  público (campo/método novo, assinatura) ou de comportamento por defeito ⇒ paragem obrigatória;
  correcção de fórmula/tabela interna sem mudar contrato ⇒ fluxo contínuo (L0 primeiro, sem parar).
- **`tools/geometry/compare.py`** (P948, estendido em P951/956) — comparador geométrico glifo a
  glifo entre PDFs, com classificação `sistemático`/`pontual`/`indeterminado` por secção. **Lição
  crítica registada no seu README**: mede distância entre bounding boxes de peças, não continuidade
  de tinta — um espaçamento grande entre bboxes pode ser desenho correcto da fonte (P949), e
  divergência de conteúdo (nomes diferentes, `lr()` literal) pode inflar a mediana sem ser
  problema geométrico (P952/966).
- **`01_core/src/testing/math_oracle.rs`** (P969) — funções puras, transcrição literal de fórmulas
  do vanilla já confirmadas, com `file:line`, usadas em teste. **Veredicto honesto da própria
  Fase C**: acelera consolidação/rastreabilidade, não acelera a investigação em si — cada fórmula
  nova ainda exige ler o vanilla e medir a fonte.
- **Documento de teste de 30 secções** (`.typ/typst-math-comprehensive-test.typ`) — precisa de ser
  revalidado visualmente (não só benchmark) depois de qualquer passo que toque `font_metrics`/
  `covering()`/`Coverage`/geometria matemática. A frente de performance (P925-943) esqueceu disto
  uma vez e uma regressão real ficou sem detectar até a auditoria externa a achar.

---

## Linha do tempo resumida: P928 → P974

### P928-943 — performance de fallback de fontes
P928 tentou a Opção 1 (pré-computar tudo) e violou a instrução (testou algo diferente do pedido,
sem gate) — revertido. P929 testou as duas ideias certas (scan paralelo, cache em disco) e
descartou as duas com números. P930 confirmou custo de arranque proibitivo para índice invertido.
P931 testou três braços de cache/lookup e nenhum cumpriu os critérios — limitação aceite,
correctamente registada. P932 achou que lazy + shaper optimizado resolvia sem regressão — mas P933
descobriu, via `mutool trace`, que a correcção introduzira falso positivo real (glifo `.notdef`
por confiar em bitmap aproximado sem verificação). P934 resolveu uma contradição de 27× entre duas
medições — o "vanilla" de um passo anterior era na verdade o cristalino mal identificado. P935
tentou implementar sem seguir o próprio plano (regrediu o caso comum) — revertido em P936, que fez
o estudo holístico correcto: mmap + coverage exacta são interdependentes. P937 implementou as duas
juntas mas regrediu o caso comum (eager demais). P938 corrigiu para lazy+exacto — caso comum
recuperado, fallback preservado. P939 investigou resíduo de I/O duplicado — pequeno, não a causa
principal. P940 achou a causa real do `render_ms` alto em emoji (subsetter falha em CBDT, embute
fonte inteira). P941 substituiu por glifos-bitmap-como-imagem — tamanho de PDF de ~135× maior para
**menor** que o vanilla, cor corrigida de brinde. P942 resolveu o mistério final: re-verificação
redundante de `glyph_index` (resíduo de quando a coverage era aproximada) custava 700ms;
removida, distância final ~1.2-1.3×. P943 confirmou que o resíduo é overhead de processo comum aos
dois lados, não vale mais esforço.

### P944-947 — causa raiz da geometria "mal escrita" + falsas pistas refutadas
P944 achou a causa real: cristalino nunca implementava o `show_set` do vanilla que força toda
equação a usar `New Computer Modern Math` — sem isso, matemática herdava a fonte de texto do
documento, com tabela MATH stub. P945 corrigiu o assembly de delimitador (grade 30% mais curta por
descida de nível `Display→Text` errada). P946 refutou duas suspeitas visuais (artefacto de baixa
DPI, não bug real) com prova de charstring byte-a-byte, e corrigiu o `ToUnicode` das peças de
assembly (divergência deliberada registada). P947 confirmou, ao nível de content stream bruto, que
a suspeita original de "duplo desenho" nunca existiu.

### P948-956 — ferramenta de medição + arquitectura de exportação
P948 construiu `compare.py`. P949 refutou um "gap" na chave como desenho correcto da fonte
(idêntico ao vanilla). P950 corrigiu nomes de fonte genéricos (`CrystallineFontN`) para nomes reais
no PDF. P951 estendeu `compare.py` com classificação sistemático/pontual. P952 decompôs um padrão
"sistemático" em **cinco causas reais** (espaçamento equação-equação, fracção display, operadores
grandes não esticados, ancoragem de grelha, centragem de tinta) — achou de brinde um bug de
subsetting que tornava somatórios invisíveis no PDF. P953 testou capacidades reais (rotação, escala,
cor) — achou e corrigiu um bug real de `scale()` (factor posicional ignorado). P954 rastreou a
origem da decisão "verboso primeiro" (conversa directa, não passo) e confirmou sem colisão de ADR.
P955 encontrou a origem do formato actual (Passo 20, nunca reconsiderado). P956 implementou o modo
verboso como novo padrão de produção, compacto como flag.

### P957-974 — geometria fina + auditoria externa
P957 corrigiu posição das peças de assembly (baseline no topo do slot em vez do fundo). P958-968
corrigiram, um a um, achados da auditoria externa: nomes gregos literais, limite superior de
operador grande (P959 só corrigira o inferior; P963 achou a causa real — `is_text_like` não
considerava bases esticadas), operador diferencial `dif` sem wrapper upright, variantes gregas
(`ϵ`/`ϑ`/`ϱ`/`ϕ`) fora do plano itálico, conteúdo de função de utilizador (`bra`/`ket`) sem default
matemático (P964→966), e o achado mais desproporcional: 74% do texto com contorno falso de negrito
(`Tr 2`) por interacção entre dois passos correctos isoladamente (gate de faux-bold de P139 +
`weight: 450` de P944). P965 formalizou o critério de gate (`ADR-0127`). P969 construiu o oráculo
de fórmulas. P970-972 corrigiram índice/altura/posição de raiz e parênteses desalinhados em
fracção — P972 achou que o bug só se manifestava com fonte real, nunca com `FixedMetrics` (lição
que motivou P973, varredura sistemática de catch-alls silenciosos sobre `FrameItem` —
resultado negativo limpo, um suspeito fora de escopo catalogado). P974 fechou a altura do símbolo
`√` (causa dupla: gap de Display + short-fall indevido), confirmando que o achado original "9.1"
nunca foram dois bugs, era um só.

---

## Estado actual — o que ficou aberto

Nenhum item crítico. Todos os seguintes são resíduos pequenos, catalogados com `file:line` ou razão
de adiamento clara, nenhum bloqueante:

1. **`sub_frame.rs:306`** (P973) — mesma classe de bug de P972 (catch-all `_ => {}` ignorando
   `Glyph`/`Line`), fora do directório-alvo da varredura — altura de sub-frame subestimada para
   conteúdo math. Precisa de reprodução própria.
2. **Itálico correction de bases `Text` com IC>0** (P971) — só bases `FrameItem::Glyph` (esticadas)
   foram corrigidas; ∫ inline e letras itálicas com IC pequena ficam residual. Precisa de acessor
   char→gid no trait (contrato adicional).
3. **Consolidação de tabelas de símbolos duplicadas** (`ident_to_unicode` vs `SYM_SIMPLE`, P958) —
   não bloqueante, união cobre o uso comum.
4. **Decorador de posição residual, secção 10** (P961) — linha de underline, posição vertical da
   legenda face à chave — fora do escopo de tamanho/itálico que P961 resolveu.
5. **`binom` com mecânica de matriz em vez de frac-like** (P946) — passo entre linhas 13.16pt vs
   15.0pt do vanilla; aguarda o passo de fracções ganhar a descida por nível para migrar no mesmo
   movimento.

---

## Ficheiros/mecanismos centrais mencionados com frequência nesta fase

- `03_infra/src/world.rs` — `preload_coverage_if_needed` (P927/938, scan condicional), coverage
  exacta lazy (P938/942).
- `03_infra/src/shaper.rs` — `face_covers_char` reintroduzido (P933), depois eliminado como
  redundante com coverage exacta (P942).
- `03_infra/src/export/{builder,bitmap_glyphs,stream}.rs` — glifos bitmap como imagem (P941), nomes
  reais de fonte (P950), modo verboso/compacto (P956).
- `01_core/src/engine/math/layout/{root,frac,attach,assembly,mod}.rs` — `show_set` de fonte
  matemática (P944, `engine/layout/equation.rs`), assembly (P945/957), fracção (P952/972),
  operadores grandes (P952/959/963/971), radical (P970/974).
- `01_core/src/entities/layout_types.rs` — `faux_bold_stroke_pt` com limiar `weight ≥ 600` (P968).
- `tools/geometry/compare.py` + `README.md` — ferramenta e lições de método.
- `00_nucleo/adr/typst-adr-0126-*.md` / `typst-adr-0127-*.md` — modo verboso/compacto; critério de
  gate.

---

## Recomendação para o próximo chat

1. Nenhuma acção urgente — os 5 itens residuais podem ser retomados em qualquer ordem, ou deixados
   como estão indefinidamente (nenhum afecta correcção visível na maioria dos documentos).
2. Se retomar geometria matemática: usar `tools/geometry/compare.py` primeiro para triagem
   objectiva, não esperar por achado manual/auditoria externa de novo.
3. Se aparecer um "bug que não reproduz em teste de unidade": verificar primeiro se o item é
   `Text` ou `Glyph`/`Line` no caminho de produção real (lição de P972/973) — `FixedMetrics` mascara
   uma classe inteira de bugs.
4. Considerar, nalgum momento, promover PDF tagueado/acessibilidade de scope-out para prioridade
   activa — decisão de produto, não técnica, já isolada como eixo próprio por `ADR-0126`.
