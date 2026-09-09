# P1333 — parecer prévio independente de escopo

Revisor `/root/p1333_review`; regime A/B, executado sem atestação de isolamento.
Leitura de baseline, sem candidato ou L0 P1333 recebidos; nenhuma execução
de produto. Escrita restrita a diagnósticos próprios `p1333-review-*`.
Skill tekt-materializacao-segregada e ambas referências lidas; ADRs
0107/0108/0127/0129/0130 e instruções L1 consultadas. Não foi encontrada ADR
local específica de segregação na busca dirigida em `00_nucleo/adr/`.

Proveniência da inspeção: UTC 2026-09-09T15:00:41Z, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
SHA-256 do L0 `compiler/stdlib/calc.md`:
`2bc4ae33aa2eb9a8a426bc5b7cd44d7397ea3727a99b1992949409d12c0203df`;
`entities/args.md`:
`1a78d43dcc7c1a72c45848494b86281aa9b8a13d03abfaa8fb6baf141a5511aa`.
Ambos foram lidos integralmente. SHA-256 do consumer calc:
`2ca44000def7b055ca9ae4f39fd0041e92373a1ea648d6c5fef20c7e436f71c3`;
Args: `f3e77a9bf2d038cbede40c8b5304dd00cb18f7c2f5b5cd9d53822a44ba7e0c4b`.
O diff dirigido aos arquivos inspecionados identifica alterações em calc L0,
calc.rs e call_dispatch.rs; não representa inventário global nem medição funcional.

A fonte ratificada fornece uma ordem inequívoca: macro
`lab/typst-original/crates/typst-macros/src/func.rs:417-419,473` faz
`expect("value")`, depois `finish`, depois corpo. Em
`foundations/args.rs:112-120,150-173` (sob
`lab/typst-original/crates/typst-library/src/`), expect procura o primeiro
posicional ignorando named, faz cast no value-span e, se ausente, procura
o primeiro named `value`. Este produz `the argument `value` is positional`,
hint `try removing `value:``, no span completo dessa ocorrência. Sem esse
named, produz `missing argument: value` no agregado. Named diferente que
precede `value` não vence o diagnóstico específico.

`foundations/calc.rs:84-94` calcula o módulo dentro de ToAbs: overflow e
Length misto são falhas do cast e vencem sobras, tal como conteúdo/fallback.
Após cast bem-sucedido, `foundations/args.rs:259-263` rejeita a primeira
ocorrência restante na ordem conjunta: `unexpected argument` ou
`unexpected argument: NAME`, no arg-span. Isto é precedência observável da
linguagem, sem obrigar cópia da mecânica Rust nem inferir intenção geral.

O carrier atual basta para esses valores e ordens: `01_core/src/entities/args.rs:53-80`
preserva sequência; síntese ordena posicionais antes dos named e deixa
ambas origens individuais detached. Seu L0, `entities/args.md:140-143`,
autoriza cursor local da nativa e proíbe duplicar validação no dispatcher.
`01_core/src/compiler/eval/call_dispatch.rs:402-439,1058-1064` conserva
arg/value-spans em chamadas, spread de arguments e With; arrays/dicts usam
span do spread. Não há necessidade demonstrada de API nova.

Há, porém, impedimento para alegar suficiência integral do owner calc:
`call_dispatch.rs:398,451` e `eval/math.rs:348` passam agregado da lista.
Vanilla `typst-eval/src/call.rs:458-460,78,150` deliberadamente usa chamada
inteira. O transporte seletivo cristalino em `call_dispatch.rs:454-482`
não inclui abs. Assim, ausência sem named `value` exige medir sua âncora;
paridade integral provavelmente requer revisão do escopo para esse owner.
Não fabricar span em calc por pesquisa textual ou acesso à AST/World.

Antes de freeze: medir ausência/alias/With/math, named value após outro
nome, ordens mistas e casts falhando com sobras; registrar diagnóstico
integral. Rever explicitamente preservações de guards P1328–P1332 e migrar
só expectativas afetadas. O rótulo histórico DRAFT em `entities/args.md:7`
não exige nova API nesta tarefa, mas a revisão não atesta sua aprovação
histórica. Conclusão condicionada aos outputs públicos: valores e ordem
cabem em calc; âncora agregada ainda refuta o recorte integral presumido.
