# P1286 — receipt da campanha de mutações semânticas

**Natureza:** receipt de execução do gate discriminatório. Não é Prompt L0,
contrato, implementação, teste, oráculo, plano adversarial, selo nem veredito
final de materialização.

## 1. Regime, papel e capacidade

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, no
  papel estrito **Executor de Mutação**.
- Executor: agente `/root/mutacao_p1286`, checkout local compartilhado, em
  2026-08-30.
- Entrada normativa dos ataques: plano adversarial P1286 congelado; nenhuma
  mutação foi inventada para acomodar o candidato depois de observar um
  resultado.
- Leituras autorizadas e usadas: skill e duas referências, plano, contrato e
  testes congelados, runner/baseline congelados e implementação candidata.
- Escrita persistente autorizada: somente este receipt. Os patches mutantes e
  dois harnesses foram transitórios em `/tmp`; nenhum L0, teste, baseline,
  runner, contrato ou outro receipt foi editado.
- Capacidade negada e respeitada: corrigir candidato/testes, redefinir
  obrigação, retirar mutante do denominador, converter `Unknown` em morte,
  ressellar hashes ou emitir o veredito final P1286.
- Atestação: **segregação de papel e artefatos sem isolamento físico do host**.
  O checkout é compartilhado; a identidade byte a byte antes/depois de cada
  injeção foi, por isso, verificada explicitamente.

## 2. Entradas congeladas

Estado causal: `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree
não commitada. A execução válida terminou em
`2026-08-30T21:00:52.411407517-03:00`. O hash identifica bytes e não prova
isolamento.

| entrada protegida | SHA-256 antes e depois |
|---|---|
| `00_nucleo/diagnosticos/p1286-adversarial-mutation-plan.md` | `81ec09b6a4dac25f1319b02df12a3ab181728751d7c644153a48e016396d819c` |
| `04_wiring/tests/p1286_contract.rs` | `8936d84bcea19279257bd77df76dbb430b08d8591eec5dd808ded9bd1f96e06b` |
| `lab/surface-inventory/run_p1286_oracles.py` | `cc0d7986804a7abf006d03e44ffb6eb12a736b794db557b9d60d52e5cc02bf61` |
| `lab/surface-inventory/p1286-oracle-baseline.json` | `7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb` |

Hashes dos harnesses determinísticos usados na execução válida:

- `/tmp/p1286_case_runner.py`:
  `e1f25f0abd05b8f37393157c6ed8c99afd13e36ba5a7da91768deefcb43c3c73`;
- `/tmp/p1286_mutation_campaign.py`:
  `8701b28f2040fecc55ca3614bf82c2980945c24ce43a66810ac62f7fc2f56769`;
- log integral JSON da execução válida:
  `/tmp/p1286-mutation-campaign.json`, SHA-256
  `709ae0d1f9cb9b5f6ece65057c97003a94f8cba5f14d337db89b4764a2053a9d`.

O harness estreito importa o runner congelado sem o modificar. Acrescenta
duas lentes diretamente exigidas pelo plano: `measure(line(...)).width`
extraído por `pdftotext` para `LN-MAX-01`, e `pdfdetach -list` para contar o
attachment do documento sem frame em `PA-DROP-01`. Nessa última lente,
falha de compilação/listagem ou ausência de contagem seria `Unknown`; uma
listagem válida com `0 embedded files` é `Violated`, como exige o plano.

### 2.1 Bytes candidatos incluídos no gate

| source | SHA-256 original e restaurado |
|---|---|
| `01_core/src/compiler/lang/quotes.rs` | `d704388548135f951a6b716853c229fb5dea74db289871d272b7d62c315661b6` |
| `01_core/src/compiler/layout/smartquote.rs` | `85e91497ac539d13d7467b9b52ab57a4fbd5f0169976711b9f6763465ba0d895` |
| `01_core/src/compiler/stdlib/shapes.rs` | `46acfa64fce122277f8d1483b2d53a35d0d1836f3ff20efe4c95e424f12e163c` |
| `01_core/src/entities/color.rs` | `1c1936b8fd6d882726c8b2761c609e1f5a74c3b8e742e70ae6b4a7999478dbec` |
| `01_core/src/compiler/stdlib/color.rs` | `e61fa14d4b28b08e78d18d4e4ddb1f03a6162463bb99ecee0c377d31ea443870` |
| `03_infra/src/export/builder.rs` | `d7a462652304d721d654235d04b48aaaac5b4827ea67d193287f6a2ebe4c0add` |
| `03_infra/src/pipeline.rs` | `fedcec9306c6189ef9aff78aa5c07f649f24c69836a0f891cca8957f77f72d37` |
| `01_core/src/compiler/layout/pdf_artifact.rs` | `00cab2e1d8a26bea1acffca2aaecc9473aa93ce9a15d0ff4d1894b3f49c7a569` |
| `03_infra/src/export/stream.rs` | `1b7a0db3461b0dcf4d2597a33b5250f1cb7f64e87ed2bfd8cd8396b3ecda3019` |

O binário-base construído antes dos witnesses tinha SHA-256
`512b24e132621428598400f0c10820d408b1f6310e68613eb43e9fa946daf8c4`.
Os 22 comandos-base distintos passaram antes da primeira injeção válida.

## 3. Ambiente e comandos

- `rustc 1.92.0 (ded5c06cf 2025-12-08)`;
- `cargo 1.92.0 (344c4567c 2025-10-21)`;
- `Python 3.12.3`;
- `qpdf 11.9.0`;
- `pdftotext`/`pdfdetach 24.02.0`;
- `mutool 1.23.10`;
- `RUSTFLAGS=-Awarnings` somente para reduzir ruído do gate.

Para cada ID, em ambas as ordens, o executor aplicou por `apply_patch` uma
substituição única cujo fragmento anterior ocorria exatamente uma vez,
provou que o SHA mudou, executou:

```text
cargo build -q -p typst-wiring --bin typst
```

e então um dos comandos abaixo, conforme o witness:

```text
python3 /tmp/p1286_case_runner.py target/debug/typst <caso(s)>
cargo test -q -p typst-wiring --test p1286_contract <teste> -- --exact --nocapture
```

Todos os 50 builds mutantes terminaram com exit `0`, stdout vazio e stderr
vazio. Após o witness, o patch inverso foi aplicado por `apply_patch`; o SHA
do source foi comparado ao original antes de iniciar o ID seguinte.

Ordem 1:

```text
SQ-PRECEDENCE-01, SQ-AUTO-02, SQ-GRAPHEME-03, SQ-LANG-04,
SQ-ALTERNATIVE-05, LN-MAX-01, LN-ABS-02, LN-NORMALIZE-03,
LN-START-04, LN-END-PRECEDENCE-05, CM-SEQUENTIAL-01,
CM-RENORM-02, CM-NEGATIVE-03, CM-HUE-NGT2-04, PA-DROP-01,
PA-BYTES-02, PA-PATH-03, PA-METADATA-04, PA-DUPLICATE-05,
AR-PASSTHROUGH-01, AR-FORMULA-02, AR-MCID-03,
AR-DESCENDANT-MCID-04, AR-TAGS-OFF-VISUAL-05,
AR-TAGS-OFF-MARK-06
```

Ordem 2: a sequência acima exatamente invertida.

## 4. Evidência por mutante e repetição

Na tabela, `SHA mutante` e `out/err` são prefixos de 12 hex dos SHA-256
integrais registrados no log JSON da execução.
Exit `1` é divergência semântica do harness estreito; exit `101` ocorre
somente nos cinco testes Rust cujo assertion congelado falhou depois de um
build mutante bem-sucedido. `rest.` confirma restauração byte-idêntica em
ordem direta/reversa.

| ID | SHA mutante | exit direta/reversa | out/err direta | out/err reversa | rest. |
|---|---:|---:|---:|---:|---:|
| `SQ-PRECEDENCE-01` | `60b0ae2bc1d2` | `1` / `1` | `de197c9091de`/`e3b0c44298fc` | `de197c9091de`/`e3b0c44298fc` | sim/sim |
| `SQ-AUTO-02` | `22b41c15b5ad` | `101` / `101` | `13af9f594ed5`/`aff1a0c56d40` | `13af9f594ed5`/`324f0fb27c6a` | sim/sim |
| `SQ-GRAPHEME-03` | `bcc24687d89a` | `101` / `101` | `5c579e0d8e5c`/`690f01b41930` | `8634ebbfd8a9`/`f8f9aa7db7f6` | sim/sim |
| `SQ-LANG-04` | `a97371ba76a8` | `1` / `1` | `5d4ecddabeda`/`e3b0c44298fc` | `5d4ecddabeda`/`e3b0c44298fc` | sim/sim |
| `SQ-ALTERNATIVE-05` | `10668627af65` | `1` / `1` | `3495514845c9`/`e3b0c44298fc` | `3495514845c9`/`e3b0c44298fc` | sim/sim |
| `LN-MAX-01` | `cefd8efba994` | `1` / `1` | `def5256020d6`/`e3b0c44298fc` | `def5256020d6`/`e3b0c44298fc` | sim/sim |
| `LN-ABS-02` | `2b657456e4a6` | `1` / `1` | `763a4a0b4a2e`/`e3b0c44298fc` | `763a4a0b4a2e`/`e3b0c44298fc` | sim/sim |
| `LN-NORMALIZE-03` | `ac00ffd33fcb` | `1` / `1` | `66f921f034cc`/`e3b0c44298fc` | `66f921f034cc`/`e3b0c44298fc` | sim/sim |
| `LN-START-04` | `8309447f701f` | `1` / `1` | `adac8cd1bf99`/`e3b0c44298fc` | `adac8cd1bf99`/`e3b0c44298fc` | sim/sim |
| `LN-END-PRECEDENCE-05` | `5449debb271c` | `1` / `1` | `691905df8d94`/`e3b0c44298fc` | `691905df8d94`/`e3b0c44298fc` | sim/sim |
| `CM-SEQUENTIAL-01` | `49e687477b35` | `1` / `1` | `691056cd72c6`/`e3b0c44298fc` | `691056cd72c6`/`e3b0c44298fc` | sim/sim |
| `CM-RENORM-02` | `a2ee9628644b` | `1` / `1` | `e776453919f7`/`e3b0c44298fc` | `e776453919f7`/`e3b0c44298fc` | sim/sim |
| `CM-NEGATIVE-03` | `422f96f5e5aa` | `1` / `1` | `8ad4d96431b0`/`e3b0c44298fc` | `8ad4d96431b0`/`e3b0c44298fc` | sim/sim |
| `CM-HUE-NGT2-04` | `ff1ba290f9a2` | `1` / `1` | `9a27b338bccb`/`e3b0c44298fc` | `9a27b338bccb`/`e3b0c44298fc` | sim/sim |
| `PA-DROP-01` | `85d02b2766ac` | `1` / `1` | `15cc4ea15534`/`e3b0c44298fc` | `15cc4ea15534`/`e3b0c44298fc` | sim/sim |
| `PA-BYTES-02` | `92c69111abe7` | `1` / `1` | `36c0cb2931cd`/`e3b0c44298fc` | `36c0cb2931cd`/`e3b0c44298fc` | sim/sim |
| `PA-PATH-03` | `b11e8865d380` | `101` / `101` | `ed57519c681a`/`c8dd02eb3034` | `ed57519c681a`/`6fc3830d281f` | sim/sim |
| `PA-METADATA-04` | `7fd5f593a03e` | `101` / `101` | `616df0a929a8`/`152597dd6030` | `616df0a929a8`/`421d81c1caaf` | sim/sim |
| `PA-DUPLICATE-05` | `6f311c50f840` | `1` / `1` | `b5a72738bde8`/`e3b0c44298fc` | `b5a72738bde8`/`e3b0c44298fc` | sim/sim |
| `AR-PASSTHROUGH-01` | `fd00ba087b44` | `1` / `1` | `1eefee3942e2`/`e3b0c44298fc` | `1eefee3942e2`/`e3b0c44298fc` | sim/sim |
| `AR-FORMULA-02` | `a174039de950` | `1` / `1` | `73acd8625793`/`e3b0c44298fc` | `73acd8625793`/`e3b0c44298fc` | sim/sim |
| `AR-MCID-03` | `9128aeb33f89` | `1` / `1` | `a5a04eac3253`/`e3b0c44298fc` | `a5a04eac3253`/`e3b0c44298fc` | sim/sim |
| `AR-DESCENDANT-MCID-04` | `ca93926c7630` | `101` / `101` | `dba03cdfeb9d`/`4f44ae387122` | `abf4d3433667`/`957d5fce5b93` | sim/sim |
| `AR-TAGS-OFF-VISUAL-05` | `493c6216c897` | `1` / `1` | `8f601d15a4c9`/`e3b0c44298fc` | `8f601d15a4c9`/`e3b0c44298fc` | sim/sim |
| `AR-TAGS-OFF-MARK-06` | `1afaeaa6bd05` | `1` / `1` | `05e14cf35bba`/`e3b0c44298fc` | `05e14cf35bba`/`e3b0c44298fc` | sim/sim |

### 4.1 Observáveis de morte

Os observáveis abaixo foram iguais nas duas ordens, salvo PID/duração nos
diagnósticos dos processos Rust:

| ID | diferença observada no mutante |
|---|---|
| `SQ-PRECEDENCE-01` | texto `»Explicit«.` em vez do override explícito |
| `SQ-AUTO-02` | assertion: left `(X)`, right `„X“` |
| `SQ-GRAPHEME-03` | compilação do fixture falha no cardinality check escalar |
| `SQ-LANG-04` | texto `“Default”.` em vez de alemão |
| `SQ-ALTERNATIVE-05` | texto `„Alt“.` em vez do par alternativo |
| `LN-MAX-01` | `measure(...).width` extrai `30pt`, esperado `40pt` |
| `LN-ABS-02` | start `(40,50)`, delta `(30,30)`, end `(70,80)` |
| `LN-NORMALIZE-03` | start normalizado para `(0,0)` |
| `LN-START-04` | start descartado para `(0,0)`, end `(30,30)` |
| `LN-END-PRECEDENCE-05` | delta `(-99,0)`, end `(-89,20)` |
| `CM-SEQUENTIAL-01` | valor público `rgb("#3c87bc")` |
| `CM-RENORM-02` | valor público `rgb("#4a4d2f99")` |
| `CM-NEGATIVE-03` | valor público `rgb("#4b7d8a")` |
| `CM-HUE-NGT2-04` | HSL, HSV e Oklch terminam com exit `0`, sem erro |
| `PA-DROP-01` | listagem válida `0 embedded files` |
| `PA-BYTES-02` | payload extraído `00410a`, `matches_expected_payload=false` |
| `PA-PATH-03` | assertion `virtual path was not preserved` |
| `PA-METADATA-04` | assertion `description was not preserved` |
| `PA-DUPLICATE-05` | duas ocorrências terminam com exit `0`, sem erro |
| `AR-PASSTHROUGH-01` | `artifact_openings=0` |
| `AR-FORMULA-02` | Header tem `artifact_openings=0`; Background-controle preservado |
| `AR-MCID-03` | property list `/MCID999`, `artifact_has_own_mcid=true` |
| `AR-DESCENDANT-MCID-04` | assertion `artifact descendant leaked MCID` |
| `AR-TAGS-OFF-VISUAL-05` | texto vira `plain-before. plain-after.` |
| `AR-TAGS-OFF-MARK-06` | tags off passa a ter `artifact_openings=1` |

Não houve `Unknown`, sobrevivente, erro de injeção ou erro de compilação na
execução válida. Os casos opacos e eixos fora do plano não foram reclassificados.

## 5. Tentativas descartadas

Duas tentativas anteriores não recebem qualquer crédito:

1. uma injeção inicial de `PA-DROP-01` usou um constructor inexistente e não
   compilou; foi classificada `Unknown`, restaurada e a ordem foi reiniciada;
2. uma execução posterior revelou que o helper geral do runner devolvia
   `Unknown` quando `pdfdetach` listava zero attachments e a extração do item
   1 falhava. O executor detectou que o agregador transitório havia tratado
   incorretamente todo exit não-zero como morte, descartou integralmente o
   score e corrigiu o harness para exits distintos: `1=Violated`,
   `3=Unknown`. Só então repetiu as duas ordens completas registradas acima.

Essa correção não alterou plano, candidato, runner, baseline nem teste; apenas
impediu que `Unknown` recebesse crédito.

## 6. Score e limite da alegação

```text
ordem direta: 25 Violated, 0 Survived, 0 Unknown
ordem reversa: 25 Violated, 0 Survived, 0 Unknown
mutation_score = 25 / 25 = 1.0
```

Os nove sources candidatos e as quatro entradas protegidas terminaram com os
mesmos SHA-256 do início. `cargo build -q -p typst-wiring --bin typst` sobre o
candidato restaurado passou; `git diff --check` passou.

Este receipt atesta somente o poder discriminatório dos 25 mutantes e do
fragmento observável congelado P1286. Não declara equivalência funcional
geral, paridade geral de ICC/CMYK, suporte geral a standards PDF, nem o
veredito final da materialização.
