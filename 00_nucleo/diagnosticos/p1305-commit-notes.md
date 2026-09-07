# P1305 — consolidação autorizada

O dono autorizou o commit após o fechamento do P1305. Foi criado
`b303f1f15b610e09872b567027e0d806387fde8c`, incluindo a implementação
certificada e os predecessores P1303/P1304 ainda pendentes de versionamento.
Nenhuma dessas evidências foi reescrita. Não houve push.

O certificado P1305-r2 continua com SHA-256
`414479e20c93251488e1840f1298118fc35e3b4698ba73ae35b7e923c769dccc`.
O diff binário de código/L0 entre `8eb41b769eb840c7ab1063f981f98fdd4047952b`
e o commit acima, limitado a `00_nucleo/prompts` e `01_core`, tem SHA-256
`a6a964ac05ef32c26cca69e478af130ce3068a015182b034e1cc360bab5a8ae5`:
é exatamente o estado produtivo previamente testado. Não foram repetidos
build/workspace tests nesta operação de consolidação.

O staging expôs 135 avisos de whitespace antes invisíveis ao diff de arquivos
tracked: uma linha em branco no EOF de `p1303-red-tests-receipt.md:4309`,
uma em `p1304-baseline-status.txt:186` e 133 linhas do
`p1304-owner-ledger.tsv` terminadas em tab para campos vazios. A medição foi
`git diff --cached --check`, exit 2, antes do commit acima, com os 79 arquivos
certificados/predecessores staged. Código e L0 passaram separadamente em
`git diff --cached --check -- 00_nucleo/prompts 01_core`.

Os bytes históricos foram preservados deliberadamente: remover whitespace
invalidaria pins e, no TSV, poderia remover a coluna final vazia. Não se
alega que o check integral do staging tenha passado. Os relatórios antigos
que dizem “sem commit” descrevem o instante original de execução, não o
estado posterior a esta autorização.
