# P1308 — implementação e resultados

Estado: código implementado; **aceitação bloqueada por conflito de contratos
históricos sobre traces**. Oráculos preservados, mutantes pausados antes de build.
Nenhum stage, commit ou push realizado.

## Correções entregues

- `Args + none` e `none + Args` devolvem Args intacto, incluindo ocorrências,
  ordem e origens. Não passam pelo join Args/Args, que destaca o agregado.
- `Func` guarda origem privada sem alterar igualdade, hash, nome, namespace
  ou repr. Closures usam o span dos parâmetros; aliases e With o conservam.
  `arguments.filter` usa essa origem e o nome de tipo ratificado no cast bool.
- `panic` recebe a chamada inteira por identidade nativa, inclusive With;
  não há reconhecimento por nome de binding nem remapeamento textual do erro.
- Eval de expressão fornece sua Source já analisada por um World privado.
  Todos os outros serviços continuam delegados. Args.filter/map também passam
  pela regra existente de contenção de traces, antes omitida no caminho rápido.
- As expectativas históricas de Location, With e Args longo foram migradas
  por autoria independente. Args longo já era correto no baseline: não se
  alterou seu formatter produtivo para fazer o teste passar.

## Evidência funcional e proveniência

Referência: upstream vanilla ratificado `a51e02804`; binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` com working tree não
commitado. Estado inicial completo em `p1308-baseline.json`, SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`.
Cada recibo de comando abaixo inclui argv, UTC, exit, stdout/stderr, HEAD,
diff/stat e estado exato antes/depois; não se usa só a string `--version`.

| Execução | Resultado | Recibo em diagnosticos |
|---|---|---|
| Medição independente, antes do candidato | 1416 processos, sem instabilidade | `p1308-measure.json` |
| Teste histórico de Args, antes da migração | 1 falha reproduzida | `p1308-baseline-focal.json` |
| RED completo | 6 passam, 12 falham semanticamente | `p1308-red-supplement.json` |
| Primeiro candidato | 13 passam, 5 falham por trace ausente no caminho rápido | `p1308-green-1.json` |
| Candidato corrigido, sem mudar testes | 18 passam, zero falhas | `p1308-green-2.json` |
| Linter | exit 0, sem erros bloqueantes; warnings/info permanecem | `p1308-lint.json` |
| Formatação | exit 0 | `p1308-fmt.json` |
| Workspace completo | 6619 passam, 1 falha, 3 ignorados preexistentes | `p1308-workspace-tests.json` |
| Build release | exit 0 | `p1308-build.json` |
| CLI P1308, quatro perfis | 236 Preserved, 0 Violated, 0 Unknown | `p1308-public-focal.json` |
| CLI P1308, ordem inversa | 236 Preserved, 0 Violated, 0 Unknown | `p1308-public-focal-reverse.json` |
| Matriz pública R6 sem alterar expected | 1906 Preserved, 76 Violated, 0 Unknown | `p1308-public-matrix.json` |

Binário P1308 medido: `/dev/shm/p1307-r4-target.8W1BEA/release/typst`,
SHA-256 `09725fff8b4c472ee6b50a9ca6105d90268b2a0da8a22f1d8abd0f45e20c4c50`.

O refinamento após o primeiro candidato corrigiu roteamento real, não o
oráculo: `call_dispatch` retornava o resultado da coleção antes de trace.
L0 atualizado primeiro, sucessor independente em
`p1308-manifest-successor-1.json`; expectativas anteriores permaneceram intactas.

## Limites explícitos

### Conflito de preservação — não ocultado

A matriz R6 resolveu as 56 células antes divergentes, mas revelou 76
violações novas em 19 controles baseline × quatro perfis: quatro de
`cbor.encode` e quinze de decoders/read. Todos passaram a publicar traces
com a Source resolvível. O contrato R6 congela os transcripts antigos sem
esses traces. O resultado permanece **Violated**, não se removeu trace do
observador para trocar o rótulo por sucesso.

Recibo da matriz: SHA-256
`0a7094e95b259ae98a9325d29a45f7ab0d5c7b3263e5967b4c8d75f55e64310a`;
oráculo R6 permanece SHA-256
`99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6`.
Verificação independente: `p1308-verification-final.json`, SHA-256
`06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60`.
O reparse offline dos 1982 registros pelo observador congelado confirma
1906/76/0. Em todas as 76 violações, somente stderr muda: o esperado é
prefixo literal e o acréscimo é um trace; os demais campos são idênticos.
As 56 divergências anteriores estão Preserved, sem interseção com as novas.
Nada disso transforma os 76 resultados em aprovação do contrato vigente.

A mesma verificação confirma zero falhas de integridade em L0 sucessor,
18 testes novos, três migrações, predecessores, HEAD e index. Veredito:
**BLOQUEADO_POR_DECISAO_CONTRATO**. A lease de builds dos mutantes foi suspensa.

`p1293_d_preexisting_arity_and_serializer_transcript_is_frozen` exige o
transcript antigo de `grid.cell()` sem trace. A execução global
(`2026-09-07T21:06:46.540328+00:00` a `21:09:27.130829+00:00`, estado
integral no recibo SHA-256
`d6d8ac5baf46ec195a164965c5140819c1dd00ec08c93e3b0a9063c507c937c2`)
mostra a mesma mensagem `grid_cell() exige body como argumento posicional`,
acrescida de `while calling cell` para a chamada resolvível. O overlay
expõe a Source; a regra de trace não foi modificada para esta função.

Essa expectativa continua intacta: o L0 P1293 autorizou somente a migração
da repr de With, não a revisão deste transcript congelado. Não se suprimiu
o trace para obter GREEN nem se chamou o workspace de aprovado. É necessária
decisão sobre migrar também esse controle histórico e os 19 casos R6,
mantendo mensagem, span primário e validação e admitindo o trace causal da
Source. Não corrigir CBOR/decoders ou suprimir traces oportunisticamente.

### Dívidas e alcance da verificação

Named inválido de panic continua dívida: mensagem portuguesa e span primário
detached não são paridade. Agora recebe o trace natural quando a Source está
disponível; os controles distinguem essa consequência da correção da mensagem.

A skill tekt-materializacao-segregada orientou a autoria separada dos testes
e a verificação dos artefatos. Regime: **executado sem atestação de isolamento
técnico**. O checkpoint independente de integridade ocorreu antes do candidato;
os arquivos de manifesto/selo limitado foram emitidos depois da mensagem de GO
e do início do código. A cronologia está explícita, sem reversão artificial.
Não há alegação de selo discriminatório pleno anterior à implementação.

Os seis mutantes P1308 foram pausados antes de qualquer build: zero executados,
nenhum mutation score. Repetição global adicional e mutantes não resolvem
o conflito de intenção entre Source resolvível e transcripts congelados.
As 37 famílias históricas P1307 não são quitadas por esse plano e continuam pendentes; este
relatório não é certificado geral de P1307 nem de equivalência do compilador.
