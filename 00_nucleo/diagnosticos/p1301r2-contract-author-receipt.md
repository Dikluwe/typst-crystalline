# P1301 revisão 2 — recibo do autor do contrato

## Resultado

Contrato candidato binding-free materializado em
`00_nucleo/diagnosticos/p1301r2-contract.json`, SHA-256
`69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad`.

Estado: `CONTRACT_CANDIDATE_UNSEALED`. Este recibo não é selo, execução do gate,
implementação, ataque nem veredito de equivalência.

Linguagem de atestação: **executado sem atestação de isolamento técnico**.

## Papel e capacidades

- Papel: `P1-r2`, autor segregado do contrato.
- Ambiente: `/repos/Antigravity/typst-crystalline`, working tree não commitida.
- Contexto herdado: tarefa restrita P1301 revisão 2 e `AGENTS.md`; não houve
  contexto de implementação candidata.
- Escritas autorizadas e realizadas: somente
  `p1301r2-contract.json` e este recibo, ambas via `apply_patch`.
- Não foram lidos código candidato, testes candidatos, patch transitório,
  `p1301-candidate-runner.py` nem conteúdo dos artefactos v1. A enumeração de
  estado Git expôs apenas nomes de caminhos, não conteúdo.
- Não foi implementada solução e não foi emitido veredito.

Regime aplicado: protocolo completo da skill `tekt-materializacao-segregada`;
este papel encerra-se na autoria do contrato. As referências operacionais da
skill foram lidas fora do repositório. A allowlist fechada da tarefa impediu
abrir ADRs ou outros ficheiros do repositório.

## Entradas congeladas e re-hash anterior à leitura

| Entrada | SHA-256 observado | Resultado |
|---|---|---|
| `00_nucleo/diagnosticos/p1301r2-manifest.json` | `1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712` | confere |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` | confere |
| `00_nucleo/prompts/compiler/eval/tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` | confere |
| `00_nucleo/diagnosticos/p1301-pre-gate-measurement.json` | `1202c06723e948f5586e10345c5f653f57b075e3ea41297f996fd5a27abb77cd` | confere |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` | confere |
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` | registado |

O manifesto congelado tem SHA-256
`1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712`.

## Superfície contratada

O contrato usa exclusivamente a forma CLI comum
`eval <expression> --format json`, opcionalmente acrescida de `--features`
pelos quatro perfis congelados. `--diagnostic-format` e `--target` são
explicitamente proibidos.

Há adaptadores separados por papel de binário. Ambos apenas extraem o envelope
real ordenado:

`exit_code`, `stdout`, `diagnostic_class`, `message`, `ordered_hints`,
`span_bytes`.

Nenhum adaptador traduz aspas, nomes, mensagens, hints ou spans. `stderr` bruto
e seu SHA-256 são obrigatoriamente registados para reprodução, mas o texto
renderizado bruto não é campo de igualdade entre renderers.

O corpus contratual ficou deliberadamente fechado em:

- 23 casos lógicos: 22 de origem vanilla e uma sentinela cristalina;
- 2 casos opacos previamente declarados, cujo único resultado aceitável é
  `Unknown/UNSUPPORTED_DIAGNOSTIC_CARDINALITY`;
- 12 coordenadas `std.{hsl,hsv,linear_rgb}` nos quatro perfis;
- 3 módulos independentes ausentes e 4 sucessos de controle;
- referência vanilla do dicionário em bytes `14..21` e sentinela cristalina
  obrigatória em bytes `10..21`;
- ordem normal e ordem integralmente invertida;
- exatamente 12 mutantes válidos, score exigido `1.0`, zero sobreviventes.

`repr(std)` permanece explicitamente fora do escopo.

## Política de resselo de linhagem

A identidade protegida de cada L0 é o SHA-256 semântico congelado. Depois do
selo, o SHA-256 integral de um prompt só pode mudar se a única diferença de
bytes for exatamente a linha única que satisfaz
`^Hash do Código: [0-9a-f]{8}$`, ressellada pelo linter. O contrato exige hash
integral antes/depois, SHA semântico idêntico, diff restrito à linha, dry-run
antes da escrita, aplicação única, segundo dry-run com `Nothing to fix` e
concordância com o `@prompt-hash` do consumer. Qualquer outro byte alterado ou
prova ausente invalida o selo.

## Medição e proveniência

Sondas usadas para congelar envelopes foram executadas somente contra
`/usr/local/bin/typst` de SHA-256 `7b4f40c…`, com argv na superfície comum.
Confirmaram mensagens, stdout e ranges dos casos obrigatórios, inclusive
`std` nos quatro perfis, módulos independentes, sucessos e dicionário. Sondas
exploratórias de forma do renderer não foram adicionadas ao corpus fechado.

- Fim da medição/autoria: `2026-09-03T21:50:04.267953439-03:00`.
- HEAD: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Estado: working tree não commitida.
- `git status --short` antes da escrita do contrato: SHA-256
  `255ce0d05a93ae3314553f6fce746fb0ecbb7077650a315c7aab637383ba08f1`.
- `git diff HEAD --stat`: SHA-256
  `df861a696777f2714969a76128a6c5deafd7383537223c9dbb1efc2f1f1e12b6`.

Ficheiros tracked alterados nesse `git diff HEAD --stat`:

```text
00_nucleo/prompts/compiler/eval.md                 |  39 +++-
00_nucleo/prompts/compiler/eval/bindings/field_access.md |  72 ++++++++
00_nucleo/prompts/compiler/eval/tests.md           |  90 ++++++++-
00_nucleo/prompts/compiler/stdlib/color.md         |  46 ++++-
01_core/src/compiler/eval/mod.rs                   |  14 +-
01_core/src/compiler/eval/tests.rs                 | 205 ++++++++++++++++++++-
01_core/src/compiler/stdlib/color.rs               |   6 +-
7 files changed, 452 insertions(+), 20 deletions(-)
```

## Validações executadas

- `python3 -m json.tool p1301r2-contract.json`: exit `0`.
- Validação estrutural independente: 23 casos, 22 vanilla, 2 opacos, 1
  sentinela cristalina, 12 mutantes e duas ordens que são permutações do mesmo
  conjunto: exit `0`.
- `sha256sum p1301r2-contract.json`: hash registado no início deste recibo.
- `jq` não estava instalado (`exit 127`); não foi necessário instalar nada e a
  validação foi repetida com o parser JSON padrão disponível.

## Budget e handoff

Revisão do contrato: `2/2`. Budget restante para redesign do harness: `0`
(`EXHAUSTED`). Outra falha de desenho do harness termina a cadeia como
`blocked`; isso não permite afrouxar o contrato nem promover `Unknown`.
Somente uma falha semântica do candidato pode ser reparada dentro da obrigação
r2 já congelada.

Próximo papel autorizado: autor de oráculos/adversário e, depois, verificador
do poder discriminatório. Este autor não sela o próprio contrato.
