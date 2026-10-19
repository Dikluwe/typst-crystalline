# D07 — resíduos de sintaxe e spans corrigidos

Estado: os três controles negativos registrados no D07 estão corrigidos,
inclusive diagnósticos integrais. D08 medido separadamente e ainda aberto.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Todos os dados em
math-residuos-table-cell-2026-10-06 conservam UTC, diff HEAD --stat, status,
hashes/conteúdo das fontes, entrada, argv/cwd/ambiente e executáveis congelados.
Snapshot baseline-state anterior ao L0; final-code conserva o estado final.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt final SHA-256 `fe117e7558a8a6df3dee9ce16b619ab7e3f26a83d8d6fcc26dc82c12a3f23236`.
Coleta final: 2026-10-06T22:02:01.095780+00:00 a 2026-10-06T22:02:06.660386+00:00.
final-results.json preserva também horários/hashes das regressões e logs.

## Medição antes da decisão

Before mede bare-none/left/center aceitos ou desviados a casts enquanto
vanilla reporta unknown variable com hints matemáticos; valores Code #none
e #left funcionam nos dois. #auto ainda era classificado como content.
args nomeados inválidos usam mensagem equivalente mas span agregado.

Fonte: upstream foundations/scope.rs:81–100 usa locals e math em get_in_math,
e math.rs:45–55 conserva esse caminho. Tekt math/arguments.rs:40–75 reparseava
none como Code e :115–130 desviava MathIdent ao global. matrix.rs:263–271
passava args.span aos casts, embora Args já tivesse value_span por ocorrência.
Sintaxe, namespaces e diagnóstico são observáveis de linguagem. Inferência de
causa foi testada pela correção dessas rotas; medição after a confirmou.
Não há inferência de intenção baseada somente no comportamento.

## Materialização e RED→GREEN

L0 primeiro, correção interna ADR-0127 sem mudança de assinatura ou fase.
Wrapper unitário contendo MathIdent permanece matemático; globals desviam
somente Ident de Code. Auto explícito percorre eval_expr como os outros
literais Code. Binding MathIdent local conserva o Value real até o cast. Casts nomeados de mat usam value_span não detached da ocorrência,
com fallback agregado para Args sintético. Ordem de validação é conservada.

arguments-red.log falha porque bare-none era aceito; span-red.log falha porque
augment inválido usava a âncora agregada. Ambos são REDs válidos. Na suíte
core-final.log, ambos passaram. Dois owners ressellados exclusivamente, com hashes
em seals.json. V15/V26 válidos antes do resselo. Receipt final-results confirma
zero alteração externa nas fontes/L0 desde o baseline deste trabalho.

## Resultados finais

- Nova coleção final: 20/20 controles de resíduos coincidem integralmente;
  6 controles D08 ficam separados (2 coincidem e 4 seguem negativos).
- D07 original: 29/29 em matrix-morfologia-2026-10-06/final-residuos-locais.json.gz.
- D07 adicional: 14/14 em final-residuos-locais-additional.json.gz no mesmo diretório.
  Esses conjuntos não são somados aos novos controles, que incluem repetição.
- D05: 29/29 em equation-where-2026-10-06/final-residuos-locais.json.gz.
- Query: 36/36 em query-compilada-2026-10-06/final-residuos-locais.json.gz.
- D06: 38/41 em math-bare-2026-10-06/final-residuos-locais.json.gz, mantendo negativos
  root-render, raw-eval e raw-mat-eval. Esses débitos anteriores não são fechados
  por reparar namespace/spans do D07.

Core final (core-final.log, estado completo em tested-final-code.json.gz):
6.577 passaram e 5 ignorados; infra: 1.194 passaram; CLI query: 10 passaram.
Builds debug/release concluídos; git diff --check limpo.
Linter final: zero erros, 533 warnings e 1593 notes.

## Continuação D08

Diagnóstico table-cell-medicao-2026-10-06.md distingue selector/where/query/
counter/show/posição/padding. D08A tem pacote L0 completo para a extensão
pública do enum, bloqueado até confirmação ADR-0127:
00_nucleo/materialization/table-cell-selector-2026-10-06.md.
D08B é a frente nomeada para realizar/show das células e permanece separado.

## Refutadores adicionais e estado final

local-refuters-before refutou conservação de tipos locais no primeiro reparo:
locals None/Align/Auto/Bool tinham sido convertidos a display Content. O L0
de arguments foi atualizado antes de outro teste RED válido (local-red.log),
que falha para aa=left. MathIdent local agora conserva seu Value clonado; o
wrapper unitário delega à mesma rota. Os quatro controles local-refuters-final
coincidem integralmente. Não somar esses controles a coleções com repetição.

As coletas after e final-residuos sem sufixo locais são intermediárias;
pre-local-final-code e pre-local-final-results preservam seu estado.
O certificado atual usa final, local-refuters-final e final-residuos-locais.
Builds, core, infra, CLI query, lint e coletas bilaterais foram revalidados após
a correção dos tipos locais; verify.py reproduz a conferência dos receipts.
