# P1317 — CSV inválido em UTF-8 sem detalhes internos do parser

Estado: implementado e validado; revisão independente PASS no recorte. Sem commit.

## Efeito entregue

`csv(bytes((255,)))` antes explicava o erro com `CSV parse error: record
0 ... invalid utf-8 ...`. A correção troca essa causa por uma mensagem clara:
`failed to parse CSV (file is not valid UTF-8)`. Vale para cabeçalho e dados,
array e dictionary, Bytes e leitura por Path/Str.

A origem Bytes ajustada no P1316 fica preservada. Não há correção da posição
textual `at l:c`, do caminho `in path` ou da origem dos erros de arquivo.
Essas diferenças para o vanilla continuam abertas, assim como coerções
Symbol, argumentos desconhecidos/ausentes/excedentes e csv.encode ausente.
O recorte não promete paridade geral CSV.

## Por que esta correção

A fonte ratificada `a51e02804`, `loading/csv.rs:138-157`, escolhe explicitamente
`file is not valid UTF-8` para a variante Utf8. Posições são acrescentadas
separadamente por `diag.rs:845-925`. O owner cristalino `loading.rs:955-965`
entrega o Display do parser para essa variante. A mensagem é observável da
linguagem, não uma obrigação de copiar a estrutura interna vanilla.

A medição inicial registrou 26 execuções bilaterais em
`p1317-measurement.json`, SHA-256
`92b04d68105d573523c673c1bcd0fb64ec2ffe57d17788c79742ce0450ee2fdf`.
Não são 26 provas de parsing: três construções de concatenação Bytes são
rejeitadas antes do CSV pelo baseline, e duas sondas de arquivo inexistente
medem apenas I/O. Foram conservadas, sem transformar falhas de sonda em RED.

A correção focal usa arrays de bytes literais: oito novas execuções em
`p1317-measurement-focal.json`, SHA-256
`2a995791a497c9579fa8854d0531eebc74428b71afe92d59f249ec6143fa4fe0`.
Elas confirmam a mensagem em dados posteriores e que falta de campos vence
UTF-8 inválido no mesmo registro. Portanto a correção não pode fazer validação
UTF-8 antecipada. A colisão parsing+excesso positional continua dívida de
precedência: quando o baseline chega a Utf8, só o texto deve mudar; o vanilla
rejeita o excesso primeiro. Esse efeito não é igualdade bilateral.

## Proveniência e processo

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree P1315/P1316
não commitado. A medição inicial preserva o texto completo e hash de fonte/L0,
hashes dos diagnósticos anteriores, diff/stat, status e UTC antes/depois.
Não atribuir o diff acumulado inteiro ao P1317. Nenhum commit está previsto.

Baseline `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256
`1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
Vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`, não inferido da string de versão.
Target exclusivo `/tmp/p1317-target.y5u9ah`, cache copiado sem hardlinks,
sem sobrescrever baseline; RAM livre insuficiente para cópia independente.

L0 atualizado antes dos testes/código. Regime A/B executado **sem atestação
técnica de isolamento**: root escreve L0/testes locais/implementação;
`/root/p1317_tests` deriva e congela oráculos sem ler fonte/testes candidatos;
`/root/p1317_review` verifica sem editar material julgado. Filesystem é
compartilhado. Sem selo completo de refinamento ou mutation score.
Unknown bloqueia; duas revisões sem ganho na mesma causa exigem rever desenho.

Uma tentativa de gerar o novo recorder por subprocesso no sandbox ficou
pendente; os processos exatos foram identificados e encerrados. O arquivo
foi criado pela ferramenta de patch e os recibos executados no namespace
host autorizado. Nenhum código produtivo foi alterado por essa tentativa.

## Resultados

Ownership/núcleos: V15/V26 passaram antes da materialização, recibo
`p1317-lineage-preflight.json`, SHA-256
`4c9839b10a01a48e631deeef4abd3779f2044398b04dff7cd75f7b86b638d4e1`.
RED genuíno: três testes falham pela mensagem antiga; um controle de
precedência/Unicode válido passa. Não é falha de compilação. Recibo
`p1317-unit-red.json`, SHA-256
`7d5f5a4af287cbe4ca0d78ffc6f4263a280e87d9457c04a1cdae2510a9b5eb18`,
exit 101, fonte/L0 estáveis antes/depois. A revisão de escopo em
`p1317-review-preflight.md` confirmou fluxo contínuo ADR-0127.

Freeze A/B `p1317-ab-freeze.json`, SHA-256
`0a701c7f10630c94c131f6e2923b45272a72290d29545592e49d8f370d6169f1`:
387 casos em quatro perfis, 1.548 expectativas e 208 RED. Os 330 casos
anteriores foram reproduzidos literalmente contra o P1316 antes do delta;
57 casos novos cobrem fronteiras e controles. A fixture binária nova também
mede Utf8 por Path/Str, distinguindo explicitamente a localização vanilla
da mensagem sem localização que este recorte exige. A medição/freeze do
testador não precisou de calibração.

Revisão `p1317-review-prepatch.md`: GO após RED e freeze, conferindo inputs
e binários. Só então entrou o braço ErrorKind::Utf8 no mapper já existente;
nenhum outro trecho produtivo mudou, exceto metadata de linhagem.

Identidades candidatas:

- Fonte SHA-256 `39cf6ef6c3cd55872dfc40cbe0787faab7379013beb3e399183d8cc97d22d86c`.
- L0 raw SHA-256 `99f8f50dc524eaa0e6742cfc6a3bb1db160895cd27ed04635721871cd5d6d7c2`.
- L0 normativo congelado `059aad946b30515b24ecfdef4c8490ab735d1bb86d374ce1371c899130ab0d51`,
  excluindo somente a linha canônica Hash do Código; resselo A `a87d59b8`, B `66b9758b`.

Linhagem final: `Nothing to fix`, recibo `p1317-lineage-final.json`, SHA-256
`bbca8667566d899a4f38a0ce75a2b7004ade51f595a990b087c875b02a23d68e`.
Fmt e diff-check também passaram, com fonte/L0 estáveis.
Lint completo: zero erros, 240 warnings e 1.137 infos, exit 0. Recibo
`p1317-lint.json`, SHA-256
`a853dc8609193642f98061cd61da074f8d94855648edd4b38691512290356b36`.
Não é ausência de avisos. Fonte/L0 permaneceram estáveis nessa execução.
GREEN: quatro testes passam, incluindo os controles de origem, Path/Str,
precedência e valores Unicode. Recibo `p1317-unit-green.json`, SHA-256
`3b31719ae7e1cbbbb3d739099be0c58ac592e25065a4443538be4b51c2b66c9b`,
exit 0, fonte/L0 estáveis. A revisão `p1317-review-candidate.md` conferiu
que retirar somente o braço Utf8 e restaurar a metadata reproduz exatamente
o source RED; os testes novos e anteriores permanecem íntegros.
Build workspace release: exit 0, fonte/L0 estáveis, recibo
`p1317-build.json`, SHA-256
`7a08e472aafdffd4e4962871e4b5c5c43d7e1417b3788bff1c5c7ee212179e8d`.
Binário candidato `/tmp/p1317-target.y5u9ah/release/typst`, SHA-256
`9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab`.
Workspace release: 6.656 testes passaram, zero falhas e três ignorados;
exit 0, fonte/L0 estáveis. Recibo `p1317-workspace-tests.json`, SHA-256
`a5b38085327ee2a4e9a09899ff4d5a2f07b39dc1aa197472165b3f9a667a8f28`.
A/B candidato: 4.644 comparações completas passaram nas ordens normal,
repetida e inversa, zero falhas e zero Unknown. Recibo
`p1317-ab-comparison.json`, SHA-256
`b3634213bbaaaa65620c44777368189125474b877a831dc073739821848dcdcd`.
Observações completas em `p1317-ab-candidate-runs.json`, SHA-256
`3b83fc33ca7524056dcbad55240e5f8afa90be8428b49bb76096695bf19176f6`.
O delta autorizado é somente a primeira linha dos casos Utf8 declarados;
o restante do diagnóstico baseline permaneceu literal. As diferenças de
localização vanilla permanecem registradas, não foram normalizadas para
alegar igualdade total. Recibo do testador `p1317-ab-receipt.md`, SHA-256
`fbbb9f37ccfd78de186d1a184526c55396c87dd964e21f59ecc7df9f22bdaa3e`.

Revisão independente: **PASS delimitado**, sem achado impeditivo.
`p1317-review-final.md`, SHA-256
`3b2ca1e235bec5221da0d00017e2b08d6b598d764201f597470301fe478d787e`.
Auditoria reproduzível `p1317-review-final-audit.json`, SHA-256
`17d74ba2e5e94fd812194087aa8cf8884c90262bff17a785443fa6a8bd6931a8`:
reconfere as observações A/B brutas, os 24 inputs congelados e os 59 artefatos
P1315/P1316, sem drift. Testes anteriores e obrigação normativa preservados.

Fechamento: causa textual Utf8 corrigida; localização e demais dívidas acima
continuam abertas. HEAD original mantido, índice sem alterações, nenhum
stage/commit/push ou remoção de temporários. Após o parecer, apenas este
fechamento documental e o status do passo foram atualizados.
