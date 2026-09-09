# P1323 — parecer separado do L0

Revisor: `/root/p1323_review`, ambiente compartilhado, contexto inicial restrito
à incumbência de revisão. Regime A/B executado sem atestação de isolamento e
sem selo de refinamento. Autor de intenção/implementação: `/root`; autor dos
testes: `/root/p1323_tests`. O revisor não altera os artefatos julgados; suas
escritas estão limitadas a `00_nucleo/diagnosticos/p1323-review-*`.

Foram lidos integralmente a skill `tekt-materializacao-segregada`, suas duas
referências, `CLAUDE.md`, ADR-0127, `prompts/wiring.md` e os três Núcleos nele
pinados. A pesquisa textual na pasta ADR não encontrou ADR de segregação.
Não houve acesso a materialization/context. A capacidade técnica de leitura
ou escrita compartilhada impede apresentar esta separação operacional como
isolamento atestado.

## Entradas e proveniência

- Baseline: `p1323-baseline.json`, SHA-256
  `8abba2b5f0e3280b4632e4d3bec1db6c2c7a8ee55ab3e9bc9d0d44f197eaa897`,
  medido em `2026-09-08T23:15:55.793062+00:00`, HEAD
  `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
  O `state.diff_stat` e diff completo preservam as alterações preexistentes em
  call_dispatch/loading L0 e Rust; o inventário produtivo permite conferir sua
  preservação final.
- L0 julgado antes do resselo: SHA-256 integral
  `25d3f5df2c9f98747afa8497a78c913f7a0439ae008106daf189ffcc2126edd0`.
  Seu predecessor integral no baseline tem SHA-256
  `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d`.
- Na inspeção, `04_wiring/src/main.rs` ainda coincide byte a byte com
  `original_owner` do baseline: nenhum candidato produtivo foi julgado aqui.
- O alvo vanilla é o ratificado `a51e02804`; o binário registrado tem SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Parecer

Aprovo o adendo P1323 para a fase seguinte, condicionado aos gates descritos.
`main.rs:388–392` confirma emissão fixa restrita ao braço HTML e ao teste de
Feature::Html; vanilla `crates/typst/src/lib.rs:249–256` confirma os três hints.
As linhas do baseline para `html` e `html+a11y` mostram a headline cristalina
isolada e o envelope vanilla completo. Nos perfis `default` e `a11y`, ambos
falham pelo gate existente, sem warning: isso só sustenta preservação do gate,
não paridade HTML exercitada nesses perfis. O baseline identifica comandos,
fonte, horários, binários, stderr, stdout, exit e artefatos.

O L0 põe a medição antes da obrigação, distingue língua diagnóstica da forma
Rust do literal, explicita intenção e refutadores, e limita aceitação ao
envelope textual sem cores. O ownership permanece no único consumer L4; uma
constante privada não cria formatter geral, tipo ou assinatura pública. O
adendo preserva canal, posição, gate, serialização, artefatos e infraestrutura
de cores. Não reivindica paridade ANSI ou HTML geral.

ADR-0127 permite fluxo contínuo para esta correção específica de paridade:
não há novo default, flag, contrato público ou mudança de fase. A simples
alteração de texto visível para convergir à referência não exige novo gate
humano. Se a implementação exigir outra API, owner, formatter ou fase,
o parecer deixa de valer e o escopo deve ser reaberto antes da mudança.

## Gates pendentes e limites

Antes do candidato, resselo consistente do L0 e introdução mecânica da seam
com a headline antiga, preservando exatamente a saída anterior. Testes
independentes congelados devem falhar semanticamente nessa preparação e
passar após completar o literal. Testes de processo precisam conferir a
ligação real a stderr e ao gate, além do conteúdo da constante.

Para o veredito final ainda faltam RED/GREEN, comparação da preparação com
o baseline, execuções A/B normal/repetida/invertida, preservação de artefatos
e controles, build, testes apropriados, lint sem violações, linhagem e
inventário das alterações preexistentes. Inputs protegidos não podem ser
reescritos silenciosamente: inconsistências requerem retificação sucessora.
Este parecer aprova somente a obrigação e sua classificação de gate; não
fecha a implementação nem substitui os gates ainda não executados.
