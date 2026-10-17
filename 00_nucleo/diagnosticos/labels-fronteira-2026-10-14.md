# Labels permanecem na página do conteúdo já composto

A referência de página de um título já composto não acompanha mais o texto
seguinte durante uma quebra automática. As sondas passaram de page 2 para
page 1 e de R8 para R7. A captura dos labels foi corrigida; a navegação por
link interno ainda tem um defeito independente no exportador PDF.

## Evidência e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshot inicial 2026-10-14T04:57:41.348620+00:00, em before.json.gz, com fontes, L0, núcleo,
status e git diff HEAD --stat. red-state.json.gz preserva o RED funcional;
candidate.json.gz conserva o candidato e pre-validation.json.gz o estado
ressellado. workspace-pre.json.gz inclui os fontes Rust da suíte inteira.
Não atribuir todas as alterações da árvore a este reparo.

links-captura-2026-10-14/followup-probes/results.json.gz documenta a falha
anterior no CLI. O título estava na primeira página; o label migrava para
a seguinte. A consulta do título continuava mostrando página física 1 e
contador 7. Em references.rs, o label é registrado após o corpo; Heading
já fechou sua linha nesse ponto, mas position_marker.rs colocava o carrier
na linha pendente. A admissão do próximo texto levava o carrier para outra
página. O RED unitário confirma página 2 em vez de 1 sem mudar os textos
ou as páginas dos controles.

## Correção e testes

O owner de position_marker foi atualizado primeiro. Um Label pós-corpo
usa o buffer comitado quando ele já contém conteúdo e a linha pendente não
tem conteúdo causal. Nos demais casos continua na linha pendente. Capturas
de Location anteriores ao corpo mantêm seu caminho. O ponto capturado,
a ordem de escrita e a projeção final não foram alterados. A correção é
interna, sem API, default ou fase nova, conforme ADR-0127.

O teste do título seguido de texto que exige nova página falhou antes e
passou depois. Os controles verificam label em corpo admitido na próxima
página e label inline entre prefixo e sufixo. Todos conservam os textos e
as páginas do controle sem label. Os dez testes do helper passaram,
incluindo matrizes, rollback, última escrita, cópia visual e Pagebreak
explícito. O primeiro erro de import do fixture está separado em
fixture-compile.log; não é considerado RED comportamental.

## Resultado no CLI e no leitor PDF

Sondas de 2026-10-14T05:00:53.308844+00:00 a 2026-10-14T05:01:07.070387+00:00; Tekt SHA-256
e5d459b24cd63b28ee1d68ad1bb8342b311a1b96ac76f255802cf765900997f2. Vanilla SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Fontes, comandos, saídas, PDFs, XML e qpdf JSON estão em probes; hashes dos
binários foram conferidos antes/depois. probe-checks.json registra cada
predicado executado.

| Sonda | Antes | Depois | Vanilla pinado |
|---|---|---|---|
| Referência de página do título | page 2 | page 1 | page 1 |
| Numeração funcional iniciada em 7 | R8 | R7 | R7 |

A consulta do título mantém página física 1 e contador 7. A entrada do
destino nomeado no PDF aponta para o objeto da primeira página. Controles
de wrap, linhas explícitas, URLs, corpo vazio e sufixo fora do link seguem
passando. A diferença de rodapé automático do caso page-ref não foi
corrigida nem classificada como equivalência geral nesta medição.

A verificação adicional com MuPDF 1.23.10, registrada em reader-probes.json.gz,
mostrou que o link interno não é resolvido pelo leitor: resolveLink retorna
-1 no Tekt e 0 (primeira página) no vanilla para page-ref, logical-ref e
label-ref. URLs externos e URLs com parênteses internos são reconhecidos
corretamente. A API usada está na [documentação do MuPDF 1.23](https://mupdf.readthedocs.io/_/downloads/en/1.23.0/pdf/).

A inspeção de navigation.rs:333-417 mostra um dicionário de destinos por
chaves de nome emitido dentro de Catalog.Names.Dests. Esse local exige uma
árvore de nomes, com chaves de string; referências à árvore também usam
strings. O formato de dicionário com nomes pertence a Catalog.Dests,
conforme [ISO 32000-1, seções 7.9.6 e 12.3.2.3](https://developer.adobe.com/document-services/docs/assets/35e4369068f86065372c18787171a17e/PDF_ISO_32000-1.pdf).
Essa mistura explica a falha de navegação e é o próximo reparo. Asserções
antigas que só procuram /Dests ou /sec1 não provam resolução por um leitor.

## Validação e pendências

Suíte workspace serial em Linux: 8416 passaram, 0
falharam e 7 foram ignorados, de 2026-10-14T05:00:14.500901+00:00 a 2026-10-14T05:06:08.231271+00:00.
TMPDIR em /dev/shm, oráculo pinado e linter publicado no PATH; fontes
inalterados durante a execução. Este estado inclui os reparos anteriores
de URI e altura dos links, além da captura de labels.

Build, check de todos os alvos, Rustdoc com -D warnings, formatação,
linhagem estrita V1/V5/V15/V26, linhagem reversa e git diff --check passaram.
Clippy de inventário mantém 228 avisos, sem variação por regra.
O lint arquitetural tem 0 erros,
382 warnings e
1631 notas. Os avisos e o gate zero violations
continuam abertos; não se declara produção completa.

Pendências imediatas: corrigir a árvore de destinos e a representação das
referências PDF, testar resolução independente e seleção de páginas;
verificar prioridade de links aninhados sobrepostos. Corpos rotulados
multipágina, Box fracionária e os demais itens da auditoria mantêm seu
escopo aberto. Não houve commit, push ou publicação.

Evidências e receitas em evidence.json.gz, com SHA-256 verificado após
releitura e descompressão.

## Seguimento da exportação

destinos-pdf-2026-10-14.md registra o reparo da árvore de destinos e das
referências por strings. O MuPDF passa a resolver os três links internos
destas sondas para a primeira página. A falha separada de associação de
labels criados por expressão permanece aberta no diagnóstico posterior.
