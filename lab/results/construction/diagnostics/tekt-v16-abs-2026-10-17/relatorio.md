# Relatório — V16 em calc.abs

Etapa concluída. **V16 125 → 124**; **V19 1065 → 1066 notas**; **V20 867 notas** preservadas. O gate global continua aberto pelos 124 V16. Nenhum contrato público, default, fase, dependência ou núcleo mudou; não houve commit/push, merge ou release.

## Correção e justificativa

Apenas `01_core/src/compiler/stdlib/calc/abs.rs` e seu L0 proprietário mudaram. L0 atualizado antes do código; resselo restrito ao par. O wildcard que delegava o vocabulário a type_name foi substituído pela enumeração explícita de Value: Str→string, Bool→boolean e outras 37 alternativas com a política comum de nome canônico. Nenhuma admissão numérica foi ampliada; tipos já tratados antes permanecem com suas condições.

O grupo V19 está prescrito no L0 pela política comum de diagnóstico, sem supressão. Uma variante nova agora exige classificação pelo compilador nesse consumer. Executar o braço não comprova cobertura individual de 37 alternativas. Não foi alegado um erro no linter nem modificada sua política global.

## Testes e evidência

A alteração preserva runtime. O **RED é estrutural**, explicitamente: o match produtivo antigo compila tanto com as 39 variantes atuais quanto com uma variante futura, violando a exigência nova de revisão. O GREEN compila o controle de 39 e rejeita a variante futura com E0004. Match extraído da produção e aridades obtidas do enum real, payloads mínimos; rustc isolado, sem target Cargo compartilhado. Isso comprova a trava de compilação, não paridade de runtime.

**Quatro testes funcionais novos passaram antes e depois**: aliases/mensagem/span detached, nomes de famílias genéricas, erro/overflow antes de extras e preservação de tipos/zero assinado/domínio de Length. Não foram apresentados como RED funcional, pois não houve falha funcional corrigida nesse formatter.

Medições bilaterais: **39 pares / 78 invocações em cada fase**, sendo 37 testemunhas válidas de abs e dois casos de construção de literal. As 37 testemunhas coincidem com o pin nos códigos, resultados e mensagens/hints medidos. Antes/depois, expressão/exit/stdout/stderr completos permaneceram exatos em ambos os binários, inclusive nos dois casos mantidos como dívida.

Os dois negativos com -9223372036854775808 não chegam ao mesmo valor nos dois compiladores: vanilla rejeita a construção manual na validação de negação em typst-eval/src/ops.rs:116–133; Tekt observa Float e prossegue. Não contam como prova de overflow de abs. Os controles int.min, int.min com extra e bound with confirmam o overflow real e sua precedência. A promoção foi localizada no lexer cristalino, compiler/lexer/code.rs:167–178; o defeito permanece aberto para auditoria de limites, bases, sufixos e fases antes de materialização.

Suíte completa: **8796 testes aprovados, zero falhas, sete ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. Diferencial SARIF integral remove exatamente um V16 e adiciona somente o V19 prescrito; todos os demais diagnósticos permanecem após mapear linhas. SARIF global ainda retorna 1 pelos 124 V16 conhecidos.

## RAM e proveniência

Comandos pesados sequenciais, por gate em cgroup: **MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero, Cargo jobs=1**. Pico **8879628288 bytes (8.270 GiB)**; eventos memory.max **0**, memory.high **0**, sem OOM/OOM-kill. Memória do grupo inclui cache; não é RSS isolado nem prova de otimização de desempenho.

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. before.json.gz, snapshots dos controles, resultados do probe, coletas por fase, validation-result.json e final-state.json.gz guardam UTC/status/diffstat, fontes/L0/configuração, argv e hashes de binários/artefatos. Oráculo 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

[Inventário restante](inventario-v16-restante.md): 124 casos em 95 consumers. [Pendências](pendencias.json) e [auditoria seguinte do literal](literal-next-audit.md). A [triagem do próximo lote](next-batch-triage.json) reúne oito V16 em quatro consumers privados, ainda sem implementação. TS1/RG1 continuam aguardando as confirmações já solicitadas; seus manifestos completos e fontes protegidas foram conferidos e preservados. Ao retomar qualquer proposta, preservar esta correção independente do par abs. O objetivo geral permanece ativo.
