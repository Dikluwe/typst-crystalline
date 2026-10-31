# Stack com imagens — correção e validação (2026-10-27)

O caso `stack16` antes aberto foi corrigido. O Stack compõe as dimensões lógicas dos filhos, respeita direção/espaçamento e avança entre regiões; imagens deixam de ser tratadas como uma altura tipográfica. Fonte, tamanho e dados JPEG não foram alterados.

| Fonte | Imagens/página recebido | Tekt final | Vanilla fixado |
|---|---|---|---|
| `ttb16` | [16] | [16] | [16] |
| `ttb20` | [20] | [19, 1] | [19, 1] |
| `ttb20-spacing` | [20] | [14, 6] | [14, 6] |
| `btt20` | [20] | [19, 1] | [19, 1] |

No controle de três imagens de 20pt, os topos mudaram de 14,834/22,072/29,310pt para 5/25/45pt, iguais ao vanilla. Há 32/32 controles geométricos principais,2/2 controles de efeitos e5/5 consultas de Here aprovados. As tolerâncias PDF/SVG continuam 0,001pt; o observador compõe matrizes e compara as páginas físicas.

A investigação também corrigiu a publicação do snapshot final das consultas, preservando a identidade/página dos contextos; o contador executa os 20 filhos uma vez e os rodapés observam 18/20. O fundo físico de matemática fechada exclui a preparação da próxima linha; o teste existente da página automática voltou a 125,236pt. Um controle positivo adicional preserva a borda descender explicitamente declarada após texto e quebra literal; nenhuma regra de borda padrão mudou.

L0 atualizado e hashes recalculados antes de materializar cada reparo, em fluxo contínuo do ADR-0127: 10 consumidores/10 proprietários e 1 novo Núcleo compartilhado; nenhum contrato público ou fase novo. Foram adicionados 16 testes de regressão e preservação, com RED→GREEN das falhas reais e expectativas anteriores preservadas, salvo a rejeição obsoleta de spacing negativo substituída pela verificação estrita do mesmo valor tipado. A correção do join de diagnóstico `A B` no teste inicial está documentada como ajuste da fixture, não falha do produto.

Os 12 gates completos passaram: **10003 testes Rust**,7 ignorados preexistentes,42 testes Python, builds, Clippy, Rustdoc, fmt e linhagem; o lint SARIF integral tem 0 avisos/erros. O estado final congelou 2167 entradas produtivas/L0 sem mudança durante a validação. Foram repetidos todos os controles de imagem/h anteriores, incluindo as 10 fronteiras; `stack16` agora é gate positivo. Os 12 controles RGB renderizados têm páginas pixel a pixel iguais ao oracle.

A receita de compilação alinha funções em 16 bytes no alvo x86_64-unknown-linux-gnu, conforme o L0 de wiring/main; a configuração original dos demais alvos foi preservada. A primeira campanha completa passou em linguagem/CI, mas reprovou no tempo JPEG e está integralmente preservada em full-attempt3/. Os controles de opt-level3 do decoder e alinhamento32 foram refutados; alinhamento16 passou antes de sua adoção. O executável final foi novamente compilado e validado com a configuração padrão efetiva. O release final tem 51343136 bytes contra 51014808 do recebido (+0.64%); esse custo de alinhamento foi aceito somente após os limites R6 preservarem a redução normalizada e a memória.

JPEG: o validador conserva o hash recebido e os 17922 casos diferenciais passaram. Gradiente atual/antes-SSE: -0.71%, IC95% pareado [0.9884830355973545, 0.9949803603606139]; atual/antes-native-Luma: -9.69%. RGB grande versus alteração2: tempo -62.27%, RSS -85.39%. Contra o recebido nesta tarefa: gradiente tempo -0.18%, IC95% [0.9882951587208991, 1.0179414828461577], RSS+0.28%;9/9 casos grandes respeitam RSS≤103% recebido. São medições finitas, não promessa global de desempenho.

R6 preservado: tamanho normalizado/R5=0.866950, RSS geomean/R5=0.952748; todos os limites de tamanho, variação, memória e matemática passaram.

Permanecem divergências preexistentes: `horizontal-math-image` reserva largura excessiva para equações de bloco numa pilha horizontal; a posição interna de texto após imagem no mesmo filho com quebra literal também diverge. Ambos foram medidos no executável recebido; não foram convertidos em aprovações. O baseline do rodapé mantém sua diferença preexistente de2.595993pt, embora a semântica18/20 esteja correta. Os 12 tracking do R10, os15 refutadores gerais antigos, variantes adicionais de linha vazia e as 4 diferenças de pintura JPEG continuam registradas nas respectivas campanhas. A correção não certifica paridade universal dos containers.

Tekt release SHA-256 `d141e2ee123c262b37d0bf7e7c83fd00429504782b9f8f3c030072cd834cbdf1`; debug SHA-256 `8e896356fdcb2f3bec2704c7f441279b78fc42dc167fe67f776c7cde5ecd0d98`.

Referência: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`, oracle SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Proveniência: HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**, congelada em 2026-10-27T20:40:46.842707+00:00; lista exata de alterações, UTC, hashes de fontes/executáveis e comandos em [estado final](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/final-state.json), [patch sobre o recebido](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/patch-against-received.diff) e [relatório integral](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/relatorio.md).

[PDF Tekt de 20 imagens](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/stack-final/ttb20-tekt.pdf) · [PDF vanilla](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/stack-final/ttb20-vanilla.pdf) · [Antes / corrigido / vanilla](../../lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/stack-preview/before-fixed-vanilla.png).
