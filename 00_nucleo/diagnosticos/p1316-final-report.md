# P1316 — erro CSV aponta para os Bytes que o causaram

Estado: implementado e validado; revisão independente PASS no recorte. Sem commit.

## Mudança entregue e limite

Em `csv(bytes("a,b\n1"))`, o diagnóstico passa a apontar para `bytes(...)`,
em vez de aparecer sem origem. Quando os dados vêm de With ou Args, a origem
é a do argumento transportado, não a chamada posterior nem um named anterior.
O mesmo vale para falhas UTF-8. Origem realmente ausente continua ausente.

Isso não é uma reescrita das mensagens: ordinal P1315, texto do decoder e
demais campos dos erros devem permanecer intactos. Os traces continuam sob
responsabilidade do dispatch; sua apresentação acompanha a origem correta.
Não se acrescenta `at linha:coluna`, nem se troca a mensagem UTF-8 legada.

CSV lido por Path/Str permanece fora da correção. A medição mostrou que o
vanilla aponta erros de texto UTF-8 para dentro do arquivo externo; apontar
esses erros para o argumento seria incorreto. Diagnóstico de arquivo, texto
de parsing, coerção Symbol, named desconhecido e missing/excesso continuam
dívidas abertas. Não há promessa de paridade geral CSV.

Há uma fronteira adicional medida antes do patch: Bytes malformado junto a
positional excedente chega ao parsing no cristalino, mas ao erro de excesso
no vanilla. Essa precedência permanece; a nova origem do primeiro argumento
é efeito normativo, não paridade com o erro vanilla de outro estrato.
Os quatro casos originais continuam em `p1316-ab-cases.json` e
`p1316-ab-baseline-runs.json`, sem reescrita. O A/B mantém excesso com fonte
válida como controle literal, e um teste local com spans diferentes verifica
o primeiro-versus-segundo argumento na colisão parsing/excesso.

## Evidência e decisão

Antes do L0/patch, `p1316-measurement.json` registrou 24 execuções bilaterais,
com argv, cwd, saídas, fontes e estado. SHA-256
`b5d25936d78b8584cec7469fdde2a2cd5130da2f52936bcdd9f103efc28154d7`.
O span do argumento está explicitamente transportado no vanilla ratificado
`a51e02804`, `loading/mod.rs:79-110`, e usado pela rota Bytes em
`diag.rs:845-925`. A rota Path distinta e o controle Args.map detached
refutam uma correção genérica que atribua origem a todo erro.

O owner cristalino `loading.rs:1301-1302` retornava diretamente os erros
detached do decoder puro na rota Bytes. O L0 foi atualizado antes do código,
com substituição expressa somente da preservação de origem P1313/P1314/P1315.
Revisão preliminar classificou fluxo contínuo ADR-0127: correção de paridade,
sem assinatura pública, cast, fase ou dependência nova.

## Proveniência e independência

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree P1315 já não
commitado no início. `p1316-measurement.json` preserva texto e hash dos dois
arquivos modificados e hashes dos diagnósticos P1315. Os recibos P1316
registram UTC, diff/stat, diff, índice e hashes antes/depois. Não atribuir
o diff acumulado inteiro ao P1316. Sem stage, commit, push ou limpeza.

Baseline `/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.
Vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`, não inferido da string de versão.
Target exclusivo `/tmp/p1316-target.1c6HK7`, cópia independente do cache,
sem hardlinks, porque a RAM livre não comportava a cópia. Nenhum binário
baseline foi sobrescrito. Paths host de RAM/tmp exigem o namespace autorizado.

Regime A/B **executado sem atestação de isolamento técnico**. Root escreve
L0/testes locais/implementação; `/root/p1316_tests` congela oráculos sem ler
código ou testes candidatos; `/root/p1316_review` julga sem editar material
julgado. O testador não recebe o texto de código embutido na medição.
Filesystem compartilhado implica limites procedimentais, não isolamento
atestado. Unknown obrigatório bloqueia. Sem selo completo ou mutation score.

## Resultados

Primeiro RED: uma falha na origem esperada e dois controles passam, não erro
de compilação. Recibo `p1316-unit-red.json`, SHA-256
`dffd0be0d035fcfee6689c37fb62ab8c4f10a6e325881f0f416c161f2acac1c9`.
O L0 foi esclarecido para a fronteira de excesso ao fim dessa execução;
os hashes before/after mostram isso. Por isso foi executado RED-r1 com L0 estável
e o teste adicional antes de materializar. Nenhum código produtivo candidato
existia durante essa revisão do contrato.

Uma contagem do harness A/B também confundiu o texto interno `CSV parse error:`
com outra entrada diagnóstica; corrigida para início de linha, sem mudar
saídas ou mensagens esperadas. O testador registra a calibração e o custo.
Tentativas operacionais de apply_patch malformadas foram rejeitadas sem
editar arquivos; não foram falhas de implementação nem resultados RED.

RED-r1 concluído com L0/source estáveis antes/depois: dois testes falham
exatamente no span, dois controles passam. Recibo `p1316-unit-red-r1.json`,
SHA-256 `41a1ec7626e8e37d61644af4608388b8a203e844a7320f966f35c830c48ec431`.
O teste adicional conserva a mensagem de parsing na colisão com excesso e
exige origem do primeiro Bytes, distinta do segundo positional.

Freeze `p1316-ab-freeze.json`, SHA-256
`200816280fc2e1c18934103574c6d33c3d972bdfb576221c4a74d3175d385e5f`:
330 casos nos quatro perfis, 1.320 expectativas completas e 260 RED.
O replay de 290 casos foi validado literalmente contra o P1315 antes de
aplicar o delta de origem declarado; 40 casos/controles são novos. A medição
final composta registra quais execuções anteriores foram reaproveitadas,
sem contá-las como novas execuções. O suplemento `p1316-ab-excess-debt.json`
preserva a comparação bilateral das colisões parsing/excesso.

O `GO` independente em `p1316-review-prepatch.md` conferiu todos os inputs
pinados e reconstruiu as expectativas antes do patch. A mudança produtiva
ficou apenas no map_err de native_csv(Bytes): reusa os diagnósticos retornados
e altera somente span. Decoder puro, Path/Str e testes anteriores intactos.

Identidades candidatas finais:

- Owner SHA-256 `2278316a90408fc6dae9ba2fd0f3d5479571055ef4bcde84019147a84b12d75a`.
- L0 SHA-256 `a4e4a71be35e5cf9008a47badb8a51e830616b6a768d0c185c24f78d79bce7d3`.
- L0 normativo congelado `f429ebf31bd6723cf8a69b7be82f2c223ced005bf367655ef17cadd26026b112`,
  excluindo somente Hash do Código. Resselo A `988daa69`, B `716ab114`.

Lint passou com zero erros, 240 warnings e 1.137 infos; não é ausência de
avisos. Recibo `p1316-lint.json`, SHA-256
`7c81f950adb5b138338674f91293f47c75c09d9f133346716f3595bce92663f5`.
Fmt, diff-check e linhagem também passaram; `p1316-lineage-final.json`
registra `Nothing to fix`.

GREEN: quatro testes passam, incluindo o caso normativo parsing/excesso,
origem ausente e preservação Path/Str. Recibo `p1316-unit-green.json`, SHA-256
`5428ecdeaa3d85bcde4086f34d327e0abe7184c1bf03eb3b075b8f9a1bc3128e`.
Código e L0 permaneceram idênticos durante essa execução.
Build workspace release: exit 0, source/L0 estáveis. Recibo `p1316-build.json`,
SHA-256 `127c5eb0505f90012619906fc7e3bced8da2e12ec8a9db342043a7766b46a93f`.
Binário `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256
`1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
Workspace release: 6.652 testes passaram, zero falhas e três ignorados;
exit 0, source/L0 estáveis. Recibo `p1316-workspace-tests.json`, SHA-256
`3c0f23a0b279b0cefaa9c4b223efa32959a7594ceaae7fa4f079280e49220486`.

A/B candidato: 3.960 comparações completas passaram nas ordens normal,
repeat e reverse, sem falhas ou Unknown. Recibo `p1316-ab-comparison.json`,
SHA-256 `d80d70ca2b1a598df9491a6eab6b716fd44191975f366c405ba626c9ccb2ccae`.
As expectativas de origem preservam a primeira linha inteira do baseline
(mensagem) e comparam o restante ao vanilla (origem/traces); os controles
exigem preservação literal. Isso não fecha paridade das mensagens legadas.
Recibo do testador: `p1316-ab-receipt.md`, SHA-256
`51000e0815aa40b6c92a1784edaafafe194e9a70a1082301cfddaee3c53a367f`.

Revisão independente: **PASS delimitado**, sem achado acionável.
`p1316-review-final.md`, SHA-256
`65d3580edcf5381fdacd9b5d7d6974fef3371b68177eb39325757feaab50f303`.
O revisor reconstruiu as expectativas e comparou novamente todas as observações,
conferiu os 29 pins do freeze, os 28 artefatos P1315 e os testes anteriores.
A auditoria reproduzível está em `p1316-review-final-audit.json`, SHA-256
`e34e68b1f1b94787f581159d6731d91905dbf2b224764031e0cb8169c23552b5`.

Fechamento: origem de parsing Bytes corrigida, sem fechar as dívidas acima.
P1315 preservado; HEAD original mantido e índice sem alterações. Nenhum
stage, commit, push ou remoção de temporários nesta entrega.
