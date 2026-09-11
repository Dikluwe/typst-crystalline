# P1349 — autoria adversarial R1 congelada

## Veredito

`AUTHORED_NOT_EXECUTED`

A suíte P1349 foi congelada antes de qualquer leitura ou execução de futuro
checker, caller, delivery ou adapter P1349. Apenas `--author-check` foi
executado. Meta-gate, focal, ptrace, candidato e full permanecem em **0**; este
artefato não é evidência de discriminação, aprovação ou pré-selo.

Regime: `executado sem atestacao de isolamento`. O filesystem e o contexto são
compartilhados, portanto a segregação está na allowlist, na ordem causal e no
escopo de escrita, sem alegação de isolamento técnico forte.

## Papel e limites

- Papel: adversário independente P1349 (`/root/p1346_adversary`).
- Escrita: somente suíte, este relatório e receipt adversarial R1.
- Entradas: passos P1349/P1348/P1347 pinados, contrato P1349 R1, blocker P1348,
  suíte/autoria/receipt P1348 e baselines contratuais/adversariais transitivas.
- O entrypoint futuro recebe checker, caller e delivery por `FrozenTargets`,
  todos com SHA-256 externo, e um adapter igualmente pinado.
- A CLI de autoria não alcança `run_with_adapter` nem resolve os 122 IDs de
  linguagem; essa resolução fica para o verificador após a entrega congelada.

## Pins

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1349.md` | `071ee50547bab6dc86b84d630b6c501867967121234cce79ab8de14075973a81` |
| `00_nucleo/materialization/typst-passo-1348.md` | `4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7` |
| `00_nucleo/materialization/typst-passo-1347.md` | `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c` |
| `p1349-contract-spec-r1.json` | `9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335` |
| `p1349-contract-binding-r1.json` | `3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd` |
| `p1349-contract-receipt-r1.json` | `6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6` |
| `p1348-verifier-blocker-r1.json` | `e5b9e9423e2e3ad20064cbc20f2594e25396f7e1bffe20123a4f8392cb8501de` |
| `p1348-adversary-suite-r1.py` | `14f3f41b28cfaa927197f5b5767612833e38944a9db74fb9f5ace7ee9b32e30c` |
| `p1348-adversary-authorship-r1.md` | `75f347b9c88dd8c0099814ddbc506e2a962ee916ad7526051871f669fcb7f424` |
| `p1348-adversary-authorship-receipt-r1.json` | `32ebf58fe116982e6c51190caf198e8346e5e1014bd10eb9d276ce96ac6e7bb6` |
| `p1348-contract-spec-r1.json` | `e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af` |
| `p1348-contract-binding-r1.json` | `de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f` |
| `p1348-contract-receipt-r1.json` | `1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf` |
| `p1346-adversary-report-r2.json` | `60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6` |
| `p1346-adversary-report-r2.md` | `bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466` |
| `p1346-adversary-receipt-r2.json` | `2fdf8efdd0f805c3dc321f34192a01715b273b9b071f51d231b80e71c88115a3` |

A sequência herdada permanece com 122 IDs ASCII distintos e SHA-256 canônico
`e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907`.

## META_TESTS fechados

`META_TESTS` contém exatamente, nesta ordem:

1. `P1348-X06-failure-after-registry-commit`;
2. `P1348-X15-historical-original-open-nameerror`.

Ambos usam `exercise_meta`, processos descartáveis e `score_eligible=false`.
X06 exige o blocker planejado `ADAPTER_EXCEPTION`, identidade `CONSUMED`,
`reopened=false`, segunda aceitação recusada, digest/ordinal preservados,
restauração por identidade e árvore de processos vazia. X15 exige o mesmo
blocker planejado com `NameError`, ausência de classificação/razão, igualdade
de `open` antes/depois, restauração completa e árvore vazia.

O resultado externo aceito é somente `p1349-meta-result-r1/PASS`, sem campos de
classificação ou blocker. O blocker interno observado não entra no numerador,
denominador ou score. Blocker errado, postcondição divergente, restauração
incompleta ou vazamento produz meta FAIL terminal.

## FOCAL classificável

O corpus classificável congelado soma **254 negativos válidos**:

- G05: **12** — X14 mais 11 variantes P1349;
- G06: **22** focais P1348 restantes, excluídos X06, X15 e X14;
- G07: **39** regressões focais P1347;
- G08: **181** replays P1346.

Os **11 controles** P1347 permanecem separados e fora do score. As expectativas
herdadas são copiadas das suítes pinadas; quando um controle anterior não
congelou reason code, P1349 não inventa um.

X14 continua `Violated/PROBE_AUTHORITY` apenas se o checker detectar o segundo
exec e terminar/recolher todos os processos antes do deadline próprio, sem
sinal do supervisor externo. As onze variantes cobrem:

- segundo exec depois do exit-event, na borda do deadline e ignorando término
  gracioso;
- pidfd divergente após handshake, PID reutilizado com starttime diferente e
  mistura de pidfd/streams/status de sibling;
- detach depois do primeiro exec e no exit-event antes do terminal;
- anchor mutante com streams autênticos e self-answer DAG de schema válido;
- tentativa de mascarar segundo exec com blocker inesperado.

Os reasons congelados são `PROBE_AUTHORITY`, `WORKER_AUTHORITY`,
`AUTHORITY_ROOT` e `DAG_VALIDATOR`, conforme a fronteira atacada. Nenhuma
variante aceita timeout externo como morte do mutante.

## Terminalidade e execução futura

`exercise_focal` só pode devolver `p1349-classification-result-r1`. Qualquer
`p1349-integration-blocker-r1`, exceção, timeout, meta_result, output parcial ou
reason divergente falha o caso e interrompe todas as fatias seguintes. O runner
conta blockers inesperados separadamente e nunca lhes concede score.

A ordem futura é meta 2/2 → X14/variantes → 22 P1348 → 39 P1347 → 181 P1346
→ 11 controles. Normal/repeat/reverse e seus processos/registries frescos são
responsabilidade do verificador, após zero falhas nas fatias anteriores.

## Medição de autoria

- Estado base: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`.
- Hora: `2026-09-11T14:34:27-03:00`.
- Working tree: não commitado e compartilhado.
- Fingerprint NUL-safe do `git status --porcelain=v1 -z` antes deste relatório:
  `c07dcd07fbf60760e1191ec82059eb232c1823cb0d84ed158cc572affd56bfab`.
- SHA-256 da suíte:
  `9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703`.
- `--author-check`: META=2, P1348 focal=23, variantes novas=11,
  regressões=39, replays=181, controles=11, denominador=254.
- Sintaxe Python: válida por `compile(...)`, sem bytecode.
- Execuções de meta/focal/ptrace/full/candidato/alvos P1349: zero.

Nenhum score ou sobrevivente foi medido. O veredito permanece exclusivamente
`AUTHORED_NOT_EXECUTED`.
