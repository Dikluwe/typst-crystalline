# P1350 — autoria adversarial R2 congelada

## Veredito

`AUTHORED_NOT_EXECUTED`

A R2 foi congelada após o contrato R2 e antes de qualquer leitura ou execução
de oracle, checker, caller, supervisor, journal parser ou adapter P1350 R2.
Somente `--author-check` foi executado. Boundary, meta, focal, ptrace,
candidato e full permanecem em **0**. Não há score, survivor, discriminação ou
pré-selo neste artefato.

Regime: `executado sem atestacao de isolamento`. O workspace e o contexto são
compartilhados; a segregação é demonstrada por allowlist, ordem causal, hashes
de arquivo inteiro e escrita limitada, sem alegação de isolamento técnico.

## Refinamento medido e preservação da R1

O blocker público R1 mediu três incompatibilidades: um parâmetro `exact_ids`
impossível de satisfazer simultaneamente como 122 IDs e 60 operações; uma
delivery anterior ao adapter que não podia autorizar execução; e a ausência de
um canal oracle-owned para executar os 36 ataques de fronteira. A R2 corrige
somente a autoria adversarial afetada por essas ligações.

Os três artefatos adversariais R1 permanecem imutáveis e pinados:

| artefato R1 preservado | SHA-256 |
|---|---|
| `p1350-adversary-suite-r1.py` | `f6cbd4badd8ae824aec3b73f03fac2fdb169922341ba408c9297ae0ab66f6382` |
| `p1350-adversary-authorship-r1.md` | `d830e822d99f807d73b95b7278d06bf209725eb87e00adf2766bdeefc0296b8e` |
| `p1350-adversary-authorship-receipt-r1.json` | `7ef1d7e894e25b17647929ef28086e138d0c1ad84e8a5ef900eb3454f652b4e6` |

## Entradas diretas congeladas

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1350.md` | `e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf` |
| `p1350-contract-spec-r2.json` | `17c3b02d2f98ca12d6f89b369b78983663a8f8af6becf08daf47227b8cb42b90` |
| `p1350-contract-binding-r2.json` | `c341b0db70cebe933bbf6dea9fc19b2122f66534c4488ca4dc00ac9f3ab8f4dd` |
| `p1350-contract-receipt-r2.json` | `667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f` |
| `p1350-adapter-integration-blocker-r1.json` | `3f2494f92d10eed22a9d8d0d3647ca9fa532f82f235415fcca93074174c2e341` |
| `p1349-adversary-suite-r1.py` | `9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703` |
| `p1349-adversary-authorship-r1.md` | `ea364c7194fa2c3f9b15da2b53cac261cb4576e925713a2329e3d7b7484818b9` |
| `p1349-adversary-authorship-receipt-r1.json` | `ec08d9406e3fbd993d750236e454b6d3f7210c11a1eddc31b1770f558af7b6a2` |

Os baselines P1348/P1347/P1346 são alcançados apenas pelo carregamento e pela
validação transitiva da suíte P1349 pinada. Nenhum rótulo herdado foi
reconstruído.

## Três domínios fechados e não intercambiáveis

1. `LANGUAGE_ID`: **122** IDs distintos em ordem congelada. Digest canônico:
   `e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907`.
2. `FOCAL_OPERATION`: **60** operações distintas, das quais 12 locais e 48
   herdadas. A allowlist, sem rotas, foi congelada pelo adversário. Digest dos
   IDs pela fórmula contratual:
   `997e107d263c2ffa4b649ac9a7f8320dc673f56956800b06cb4316bab3baabd1`.
   O digest do mapa com rotas só poderá existir após autoria do adapter.
3. `BOUNDARY_ATTACK`: **36** tuplas contratuais exatas, em ordem, divididas em
   4 `STATIC_BYPASS`, 21 `DYNAMIC_SUPERVISOR`, 8 `JOURNAL` e 3 `FRESHNESS`.
   Digest canônico:
   `d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8`.

O nome `exact_ids`, a tradução entre domínios e o uso de um digest no campo de
outro domínio são rejeitados. A API futura usa, nominalmente e separadamente,
`language_id_sequence_sha256`, `focal_operation_route_map_sha256` e
`boundary_attack_manifest_sha256`.

## Fronteira adversarial dinâmica

Os 36 ataques preservam os blockers exatos definidos pelo contrato. Nenhum
ataque é aceito por resposta estática, nem os quatro de bypass. Para cada
invocação, a suíte gera nonce e challenge novos e só recomputa `PASS` quando o
resultado fechado contém:

- ID, boundary e blocker exatos;
- nonce/challenge devolvidos pelos hashes esperados e sem reutilização;
- injeção observada e marcada `oracle_owned=true`;
- invocação pública realmente observada por supervisor pai;
- caminho de executor real ou recusa pública autenticada;
- tempos monotônicos coerentes e identidade de executor coerente;
- journal de caso com SHA-256, `fdatasync` observado e run ID fresco;
- `group_absent`, `zombies_absent`, descritores fechados, registry destruído,
  patches restaurados e `/dev/shm` removido;
- blocker aninhado no domínio de integração, sem classificação, reason, meta
  ou score.

Nonce, challenge, journal, run ID e PID+starttime de executor não podem ser
reutilizados entre experimentos. Um status `PASS` ou o blocker esperado sem
essas evidências é falha terminal de G07.

## Retestes e denominadores preservados

- meta X06/X15: **2**, fora do score;
- X14 e variantes supervisoras: **12**;
- demais focais P1348: **22**;
- regressões P1347: **39**;
- replays P1346: **181**;
- controles: **11**;
- negativos semânticos válidos: **254**.

O denominador semântico continua 254. Os 36 ataques de fronteira, 2 meta,
11 controles, blockers e o `Unknown/OPAQUE_PAYLOAD` legítimo permanecem fora
do mutation score. Qualquer blocker inesperado é terminal.

## Custos e ordem futura

- deadline por caso: 12.000.000.000 ns;
- graça TERM: 250.000.000 ns;
- cleanup: 2.000.000.000 ns;
- teto por executor: 14.250.000.000 ns;
- teto sequencial conservador para os 36 ataques: 598.500.000.000 ns.

Ordem futura: delivery posterior ao adapter e mapa → bind do `RunContext` pelo
checker → pins dos três domínios → 36/36 boundary dinâmicos → meta 2/2 → 12
supervisoras → demais P1348 → 39 P1347 → 181 P1346 e 11 controles →
normal/repeat/reverse. A CLI desta autoria não possui caminho para essa
execução.

## Proveniência e validação

- estado base: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`;
- hora da medição: `2026-09-11T16:47:58-03:00`;
- working tree: não commitado e compartilhado, 93 entradas no instante;
- fingerprint NUL-safe de `git status --porcelain=v1 -z`:
  `bc24ff8e1992863c7cc97eacdee1ba0ae71d1275ae665b3764041b813798a8f8`;
- tracked diff e tracked diff-stat: ambos vazios, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- SHA-256 da suíte R2:
  `7b34c0ca03649a68a06bc88c27ec955a1fddcf1a62829397e7da068c91f68cdb`;
- `--author-check`: 0,13 s wall, 0,10 s user, 0,02 s system e 22.324 KiB
  de RSS máximo.

Foram validados sintaxe Python sem bytecode, hashes protegidos, JSON estrito,
cardinalidades, digests dos três domínios, cobertura transitiva, ausência de
execução e diff-check. O linter cristalino não foi executado porque varreria
inputs P1350 fora da allowlist cega desta autoria.
