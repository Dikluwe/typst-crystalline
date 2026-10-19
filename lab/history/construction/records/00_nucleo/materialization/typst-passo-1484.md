# Passo 1484 — restaurar o gate `bundle` sem dar crédito falso a `asset`

**Estado:** P1484 materializado após confirmação ADR-0127; constructor e target bundle permanecem RED até P1485.
Passo **parcial**: P1485 completa `asset` e o target bundle.


**Baseline:** HEAD limpo `90aad760ba472cffb70063baa684c461a240e447`
(2026-09-22), vanilla pinado `586e1bd43`. Binários e hashes na auditoria
de P1480 §1.

## 1. Medição anterior à decisão

Sem feature, `type(asset)` no vanilla falha com `cannot access variable
'asset' because the 'bundle' feature is not enabled`; no cristalino retorna
`function`. Com `--features bundle`, o vanilla retorna `function`.
`01_core/src/entities/compiler_features.rs:5-35` só tem `Html` e
`A11yExtras`; `02_shell/src/cli.rs:167-185` aceita o argumento `Bundle`
mas o descarta; `01_core/src/compiler/eval/mod.rs:8327` registra `asset`
incondicionalmente. O núcleo
`00_nucleo/prompts/_nuclei/compiler-feature-gates.toml` exige default
vazio e ausência de binding gated quando desligado.

Contra a hipótese cômoda de que basta esconder o global, a fonte vanilla
`lab/typst-original/crates/typst-bundle/src/format.rs:160-240` define
`asset(path, data)` com bytes/string e saída no target bundle. O cristalino
`01_core/src/compiler/stdlib/structural/document.rs:76-110` implementa
`asset(path, kind:?)` como placeholder. O L0
`compiler/stdlib/structural/document.md:10-35` o chama de extensão:
precisa ser **retificado**, não tratado como contrato vanilla atual.

Classificação: feature gate, assinatura e saída de bundle são semântica
pública; o placeholder não satisfaz o contrato. Inferência: separar
infraestrutura do gate da substituição do constructor evita publicar
“MATCH” prematuro. A refutação seria uma sonda bilateral que provasse
equivalência do constructor atual — a fonte acima já a contraria.

## 2. Contrato desta fatia

1. Adicionar `Bundle` ao conjunto efetivo de features e fazer `--features
   bundle` transportá-lo de CLI por wiring/pipeline até eval, independente
   de `Html`/`A11yExtras` e do formato de saída.
2. Sem `bundle`, o binding global `asset` não é utilizável e seu diagnóstico
   segue a superfície medida; não basta omitir o nome se o vanilla fornece
   diagnóstico gated ao acesso. Preservar os outros perfis.
3. Com `bundle`, **não** expor o placeholder `asset(path, kind:?)` como se
   fosse o vanilla. P1485 substitui a assinatura e torna o target funcional.
   Se a infraestrutura permitir manter a rota temporariamente bloqueada,
   ela permanece explicitamente RED neste passo; não marcar o perfil bundle
   como MATCH.
4. Atualizar inventário/matriz para o quinto perfil `bundle` e suas
   combinações relevantes. Caso desativado não recebe crédito de paridade.

## 3. L0 e gate humano

Owners iniciais: `entities/compiler_features.md`, `shell/cli.md`,
`compiler/eval.md`; auditar wiring/pipeline/testes efetivamente alterados.
Atualizar os prompts e pins do núcleo antes de código, mantendo V15/V26.
O L0 de `compiler_features` deve declarar que P1484 só transporta o gate e
P1485 conclui a funcionalidade; o de `eval` deve declarar a mesma divisão
para `asset`. O L0 antigo de `structural/document` deve registrar que o
placeholder não é paridade vanilla, sem legitimar sua publicação gated.

Adicionar variante pública `Feature::Bundle` e mudar default visível de
`asset` exigem **paragem para confirmação humana após L0/hash** (ADR-0127).
Depois, RED→GREEN para transporte, independência de features e ausência no
default, com teste RED ainda documentado para `asset(path,data)` no bundle.
Fechar só se `cargo build`, testes afetados, `crystalline-lint .`,
V5/V15/V26 e diff estiverem limpos; reportar P1485 como dependência aberta.

## 5. Auditoria L0 pré-confirmação (2026-09-23T20:33:30-03:00)

Estado atual: HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree
não commitada. Vanilla ratificado `586e1bd43` (binário SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`);
cristalino `target/debug/typst` SHA-256
`58236613b1ba682a55b23074f7500bbea2b21c0b06c989e0df0fca2bb75691c8`.
A matriz bilateral dos oito perfis, o controle de shadowing local e a
divergência real de `asset(path, data)` estão em
`00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md`. O inventário
`typst-cobertura-vanilla-vs-cristalino.md` retira o crédito falso P397.

Os L0s propostos foram medidos antes da decisão e declararam P1484 parcial:

```text
entities/compiler_features.md                   e736ea4fcea07c6e944f9de4994203a27816d5425ff3f1410cf92fceadb0e490
shell/cli.md                                    59c44bd96cdc5a4ae6ad8450472489c83c9758e08ee9d1ae89ce828a59a944ef
compiler/eval.md                                e22040e57e51f037362d772b938eb73acc52fd4aa8f9996bf72a81fc261854b9
compiler/eval/bindings/field_access.md          594f75b75b96e88d82a3769c1672659409bc3962d04b839ccaf03a458bb37c3d
compiler/stdlib/structural/document.md         bc7fb30050a8ca550e49e5ff4934d1f2724f1e48499e25a8574de90906cc73f4
```

Auditoria de transporte: `02_shell/src/cli.rs:760,856` já entrega o set
a `RunIntent`; `04_wiring/src/main.rs:572,779-875,1019-1028` já passa
o mesmo set aos caminhos compile/eval; `03_infra/src/pipeline.rs:91-122,
821-840` já o encaminha ao eval. Não há mudança de assinatura ou fase
necessária em L3/L4. O owner adicional `bindings/field_access.md` cobre
`std.asset`, ausente do enunciado inicial. Nenhum Núcleo Tekt foi alterado;
os pins vigentes permanecem válidos.

`crystalline-lint --checks v5,v15,v26 --fix-hashes .` terminou com zero
drift e sem falha de ownership/integridade; os cinco arquivos Rust
correspondentes mudaram somente em `@prompt-hash`, sem código funcional.
`git diff --check` passou. `git diff HEAD --stat` neste instante:
11 arquivos rastreados, 152 inserções e 28 remoções. Lista exata:

```text
00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/stdlib/structural/document.md
00_nucleo/prompts/entities/compiler_features.md
00_nucleo/prompts/shell/cli.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/stdlib/structural/document.rs
01_core/src/entities/compiler_features.rs
02_shell/src/cli.rs
```

Este passo e a matriz são adicionalmente não rastreados; P1485–P1488
continuam não rastreados e intocados. O dono confirmou o L0/gate ADR-0127
em 2026-09-23T20:38:18-03:00, antes do código funcional.

## 6. Materialização e validação (2026-09-23T20:53:31-03:00)

Proveniência: HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`;
working tree não commitada. Teste RED `p1484_asset_default_gate_and_local_shadowing_red`
falhou antes da implementação: `asset` resolvia o placeholder.
`Feature::Bundle` foi adicionado ao set canônico; CLI compile/watch/eval
passa a transportá-lo. A stdlib deixou de registrar o `native_asset`
legado; os acessos globais e `std.asset` sem feature recebem o gate
exato, sem bloquear homônimos locais nem Dicts.

Verificação GREEN: `cargo test -p typst-core --lib p1484_` passou;
`cargo test -p typst-shell p1484_` passou; suíte L1 completa
`cargo test -p typst-core --lib` passou (6138 testes); suíte L2
`cargo test -p typst-shell` passou (84 unitários e 3 de integração).
`cargo build` e `crystalline-lint .` saíram com status zero; este
último ainda reporta avisos informativos preexistentes, sem violations.
V5/V15/V26 e resselo: `crystalline-lint --checks v5,v15,v26 --fix-hashes .`
terminou com zero drift. A CLI pós-build mediu os oito perfis na matriz
`00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md`.
Binário `target/debug/typst` SHA-256
`459e434d51e97039491134203b64b8ccd2dbbdb227706f142e54942b92874607`.

L0s finais (SHA-256 completo):

```text
entities/compiler_features.md                   112cf905311abafa57a848068184b2819794364e5bc9bfbc9346825e35659449
shell/cli.md                                    7ada76a83bbac241b27f9d0584fd2a966393c5df8a25820b820e45804b1a42c1
compiler/eval.md                                c1ba002fcab30e8142991b3263e2ad92712193da6d63fd3daede341bc197dba0
compiler/eval/bindings/field_access.md          907662b5ce64f1fba97b0d1c6836d969fd7351d34bc4ac4fc341a892a67265d2
compiler/eval/tests.md                          0e2ea86317d3af86f5bb51497d52141d7ba4b704ad84c0de19297f8e12626469
compiler/stdlib/structural/document.md         85796dad26a9d38494af06326d1b63898f87a23af8809fd57cb86333e8bd8661
```

Arquivos rastreados alterados durante a medição final (`git diff HEAD --stat`):
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`;
`00_nucleo/prompts/compiler/eval.md`,
`compiler/eval/bindings/field_access.md`, `compiler/eval/tests.md`,
`compiler/stdlib/structural/document.md`, `entities/compiler_features.md`,
`shell/cli.md` (todos sob `00_nucleo/prompts/`);
`01_core/src/compiler/eval/bindings/field_access.rs`,
`01_core/src/compiler/eval/mod.rs`, `01_core/src/compiler/eval/tests.rs`,
`01_core/src/compiler/stdlib/structural/document.rs`,
`01_core/src/entities/compiler_features.rs`, `02_shell/src/cli.rs`.
Este passo e a matriz são novos não rastreados. Os rascunhos não rastreados
P1485–P1488 pertencem ao usuário e não foram alterados. Sem commit neste
passo, pois o pedido atual foi implementar; P1485 continua aberto.

## 7. Regressão de teste legado e fechamento (2026-09-23T21:15:08-03:00)

A corrida ampliada `cargo test --workspace` alcançou a suíte L3: dos
1052 testes, 1051 passaram e um teste P397 falhou. Era
`integration_tests::integration::asset_placeholder_nao_emite_frames_no_pdf`,
que fazia `unwrap()` de `#asset("logo.png")` sem `bundle`. Essa
expectativa contrariava a decisão P1484 já confirmada. Antes de alterar
o teste, o L0 proprietário
`00_nucleo/prompts/infra/integration_tests.md` foi atualizado com a
medição e decisão; o teste passou a exigir a mensagem e os hints do gate
por `SystemWorld`. `cargo test -p typst-infra --lib
p1484_asset_sem_bundle_emite_gate_no_eval` passou. Hash SHA-256 final do
L0: `20c35c3b710911da7f7b1439afa547fefbb9f784b27f257b093dc421694c484f`;
`Hash do Código` do consumer: `651faa9c`.

O runner completo de wiring avançou por vários contratos, mas foi
interrompido em contratos CLI antigos e demorados (`exit 130`); logo
`cargo test --workspace` **não** tem resultado GREEN final. A evidência
direta P1484 permanece: suítes L1/L2 completas, teste L3 corrigido,
oito sondas CLI, `cargo build`, `crystalline-lint .`,
V5/V15/V26 zero drift e `git diff --check`. Depois da retificação
L3, build e lint foram repetidos com status zero.

À lista de arquivos rastreados da §6 juntaram-se apenas
`00_nucleo/prompts/infra/integration_tests.md` e
`03_infra/src/integration_tests.rs`. HEAD permanece
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree não
commitada. P1485–P1488 não foram editados.
