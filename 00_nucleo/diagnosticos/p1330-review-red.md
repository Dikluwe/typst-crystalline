# P1330 — gate RED antes de C

**PASS_RED**: RED compilado e semanticamente pertinente. C pode seguir
no recorte L0 R2 aprovado, preservando os inputs congelados. Revisor
`/root/p1330_review`; sem atestação de isolamento e sem selo de refinamento.

Recibo `p1330-unit-red.json`, SHA-256
`faab6eada818a22c59f0679a0073eb5a24444b869168d157521d21d3e280ac7c`,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
de `2026-09-09T13:16:49.575816+00:00` a
`2026-09-09T13:18:36.686130+00:00`, target `/tmp/p1330-target.f0lmDu`:

```text
cargo test --release --locked -p typst-core compiler::stdlib::calc::p13 -- --nocapture
exit 101; running 23 tests; 17 passed; 6 failed
```

Todas as falhas são rejeições esperadas do overflow que recebem sucesso
do baseline: as duas expectativas históricas sucedidas, o novo teste
nativo de limites, o de origens, o de rotas públicas e o de warning com
overflow. Não houve erro de compilação. Os demais testes, incluindo
P1329 inteiro, guards, tipos não inteiros e math Content, passaram.
Os testes densos interrompem na primeira falha; este RED não prova que
todas as rotas/perfis posteriores já foram exercitadas. O GREEN dos
mesmos bytes é que deverá percorrê-los integralmente.

Integração `p1330-test-integration.json`, SHA-256
`9dd3daab105e0bd492299fe384de60dcae2773d69a00560cd23502e3fb934d9d`,
referencia incidente SHA-256
`2a98779c7d50d46b6a5d4c925fbfa160b14f53dfce302a53f5693a3748603899`.
Às `2026-09-09T13:17:20.752Z`, reconstrução em memória demonstrou
igualdade exata de owner = baseline + único sucessor + snippet em EOF,
excluída somente metadata @prompt-hash. Runtime e P1329 intactos.
Isso resolve o gate da posição canônica pendente em review-pre-c.

Owner do RED SHA-256
`8db4307a343ea5ebfcc0f06fe45d60e08bf4a517d7c46818f81468cd77be7c09`.
Inventário produtivo before/after idêntico; owner atual confere com o
recibo, manifesto R2 confere e todos os seis artefatos congelados da
integração conservam seus hashes. O recibo contém snapshot/diff/stat
exatos: mesmas entradas listadas na revisão preliminar, calc.md com 249
linhas no stat e calc.rs com 1253, total
`14 files changed, 3642 insertions(+), 107 deletions(-)`.

O revisor auditou o recibo real e as entradas, não reexecutou o build.
Nenhum teste/oráculo/output foi corrigido pelo revisor. Este gate não
aprova candidato, GREEN, CLI final ou paridade geral.
