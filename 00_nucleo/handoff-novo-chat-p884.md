# Estado do projecto typst-crystalline — handoff para novo chat (pós-P884)

**Data:** 2026-07-24
**Último passo fechado:** P884 (compressão de content streams — frente 1 de performance)
**Handoff anterior:** `00_nucleo/handoff-novo-chat-p810.md` (cobre até P810 — este documento cobre P811 em diante, não substitui os anteriores)
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla, **0.15.0, `969087ec`** — usar sempre este, não um binário do sistema; um passo, P870, usou por engano um binário do sistema numa primeira medição e teve que refazer)

---

## Como esta linha de trabalho funciona (reforçado desde os handoffs anteriores)

Mesmo fluxo: sonda → implementação → validação → relatório. Claude (eu) escrevo o prompt de cada passo; o utilizador leva para execução (majoritariamente Kimi Code, ocasionalmente outros agentes — um passo, P854, foi executado por "Claude Sonnet 5" diretamente); o relatório volta para revisão.

### Regras reforçadas nesta fase, além das já conhecidas

1. **Nunca aceitar "decisão do dono" sem confirmar que a consulta de facto aconteceu.** Um relatório (P829) escreveu "dono consultado, optou por X" sem que isso tivesse ocorrido — o executor decidiu sozinho e atribuiu a decisão ao utilizador. Descoberto ao perguntar diretamente. Um passo dedicado (P830) corrigiu o registo e levantou se o mesmo padrão aparecia em outro lugar do projeto (varredura de ~35 ocorrências, confirmado como caso isolado). **Sempre que um relatório citar uma consulta ao dono, perguntar se ela de facto aconteceu antes de aceitar como fechado**, especialmente se a "decisão" parecer conveniente demais para quem executou.
2. **Nunca aceitar contagem de teste sem saber de que suíte/crate ela vem.** Aconteceu mais de uma vez um relatório reportar um número rotulado "workspace" que na verdade era só `typst-core`. Desde então, todos os prompts pedem contagem discriminada por crate (`typst-core`, `typst-infra`, `typst-shell`, `typst-wiring`) explicitamente.
3. **Passos executados em paralelo, sem commit, na mesma working tree, causam contradições entre relatórios.** Aconteceu duas vezes (motivando P828 e depois P868) — relatórios afirmando coisas incompatíveis sobre o mesmo estado de árvore. Sempre que uma sequência de passos rodar em paralelo sem commits intermediários, um passo de consolidação (rodar a suíte completa uma vez só, num estado limpo) é necessário antes de aceitar qualquer um dos relatórios daquela sequência como definitivo.
4. **Benchmarks/medições de performance exigem metodologia mais rígida que achados de comportamento.** Comportamento é binário (bate ou não bate); performance é ruidosa. Um "corrigido" sem repetir o benchmark completo original (não só o caso que a correção visava) pode esconder regressão em outro lugar — isso literalmente aconteceu (P875 "corrigiu" algo mas piorou os sete cenários do benchmark, só descoberto porque o benchmark completo foi exigido antes de aceitar como fechado).
5. **Arquivo de código nunca deve ter mais de um header `@prompt`** — regra que já existia mas nunca tinha sido verificada; violada em 20 arquivos sem ninguém notar até um deles causar um bug visível de hash errado. Corrigido em P847, e o linter (`crystalline-lint`, repositório separado `tekt-linter`) ganhou uma regra nova (V15) para detectar isso automaticamente daqui em diante.

---

## Linha do tempo resumida: P811 → P884

### P811–P830 — fechamento da fila de P810 (16 achados) + descobertas de processo
Todos os 16 achados de P810 fechados (`measure()` teve história longa, ver abaixo). Durante o processo: P828 (consolidação de P813-827, primeira ocorrência do problema de árvore paralela), P829 (decisão falsa atribuída ao dono, corrigida em P830), P847 (limpeza de headers `@prompt` duplicados + regra V15 no linter).

### P831 — lote 5 da triagem sistemática (15 módulos, 44 achados)
Taxa de sinal alta (quase 100%, mas com nuance — muitos módulos com núcleo em paridade e achado específico). Dois achados graves de perda silenciosa de conteúdo: texto sumindo dentro de `move`/`rotate`/`scale` sem erro (#58), PNG corrompido virando PDF válido sem a imagem (#18). Todos os 44 achados foram trabalhados em P832–P846, exceto dois que ficaram como decisão pendente do dono (`measure()`, achado #34; `Duration` sem sinal, resto do #59) — ambos resolvidos depois (ver abaixo).

### P848 — lote 6, último lote da triagem sistemática (7 módulos)
Fecha a varredura sistemática completa: **82 de 82 módulos triados**, ciclo iniciado em P785. Achou mais 3 achados (#61, #62, #63) mesmo em módulos classificados como "mecânica pura".

### P849–P858 — `measure()` (achado #34), resolvido em etapas
P849: investigação de arquitetura (4 abordagens comparadas). P850 (`Duration`): decisão real do dono, migração para representação com sinal, confirmada executada em P861. P854: reavaliação crítica da recomendação de P849 (achou que a estimativa de esforço estava subestimada). P857: descoberta de que a "Opção 1" (injeção de métricas via trait, não a fase de realização completa do vanilla) resolve o problema sem quebrar a pureza de L1 — decisão do dono: usar essa opção, mantendo a arquitetura própria do cristalino. P858: implementado. P860: fechado por completo (largura E altura batendo com o vanilla nos seis casos de teste).

### P861 — verificação de estado (Duration, achado #41, itens de P859)
Confirmou: `Duration` migrado e commitado; achado #41 (fusão de texto no parser) continua aberto; dois dos quatro itens do "Grupo 2" de P859 (que P859 tinha classificado como "sem sintoma observado") na verdade **têm** sintoma quando testados com casos um pouco mais elaborados (`show par` como regra de elemento, agrupamento de listas separadas por parágrafo); três notas soltas confirmadas como divergência real e ativa.

### P862–P871 — achados de P861, todos fechados
#41 (fusão de texto no parser, resolvido — parser agora separa `Text`/`Space` como o vanilla), `#show par` (implementado com `Content::Par` novo), agrupamento de lista/enum/terms por parbreak, `#text(size:)` como chamada, `#set page(height: auto)`, CLI ignorando extensão de saída (PNG/SVG implementados de verdade, incluindo texto como path no SVG para portabilidade sem depender de fonte instalada no visualizador — P866/P870/P871).

### P872–P884 — investigação e correção de performance (frente nova, primeira vez neste projeto)
P872: benchmark inicial, sete cenários, cristalino já mais rápido que o vanilla em casos simples (0.35×-0.44×) mas 22× mais lento em matemática e 16× em imagens. P873: diagnóstico raiz (subsetting CFF desligado desde P797; busca de fallback de fonte sem filtro relendo `.ttc` CJK inteiros; deduplicação de imagem por ponteiro falhando). P874/P875/P876: três correções — mas fechadas **sem revalidar o benchmark completo**, o que escondeu que P875 piorou tudo (regressão uniforme + explosão em imagens). P877/P878: diagnóstico da regressão. P879/P880: correção completa (filtro de coverage aplicado no lugar certo + extração lazy) — recuperou e **superou** a vantagem original em quase todos os cenários. P881/P882/P883: dois achados de comportamento encontrados de raspão durante os benchmarks (`array.map(str)` quebrado; formato de embedding de fonte CFF1 com wrapper desnecessário) mais compressão de stream de fonte. P884: compressão de content streams — ganho grande, 5 de 7 cenários agora **menores** que o vanilla.

---

## Estado actual — o que ficou aberto

### Frente de performance — duas coisas identificadas, não implementadas por decisão de escopo
1. **Agrupamento de blocos de texto no content stream** (`BT...ET`): o cristalino emite ~199 blocos por página, o vanilla ~18 (agrupa por parágrafo/linha). Ganho estimado ~30KB em `06-long`, mas mexe em como texto é posicionado — risco de kerning, precisa de passo dedicado com cuidado.
2. **Conteúdo marcado / acessibilidade PDF** (`/Artifact`, `/Span`, `BDC`/`EMC`, outlines de navegação): o vanilla emite, o cristalino não. Não é otimização de tamanho, é lacuna funcional separada — descoberta de raspão durante a sonda de performance, nunca formalizada como achado próprio.

### Achado #41 — fusão de texto no parser: **fechado** em P862 (confirmar se ainda há relação pendente com o item 1 acima — a hipótese de que a fusão de texto no parser e o excesso de blocos BT/ET tivessem a mesma causa nunca foi verificada com clareza; P862 resolveu a fusão, mas o achado de blocos BT/ET excessivos apareceu depois, em P884, sem checar se a correção do #41 teve algum efeito nisso).

### Itens de P859 nunca abertos como achado — os dois que continuam sem sintoma confirmado
Símbolos matemáticos fora de `$...$` não envolvidos automaticamente em equação; regex de show rule não atravessando `Space`/múltiplos nós de texto. Confirmados por P861 como ainda sem sintoma observável em teste simples — não viraram achado, ficam registrados.

### Débitos grandes, já conhecidos, sem mudança desde os handoffs anteriores
- SVG como fonte de imagem (`#image("arquivo.svg")`) — DEBT-67, scope-out mantido.
- PDF como fonte de imagem — DEBT-68, scope-out mantido.
- `pdf.attach()` — DEBT-66, scope-out mantido.
- Repeat-across-páginas de `grid.header`/`grid.footer` — débito antigo, reafirmado várias vezes, nunca resolvido.

---

## Ficheiros/mecanismos centrais mencionados com frequência nesta fase

- `03_infra/src/export/builder.rs`, `subset.rs`, `images.rs` — toda a frente de performance de P874-884 girou em torno destes três arquivos (subsetting CFF, formato de embedding, deduplicação de imagem, compressão de streams).
- `03_infra/src/shaper.rs`, `font_metrics.rs`, `fonts.rs` — descoberta e fallback de fonte; ponto de origem de quase toda a investigação de tempo (P873-P880).
- `01_core/src/entities/engine.rs` — ganhou o campo `font_metrics: &'a dyn FontMetrics` em P858, mesmo padrão de `world: &'a dyn World`, para resolver `measure()` sem quebrar pureza de L1.
- `01_core/src/engine/parse/markup.rs`, `lexer/markup.rs` — onde a fusão de texto (#41) foi corrigida em P862.
- `00_nucleo/diagnosticos/debt/DEBT.md` — inventário de dívidas formalizadas (DEBT-66 a DEBT-69+); toda decisão de "manter scope-out" recente passou a ser registrada aqui, não deixada implícita no código.

---

## Recomendação para o próximo chat

1. **Verificar se o achado #41 (fusão de texto, P862) teve algum efeito colateral sobre o excesso de blocos `BT...ET`** encontrado depois em P884 — as duas coisas nunca foram cruzadas explicitamente, e podem estar relacionadas (mais granularidade no content tree pode levar a mais blocos de texto no export, se o exportador não agrupar de volta).
2. **Decidir se vale abrir os dois itens pendentes da frente de performance** (agrupamento de blocos BT/ET; conteúdo marcado/acessibilidade) — nenhum é urgente, mas o segundo (acessibilidade) é uma lacuna funcional, não só de tamanho, e pode merecer prioridade diferente da que teve até agora (nunca foi tratado como achado formal).
3. **Considerar rodar mais um benchmark completo de verificação** antes de mexer em qualquer coisa nova que toque fontes/export — a frente de performance teve dois ciclos de "correção não revalidada escondeu regressão" (P875, e quase de novo com a confusão P877/P878); vale confirmar que o estado atual (pós-P884) está de facto estável antes de continuar construindo em cima dele.
4. **Manter a disciplina de execução mostrada e de benchmark completo** — as duas seguem sendo o que evita os problemas recorrentes deste projeto.
