# P1350 — autoria adversarial R1 congelada

## Veredito

`AUTHORED_NOT_EXECUTED`

A suíte foi congelada antes de qualquer leitura ou execução de oracle,
checker, caller, supervisor, parser ou adapter P1350. Somente `--author-check`
foi executado. Focal, ptrace, candidato e full permanecem em **0**; este
resultado prova autoria e cobertura estática, não discriminação nem pré-selo.

Regime: `executado sem atestacao de isolamento`. O workspace/contexto são
compartilhados, logo a segregação é por allowlist, ordem causal e escrita
restrita, sem alegação de isolamento técnico forte.

## Papel, entradas e saída

- Papel: adversário independente P1350 (`/root/p1346_adversary`).
- Escrita: suíte, este relatório e receipt adversarial R1.
- Entradas diretas: passo/contrato P1350, blocker público P1349 R2, contrato e
  autoria adversarial P1349.
- Baselines P1348/P1347/P1346 são consumidos apenas pela suíte P1349 pinada e
  pelas suas validações transitivas; nenhum rótulo herdado é recriado.
- A execução futura recebe alvos e um driver do verificador por hashes de
  arquivo inteiro. O driver exercita ataques de borda em invocações isoladas;
  ele não acrescenta uma segunda entrada pública ao adapter P1350.

## Pins diretos

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1350.md` | `e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf` |
| `p1350-contract-spec-r1.json` | `bdc3f3c0a6ef7baec3b2cb2e31c88c099dc5a49b18e97ae0a0fcc86535286e82` |
| `p1350-contract-binding-r1.json` | `257114e18af3ef0d968294e5e9b8326bd963eb7fa92a1d2c489c6e05f388fcfb` |
| `p1350-contract-receipt-r1.json` | `e741751e0641c46be2e8aa634cc96b0ee400cd3b6d5693b25eec66cba283008d` |
| `p1349-verifier-blocker-r2.json` | `16780a65c46ba958b6e1535184ea039d022f558a01fb384cc0c110c261d55417` |
| `p1349-contract-spec-r1.json` | `9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335` |
| `p1349-contract-binding-r1.json` | `3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd` |
| `p1349-contract-receipt-r1.json` | `6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6` |
| `p1349-adversary-suite-r1.py` | `9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703` |
| `p1349-adversary-authorship-r1.md` | `ea364c7194fa2c3f9b15da2b53cac261cb4576e925713a2329e3d7b7484818b9` |
| `p1349-adversary-authorship-receipt-r1.json` | `ec08d9406e3fbd993d750236e454b6d3f7210c11a1eddc31b1770f558af7b6a2` |

## Cobertura não contornável

O manifesto derivado contém exatamente **60 operações únicas**: 12 rotas
P1349 locais e 48 rotas herdadas. Cada uma se liga à entrada focal única e a
um executor descartável; são exigidos `supervised=60`, `unbound=0`,
`duplicate=0` e `direct_inherited_edges=0`.

SHA-256 canônico do manifesto de rotas:
`188a87cdd88f273bcf1129f5a61b0498b86ccf8dad4bdbcc5332fb29a2108d63`.

## Ataques de fronteira

Foram congelados **36 ataques**, todos fora do mutation score e executáveis
futuramente em invocações adversariais isoladas:

- **4 bypass estáticos:** chamada herdada crua no batch, flag/fallback de
  desativação, operação omitida/duplicada e rota escolhida antes do ACK.
- **21 supervisor dinâmicos:** stop antes/depois do handshake, deadline/TERM
  ignorados, descendente em `ptrace_stop`, divergência SID/PGID,
  PID/starttime/pidfd, fork sem ACK, órfão, reap errado, frames ausente/parcial/
  duplicado/não canônico/sobredimensionado, capability ausente/repetida/trocada
  e exceções de adapter/checker.
- **8 journal:** truncamento/LF ausente, hash anterior/atual quebrado, evento
  fora de ordem, dois casos ativos, caso sem fechamento, resultado antes de
  `group_absent`, `run_complete` prematuro/duplicado e writer executor.
- **3 frescor:** registry reutilizado em repeat, FD reutilizado em reverse e
  processo/monkeypatch residual entre casos.

Os outcomes congelados são blockers `SUPERVISOR_AUTHORITY`,
`SUPERVISOR_TIMEOUT`, `ORPHAN_PROCESS`, `CLEANUP_FAILURE`, `FRAME_AUTHORITY`,
`JOURNAL_AUTHORITY`, `ADAPTER_EXCEPTION` ou `CHECKER_ERROR`, de acordo com a
primeira fronteira violada. Cada resultado deve estar no domínio fechado
`p1349-integration-blocker-r1`, sem `classification` ou `reason_code`, e só é
aceito pelo ensaio adversarial após `group_absent=true` e
`zombies_absent=true`.

Um blocker previsto numa invocação de ataque demonstra fail-closed dessa
invocação; não pontua nem é promovido a razão semântica. Qualquer blocker na
execução principal continua terminal.

## Retestes preservados

A suíte liga, sem alterar expectativas:

- meta X06/X15: **2**, fora do score;
- X14 e variantes supervisoras P1349: **12**;
- rotas de operação herdadas: **48**;
- demais focais P1348: **22** casos;
- regressões P1347: **39**;
- replays P1346: **181**;
- controles: **11**.

O denominador semântico continua **254**. Os 36 ataques de fronteira, meta,
controles, blockers e o único `Unknown/OPAQUE_PAYLOAD` legítimo não entram no
denominador. Reasons ausentes em controles herdados permanecem não
especificados; a suíte não fabrica rótulos.

## Custos e ordem futura

- Deadline por caso: 12.000.000.000 ns.
- Graça TERM: 250.000.000 ns.
- Cleanup após KILL: 2.000.000.000 ns.
- Teto conservador por invocação/executor: 14.250.000.000 ns.
- Teto conservador sequencial declarado para a fatia de 36 ataques, incluindo
  casos multi-executor: **598.500.000.000 ns** (598,5 s).
- Autoverificação medida: 0,09 s wall, 0,07 s user, 0,01 s system e 22.312 KiB
  de RSS máximo; processos-alvo, ptrace, focal e full = 0.

Ordem futura: cobertura 60/60 → ataques de fronteira → meta 2/2 → X14+11 →
demais P1348 → 39 P1347 → 181 P1346 + 11 controles → schedules frescos.
Qualquer resíduo, cleanup externo, hash/journal inválido, survivor ou blocker
inesperado interrompe imediatamente.

## Proveniência

- Estado base: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`.
- Hora: `2026-09-11T16:14:08-03:00`.
- Working tree: não commitado e compartilhado.
- Fingerprint NUL-safe de `git status --porcelain=v1 -z` antes deste relatório:
  `43cbed758f68e6632a7b276971f772fcc9b851ee4767f77d14d5219f09aa3bb7`.
- SHA-256 da suíte:
  `f6cbd4badd8ae824aec3b73f03fac2fdb169922341ba408c9297ae0ab66f6382`.

Sintaxe, pins, cardinalidades, rotas e custos foram validados apenas em modo de
autoria. Nenhum score, survivor ou resultado dinâmico P1350 foi medido.
