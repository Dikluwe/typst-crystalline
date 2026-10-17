# Warnings CLI — materialização do pacote confirmado

O humano confirmou o pacote em 2026-10-06, resposta `confirmo`. O recibo em
00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json preserva os
hashes base/candidato e a proposta original. L0s proprietários foram guardados
antes dos testes/código. A ADR-0045 recebeu adendo substituindo a ordem histórica
P1139 §8; a decisão antiga continua registrada como histórica.

## Medição antes da decisão

Coleta before.json.gz nesta pasta de evidências comparou os binários no mesmo
cwd e ambiente registrado. O controle HTML refutou a inferência do candidato:
o aviso experimental pertence ao início da coleção de warnings upstream.
Assim, a ordem ratificada/refinada é errors → experimental → domínio em compile,
e errors → domínio → depreciação em query. Em sucesso, a ordem relativa dos
avisos existentes continua. O status observado permanece primeiro.

## O que mudou

L2 possui `query_deprecation_hint(&QueryIntent) -> String`, pura, e gera a
expressão query com `.first()`, `.map(...)` ou acesso de campo apropriado.
Campos que não são identificadores usam `.at(repr)`; escape de shell vem da
dependência exata shell-escape 0.1.5. As funções puras específicas de Unix/
Windows evitam o wrapper que consulta MSYSTEM. A conformidade medida é Unix;
Windows/MSYS não foi medido. O caminho em --in conserva input.display(), como
observado no oracle, inclusive espaços e caminho absoluto informado.

Query emite depreciação depois de diagnósticos de compilação ou de publicação
bem-sucedida; falha de aquisição/cardinalidade/campo não recebe depreciação.
Stdin usa SystemWorld::for_stdin e sua Source registrada. Origem e trace de
imports agora são resolvidos pelo formatter existente. Compile drena errors
antes dos avisos, mantendo a ordem experimental/domínio e os demais efeitos.

Somente estes quatro pares produtivos foram ressellados:

- 00_nucleo/prompts/shell/cli.md → 02_shell/src/cli.rs.
- 00_nucleo/prompts/wiring/query.md → 04_wiring/src/query.rs.
- 00_nucleo/prompts/wiring/compile.md → 04_wiring/src/compile.rs.
- 00_nucleo/prompts/wiring/tests/cli.md → 04_wiring/tests/cli.rs.

Cargo.toml/Cargo.lock registram a dependência; nenhuma API de watch, fase
matemática, enum público ou unsafe novo foi introduzido. Gates PDF/bundle
pendentes não foram ativados. Ownership/núcleos passaram antes do resselo.

## RED e validação

red-shell.log: RED de presença da nova API (E0425), distinto de RED comportamental.
red-cli-corrected.log reproduz falhas de cardinalidade/hint e Source stdin;
red-html.log reproduz a ordem experimental/domínio/errors. As primeiras versões
dos testes usaram nomes de warning incorretos, foram corrigidas por consulta à
coleta antes da implementação; os logs iniciais também foram conservados.
Green-shell-final.log contém 88 testes L2 passando no estado final; green-cli-focused.log, três
controles novos passando. A suíte completa é green-cli-full.log.

A expectativa histórica de diagnostic_paths_sibling proibia caminho absoluto
em todo stderr. O novo hint deliberadamente usa o input fornecido, como vanilla.
O teste foi refinado para continuar exigindo path relativo no diagnóstico e
permitir o caminho original no hint; green-path-refinement.log comprova GREEN.
O snapshot materialized.json.gz registra o estado da rodada completa; delivered
registra a expectativa refinada. Esta alteração só muda o teste/L0 de teste.

## Proveniência da medição entregue

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitada.
Coleta UTC 2026-10-06T17:51:52.160540+00:00 até 2026-10-06T17:51:54.556795+00:00.
After.json.gz contém git diff HEAD --stat/status, hashes dos arquivos alterados,
fontes, cwd, ambiente, comandos, stdout/stderr/status e texto extraído dos PDFs.
Snapshot final.json.gz conserva conteúdo e hashes; preservation.json verifica
que o trabalho anterior fora do escopo foi preservado. Seals.json registra os
quatro pares e seus hashes. Binários:

- Tekt: 62b3ec72c649269ddf5773bd2824d7c80abb768b8bbff2a0e8e4ab1199995696.
- Vanilla: 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea,
  pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489.

Summary.json registra 20 dos 23 controles com status/stdout/stderr idênticos
na coleta entregue. É o recorte desta frente, sem percentual de paridade global
ou igualdade mecânica de PDFs. Os controles PDF warning-only preservam o texto.

## Resíduos comprovados e estado do plano

- query-stdin-callback: `#context bad()` ainda retorna sucesso no Tekt enquanto
  o oracle executa o callback e reporta unknown. Já divergente no baseline;
  permanece débito de realização/contexto, não fonte direta corrigida por D03.
- query-selector-quotes: hint coincide, mas Tekt rejeita selector composto
  antes do erro do documento. Débito de selector, fora do owner desta mudança.
- query-missing-file: ambos rejeitam sem depreciação; mensagem/status continuam
  divergentes (Tekt 2, oracle 1). Preservado o contrato de aquisição existente.

D02 está materializado e validado no recorte. D03 fecha Source direta/import,
com callback contextual ainda aberto. D05–D13 e demais lacunas continuam conforme
plano-debitos-2026-10-06.md. Não houve benchmark de desempenho.

## Resultado final dos checks

A rodada completa CLI executou 121 testes: 120 passaram, e a expectativa histórica
sobre path no hint falhou. Após o refinamento descrito, o único teste falho foi
reexecutado e passou em green-path-refinement.log. Não se declara uma segunda
rodada completa de 121 testes no estado refinado. Os três novos controles também
passaram; a mudança final afetou somente a expectativa de path e sua linhagem.
Os dez testes unitários do binário passaram em green-wiring-bin.log (o pacote
não possui lib target; a tentativa --lib foi conservada no log correspondente).
Build-debug.log/build-release.log passaram. Diff-check passou.
Linter final.sarif: zero erros, 528 warnings, 1578 notes; nenhum V5/V6/V15/V26
nos quatro owners alterados. Avisos herdados fora do escopo permanecem: não é
zero violations global. Não houve resselo global nem mudança para ocultá-los.

## Atualização dos resíduos

O pedido seguinte de corrigir os três resíduos já materializou missing-file;
query-residuos-execucao-2026-10-06.md registra RED/GREEN e controles. Os dois
outros têm proposta concreta de mudança de pipeline/API, sujeita à ADR-0127,
em materialization/query-compilada-2026-10-06.md; ainda não materializados.

## Continuação ratificada

O gate query-compilada foi confirmado e materializado; os três resíduos
originais agora coincidem nos controles. Relatório atual:
query-compilada-execucao-2026-10-06.md. As menções a gate pendente acima
são registro histórico da etapa anterior.
