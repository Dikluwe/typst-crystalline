# P1334 — testes congelados; primeiro RED não compilou

Sem objeção semântica ao ledger ou à cobertura nova inspecionada. Porém o
primeiro recibo `p1334-unit-red.json` **não é RED semântico**: cargo falhou
na compilação do teste de dispatcher com E0599. `make_calc_module()` retorna
Value; `p1334-ab-dispatch-tests.rs:24` chama `.scope()` diretamente nesse
Value, sem extrair `Value::Module`. Autor B deve corrigir somente essa
adaptação, preservar freeze/recibo antigos, congelar o sucessor e repetir
RED compilado antes de C. O revisor não corrigiu o teste.

Freeze nativo conferido
`8130650591a17ecdfebd1969707967964924714168c6ab428887d0ad1b8a2368`;
integração conferida
`b2c3a51b136ca7eb47fad1dce99399ca80476defb0bbb2f9ebd5d642e06be9f2`.
Todos os hashes de inputs/artifacts registrados coincidem. Os oito módulos
estão presentes integralmente nos respectivos owners. Uma reconstrução
somente leitura, substituindo cada sucessor pelo predecessor e removendo
os dois módulos novos, restituiu ambos os consumers do baseline depois de
excluir headers e separadores de linhas vazias. Não há candidato runtime
misturado à integração auditada.

Li as diferenças completas em `p1334-ab-migration-ledger.json` e
`p1334-ab-migration-formatted.json`: somente mensagens de guards,
âncoras/traces correspondentes e fixture missing stale convertida a None.
As expectativas de primeiro inválido ficam intactas. A retirada do trace
próprio de missing é acompanhada de asserção explícita de trace vazio;
as demais verificações integrais continuam. Nenhum histórico foi apagado.

Os testes novos cobrem Some/None, missing agregado conhecido/detached,
primeiro named value e duplicatas com origens distintas, arg-span diferente
de value-span, sobra em ambas ordens, named value sem hint após valor
válido, primeiro inválido antes de sobras e espécies válidas sem cast da
sobra. Chamadas públicas incluem alias/import/With encadeado, spread
Array/Dict/None/Args e math qualificada. O módulo de dispatcher verifica
identidade real, falsos homônimos, preargs e preservação das identidades
anteriores. O erro de compilação é de construção da fixture, não de
insuficiência da intenção ou sinal para ampliar API do produto.

Proveniência: manifesto
`2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da`;
baseline/HEAD/diff stat conforme `p1334-review-l0.md`. O recibo RED inválido
tem SHA-256
`ff7fe2a4c528aab97fb94ccd5fed7b4bc28e38184bbb27300598e6dcb8876e6a`,
argv `cargo test --release --locked -p typst-core p13 -- --nocapture`,
exit 101, stdout vazio, UTC `2026-09-09T16:02:37.156189+00:00` a
`2026-09-09T16:02:55.346246+00:00`, target `/tmp/p1334-target.Ujq0xU`.
Seus estados antes/depois identificam os consumers testados. Não inventar
contagem de testes executados a partir deste compile failure.

A/B sem atestação de isolamento; CLI freeze ainda pendente nesta revisão.
Incidente anterior de listagem incidental de nomes restritos segue
registrado em `p1334-review-scope.md`. Nenhum passo histórico foi lido.
