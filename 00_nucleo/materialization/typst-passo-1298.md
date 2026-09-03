# Passo 1298 — sanitização terminal append-only e commit atômico P1294–P1297

## Objetivo

Fechar o grafo evidencial deixado pelo P1297 sem alterar produto, L0, testes,
contratos ou superfícies. O P1298:

1. preserva integralmente os artefatos terminais existentes;
2. registra uma continuação append-only para o ledger P1297 incompleto;
3. reconcilia, sem reescrever, o rótulo divergente do receipt final;
4. inclui no Git todos os predecessores referenciados ainda não rastreados;
5. verifica a allowlist exata e cria um único commit atômico.

P1298 não reexecuta nem reabre R1, R2 ou R3. O redesenho vencedor continua
`R1_CAPABILITY`. P1294, P1295 e P1296 continuam `BLOCKED`, byte-preservados e
não absolvidos.

## Regime e limite da alegação

Regime: selagem evidencial Tekt segregada por entradas, capacidades, ordem e
artefatos. Mutation testing não é aplicável porque P1298 não materializa nova
semântica de produto. A evidência discriminatória R1 já congelada permanece
entrada, não é reexecutada nem recebe novo crédito.

Não há alegação de isolamento técnico de leitura do filesystem compartilhado.
Não há alegação de equivalência funcional geral.

Política de `Unknown`: identidade ausente, hash não reproduzível, relação
ambígua ou path não classificado permanece `Unknown` e bloqueia commit. Nunca é
convertido em sucesso por default.

## Proveniência congelada

Medição em `2026-09-03T17:04:13,464127274-03:00`:

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`;
- commit: `docs: record blocked step 1294 verification`;
- data: `2026-09-02T22:23:37-03:00`;
- working tree: 40 entradas em `git status --porcelain=v1`;
- SHA-256 do output exato de `git status --porcelain=v1`:
  `a68a7abb4988874507ffc077575362bbe1234dd10cf3034d5592ac38829ad217`;
- diff rastreado: 22 arquivos, 13.738 inserções e 124 remoções;
- SHA-256 do output exato de `git diff HEAD --stat`:
  `f35801a853628f0984f72de88ec3aaf6948a072679c494ce5b54ae344b27cf46`;
- índice atual: 22 paths, todos da cadeia P1297;
- SHA-256 do output exato de `git diff --cached --name-only`:
  `83cf1db8b37e229f8fe2d9e6276a28f3306f23c2991f9826afe71caad26ca384`;
- SHA-256 do patch binário exato `git diff --cached --binary`:
  `65d890b79c83bee191dbbf5d24f38bf180dc0f9135e23cc3a179f3a17d3f1314`;
- 18 paths causais P1294–P1296 permanecem não rastreados.

O índice não deve ser esvaziado nem reconstruído por comando destrutivo. O
estado de 22 paths é uma entrada a auditar; só pode receber adições da allowlist
P1298 depois do receipt de verificação.

## Entradas terminais P1297

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1297.md` | `5116ffc535bb8cbd33ca8fb449b492ddfcf9e133246d7143c27a97960748c682` |
| `00_nucleo/diagnosticos/p1297-manifest.json` | `5bf3f29e353403006b90971e05f786ad2548648c053b9db59f879841aecdf322` |
| `00_nucleo/diagnosticos/p1297-redesign-ledger.json` | `bfc87da5047004f1cce67df0a14a7f6b9e1fb341a77169fc6ac1af8faaa91db0` |
| `00_nucleo/diagnosticos/p1297-r1-contract-seal.json` | `2710fbbbe8748176764d37be40b40c30d93d39f1442e02064f32c7ff31348e92` |
| `00_nucleo/diagnosticos/p1297-implementation-receipt.md` | `7b23e0c2de48343debfa9ae684202729db450f5adbfe15a000dcb24495e228cb` |
| `00_nucleo/diagnosticos/p1297-verification-receipt.json` | `c9dd6046ebea9c90397212c1fce2ec3e00d5e473df0211f3cecef9c0c50ba111` |
| `00_nucleo/diagnosticos/p1297-certificate.json` | `b7d429d76fd691238eef6ad2a67c8bf7631a5e91fa48b042d2d4f947f6ddac2b` |
| `00_nucleo/diagnosticos/p1297-final-report.md` | `e0175e76d01ce86114996f2abbb59408226f739aa5ebfa960e740999ea89722b` |

Consumers produtivos/test-only congelados pelo certificado:

| Path | SHA-256 |
|---|---|
| `03_infra/src/watch.rs` | `34188c0a3bdc129abc5c3eb9636a5fc5a701fc68f61c530c7a7f08a80a961b3c` |
| `04_wiring/src/main.rs` | `40a7885a2d4aa6c9a02921461be44928f14e75f072bcda203433c55d1771a6f7` |
| `04_wiring/tests/cli.rs` | `56d989a4f79112d59558a71b0dbf609aa4022f9df0290e56afebc1b2b218a42b` |
| `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` | `4b30a7da9310892ad639ba2483fd8d995f6cae4adb4f5ec959247b7911f583e8` |
| `03_infra/tests/p1297_watch_capability_contract.rs` | `9721cfb5fce6dc5bb9c41d004495a0838069c2f9ae2492731fde30b53827f089` |

Qualquer divergência nesses hashes bloqueia P1298; não autoriza correção ou
reexecução de produto dentro deste passo.

## Medição antes da decisão

### Achado A — ledger congelado antes da conclusão

O ledger P1297 atual, apesar de pinado pelo certificado, contém:

```text
terminal_state = ...P1297_NOT_CERTIFIED
R1.status = DISCRIMINATED_READY_FOR_P4
R1.sealed = false
R1.candidate_promoted = false
next_authority = P4_R1_INDEPENDENT_SEALER
```

Sua lista de transições termina em
`R1_CAPABILITY_DISCRIMINATED_READY_FOR_P4`. Entretanto existem, com hashes
posteriores válidos, selo P4, receipt de implementação P5, três execuções P6,
receipt final PASS, certificado e relatório.

Inferência: o ledger foi tratado de fato como snapshot pré-P4, mas o P1297 o
nomeia como ledger canônico de transições e exige atualização após gates. A
inferência seria refutada se o próprio ledger se declarasse deliberadamente
terminal em P3 ou apontasse para um journal sucessor; seus bytes fazem o
contrário.

### Achado B — rótulo divergente no certificado

O receipt final P1297 contém simultaneamente:

```text
verdict = PASS
ready_for_certificate = true
terminal_state = ...P1297_PASS_READY_FOR_CERTIFICATE
```

O certificado pina corretamente o SHA-256 do receipt, mas registra em
`causal_chain.final_verification.verdict` o texto
`PASS_READY_FOR_CERTIFICATE`, que não é o valor byte-semântico do campo
`verdict`. Esse texto corresponde ao estado/autorização combinados, não ao
campo citado.

Inferência: é uma divergência de rotulagem documental, não de execução, porque
o hash do receipt, `ready_for_certificate=true`, o estado terminal e os gates
continuam coerentes. Refutadores: hash diferente, `ready_for_certificate=false`,
gate real falho na terceira execução ou receipt editado depois do certificado.
Qualquer refutador bloqueia; nenhum pode ser normalizado silenciosamente.

### Achado C — grafo Git incompleto

O índice contém 22 paths P1297, mas não contém 18 predecessores presentes no
filesystem e referenciados causalmente. Um commit do índice atual deixaria
passos, receipts e contrato P1295/P1296 fora do histórico que contém o
certificado P1297.

Classificação: isso não altera semântica Typst, porém quebra a reprodutibilidade
do certificado e a proveniência do commit. O commit atual não está autorizado.

## Decisão de sanitização append-only

### S1 — imutabilidade dos predecessores

É proibido editar, renomear, regenerar, “corrigir” ou substituir:

- qualquer artefato P1294–P1297;
- o ledger P1297;
- o receipt final P1297;
- o certificado/relatório P1297;
- L0s, Núcleos, Rust, testes ou superfícies.

Os achados A/B são reconciliados por novos artefatos P1298 que pinam os bytes
originais e dizem explicitamente onde a cadeia continuou. Não há retroação.

### S2 — terminal ledger sucessor

Criar `00_nucleo/diagnosticos/p1298-terminal-ledger.json` com:

- predecessor exato `p1297-redesign-ledger.json` e seu SHA;
- classificação do predecessor como `PRE_P4_SNAPSHOT_FROZEN`;
- transições append-only, nesta ordem:
  1. `R1_DISCRIMINATED_READY_FOR_P4` → `R1_CONTRACT_SEALED`;
  2. `R1_CONTRACT_SEALED` → `R1_CANDIDATE_PROMOTED`;
  3. `R1_CANDIDATE_PROMOTED` → `P6_ATTEMPT_1_BLOCKED_ENOSPC`;
  4. tentativa 1 → `P6_ATTEMPT_2_BLOCKED_TMPDIR_OBSERVABLE`;
  5. tentativa 2 → `P6_ATTEMPT_3_PASS_READY_FOR_CERTIFICATE`;
  6. receipt PASS → `P1297_CERTIFICATE_EMITTED`;
  7. certificado → `P1297_FINAL_REPORT_EMITTED`;
  8. relatório → `P1298_EVIDENCE_CLOSURE_PENDING_COMMIT`;
- hashes dos receipts/selo/implementação/certificado/relatório usados em cada
  aresta;
- estado R1 final `sealed=true`, `candidate_promoted=true`, R2/R3 não iniciados;
- P1294/P1295/P1296 preservados como `BLOCKED` e não absolvidos.

O ledger sucessor não diz que o ledger P1297 já continha essas transições. Ele
declara a continuidade explicitamente.

### S3 — reconciliação semântica do receipt

Criar `00_nucleo/diagnosticos/p1298-audit-receipt.json` contendo, entre outros:

```json
{
  "actual_receipt_verdict": "PASS",
  "actual_ready_for_certificate": true,
  "actual_terminal_state_suffix": "P1297_PASS_READY_FOR_CERTIFICATE",
  "certificate_nested_label": "PASS_READY_FOR_CERTIFICATE",
  "field_value_equal": false,
  "semantic_authorization_consistent": true
}
```

`semantic_authorization_consistent=true` só é permitido se:

- receipt e certificado reproduzirem os hashes congelados;
- o receipt tiver `verdict=PASS`;
- `ready_for_certificate` for booleano `true`;
- o estado terminar exatamente em `PASS_READY_FOR_CERTIFICATE`;
- a terceira execução declarar zero falha real de gate;
- as tentativas anteriores permanecerem bloqueadas e incorporadas/pinadas;
- nenhum predecessor tiver mudado.

O audit receipt deve chamar a divergência pelo nome
`DOCUMENTARY_FIELD_LABEL_MISMATCH_NON_RETROACTIVE`. Não pode afirmar igualdade
de campos nem reescrever o certificado P1297.

### S4 — fechamento do conjunto Git

O commit P1298 deve conter exatamente 48 paths da allowlist abaixo:

- os 22 já staged de P1297;
- os 18 predecessores atualmente não rastreados;
- este passo e os 7 artefatos P1298 previstos.

Qualquer quantidade diferente de 48, path adicional, path ausente ou alteração
de bytes entre verificação e commit bloqueia.

## Artefatos P1298 previstos

1. `00_nucleo/materialization/typst-passo-1298.md`;
2. `00_nucleo/diagnosticos/p1298-manifest.json`;
3. `00_nucleo/diagnosticos/p1298-audit-receipt.json`;
4. `00_nucleo/diagnosticos/p1298-terminal-ledger.json`;
5. `00_nucleo/diagnosticos/p1298-staging-manifest.json`;
6. `00_nucleo/diagnosticos/p1298-verification-receipt.json`;
7. `00_nucleo/diagnosticos/p1298-certificate.json`;
8. `00_nucleo/diagnosticos/p1298-final-report.md`.

Não criar L0, teste, superfície, contrato ou arquivo de produto P1298.

## Allowlist exata do commit

### Grupo A — 22 paths P1297 já no índice

1. `00_nucleo/diagnosticos/p1297-certificate.json`
2. `00_nucleo/diagnosticos/p1297-final-report.md`
3. `00_nucleo/diagnosticos/p1297-implementation-receipt.md`
4. `00_nucleo/diagnosticos/p1297-manifest.json`
5. `00_nucleo/diagnosticos/p1297-r1-adversarial-plan.md`
6. `00_nucleo/diagnosticos/p1297-r1-contract-seal.json`
7. `00_nucleo/diagnosticos/p1297-r1-discrimination-receipt.json`
8. `00_nucleo/diagnosticos/p1297-r1-l0-gate-receipt.md`
9. `00_nucleo/diagnosticos/p1297-r1-red-tests-receipt.json`
10. `00_nucleo/diagnosticos/p1297-redesign-ledger.json`
11. `00_nucleo/diagnosticos/p1297-surface-default.json`
12. `00_nucleo/diagnosticos/p1297-surface-html.json`
13. `00_nucleo/diagnosticos/p1297-verification-receipt.json`
14. `00_nucleo/materialization/typst-passo-1297.md`
15. `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md`
16. `00_nucleo/prompts/shell/watch.md`
17. `00_nucleo/prompts/wiring.md`
18. `00_nucleo/prompts/wiring/tests/cli.md`
19. `03_infra/src/watch.rs`
20. `03_infra/tests/p1297_watch_capability_contract.rs`
21. `04_wiring/src/main.rs`
22. `04_wiring/tests/cli.rs`

### Grupo B — 18 predecessores a adicionar

1. `00_nucleo/diagnosticos/p1294-blocked-report.md`
2. `00_nucleo/materialization/typst-passo-1295.md`
3. `00_nucleo/diagnosticos/p1295-manifest.json`
4. `00_nucleo/diagnosticos/p1295-l0-gate-receipt.md`
5. `00_nucleo/diagnosticos/p1295-red-tests-receipt.json`
6. `00_nucleo/diagnosticos/p1295-adversarial-plan.md`
7. `00_nucleo/diagnosticos/p1295-discrimination-receipt.json`
8. `00_nucleo/diagnosticos/p1295-contract-seal.json`
9. `00_nucleo/diagnosticos/p1295-implementation-receipt.md`
10. `00_nucleo/diagnosticos/p1295-verification-receipt.json`
11. `00_nucleo/prompts/infra/tests/p1295_watch_contract.md`
12. `03_infra/tests/p1295_watch_contract.rs`
13. `00_nucleo/materialization/typst-passo-1296.md`
14. `00_nucleo/diagnosticos/p1296-manifest.json`
15. `00_nucleo/diagnosticos/p1296-l0-receipt.md`
16. `00_nucleo/diagnosticos/p1296-test-receipt.json`
17. `00_nucleo/diagnosticos/p1296-adversarial-plan.md`
18. `00_nucleo/diagnosticos/p1296-discrimination-receipt.json`

### Grupo C — 8 paths P1298

Os oito caminhos listados em “Artefatos P1298 previstos”.

## Hashes congelados do Grupo B

| Path | SHA-256 |
|---|---|
| `p1294-blocked-report.md` | `b87dfe26f22bf46dd20e80a568a34b8f9da26138616932b3a84e6e522a8f959b` |
| `typst-passo-1295.md` | `e0328b02cecdfefb8dc7acc29d3faff50a2ce5ab1cf92d8cd2d77a0c6e0f0663` |
| `p1295-manifest.json` | `545c3c661dd11685ff152d889dab7e796bda0659d91ca447c6d3db3794fc2999` |
| `p1295-l0-gate-receipt.md` | `155f2c22359b98739b66507e6fc7c678d187a1d8030a3f7b7ed70a1f31b68d78` |
| `p1295-red-tests-receipt.json` | `ac9a8fb41675026bafc884575b387998c45b8318f110508293e99faca1ec4d05` |
| `p1295-adversarial-plan.md` | `4788f57715e193527bf2e0a1ab86bf1fceee955e7f5933ace51cbb3291e48487` |
| `p1295-discrimination-receipt.json` | `30392dda4a3f639d39b18d693118feb42182f5f35e0cb615cb7b09d77c75ca51` |
| `p1295-contract-seal.json` | `cdd0f5ddc9740808fdee4c0c24d9d05d50c3243bab124678f9d6fba83b86d021` |
| `p1295-implementation-receipt.md` | `c02546962521a58c945114249b39d3a3d9959e91f7a1fd77a4505a17eb3f8bdc` |
| `p1295-verification-receipt.json` | `06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573` |
| `prompts/infra/tests/p1295_watch_contract.md` | `e39d5206b63b883f9a886a01ba2aaa7ff94259542a33693d5927a0a20445b233` |
| `03_infra/tests/p1295_watch_contract.rs` | `8dae68ffc12f2708b6bbe3d503a7d0d90a2cb92681762ff5a992ca3f178c7abf` |
| `typst-passo-1296.md` | `5a7f7c5d364fa6a373c1d411b8710131c68198bfeaaa4b19ec53fb17f7b8c00b` |
| `p1296-manifest.json` | `7c9e2f75aab348ce434c293ea0b38259fc8c42b39360d6ca3b63a17491edd7d0` |
| `p1296-l0-receipt.md` | `224dd65d269f3a81b132f2679b2c373ab5bcf910d0ed21e94b554dbe761924a2` |
| `p1296-test-receipt.json` | `be8aaf3537bb068aab8eff1c5b656fe74c56cec8bd144f22e48a12b486d3e8a0` |
| `p1296-adversarial-plan.md` | `8b8a8435ff794f7d0d8ba5ecc0c0af2c8f88ac2b3133bc05dd8eaa2c2eb9ce6b` |
| `p1296-discrimination-receipt.json` | `51073959b26cc834f1f1dbe69b5c4e70b5e7f1c71d28282b7424e0306e641525` |

Os labels abreviados desta tabela não substituem os paths absolutos relativos
da allowlist. O staging manifest usa sempre o path completo.

## Papéis e capacidades

### P0 — autor da obrigação e manifesto

Pode escrever somente este passo e `p1298-manifest.json`. Congela HEAD, os 40
paths iniciais, hashes, achados A/B/C, allowlists e política de `Unknown`. Não
edita predecessores nem emite veredito.

### P1 — auditor independente do grafo

Recebe passo, manifesto e artefatos P1294–P1297 em somente leitura. Recalcula
hashes, valida JSON, relações e payloads incorporados. Escreve apenas
`p1298-audit-receipt.json`. Não corrige o que audita.

### P2 — autor do ledger sucessor

Recebe audit receipt congelado. Escreve somente
`p1298-terminal-ledger.json`. Não altera o ledger P1297 nem escolhe
retrospectivamente o resultado dos gates.

### P3 — autor do staging manifest

Recebe passo, manifesto, audit receipt e terminal ledger. Escreve somente
`p1298-staging-manifest.json`, com os 48 paths esperados, classe de cada path e
hash conhecido naquele predecessor causal. Não toca no índice.

### P4 — verificador independente

Recebe todos os artefatos congelados e não os edita. Escreve somente
`p1298-verification-receipt.json`. Autoriza certificado/staging apenas se o
grafo, os hashes, a reconciliação e os checks forem verdes.

### P5 — selador documental

Recebe receipt P4 congelado. Escreve apenas `p1298-certificate.json` e depois
`p1298-final-report.md`. O certificado não afirma que o commit já ocorreu;
autoriza o commit da árvore exata.

### P6 — stager e committer

Não edita nenhum arquivo. Pode somente:

- verificar novamente os hashes;
- executar `git add` com a lista literal de 48 paths;
- executar checks cached;
- criar o commit único;
- reportar fora do repositório o hash do commit e estado posterior.

P6 não escreve recibo autorreferente dentro do commit. A identidade terminal é
o próprio objeto Git criado.

## Ordem causal obrigatória

```text
passo P1298
-> manifesto
-> auditoria read-only
-> audit receipt
-> terminal ledger sucessor
-> staging manifest
-> verificação independente
-> verification receipt
-> certificado pre-commit
-> relatório final
-> revalidação de hashes
-> staging literal de 48 paths
-> checks cached
-> commit atômico
```

Nenhuma etapa posterior pode reescrever uma entrada anterior. Se um hash mudar,
a cadeia para na primeira fase afetada; não se recalcula o hash apenas para
acompanhar a mutação.

## Gates de auditoria

P1 deve verificar:

1. parse JSON de todos os manifests, receipts, ledger, selo, superfícies e
   certificado P1295–P1297;
2. todos os hashes explicitamente pinados pelo certificado P1297;
3. hashes atuais dos três consumers e dois contratos P1297;
4. P1294/P1295/P1296 preservados nos valores registrados;
5. receipt P2 P1296 atual `be8aaf35...` e referência histórica
   `b958974c...` sem confundi-los;
6. payloads base64 das tentativas P6 anteriores decodificam para
   `f226cc3a...` e `a36470ab...`, conforme o receipt/certificado;
7. receipt final P1297 possui o tuple real medido em S3;
8. certificado pina o SHA real do receipt apesar do label divergente;
9. R1 tem score `1.0`, 4/4 mortos, zero survivor/Unknown e ordens iguais;
10. superfícies têm `111/99/12` e `115/104/11`, zero missing,
    unverified/Unknown, resultados iguais a P1293 e somente hash de binário
    diferente;
11. nenhum certificado/relatório final P1294/P1295/P1296 foi criado.

Falha em qualquer item produz audit receipt `BLOCKED`; P2–P6 não começam.

## Gates proporcionais P1298

Como nenhum byte de produto/L0/teste/superfície pode mudar, não repetir workspace,
stress, campanha mutante ou build. Os gates funcionais P1297 permanecem
evidência somente se todos os hashes correspondentes forem idênticos.

Executar:

- validação JSON/canonicalização declarada no audit receipt;
- verificador de hashes e referências P1298;
- `cargo fmt --all -- --check` apenas como confirmação não mutante;
- `crystalline-lint .`;
- V5, V15 e V26 individualmente com `--fail-on warning`;
- `crystalline-lint --fix-hashes --dry-run .` → `Nothing to fix`;
- `git diff --check`;
- antes do staging adicional, confirmar exatamente os 22 paths/patch hash
  congelados;
- depois do staging, `git diff --cached --check` e igualdade de conjuntos com
  a allowlist de 48 paths.

Se um hash produtivo divergir, bloquear em vez de “confirmar” executando testes
novos. Isso seria outra materialização e exige novo passo.

## Staging e commit

Após certificado e relatório P1298:

1. recalcular hashes de todos os 48 paths;
2. confirmar que os 22 paths já staged mantêm patch e nomes congelados;
3. adicionar literalmente Grupo B e Grupo C, sem glob e sem `git add .`;
4. comparar o conjunto sorted do índice com a allowlist sorted;
5. exigir exatamente 48 paths;
6. executar `git diff --cached --check`;
7. confirmar que não há alteração unstaged em nenhum dos 48 paths;
8. criar um único commit com mensagem:

```text
fix: seal causal watch recovery through step 1298
```

9. registrar na resposta de execução:
   - hash completo do commit;
   - commit subject e timestamp;
   - número de paths;
   - `git status --porcelain=v1` posterior;
   - confirmação de que nenhum write ocorreu depois do commit.

Se o commit falhar por hook/assinatura/configuração, preservar o índice e
reportar a causa. Uma repetição mecânica é permitida somente sem mudança de
bytes, allowlist ou mensagem; falha que exija editar conteúdo bloqueia P1298.

## Certificado P1298

O certificado deve pinar:

- passo, manifesto, audit receipt, terminal ledger e staging manifest;
- hashes originais do ledger/receipt/certificado P1297;
- reconciliação explícita `field_value_equal=false` e
  `semantic_authorization_consistent=true`;
- os três predecessores bloqueados e não absolvidos;
- código/L0/contratos/superfícies P1297 byte-preservados;
- receipt de verificação P1298;
- allowlist de 48 paths e mensagem de commit autorizada.

Claim máxima antes do commit:

```text
Fechamento append-only do grafo evidencial P1294–P1297 verificado para os
artefatos e hashes registrados; o certificado P1297 foi preservado com sua
divergência documental explicitada, sem reexecução de produto, sem absolvição
de predecessores e sem alegação de equivalência funcional geral.
```

Repetir literalmente:

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

O certificado P1298 autoriza o commit; não afirma conter seu próprio hash nem o
hash futuro do commit.

## Condições de bloqueio

Bloquear imediatamente se:

- qualquer predecessor, L0, Rust, teste, contrato ou superfície mudar;
- ledger/receipt/certificado P1297 for “corrigido” in-place;
- o label divergente for declarado igual ao campo real;
- uma tentativa P6 bloqueada for absolvida ou omitida;
- P1294/P1295/P1296 receber certificado/relatório retroativo;
- qualquer hash ou payload incorporado não reproduzir;
- algum path permanecer `Unknown`;
- o índice inicial não corresponder aos 22 paths congelados;
- staging usar glob, `git add .` ou incluir path fora da allowlist;
- o conjunto final não tiver exatamente 48 paths;
- houver alteração unstaged nos 48 paths antes do commit;
- lint, dry-run, formato, JSON, diff ou cached diff falhar;
- qualquer cargo test, build ou mutação for usado para absolver hash divergente;
- verificador editar entrada verificada;
- o commit exigir mudança de conteúdo depois do certificado.

## Critério de conclusão

P1298 conclui somente quando:

1. audit receipt reproduz todos os hashes e classifica A/B/C;
2. terminal ledger acrescenta P4→P5→P6→certificado sem alterar P1297;
3. reconciliação registra o tuple real, sem igualdade fictícia;
4. os 48 paths exatos estão no índice e passam cached check;
5. nenhuma entrada verificada possui alteração unstaged;
6. o commit atômico é criado com a mensagem autorizada;
7. o estado posterior e o hash do commit são reportados sem novo write.

Antes do commit:

```text
P1297_CERTIFICATE_PRESERVED_P1298_EVIDENCE_CLOSURE_READY_NOT_COMMITTED
```

Depois do commit:

```text
P1294_BLOCKED_P1295_BLOCKED_P1296_BLOCKED_P1297_CERTIFIED_P1298_EVIDENCE_CLOSED_COMMITTED
```
