# P1292 — recibo de implementação do lote D, v12

Data: 2026-09-01
Papel: `/root/implementador_d_v11`, implementador de produto segregado
Regime: materialização segregada completa, sem alegação de isolamento técnico
Manifest v12 atualizado: `fcf9c3dff41e869272068c12d65cd0b4c1801cf36b6ce9aec337c6dac86f0552`
Contrato canônico v12: `493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`
Seal v12: `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`

## Refino nested-region Block-only v12 — evidência supersedente

O receipt Block-only anterior de SHA-256
`a185bf8dc1659e5dd2ea8449b48749885a5d5dc8876a2ef11a64e088fc759a81`
fica expressamente invalidado como evidência corrente. O resumo permitido da
execução protegida informou main e no-flush GREEN, `Unknown=0`, e um único
RED nested: na fixture exata SHA-256 `ff19ec86…`, o candidato ancorava
`NESTED_FLOAT yMin=67.404pt` contra `7.404pt` no vanilla, enquanto
`NESTED_AFTER=4.642pt` já era bilateral. O implementador não leu nem executou
o oracle protegido.

### Diagnóstico e correção restrita

`layout/block.rs` já tornava `width` explícita a região efetiva durante o
body, mas deixava `regions.current.height` apontar para os 100pt externos.
Assim, o owner Place calculava corretamente um bottom float — porém contra o
fundo da página externa, não contra o `block(height:40pt)` que o continha.

A correção, exclusiva de `layout/block.rs`, salva a altura externa, expõe
durante o body o limite físico local
`start_y + outset_top + explicit_height` e restaura a altura externa ao sair.
O limite efetivo é a interseção com a região recebida:
`min(saved_height, local_end)`. Essa interseção é necessária para que um Block
nunca amplie a região já reduzida por um float exterior.

Uma primeira tentativa que substituía a região por `local_end` sem a
interseção corrigiu o nested, mas moveu `AFTER_FLOW/AFTER_MARKER` do vetor
principal para p2. Ela foi refutada e não integra o resultado final. A forma
final mantém width, frame-end, gap `1.2em`, Place, Cursor, Flush, fitting,
replay e testes congelados.

### Medição final

| Fixture | Token | Página | `yMin` candidato | Referência |
|---|---|---:|---:|---:|
| nested exata | NESTED_AFTER | 1 | `4.642pt` | `4.642pt` |
| nested exata | NESTED_FLOAT | 1 | `7.404pt` | `7.404pt` |
| main | FLOAT_BEFORE | 2 | `17.404pt` | `17.404pt` |
| main | AFTER_FLOW | 3 | `-2.596pt` | `-2.596pt` |
| main | AFTER_MARKER | 3 | `4.642pt` | `4.642pt` |
| main | FLOAT_AFTER | 3 | `87.404pt` | `87.404pt` |
| no-flush | after | 1 | `40.604pt` | `40.604pt` |
| no-flush | AFTER_MARKER | 1 | `47.842pt` | `47.842pt` |
| no-flush | FLOAT_BEFORE | 2 | `17.404pt` | `17.404pt` |

O teste público FixedMetrics refutado descrito na seção anterior permanece
congelado e fora destes controles; não foi editado nem convertido em sucesso.
Esta entrega continua sendo evidência de implementação, não veredito geral.

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T11:46:30-03:00`. `git diff HEAD --stat` registrou
`47 files changed, 3316 insertions(+), 506 deletions(-)`, além de artefatos
untracked. Nesta reabertura, o único arquivo produtivo alterado pelo
implementador foi `01_core/src/compiler/layout/block.rs`; receipt é a única
saída documental. Não há alegação de isolamento técnico da árvore
compartilhada.

| Comando | Resultado final |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_flush_checkpoint_atomico_move_todo_o_sufixo -- --nocapture` | PASS — 1/1 após interseção; a tentativa sem `min` havia sido RED, p2 em vez de p3 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_place_nao_float_usa_origem_da_linha_pendente_sem_flush -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_bottom_float_ -- --nocapture` | PASS — 2/2 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p245_place_float -- --nocapture` | PASS — 3/3 |
| `env RUSTFLAGS=-Awarnings timeout 300s cargo build -q -p typst-wiring --bin typst` | PASS |
| `timeout 240s cargo check -q -p typst-core` | PASS; warnings preexistentes |
| `cargo fmt --all -- --check` e `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v5` | owner Block PASS; somente dois resíduos alheios em `math/layout/cancel.rs` e `entities/elements/math_cancel.rs` |
| nested exata + main + no-flush sob `timeout 30s`, seguidos de `pdftotext -bbox` | PASS — valores da tabela final |

Hashes desta evidência nested-region:

```text
635135faa7160030e73e39eff692f47d654023752a4a189f98e2bb980a3d1857  01_core/src/compiler/layout/block.rs
1bbdc72ee23d7d0d6b79fd1efee7866bf2d5efcadcb0b71957ccad8e05e98a24  01_core/src/compiler/layout/place.rs (congelado)
a29afe2f09bb141fffbdfb00275fde0c385ee14746baaa2958070b7414f8caf0  target/debug/typst
ff19ec86bc905fab20adf62668e73c465c3994a1b8baee876656b08a83e2c953  /tmp/p1292-exact-nested.typ
9a8c5947311d865918a0172f4588ee7e8630d4a9331b769099cb660ecab1e63e  /tmp/p1292-v12-nested-final.pdf
266584627d8dcda32514e209b54d60c132b31e52ca79c15dc9dbcb0cf8e910a3  /tmp/p1292-v12-nested-main-control.pdf
9b43b396917c4c5f670076e3f30f5ec93927ef223b02d65a27115e1daab206d3  /tmp/p1292-v12-nested-no-flush-control.pdf
3585212f8f337c375b28d0c1f0994878d7815ede39c186d06f525a5e8b3310f8  /tmp/p1292-vanilla-nested.pdf
```

Esta seção supersede o receipt Block-only anterior mantido abaixo como
histórico. Produto suspenso após evidência nested-region; parada antes de nova
execução protegida, ataques e veredito.

---

## Histórico Block-only v12 — manifest `0c024d25…`

## Refino Block-only v12 — evidência supersedente com conflito focal

O receipt Place-only anterior de SHA-256
`3cb131e1e0db7cadae37a38b3f84fd042d5aa5e8d2d6e0eabc255c07640062a1`
fica expressamente invalidado como evidência de fecho. A reprodução externa
autorizada, fixture SHA-256 `1e88beab…`, mediu no candidato anterior
`AFTER_FLOW=33.366pt` e `AFTER_MARKER=40.604pt`, contra
`40.604/47.842pt` no vanilla ratificado. Place já preservava corretamente a
distância relativa de `7.238pt`; toda a unidade seguinte ao Block estava
`7.238pt` acima. O implementador não leu nem executou o oracle protegido.

### Diagnóstico e correção restrita

No branch de container geométrico, `layout/block.rs` registrava
`prev_line_baseline = frame_end - top_edge` quando o corpo tinha linha de
flow. Isso removia `7.238pt` da fronteira física que a próxima unidade deve
herdar. A correção define incondicionalmente
`prev_line_baseline = frame_end`, conforme o L0 v12, e remove somente o estado
auxiliar `body_had_flow_line` que ficou morto. O gap pendente default `1.2em`
e seu colapso permanecem intactos. Nenhum outro arquivo produtivo foi alterado
nesta reabertura; Place, Cursor, Flush, floats, replay e testes ficaram
congelados.

### Reprodução externa e controles finais

| Fixture | Token | Candidato final | Vanilla ratificado |
|---|---|---:|---:|
| externa exata sem Flush | before | `-2.596pt` | `-2.596pt` |
| externa exata sem Flush | after | `40.604pt` | `40.604pt` |
| externa exata sem Flush | AFTER_MARKER | `47.842pt` | `47.842pt` |
| externa exata sem Flush | FLOAT_BEFORE | `17.404pt` | `17.404pt` |
| principal com replay | AFTER_FLOW | `-2.596pt` | `-2.596pt` |
| principal com replay | AFTER_MARKER | p3, `4.642pt` | p3, `4.642pt` |
| principal com replay | FLOAT_BEFORE | p2, `17.404pt` | p2, `17.404pt` |
| principal com replay | FLOAT_AFTER | p3, `87.404pt` | p3, `87.404pt` |
| sem float | AFTER_FLOW | `40.604pt` | `40.604pt` |
| sem float | AFTER_MARKER | `47.842pt` | `47.842pt` |

Existe um conflito público conhecido, não mascarado: o teste congelado
`p1292_block_default_12em_transporta_origem_do_flow_seguinte`, sob
`FixedMetrics`, ainda espera delta `35.5pt` e após a fórmula owner-correct
mede `43.2pt`. Esse teste havia legitimado a subtração agora refutada pela
fixture externa e pelo L0 de fim físico. Por instrução do coordenador, ele foi
executado e registrado como RED stale/refutado, mas não editado nem usado para
reverter o reparo. Por isso esta seção é evidência de implementação e conflito,
não veredito de fecho geral.

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T11:38:20-03:00`. `git diff HEAD --stat` registrou
`47 files changed, 3302 insertions(+), 506 deletions(-)`, além de artefatos
untracked. Nesta reabertura, o único arquivo produtivo alterado pelo
implementador foi `01_core/src/compiler/layout/block.rs`; receipt é a única
saída documental. Não há alegação de isolamento técnico da árvore
compartilhada.

| Comando | Resultado final |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_block_default_12em_transporta_origem_do_flow_seguinte -- --nocapture` | RED conhecido — esperado `35.5`, obtido `43.2`; teste FixedMetrics refutado e congelado |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_flush_checkpoint_atomico_move_todo_o_sufixo -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_place_nao_float_usa_origem_da_linha_pendente_sem_flush -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_bottom_float_ -- --nocapture` | PASS — 2/2 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p245_place_float -- --nocapture` | PASS — 3/3 |
| `env RUSTFLAGS=-Awarnings timeout 300s cargo build -q -p typst-wiring --bin typst` | PASS |
| `timeout 240s cargo check -q -p typst-core` | PASS; warnings preexistentes |
| `cargo fmt --all -- --check` e `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v5` | owner Block PASS; somente dois resíduos alheios em `math/layout/cancel.rs` e `entities/elements/math_cancel.rs` |
| três compilações próprias sob `timeout 30s` + `pdftotext -bbox` | PASS — valores da tabela final |

Hashes desta evidência Block-only:

```text
489a5f9cca718ccfea284f3a43157cf88367700c3c489fb189a6f3340f26ba48  01_core/src/compiler/layout/block.rs
1bbdc72ee23d7d0d6b79fd1efee7866bf2d5efcadcb0b71957ccad8e05e98a24  01_core/src/compiler/layout/place.rs (congelado)
beb08819604a5815ec14e117d09135681a286686e94c6a1e660290d089d20832  target/debug/typst
1e88beab45469c8dee74e0f9b532de552d4a62c3ac12263164b21d4771ca8b57  /tmp/p1292-exact-no-flush.typ
2d97c81a096a83e863cdf3bad2d23db9da928f346781196741bb1896a5768b19  /tmp/p1292-v12-block-final-exact.pdf
3d429d887b8ca0affb1b5f695a3fcb56006ebc1c4df90c58bf0e1cb7cb320550  /tmp/p1292-v12-block-final-main.pdf
bc0ea02fbd9c3fc33c95aca24b77c927410f96a5fe548a752b2109ff65d9c78b  /tmp/p1292-v12-block-final-no-float.pdf
ff8919b43c367e8e07de2b5465f0d12b1f35b313ecf14c09de69e06099b4550c  /tmp/p1292-vanilla-no-flush.pdf
```

Esta seção supersede o receipt Place-only mantido abaixo como histórico.
Produto suspenso após evidência Block-only; parada antes de nova execução
protegida, ataques e veredito.

---

## Histórico Place-only v12 — manifest `96e3800f…`

## Refino Place-only v12 — evidência final supersedente

O receipt v12 anterior de SHA-256
`a230cd547dc35934407894dd69f1ec2316599efb19a4137d737d7e27151bd16d`
fica expressamente invalidado como evidência de fecho. Sob o manifest
atualizado, o resumo permitido da execução protegida informou Block correto,
vetor principal e outros dez testes GREEN, `Unknown=0`, mas o controle sem
Flush ainda tinha `AFTER_FLOW yMin=40.604pt` e
`AFTER_MARKER yMin=40.604pt`, contra `47.842±0.002pt`: faltava exatamente o
top-edge semântico de `7.238pt`. O implementador não leu nem executou o
oracle protegido.

### Diagnóstico e correção restrita

O wrapper de `layout/place.rs` tentava comunicar a origem ordinária alterando
temporariamente `page_config.margin.top`. A leitura do owner comum confirmou
que `layout_place`, quando executado dentro de sub-frame, usa
`origin_y=0.0`; portanto a margem não participa desse ramo e o override era
ineficaz.

A correção ficou exclusivamente em `layout/place.rs`. Para Place não-float
default/Top dentro de sub-frame com linha ativa, o owner captura a fronteira
dos efeitos criados por sua própria chamada e translada somente essa cauda
pelo top-edge semântico. Entradas Y de alinhamento diferido e segmentos de
decoração criados pela mesma ocorrência recebem a mesma origem, impedindo que
fixups posteriores desfaçam o transporte. Fora de sub-frame, o caminho já
existente por `margin.top` permanece; floats e transação não foram alterados.
Nenhuma constante de fixture, inspeção de Block/marker ou compensação global
foi introduzida.

### Medição bilateral própria final

| Fixture própria | Token | Página | `yMin` candidato |
|---|---|---:|---:|
| sem Flush | AFTER_FLOW | 1 | `40.604pt` |
| sem Flush | AFTER_MARKER | 1 | `47.842pt` |
| sem Flush | FLOAT_BEFORE | 2 | `17.404pt` |
| principal com replay | AFTER_FLOW | 3 | `-2.596pt` |
| principal com replay | AFTER_MARKER | 3 | `4.642pt` |
| principal com replay | FLOAT_BEFORE | 2 | `17.404pt` |
| principal com replay | FLOAT_AFTER | 3 | `87.404pt` |
| sem float | AFTER_FLOW | 1 | `40.604pt` |
| sem float | AFTER_MARKER | 1 | `47.842pt` |

O controle ganhou exatamente `7.238pt`; o replay continuou em p3
`4.642pt`. Página, ordem, cardinalidade, anchors, default relativo `1.5em`,
fitting e a correção Block de `13.2pt` foram preservados.

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T11:32:10-03:00`. `git diff HEAD --stat` registrou
`47 files changed, 3314 insertions(+), 506 deletions(-)`, além de artefatos
untracked. Nesta reabertura, o único arquivo produtivo alterado pelo
implementador foi `01_core/src/compiler/layout/place.rs`; receipt é a única
saída documental. A árvore compartilhada contém trabalho simultâneo de
outros lotes, portanto não há alegação de isolamento técnico.

| Comando | Resultado final |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_place_nao_float_usa_origem_da_linha_pendente_sem_flush -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 240s cargo test -q -p typst-core p1292_ -- --nocapture` | PASS — 17/17 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p245_place_float -- --nocapture` | PASS — 3/3 |
| `env RUSTFLAGS=-Awarnings timeout 300s cargo build -q -p typst-wiring --bin typst` | PASS |
| `timeout 240s cargo check -q -p typst-core` | PASS; warnings preexistentes |
| `cargo fmt --all -- --check` e `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v5` | owner Place PASS; somente dois resíduos alheios em `math/layout/cancel.rs` e `entities/elements/math_cancel.rs` |
| três compilações próprias sob `timeout 30s` + `pdftotext -bbox` | PASS — valores da tabela final |

Hashes desta evidência Place-only:

```text
1bbdc72ee23d7d0d6b79fd1efee7866bf2d5efcadcb0b71957ccad8e05e98a24  01_core/src/compiler/layout/place.rs
b88130fe74f0ef04a991e6f38764d38e6cae661dd96cd279375cb9ef213f3a24  target/debug/typst
d0ef9c0534aec9c9fd23104bbb5b8ca9dc82f99b63b9f92a1e0b7c51ec838be0  /tmp/p1292-v11-no-flush.typ
0b3a5b8a81f26f0969ec5a4cb7ba420dbd81fbc4e3c9ec15096de7f64213f20f  /tmp/p1292-v11-public-shape.typ
31b6fed8403c7138f4eebc093d6b00ac3cdf1f0eb1f5df53caf92795a33df7a2  /tmp/p1292-v11-block-control.typ
dced91c11d82b749b5c35473e9a824b925eff34a2da1eb9ab5ab66c73283d150  /tmp/p1292-v12-no-flush-place-fix.pdf
6288202152fb3b2446f9b5852219efb54ceb13d3f14d36002b95c805a7d0d503  /tmp/p1292-v12-main-place-fix.pdf
869f46e85ef9c17953155a7719c835304e14ac23c133fbf9603007eda333887b  /tmp/p1292-v12-no-float-place-fix.pdf
```

Esta seção supersede a evidência v12 anterior mantida abaixo apenas como
histórico auditável. Implementação e receipt D Place-only encerrados; parada
antes de nova execução protegida, ataques e veredito.

---

## Histórico v12 anterior — manifest `3e13499e…`

## Reabertura causal v12 — evidência final supersedente

O receipt v11 imediatamente anterior de SHA-256
`5bd77ee6d8e48000953187f529dd9d186f5957ada316ac5ac0f8b90edec31391`
fica expressamente invalidado como evidência de fecho. O resumo permitido dos
testes independentes v12 apresentava 17 testes P1292, 15 GREEN, dois RED
causais e `Unknown=0`: o encadeamento geométrico de Block produzia delta
`22.3pt` contra `35.5pt`, faltando `13.2pt`; Place não-float produzia
`-20.9pt` contra a origem esperada `baseline + top-edge = 7.7pt`. O resumo
permitido do oracle protegido mantinha dez GREEN e um controle sem Flush RED,
com `AFTER_MARKER yMin=-9.834pt` contra `47.842±0.002pt`. O implementador não
leu nem executou o oracle protegido.

### Diagnóstico medido e correção nos owners

Os L0 v12 vigentes atribuem as duas causalidades a owners distintos. A
fronteira geométrica de `Block` transporta o fim físico e o espaçamento abaixo
efetivo, cujo default relativo é `1.2em`; quando o corpo possui linha de flow,
a baseline lógica é derivada do fim físico menos o top-edge semântico. Quando
o corpo só materializa Place fora da linha, o próprio fim físico é a origem.
Essa regra foi implementada em `layout/block.rs`, sem especialização por
fixture e sem compensação em Cursor.

`layout/place.rs` agora ancora Place não-float com alinhamento default/Top na
baseline e no top-edge semântico da linha pendente. A correção é local à
chamada de Place e restaura a margem em seguida, preservando deslocamentos,
alinhamento horizontal e fixups existentes. A distinção `None`/`Some(Top)` foi
medida na superfície pública: a stdlib materializa o default como
`Some(Top)`.

A compensação global de descendentes de replay mantida no receipt v11 foi
removida integralmente de `Layouter` e `Cursor`. Checkpoint, rollback,
`new_page` e replay permanecem transacionais, mas o caminho comum sem marker
não herda origem de replay. Uma primeira implementação v12, depois refutada,
produziu `43.2pt` no teste Block por duplicar o top-edge, e `-50.9pt` no teste
Place por considerar somente `None`; esses resultados intermediários não
integram o fecho. Após restringir cada regra ao seu owner, os dois testes
causais passaram e o focal P1292 ficou GREEN em 17/17.

### Medição bilateral própria final

Foram usadas três fixtures próprias, sem leitura do oracle: vetor principal
com marker, controle sem Flush e controle sem float. Os resultados finais do
candidato foram:

| Fixture | Token | Página | `yMin` candidato |
|---|---|---:|---:|
| principal | AFTER_FLOW | 3 | `-2.596pt` |
| principal | AFTER_MARKER | 3 | `4.642pt` |
| principal | FLOAT_BEFORE | 2 | `17.404pt` |
| principal | FLOAT_AFTER | 3 | `87.404pt` |
| sem Flush | AFTER_FLOW | 1 | `40.604pt` |
| sem Flush | AFTER_MARKER | 1 | `47.842pt` |
| sem Flush | FLOAT_BEFORE | 2 | `17.404pt` |
| sem float | AFTER_FLOW | 1 | `40.604pt` |
| sem float | AFTER_MARKER | 1 | `47.842pt` |

O vetor principal preserva três páginas, cardinalidade exatamente uma vez,
ordem, `AFTER_MARKER` em p3, anchors `17.404/87.404`, fitting e o default de
clearance relativo `1.5em`. Os controles confirmam que a origem comum sem
marker/transação voltou a `47.842pt` e não depende de compensação global.

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T11:24:54-03:00`. `git diff HEAD --stat` registrou
`47 files changed, 3285 insertions(+), 506 deletions(-)`, além dos artefatos
untracked listados por `git status --short`. A árvore inclui alterações
simultâneas de outros lotes; esta implementação D v12 alterou produtivamente
somente os owners `layout/mod.rs`, `layout/cursor.rs`, `layout/block.rs` e
`layout/place.rs`. `layout/flush.rs` foi preservado.

| Comando | Resultado final |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 240s cargo test -q -p typst-core p1292_ -- --nocapture` | PASS — 17/17 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_bottom_float_ -- --nocapture` | PASS — 2/2 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p245_place_float -- --nocapture` | PASS — 3/3 |
| `timeout 240s cargo check -q -p typst-core` | PASS; warnings preexistentes |
| `timeout 300s cargo build -q --workspace` | PASS; warnings preexistentes |
| `cargo fmt --all -- --check` e `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v1,v2,v15,v26` | PASS — zero violações |
| `timeout 180s crystalline-lint . --checks v5` | owners D PASS; somente dois resíduos alheios e preexistentes em `math/layout/cancel.rs` e `entities/elements/math_cancel.rs` |
| compilações das três fixtures próprias sob timeout + `pdftotext -bbox` | PASS — valores da tabela final |

Hashes desta evidência v12:

```text
9d4aa8e56a4e9157cbab345ddd40cfaae73c4c4c60f7fac85741b4c79e05069d  01_core/src/compiler/layout/mod.rs
55f3ee8c7287e22f5a9f93cbd263d8483be8fa9b1c3fd00a3f45711679e3709f  01_core/src/compiler/layout/cursor.rs
98a2697a0e7383ad0a582bd27a97b2269c01a842f7656a360b1c97143689bc6e  01_core/src/compiler/layout/block.rs
9d6eb4ebc42e619097776100d13128f2f8cc4700f6b2f42e1d69855a7921c1b0  01_core/src/compiler/layout/place.rs
54b2d5e97e1d009151efbae116db6058356dcf96cf73fa151e48891acbb5a105  01_core/src/compiler/layout/flush.rs
1c1fe6bf9161b801d9f3074a32078710a65e580b045e7fda5fedc06f5017e79a  target/debug/typst
0b3a5b8a81f26f0969ec5a4cb7ba420dbd81fbc4e3c9ec15096de7f64213f20f  /tmp/p1292-v11-public-shape.typ
d0ef9c0534aec9c9fd23104bbb5b8ca9dc82f99b63b9f92a1e0b7c51ec838be0  /tmp/p1292-v11-no-flush.typ
31b6fed8403c7138f4eebc093d6b00ac3cdf1f0eb1f5df53caf92795a33df7a2  /tmp/p1292-v11-block-control.typ
a7fb61c7db3a7f940bc251eae6312e5b076b6ed4b837c86add1a8a644ee347f6  /tmp/p1292-v12-main-c.pdf
8251abdd6ea9e095325a7f1d069f41fe6244a514690eda609616d0b6358458a8  /tmp/p1292-v12-no-flush-c.pdf
98dab71c5a24f6c8add22a8ba99299c438a31e8b211a63996834ddefd8a5a25a  /tmp/p1292-v12-no-float-c.pdf
```

Esta seção supersede as evidências v11 mantidas abaixo apenas como histórico
auditável. Implementação e receipt D v12 encerrados; parada antes de ataques,
nova execução protegida e veredito.

---

## Histórico v11

## Reabertura posicional v11 — evidência final supersedente

O receipt anterior de SHA-256
`61b5bc594b25770e93952d12cbff952f29d53d1414015018899f0c3f69f56f34`
fica expressamente invalidado como evidência de fecho. A adjudicação protegida
resumida pelo coordenador refutou somente a posição de `AFTER_MARKER`: destino
p3 e cardinalidade 1× estavam corretos, mas o candidato produzia
`yMin=-9.834pt` contra `4.642±0.002pt`. Os demais dez testes estavam GREEN e
`Unknown=0`. O implementador não leu nem executou o oracle protegido durante
esta reabertura.

### Medição anterior à correção

Uma fixture própria bilateral com o mesmo comportamento observável confirmou:

| Token | Vanilla ratificado | Candidato antes do reparo |
|---|---:|---:|
| AFTER_FLOW | p3, `-2.596000` | p3, `-2.596000` |
| AFTER_MARKER | p3, `4.642000` | p3, `-9.834000` |
| FLOAT_BEFORE | p2, `17.404000` | p2, `17.404000` |
| FLOAT_AFTER | p3, `87.403999` | p3, `87.404000` |

O rollback removia corretamente toda a cauda, mas o replay recompunha um
efeito direto em `current_items`, produzido enquanto `current_line` já tinha
flow, contra o topo físico da página nova. Assim ele perdia a origem temporal
da primeira unidade de flow embora sua ocorrência, página e identidade fossem
preservadas.

### Correção

O `Layouter` registra estado transitório privado do replay e a fronteira de
items já ajustados. O `Cursor`, durante o replay apenas, observa genericamente
o crescimento de `current_items` enquanto existe `current_line` pendente e
recompõe a origem ordinária dessa unidade a partir de
`cursor_y + text_edges(style).top - margin.top`. É o mesmo top-edge semântico
usado por `ensure_initial_baseline`; não existe constante `4.642`, inspeção de
variant, tratamento de Block ou alteração em Place/Flush. Wrappers ancestrais
não reaplicam a translação porque a fronteira ajustada é monotônica.

Um ensaio intermediário com ascender físico produziu `7.238pt` e foi refutado;
ele não integra o resultado final. O top-edge semântico produziu o resultado
bilateral final:

| Token | Vanilla ratificado | Candidato final |
|---|---:|---:|
| AFTER_FLOW | p3, `-2.596000` | p3, `-2.596000` |
| AFTER_MARKER | p3, `4.642000` | p3, `4.642000` |
| FLOAT_BEFORE | p2, `17.404000` | p2, `17.404000` |
| FLOAT_AFTER | p3, `87.403999` | p3, `87.404000` |

Ambos têm três páginas e cada um dos cinco tokens aparece exatamente uma vez.
Cardinalidade, ordem, anchors, default relativo `1.5em`, fitting, Place, Flush
e Block não foram alterados nesta reabertura.

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T10:45:03-03:00`; `git diff HEAD --stat`:
`45 files changed, 3053 insertions(+), 497 deletions(-)` mais este receipt
untracked.

| Comando | Resultado final |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_flush_checkpoint_atomico_move_todo_o_sufixo -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 240s cargo test -q -p typst-core p1292_ -- --nocapture` | PASS — 15/15 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_bottom_float_ -- --nocapture` | PASS — 2/2 |
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p245_place_float -- --nocapture` | PASS — 3/3 |
| `timeout 240s cargo check -q -p typst-core` | PASS; warnings preexistentes |
| `timeout 300s cargo build -q --workspace` e rebuild focal `typst-wiring` | PASS; warnings preexistentes |
| `cargo fmt --all -- --check` e `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v1,v2,v15,v26` | PASS — zero violações |
| `timeout 180s crystalline-lint . --checks v5` | owners D PASS; resíduos fora de D somente em `cancel.rs` e `math_cancel.rs` |
| compilações bilaterais próprias sob timeout + `pdftotext -bbox` | PASS — valores da tabela final |

Hashes desta evidência supersedente:

```text
fe29f773a7e59bfcc2f59b479f5c2e7072797a315a86e06a16e7fa6315412454  01_core/src/compiler/layout/mod.rs
e10095147b7067c9fc738faa8c9f2a19472bba8ffbd6858f6601d06b8650e8f1  01_core/src/compiler/layout/cursor.rs
b691100273d02fc4cc95b1173bce23760158b52db851910bfcf5b6d98ea83887  01_core/src/compiler/layout/place.rs
54b2d5e97e1d009151efbae116db6058356dcf96cf73fa151e48891acbb5a105  01_core/src/compiler/layout/flush.rs
e15ee67575bd87d6770927086ab979b3fe30d8addf348eb90b2710a2a28966df  target/debug/typst
0b3a5b8a81f26f0969ec5a4cb7ba420dbd81fbc4e3c9ec15096de7f64213f20f  /tmp/p1292-v11-public-shape.typ
e43d308f83a2ca892fe47ba45fa3e71d1e6087c34df4b38a91f414ebe973c79a  /tmp/p1292-v11-origin-candidate2.pdf
1e2d361762ab242c06baaf7a6b7124d2b7fba4cdc5809393b09e4a28f72c14f5  /tmp/p1292-v11-vanilla-refute.pdf
```

Esta seção supersede quaisquer números ou alegações de fecho v11 posteriores
mantidos abaixo apenas como histórico auditável. Parada antes de novos ataques
e veredito.

Este recibo v11 sucede o relato histórico v10 mantido abaixo. O implementador
não leu, executou nem editou `04_wiring/tests/p1292_contract.rs`; não editou
testes independentes ou L0, não executou `--fix-hashes` e parou antes de
ataques e veredito. O filesystem era compartilhado, portanto a segregação é de
papel/capacidade e não uma alegação falsa de isolamento técnico.

## Diagnóstico e implementação v11

A medição pública por comportamento isolou a ordem causal: o marcador realiza
o float-prefixo e reduz a região; o Place não-float posterior já entra em
`current_items`; o Block termina sem rejeição imediata; somente quando o
float-sufixo entra no buffer deferred o fitting torna a rejeição observável.
Um checkpoint restrito à chamada ou a `current_line` deixava esse efeito na
página anterior.

`Layouter` agora possui checkpoint persistente e privado da unidade posterior
ao marker. Páginas, items, linhas, floats e buffers confirmados não são
clonados: o checkpoint grava comprimentos de cauda e geometria escalar, e o
rollback usa `truncate` para remover atomicamente apenas efeitos posteriores à
fronteira. Estado lógico substituível é capturado por valor. `Cursor` mantém o
log de ocorrências, pede ao owner Place o fitting puro, faz commit quando a
região aceita, ou rollback, avanço normal e replay quando rejeita. Uma tentativa
já rejeitada não materializa página antes do rollback. `Place` continua dono de
fitting/reservas; `Flush` continua somente sentinela. Não há inspeção do
próximo variant, caso especial de Block ou antecipação de `FLOAT_AFTER`.

O default relativo `1.5em`, anchors, fitting, identidade, contagem de páginas e
Block foram preservados. No fixture próprio bilateral equivalente ao caso
focal, o resultado candidato final contém exatamente uma ocorrência de cada
token: `PREFLOW` p1; `FLOAT_BEFORE` p2 em `yMin=17.404`; `AFTER_MARKER`,
`AFTER_FLOW` e `FLOAT_AFTER` p3, com `FLOAT_AFTER yMin=87.404`.

## Evidência v11

Estado medido: working tree compartilhado e não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, em
`2026-09-01T10:35:39-03:00`. `git diff HEAD --stat` naquele instante:
`45 files changed, 2996 insertions(+), 497 deletions(-)`. Os quatro owners D
eram `layout/mod.rs`, `layout/cursor.rs`, `layout/place.rs` e `layout/flush.rs`;
o receipt era untracked. A árvore compartilhada também continha alterações dos
outros lotes nos seguintes paths exatos:

```text
00_nucleo/prompts/compiler/{eval.md,eval/math.md,eval/repr.md,introspect.md,layout.md,layout/cursor.md,layout/equation.md,layout/place.md,layout/tests.md}
00_nucleo/prompts/compiler/math/layout/{_comum.md,cancel.md,spacing.md,underline.md,vec.md}
00_nucleo/prompts/compiler/stdlib/{_comum.md,layout.md,structural/math.md}
00_nucleo/prompts/entities/{content.md,elements/_comum.md,elements/math_cancel.md,elements/math_underline.md,elements/math_vec.md}
00_nucleo/prompts/infra/query-helpers.md
01_core/src/compiler/{eval/math.rs,eval/mod.rs,eval/repr.rs,introspect.rs,introspect/locatable.rs,layout/cursor.rs,layout/equation.rs,layout/mod.rs,layout/place.rs,layout/tests.rs,math/layout/cancel.rs,math/layout/mod.rs,math/layout/spacing.rs,math/layout/tests.rs,stdlib/layout.rs,stdlib/mod.rs,stdlib/structural/math.rs}
01_core/src/entities/{content.rs,elements/math_cancel.rs,elements/mod.rs}
03_infra/src/{layout.rs,query_helpers.rs}
```

| Comando | Resultado |
|---|---|
| `env RUSTFLAGS=-Awarnings timeout 180s cargo test -q -p typst-core p1292_flush_checkpoint_atomico_move_todo_o_sufixo -- --nocapture` | PASS — 1/1 |
| `env RUSTFLAGS=-Awarnings timeout 240s cargo test -q -p typst-core p1292_ -- --nocapture` | PASS — 15/15 |
| `timeout 240s cargo check -q -p typst-core` | PASS; apenas warnings preexistentes |
| `timeout 300s cargo build -q --workspace` | PASS; apenas warnings preexistentes |
| `cargo fmt --all -- --check` | PASS |
| `git diff --check` | PASS |
| `timeout 180s crystalline-lint . --checks v1,v2,v15,v26` | PASS — `No violations found` |
| `timeout 180s crystalline-lint . --checks v5` | owners D PASS; resíduos fora de D somente em `cancel.rs` (`95a40caa`) e `math_cancel.rs` (`112fd354`) |
| `timeout 30s target/debug/typst compile /tmp/p1292-v11-public-shape.typ ...` + `pdftotext -bbox` | PASS — 3 páginas e cinco tokens exatamente uma vez nos destinos acima |

Hashes finais desta execução:

```text
c431d58b877e063613566a5a429391394d77143c1e3bd03300456111ec062b27  01_core/src/compiler/layout/mod.rs
4713cf3e96504eb7d104093f788444ed796ffa354c7aff7a7166315a68c62e3f  01_core/src/compiler/layout/cursor.rs
b691100273d02fc4cc95b1173bce23760158b52db851910bfcf5b6d98ea83887  01_core/src/compiler/layout/place.rs
54b2d5e97e1d009151efbae116db6058356dcf96cf73fa151e48891acbb5a105  01_core/src/compiler/layout/flush.rs
67460fdd04e6ece59ef941931a76b534b1d6a68c84ed7e0c8f92b13ebd54078c  target/debug/typst
0b3a5b8a81f26f0969ec5a4cb7ba420dbd81fbc4e3c9ec15096de7f64213f20f  /tmp/p1292-v11-public-shape.typ
a0eeff3698923b17d55b95cea25743e5ac47dd271ccfb454ba93970b5b707323  /tmp/p1292-v11-public-crystal-tail-final.pdf
```

## Parada v11

Implementação e evidência D concluídas. Parada antes do oracle protegido,
ataques e veredito, que pertencem aos papéis segregados seguintes.

---

## Histórico v10

Data: 2026-09-01
Papel: `implementador_b_erros` / lote D exclusivamente
Regime: materialização segregada completa, sem alegação de isolamento técnico
Manifest aplicado: `bce9d76d084e5aabe25d9e8208af1962008ca5982c79195eb0e2ae08f4e65c8e`
Contrato canônico v10: `3b1ead87735df2e4e7c72a84c899a45c61f45067bf75f808812e9a4f9056bd69`
Seal bruto observado: `f798bfc18ebdcd4b9bcc684f3b95d398fbb37b072f6e8ea89e0fb7676647d630`
Receipt do contrato observado: `b79af4ec4c9a5bdc59a34b28168dfb5f78c93666f8b0cbc54557bc44df96baa7`
Oracle protegido declarado: `fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`
Ack declarado pelo coordenador: `1d347e930b3288bb4fc460c3cfed5265091e14e479425d363efcbd706b3a422b`

Este recibo substitui as versões D v8/v9 anteriores. O implementador não leu,
executou nem editou `04_wiring/tests/p1292_contract.rs`, não executou
`--fix-hashes` e não produziu ataques ou veredito.

## Reabertura por progresso

O coordenador declarou que o source canônico do vetor prefix/suffix tem SHA-256
`c13d0dd5f0d8fb72afb64e1ec5387adddf24f8602a7a3c98a9c8699f16fc6da3`
e que a execução protegida excedeu 90 segundos. Sem ler nem executar esse
oracle, o implementador reproduziu a mesma semântica numa fixture própria
(`block 30pt` + bottom float `80pt` + `place.flush()` + block `10pt` + bottom
float posterior `10pt`). O source próprio tinha SHA-256
`f8a6e82d474625611821a80f71ccaf90e167ae96a5a2e32f65632fec1c0d9028`.

Antes da correção, `timeout 15s target/debug/typst compile ...` terminou com
`124` após 15 segundos. A recursão era:

```text
flush marker
  -> prefixo não cabe na região com block 30pt
  -> new_page
  -> página nova recebe apenas a baseline inicial
  -> baseline era classificada como fluxo
  -> body 80pt + clearance 1.5em recusado
  -> new_page sem reduzir a fila
  -> repetição
```

`place.rs` passou a distinguir fluxo materializado (`current_items` ou
`current_line`) da baseline geométrica. Numa região materialmente vazia,
`used_height` é zero e a regra já selada admite a primeira ocorrência pelo
corpo, incorporando a reserva completa depois. `cursor.rs` removeu os retries
defensivos redundantes: cada ciclo agora ou reduz o prefixo ou avança uma região
que será materialmente vazia e, portanto, admissível. Não há descarte,
duplicação, antecipação do sufixo nem mudança da ancoragem v10.

## Reabertura da âncora física

Depois do reparo de progresso, o coordenador declarou execução protegida em
6,99s com 10 GREEN e um único RED geométrico: `FLOAT_BEFORE yMin` esperado
`17.404±0.002`, candidato `20.000`. O estado refutado que subtraía o ascender
integral produzira `10.166`; esse mecanismo não foi restaurado.

A medição própria mostrou que `layout_sub_frame` inicia os itens na baseline,
mas o top-edge semântico local não coincide com zero. No vetor a 11pt, a
diferença `baseline local - top-edge` é `2.596pt`. A translação do cursor agora
mede essa origem física nos `FrameItem`s locais e aplica
`target_y - local_origin_y`. O target lógico usado por alinhamentos órfãos
permanece intacto. Shapes/grupos/imagens usam sua origem geométrica; texto e
glyph usam seu top-edge; linhas incluem meia espessura. Portanto a correção não
é uma constante nem uma subtração global de ascender.

Foram criados três sources próprios zero-flow para congelar o mesmo transporte
visível sem consultar o oracle:

```text
eb51a52ae6efbc7601204dbd869f59a3d287ea92dc37820e0cb22cd9448cf1fb  prefix/suffix
f7d123d7f3bd6ad6b5e1cf5b3c5d1306584631c19ea95bec19b6b50b018c9d39  top+bottom
da0fab9664f7f527cf38187f438475de58c33da6b456ee0976eb351420614969  boundary
```

Cada compilação vanilla/candidata foi executada sob `timeout 15s`, seguida de
`pdftotext -bbox`:

| Token | Vanilla `yMin` | Cristalino `yMin` |
|---|---:|---:|
| FLOATPREFIX | 17.404000 | 17.404000 |
| FLOATSUFFIX | 87.403999 | 87.404000 |
| TOPFLOAT | -2.596000 | -2.596000 |
| BOTTOMFLOAT | 52.404000 | 52.404000 |
| BOUNDARYFLOAT | 77.403999 | 77.404000 |

Todas as compilações terminaram com exit `0`; contagem de páginas, ordem,
fitting, defaults e clearance não mudaram. O bottom frame continua ancorado ao
fundo físico, sem deslocamento pelo clearance.

## Materialização v10

- `compiler/stdlib/layout.rs` materializa o default omitido de
  `place.clearance` como `Length::em(1.5)`. O valor permanece relativo até ao
  layout; valores explícitos, inclusive `0pt`, `5pt` e `1.5em`, não são
  substituídos.
- `compiler/layout/place.rs` é o owner de fitting, reserva e ancoragem. A
  admissão preserva a ordem da fila, consome somente o prefixo admissível e
  conta `body + clearance` contra fluxo existente. Numa região vazia, a
  admissão da primeira ocorrência considera o corpo para fitting e incorpora a
  reserva completa depois. Floats bottom permanecem ancorados ao fundo físico;
  a clearance fica adjacente ao fluxo futuro.
- `compiler/layout/cursor.rs` pede a estabilização da fila pelo owner Place,
  avança regiões pelo distribuidor normal enquanto o prefixo do marcador não
  zerar e conserva o sufixo posterior. Uma linha ainda não comitada é migrada
  para a nova região quando a reserva reduz a geometria efetiva. Uma linha cuja
  tinta ainda cabe não é expulsa retroativamente só porque o próximo avanço de
  baseline ultrapassaria a reserva. A ocorrência admitida é retirada uma vez da
  fila; nova estabilização não duplica a emissão.
- `compiler/layout/flush.rs` contém somente a sentinela/fronteira: captura o
  tamanho do prefixo já pendente e delega ao hook do cursor. Não cria item,
  cursor, linha ou runtime paralelo.
- O ramo block e `compiler/layout/block.rs` permaneceram inalterados.

Os headers dos quatro owners v10 usam os hashes canônicos fornecidos pelo
linter: `6e784ef0` (stdlib layout), `fe9ea0a6` (place), `6af70c85` (cursor) e
`c105f59a` (flush), todos com data `2026-09-01`.

## Fixtures próprias bilaterais

Estado: working tree não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, medição final em
`2026-09-01T09:37:46-03:00`. O binário cristalino reconstruído tinha SHA-256
`fe6b740b1f044bccc9cdca651a4cc971349bef90dce11626987fe5a916fbfd8f`.
Foram compiladas fixtures próprias, fora do repositório, tanto com o vanilla
ratificado como com esse binário.

| Vetor | Vanilla | Cristalino | Resultado factual |
|---|---:|---:|---|
| prefixo + flush + sufixo, default omitido | 3 páginas | 3 páginas | prefixo na p.2; conteúdo pós-flush e sufixo na p.3 |
| `clearance: 0pt` | 2 páginas | 2 páginas | conteúdo pós-flush na p.2 com o float |
| `clearance: 5pt` | 2 páginas | 2 páginas | conteúdo pós-flush na p.2 com o float |
| `clearance: 1.5em` | 3 páginas | 3 páginas | prefixo na p.2; conteúdo pós-flush na p.3 |
| top + bottom, `5pt` | 2 páginas | 2 páginas | top/flow na p.1; conteúdo pós-flush/bottom na p.2 |
| sem floats | 1 página | 1 página | marcador no-op |
| nested | 1 página | 1 página | sem item/cursor artificial |
| sem flush | 2 páginas | 2 páginas | o marcador não recebe crédito retroativo |
| boundary com block | 2 páginas | 2 páginas | frame bottom ancorado no limite físico |

No boundary próprio com block, o texto interno do block cristalino conserva uma
diferença preexistente de layout interno (`y=80` contra `y=90.166` no vanilla),
mas a origem física do frame bottom está no limite selado. Não houve alteração
de `block.rs` nem compensação em cursor/avanço por causa dessa diferença.

## Comandos e resultados

| Comando | Resultado |
|---|---|
| `timeout 15s target/debug/typst compile <prefix-suffix> ...` antes da correção | RED de progresso — exit `124`, 15s |
| o mesmo comando após a correção | PASS — exit `0`, aproximadamente 0,22s |
| nove compilações próprias (`prefix/suffix`, no-flush, `0pt`, `5pt`, `1.5em`, top+bottom, no-float, nested, boundary), cada uma sob `timeout 15s` | PASS — nove exits `0`; nenhuma atingiu o timeout |
| seis compilações bilaterais de âncora (três sources × vanilla/cristalino), cada uma sob `timeout 15s`, seguidas de `pdftotext -bbox` | PASS — cinco tokens nas tolerâncias acima |
| `cargo check -p typst-core` | PASS |
| `cargo build --workspace` | PASS |
| `cargo test -p typst-core p1292_d_ -- --nocapture` | PASS — 5 passed, 0 failed |
| `cargo test -q -p typst-core p1292_ -- --nocapture` | PASS — 12 passed, 0 failed |
| `cargo test -p typst-core p245_ -- --nocapture` | 4 PASS, 1 RED: `p245_place_float_com_clearance_adiciona_espaco_y` |
| `crystalline-lint . --checks v1,v2,v15,v26` | PASS — `No violations found` |
| `crystalline-lint . --checks v5` | owners D PASS; dois warnings residuais fora de D em `cancel.rs` (`95a40caa`) e `math_cancel.rs` (`112fd354`) |
| `rustfmt --edition 2021 --check` nos quatro owners v10 | PASS |
| `git diff --check` | PASS |

O RED legado P245 reporta literalmente:

```text
P245 — clearance Bottom afasta float do fundo; sem clearance y=759.8, com y=759.8
```

Essa asserção anterior exige deslocar o próprio float bottom para cima. O
contrato v10 exige o oposto: ancoragem no fundo físico e reserva da clearance
contra o fluxo adjacente. O teste não foi editado e a implementação v10 não foi
revertida para satisfazer a expectativa anterior. Portanto, os focais P1292 e
os gates arquiteturais estão GREEN, mas a suíte integrada permanece com este
conflito contratual legado explicitamente aberto.

## Hashes finais

```text
f31a13ee93df8039bf8727221c36fa64d50a0715f32814b3a8b74bd7a7a6f974  01_core/src/entities/content.rs
8c428fdaa42c300e82a35a8db2bad348e81ca809bfec5dc061ec1a8d5468105e  01_core/src/entities/elements/mod.rs
aaa045d8f64537d909eb95b201c479cbc491329463ba9c69cba2eb1a084b4d17  01_core/src/entities/elements/flush.rs
495b7865ddabc2a557ed48ab152ffa07f304fc84cf5562135abdee64f6b47aa2  01_core/src/compiler/stdlib/layout.rs
22b9ae3ff9efec1260e91c158eac00a7201f7bd86e04fa7c8653543eb94190d4  01_core/src/compiler/stdlib/mod.rs
f774abf709988864c2b0ae462f52427215fad599cbe8189453f577ce22422c75  01_core/src/compiler/eval/mod.rs
6582446811b7469bb5e632d1c4f130417d27ad30388138a22027ee3add083eb4  01_core/src/compiler/eval/repr.rs
2c8937b9c97fde51bcdef2e100ce63e1772fcd3bd4b3196e3e0e7d6c0e81202e  01_core/src/compiler/layout/mod.rs
ed7f3f2a74b84d89ba22b559ae250fa421964260cb30a9ff1f44cad2831e8b82  01_core/src/compiler/layout/place.rs
e1efb53437c88b5c02a052a8135b9dc9216888dcc6cb95fe697b0163e29c6867  01_core/src/compiler/layout/cursor.rs
54b2d5e97e1d009151efbae116db6058356dcf96cf73fa151e48891acbb5a105  01_core/src/compiler/layout/flush.rs
7bbce08cf3ff28ea1abe548d062e6cc5d188ec122073306bc6c05cbfd423e750  01_core/src/compiler/math/layout/spacing.rs
6e50fed8b66e17855c25fc81f945e8e88c888a0e8afa2c814331ad64ab3fb6ad  01_core/src/compiler/introspect.rs
7ffb49d8474cde996c0c32c7cccc929ec64935076946f332fe0bfe86b56e9cd1  01_core/src/compiler/introspect/locatable.rs
6b95408d0ce85f82c97cc54d58d35aaad8dfe1e2ea19d26546ff80c769252df3  03_infra/src/query_helpers.rs
```

## Parada

Parada antes de ataques e veredito. A execução do oracle protegido e a decisão
sobre a atualização/resselo do teste P245 pertencem ao coordenador/autor de
testes, não a este implementador.
