# P1288 — receipt do autor/testador da Fase A (harness)

## Autoridade e limites

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, executado
  com segregação por papel/capacidade/ordem, sem atestação de isolamento forte do
  checkout compartilhado.
- Papel: autor/testador A do laboratório da Fase A.
- Leitura permitida usada: Passo 1288, manifesto/runner/testes da matriz e os
  artefatos técnicos P1287 estritamente necessários (`p1287-manifest.json`,
  `p1287_global.py`, `test_p1287_global.py`).
- Escrita: `lab/parity/matrix/{manifest.yaml,runner.py,test_runner.py}` e este
  receipt. Nenhum L0, L1–L4, baseline vanilla, contrato global, implementação
  a11y, artefato de outro papel ou relatório/veredito final foi alterado.

## Entradas congeladas

Instante inicial: 2026-08-31, HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree compartilhada e não
commitada. Hashes recebidos antes das alterações deste papel:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1288.md` | `3858860e2d3e9916b051dfbbea0ff66009382bb5daa3fcec8025e9a2492a3e28` |
| `lab/parity/matrix/manifest.yaml` | `9cdcb75527d5c0f2ae5c69cb2d7238c0c558c5fc27f65ef149b8e98c3fee241e` |
| `lab/parity/matrix/runner.py` | `25e025b80d9c19fb1bce2e3d3c6f7caa9d0a680c63a17e9d6e5c6e27e0da8d5b` |
| `lab/parity/matrix/test_runner.py` | `b712d52bcbb3beb56f7767f937642018b8fd9bf2d983907c175390bc6c2fbf9b` |
| `00_nucleo/diagnosticos/p1287-manifest.json` | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| `lab/parity/matrix/p1287_global.py` | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| `lab/parity/matrix/test_p1287_global.py` | `3ac4daa6ff8cb2c9d8396ec4288b737c8496d0473c5d5d901165574af241c535` |
| skill / papéis / gates | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` / `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` / `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |

## RED observado antes da implementação

Comando:

```text
python3 -m unittest lab/parity/matrix/test_runner.py
```

Resultado: `Ran 36 tests`; `FAILED (failures=1, errors=9)`. Os dez REDs
independentes demonstraram ausência de lattice/perfis, falha bilateral desligada
sem classificação própria, HTML desligado sem proteção contra crédito, ausência
do peer ativo de `pdf.data-cell`, flag desconhecida ativa sem política, totais
abertos e ausência de payload determinístico forward/reverse.

## Materialização da Fase A

- Lattice fechado: `MATCH`, `DIFFERENCE`, `DISABLED_BY_PROFILE`, `UNKNOWN`,
  `BASELINE_ONLY`, `EXTRA_BINDING`.
- Manifesto congela `default=[]`, `html=[html]` e
  `a11y-extras=[a11y-extras]`; `bundle` é scope-out explícito.
- Casos gated só viram `DISABLED_BY_PROFILE` quando existe outro perfil ativo
  que cobre todas as features requeridas. Falta de peer é `UNKNOWN`.
- Perfil ativo sempre executa; rejeição da flag pelo candidato é
  `DIFFERENCE` ou `UNKNOWN`, nunca disabled.
- Flags combinadas/repetidas são canonicalizadas simetricamente e em ordem
  determinística.
- Contadores incluem os seis estados, fecham exatamente no total e disabled/as-
  simetrias não recebem crédito de `MATCH`.
- Cada execução faz forward e reverse, normaliza somente raízes temporárias,
  compara digests e degrada divergência de ordem para `UNKNOWN`.
- JSON é canônico (`sort_keys`, separadores fixos), sem relógio, com resultados
  ordenados por id; duas execuções equivalentes produzem bytes iguais.

## GREEN e medições reproduzíveis

Estado medido em `2026-08-31T10:27:58-03:00`, mesmo HEAD e working tree não
commitada. O diff dos caminhos sob autoridade era:

```text
lab/parity/matrix/manifest.yaml  |  15 +-
lab/parity/matrix/runner.py      | 403 +++++++++++++++++++++++++++++++++++----
lab/parity/matrix/test_runner.py | 277 +++++++++++++++++++++++++++
3 files changed, 656 insertions(+), 39 deletions(-)
```

Os números incluem alterações já presentes nos mesmos ficheiros quando o papel
começou; os hashes iniciais acima separam causalmente a contribuição desta fase.

Comandos e resultados:

```text
python3 -m unittest lab/parity/matrix/test_runner.py
Ran 37 tests — OK

python3 -m unittest discover -s lab/parity/matrix -p 'test_*.py'
Ran 79 tests — OK

python3 lab/parity/matrix/runner.py --validate-only
valid manifest: 20 cases

python3 -m py_compile lab/parity/matrix/runner.py lab/parity/matrix/test_runner.py
git diff --check -- lab/parity/matrix/manifest.yaml lab/parity/matrix/runner.py lab/parity/matrix/test_runner.py
ambos sem output, exit 0
```

Probes reais e determinismo:

- default + `P1137-X-002`, mesmo com ambos os binários inexistentes:
  `DISABLED_BY_PROFILE=1`, `MATCH=0`; duas saídas byte-idênticas, SHA-256
  `79b2abafe57ea6dee44f605da1dee4cbfab7bee73daa7b6dee9d3fbcbf497015`;
- html + `P1137-X-002`: `MATCH=1`, forward/reverse idênticos;
- a11y-extras + `P1288-B-001`: vanilla devolveu o trio de funções, candidato
  rejeitou a feature; `DIFFERENCE=1`, forward/reverse idênticos; duas saídas
  byte-idênticas, SHA-256
  `fae2ca1f783edef85aa6a74be6defc755832da75f742d29a8d5d04359ab5756f`.

Binários medidos:

- `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- `target/release/typst`:
  `acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a`.

## Saídas e limitações

Hashes antes deste receipt:

| Saída | SHA-256 |
|---|---|
| `lab/parity/matrix/manifest.yaml` | `9ffc5535828f4f0af655356c58ddd050bbeeb7137ffbb4c9cd4c55c4b58441cc` |
| `lab/parity/matrix/runner.py` | `9cd9a1ade2a3ca512e32de4a76686ab13ef1160d6994f5b4a12f5ce1922092bf` |
| `lab/parity/matrix/test_runner.py` | `92511e5d195227f83796ed0f42796ba7a764ea035bc6f31c51d4f802d9576e5d` |

Este receipt atesta somente a Fase A do harness. Não mede/congela o baseline da
Fase B, não legitima L0/código produtivo, não implementa acessibilidade PDF e não
emite veredito de paridade ou encerramento do Passo 1288. A execução foi segregada
por capacidades e ordem, mas não possui isolamento ambiental forte.

## Adendo pós-gate — composição restrita de features

Retomada em `2026-08-31T13:24:18-03:00`, no mesmo HEAD e working tree
compartilhada. O gate final identificou dois defeitos exclusivos do harness:

1. `with_profile_features` aplicava a feature do perfil até a comandos sem
   declaração de `--features` ou `required_features`, fazendo a query
   `P1137-I-001` receber uma flag ilegítima;
2. `P1288-B-001` ainda conservava a expectativa pré-implementação
   `DIFFERENCE`, embora o perfil ativo agora observe `MATCH`.

Testes foram acrescentados antes da correção. RED observado:

```text
python3 -m unittest lab/parity/matrix/test_runner.py
Ran 41 tests
FAILED (failures=2, errors=2)
```

As falhas provaram a injeção de `--features html` na query e a expectativa
desatualizada. Os erros provaram que a API do helper ainda não distinguia uma
declaração de caso. A correção preserva chamadas antigas de dois argumentos,
canonicaliza features somente quando o próprio comando contém `--features` ou o
caso declara `required_features`, e passa essa declaração explicitamente a
partir de `run_case`. O manifesto final fixa `P1288-B-001.expected_state=MATCH`;
o gate antecipado continua retornando `DISABLED_BY_PROFILE` fora do perfil, sem
crédito de match, e rejeições no perfil ativo continuam visíveis.

GREEN unitário:

```text
python3 -m unittest lab/parity/matrix/test_runner.py
Ran 41 tests — OK
```

Gates reais, com outputs e artefatos exclusivamente em `/tmp`:

```text
python3 lab/parity/matrix/runner.py --profile default --vanilla /usr/local/bin/typst --crystalline target/release/typst --output /tmp/p1288-runner-default.json
python3 lab/parity/matrix/runner.py --profile html --vanilla /usr/local/bin/typst --crystalline target/release/typst --output /tmp/p1288-runner-html.json
python3 lab/parity/matrix/runner.py --profile a11y-extras --vanilla /usr/local/bin/typst --crystalline target/release/typst --output /tmp/p1288-runner-a11y-extras.json
```

Os três comandos terminaram com exit `0`, sem expectativas falhadas:

| Perfil | Totais fechados | Forward/reverse | SHA-256 do output `/tmp` |
|---|---|---|---|
| default | `MATCH=15`, `DIFFERENCE=2`, `DISABLED_BY_PROFILE=3`, demais `0`; total `20` | idêntico, digest `33ca7cc4e253d4f2859e0efac4aa41fb879010464ba8ebfa0d560861b96be983` | `b7fc4fa6bc47b07781f0192260cd0a36ce4f63c8c44432712a3959cfe95e0221` |
| html | `MATCH=17`, `DIFFERENCE=2`, `DISABLED_BY_PROFILE=1`, demais `0`; total `20` | idêntico, digest `d89921ad60259f89a47e8a394b1e17d43412a7ee15fbdc06b690153039a6efa0` | `1db96db4077171caf46a5b63acb8de09363647880db7a885076eddd8228250d8` |
| a11y-extras | `MATCH=16`, `DIFFERENCE=2`, `DISABLED_BY_PROFILE=2`, demais `0`; total `20` | idêntico, digest `81956ea4b65401c94221bc6afba898ffa55e07063e38867e5c28d8bc707986f1` | `7a8a41b6b5b93c5f8abaae2e24edc0093bb718f0fb84be1ac536f91d965d82c4` |

Nos três payloads, `P1137-I-001` foi `MATCH` e seus comandos oracle/candidato
não contêm `--features`. `P1288-B-001` foi `DISABLED_BY_PROFILE` em default e
html, mas `MATCH` com `expected_state=MATCH` em a11y-extras. Assim, disabled não
é contado como match e uma incapacidade ativa futura não seria escondida.

Identidades dos binários desta medição:

- `/usr/local/bin/typst`: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- `target/release/typst`: `5f2841b03cc776dfdc55b43211a8f1f90307f8ba8b5b1f0f448a6adb0398553d`.

Hashes finais antes deste adendo:

| Saída | SHA-256 |
|---|---|
| `lab/parity/matrix/manifest.yaml` | `8c86e595af2ca93ea99945ec9ad5d65853052861962ad3354e1edc8947e1d40c` |
| `lab/parity/matrix/runner.py` | `e9d6c553ceb4fd896653a2b75ee71a33de808796b7f30ffb64d708ba7c908581` |
| `lab/parity/matrix/test_runner.py` | `cc6efb333bca3832f550ba80c7ad0a0c05a93fc5bcdd3f2fdc14f5273b015e6d` |

Os hashes protegidos permaneceram invariantes: selo
`22042fef8243a7bf9ce0f8e4ce1b5300dbc0bd1a13da97e97059bf59983b397e`,
runner de oráculos
`98bb462601c255708a646fdc7dc27e2f59455ade5198b2d26105299934723e48`,
testes de oráculos
`59f3c8ea71df9fee0221c4daf50c5fb53c4fde6dd0992eb1fd0aa3e849c77f99`,
baseline de oráculos
`ac0d97379820eb09f89c69e52858b231794bb4db6455d16527be2d04f1fd7a3d`
e receipt de oráculos
`894bf70aef8b0cce128f1c3a050a7ed8a2cb895d48feafbc2f97a3c22f108ff4`.
