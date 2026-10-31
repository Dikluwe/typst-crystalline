# Nota de rodapé com quebra terminal — 2026-10-26

> Atualização posterior: a [correção da linha inicial vazia](leading-footnote-line-2026-10-26.md) preserva este resultado e qualifica mais um dos 32 refutadores; restam 15. Este relatório conserva as medições da correção terminal.

**Corrigida a fragmentação do reprodutor com quebra após L5:** o Crystalline
agora produz as mesmas três páginas do vanilla fixado, com a nota iniciando na
segunda página. Os ajustes de viúvas/órfãs voltam a participar da escolha dos
cortes. A correção foi validada em debug e release em 48 reprodutores.

## Proveniência e autorização L0

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, árvore não commitada recebida com R11/R12/R13 e a comparação
posterior. [baseline.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/baseline.json) preserva UTC, status, stat
exato de `git diff HEAD --stat`, 2165 hashes de input e identidades recebidas;
[received-source.json.gz](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/received-source.json.gz) conserva os bytes.
Os números abaixo se vinculam aos recibos de cada estado, não somente ao HEAD.

Vanilla fixado: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Não houve resync. O [encerramento](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/completion.json) sela os inputs,
fontes completos, documentos, executáveis e manifesto de evidências finais.

O [L0 proprietário](../prompts/core/compiler/layout/footnote_fragments.md) foi
atualizado antes dos testes/código, incluindo a evidência diferencial anterior,
os limites do reconhecimento e o refinamento medido no RED. É correção interna
de paridade sob fluxo contínuo do ADR-0127: permanece no layout privado, sem novo
campo público, trait, assinatura pública, modo, default ou mudança de fase.
Um único consumidor e seu L0 foram alterados; os pins dos Nuclei permanecem.

## Causa e correção

A investigação anterior reteve oito fontes com/sem quebra final e custos
default/zero/orphan0/widow0 em
`lab/results/construction/diagnostics/trailing-footnote-investigation-2026-10-25/`.
O [before](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/before/receipt.json) ampliou a medição para os 48 reprodutores desta
correção, preservando os inputs do binário recebido separadamente dos inputs
do observador. Fonte efetiva e tamanhos eram iguais: DejaVu Sans, 11 pt principal
e 9,35 pt na nota no reprodutor original; a diferença era de fragmentação.

A última quebra obrigatória deixava uma linha lógica pendente sem testemunho de
linha seguinte registrada. O certificado rejeitava a nota inteira ao encontrar
essa linha vazia. Isso também impedia a proteção das primeiras/últimas linhas,
a reconciliação da origem e o reconhecimento da continuação. O tamanho capturado
ainda incluía um avanço após o término real da última linha.

Agora o reconhecimento acompanha o estado pendente da quebra e usa o último
limite efetivamente registrado, cujo índice cobre todos os itens capturados.
Somente a linha lógica sem testemunho correspondente é reconciliada, após
conferência de contagens e geometria finita. O tamanho final vem desse testemunho,
sem subtrair uma entrelinha presumida nem reconstruir semântica a partir de
glifos. A origem também é corrigida no singleton que foi realmente comprometido
pela quebra, preservando o caminho do singleton drenado sem quebra.

O Content original, os itens, as decorações e os marcadores são preservados.
Linhas vazias reais, leading/internal blanks não certificados, CR isolado e corpos
opacos conservam suas recusas anteriores neste escopo; não se eliminam todas as
linhas vazias nem se declara resolvida a divergência da nota com linha inicial vazia.

| Quebra de página do reprodutor original | Recebido | Corrigido / vanilla |
|---|---|---|
| Página 1 → 2 | Depois de L1 | Antes de começar a nota |
| Página 2 → 3 | Depois de L3 | Depois de L2 |
| Página 3 → 4 | Depois de L4 | Não ocorre; L3/L4/L5 encerram na página 3 |

## RED→GREEN e revalidação

[red.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/red.json) e seu arquivo completo de fontes conservam três
falhas assertionais e dois controles aprovados: reconhecimento da nota, geometria
e proteção de órfã falhavam; o controle de linha vazia real e um teste regional
existente passavam. Os quatro testes novos passam no estado final. O
[candidato](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/candidate2-evaluation.json) já qualificava os 48 reprodutores;
a validação completa repete a comparação sobre o reseal final, que também
preserva a verificação anterior de finitude do frame.

- 48 reprodutores por build, PDF e SVG a 0,001 pt: três famílias tipográficas,
  custos default/zero/orphan0/widow0, controles sem quebra terminal, alturas de
  página de 80/100/120/160 pt, notas de 1/2/3/5/7 linhas e Strong/Emph.
- Reprodutor original mantido byte a byte entre os 32 refutadores anteriores;
  passa em debug/release. Palavras completas por página, dimensões físicas,
  bounding boxes e origens/regras SVG são conferidos separadamente.
- R11: 68 PDF/SVG; R12: 54+4; R13: 25, incluindo os três separadores percentuais;
  R8: 28 PDF/22 SVG; R9: 75 PDF/SVG; R10: 69 PDF/SVG e bitmap, em ambos os builds.
- 21 replays e cinco suplementos A–H/R1–R7 preservados com auditoria semântica,
  campos/construtores, símbolos, render e configurações. Warnings conhecidos
  de R11 foram conservados, não convertidos em silêncio universal.
- 12 gates completos passaram: 9950 testes Rust aprovados, sete ignorados e 42
  Python; workspace build, todos os targets, Clippy, fmt/rustdoc, estrutura,
  linhagem estrita/reversa e lint completo sem violações.

[pipeline.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/pipeline.json), [post.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/post.json) e
[closing-audit.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/closing-audit.json) vinculam todos os resultados
aos respectivos comandos, UTC, executáveis e 2165 inputs finais.

## Pintura e recursos

[Pintura terminal](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/terminal-paint/receipt.json) mede seis reprodutores por
build, em todas as páginas PDF/SVG a 72/600 dpi: os três defaults de cinco linhas
e os três singletons. As observações são iguais entre debug/release. Resíduos
na amostra release:

- `DejaVu Sans-default-terminalTrue`, PDF 600 dpi, página 2: 2 pixels, delta máximo de canal 11.
- `DejaVu Sans-default-terminalTrue`, PDF 600 dpi, página 3: 2 pixels, delta máximo de canal 11.
- `DejaVu Sans-height80-lines1`, PDF 600 dpi, página 1: 2 pixels, delta máximo de canal 11.
- `Nimbus Sans-default-terminalTrue`, PDF 600 dpi, página 2: 2 pixels, delta máximo de canal 11.
- `Nimbus Sans-default-terminalTrue`, PDF 600 dpi, página 3: 2 pixels, delta máximo de canal 11.
- `Nimbus Sans-height80-lines1`, PDF 600 dpi, página 1: 2 pixels, delta máximo de canal 11.
- `Liberation Serif-default-terminalTrue`, PDF 600 dpi, página 2: 2 pixels, delta máximo de canal 11.
- `Liberation Serif-default-terminalTrue`, PDF 600 dpi, página 3: 2 pixels, delta máximo de canal 11.
- `Liberation Serif-height80-lines1`, PDF 600 dpi, página 1: 2 pixels, delta máximo de canal 11.

[Pintura dos 32 refutadores](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/refuter-paint/receipt.json) conserva os
demais resíduos de casos geometricamente qualificados e observa também o
reprodutor terminal a 600 dpi. Pintura não substitui nem relaxa o gate geométrico.

Os gates originais R6 passaram nos sete workloads, incluindo repetibilidade e
latência de math contra o controle definido pela receita; os três stresses
também passaram. Razão de tamanho normalizado atual/R5: **0.855772604**;
razão geométrica RSS atual/R5: **0.950033423**.
O controle R5 é identificado por SHA-256
`c3d2d2e416e506f9e49bff28d55a05138030440a6288d88bc77631348dae5549`.
O HEAD atual não é apresentado como a construção histórica desse controle, nem
se atribui ganho causal de recursos a esta correção.
[Recursos](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/revalidation/final-resources/receipt.json) e
[stresses](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/revalidation/legacy-resources/receipt.json) guardam as amostras.

## Preservação e limites remanescentes

[Preservação](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/preservation.json) confere 2163 inputs recebidos fora do
par autorizado e 108957 arquivos históricos, além dos assets/fontes. A licença
DejaVu Sans já normalizada em HEAD212 continua igual ao HEAD recebido; ela não
foi alterada. As alterações existentes de R11/R12/R13 foram preservadas.

Dos 32 refutadores anteriores, 16 agora qualificam e 16 permanecem abertos:
o único que mudou de classificação nesta correção é `note-long-trailing-break`.
A linha inicial vazia, os argumentos públicos de super e a composição de scripts
continuam no diagnóstico, assim como os limites anteriores de R2–R5. Esses
16 reprodutores não representam 16 causas independentes nem o total global de defeitos.

A auditoria de encerramento inicialmente exigia silêncio nos novos controles de
`text.costs`. O observador foi corrigido para exigir o mesmo stderr completo do
baseline de 48 casos, normalizando somente o caminho do arquivo. Os avisos
preexistentes foram preservados; os inputs e testes do produto não mudaram.
A tentativa falha e o observador anterior foram mantidos, e a auditoria final
foi retomada com os mesmos inputs.

O primeiro preflight V26 apontou a seção nova antes do bloco de Núcleos. Um
candidato iniciado antes da conclusão desse reseal foi interrompido; o bloco
foi reordenado antes da execução válida. Logs e fontes parciais foram retidos
em [candidate-cancelled.json](../../lab/results/construction/diagnostics/terminal-footnote-break-2026-10-26/candidate-cancelled.json), sem aprovação
fictícia ou uso da execução cancelada para qualificar o produto.
