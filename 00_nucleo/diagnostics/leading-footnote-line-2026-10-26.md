# Nota com linha inicial vazia — 2026-10-26

**Corrigida a divergência do reprodutor original:** o marcador da nota agora tem
a mesma posição do vanilla fixado, sem aumentar seu glifo ou mudar a identidade
da quebra de linha. Os 21 reprodutores positivos qualificam em PDF/SVG, em debug
e release, a 0,001 pt. A correção da quebra terminal foi preservada integralmente.

## Proveniência e L0

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, árvore não commitada recebida após a correção terminal.
[baseline.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/baseline.json) conserva UTC, status, stat exato de
`git diff HEAD --stat`, os 2165 hashes de entrada e identidades dos executáveis;
[received-source.json.gz](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/received-source.json.gz) preserva os bytes.
Cada número se vincula ao estado e aos comandos do respectivo recibo.

Vanilla: commit fixado `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`, binário SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Não houve resync. O [encerramento](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/completion.json) vincula fontes
completos, inputs, binários finais, documentos e manifesto de evidências.

O [L0 de line_commit](../prompts/core/compiler/layout/line_commit.md) foi
atualizado antes da implementação. É correção interna de paridade sob fluxo
contínuo do ADR-0127: um único consumidor produtivo e seu L0, sem alteração
pública, novo modo, mudança de fase, reconstrução de Content ou nova execução
do corpo. Os Nuclei e seus pins permanecem iguais. O módulo footnote_fragments
recebido, incluindo a correção terminal, não foi modificado.

## Medição, causa e correção

[before](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/before/receipt.json) retém os primeiros 18 reprodutores;
[before-expanded](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/before-expanded/receipt.json) amplia a exploração a
24, incluindo LF/CRLF literais. Em DejaVu Sans 11 pt, o texto e as quatro páginas
eram preservados, mas o marcador da entrada começava 1,403865 pt abaixo do vanilla.
Nota: 9,35 pt; marcador sintetizado: 6,54226 pt. A metade da diferença entre esses
tamanhos é 1,40387 pt: a entrelinha de `0.5em` estava resolvida no tamanho reduzido
do marcador. Nimbus Sans, com marcador nativo no mesmo tamanho nominal da nota,
é um controle sem esse deslocamento. Outras combinações de tamanho/fonte expõem
também diferenças na admissão por página; diferenças locais de Y não qualificam
geometria quando o marcador está em páginas distintas.

[leading-study.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/leading-study.json) conserva Tf, caixas/páginas e
fontes dos dois compiladores. A entrelinha do parágrafo é resolvida antes de
coletar suas linhas no vanilla. O caminho uniforme legado do Crystalline usava
o estilo do último fragmento textual; na primeira linha vazia do corpo, esse
fragmento era somente o marcador sobrescrito.

Agora, quando o primeiro fechamento de linha de uma captura de nota habilitada,
fora de um bloco atômico e sem cortes anteriores, contém somente texto
sobrescrito, a entrelinha usa o estilo ativo do parágrafo da nota. O mesmo
visitante de estilos trata marcadores com mais de um fragmento. As dimensões,
o ascent e a baseline próprios dos glifos continuam iguais; não se insere texto
vazio nem se aumenta um strut para forçar uma classificação mista. As regras de
linhas comuns, fórmulas, colunas e sobrescritos fora dessa captura são preservadas.

A confirmação em release mede diferença máxima de 0,000006 pt na caixa do
marcador original, dentro do gate de 0,001 pt. Os recibos
[debug](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/leading-confirmation-debug.json) e
[release](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/leading-confirmation-release.json) conservam as caixas antes,
depois e na referência.

## RED→GREEN e validação

[red3.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/red3.json) é o RED assertional válido: um teste de entrelinha
falhou e três controles passaram. Os quatro testes novos passam no estado final:
entrelinha de marcador inicial, geometria do glifo, múltiplos fragmentos e regra
de sobrescrito comum fora da nota. Os fontes completos de cada tentativa foram
arquivados antes da execução. Os testes anteriores com observador/fixture ainda
incorretos foram retidos, sem serem usados como certificação RED.

[Exploração do candidato](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/candidate-exploration-evaluation.json) registra
24 comparações: 21 qualificam e três casos de quebras iniciais repetidas continuam
refutadores. [Classificação dos corpora](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/cohort-classification.json)
preserva todos os hashes e o motivo da separação. A validação final executa
novamente os dois corpora; não houve remoção silenciosa de casos falhos.

- 21 reprodutores PDF/SVG por build, a 0,001 pt: três famílias, texto principal
  de 8/11/16 pt, controles sem linha inicial vazia, combinação com quebra terminal
  e equivalentes com LF/CRLF literal.
- Os três casos com duas quebras iniciais são repetidos em ambos os builds,
  conservam todos os caracteres e permanecem geometricamente não qualificados.
- Os 48 casos da correção terminal passam nos dois builds, assim como R11 (68),
  R12 (54+4), R13 (25), R8 (28 PDF/22 SVG), R9 (75) e R10 (69 e bitmap).
- Os 21 replays e cinco suplementos A–H/R1–R7 passaram com auditoria de palavras,
  dimensões físicas, origens/regras SVG, campos/construtores, símbolos e pintura.
  Warnings conhecidos são conferidos contra seus baselines, não descartados.
- Os 12 gates completos passaram: 9954 testes Rust aprovados, sete ignorados e
  42 Python; workspace build, todos os targets, Clippy, fmt/rustdoc, estrutura,
  linhagem estrita/reversa e lint completo sem violações.

[pipeline.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/pipeline.json), [post.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/post.json) e
[closing-audit.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/closing-audit.json) vinculam os resultados ao mesmo
estado final de 2165 inputs e aos executáveis efetivamente medidos.

## Pintura e recursos

[Pintura](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/leading-paint/receipt.json) observa nove reprodutores por build em
todas as páginas PDF/SVG a 72/600 dpi: originais, controles sem a linha inicial e
texto de 8 pt nas três famílias. As observações são iguais entre debug/release.
Resíduos na amostra release:

- `DejaVu Sans-original`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `DejaVu Sans-original`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `DejaVu Sans-original`, PDF 600 dpi, página 4: 2 pixels, delta de canal 11.
- `DejaVu Sans-no-leading`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `DejaVu Sans-no-leading`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `DejaVu Sans-size8`, PDF 600 dpi, página 1: 2 pixels, delta de canal 11.
- `DejaVu Sans-size8`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Nimbus Sans-original`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Nimbus Sans-original`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `Nimbus Sans-original`, PDF 600 dpi, página 4: 2 pixels, delta de canal 11.
- `Nimbus Sans-no-leading`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Nimbus Sans-no-leading`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `Nimbus Sans-size8`, PDF 600 dpi, página 1: 2 pixels, delta de canal 11.
- `Nimbus Sans-size8`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Liberation Serif-original`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Liberation Serif-original`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `Liberation Serif-no-leading`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.
- `Liberation Serif-no-leading`, PDF 600 dpi, página 3: 2 pixels, delta de canal 11.
- `Liberation Serif-size8`, PDF 600 dpi, página 1: 2 pixels, delta de canal 11.
- `Liberation Serif-size8`, PDF 600 dpi, página 2: 2 pixels, delta de canal 11.

[Pintura dos 32 refutadores](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/refuter-paint/receipt.json) conserva as
demais observações. Pintura e geometria permanecem gates distintos; não se
declara igualdade universal de pixels.

Os gates originais R6 passaram nos sete workloads, com repetibilidade, latência
de math contra o controle definido e os três stresses. Razão de tamanho
normalizado atual/R5: **0.855782837**; razão geométrica RSS atual/R5:
**0.948586018**. O controle R5 permanece o binário SHA-256
`c3d2d2e416e506f9e49bff28d55a05138030440a6288d88bc77631348dae5549`.
O HEAD atual não é apresentado como o HEAD histórico dessa construção, nem
se atribui ganho causal de recursos à correção. [Recursos](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/revalidation/final-resources/receipt.json)
e [stresses](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/revalidation/legacy-resources/receipt.json) retêm as amostras.

## Preservação e limites

[Preservação](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/preservation.json) confere os 2163 inputs recebidos fora
do par autorizado e 122231 arquivos históricos, incluindo todo o encerramento
da correção terminal, além dos assets/fontes. A licença DejaVu Sans já normalizada
em HEAD212 continua idêntica ao HEAD recebido; não é alteração desta tarefa.
As alterações existentes de R11/R12/R13 e da quebra terminal foram preservadas.

Dos 32 refutadores anteriores, 17 qualificam e 15 permanecem abertos; o único que
muda de classificação nesta tarefa é `note-long-leading-blank`. São fontes de
teste, não causas independentes ou o total global de defeitos.
As três fontes adicionais de quebras iniciais repetidas estão registradas em
[blank-line-refuters.json](../../lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/blank-line-refuters.json). A segunda linha é
vazia de verdade, diferente da primeira linha com o marcador. Seu certificado
de fragmentação continua limitado; esta correção não remove essa linha nem
generaliza a certificação das demais linhas vazias.

A primeira tentativa RED foi interrompida para medir a entrelinha usando o
ascent seguinte realmente registrado, em vez de presumir o ascent do marcador.
A segunda ainda não instalava explicitamente TextStyle.size de 9,35 pt: o
parâmetro legado de Layouter::new não altera esse estilo. Os fontes/logs foram
preservados, e só o RED corrigido foi usado como evidência assertional.
