# P1293 — receipt de reabertura final dos observadores

**Papel segregado:** autor contratual/L0  
**Estado:** `L0_UPDATED_AWAITING_LINEAGE_RESEAL_AND_FINAL_GATES`  
**Timestamp:** `2026-09-02T16:57:35-03:00`  
**Commit base:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Working tree:** não commitada; a árvore já continha mudanças P1293 fora
deste lote. Este receipt atribui somente os três L0s e este documento ao autor
contratual; produto, consumers, oráculos, manifesto e selo ficaram intocados.

## Entradas autorizadas e pins

- passo explícito `00_nucleo/materialization/typst-passo-1293.md`:
  `d577a03c27731d11cdcc161a3e93646e90bdb62f0a7f9cfb8d1b4f5980a9dc3a`;
- plano adversarial `p1293-adversarial-plan.md`:
  `ecc12db3e5d9826dafdb7b42653e28d5f4f8e01772a8554e6718b1ac8129e226`;
- receipt adversarial `p1293-adversarial-receipt.json`:
  `8c742753c64524578f08151e604668d047be23a526d9acac9088398870f79868`;
- manifesto congelado:
  `0e589dd49012c781b54aea15b8e0a002ee11eefdac17306b8c724a9a43d1e52c`;
- selo congelado:
  `539bb6abad33152150434d585924d3ae64ea74b909a95f509c5ddeeae15c767f`.

Foram lidos integralmente os três L0s, seus consumers e ADR-0107, ADR-0108,
ADR-0127 e ADR-0129. Não foi lido patch candidato. O único ficheiro de
`materialization/` lido foi o P1293 explicitamente autorizado.

## Medição anterior às decisões

### Observador L3 de frames

O receipt adversarial, no estado de árvore acima e em
`2026-09-02T16:38:34-03:00`, registra `cargo test -p typst-infra --lib` com
`910` passes e `8` falhas nas asserções das linhas `4650`, `4695`, `4737`,
`4795`, `4892`, `4957`, `5008` e `5045` de
`03_infra/src/integration_tests.rs`. O helper `frame_items_recursive` em
`:34-51` atravessa `Group` sem transportar `Group.pos` ou `matrix`.
`01_core/src/entities/layout_types.rs:484-502` define posição, matriz e filhos
locais do grupo; `03_infra/src/export/svg.rs:1638-1682` aplica translação e
matriz antes de visitar os filhos. O consumer medido tem SHA-256
`6377db547065c4fa912924bfae88d6c0e296b1446fd55571f57c2a43783d0086`.

### Observador SVG P1292

O teste P1292 medido passou `10/11`; somente
`04_wiring/tests/p1292_contract.rs:422` divergiu. O helper
`svg_glyph_baselines` em `:144-165` usa o operando y da matriz interna
`4.862`, mas não acumula o `translate(... 2.651)` externo. A prova global
congelada é `2.651 + 4.862 = 7.513`; a expectativa não muda. O consumer
medido tem SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

### Owner do oráculo P1293

O consumer protegido tem C-P10 em
`04_wiring/tests/p1293_contract.rs:917-947`, MC13/MC14 em `:1147-1148` e
registry 31 em `:1155-1162`, mas seu owner ainda documentava 29 casos e fase C.
O consumer raw é
`ed3e0b57c703a1c4dbdbeda187fcf74dcf46476f60a2c850e48a9ef8f0b2df1e`,
o corpo canônico é
`b07eb8d6eb6bfa0a892c06bb1f5599d8e2e188963dcca821d83f4a34badfb13a`
e o registry é
`417d82202439b39388d223fe46c9dc193e41b705a951dfeb19c80eddfd75bd8b`.
O adversário obteve `11/11`, zero `Unknown` e confirmou A–D aprovados, mas
rejeitou a finalização até corrigir os observadores e repetir os gates globais.

## Decisões e classificação

1. O owner L3 exige que helpers geométricos componham transitivamente
   `Group.pos` e matriz na ordem do exporter e comparem coordenadas globais,
   sem mudar qualquer expectativa ou layout produtivo.
2. O owner P1292 exige que o parser SVG acumule transforms ancestrais, ao menos
   `translate` e `matrix`, preservando literalmente `7.513` e `21.901`.
3. O owner P1293 passa a registrar C-P10, MC13/MC14, registry/gate 31, hashes
   raw/canônico vigentes, A–D aprovados e correções finais pendentes. Nenhuma
   expectativa, fixture, política de comparação ou corpo protegido é alterado.

ADR-0107: coordenadas globais e DOM/diagnósticos são observáveis; recursão,
XML e representação local são mecânica de transporte. ADR-0108: cada obrigação
acima sucede a medição `file:line`, declara inferência e refutadores. ADR-0127:
são correções internas/test-only e atualização documental, sem API pública,
default, fase ou quebra de compatibilidade; fluxo contínuo, sem novo gate
humano. ADR-0129: cada Prompt owner mantém exatamente um consumer.

## Ownership e hashes L0

| Prompt|Prompt L0 1:1|SHA pré|SHA pós|Consumer congelado|SHA consumer|
|---|---|---|---|---|---|
|frames L3|`00_nucleo/prompts/infra/integration_tests.md`|`f711e6ee409dc34ed977020788c341b45c0d0e0fc2e6c965ce003f26f16af455`|`c7eb49160cb7516df502eb99124c93c2759d9666e573c638bfcf0b1e9d639bab`|`03_infra/src/integration_tests.rs`|`6377db547065c4fa912924bfae88d6c0e296b1446fd55571f57c2a43783d0086`|
|SVG P1292|`00_nucleo/prompts/wiring/tests/p1292_contract.md`|`41dd2e0ed56814baf94acf565d1fa59eb64f1f11b9d11cd1ea4937a998f2d0a1`|`614707e0a2d585bfec897c50f83fb52379e1b79f90dffc30c45c56caf37c1a7d`|`04_wiring/tests/p1292_contract.rs`|`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`|
|oráculo P1293|`00_nucleo/prompts/wiring/tests/p1293_contract.md`|`adf47e09192bf792ede1cf227696e6314f1b4aa059c510141d05118ebb799989`|`384c641845c0692e37cc6911d31866de936a82b51e9156281c724b485a7abbe7`|`04_wiring/tests/p1293_contract.rs`|`ed3e0b57c703a1c4dbdbeda187fcf74dcf46476f60a2c850e48a9ef8f0b2df1e`|

Os campos `Hash do Código` foram sincronizados documentalmente com os corpos
canônicos já existentes dos consumers (`99985da2`, `cf1c1953`, `b07eb8d6`),
sem editar esses consumers. Isso torna o preflight do dry-run determinístico;
o resselo pendente continua restrito aos três `@prompt-hash`.

O dry-run de lineage deve propor exatamente estes três headers, sem escrever:

```text
03_infra/src/integration_tests.rs
04_wiring/tests/p1292_contract.rs
04_wiring/tests/p1293_contract.rs
```

Qualquer quarto consumer, drift produtivo ou alteração de expectativa é
blocker. `crystalline-lint --fix-hashes` não foi executado.

## Validações executadas

Em `2026-09-02T16:57:35-03:00`, sobre o commit/working tree declarados:

- `crystalline-lint --checks v15 --fail-on warning .` → PASS, zero violações;
- `crystalline-lint --checks v26 --fail-on warning .` → PASS, zero violações;
- `git diff --check` → PASS;
- `crystalline-lint --checks v5 --fail-on warning .` → exatamente os três
  drifts transitórios esperados, e nenhum quarto: `integration_tests.rs`
  (`b920414e → 75a971ae`), `p1292_contract.rs`
  (`41dd2e0e → 05d0eea8`) e `p1293_contract.rs`
  (`b1f580e2 → b6f3b3af`);
- `crystalline-lint --fix-hashes --dry-run .` → PASS e “Would fix” exatamente
  os três consumers declarados: `hash-a` L0 `75a971ae`, `05d0eea8`,
  `b6f3b3af`; `hash-b` canônico do consumer `99985da2`, `cf1c1953`,
  `b07eb8d6`.

Nenhum comando de escrita do linter foi executado.

## Preservações verificáveis

- `00_nucleo/prompts/compiler/layout/equation.md` permaneceu em
  `9efca34585e1776267286b66dc74ed6ca213832b91568a8ffd79ccd2b8578179`;
- `01_core/src/compiler/layout/equation.rs` permaneceu em
  `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23`;
- manifesto e selo permaneceram nos pins acima;
- nenhum produto, consumer, teste protegido, oráculo, expectativa ou gate foi
  editado por este papel.

## Próximo gate

O coordenador pode fazer somente o resselo mecânico dos três headers listados.
Depois, implementadores test-only separados corrigem os dois observadores sob
seus owners; o owner P1293 recebe apenas header. A finalização continua
bloqueada até testes focais, V5/V15/V26, campanha discriminatória 31/31,
adversário e adjudicação independentes sem `Unknown`.
