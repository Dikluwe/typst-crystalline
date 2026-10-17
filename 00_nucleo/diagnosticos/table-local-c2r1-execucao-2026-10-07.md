# C2-R1 — tamanho herdado e colunas Auto corrigidos

A prioridade1 foi materializada em fluxo contínuo ADR-0127. A tabela com cem aliases de Cell Styled local volta a ter texto10pt, largura correta e nenhuma sobreposição; o PDF coincide com o vanilla pinado em palavras, família/tamanho/cor e bboxes até0.00002pt. A recoleta de tempo mostra mudança pequena,170.94→170.10ms (mediana pareada −0.50%), não aceleração geral. Tamanhos/estilos adicionais ainda conservam resíduos métricos nomeados abaixo; não declarar Table/Styled universalmente fechados.

## Medição anterior e classificação

A coleta performance-c2-2026-10-07, com HEAD/working tree/UTC/fontes/argv/hashes completos, mediu o release C2 ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f contra C congelado e vanilla9f2b6e871. table-local-regression.json conserva cem palavras Local, mas largura21.880→24.068pt e segundo x60.880→38.999, sobrepondo o primeiro bbox x29.000–53.068. Não é diferença de bytes PDF, whitespace ou igualdade Rust. O pedido humano “Faça a prioridade1” autoriza a correção deste testemunho.

A fonte explica duas fronteiras. expression_dispatch.rs:132 colapsava toda a cadeia local, incluindo root size11/boldfalse/italicfalse de style_chain.rs:304–315, congelando defaults da avaliação no Styled inserido sob text(size10pt). O vanilla typst-eval/src/code.rs:32–43 aplica somente os styles da declaração à cauda. Em tracks.rs:328, measurement_visual só projetava Cell com resolved; a Cell standalone Complete retida no valor local permanecia na projeção, e measurement.rs não possui medida estática de TableCell. Colunas Auto ficavam praticamente só com inset. A hipótese conjunta foi refutada/confirmada pelos testes independentes de tamanho e separação e pela medida física posterior, sem inferir intenção upstream.

Classificação: correção interna de paridade; nenhum contrato público, tipo/campo/trait/flag/fase/default de produto novo. L0 primeiro e resselo somente dos quatro owners. Mantidos C2 Source, Styled lexical, snapshots causais, efeitos uma vez, identidade nominal, pipeline e regras anteriores.

## Implementação

- `01_core/src/entities/style_chain.rs`: helper crate-private local_delta_since delimita o prefixo de declarações posteriores a um ancestral compartilhado. Não confunde valor igual ao default com ausência de declaração; size11 e boldfalse explícitos permanecem. Identidade Arc fica interna à delimitação lexical, não é exposição de identidade ou telemetria. Não ancestral retorna None; mesmo nó retorna delta vazio. A dobra top-wins existente é preservada.
- `01_core/src/compiler/eval/expression_dispatch.rs`: Styled local Table/Cell recebe somente esse prefixo, acrescido dos canais privados C2. Snapshot interno conserva regras/guards/styles/file/floor; não altera whole-context ou bare math. A condição de captura reconhece declaração local mesmo quando seu valor coincide com o herdado.
- `01_core/src/compiler/layout/grid/tracks.rs`: projeção de medição usa visual de Cell com stage conhecido, inclusive standalone. Cell crua sem stage conserva fallback; Table aninhada permanece atômica. Projeção é pura, sem executar receita/callback. Inset/tinta/placement/cache permanecem em seus owners.
- `01_core/src/compiler/layout/tests.rs`: testes de linguagem verificam10pt herdados, duas colunas sem overlap e11pt local explícito. O helper de prefixo também tem teste de ancestral/declaração igual ao default/não ancestral.

Ownership1:1, L0s originais e hashes auditados em audit.json; todas as mudanças produtivas/L0 são os quatro pares de owners.json. Não houve criação de núcleo ou resselo global.

## RED → GREEN e validação

red.log: dois testes semânticos falharam antes da implementação (tamanho e overlap);11pt local já passava. O primeiro ensaio usou a assinatura incompleta de FixedMetrics.text_width; seu erro de compilação está separado em test-api-error.log, não foi usado como RED semântico. green.log: quatro testes locais aprovados. Estado final imutável em tested-final.json.gz.

Build debug/release aprovados. Suíte final:6634 core +1195 infra +90 shell +10 integração query =7929 aprovados; cinco core ignorados. validation.json registra todos os comandos/status/UTC/snapshot. Linter final: zero erros,535 warnings/1636 notes na árvore inteira; owners R1 sem V5/V6/V15/V26. Não afirmar zero violations global.

Coletas bilaterais: origem C2 principal8/8, adicionais10/10, estilos Cell58/58, controles adicionais8/8, profundidade debug10/10, isolada51/53 (somente warnings antigos raw-fields/raw-x). Corpus presence92/92,selector23/23,equation29/29,matrix29/29+14/14,query36/36,Table62/62. D06 conserva root-render e dois warnings antigos; Table refutadores mantém additional/html-table e order/table-root-equality. Raw e Raw bloco conservam suas saídas e limites métricos anteriores. Não surgiu divergência nova nesses corpus contra a coleta final C2.

C20 controles geométricos e quatro de tinta passam os gates previamente certificados: palavras/ordem e tinta preservadas, delta vertical máximo0.00002pt. Negativos horizontais align/inset5pt/0.005pt e set-text posicional permanecem históricos. C2 mixed default nested-local-text ainda tem6.04996pt vertical; essa frente C3 não foi fechada pelo R1.

## PDF e controles ampliados

physical-r1.json.gz contém doze variantes. O caso principal100 palavras e os controles de duas Cells, Cell plain, show identity e show body coincidem em fontes/cor/palavras/coordenadas até0.00002pt. No caso principal, a primeira palavra voltou a x29.000–50.880 e a segunda a x60.880, eliminando sobreposição. Tamanho herdado14pt, bold/italic e tamanho local11/18pt são preservados como propriedades; declared11 não é apagado por coincidir com o default de avaliação.

A ampliação não certifica todas as métricas de Styled: outer14pt/later14pt conservam0.004pt horizontal; bold/italic0.68pt horizontal; local11pt0.658pt vertical; local18pt5.264pt vertical; local11 sob inserção14pt1.974pt vertical; regular local sob bold externo0.13pt vertical. before-r1.json.gz e before-c2.json.gz medem os mesmos fixtures nos dois releases congelados. O R1 reduz fortemente os erros C2 de tamanho/overlap; vários desses casos já eram negativos no C anterior. O residual0.004pt de14pt não aparece naquele C, embora C2 estivesse37.197pt horizontal/2.682pt vertical por tamanho errado. Não ocultar esse residual nem compensar por constante ad hoc. Detalhes comparativos completos em final-results.json/extended_geometry_comparison.

As coleções legacy executam bytes congelados com suas receitas/snapshots previamente comprovados; seu source_hashes descreve a árvore anfitriã no momento da sonda, não a fonte que produziu o binário antigo. A proveniência desses binários vem dos relatórios C/C2 e dos receipts arquivados.

## Recoleta de desempenho

Três fixtures sem alterar os bytes do benchmark anterior: Hello,100 aliases bare e100 Cells locais. Três rodadas/doze trios, seis permutações de ordem, duas compilações warmup por engine/bloco.324 execuções medidas +54 warmups, CPU4/Rayon1/vanilla jobs1. Quiet gate30s e monitor; nove blocos válidos, hashes/fonte estáveis, sem build/test/benchmark concorrente observado. A visão de processos e o polling não garantem ausência de carga breve do desktop.

Wall inclui launcher GNU time/taskset e lifecycle completo CLI; os mesmos wrappers são usados nos três engines. RSS é medido no filho GNU time após exec, sem pico herdado do Python. Não comparar valores absolutos com os tempos anteriores sem wrapper como ganho causal. Cache de filesystem aquecido, processos novos; sem cold-disk/watch/incremental. Dados/distribuições/argv/UTC/fingerprint/patches/fontes em performance/.

| Fixture | C2 anterior,ms | R1 atual,ms | Vanilla,ms | Δ pareado R1/C2 | RSS C2→R1,MiB |
|---|---:|---:|---:|---:|---:|
| Hello |151.97|152.22|125.93|+0.33%|42.76→42.91|
| Table bare100 |159.98|160.09|129.11|+0.26%|45.31→45.34|
| Table local100 |170.94|170.10|129.26|−0.50%|46.11→45.36|

São medianas wall e razões pareadas, não quocientes das colunas. Média/mediana e faixa de medianas entre rodadas ficaram abaixo de3% nos três engines/fixtures. O intervalo bootstrap descritivo da razão Table local é0.99177–0.99758, mediana0.9950; amostragem por trios não elimina limitações do host. Há redução pequena nesta sonda; o anterior ainda tinha saída incorreta, logo não chamar isso de aceleração para trabalho equivalente. R1 e vanilla passam o controle geométrico nos três fixtures; para Table local, C2 anterior falha. Os controles Hello/bare permaneceram próximos; não há aceleração geral demonstrada. RSS Table local reduz cerca de0.75MiB nas medianas, sem promessa para documentos grandes.

## Proveniência e preservação

HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before-code UTC2026-10-07T21:49:48.055998+00:00, tested-final UTC21:54:50.438341+00:00. Ambos conservam lista exata de arquivos, conteúdos/SHA-256/status/diff HEAD --stat. O verificador compara com o snapshot recebido e usa git show HEAD somente para arquivos inicialmente limpos: oito arquivos produtivos/L0 mudaram, restritos a quatro pares, nenhum fora do escopo. implementation.patch e seals.json preservam delta próprio e linhagem. Alterações herdadas preservadas; sem commit/reset.

Release R1 SHA-2560a2410ec5b4a075417943d25a4d24ac860da6ff0adf4845707d4f18dc9cc7add. Vanilla upstream pin9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489, binário7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. C2 anterior congeladoac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f. Não usar versão textual como prova.

Janela de desempenho:2026-10-07T21:59:30.021946+00:00 até22:02:05.407403+00:00. Coletas físicas e validações têm UTC próprios e fonte/argv/estado/hashes em seus receipts. final-results.json e performance/summary.json verificam identidade dos binários e fontes antes/depois; final-state confirma ausência de mudança produtiva após testes/coletas.

C2-R1 fecha o testemunho de tamanho herdado/overlap100. Para velocidade, a próxima prioridade continua cobertura de fontes por intervalos, precedida de medição/contrato próprio; C3 e os resíduos métricos explícitos permanecem frentes de render separadas. Nenhum ganho de experimento de fontes foi somado a esta correção.
