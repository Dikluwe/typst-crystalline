# P1286 — receipt de verificação independente final

**Veredito:** `Preserved` para o fragmento observável e para a transformação
P1286 congelados. Este veredito não afirma equivalência funcional geral.

**Revisão pós-veredito:** reauditoria do delta exclusivamente documental em
`compiler/stdlib/text/smartquote.rs`; substitui o receipt predecessor SHA-256
`e4c6c30cce51e5166a02b45d8b3fa24f7f3fba5f4a07f24c2703233b73530096`.

## 1. Regime, papel e capacidade

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Papel: Verificador independente `/root/verificador_p1286`.
- Entradas: contrato v2, confirmação humana, owners L0, sources candidatos,
  teste/oráculos/baseline congelados, plano adversarial, receipt e log integral
  da campanha de mutações.
- Escrita permitida e exercida: somente este receipt.
- Capacidade negada e respeitada: corrigir L0, código, testes, oráculos,
  baseline, plano, campanha, configuração ou qualquer outro receipt.
- Atestação proporcional: papéis e capacidades segregados, mas sem isolamento
  físico do host; o checkout é compartilhado. Os artefatos protegidos foram
  identificados byte a byte antes e depois dos gates.

Foram lidos integralmente a skill e
`references/{papeis-e-capacidades,artefatos-e-gates}.md`. Não existe ADR
específica de materialização segregada entre as ADRs vigentes encontradas.

## 2. Obrigação, gate humano e ownership

O contrato autorizado é
`00_nucleo/diagnosticos/p1286-contract-receipt.md` v2, SHA-256
`16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`.
Ele classifica como paragem ADR-0127 os novos campos/tipos públicos de
smartquote e os carriers públicos de `pdf.attach`/`pdf.artifact`; line e
`color.mix` permanecem correções internas de paridade.

O receipt pós-gate, SHA-256
`27e2f5b931d898396319d7a62875ca377511e93ebe0e8b97cdd4f6817d6663c0`,
registra literalmente a confirmação humana de 2026-08-30: **“Siga a
engenharia da refatoração e pode implementar”**. Portanto o gate público foi
satisfeito antes da criação/materialização dos quatro owners novos.

Auditoria mecânica final:

```text
crystalline-lint --checks v5,v15,v26 --fail-on warning .
exit=0
✓ No violations found
```

Isso confirma no estado final o selo L0/source, a cardinalidade 1:1 e a
integridade dos Núcleos. Os quatro pares novos são:

| Prompt L0 proprietário (SHA-256 atual) | consumer exclusivo (SHA-256 atual) |
|---|---|
| `entities/elements/pdf_attach.md` — `37da5c125b16a5109837dd4742aa2d1a8370c244f1a387f16bcd2b48569393d2` | `entities/elements/pdf_attach.rs` — `a27fc2bb5b8b75ea93738939547a13d3e4213a7c5bf598159d02d25e62152179` |
| `entities/elements/pdf_artifact.md` — `1d06c44b410b7c951ca8a05214632993267cbeec5030c0697cd1f836e0d368a6` | `entities/elements/pdf_artifact.rs` — `5bd69660bafd428ecc0c3769aea87d1a84a0926c20a581c0f70086dd011c1d42` |
| `compiler/layout/pdf_attach.md` — `c76d6be7cc085556891a2d02ceae19d5b6b28c5415ba331236eacb5c62c32197` | `compiler/layout/pdf_attach.rs` — `4e3995322e71f0a158525d3ba45fc1db3e53b9313d64d65f99fee50b38d5f97b` |
| `compiler/layout/pdf_artifact.md` — `604d7c6a2c3b9d863c59d3f85d7f60d9a4ffc1cae28093c176c0756d06f54628` | `compiler/layout/pdf_artifact.rs` — `eb1d5c338d8e4ee02b9bbb043e0f36b49d255f456d539f4195eee91d190a85e3` |

Os layouts apontam aos seus prompts proprietários, não diretamente ao Núcleo
Forma B. O `match` de layout permanece estático/exaustivo e delega aos módulos
descendentes.

## 3. Integridade dos artefatos congelados

| artefato | SHA-256 exigido e observado após todos os gates |
|---|---|
| `04_wiring/tests/p1286_contract.rs` | `8936d84bcea19279257bd77df76dbb430b08d8591eec5dd808ded9bd1f96e06b` |
| `lab/surface-inventory/run_p1286_oracles.py` | `cc0d7986804a7abf006d03e44ffb6eb12a736b794db557b9d60d52e5cc02bf61` |
| `lab/surface-inventory/p1286-oracle-baseline.json` | `7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb` |
| `00_nucleo/diagnosticos/p1286-adversarial-mutation-plan.md` | `81ec09b6a4dac25f1319b02df12a3ab181728751d7c644153a48e016396d819c` |
| `00_nucleo/diagnosticos/p1286-mutation-campaign-receipt.md` | `93c23f037260758412a2d816c4b3718f592a26d2793bf23a417dae90f66d64e0` |

`crystalline.toml` contém exatamente a entrada
`p1286_contract = "04_wiring/tests/p1286_contract.rs"` em `[excluded_files]`,
comentada como artefato de verificação congelado e não consumer produtivo. A
exclusão não abrange diretório, wildcard, runner, baseline ou source de
produção. O teste continuou byte-idêntico ao hash protegido.

## 4. Gate discriminatório e campanha

O plano adversarial fecha 25 mutantes semanticamente não equivalentes:
5 smartquote, 5 line, 4 color, 5 attach e 6 artifact. O receipt da campanha e
o log JSON integral foram cruzados. Identidades do harness/log ainda presentes:

| artefato transitório | SHA-256 observado |
|---|---|
| `/tmp/p1286_case_runner.py` | `e1f25f0abd05b8f37393157c6ed8c99afd13e36ba5a7da91768deefcb43c3c73` |
| `/tmp/p1286_mutation_campaign.py` | `8701b28f2040fecc55ca3614bf82c2980945c24ce43a66810ac62f7fc2f56769` |
| `/tmp/p1286-mutation-campaign.json` | `709ae0d1f9cb9b5f6ece65057c97003a94f8cba5f14d337db89b4764a2053a9d` |

A inspeção independente do JSON confirmou:

```text
ordem forward: 25 execuções, 25 Violated, 0 Unknown
ordem reverse: 25 execuções, 25 Violated, 0 Unknown
ordem reverse = inversão exata da forward
50/50 injeções provadas; 50/50 restaurações byte-idênticas
50/50 builds mutantes com exit 0
mutation_score = 25 / 25 = 1.0
```

As duas tentativas descartadas do executor não recebem crédito; a campanha
válida posterior usa exits distintos para `Violated` e `Unknown`. Nenhum caso
opaco foi convertido em sucesso.

Depois da campanha, `cargo fmt`/resselo mudaram bytes de headers/formatação de
alguns sources. Isso não mudou o executável auditado: o SHA-256 do binário-base
da campanha e o do `target/debug/typst` reconstruído neste gate são ambos
`512b24e132621428598400f0c10820d408b1f6310e68613eb43e9fa946daf8c4`.
Os 25 fragmentos de decisão usados para injeção continuam ocorrendo exatamente
uma vez no candidato final. Logo o gate discriminatório permanece aplicável
ao comportamento final observado.

Após o primeiro veredito, foi corrigido somente o comentário histórico de
smartquote que ainda chamava `alternative` de scope-out, e `@updated` passou a
`2026-08-30`. A implementação não mudou; `@prompt-hash 961cbafa` permaneceu
idêntico e o executor informou `crystalline-lint --fix-hashes .` como
`Nothing to fix`. A reauditoria confirmou V5/V15/V26 sem violações e o binário
continuou com o mesmo SHA-256 acima. Portanto esse delta documental não altera
campanha, oráculo, testes nem veredito.

## 5. Oráculo final

Comando:

```text
python3 lab/surface-inventory/run_p1286_oracles.py --binary target/debug/typst
```

Resultado, em duas ordens, exit 0 e veredito `Preserved`:

| classificação | casos |
|---|---:|
| `Preserved` | 58 |
| `BaselineOnly` | 2 |
| `NotApplicable` | 1 |
| `Unknown` | 0 |
| `Violated` | 0 |

Os dois `BaselineOnly` são `line(dx:)`/`line(dy:)`, extensões cristalinas
explicitamente fora da alegação de paridade. O único `NotApplicable` é
`attach_bytes_metadata_a3`: o runner não observa superfície real de standard
PDF no candidato. Ele não foi contado como `Preserved`.

## 6. Gates finais reproduzidos

| comando | resultado observado |
|---|---|
| `cargo build` | exit 0 |
| `cargo test -p typst-core -p typst-infra --lib` | exit 0; core `5323 passed`; infra `911 passed`; zero falhas |
| `cargo test -p typst-wiring --test p1286_contract -- --nocapture` | exit 0; `6 passed`, zero falhas |
| `crystalline-lint .` | exit 0; sem erro fatal; advertências do backlog amplo permanecem fora deste fragmento |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | exit 0; `No violations found` |
| `cargo fmt --all -- --check` | exit 0; stream vazio |
| `git diff --check` | exit 0; stream vazio |

Ambiente observado: `rustc 1.92.0`, `cargo 1.92.0`, qpdf `11.9.0`,
pdftotext/pdfdetach `24.02.0` e mutool `1.23.10`.

## 7. Proveniência do veredito

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Estado: working tree não commitada e compartilhada.
- Instante da reauditoria documental: `2026-08-30T21:21:34-03:00`.
- `git diff HEAD --stat`: `124 files changed, 647273 insertions(+), 1789 deletions(-)`.
- SHA-256 do stdout exato de `git diff HEAD --stat`:
  `8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c`.
- SHA-256 do stdout de `git diff HEAD --numstat`:
  `249985a9cbbb7ca5409339500fccada45105042e8d285b44d71508c4859c6fb6`.
- SHA-256 de `git status --short`:
  `aaccc8c5ff6b879c869ae3fe4fefa7b40c902721931563322f8362b2b9c34836`.

O stat global inclui trabalho concorrente dos passos P1281–P1286 e não é
identidade causal isolada do P1286. A identidade causal usada no veredito é o
conjunto de hashes protegidos, o binário reconstruído e os comandos acima.
Como `git diff --stat` não inclui untracked, os receipts/teste/runner/baseline
novos aparecem no `git status --short`, cuja saída também foi pinada.

## 8. Limites e decisão

- A ponte `rgb(color)` sela apenas o vetor CMYK medido; **não** atesta
  paridade ICC/CMYK geral, spot colors ou gestão de cor geral.
- O argumento oculto `--pdf-standard` é compatibilidade de invocação do
  witness de Background; não seleciona standard, não prova PDF 2.0 e não
  constitui suporte PDF/A-3. `/AFRelationship` no eixo PDF/A-3 permanece fora
  do fragmento.
- Detectar `/Artifact`, ausência de MCID próprio e transparência visual/textual
  no corpus não prova efeito real em tecnologia assistiva, screen reader,
  reflow ou copy/paste.
- Standards, compressão, casos regionais/opacos e fallbacks não exercidos
  conservam `Unknown`; nenhum foi promovido a sucesso.
- O comentário histórico não normativo de smartquote foi alinhado ao contrato
  P1286 na reauditoria documental; o selo semântico, binário e resultados
  permaneceram idênticos.

Com os limites acima, obrigação, contrato, implementação, testes, oráculos,
ataques e veredito permanecem segregados por autoridade. O resultado final é
**`Preserved`**, com `mutation_score = 1.0` e zero `Unknown` dentro do fragmento
selado P1286.
