# Matemática determinística no gate de qualidade

As famílias trigonométrica, hiperbólica e logarítmica de `calc` passaram a usar
`libm 0.2.11`, já pinada no projeto. Os três consumers ficaram sem diagnósticos
na execução do Clippy. O gate global continua bloqueado: **693 diagnósticos
distintos**, contra 706 na rodada anterior.

## Proveniência

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. Foram preservadas as alterações anteriores. O diretório
`matematica-ci-2026-10-13/` contém:

- `before.json.gz`: fontes e L0 antes da edição, configuração, hashes, UTC,
  HEAD, status e `git diff HEAD --stat`.
- `after.json.gz`: os mesmos arquivos no estado final e proveniência.
- `evidence.json.gz`: sondas antes/depois, identidade dos binários, logs de
  testes/build/Clippy, inventários, comparação e gates de arquitetura.
- `probe.py`: reprodução das sondas, sem sobrescrever medições anteriores e
  rejeitando um oráculo com SHA-256 diferente do ratificado.

O oráculo é `lab/typst-original/target/release/typst`, pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
As medições não pertencem ao HEAD isolado. Não houve commit, push ou publicação.

## Medição e reparo

`clippy.toml:3-24` proíbe as transcendentais nativas por dependerem da
plataforma. Os owners de `calc/trig`, `calc/hyperbolic` e `calc/logarithms`
ainda usavam esses métodos; alguns tinham exceções locais ao lint. O comentário
P283 registrava expressamente que essa substituição estava adiada.

A leitura de `lab/typst-original/crates/typst-library/src/foundations/calc.rs`
e `layout/angle.rs` confirmou o uso de libm nas famílias correspondentes. O
lockfile upstream e `01_core/Cargo.toml` fixam a mesma versão 0.2.11. Nas sondas
anteriores, tipos e quadrantes coincidiram, mas cosh(1), exp(1) e log(81, base: 3)
tinham diferenças nos últimos dígitos. Os erros de domínio eram rejeitados por
ambos, com mensagens diferentes.

Depois dessa medição, os três L0 foram atualizados. A implementação substituiu
as chamadas por funções puras libm, removeu as exceções locais e o comentário
de adiamento. A whitelist foi estendida apenas aos símbolos usados. Foram
preservados coerções, tipos, ordem pública x,y de atan2, guards, extensões,
diagnósticos, assinaturas, defaults e fase do pipeline.

Os testes adicionais usam valores independentes, tolerâncias numéricas,
quadrantes, tipos de resultado, limites de domínio e o sinal observável do
zero pela divisão. Foram escritos e executados antes da implementação: os
três passaram também no baseline. O RED desta correção é o gate de qualidade
com métodos proibidos; não se afirma que esses testes testemunharam uma falha
funcional anterior. A escolha do backend matemático é mecânica. Coincidência
dos resultados serializados das sondas não constitui prova geral de paridade.

## Validação

| Verificação | Resultado |
| --- | --- |
| Testes adicionais antes da implementação | 3 passaram |
| `cargo test --offline --locked -p typst-core --lib calc -- --test-threads=1` | 143 passaram, zero falhas |
| `cargo build --offline --locked` | Passou |
| Sondas do binário recompilado | 5 expressões com resultados numéricos/ângulos coincidentes com o oráculo; 4 rejeições de domínio preservadas |
| Clippy nos três consumers | Zero diagnósticos |
| Clippy do core, lib e testes | 706 → 693 distintos; ainda falha |
| V1/V5/V15/V26 estritos e gate reverso | Passaram; 965 consumers, zero erros |
| Relatório arquitetural completo | Zero erros, 381 warnings e 1616 notes |
| Formatação e `git diff HEAD --check` | Passaram |

O inventário usa código, mensagem e spans primários completos
`(file, byte_start, byte_end)`, como a rodada anterior. Os 13 diagnósticos
removidos são `disallowed_methods` (43 → 30). Não se somam lib e testes nem se
colapsam ocorrências distintas apenas por linha. Os warnings arquiteturais
continuam V16=328, V17=28, V12=19 e V18=6. Os receipts registram o estado exato
usado nessas contagens.

Os resultados de erro do cristalino permaneceram iguais aos anteriores. As
mensagens ainda diferem do vanilla; este reparo não fecha essa diferença.
A suíte inteira do workspace não foi reexecutada nesta rodada: a evidência
nova cobre as regressões de calc, build, sondas e gates indicados acima.

## Continuidade

Esta rodada continua [o saneamento de qualidade](qualidade-ci-2026-10-13.md).
O objetivo integral continua aberto: restante do Clippy, pendências funcionais
de paginação/tabelas/notas/contexto, warnings arquiteturais e validação remota
da matriz. Os workflows de [linhagem, CI e release](linhagem-ci-release-2026-10-13.md)
mantêm a publicação condicionada aos gates. Não há certificação de produção.

Continuação: [testes discriminantes e saneamento do CI](testes-ci-2026-10-13.md).
