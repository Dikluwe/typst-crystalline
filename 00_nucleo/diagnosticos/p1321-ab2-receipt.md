# P1321 — recibo A/B com contexto novo

Regime: **executado sem atestação de isolamento**. Autor dos oráculos: agente
`/root/p1321_ab_clean`, com contexto novo e allowlist declarada no freeze. Não
foram lidos código/testes cristalinos, diff real nem diagnósticos de outros
passos. A sessão compartilhada não constitui isolamento atestado por capacidades.

## Evidência anterior ao candidato

Estado medido em 2026-09-08T20:17:58.660236Z: working tree não commitado sobre
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`. O baseline JSON conserva
status integral e o seguinte diff/stat do momento:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 171 ++++++++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 247 +++++++++++++++++++++++----
2 files changed, 385 insertions(+), 33 deletions(-)
```

Baseline: `/tmp/p1319-target.VqXtmj/release/typst`, SHA-256
`37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd`.
Vanilla ratificado upstream `a51e02804`: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Corpus próprio: 110 expressões × quatro perfis (`default`, `html`, `a11y`,
`html-a11y`) × dois binários = 880 execuções, sem timeout. Custo acumulado dos
processos: 60,16 s. Todas as expressões chegaram ao observável pretendido;
duplicata sintática direta é controle explícito de erro anterior ao consumer.

As fontes vanilla consultadas foram `loading/csv.rs:27–46`,
`foundations/args.rs:152–183,218–235,259–266,431–455` e
`typst-macros/src/func.rs:375–425`. O L0 foi lido integralmente e a nova seção
P1321 foi lida antes do freeze. ADRs 0107/0108/0127/0129 e skill/referências
também foram lidas. Os hashes dos arquivos permitidos relevantes estão selados.

Resultado contra as expectativas P1321: **260 RED e 180 já preservados**, em
440 células caso/perfil. Nenhum Unknown. As expectativas comparam exit,
stdout e stderr integrais, incluindo mensagens, hints, âncoras e traces.
Não normalizam diagnóstico para simular igualdade.

As políticas são: 85 casos com referência vanilla integral, 23 controles
com referência baseline integral e dois casos Symbol com obrigação normativa
explícita. Nesses dois casos mistos, a rejeição Symbol vigente ganha prioridade
sobre unknown; a apresentação completa foi construída antes do candidato e
conferida contra os dois controles Symbol do baseline. Não é paridade Symbol.

Foi observado um limite relevante de transporte: o sink em
`{ let f(..a) = csv(..a); f(bytes("a,b\\n1,2"), 17, zeta: 1) }`
produz `unexpected argument: zeta` no vanilla. O caso está congelado integralmente
e foi comunicado ao coordenador antes do patch; não se presume que a ordem
lexical externa é a ordem fornecida pelo sink ao consumer.

## Freeze

`p1321-ab2-freeze.json` SHA-256:
`b90c53cc2d982160dce2faca2f4afb6910d07b06e321b8bf8090bc51ead0d6ca`.
Baseline JSON SHA-256:
`10ea61b936ad2c6703aa8984451516d8e5fd808fe4c21ef6f13cf540bac9bc35`.
L0 raw SHA-256:
`03de416f06983fa6a5a29253e672a795f357fda18e9322e79b4f66b925940f96`.
L0 normativo, excluindo somente a única linha canônica `Hash do Código`, SHA-256:
`2f6dcab26aefce4a7db207bd347b46998dd845e7205954846be4512b5c2f2cc3`.

O runner valida hashes antes do candidato e contém rotas normal/repeat/reverse.
Comando de reprodução do candidato, após liberação do coordenador:

```text
python3 00_nucleo/diagnosticos/p1321-ab2-runner.py candidate --binary CAMINHO_LIBERADO
```

## Limites

Esta evidência A/B mede a superfície pública via `eval`, não a API Rust de
Args sintético, contadores instrumentados de I/O nem identidade interna. A
origem detached pública é atacada por `arguments.map`; os campos preservados
do argumento remanescente são comparados integralmente. Os controles de caminho
e parsing não fecham as dívidas de origem externa/resolução. Não há alegação
de equivalência geral CSV nem de isolamento atestado.

## Verificação do candidato liberado

**FAIL delimitado ao contrato congelado**, por âncora incompleta de missing
source. Não foi alterada expectativa para converter esse resultado em PASS.

Candidato `/tmp/p1321-target.669PuL/release/typst`, SHA-256
`deb6aed85d133665b00d416fabee92aeda04196ff76b6a734ef0451bbdeaea3c`, liberado
pelo coordenador antes da execução. Medição iniciada em
2026-09-08T20:32:49.259204Z, mesmo HEAD, working tree não commitado:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 171 +++++++++-
01_core/src/compiler/stdlib/loading.rs       | 466 ++++++++++++++++++++++++---
2 files changed, 590 insertions(+), 47 deletions(-)
```

Foram executadas 1320 comparações, 440 em cada ordem normal/repeat/reverse:
**1224 PASS, 96 FAIL, zero Unknown**. As 440 saídas são determinísticas entre
as três ordens. Custo acumulado dos processos: 143,43 s. Todos os hashes
protegidos e o L0 normativo passaram pela verificação anterior à execução.

Falhas: `missing_none`, `missing_delimiter`, `missing_unknown`, `alias_missing`,
`args_missing`, `sink_missing`, `with_missing`, `map_missing`, em todos os
perfis/ordens. Exemplo mínimo:

```text
csv()
atual:    coluna 3, range dos parênteses, ^^
esperado: coluna 0, range da chamada toda, ^^^^^
```

Mensagem `missing argument: source` correta; a diferença é exclusivamente
a âncora primária. Nos wrappers, a extensão incorreta mantém o padrão de
argumentos sem o nome do callee. O trace preservado não substitui a âncora.
Os demais 102 casos passaram integralmente, incluindo source-named com hint,
cast/opções antes de unknown, remanescente antes de parser/I/O, causalidade do
sink e Symbol misto normativo. Esses resultados não eliminam a falha obrigatória.

Saídas e expectativas integrais em `p1321-ab2-candidate.json`, SHA-256
`d1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25`.
O achado foi comunicado imediatamente ao coordenador. Nenhuma implementação
ou entrada candidata foi lida. Nova execução dependerá de candidato liberado
e orçamento focal explícito; o corpus verde não foi repetido após o gate.
