# P1339 — style harness r3, correção mecânica E0599

Autor: `/root/p1336_tests`, autoridade restrita de adapter/harness.
Regime: executado sem atestação de isolamento; contexto P1336 herdado.
Preparação: 2026-09-10 10:15:00 UTC, após autorização explícita retransmitida
pelo coordenador para corrigir testes preservando expectativas. Esta subtarefa
não aplica a extensão array(bytes), não altera produto e não lê candidato.

Manifesto ancestral: `p1339-authority-manifest-r2.json`, SHA
`842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.

## Causa e delta

O coordenador reportou cinco E0599 no r2, linhas 170, 194, 196, 265 e 267:
`TrackedMut::reborrow_mut` é função associada, não método. A convenção 3 de
`01_core/CLAUDE.md`, lida integralmente, documenta a forma associada.
Seu SHA nesta preparação é
`d358e69f77ceb88bfd7f64bbf64f90b88b6e97c0bac88566f0eb94315b48829f`.

O diff substitui somente as cinco chamadas:
`tracked.reborrow_mut()` por
`comemo::TrackedMut::reborrow_mut(&mut tracked)`.
O caminho qualificado não depende de novo import pelo módulo pai. Há também
uma atualização do comentário de versão; nenhum outro trecho é alterado.
As cinco ocorrências foram exigidas por contagem textual antes de gerar o
sucessor, a partir do r2 integral. Expectativas, DTO, operações, lifetimes
estruturais do r2, canais, perfis e ordens permanecem textualmente iguais.

## Pins

- Original preservado e hash reconferido:
  `p1339-mutant-closed-state-style-harness.rs`, SHA
  `2f01811e5ae26ea7ed5650adcf359c47b00afdbdd22089bd0775cd6d6f7d8f21`.
- Predecessor protegido, preservado e hash reconferido:
  `p1339-mutant-closed-state-style-harness-r2.rs`, SHA
  `6527552f2ef6518e2fbc22f02ac1a73af8bb7c34803d0f572c1320b1e3258231`.
- Sucessor `p1339-mutant-closed-state-style-harness-r3.rs`, SHA
  `9a8d7ed98658d2121dfa8761abbdbd23fc18d88183ff56a3a58b4ec2e43a89dc`.
- Diff `p1339-mutant-closed-state-style-harness-r3.diff`, SHA
  `00ff740853d7fd88079b66f2c02620c5e9d2c5aaaa2688cd11262003156ef359`.

## Estado e custo

`PROPOSED_MECHANICAL_SUCCESSOR_PENDING_INDEPENDENT_ACCEPTANCE`.
Nenhum rustfmt, rustc, Cargo, checker, teste ou runtime foi executado nesta
subtarefa. Leitura, cinco substituições controladas por apply_patch, diff e
hashes somente. Exit 1 de `git diff --no-index` significa diferença esperada,
não falha de compilação. Os E0599 são evidência retransmitida, não um build
local deste papel. Não há crédito de compilação ou veredito runtime.

Skill e ambas referências relidas integralmente. Escritas limitadas ao r3,
diff e este recibo; nenhum predecessor, driver, registry, oráculo ou fonte
produtiva foi modificado. Aceitação mecânica e decisão de gate/resselo ficam
com o verificador; este recibo não concede autorização autônoma de fase F.
