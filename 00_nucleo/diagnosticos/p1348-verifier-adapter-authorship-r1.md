# Autoria segregada do adapter operacional P1348 R1

Data: 2026-09-11T13:54:55-03:00

Papel: autor exclusivo do adapter P1348; não autor do contrato, do oráculo, da suíte adversarial, do candidato ou do veredito.

Regime: `executado sem atestacao de isolamento`.

## Resultado de autoria

Foi escrito `00_nucleo/diagnosticos/p1348-verifier-adapter-r1.py` depois do congelamento independente do contrato, do oráculo e da suíte adversarial P1348 R1. O adapter implementa `exercise(case, targets, order)` e possui despacho fechado para todas as 51 operações distintas presentes nas fatias congeladas: 25 ataques primeiro-focal, 39 regressões P1347, 181 replays P1346 e 11 controles P1347.

Isto é somente uma declaração de autoria e cobertura estática. Nenhum caso foi executado, nenhuma classificação foi aceita e nenhum veredito P1348 foi produzido.

## Mapa operacional estático

| rota | operações distintas | API ou fronteira operacional |
|---|---:|---|
| `worker` | 12 | `checker.start_worker` + `checker.validate_worker`, com mutação do `PendingWorkerRun` ou operação explícita do worker |
| `freshness` | 6 | `ParentFreshnessRegistry`, `start_worker`, `validate_worker` e `fail_after_commit`; registry compartilhado somente dentro dos casos que pedem replay/transição multi-etapa |
| `dag` | 8 | `checker.run_dag`; adulteração de stream no retorno de `capture_process` somente no caso explicitamente nomeado |
| `probe` | 17 | `checker.trace_probe`, incluindo hooks operacionais explícitos e mutações restauradas; imagem compilada somente quando o verificador futuro exercer caso probe, em diretório temporário sob `/dev/shm` |
| `parameterized_process` | 1 | seleciona a API worker ou probe exclusivamente pelo parâmetro congelado `foreign` |
| `replay` | 1 | uma execução autenticada do caller P1346 `--focus` via `checker.capture_process`/`validate_transaction`, seguida da projeção exata por `source_case_id` para os 181 replays |
| `control` | 5 | worker, DAG, probe e projeções de controles pelas mesmas APIs canônicas |
| `adapter_meta` | 1 | exceção histórica injetada depois de instalar/restaurar `open`, produzindo somente blocker de integração |

Total estático: 51 nomes na suíte, 51 nomes no adapter, ausentes 0, excedentes 0. Cardinalidades congeladas observadas sem chamar o runner: sobreviventes P1347 10, variantes P1348 15, regressões P1347 39, controles P1347 11; os 181 replays são representados pela operação parametrizada `replay_p1346_negative`.

## Boundary de exceção e reparo P04

Toda exceção inesperada de import, setup, mutação, invocação, cleanup ou restauração atravessa `AdapterFault` e retorna `outcome_kind = integration_blocker`, `classification = null` e `reason_code = null`. O blocker aninhado tem exatamente as chaves fechadas pelo contrato e registra fase, tipo, digest da mensagem e resultado de restauração. O adapter não lê `expected_classification`, `expected_reason_code` ou `expected_observations`.

Na operação `swap_fd_after_exec_stop`, `original_open = checker.os.open`, `original_memfd_create` e `original_write` são capturados no mesmo escopo antes das closures. Os três swaps são instalados pela mesma fronteira `try/finally`, a API canônica `checker.trace_probe` é invocada uma vez e cada objeto é restaurado e comparado por identidade antes do retorno. O caso meta histórico também cruza uma instalação/restauração real de `open` e só então emite `NameError` como blocker de fase `mutation`; ele não fabrica `PROBE_AUTHORITY`.

As classificações e razões transportadas são exclusivamente as observadas nas APIs congeladas. Em particular, o adapter não remapeia `AUTHORITY_ROOT`, `DAG_VALIDATOR`, `WORKER_AUTHORITY` ou `PROBE_AUTHORITY` e converte somente falhas internas reconhecidas do checker em observações do checker. Exceções do próprio adapter não recebem razão de caso.

## Entradas congeladas consumidas

- passo P1348: `4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7`;
- passo P1347: `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c`;
- manifest P1348: `70754da5c833a43f1943ec7abdb453f67e8889b146b27b883e04097531bad6bb`;
- contract spec/binding/receipt P1348: `e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af`, `de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f`, `1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf`;
- adversary suite/authorship/receipt P1348: `14f3f41b28cfaa927197f5b5767612833e38944a9db74fb9f5ace7ee9b32e30c`, `75f347b9c88dd8c0099814ddbc506e2a962ee916ad7526051871f669fcb7f424`, `32ebf58fe116982e6c51190caf198e8346e5e1014bd10eb9d276ce96ac6e7bb6`;
- oracle corpus/checker/caller P1348: `3e33630290cdfe681e855dee4c3becfde0aac53560d6d1c72c4e0fe83bb25458`, `fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249`, `2ebb7f9497b3c1ef9088b06b5e32704df464296f45c831b18c695c08f72e1e43`;
- oracle authorship/receipt/delivery P1348: `3742abff455a71865a1196712a641ef622f6a35ed39b996e9ee9398d13af9a46`, `54f03388e58e5adb1acd55f27a19e8a0f0e28072c4e8ee3f9d23949c860bf24f`, `c06a0c78369f33e677830782a6fcd20ada7dc6ae037b90b6160b5517390f7040`;
- blocker P1347: `3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522`;
- adapter P1347: `cc71d74b85b14e6624aae53630f25a3c5fe34724cab7d62222f84377d6845a7a`.

Todos os hashes acima foram recalculados sobre bytes de ficheiros regulares e coincidiram com os pins congelados aplicáveis.

## Validação permitida

- `compile(source, path, "exec")`: passou; não importou nem executou o adapter;
- cobertura estática: 51/51 nomes, 0 ausentes, 0 excedentes;
- busca estática: nenhuma leitura das chaves de expectativa e nenhuma referência a `run_with_adapter`, `run_focal` ou `--full` no adapter;
- pins SHA-256 das entradas: passaram;
- `git diff --check` e equivalente `--no-index --check` para ficheiros ainda não rastreados: sem erro de whitespace.

Não foram chamados `run_with_adapter`, focal P1348/P1347/P1346, full, candidato, `ptrace` nem qualquer API operacional do checker/caller. A presença de chamadas operacionais no código é a implementação a ser exercida somente por verificador independente posterior.

## Proveniência da medição

Commit observado: `496c45ac6279d4298079f848b24b5e97ba88edc1`. Working tree não commitado. No escopo de autoria, antes deste relatório, o único output novo era `00_nucleo/diagnosticos/p1348-verifier-adapter-r1.py`. Hora da medição: `2026-09-11T13:54:55-03:00`.

Status de autoria: `AUTHORED_NOT_EXECUTED`. Este status não é aceitação, pré-selo ou veredito.
