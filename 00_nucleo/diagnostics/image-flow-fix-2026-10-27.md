# Avanço vertical das imagens e h — 2026-10-27

Corrigido no fluxo normal: 16 imagens de 30 × 20 pt com `h(1pt)` agora ocupam duas páginas, oito em cada uma, com a geometria do vanilla fixado. A imagem publica seu fundo físico; `h`, inclusive zero/fraco, forma um parágrafo sem pintura cuja margem é resolvida pelo ancestral lexical comum. Checkpoints, subframes e transições preservam ou isolam a pendência. O parágrafo terminal que não cabe passa à próxima região, incluindo material oculto; a imagem seguinte começa em 18,2 pt, como no oráculo. Uma margem inferior apenas pendente não cria página.

## Estado e evidência

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitada, congelada em `2026-10-27T15:21:29.060001+00:00`. [Estado final](../../lab/results/construction/diagnostics/image-flow-fix-2026-10-27/final-state.json) contém 2166 hashes e `git diff HEAD --stat`; [patch contra a entrada recebida](../../lab/results/construction/diagnostics/image-flow-fix-2026-10-27/patch-against-received.diff) distingue esta tarefa das alterações anteriores. Release `17f39776137e4c22215a4edfa09b2c574da51711ea41d04f57157fef8b85c3ec`. Oráculo ratificado 64c5e8c0, árvore 483bd6f1, SHA 22897f93; identidades completas, UTC/comandos/receitas e arquivos-fonte estão no [relatório detalhado](../../lab/results/construction/diagnostics/image-flow-fix-2026-10-27/relatorio.md).

L0 atualizado antes do código: oito owners de layout e um Nucleus compartilhado. Sem contrato público, fase, dependência ou modo novos. RED inicial 7 falhas; refinamentos de margem, escopo e limite inferior têm falhas reais preservadas. Dois controles inicialmente incorretos de projeção/isolamento foram corrigidos e identificados, sem contá-los como defeitos do produto.

## Validação

- **12 gates completos**, **9987 Rust passados**, sete ignorados preexistentes, 42 Python passados; 15 novos testes. Clippy, fmt, Rustdoc, build/all-targets e linhagem passam; linter completo com zero warning/error. Um job, 12 GiB e zero swap.
- **36 fontes PDF/SVG positivas**: 26 controles de fluxo normal e 10 do limite inferior. Topos do original 5,51.4,…329.8pt por página; delta máximo PDF 0,000012 pt, tolerância 0,001 pt. As duas páginas renderizadas do original RGB coincidem pixel a pixel a 800 px.
- Revalidação debug/release: 392 PDF/386 SVG por build, 380/374 qualificados. Permanecem as mesmas 12 fontes de tracking R10; notas R8–R12, R13 e quebras inicial/terminal preservadas. Matrizes de linguagem, seis perfis release e dois debug repetidos.

## JPEG e recursos preservados

O código de validação JPEG mantém o hash recebido; 17922 casos diferenciais sem mudança de aceitação/diagnóstico. Os gates históricos de memória/tempo passam. Gradiente atual/antes-SSE: -1.77%; atual/antes-native-Luma: -10.64%. RGB-large versus alteração 2: tempo -61.40%, RSS -85.55%.

Contra o executável recebido, gradiente: tempo -0.71%, IC95% da razão [0.9861, 0.9943], RSS +0.24%; nove/nove casos grandes mantêm RSS<=103%. Custos de layouts compartilhados agora corretos são registrados separadamente. R6 preservado: tamanho normalizado/R5 0.860385; RSS geomean/R5 0.950527; todos os limites de tamanho, variação, memória e matemática passam.

## Divergências remanescentes

Stack16 mantém exatamente a divergência recebida. Continuam 12 tracking R10, 15 refuters antigos de super, três variantes adicionais de linhas vazias e limites de pintura já registrados. Nesta rodada JPEG, pintura ainda difere em cmyk, gray-gradient, gray-horizontal, gray-vertical. Payloads originais são preservados; JPEGs truncados/inválidos continuam sendo rejeitados pelo Tekt. O controle inválido oversized do vanilla foi interrompido por pressão sustentada de memória (8.87 GiB; SIGTERM −15) e fica não qualificado por recurso, sem ocultar sua saída nem tratá-la como match. Evidência em oversized-oracle-termination.json. Não se afirma paridade universal de pixels ou da linguagem.

Exatamente 17 entradas autorizadas mudaram: oito fontes, oito L0, um Nucleus. Demais entradas/alterações recebidas preservadas; 175493 arquivos históricos com SHA selado válido e um log com saída original exata e 1853 assets verificados. O diagnóstico geral anterior permanece histórico. Um log histórico tinha selo autorreferente inválido: o manifesto capturou completion.log vazio antes da impressão. O conteúdo corresponde exatamente à saída original, com mtime anterior à tarefa; log/manifesto preservados e exceção explícita no recibo. Todos os demais hashes foram exigidos sem exceção. Sem commit/push nesta tarefa.
