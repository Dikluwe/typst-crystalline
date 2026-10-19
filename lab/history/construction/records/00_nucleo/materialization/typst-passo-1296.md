# Passo 1296 — fechar P1295: armamento observável da recuperação sem `sleep`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado inicial e continuidade

P1295 permanece terminalmente `BLOCKED`, sem certificado, relatório final,
superfícies ou staging. Este passo cria uma cadeia nova; não reabre, sobrescreve
nem absolve o recibo vermelho P1295.

Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
alegação de isolamento técnico de leitura no filesystem compartilhado.

Política de `Unknown`: `Unknown` nunca é sucesso, não recebe crédito e bloqueia
selo ou certificado, salvo caso opaco criado explicitamente para validar a
própria classe `Unknown`.

## Proveniência congelada

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`
- Commit: `docs: record blocked step 1294 verification`
- Estado: working tree não commitida com a cadeia P1295 inteira preservada.
- Medição: `2026-09-03T00:23:46-03:00`.
- identificador administrativo de `git diff HEAD --stat`:
  [dado administrativo removido].
- identificador administrativo de `git status --porcelain=v1`:
  [dado administrativo removido].

Entradas causais P1295:

| Artefato | identificador administrativo |
|---|---|
| `00_nucleo/materialization/typst-passo-1295.md` | [dado administrativo removido] |
| `00_nucleo/diagnosticos/p1295-manifest.json` | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |
| `00_nucleo/diagnosticos/p1295-red-tests-receipt.json` | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |
| `00_nucleo/diagnosticos/p1295-discrimination-receipt.json` | [dado administrativo removido] |
| `00_nucleo/diagnosticos/p1295-contract-seal.json` | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |

Entradas L0/código afetadas:

| Papel | Caminho | identificador administrativo inicial |
|---|---|---|
| L0 test-only | `00_nucleo/prompts/wiring/tests/cli.md` | [dado administrativo removido] |
| Consumer test-only | `04_wiring/tests/cli.rs` | [dado administrativo removido] |
| L4 produto congelado | `04_wiring/src/main.rs` | [dado administrativo removido] |
| L3 produto congelado | `03_infra/src/watch.rs` | [dado administrativo removido] |
| Contrato externo congelado | `03_infra/tests/p1295_watch_contract.rs` | [dado administrativo removido] |

Evidência de falha:

- `/tmp/p1295-p6-cli-1.log`, identificador administrativo
  [dado administrativo removido];
- suíte CLI integral P1295 run 1: `70/71`, exit `101`;
- único failure: `p1137_watch_dependencias_recuperacao_e_filtro`;
- mensagem: `timeout durante recuperação após 20s`;
- stress isolado anterior: `20/20` em `43.288 s`;
- run 2 posterior: `71/71`, explicitamente não absolvente.

## Medição antes da decisão

### Causa O — o observador da recuperação ainda usa tempo como armamento

Na fonte congelada, `04_wiring/tests/cli.rs:166-170` escreve o source inválido,
dorme `500 ms`, confirma somente que o child e o artefato anterior sobrevivem e
então escreve `Recovered` em `:172`. Não há observável que prove que a iteração
de erro terminou a compilação e capturou seu snapshot antes da recuperação.

No produto P1295, `04_wiring/src/main.rs:95` termina
`run_compile_observed`; `:96-101` normaliza dependências e captura o snapshot;
somente depois `:114-115` descarta staging. Logo, se a recuperação for escrita
durante a compilação de erro ou antes de `:101`, ela pode virar o próprio
baseline e `wait_for_change_since` aguarda uma mudança que nunca ocorrerá.

A suíte integral expôs exatamente essa possibilidade sob carga: o focal passou
20 vezes isolado, mas a primeira suíte integral expirou na fase de recuperação.
A inferência de que `500 ms` basta seria refutada por qualquer execução mais
lenta; a evidência P1295 já a refutou.

### Observável já existente — remoção do staging depois do snapshot

O staging tem identidade determinística no próprio processo:

```text
.<destination-file-name>.crystalline-watch-<child-pid>.tmp
```

O teste conhece `destination` e `Child::id()`. Se criar nesse path um sentinel
antes de escrever o source inválido, a compilação com erro não publica novo
output e `discard_output` remove o sentinel. Pela ordem P1295, observar sua
remoção prova que o snapshot da iteração de erro já foi capturado. Só então o
teste escreve `Recovered` uma vez.

Classificação ADR-0107/0108: nome do staging, PID e remoção são mecânica; aqui a
mecânica é deliberadamente o observável causal do harness. Semântica Typst,
bytes PDF e API pública não mudam. Inferência: o sentinel fecha somente a janela
do observador, sem alteração de produto. Refutadores: o produto escrever staging
antes de concluir uma compilação com erro; `discard_output` ocorrer antes do
snapshot; ou o path real não ser derivável de `destination + child.id()`. Qualquer
refutador bloqueia e exige nova decisão, não timeout maior.

## Decisão test-only O1

Atualizar primeiro o L0 proprietário `wiring/tests/cli.md` e depois somente o
consumer `04_wiring/tests/cli.rs`:

1. derivar o path exato de staging com o nome do output e `child.id()`;
2. imediatamente antes do source inválido, escrever um sentinel único nesse
   path;
3. escrever o source inválido uma vez;
4. aguardar, com o mesmo limite máximo de 20 segundos e diagnóstico de fase
   `armamento após erro transitório`, até o sentinel deixar de existir;
5. durante a espera, verificar liveness por `try_wait`;
6. somente após a remoção confirmar que o artefato publicado ainda é o segundo;
7. escrever `Recovered` uma vez e aguardar a publicação recuperada;
8. remover o `sleep(500 ms)`; não adicionar outro sleep de prontidão, timeout,
   retry-until-pass, segunda recuperação ou loop corretivo;
9. preservar asset alterado uma vez, filtro de irrelevante, artefato anterior,
   RAII do child e limpeza da fixture.

O sentinel pertence à fixture test-only, vive no mesmo diretório temporário e é
removido pelo produto ou pela limpeza RAII. Não vira arquivo, flag, mensagem ou
API de produção.

## Classificação ADR-0127

Fluxo contínuo. O1 corrige somente a sincronização do consumer de teste; não
altera assinatura pública, comportamento por defeito, fase de pipeline ou
compatibilidade. L0 é editado primeiro e ressellado, sem novo gate humano. Se a
execução revelar necessidade de mudar L3/L4, esta classificação é refutada e o
passo para antes de qualquer write produtivo.

## Artefatos P1296 previstos

1. `00_nucleo/diagnosticos/p1296-manifest.json`
2. relatório consolidado da limpeza
3. relatório consolidado da limpeza
4. relatório consolidado da limpeza
5. `00_nucleo/diagnosticos/p1296-discrimination-receipt.json`
6. `00_nucleo/diagnosticos/p1296-contract-seal.json`
7. `00_nucleo/diagnosticos/p1296-surface-default.json`
8. `00_nucleo/diagnosticos/p1296-surface-html.json`
9. `00_nucleo/diagnosticos/p1296-verification-receipt.json`
10. `00_nucleo/diagnosticos/p1296-certificate.json`
11. `00_nucleo/diagnosticos/p1296-final-report.md`

É proibido criar certificado, relatório ou superfícies P1295.

## Papéis segregados

### P1 — autor da obrigação/L0

Pode escrever somente este passo, `p1296-manifest.json`, a cláusula P1296 no
L0 `wiring/tests/cli.md`, seu resselo mecânico no header de
`04_wiring/tests/cli.rs` e relatório consolidado da limpeza. Não escreve corpo de teste,
mutante, selo ou veredito.

### P2 — materializador independente do observador

Recebe este passo, L0 ressellado, baseline e falha P1295. Não recebe plano ou
mutantes P3. Pode escrever somente o corpo P1137 em
`04_wiring/tests/cli.rs` e relatório consolidado da limpeza. Não altera produto, L0,
headers, contratos externos ou evidência P1295.

### P3 — adversário/calibrador

Recebe passo, L0, consumer P2 e produtos congelados, sem saídas privadas de
implementação além do artefato canônico P2. Trabalha em cópias `/tmp` e escreve
somente plano e recibo discriminatório P1296.

Mutantes mínimos:

- `MO1`: no caminho de erro, remover staging antes de capturar snapshot;
- `MO2`: no caminho de erro, omitir `discard_output`, mantendo o sentinel;
- `MO3`: restaurar o observador temporal antigo e, somente no mutante de
  calibração, atrasar a captura de erro para além de 500 ms, expondo a janela
  sem converter esse atraso em requisito de produção.

Controle atual deve ser `Preserved`; MO1–MO3 devem ser `Violated` com testemunha
de fase específica. Exigir score `1.0`, zero survivors e zero `Unknown`, em
ordem direta e inversa.

### P4 — selador

Recebe manifesto, L0, consumer, recibo P2 e campanha P3. Confirma hashes,
allowlists e ausência de write produtivo. Escreve somente
`p1296-contract-seal.json`.

### P5 — não aplicável

Não há implementação produtiva P1296. Os corpos L3/L4 P1295 são entradas
congeladas e não podem mudar. O único consumer materializado é test-only em P2.

### P6 — verificador final

Recebe entradas seladas e não edita nada verificado. Pode escrever apenas as
duas superfícies, recibo, certificado e relatório P1296, nessa ordem causal.

## Budget de calibração

- Máximo de duas revisões por `reason_code`.
- Primeiro somente teste focal, controle e MO1–MO3.
- Ordens, stress e corpus completo somente após o recorte focal verde.
- Duas revisões consecutivas sem delta interrompem a execução.
- Nenhum aumento de timeout, retry-until-pass ou classificação de flake conta
  como ganho discriminatório.

## Gates finais

### Focais e ordem

- P1137 corrigido: duas execuções isoladas verdes.
- Contrato P1295 externo: `6/6` duas vezes.
- P1292 protegido: `11/11` e P1293 protegido: `11/11`, nas ordens direta e
  inversa.
- P1137: 20 execuções isoladas consecutivas, com duração individual e total.
- Suíte CLI integral: duas execuções verdes; qualquer falha bloqueia e não é
  absolvida por execução posterior.

### Workspace e arquitetura

- `cargo test --workspace -q` duas vezes.
- `cargo fmt --all -- --check`.
- `crystalline-lint .`.
- V3, V4, V5, V7, V13, V14, V15 e V26 individualmente com
  `--fail-on warning`.
- `crystalline-lint --fix-hashes --dry-run .` deve imprimir `Nothing to fix`.
- `git diff --check` e, após staging exato, `git diff --cached --check`.

### Build e superfícies

1. Registrar HEAD, dirty stat/hash, `Cargo.lock`, `rustc -Vv`, `cargo -V` e
   inventário de fontes.
2. Executar release com
   `TYPST_COMMIT_SHA=76fb7336311bdb6497456ab5fdc0a8ce355ff39b`.
3. Pinar identificador administrativo do binário usado e confirmar
   `typst 0.15.1 (76fb7336)`.
4. Regenerar superfícies P1296 default/HTML pelos comandos canônicos P1293.
5. Exigir `111/99/12` e `115/104/11`, zero missing, unverified e `Unknown`.
6. Comparar todos os objetos `results` com P1293: zero divergências.
7. Permitir contra P1293 somente diferença de
   `$.binaries.crystalline.sha256`; qualquer outra diferença bloqueia.

### Preservação

- P1294 e P1295 permanecem byte a byte, ambos com recibos vermelhos.
- Nenhum certificado, relatório final ou superfície P1294/P1295 é criado.
- Evidência/inventário P1293 permanece com cardinalidade registrada `74/74` e
  mutation score `1.0`, sem reexecutar mutantes P1293 contra produto.
- L3 `watch.rs`, L4 `main.rs` e contrato externo P1295 permanecem nos hashes
  congelados deste passo.

## Staging exato

Antes do staging, o índice deve estar vazio. Após todos os gates, stage somente
a cadeia P1294/P1295 já prevista no passo anterior mais:

- `00_nucleo/materialization/typst-passo-1296.md`;
- `00_nucleo/prompts/wiring/tests/cli.md`;
- `04_wiring/tests/cli.rs`;
- todos os onze artefatos P1296 listados acima.

Nenhum path fora da allowlist pode entrar no índice.

## Certificado P1296

O certificado pina passo, manifesto, L0/consumer test-only, recibos P2/P3,
selo, produtos P1295 congelados, recibo vermelho P1295, HEAD, inventário,
binário, testes, superfícies e recibo final P1296.

Claim máxima:

```text
Correção do armamento observável da recuperação do watch e continuidade
evidencial P1295 verificadas para os artefatos, versões e observáveis
registrados, sem isolamento técnico de leitura e sem alegação de equivalência
funcional geral.
```

Repetir literalmente:

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

O certificado é terminal e não retroage sobre manifestos, selos ou recibos
vermelhos anteriores.

## Condições de bloqueio

Bloquear imediatamente se:

- qualquer produto L3/L4 ou contrato externo P1295 mudar;
- P1294/P1295 for reescrito ou absolvido;
- o sentinel for substituído por sleep, timeout maior ou retry;
- o sentinel desaparecer antes do snapshot no controle;
- MO1–MO3 sobreviver, produzir `Unknown` ou depender de testemunha não causal;
- qualquer execução focal, stress, CLI ou workspace falhar;
- V5/V15/V26, dry-run, formato, diff ou superfície falhar;
- o hash do binário não corresponder ao executável usado;
- o verificador editar uma entrada ou stage fora da allowlist.

## Critério de conclusão

P1296 conclui somente com O1 ressellado, controle/mutantes discriminatórios,
produto P1295 intocado, P1137 `20/20`, CLI/workspace `2/2`, superfícies
semanticamente idênticas, receipt `PASS_READY_FOR_CERTIFICATE`, certificado e
relatório P1296 terminais.

Até lá:

```text
P1295_BLOCKED_P1296_NOT_CERTIFIED
```
