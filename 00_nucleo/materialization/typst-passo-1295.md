# Passo 1295 — fechar P1294: watch armado antes da publicação e proveniência de build vinculada

## Estado inicial

O P1294 foi executado até S3 e terminou corretamente como `BLOCKED`, sem
certificado. Este passo preserva esse recibo vermelho e cria uma nova cadeia;
não sobrescreve nem completa retroativamente o P1294.

Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
alegação de isolamento técnico de leitura no filesystem compartilhado.

Política de `Unknown`: `Unknown` nunca é sucesso, não recebe crédito e bloqueia
o selo ou certificado, salvo caso opaco criado explicitamente para validar a
própria classe `Unknown`.

## Proveniência congelada

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`
- Commit: `docs: record blocked step 1294 verification`
- Data do commit: `2026-09-02T22:23:37-03:00`
- Estado adicional conhecido: um arquivo não rastreado,
  `00_nucleo/diagnosticos/p1294-blocked-report.md`, SHA-256
  `b87dfe26f22bf46dd20e80a568a34b8f9da26138616932b3a84e6e522a8f959b`.
- O relatório não faz parte do selo P1294, mas deve ser preservado e incluído
  explicitamente na proveniência P1295; não pode ser confundido com
  `p1294-final-report.md`.

Entradas P1294:

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1294.md` | `10d6393ad9615f0ed38bc7ad8192d167a5bf91388e52cf38a1acb4271b1cc2e4` |
| `00_nucleo/diagnosticos/p1294-sanitization-manifest.json` | `0d3f24c9ac3f5691d4f07c1658440b58134bb9e94fd5d7e47f42ff84bedf7c41` |
| `00_nucleo/diagnosticos/p1294-terminal-seal.json` | `352ad6184b4c25c3ec5136ff56cca7ec8600885f69f89d248506fa698d4546a0` |
| `00_nucleo/diagnosticos/p1294-verification-receipt.json` | `0d31aa9b1e31440552c1d52f571923b40d68f7c65027bd093417c906c04535d1` |

O bloco `$.sanitized_terminal` do selo P1294 deve continuar a recalcular, pelo
algoritmo Python declarado, para
`6238a84f546d7b15c18b64f6a9402008d2b4c59d719f189ef6f828aaf21d8868`.
O recibo P1294 deve continuar com `verdict = BLOCKED`,
`certificate_authorized = false` e `report_authorized = false`.

Entradas de produto e L0:

| Papel | Caminho | SHA-256 inicial |
|---|---|---|
| L0 L3 watch | `00_nucleo/prompts/shell/watch.md` | `a40e3a6214da0cbc79b1cdd6fd6ad7305704e2054deae915348ff6acb4bb091d` |
| Consumer L3 | `03_infra/src/watch.rs` | `33c74822afe34e0eb70f6b211a77c86231e2cf69c52e98cd1ab2687d7d36e2a9` |
| L0 L4 | `00_nucleo/prompts/wiring.md` | `539815d1e892117baec6e5e8e51e37a155dadcaa682e3349c11cc66dd52f623c` |
| Consumer L4 | `04_wiring/src/main.rs` | `ca65db98f911f03c7e1772802dafc8003e50c0c3cfefcef7cc2249549c2004da` |
| L0 suíte CLI | `00_nucleo/prompts/wiring/tests/cli.md` | `7f48645d50982a0adc2bdeabbc5b62b40390da795d8ee9f2caca4c793e351e7b` |
| Consumer suíte CLI | `04_wiring/tests/cli.rs` | `6a0075c7021f01de3888b1e50f40ee35c8e03a4d507f90b62ae2524e9aafaaca` |
| Build script congelado | `02_shell/build.rs` | `8fa42b2aecdaa6d8cbd31dfcfe901cef27474ab14cc42c0c05fe27d20fe175b5` |
| L0 CLI congelado | `00_nucleo/prompts/shell/cli.md` | `a2e050633516ae486a6a00968ee8e20a28990c9006582a4b82a820a0b254f30f` |

## Medição antes da decisão

### Causa W — o timeout expõe uma janela real do produto

Medição na fonte vigente:

1. `04_wiring/src/main.rs:95` compila e recebe as dependências.
2. `main.rs:97` publica o staging no destino.
3. Somente em `main.rs:117` chama `wait_for_change`.
4. `03_infra/src/watch.rs:38-41` captura o baseline apenas dentro dessa chamada.
5. O teste `04_wiring/tests/cli.rs:117` observa `output.exists()` e pode alterar
   o asset antes de L3 executar a captura.

Logo, aumentar o timeout, adicionar `sleep` ao teste ou repetir até passar
mascararia uma mudança legitimamente perdida entre publicação e armamento.
A hipótese seria refutada se a implementação demonstrasse que o baseline já
existia antes de `commit_output`; a fonte atual mostra o contrário.

O diretório temporário do teste também usa apenas PID e não remove resíduo antes
de `create_dir_all`, criando uma segunda fonte de falso estado inicial após
execução abortada ou reutilização de PID.

### Causa B — o hash bruto do binário foi usado como igualdade semântica

O P1294 exigiu o hash P1293 `c527b411...`, medido sobre HEAD `7dd25ff0...` com
working tree não commitada. O rebuild sobre `5079a0cd...` produziu
`10fd9cf488be69cfc35796f4535e3efab71a970be874d5bbfee194e7c4460151` e
`typst 0.15.1 (5079a0cd)`.

`02_shell/build.rs:21-35` captura `TYPST_COMMIT_SHA`; portanto mudar o HEAD muda
legitimamente os bytes do binário. As superfícies regeneradas tiveram zero
divergências nos objetos de resultado e diferiram somente em
`$.binaries.crystalline.sha256`.

Pela ADR-0107, igualdade de bytes do executável é mecânica; a proveniência do
binário continua obrigatória, mas não pode ser tratada como paridade semântica
entre commits distintos.

## Decisão arquitetural

### W1 — snapshot opaco em L3

Atualizar o L0 `shell/watch.md` para possuir exclusivamente
`03_infra/src/watch.rs`, removendo a descrição antiga de múltiplos ficheiros
alvo. O compartilhamento de semântica entre L3, L4 e teste permanece nos L0s
proprietários e, se necessário, em Núcleo Tekt; nunca por ownership `1:N`.

L3 passa a expor contrato público aditivo equivalente a:

```rust
pub struct WatchSnapshot { /* campos privados */ }

pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot;

pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration);
```

Requisitos:

- `WatchSnapshot` é opaco fora de L3, não expõe `Fingerprint` nem filesystem;
- `snapshot` captura path e fingerprint uma única vez;
- `wait_for_change_since` consome o snapshot recebido e não recaptura baseline;
- criação, alteração, remoção e conteúdo de mesmo tamanho continuam detectáveis;
- `wait_for_change(paths, interval)` pode permanecer como compatibilidade e
  delegar para `snapshot` + `wait_for_change_since`;
- nenhum relógio ou I/O cru atravessa L1.

### W2 — armamento antes de publicar

Atualizar `wiring.md`: após `run_compile_observed` produzir dependências, L4
normaliza fallback para o input e cria o `WatchSnapshot` **antes** de tornar o
resultado da compilação observável em `destination`.

Ordem obrigatória em iteração bem-sucedida:

```text
compile staging + dependencies
-> normalize dependency set
-> snapshot(dependencies)
-> commit_output(staging, destination)
-> evict
-> wait_for_change_since(snapshot)
```

Em erro de compilação, o último artefato permanece intacto; o snapshot ainda é
criado antes de descartar staging e aguardar recuperação. Erro de publicação
continua fatal. Não mover fingerprint, polling ou filesystem para L4.

### W3 — observador CLI sem sleeps de prontidão

Atualizar `wiring/tests/cli.md` antes do teste:

- remover o diretório P1137 residual antes de recriá-lo;
- distinguir as fases nos diagnósticos de timeout;
- verificar que o child permanece vivo;
- tratar a primeira publicação nova como sinal de que o snapshot já foi
  capturado, conforme W2;
- alterar o asset uma única vez depois da publicação e exigir recompilação;
- preservar teste de ficheiro irrelevante, erro transitório, último artefato e
  recuperação;
- proibir aumento de timeout e `sleep` adicional como correção da race.

### B1 — testemunha de build, não igualdade cruzada

Não alterar `02_shell/build.rs`, `02_shell/src/cli.rs` ou `shell/cli.md`.

Cada build verificado recebe um registro com:

- HEAD completo e prefixo esperado no `--version`;
- estado dirty e hash de `git diff HEAD --stat`;
- SHA-256 de `Cargo.lock`, versões `rustc -Vv` e `cargo -V`;
- comando e `TYPST_COMMIT_SHA` efetivo;
- SHA-256 bruto do binário produzido;
- SHA-256 do inventário de fontes que o legitima.

O hash bruto identifica aquele artefato e é pinado no certificado. Ele não é
comparado ao hash de executável construído em outro HEAD.

Superfícies antigas e novas são comparadas em dois eixos:

1. semântico: todos os objetos `results` e suas classificações devem coincidir;
2. proveniência: o campo `binaries.crystalline.sha256` deve coincidir com o
   binário efetivamente usado naquela execução, podendo diferir do P1293.

Não exigir igualdade byte a byte de JSONs que incorporam hashes de binários
legitimamente diferentes. Qualquer outra diferença de caminho JSON continua
bloqueante.

## Gate humano obrigatório ADR-0127

W1 adiciona API pública entre L3 e L4 e W2 corrige comportamento persistente do
comando `watch`. Portanto:

1. o autor L0 escreve primeiro `shell/watch.md`, `wiring.md` e
   `wiring/tests/cli.md`;
2. ressela os três consumers apenas após o texto estar completo;
3. publica hashes e diff do L0;
4. **PARA** antes de escrever teste, oráculo ou código;
5. só prossegue depois de confirmação humana explícita sobre W1/W2/W3/B1.

Sem essa confirmação, o estado é `P1295_L0_AUTHORED_AWAITING_HUMAN_GATE`.

## Artefatos novos previstos

1. `00_nucleo/diagnosticos/p1295-manifest.json`
2. `00_nucleo/diagnosticos/p1295-l0-gate-receipt.md`
3. `00_nucleo/prompts/infra/tests/p1295_watch_contract.md`
4. `03_infra/tests/p1295_watch_contract.rs`
5. `00_nucleo/diagnosticos/p1295-red-tests-receipt.json`
6. `00_nucleo/diagnosticos/p1295-adversarial-plan.md`
7. `00_nucleo/diagnosticos/p1295-discrimination-receipt.json`
8. `00_nucleo/diagnosticos/p1295-contract-seal.json`
9. `00_nucleo/diagnosticos/p1295-implementation-receipt.md`
10. `00_nucleo/diagnosticos/p1295-surface-default.json`
11. `00_nucleo/diagnosticos/p1295-surface-html.json`
12. `00_nucleo/diagnosticos/p1295-verification-receipt.json`
13. `00_nucleo/diagnosticos/p1295-certificate.json`
14. `00_nucleo/diagnosticos/p1295-final-report.md`

É proibido criar `p1294-certificate.json` ou `p1294-final-report.md`.

## Papéis segregados

### P1 — autor da obrigação e L0

Pode ler medições, ADRs e L0s vigentes. Pode escrever somente:

- os três L0s W1/W2/W3;
- headers de seus três consumers, exclusivamente após L0 completo;
- `p1295-manifest.json` e `p1295-l0-gate-receipt.md`.

Não escreve corpo de Rust, teste, oráculo, mutante, selo ou veredito. Para no
gate humano.

### P2 — autor independente do contrato e oráculo

Começa somente após confirmação humana. Recebe L0s confirmados e baseline, sem
ler implementação candidata. Pode escrever:

- o novo Prompt L0 1:1 do oráculo;
- `03_infra/tests/p1295_watch_contract.rs`;
- ajustes P1137 estritamente test-only em `04_wiring/tests/cli.rs`;
- `p1295-red-tests-receipt.json`.

O RED deve ser determinístico no contrato L3 ausente. A flake histórica é
evidência adicional, não substituto do RED.

### P3 — adversário e calibrador

Recebe L0s, contrato e baseline, sem solução candidata. Escreve somente plano,
runner temporário e recibo discriminatório. Não corrige produto ou testes.

Mutantes válidos mínimos:

- `MW1`: L4 captura snapshot depois de publicar o output;
- `MW2`: `wait_for_change_since` ignora o snapshot e recaptura baseline;
- `MW3`: o conjunto observado descarta o asset e mantém somente o input.

Cada mutante deve ser morto com testemunha específica. Exigir score `1.0`, zero
survivors e zero `Unknown`. Repetir em ordem direta e inversa.

### P4 — selador

Recebe manifesto, L0s, oráculos RED e gate discriminatório. Escreve somente
`p1295-contract-seal.json`. Confirma hashes, allowlists, score, gate humano e
ausência de implementação antes do selo.

### P5 — implementador

Recebe L0s confirmados e selo, sem saídas privadas do oráculo/adversário. Pode
escrever corpos somente em:

- `03_infra/src/watch.rs`;
- `04_wiring/src/main.rs`;
- `p1295-implementation-receipt.md`.

Não altera teste, oráculo, manifesto, selo, build script, L0 ou P1294/P1293.

### P6 — verificador final

Recebe entradas congeladas e candidato. Não edita nada verificado. Pode escrever
somente superfícies, recibo, certificado e relatório P1295, nessa ordem causal.
Certificado e relatório só existem após recibo `PASS_READY_FOR_CERTIFICATE`.

## Gate RED e contrato protegido

O novo contrato externo deve verificar pelo menos:

1. snapshot capturado antes de uma alteração de conteúdo do mesmo tamanho;
2. `wait_for_change_since` retorna por causa dessa alteração sem recaptura;
3. criação e remoção de path observado;
4. `wait_for_change` legado continua funcional;
5. o tipo é opaco e não oferece mutação pública de fingerprints.

O teste CLI P1137 deve continuar cobrindo a sequência completa de publicação,
asset, irrelevante, erro e recuperação.

Registrar o RED com comando, exit code, testes que falharam, motivo e hashes.
Falha de compilação pela API ainda ausente é RED válido; timeout aleatório sem
testemunha não é suficiente para selar.

## Budget de calibração

- Máximo de duas revisões locais por `reason_code`.
- Cada revisão executa primeiro somente o teste focal e os três mutantes.
- Corpus completo, duas ordens e stress só rodam depois do recorte focal verde.
- Duas revisões consecutivas sem mudança do vetor ou da causa interrompem o
  passo e exigem redesenho; não executar uma terceira tentativa equivalente.
- Nenhum aumento de timeout, repetição-until-pass ou conversão de falha em flake
  conta como ganho discriminatório.

## Implementação autorizada após selo

P5 deve:

1. implementar o snapshot opaco no owner L3;
2. manter `Fingerprint` privado e detecção por conteúdo;
3. fazer L4 capturar o snapshot antes de `commit_output`/`discard_output`;
4. consumir exatamente aquele snapshot na espera;
5. preservar publicação atómica, erro fatal de rename, artefato anterior em
   erro, conjunto transitivo e `evict(10)`;
6. não tocar em build, CLI parsing, export ou semântica Typst.

## Gates finais

### Focais e stress

- Novo contrato P1295: duas execuções, depois ordem inversa com P1292/P1293.
- P1292 protegido: `11/11` em ambas as ordens.
- P1293 protegido: `11/11` em ambas as ordens.
- `p1137_watch_dependencias_recuperacao_e_filtro`: 20 execuções isoladas
  consecutivas, todas verdes; registrar duração individual e total.
- Suíte CLI integral: duas execuções verdes.
- Não executar em loop até passar; uma falha permanece no recibo.

### Workspace e arquitetura

- `cargo test --workspace -q` duas vezes, ambas verdes.
- `cargo fmt --all -- --check`.
- `crystalline-lint .`.
- V3, V4, V5, V7, V13, V14, V15 e V26 individualmente com
  `--fail-on warning`.
- `crystalline-lint --fix-hashes --dry-run .` imprime `Nothing to fix`.
- `git diff --check` e, após staging da allowlist exata,
  `git diff --cached --check`.

### Build e superfícies

1. Registrar HEAD, dirty stat, toolchain e `Cargo.lock`.
2. Executar `cargo build --release` com o `TYPST_COMMIT_SHA` efetivo explicitado.
3. Medir o novo SHA-256; não compará-lo a `c527b411...` ou `10fd9cf4...` como
   gate semântico.
4. Confirmar `typst --version` com versão `0.15.1` e prefixo do HEAD declarado.
5. Regenerar superfícies default e HTML em novos arquivos P1295.
6. Exigir contagens `111/99/12` e `115/104/11`, zero missing, zero unverified e
   zero `Unknown`.
7. Comparar todos os objetos de resultado com P1293: zero divergências.
8. Confirmar que o hash de proveniência interno aponta para o binário P1295.
9. Permitir diferença contra P1293 somente em
   `$.binaries.crystalline.sha256`; qualquer outra diferença bloqueia.

### Preservação P1294/P1293

- Todos os hashes P1294/P1293 congelados permanecem byte a byte.
- O recibo vermelho P1294 continua presente e `BLOCKED`.
- Nenhum certificado ou relatório final P1294 é criado.
- Inventário P1293 permanece `74/74`.
- Evidência de mutação P1293 permanece score `1.0`, zero survivor e zero
  `Unknown`; não a refazer contra a árvore produtiva.

## Certificado P1295

O certificado deve pinar:

- manifesto e selo P1295;
- confirmação humana e hashes dos L0s;
- contrato protegido, campanha MW1–MW3 e recibo de implementação;
- recibo vermelho P1294 preservado;
- HEAD, inventário de fontes e binário efetivamente testado;
- duas execuções workspace, stress 20/20 e superfícies P1295;
- recibo final P1295 congelado.

Claim máxima:

```text
Correção da janela de armamento do watch e continuidade evidencial P1294
verificadas para os artefatos, versões e observáveis registrados, sem isolamento
técnico de leitura e sem alegação de equivalência funcional geral.
```

Repetir literalmente:

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

O certificado é terminal. Não inseri-lo retroativamente em manifesto ou selo.

## Condições de bloqueio

Bloquear imediatamente se:

- o gate humano não ocorrer depois dos L0s e antes de teste/código;
- alguma autoridade ultrapassar sua allowlist;
- P1294/P1293 for reescrito ou absolvido;
- a correção for apenas timeout, sleep ou retry-until-pass;
- o snapshot for capturado depois da publicação;
- teste, adversário e implementação compartilharem saída privada;
- algum mutante válido sobreviver ou resultar em `Unknown`;
- qualquer uma das 20 execuções focais, duas suítes CLI ou duas suítes workspace
  falhar;
- houver divergência semântica de superfície;
- o hash do binário registrado não corresponder ao executável realmente usado;
- V5, V15, V26, dry-run, formato ou diff falhar;
- o verificador editar uma entrada verificada.

## Critério de conclusão

P1295 conclui somente com:

1. L0s atualizados e confirmação humana registrada;
2. RED determinístico e gate MW1–MW3 score `1.0` antes da implementação;
3. implementação limitada a L3/L4 e testes independentes preservados;
4. P1137 `20/20`, CLI `2/2` e workspace `2/2` sem apagar falhas;
5. superfícies semanticamente idênticas, com proveniência do novo binário;
6. P1294/P1293 byte a byte e recibo vermelho preservado;
7. recibo final independente `PASS_READY_FOR_CERTIFICATE`;
8. certificado P1295 terminal, sem autorreferência.

Até todos os critérios passarem, o estado é:

```text
P1294_BLOCKED_P1295_NOT_CERTIFIED
```
