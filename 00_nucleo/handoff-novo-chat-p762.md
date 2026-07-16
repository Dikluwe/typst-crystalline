# Estado do projecto typst-crystalline — handoff para novo chat

**Data:** 2026-07-16
**Último passo fechado:** P779
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla Typst 0.15.0 — **usar sempre este, nunca `/usr/local/bin/typst`, que é 0.14.2 e já causou uma revalidação inteira, P757**)

---

## Como este projecto funciona

Cada passo segue: **sonda** (medir/confirmar contra o vanilla antes de qualquer código) → **implementação** → **validação** (`cargo test --workspace`, `crystalline-lint .`, comparação directa com o vanilla) → **relatório**. Claude (eu) escrevo o prompt de cada passo com a sonda e os critérios de fecho; o utilizador leva-o para o Claude Code executar; o relatório volta para revisão. **Relatórios devem ser colados directamente na mensagem — anexos `.md` têm chegado vazios repetidamente nesta conversa.**

### Regras/ADRs em vigor, por ordem de aparecimento

1. **Decisão nova obrigatória** — nenhuma correcção sem decisão explícita registada.
2. **Disciplina de verificação** (ADR-0108) — medir antes de decidir, sempre. Nunca aceitar "parece razão suficiente" sem confirmar directamente.
3. **Proveniência de medição** — toda a medição tem de ter origem rastreável.
4. **Paridade de defeitos** — bater com o vanilla não prova estar certo; o vanilla pode ter os mesmos bugs.
5. **Checklist de sub-layouts** — mecanismo corrigido no fluxo principal tem de ser verificado nos 4 sub-layouts (grid, box, columns, place).
6. **Fronteira L1/L3** (ADR-0109) — L1 só código puro, sem I/O; qualquer coisa que precise de dados externos (fontes, WASM, dicionários) vai para L3, com trait em L1. Excepção: crates com dados 100% embutidos em tempo de compilação (`compiled_data`, sem `std::fs`/I/O real) podem ficar em L1 — **confirmar isto directamente antes de assumir**, não basta a crate "parecer" pura.
7. **Diferença de implementação vs diferença de linguagem** — implementação pode divergir (cache diferente, algoritmo diferente) desde que o resultado observável do documento seja igual. Sintaxe/comportamento da linguagem **não pode divergir** sem decisão consciente e nome distinto (ex: `table.numbering` é extensão documentada; `variant: (eixo:)` foi erro e foi revertido).
8. **Sonda obrigatória para funcionalidade nova** (ADR-0114) — nunca implementar algo nunca tocado sem sonda prévia escrita antes do código.
9. **Registo, não reconstrução** — se um passo for decidido directamente na sessão de execução sem prompt prévio meu, não reconstruir o prompt depois do facto (fazia isso até P705/713-715; descontinuado). Registar como linha numa tabela `passo → commit → relatório`, mantendo a sequência auditável sem fingir uma ordem que não aconteceu.
10. **Auditar scripts de validação antes de confiar nos números** — qualquer `validate.py` ou script equivalente usado para gerar números de um relatório de paridade tem de ser lido e confirmado, não aceite pelo nome. A ferramenta e resolução de rasterização têm de bater com a convenção do projecto (`mutool draw -r 300` para comparação de imagens) e devem estar fixadas no próprio script (constante/comentário), não deixadas à escolha de quem o escreveu num passo específico. (Lição de P778/P779: `pdftoppm -r 150` num script não auditado gerou resíduos de AE que não existiam com a metodologia padrão.)

---

## Linhas de trabalho fechadas (resumo por assunto, não cronológico)

### Falhas silenciosas (P633-656)
Três rondas de auditoria. 23+ casos confirmados: `FlowEvent` (`#break`/`#continue`/`#return` nunca funcionavam dentro de ciclos/funções), regras `#set` a ignorar tipos inválidos, bibliografia a omitir entradas, grid a renderizar vazio, etc.

### Desempenho (P657-677)
`macro-10x` (documento de stress com conteúdo repetido) foi de 6,78× mais lento que o vanilla para **mais rápido** que o vanilla (0,92-1,18×, consoante a fase). Documentos pequenos ficaram 2,3-2,5× mais rápidos. Causas: cache de resultado de shaping em falta, `Face::parse` repetido sem cache (duas vezes, em dois caminhos diferentes), leitura duplicada de fontes no arranque, walk triplicado do documento numa função de render.

### Divergências de linguagem vs implementação (P660-665)
Regra 7 acima estabelecida aqui. `variant: (eixo:)` inventado por erro, revertido. `text.bold`/`text.italic` revertidos, substituídos por `weight`/`style` (nomes correctos do vanilla).

### Fontes variáveis — pendência antiga (P525 → P666-669)
Contornos visuais errados para pesos não-default de fontes variáveis, corrigido nos dois caminhos de export (single-font e multi-font), com erro claro se Python/fontTools (dependência para instanciação) não disponíveis.

### Pacotes, `#import`, WASM, e uma auditoria profunda de semântica da linguagem (P678-762 — a maior linha desta conversa, ~85 passos)

Começou como "resolver o WASM" (infra-estrutura de plugins, protocolo `wasm-minimal-protocol`, runtime `wasmi` em L3). Ao longo do caminho, validar um pacote real da comunidade (`cetz`, uma biblioteca de desenho) revelou uma quantidade enorme de semântica fundamental da linguagem nunca antes tocada:

- `#import` de ficheiros locais **não existia** (implementado do zero).
- Desestruturação (`let (a,b) = ...`) estava **quebrada** (só ligava o primeiro nome).
- Atribuição simples/composta (`x = v`, `x += v`) **não existia**.
- `and`/`or` **não faziam short-circuit** — `false and (1/0)` dava erro em vez de `false`.
- Blocos de código **não faziam `join` sequencial** — só devolviam a última expressão.
- Binding de argumentos de closure tinha um bug sério: argumentos posicionais extra eram aceites silenciosamente, corrompendo parâmetros `nome: default` com o tipo errado.
- `context` não herdava o `StyleChain` realmente activo (usava um por defeito).
- `measure()` devolvia sempre `0pt`, sempre, sem erro.
- Sistema de tipos-como-valores (`int`, `str`, `color`, `gradient`, `counter`, `state`) inconsistente — alguns eram `Dict`/`Func` quando deviam ser `type`.

Muitos destes foram encontrados **por acidente**, como efeito colateral de perseguir o próximo bloqueio de `cetz`, não como objectivo de nenhum passo. `cetz` acabou por renderizar com **paridade de pixels exacta** (AE=0) no caso de teste final.

**Lição de processo, registada por escrito**: sempre que um relatório atribuir um resíduo a "diferença mecânica"/"anti-aliasing" sem prova directa, questionar — já aconteceu duas vezes nesta conversa (P745-748 para formas de desenho, P759-762 para texto) que essa explicação escondia um bug real. Nunca aceitar sem medição directa (mapa de diferenças, comparação de coordenadas exactas, confirmação de identidade de fonte).

### Layout: baseline, margem, avanço de linha (P745-762)
Cadeia de investigação sobre um resíduo de "0,14% de anti-aliasing" que se revelou, sucessivamente: margem de página errada (P748), fórmula de primeira baseline errada — `ascender` em vez de `cap-height` (P750), `cursor_y` fixado cedo demais sem esperar pelo estilo activo (P751), `FixedMetrics` aproximado em vez de métricas reais da fonte (P752), fonte por defeito errada — `Liberation Serif` em vez de `Libertinus Serif` (P753), regressão CJK causada por essa correcção (P754), e finalmente a causa principal: **modelo de avanço entre linhas inteiramente diferente** — cristalino usava `ascender+descender+lineGap` (convenção comum, tipo CSS), vanilla usa `cap-height + leading` (convenção própria do Typst) (P761-762). Corrigido com `top-edge`/`bottom-edge` como propriedades reais de `TextStyle`, configuráveis via `#set text(...)`, aplicadas consistentemente a texto, listas, grids, sub-frames. Resultado final: **AE=0, RMSE=0** no caso de teste.

### CJK/Thai — quebra de linha para scripts sem espaços (P755-759)
Vanilla 0.15.0 usa `icu_segmenter 2.2.0` (não `xi-unicode` puro, como uma issue de 2023 sugeria — sempre confirmar contra a versão real, não issues antigas) com um `CJ_SEGMENTER` customizado para tailoring de aspas chinês/japonês. Implementado em L1 (confirmado como I/O-livre via `compiled_data`). Scope-out consciente: aspas ainda mal posicionadas em espaço muito apertado (exige replicar o segmentador customizado a fundo, não só os breakpoints — P758), Knuth-Plass vs greedy **não é a causa principal** dessa diferença (confirmado testando o próprio vanilla em modo greedy — P758), e greedy vs Knuth-Plass para texto latino comum produz **quebras idênticas** (decisão de não implementar Knuth-Plass, P759).

---

## Itens conscientemente não resolvidos (scope-out, não esquecidos)

- `polygon` com vértices `Ratio` (`50%`) — exige resolução em tempo de layout, não de eval; sem consumidor a justificar o custo.
- Ordem entre tipos numa mensagem de erro de argumento extra — caso de canto, ambos os lados já erram.
- Aspas CJK em espaço muito apertado — precisa do `CJ_SEGMENTER` completo do vanilla replicado, não só os breakpoints.
- Knuth-Plass — decidido não implementar; greedy já bate com o vanilla para texto latino comum.

---

## Numeração

P590 a P762, sem lacunas reconstruídas a partir de P716 em diante (ver regra 9 acima). P705, P713, P714, P715 têm reconstrução retroactiva feita **antes** dessa regra existir — não repetir esse padrão.

## Ficheiros/mecanismos centrais mencionados com frequência

- `01_core/src/rules/layout/cursor.rs` — `flush_line()`, `ensure_initial_baseline()`, layout de linha.
- `01_core/src/rules/layout/metrics.rs`, `03_infra/src/font_metrics.rs` — `FontMetrics` trait, `FixedMetrics`/`FontBookMetrics`/`FallbackFontMetrics`.
- `01_core/src/rules/eval/closures.rs` — `apply_closure`, binding de argumentos.
- `03_infra/src/embedded_fonts.rs` — fontes embutidas via `typst-assets`, separadas em grupos texto/math-code.
- `achados-adiados-cetz.md` — lista de controlo de achados menores (deve estar vazia ou só com scope-outs conscientes; verificar se ainda existe/está actualizada).
