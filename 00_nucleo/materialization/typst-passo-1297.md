# Passo 1297 — fechar P1296 por uma escada de redesenhos causais do `watch`

## Estado inicial e regra de continuidade

P1294, P1295 e P1296 permanecem corretamente `BLOCKED`. Nenhum deles recebe
certificado, relatório final ou absolvição retroativa. P1297 inicia uma cadeia
nova a partir dos bytes atuais e preserva todos os recibos vermelhos.

Regime: protocolo Tekt completo, segregado por capacidades, ordem e artefatos,
sem alegação de isolamento técnico de leitura no filesystem compartilhado.

Política de `Unknown`: `Unknown` nunca é sucesso, não recebe crédito e bloqueia
selo/certificado, salvo caso opaco criado explicitamente para provar a própria
classe `Unknown`.

Este passo autoriza uma **escada finita de três redesenhos distintos**. Somente
um redesenho fica ativo por vez. Falha semântica/discriminatória de R1 conduz a
R2; falha de R2 conduz a R3. Não se alterna entre variantes, não se executam os
três em paralelo e não se volta a um desenho já refutado.

Se R3 também falhar, P1297 bloqueia. “Continuar tentando” não autoriza loop
infinito, aumento de timeout, sleeps de prontidão, retry-until-pass ou ajuste do
oráculo ao mutante sobrevivente.

## Proveniência congelada

Medição em `2026-09-03T08:28:18,757994335-03:00`:

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`;
- commit: `docs: record blocked step 1294 verification`;
- data do commit: `2026-09-02T22:23:37-03:00`;
- working tree não commitida: 24 entradas em
  `git status --porcelain=v1`;
- SHA-256 do output exato de `git status --porcelain=v1`:
  `aa39f054caa0f1ee1196996e0a472e06148962ac2535cfa80ed0be2617ed09c5`;
- diff rastreado: `6 files changed, 388 insertions(+), 93 deletions(-)`;
- SHA-256 do output exato de `git diff HEAD --stat`:
  `d7552c82635cd0f81f6995b0454860eb98fca3e0f6fe7649ea5eb91bc3943582`.

### Entradas causais preservadas

| Artefato | SHA-256 atual |
|---|---|
| `00_nucleo/materialization/typst-passo-1295.md` | `e0328b02cecdfefb8dc7acc29d3faff50a2ce5ab1cf92d8cd2d77a0c6e0f0663` |
| `00_nucleo/materialization/typst-passo-1296.md` | `5a7f7c5d364fa6a373c1d411b8710131c68198bfeaaa4b19ec53fb17f7b8c00b` |
| `00_nucleo/diagnosticos/p1295-verification-receipt.json` | `06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573` |
| `00_nucleo/diagnosticos/p1296-manifest.json` | `7c9e2f75aab348ce434c293ea0b38259fc8c42b39360d6ca3b63a17491edd7d0` |
| `00_nucleo/diagnosticos/p1296-l0-receipt.md` | `224dd65d269f3a81b132f2679b2c373ab5bcf910d0ed21e94b554dbe761924a2` |
| `00_nucleo/diagnosticos/p1296-test-receipt.json` | `be8aaf3537bb068aab8eff1c5b656fe74c56cec8bd144f22e48a12b486d3e8a0` |
| `00_nucleo/diagnosticos/p1296-adversarial-plan.md` | `8b8a8435ff794f7d0d8ba5ecc0c0af2c8f88ac2b3133bc05dd8eaa2c2eb9ce6b` |
| `00_nucleo/diagnosticos/p1296-discrimination-receipt.json` | `51073959b26cc834f1f1dbe69b5c4e70b5e7f1c71d28282b7424e0306e641525` |

O recibo P3 de P1296 pina a revisão 1 de `p1296-test-receipt.json` com SHA
`b958974c3d9dd895a10cfd736eb64fd230d6a5fb98c284eb4aa906ba89b88768`.
Depois daquela campanha, P2 consumiu publicamente o survivor MO1, tentou a
revisão 2, restaurou o consumer byte a byte e substituiu seu próprio recibo
pelo SHA atual `be8aaf35...`. Essa sequência é evidência causal válida, mas
invalida qualquer continuação de selo baseada no recibo P2 antigo. Não é
permitido tratar o receipt P3 como gate verde atual.

### L0s e consumers atuais

| Papel | Caminho | SHA-256 atual |
|---|---|---|
| L0 L3 | `00_nucleo/prompts/shell/watch.md` | `f39a63e6a1e2025d3539b61b34b46ffa3a3b0ea2aa64711ed7699abf3e3f7332` |
| Consumer L3 | `03_infra/src/watch.rs` | `d543b1c32844b6f63085635ae34fa2beb01ba989c4ebed7f789bb5ae77e9ee41` |
| L0 L4 | `00_nucleo/prompts/wiring.md` | `81db17f53cdb344cfbee0a5aee230ae1e480cab1856618f846a5a43a9551e0b5` |
| Consumer L4 | `04_wiring/src/main.rs` | `e119a46479e1d2e7b3f0052df2b57ab5d31be573969bbe841dd8ee2d0e3e9c48` |
| L0 CLI | `00_nucleo/prompts/wiring/tests/cli.md` | `ac81dcf4404d505cd10db05964351b35f5ba0e9b959d83ef903fd4b34241a49b` |
| Consumer CLI | `04_wiring/tests/cli.rs` | `addf970325c6b9df11482b6d7f28bd1099cbb0d225080044268e3dc3a304206b` |
| Contrato P1295 congelado | `03_infra/tests/p1295_watch_contract.rs` | `8dae68ffc12f2708b6bbe3d503a7d0d90a2cb92681762ff5a992ca3f178c7abf` |

## Medição antes da decisão

### M1 — ordem produtiva P1295

Na fonte atual:

1. `04_wiring/src/main.rs:95` termina `run_compile_observed`;
2. `main.rs:96-100` normaliza dependências;
3. `main.rs:101` captura `WatchSnapshot`;
4. no erro, `main.rs:114-115` descarta staging;
5. `main.rs:119-122` espera usando o snapshot capturado.

Em L3, `03_infra/src/watch.rs:42-45` captura fingerprints imediatamente e
`wait_for_change_since` em `:49-59` não recaptura. A implementação produtiva
atual já possui a ordem desejada; a falha está em conseguir tornar essa ordem
discriminável e sincronizar o harness sem depender de tempo.

### M2 — primeira revisão P1296

P1296 R1 criou um sentinel no path de staging. O controle passou, MO2 e MO3
foram mortos, mas MO1 — `discard -> snapshot` — passou. O score observado foi
`2/3 = 0.6666666666666666`, com um survivor e zero `Unknown`.

O motivo medido é causal: o teste consulta a remoção a cada 50 ms. MO1 pode
remover o sentinel e capturar o snapshot antes do próximo poll. Quando o pai
observa a remoção e escreve `Recovered`, a captura mutante já terminou; logo a
recuperação funciona e o defeito de ordem sobrevive.

### M3 — segunda revisão P1296

P2 tentou uma dependência FIFO e barreiras test-only para tornar a janela
determinística. O primeiro controle positivo expirou durante
`recompilação por asset` após 20 s. MO1 nem foi executado porque o controle já
havia regredido. O candidato foi removido e `04_wiring/tests/cli.rs` voltou ao
SHA da revisão 1.

O receipt P2 registra duas revisões consumidas, zero ganho na revisão 2 e
`additional_local_revision_authorized = false`. Portanto o escopo test-only
P1296 está esgotado.

### M4 — estado documental que precisa ser reconciliado

O L0 atual `shell/watch.md` ainda declara no cabeçalho
`P1295 — contrato escrito; aguarda confirmação humana ADR-0127`, embora
`p1295-l0-gate-receipt.md` registre a confirmação e a implementação já exista.
`wiring.md` conserva a parada P1295 em linguagem presente, e
`wiring/tests/cli.md` ainda contém O1/P1296 como obrigação vigente apesar da
revisão 2 terminalmente bloqueada.

Esses textos são bytes causais, não autorização para fingir que o gate não
ocorreu nem para continuar O1. Antes de acrescentar R1, P1 deve reconciliá-los:
marcar P1295 como histórico confirmado/materializado porém não certificado, e
P1296/O1 como histórico refutado/supersedido por P1297. Não apagar medições,
refutadores ou receipts. O L0 final deve possuir uma única obrigação produtiva
ativa, sem alternativas simultaneamente normativas.

### Classificação ADR-0107/0108

Snapshot, remoção, polling, PID e ordem de chamadas são mecânica. Neste caso a
mecânica é o observável de processo: uma recuperação escrita antes do armamento
pode tornar-se o próprio baseline e nunca provocar nova compilação.

Inferência: um observador externo que só vê “staging ausente” não distingue
`snapshot -> unlink` de `unlink -> snapshot` quando ambos terminam antes do
próximo agendamento do observador. Refutador: um contrato público determinístico
que mate a inversão mínima sem novo sinal produtivo, timeout maior, sleep,
retry ou carga artificial. P1296 não produziu esse refutador e a tentativa FIFO
regrediu um positivo.

Conclusão: a classificação “somente test-only” de P1296 foi refutada. P1297
precisa redesenhar a fronteira produtiva que representa armamento/finalização.

## Invariantes comuns R1–R3

Todo redesenho admissível deve preservar:

1. fingerprints e I/O permanecem em L3;
2. L4 apenas compõe e nunca chama `std::fs` para implementar watch;
3. o baseline é capturado exatamente uma vez por iteração, depois da
   compilação e antes de qualquer publicação ou descarte;
4. a espera consome exatamente o baseline daquela iteração;
5. sucesso publica atomicamente; falha de rename é fatal e limpa staging;
6. erro de compilação descarta staging, preserva o último destino válido e
   continua aguardando recuperação;
7. conjunto transitivo, fallback para input e `crystalline_evict(10)` são
   preservados;
8. nenhuma prontidão depende de duração, carga, tamanho de arquivo ou chance de
   agendamento;
9. P1137 mantém liveness, asset observado, irrelevante filtrado, erro,
   preservação e recuperação;
10. P1294/P1295/P1296 permanecem vermelhos e byte a byte.

## Máquina de transição dos redesenhos

```text
R1_CAPABILITY
  PASS discriminatório -> selo R1 -> implementação final -> gates
  FAIL semântico        -> receipt vermelho R1 -> R2_TRANSACTION

R2_TRANSACTION
  PASS discriminatório -> selo R2 -> implementação final -> gates
  FAIL semântico        -> receipt vermelho R2 -> R3_ACK

R3_ACK
  PASS discriminatório -> selo R3 -> implementação final -> gates
  FAIL semântico        -> P1297_BLOCKED_REDESIGN_EXHAUSTED
```

Falha mecânica corrigível — erro de sintaxe, formatação ou fixture malformada
com causa nova e testemunha objetiva — admite uma correção focal dentro do
mesmo R. Survivor válido, `Unknown`, regressão de controle ou repetição do mesmo
`reason_code` muda imediatamente para o próximo R; não admite “só mais uma”
variação local.

Antes de cada transição:

- produzir receipt bloqueante do R encerrado;
- registrar vetor, score, custo, `reason_code` e refutador;
- restaurar na cópia de trabalho os hashes produtivos do baseline P1297;
- confirmar que nenhum arquivo do candidato falho entrou no índice;
- invalidar qualquer selo/manifesto dependente das entradas substituídas;
- atualizar o ledger canônico antes de autorar o L0 do próximo R.

## R1 — capacidade tipada de ciclo já armado

### Hipótese R1

Representar “snapshot já capturado” como uma capacidade que precisa existir
antes de publicar ou descartar torna a ordem inexprimível pelo consumer L4.

Atualizar primeiro `shell/watch.md` e `wiring.md` com contrato equivalente a:

```rust
pub struct ArmedWatch { /* contém somente WatchSnapshot já capturado */ }

pub fn arm(paths: &[PathBuf]) -> ArmedWatch;

impl ArmedWatch {
    pub fn publish(
        self,
        staging: &Path,
        destination: &Path,
    ) -> io::Result<WatchSnapshot>;

    pub fn abandon(self, staging: &Path) -> WatchSnapshot;
}
```

Requisitos normativos:

- `arm` captura todas as fingerprints imediatamente;
- `ArmedWatch` armazena somente o `WatchSnapshot` já materializado, nunca os
  paths crus que permitiriam captura lazy;
- `publish` e `abandon` consomem a capacidade e devolvem o mesmo snapshot;
- `publish` faz rename atômico; se falhar, limpa staging best-effort e propaga
  o erro original;
- `abandon` descarta staging best-effort sem tocar no destino;
- helpers crus de commit/discard deixam de ser chamáveis por L4;
- `wait_for_change_since` continua consumindo `WatchSnapshot`;
- `wait_for_change(paths, interval)` pode permanecer como compatibilidade;
- L4 não cria tipo próprio e não conhece fingerprints.

Forma L4 obrigatória:

```text
compile -> normalize -> arm
  success: ArmedWatch.publish
  error:   ArmedWatch.abandon
-> evict -> wait_for_change_since(snapshot)
```

### Contrato discriminatório R1

Criar owner 1:1 e consumer novos:

- `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md`;
- `03_infra/tests/p1297_watch_capability_contract.rs`.

O contrato deve usar deliberadamente um path válido simultaneamente como path
observado e staging:

1. cria o arquivo;
2. chama `arm([path])` enquanto ele existe;
3. chama `abandon(path)`;
4. entrega o snapshot retornado à espera;
5. exige detecção da remoção.

Isso não depende de o testador observar o intervalo entre operações: o baseline
correto contém `Some(fingerprint)` e o estado posterior é `None`. Captura lazy
após a remoção produz baseline `None` e não pode satisfazer o contrato.

Repetir o princípio no ramo `publish`, observando o desaparecimento do staging
após rename. Timeout existe somente como limite de falha do test runner; a
testemunha é a transição de estado já congelada.

Mutantes mínimos R1:

- `R1M1`: `ArmedWatch` guarda paths e captura somente dentro de `abandon` após
  remover staging;
- `R1M2`: `publish` faz rename antes de capturar o baseline;
- `R1M3`: L4 contorna a capacidade e finaliza antes de `arm`;
- `R1M4`: `wait_for_change_since` recaptura o baseline recebido.

R1 passa somente com score `1.0`, zero survivors, zero `Unknown`, controle
preservado e mesmas classificações nas ordens direta e inversa.

### Condição de transição R1 → R2

Ir para R2 se qualquer mutante válido sobreviver, a API tipada exigir paths
lazy, helpers crus precisarem continuar disponíveis a L4, ou algum controle
positivo regressar. Não reparar R1 adicionando sleeps, inspeção textual do
source ou mutante artificialmente bloqueado.

## R2 — transação única de finalização em L3

R2 só começa após receipt vermelho R1, restauração verificada e nova autoria
L0. Nenhum selo R1 permanece válido.

### Hipótese R2

Se a capacidade tipada for insuficiente, L3 recebe o resultado da compilação e
executa captura + finalização dentro de uma única operação pública, impedindo
que L4 intercale as duas ações.

Contrato equivalente:

```rust
pub enum WatchFinalization<'a> {
    Publish { staging: &'a Path, destination: &'a Path },
    Discard { staging: &'a Path },
}

pub fn finalize_after_snapshot(
    paths: &[PathBuf],
    action: WatchFinalization<'_>,
) -> io::Result<WatchSnapshot>;
```

Requisitos:

- a primeira ação interna é capturar o snapshot completo;
- somente depois o enum é executado;
- `Publish` mantém rename atômico, cleanup em erro e propagação do erro;
- `Discard` é best-effort e nunca remove destino;
- helpers crus de finalização são privados ao módulo;
- L4 escolhe apenas o enum a partir do exit code e recebe o snapshot pronto;
- nenhum callback arbitrário ou closure de L4 atravessa a fronteira L3.

Criar somente se R2 for ativado:

- `00_nucleo/prompts/infra/tests/p1297_watch_transaction_contract.md`;
- `03_infra/tests/p1297_watch_transaction_contract.rs`.

Mutantes mínimos R2:

- `R2M1`: `Discard` executado antes da captura;
- `R2M2`: `Publish` executado antes da captura;
- `R2M3`: L4 chama finalização fora da transação;
- `R2M4`: ramo de erro remove o destino válido;
- `R2M5`: espera usa snapshot novo em vez do retornado.

O contrato usa a sobreposição controlada path observado/staging para matar
R2M1/R2M2 sem observar o intervalo. Exigir score `1.0`, zero survivors e zero
`Unknown` nas duas ordens.

### Condição de transição R2 → R3

Ir para R3 se a transação ainda não matar inversão mínima, exigir lógica de
compilação em L3, expor filesystem a L4, regressar publicação/recuperação ou
produzir `Unknown`. Não transformar `WatchFinalization` em enum de pipeline nem
mover eval/export para L3 watch.

## R3 — confirmação causal explícita e opt-in

R3 é último recurso. Só começa após R1 e R2 possuírem receipts vermelhos e
restauração verificada.

### Hipótese R3

Se nem capacidade nem transação tornam a ordem discriminável, o processo deve
emitir uma confirmação explícita **depois** da captura e **antes** da
finalização. O harness aguarda esse evento, não duração nem desaparecimento
inferido.

O L0 deve escolher e fixar uma única forma de injeção antes do gate:

- observer L3 síncrono com implementação no-op normal; e
- canal opt-in de teste por path/handle explicitamente fornecido, ausente por
  defeito e omitido de help/uso público normal.

Requisitos:

1. evento `armed` contém nonce da fixture e número monotônico da iteração;
2. é emitido somente após snapshot completo;
3. é emitido antes de publish/discard;
4. falha no canal de teste é fatal para o teste e nunca convertida em sucesso;
5. sem opção/injeção, o produto não cria arquivos, não lê env adicional, não
   muda mensagens e não paga I/O extra além de uma chamada no-op comprovada;
6. o canal não transporta fingerprint, paths de dependência ou conteúdo;
7. P1137 escreve `Recovered` somente após receber o `armed` da iteração de
   erro com nonce/contador esperados;
8. canal é removido da fixture por RAII.

Se a forma escolhida exigir flag, campo de intent, trait público, env var ou
comportamento de release, o L0 deve dizer isso expressamente. Não esconder uma
superfície real sob o rótulo “test-only”.

Criar somente se R3 for ativado:

- `00_nucleo/prompts/infra/tests/p1297_watch_ack_contract.md`;
- `03_infra/tests/p1297_watch_ack_contract.rs`.

Mutantes mínimos R3:

- `R3M1`: emite ack antes do snapshot;
- `R3M2`: emite ack depois do descarte;
- `R3M3`: reutiliza nonce/contador da iteração anterior;
- `R3M4`: omite ack no erro;
- `R3M5`: ativa canal no comportamento normal sem opt-in;
- `R3M6`: recaptura snapshot depois do ack.

O adversário faz o callback/evento provocar uma única alteração observada no
ponto causal. Ack anterior à captura incorpora essa alteração ao baseline e é
`Violated`; ack posterior ou ausente falha na fase correta. Exigir score `1.0`,
zero survivors e zero `Unknown` nas duas ordens.

### Esgotamento R3

Qualquer survivor, regressão ou `Unknown` encerra P1297 como
`P1297_BLOCKED_REDESIGN_EXHAUSTED`. Não existe R4 implícito. Nova tentativa
exige novo passo, nova medição e decisão humana sobre a fronteira.

## Gate humano ADR-0127 por redesenho

R1, R2 e R3 alteram assinatura/API pública de L3; R1/R2 também mudam a forma
arquitetural de finalização e R3 pode criar superfície opt-in. Logo cada R ativo
segue obrigatoriamente:

1. atualizar primeiro `shell/watch.md`, `wiring.md` e, se o harness mudar,
   `wiring/tests/cli.md`;
2. reconciliar os estados documentais medidos em M4, sem apagar histórico;
3. declarar no L0 que o redesenho anterior falhou e qual receipt o refutou;
4. atualizar/individualizar o L0 do contrato externo daquele R;
5. ressellar somente headers dos consumers correspondentes;
6. publicar hashes e diff dos L0s em `p1297-rN-l0-gate-receipt.md`;
7. **PARAR antes de teste, mutante ou código**;
8. prosseguir somente após confirmação humana explícita daquele texto.

A autorização do usuário para “tentar as outras” autoriza a escada logística,
mas não substitui a confirmação ADR-0127 posterior aos bytes concretos de cada
L0. Uma confirmação de R1 não confirma antecipadamente R2/R3.

Estado durante a primeira parada:

```text
P1296_BLOCKED_P1297_R1_L0_AWAITING_HUMAN_GATE
```

## Isolamento dos candidatos e promoção

- contratos e mutantes são preparados a partir do L0 confirmado, antes do
  candidato produtivo;
- cada R é implementado primeiro em cópia/worktree temporário próprio, sem
  promover seus corpos ao repositório compartilhado;
- o adversário não lê saídas privadas do implementador;
- somente artefatos canônicos cruzam autoridades;
- candidato só é promovido ao repositório depois de RED válido, score `1.0`,
  selo daquele R e confirmação de que os hashes selados não mudaram;
- candidato falho é descartado na cópia; restauração é comprovada por hashes,
  nunca presumida;
- nenhuma ferramenta executa rollback destrutivo do working tree do usuário.

## Papéis segregados

### P0 — autor da intenção e ledger

Escreve este passo, manifesto inicial e ledger. Não escreve contrato, candidato,
mutante, selo ou veredito final.

### P1-RN — autor L0 do redesenho ativo

Recebe medições e receipt vermelho anterior. Escreve somente L0s/headers do R
ativo e seu gate receipt; para no ADR-0127.

### P2-RN — autor independente de contrato/oráculos

Recebe L0 confirmado e baseline, sem candidato. Escreve somente owner/consumer
de contrato daquele R e receipt RED. Não edita produto.

### P3-RN — adversário/calibrador

Recebe L0, contrato e baseline, sem solução privada. Produz mutantes em cópia
temporária, plano e receipt discriminatório. Não corrige contrato ou produto.

### P4-RN — selador

Sela somente se RED, score `1.0`, ordens, allowlists e gate humano passarem.
Em falha, não cria selo; a autoridade própria escreve receipt vermelho do R.

### P5-RN — implementador

Recebe apenas L0 confirmado e contrato selado. Escreve somente os consumers
produtivos/test-only permitidos pelo manifesto ativo e receipt de implementação.
Não altera contrato, mutantes, selo ou predecessores.

### P6 — verificador final

Recebe candidato promovido e entradas seladas. Não edita o que verifica.
Escreve superfícies, receipt final, certificado e relatório, nessa ordem.

## Artefatos previstos

Artefatos comuns:

1. `00_nucleo/diagnosticos/p1297-manifest.json`;
2. `00_nucleo/diagnosticos/p1297-redesign-ledger.json`;
3. `00_nucleo/diagnosticos/p1297-implementation-receipt.md`;
4. `00_nucleo/diagnosticos/p1297-surface-default.json`;
5. `00_nucleo/diagnosticos/p1297-surface-html.json`;
6. `00_nucleo/diagnosticos/p1297-verification-receipt.json`;
7. `00_nucleo/diagnosticos/p1297-certificate.json`;
8. `00_nucleo/diagnosticos/p1297-final-report.md`.

Para cada R realmente iniciado:

- `p1297-rN-l0-gate-receipt.md`;
- `p1297-rN-red-tests-receipt.json`;
- `p1297-rN-adversarial-plan.md`;
- `p1297-rN-discrimination-receipt.json`;
- `p1297-rN-contract-seal.json`, somente se passar; ou
- `p1297-rN-blocked-receipt.json`, somente se falhar.

Não criar artefatos de R2/R3 antes da transição que os ativa. Não criar
certificado, relatório final ou superfícies P1294/P1295/P1296.

## Budget e disciplina de calibração

- um survivor válido, `Unknown` inesperado ou regressão positiva fecha o R
  semanticamente e move à próxima arquitetura;
- uma correção mecânica focal por R é permitida somente com `reason_code` novo;
- repetir o mesmo vetor/causa duas vezes bloqueia imediatamente, mesmo dentro
  do limite mecânico;
- executar primeiro controle + mutantes do R ativo;
- ordens, stress e corpus completo só depois do recorte focal verde;
- registrar duração e delta discriminatório de toda execução;
- falha registrada nunca é apagada por verde posterior;
- alterar timeout, carga, número de polls ou inserir sleep não conta como
  redesenho nem ganho.

## Gates finais do redesenho vencedor

### Focais e adversariais

- contrato do R vencedor: duas execuções verdes;
- todos os mutantes do R vencedor: score `1.0`, zero survivor/`Unknown`;
- ordem direta e inversa: mesmo vetor;
- contrato P1295 congelado: `6/6` duas vezes;
- P1292 protegido: `11/11` nas duas ordens;
- P1293 protegido: `11/11` nas duas ordens;
- P1137: duas execuções focais e depois `20/20` consecutivas;
- suíte CLI integral: duas execuções verdes;
- não executar loop até passar.

### Workspace e arquitetura

- `cargo test --workspace -q` duas vezes;
- `cargo fmt --all -- --check`;
- `crystalline-lint .`;
- V3, V4, V5, V7, V13, V14, V15 e V26 individualmente com
  `--fail-on warning`;
- `crystalline-lint --fix-hashes --dry-run .` imprime `Nothing to fix`;
- `git diff --check`;
- índice vazio antes do staging e `git diff --cached --check` depois da
  allowlist exata resolvida.

### Build, proveniência e superfícies

1. registrar HEAD, status, diff stat, `Cargo.lock`, `rustc -Vv`, `cargo -V` e
   inventário de fontes no instante do build;
2. registrar o `TYPST_COMMIT_SHA` efetivamente usado;
3. executar `cargo build --release` e pinar SHA-256 do binário real;
4. confirmar `typst 0.15.1` e prefixo coerente com o SHA declarado;
5. regenerar superfícies P1297 default e HTML pelos comandos canônicos P1293;
6. exigir `111/99/12` e `115/104/11`, zero missing, unverified e `Unknown`;
7. comparar todos os objetos `results` com P1293: zero divergências;
8. permitir contra P1293 somente diferença em
   `$.binaries.crystalline.sha256`;
9. confirmar que o hash interno aponta ao binário P1297 efetivamente usado.

Hash bruto de binário é proveniência mecânica, não igualdade semântica entre
HEADs distintos.

## Preservação e staging

- preservar byte a byte P1294/P1295/P1296 e seus recibos vermelhos;
- preservar o receipt P2 final P1296 `be8aaf35...` e a evidência histórica da
  revisão 1 `b958974c...` referenciada pelos receipts;
- não criar nem completar selos/certificados predecessores;
- preservar inventário P1293 `74/74` e evidência score `1.0` sem reutilizá-la
  como prova do novo contrato;
- o manifesto P1297 resolve antes de writes a allowlist exata de cada R;
- stage somente este passo, artefatos comuns, receipts dos Rs iniciados, L0s,
  contracts e consumers do R vencedor;
- candidatos falhos temporários nunca entram no índice;
- qualquer path fora da allowlist bloqueia staging.

## Certificado P1297

O certificado deve pinar:

- este passo, manifesto e ledger de transições;
- receipts vermelhos de cada R tentado e selo do R vencedor;
- confirmação humana posterior ao L0 do R vencedor;
- contrato/oráculos, campanha adversarial e implementação vencedores;
- P1294/P1295/P1296 preservados e não absolvidos;
- HEAD, inventário, toolchain e binário efetivamente testado;
- stress, suítes, lints, superfícies e receipt final congelado.

Claim máxima:

```text
Armamento e finalização causal do ciclo watch verificados pelo redesenho
vencedor registrado, para os artefatos, versões e observáveis pinados, sem
isolamento técnico de leitura e sem alegação de equivalência funcional geral.
```

Repetir literalmente:

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

O certificado é terminal e não é inserido retroativamente em manifesto, ledger
ou selo.

## Condições de bloqueio

Bloquear se:

- algum R começar sem receipt vermelho/restauração do anterior;
- o gate humano daquele R não ocorrer depois do L0 e antes do código;
- duas arquiteturas forem materializadas simultaneamente;
- contrato/oráculo ler o candidato antes de ser congelado;
- implementador editar contrato, mutantes, manifesto ou selo;
- L4 fizer I/O de watch diretamente;
- baseline for capturado depois de publish/discard;
- houver captura lazy, recaptura ou troca do snapshot na espera;
- sentinel, FIFO, sleep, timeout maior, carga ou retry reaparecer como prova de
  prontidão;
- mutante válido sobreviver, positivo regredir ou `Unknown` receber crédito;
- candidato falho tocar a árvore promovida ou o índice;
- P1294/P1295/P1296 mudar ou for absolvido;
- qualquer stress, suíte, lint, dry-run, diff, build ou superfície falhar;
- hash de binário não corresponder ao executável usado;
- verificador editar entrada verificada.

## Critério de conclusão

P1297 conclui somente quando exatamente um R:

1. possui L0 confirmado e contrato RED independente;
2. mata todos os seus mutantes nas duas ordens;
3. possui selo anterior à implementação promovida;
4. mantém invariantes comuns e predecessors vermelhos;
5. passa P1137 `20/20`, CLI/workspace `2/2` e gates arquiteturais;
6. preserva superfícies semanticamente, com proveniência do binário atual;
7. recebe receipt independente `PASS_READY_FOR_CERTIFICATE`;
8. produz certificado e relatório terminais P1297.

Até lá:

```text
P1294_BLOCKED_P1295_BLOCKED_P1296_BLOCKED_P1297_NOT_CERTIFIED
```
