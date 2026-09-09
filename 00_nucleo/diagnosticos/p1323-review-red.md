# P1323 — preparação e RED separados

Sucessor de `p1323-review-l0.md`, mesmo revisor, regime e limites. Não altera
artefatos julgados. Os detalhes privados da assertiva não foram comunicados
ao implementador antes do patch; somente a validade semântica do RED.

O receipt `p1323-unit-red.json`, SHA-256
`34d4b95006dc69216d4e28d1219980440edbc7d4f7a3edbae213c1ece6601fc6`,
registra `cargo test -p typst-wiring --bin typst --release --locked p1323 --
--nocapture` de `2026-09-08T23:23:23.706246+00:00` a
`2026-09-08T23:23:24.582778+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
O diff/stat integral before/after é estável e identifica as alterações
preexistentes call_dispatch/loading e as alterações P1323 wiring L0/Rust.
A compilação terminou; houve um teste executado e uma falha por assertiva de
conteúdo. Portanto o RED é semântico, não falha de compilação ou seleção vazia.

O snippet independente preserva SHA-256
`376719d97b8aa1648dcaf3ad40dfe5bc9c2d1fcda82a05fc9056650cb53934bb`.
Na revisão, os arquivos protegidos pelo freeze A/B permanecem intactos e o
corpo normativo L0 conserva SHA-256
`33ca7a522ae37d8fc9e206c6f5cc592638adbf6d182b5eac189c138e197b4926`.
O SHA integral pode mudar somente pela linha recíproca `Hash do Código`,
conforme a política explícita de `p1323-obligation-freeze.json`.

O diff produtivo de preparação apenas nomeia a headline antiga terminada por
newline e troca sua emissão por `eprint!` no mesmo ponto/gate. O receipt
`p1323-seam-build.json`, SHA-256
`f4294245a82ae9086e76027de8050b985c4dd8edfc9348518f1030d60c34b8ba`,
registra build workspace release com exit zero. A comparação de processo em
`p1323-seam-preservation.json`, SHA-256
`87fb5bb368c1aa4a7501ae43c8420c57987fbbd13e85e5ab77645edd22b5673f`,
medida em `2026-09-08T23:24:25.187045+00:00`, confirma preservação dos quatro
perfis do baseline para exit, stdout, stderr e hash do artefato. O binário
preparatório está identificado por SHA-256
`d97e00a80e3278a18060c6323000b69187329d9c78c872dc97e4171e28103b18`.
Trata-se de preservação do mesmo produto, sem inferir paridade geral de bytes
entre implementações.

A troca logística de target para `/tmp/p1323-target.9NrOxY` não modifica a
obrigação. O sucessor `p1323-obligation-freeze.json` referencia o receipt de
fallback e mantém o manifesto inicial; não há apagamento do estado anterior.

Veredito desta fase: preparação preservada e RED válido. A correção do
literal pode ser julgada contra esta cadeia. GREEN, A/B externo, linhagem,
gates arquiteturais e preservação global preexistente continuam pendentes.
