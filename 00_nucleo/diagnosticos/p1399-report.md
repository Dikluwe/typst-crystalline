# P1399 — relatório de verificação final

## Veredito

**PASS_FRAGMENT.** O candidato preserva o fragmento congelado: todo caso
observado de `Content::ContextBlock` produz exatamente `context()`, enquanto
os controles de função e conteúdo comum permanecem `(..) => ..` e `[x]`.
Este veredito não declara paridade funcional geral de `repr`, `context` ou
Typst.

O protocolo completo da skill `tekt-materializacao-segregada` foi aplicado
com papéis e artefatos distintos, porém no mesmo workspace: **executado sem
atestação de isolamento**.

## Estado e identidades

A verificação ocorreu sobre o candidato não commitado cujo predecessor é
`bdec21bf935cbf7f61070756cd0f218e441b7e07`. A fotografia final, tomada em
`2026-09-14T18:24:43-03:00`, fez `git status --short` listar somente os dois
arquivos produtivo/normativo alterados e os nove artefatos novos de P1399.
`git diff HEAD --stat` registrou dois arquivos rastreados, 117 inserções e três
remoções; os nove caminhos não rastreados estão enumerados no recibo JSON.

As identidades conferem: preseal R1
`209a13193f9db4ff315d717db2c8a7ec39414a8d96bfa67015bbe7cf47d28b30`,
L0 `0262bcc4e267da9f3444b0cc881ba04a6194f58a426104aa87c270ba7e5d332d`,
consumer
`345309fc499cf545d89382939ee81e2a3ad2754f263e950bc04f98804e6c380b`
e binário candidato
`6377949e4ebb62ddc759214346cc11c0548df42f6c28f701f8fdef41047ba6f8`.
O L0 registra `Hash do Código: ba7715e7`; o consumer aponta para o mesmo L0
com `@prompt-hash 615b778a`. O linter validou a linhagem.

O delta produtivo limita-se ao braço `Content::ContextBlock`, que muda
`context(...)` para `context()`, ao teste local correspondente e à metadata
de linhagem. Não houve mudança em helper geral, entidade, parser, eval,
layout ou export, e os hashes protegidos do passo, manifesto, contrato,
oráculo e mutantes permanecem os do preseal.

## Oráculo independente

Um harness efêmero próprio executou os seis casos em ordem direta, repetida e
inversa. Cada processo foi serial, com `RLIMIT_AS` de 512 MiB, timeout de 10 s
e teto de 64 KiB por stream; o lote ficou sob 60 s. A comparação foi por
bytes UTF-8 e codepoints, sem normalização.

- Ordem direta: **6 Preserved, 0 Difference, 0 Unknown**.
- Três ordens: **18 Preserved, 0 Difference, 0 Unknown**.
- Duração: 1,793538 s; RSS máximo: 33.332 KiB; swap antes/depois: zero.
- `context()` foi exatamente `636f6e746578742829`;
  `(..) => ..`, `282e2e29203d3e202e2e`; `[x]`, `5b785d`.

## Gates finais e memória

- `crystalline-lint .`, com `RAYON_NUM_THREADS=1` e `RLIMIT_DATA` de 2 GiB:
  exit 0, zero violações, RSS máximo 70.640 KiB e swap zero. Os quatro
  registros existentes em `repr.rs` são informativos V19 nas linhas 487,
  566, 800 e 1086; nenhum incide nas linhas alteradas e não surgiu warning
  novo no delta.
- `cargo fmt --check`, `git diff --check` e validação dos cinco JSONs
  protegidos: exit 0.
- `cargo test` e lib-test monolítico não foram executados, conforme a trava de
  memória do passo.

O verificador não repetiu o build. O recibo fornecido pelo implementador
registra build serial com uma job, `RLIMIT_DATA` de 2 GiB, 158,38 s, RSS
máximo de 1.140.048 KiB e swap zero, abaixo do limite de aborto de
1.782.579 KiB. A identidade do binário produzido foi conferida antes do
oráculo.

Os detalhes reproduzíveis, codepoints e classificação fail-closed estão em
`00_nucleo/diagnosticos/p1399-verification.json`.
