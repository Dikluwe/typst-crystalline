# Estado do projecto typst-crystalline — handoff para novo chat (pós-P927)

**Data:** 2026-07-28
**Último passo fechado:** P927 (fallback lazy de fontes UTF-8/CJK/emoji disparado só quando
necessário) — **com uma pendência**, ver "Estado actual", item 1.
**Handoff anterior:** `00_nucleo/handoff-novo-chat-p884.md` (cobre até P884 — este documento cobre
P885 em diante, não substitui os anteriores).
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/
release/typst` (vanilla, **0.15.0**) — mesma disciplina de sempre: nunca usar binário do sistema.

---

## Como esta linha de trabalho funciona (reforçado nesta fase)

Mesmo fluxo geral (sonda → implementação → validação → relatório), com três mecanismos novos
introduzidos e usados repetidamente ao longo de P885-927:

1. **Protocolo de dois agentes** (introduzido P898): para qualquer correção de geometria/cálculo,
   um agente escreve os testes sem ver a implementação de referência, outro implementa sem poder
   editar os testes. Pegou pelo menos um bug real que a disciplina sozinha não teria pego (P898,
   truque de linearidade `dy` que funcionava no eixo X mas não no Y).
2. **Revisão do orquestrador**: depois dos dois agentes, uma terceira leitura cética, cético
   testando um caso composto não coberto pelos testes originais. Pegou achados novos em P898, P901,
   P906, P908, P916.
3. **`ADR-0123` (geometria tipográfica) + `L11` (Tekt, `LESSONS.md`)**: qualquer fórmula de
   posição/gap/offset em `math/layout/` tem de ser **portada literalmente da leitura do
   código-fonte do vanilla**, não inventada e validada depois — e toda alegação de correção
   geométrica precisa de **recibo** (comando exacto, números, comparação), não adjetivo ("escala
   adequadamente" não basta). A ausência desta disciplina em P912-914 custou 3 passos extra
   (P916/917) para produzir a prova que devia ter vindo junto.

### Regras reforçadas nesta fase, além das já conhecidas

1. **Benchmark de regressão é sempre `depois/antes` nos 7 cenários canônicos, nunca
   `cristalino/vanilla`.** P922/923 mediram `cristalino/vanilla` por engano — não deteta se *o
   próprio passo* introduziu regressão, só se o cristalino é mais rápido que o vanilla (pergunta
   diferente). Corrigido só depois de pedido explícito na revisão.
2. **Nunca commitar passos diferentes juntos.** P922/923/924 foram integrados num commit único
   (`da18ea9f3`) — quebra a possibilidade de `git bisect` isolar qual dos três causou um problema,
   se aparecer depois. Já tinha sido estabelecido em P899/909/918 e foi esquecido aqui.
3. **L0 actualizado é parte de fechar o passo, não um adendo posterior.** P927 escreveu código
   novo em `world.rs`/`main.rs` sem estender os L0s correspondentes antes — só descoberto porque a
   revisão perguntou explicitamente. `crystalline-lint` **não pegou isto sozinho**: hash bate
   quando o ficheiro L0 e o `.rs` estão sincronizados um com o outro, mesmo que o *conteúdo* do L0
   esteja desactualizado face ao código. **Isto é uma lacuna real da Trava Arquitectural, não só um
   lapso — vale considerar registar formalmente.**
4. **Nunca assumir número de ADR livre sem varrer o directório real.** Colisão real encontrada em
   `0112` (proposto por mim, já ocupado por `rust_decimal`) — corrigida em P910, que também achou
   `0026` duplicada (não resolvida), `0056` ausente (não resolvido), e uma reserva obsoleta em
   `0063` (`DEBT-56`/column flow já tinha fechado sob `ADR-0078`, a reserva nunca foi limpa).

---

## Linha do tempo resumida: P885 → P927

### P885–P891 — abertura da frente de matemática, achados de fonte MATH
Comparação inicial de PDFs (cristalino vs vanilla) revelou divergências sistemáticas em símbolos,
espaçamento e delimitadores. P890/891/893 diagnosticaram e corrigiram a causa raiz repetida:
`covering()`/resolução de fonte não priorizava a fonte MATH (`math_kern`, depois generalizado,
depois `math_constants` — Fase B desta última ficou parada, sem confirmação, até P917).

### P892–P899 — símbolos, funções nativas, espaçamento
`italic_correction` testado e refutado como causa do gap de `i²` (P892). Catálogo grande de funções
matemáticas nativas em falta implementado em lote (P899: acentos, delimitadores, estilos de fonte).

### P900–P909 — correções pontuais + primeira atomização
`Str+Content` (P900), barra de radical mal posicionada (P901), mapeamento de `partial` (P902),
espaçamento de texto citado (P903), três achados pequenos agrupados (P904), esticamento horizontal
de glifo — cadeia de 5 bugs pré-existentes descoberta ao confirmar visualmente (P906), offset
infinito sob `width/height: auto` (P895-898), e a primeira rodada de atomização
(`accent`/`cancel`/`underover`/`op` para fora de `math/layout/mod.rs`, P909).

### P910 — reconciliação de numeração de ADRs
Achado fora da frente de matemática: 6 ADRs sem número (`0119`, `0121`, `0122`, `0124`, `0125`, mais
a nova `0123`), uma colisão real (`0112`), uma reserva obsoleta (`0063`). `ADR-0123` (geometria
tipográfica) nasceu deste processo — nomeia formalmente a regra de "portar fórmula literal do
vanilla" que já estava a ser seguida informalmente desde P901.

### P911–P918 — auditoria sistemática e "exocitose" geométrica
P911 auditou os 6 módulos de `math/layout/` já atomizados mas nunca verificados formula-a-fórmula
contra o vanilla — encontrou 3 achados grandes (não o resultado "limpo" esperado): `covering()`
ainda preferia fonte de corpo para glifos comuns (parênteses); `assembly.rs` nunca repetia peças
extensoras; `attach.rs` usava offsets fixos onde o vanilla computa valor adaptativo. P912-914
corrigiram os três, mas **sem prova geométrica** — só reportagem em prosa. P916/917 forçaram essa
prova a posteriori, e P917 encontrou a causa real (confusão entre `advance`/`hor_advance`) e, de
caminho, fechou a Fase B de `math_constants` que estava parada desde P893. P918 extraiu o núcleo
geométrico genuinamente partilhado entre módulos (`stack_tight_above`, `grid_delim_target_du`),
com prova de PDF byte-idêntico.

### P919–P924 — última rodada de achados residuais
Cada um começou com uma hipótese e encontrou causa mais funda: P919 (desalinhamento entre
elementos — bug de omissão em `apply_axis_offset`, não "falta de eixo comum"); P920 (gap de
`underover` — mecanismo real é acento, não `LineItem`; bloqueio arquitectural de sinal descoberto e
destacado para P922); P921 (assembly "incompleto" em matrizes — causa real era `layout_text_node`
usando métricas globais erradas para QUALQUER texto em modo matemático, achado muito mais amplo que
o nome sugeria); P922 (extensão de contrato `text_ink_bounds_signed`); P923 (resíduo de ~4pt/linha —
três causas compostas, fechado com `Δ=0` exacto contra o vanilla); P924 (flake não reproduzido, 22
execuções, candidatos registados sem correcção forçada).

### P925–P927 — nova frente: performance de fallback de fontes
Achado lateral de P923 (outlier `05-utf8`, 25.91×) virou frente própria. P925 diagnosticou:
cristalino faz parsing lazy completo de todas as fontes do sistema no primeiro carácter não
coberto; vanilla pré-computa no arranque. Opção 1 (pré-computar sempre) resolveu o caso CJK/emoji
mas **regrediu o caso comum em 1.12-1.48×** — só descoberto porque a revisão pediu para medir o
benchmark canônico, não só o caso que motivou a investigação. Opção 5 (thread de fundo) testada e
descartada com números (não ajuda — o fallback é tipicamente necessário antes da thread terminar).
Opção 6 (escanear o source primeiro, só disparar o scan caro se houver bloco não coberto)
implementada em P927: zero regressão no caso comum, resolve o "pagar sem necessidade", mas **não
reduz o custo absoluto (~7s) para quem de facto usa CJK/emoji**.

---

## Estado actual — o que ficou aberto

### 1. Pendência imediata — L0 de P927 não sincronizado
Confirmado pelo dono: `system-world.md` e `wiring.md` não foram estendidos antes do código de
P927 (`preload_coverage_if_needed`, `embedded_coverage_union`, `source_text_nodes`, chamada em
`main.rs`). `crystalline-lint` passou porque os hashes batem entre si, não porque o conteúdo está
correcto — a Trava Arquitectural não pegou isto sozinha. **Primeira coisa a confirmar/fechar no
próximo chat**, antes de qualquer trabalho novo.

### 2. Custo absoluto de fallback CJK/emoji — não resolvido, decisão consciente de parar
P927 resolveu "quem não precisa não paga"; não resolveu "quem precisa paga caro" (~7s ainda para
`utf8-cjk`/`utf8-emoji`). Duas opções candidatas, não tentadas, registadas em P926/927:
- **Paralelizar o scan** quando dispara (múltiplos slots de fonte em paralelo).
- **Cache em disco** da coverage entre execuções (evita reparsear `cmap` a cada corrida).
Decisão explícita de parar aqui e não abrir mais uma rodada agora — ver conversa que motivou este
handoff.

### 3. Achados residuais menores da frente de matemática, todos registados, nenhum bloqueante
- `flattened_accent_base_height` (P922): campo lido da fonte, sem consumidor ainda (variante
  "flattened" de acento não implementada).
- Inferência refutável em P922: o gap de acentos esticados (`hat(a+b)`) usa o descent com sinal do
  carácter base original, não do resultado esticado — marcado como candidato a revisão se medição
  futura contra o vanilla mostrar desvio.
- `math_leading` (lido da fonte) vs `DEFAULT_ROW_GAP` (constante fixa do vanilla) para `layout_grid`
  multiline (`&`/`\\`) — mantido como estava; só `matrix.rs`/`cases.rs` foram corrigidos para usar
  `0.2em` do estilo exterior (P923).

### 4. Reconciliação de ADRs (P910) — parcialmente confirmada
- `ADR-0112` a `0125` reconciliadas e escritas nesta conversa.
- **"Decisão nova obrigatória"** — terceira regra citada por `ADR-0119`/`0121`/`0122`/`0124`/`0125`,
  **nunca localizada**. Mais antiga que `2026-07-03` (citada como já existente nessa data). Se
  aparecer, desloca toda a numeração desta faixa.
- **`ADR-0026` duplicada** — não confirmado se é revisão `-R1` legítima ou conflito real.
- **`ADR-0056` ausente** — não confirmado se é reserva ou lacuna genuína.
- **Varredura sistemática de `"Prompt L0 — ADR:"`** (P910, ponto 8, `grep -rl` em todo
  `00_nucleo/`) — **nunca confirmada como executada**. Só foram reconciliados os documentos que o
  dono foi enviando manualmente; pode haver mais.
- `README.md`/índice de `00_nucleo/adr/` — ainda desactualizado (parava por volta de P441-443
  antes desta reconciliação; não confirmado se já foi actualizado com as entradas novas).

---

## Ficheiros/mecanismos centrais mencionados com frequência nesta fase

- `01_core/src/rules/math/layout/` — agora **totalmente atomizado**: `frac.rs`, `root.rs`,
  `underover.rs`, `accent.rs`, `cancel.rs`, `op.rs`, `attach.rs`, `matrix.rs`, `cases.rs`,
  `delimited.rs`, `stretchy.rs`, `assembly.rs`, mais núcleo geométrico partilhado extraído em P918
  (`stack_tight_above`, `grid_delim_target_du`, ambos em `mod.rs`).
- `03_infra/src/font_metrics.rs` — `covering()` prioriza fonte MATH na raiz desde P912;
  `text_ink_bounds_signed` (P922); `math_constants_from_face` (Fase B fechada em P917, depois de
  parada desde P893).
- `03_infra/src/world.rs` — `preload_coverage_if_needed`/`embedded_coverage_union`/
  `source_text_nodes` (P927, **L0 pendente**).
- `01_core/src/entities/math_constants.rs` — 15+ campos agora, incluindo
  `superscript_shift_up_cramped` (P915), `accent_base_height`/`flattened_accent_base_height`
  (P922).
- `00_nucleo/adr/` — `ADR-0107` (paridade é língua, não mecânica), `ADR-0108` (medir antes de
  decidir), `ADR-0119`-`0125` (reconciliadas nesta fase), `ADR-0123` (geometria tipográfica —
  governa qualquer trabalho futuro em `math/layout/`).
- `Tekt/LESSONS.md`/`LESSONS_pt.md` — `L11` (alegações atestadas), inspirado no Open Knowledge
  Format (`GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md`), nomeando a exigência de
  executor/recibo/atestador para qualquer alegação da camada de oráculo.

---

## Recomendação para o próximo chat

1. **Confirmar/fechar a pendência de L0 de P927 primeiro**, antes de qualquer trabalho novo — não
   deixar acumular mais uma pendência não resolvida, como quase aconteceu com o `README.md` de
   ADRs.
2. Decidir se vale abrir P928+ para o custo absoluto de CJK/emoji (paralelizar scan; cache em
   disco), ou se fica registado como aceite por agora.
3. Executar a varredura sistemática de `"Prompt L0 — ADR:"` (P910, ponto 8) pelo menos uma vez —
   nunca foi confirmada, e é o tipo de coisa que só vai continuar a aparecer por acaso se não for
   feita de propósito.
4. Resolver `ADR-0026` (duplicada) e `ADR-0056` (ausente), e continuar a busca por "Decisão nova
   obrigatória".
5. Considerar se `crystalline-lint` deveria ganhar uma verificação nova: hash sincronizado não é o
   mesmo que conteúdo de L0 cobrindo o código real — a lacuna que permitiu a pendência do item 1
   pode voltar a acontecer enquanto isso não for verificado mecanicamente.
