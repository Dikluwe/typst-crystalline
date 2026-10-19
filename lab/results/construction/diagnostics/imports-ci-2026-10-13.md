# Imports produtivos, imports de teste e mutabilidade local

Limpeza da composição interna que bloqueava o Clippy. Imports usados somente
por testes ficam sob `cfg(test)`; imports sem consumidores são removidos.
Não foram alteradas fórmulas, expectativas de teste, assinaturas públicas,
defaults nem fases do pipeline.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. Fontes, owners L0, UTC, status e `git diff HEAD --stat`
estão em `imports-ci-2026-10-13/before.json.gz`; estados intermediário,
pré-validação e final têm capturas separadas. Receitas, logs completos,
inventários e diffs desta etapa ficam em `evidence.json.gz`.

O inventário inicial de `rustdoc-ci-2026-10-13` contém 396 diagnósticos
distintos no core lib/testes, dos quais 62 são `unused_imports` e 10 são
`unused_mut`. A desduplicação usa código, mensagem e os spans primários
completos. Um aviso no corpo do macro pode corresponder a mais de uma
expansão; por isso esta contagem não é a quantidade de chamadas afetadas.

A suíte anterior à edição passou com 6.701 testes, zero falhas e cinco
ignorados, encerrada em `2026-10-13T23:05:07.281611+00:00`. Os mesmos arquivos
capturados no início permaneceram intactos durante essa execução.

Inspeções que determinaram a correção:

- `compiler/eval/mod.rs:55` e `compiler/layout/mod.rs:13` forneciam `Type`
  e `RegionTailCheckpoint` a testes descendentes, apesar dos avisos no build
  produtivo. A solução preserva esses imports sob `cfg(test)`.
- `compiler/stdlib/layout.rs:64-77` expunha helpers usados pelos testes de
  papel, binding, flush e miter. As implementações produtivas ficam nos filhos;
  apenas as reexportações de teste recebem a condição.
- `compiler/stdlib/mod.rs:123` e `structural/mod.rs:107` mantinham aliases
  crate-internos de `math_matrix_augment` sem consumidores. Os usos efetivos
  estão no próprio `structural/math/matrix.rs:81,163,406`.
- `compiler/stdlib/tests/support.rs:89` cria contexto mutável para muitos
  testes. Só as chamadas em `tests/introspection.rs:410,421,517` leem sem
  modificar. Remover `mut` do macro inteiro quebraria seus demais usuários.

## Decisão e implementação

Os L0 dos hubs afetados foram atualizados antes das alterações de composição:
condicionar aliases exclusivos de testes e retirar aliases internos sem
consumidores. Os caminhos públicos externos à crate permanecem iguais.
Não foi feita nova alegação de paridade nem de ganho de desempenho.

Os helpers originais de igualdade e matriz permanecem em seus owners, com
suas chamadas e testes. As ligações removidas não eram implementações.
Os demais imports usados por testes, inclusive math, CSV/CBOR e tiling,
continuam disponíveis quando `cfg(test)` está ativo.

Foram removidos qualificadores `mut` de bindings que não são reassinados.
Em boxed, a declaração redundante `let mut outer_w = outer_w` foi retirada;
as expressões seguintes continuam usando o mesmo valor original. Os três
testes de introspecção que apenas leem o contexto usam diretamente
`let ctx = EvalContext::new()`. O macro compartilhado foi preservado
byte a byte. Não foram acrescentados `allow` ou `expect`.

Os avisos `unused_variables` de layout não foram silenciados: podem indicar
lógica incompleta e exigem análise do comportamento antes de qualquer edição.

## Refinamento verificado

A primeira compilação de todos os targets rejeitou a retirada dos imports de
Content nos testes de tiling e de conversões nos testes de cor. Ambos foram
restaurados sob `cfg(test)`, sem mudar os testes. O mesmo comando revelou o
alias intermediário de `eval_content_method_at` no owner field_access; seu
L0 foi atualizado antes de condicioná-lo também.

`first-check-state.json.gz` e `check-first.log` preservam essa tentativa,
inclusive os erros. As medições finais usam a árvore corrigida capturada em
`pre-validation.json.gz`, não a tentativa rejeitada.

## Validação final

Medição consolidada em `2026-10-13T23:14:45.341867+00:00`; as fontes ficaram
intactas durante os comandos. Os resultados finais estão em `summary.json`
e `validation-results.json`, preservados nas evidências. `final-diffs.json`
delimita esta rodada em relação ao working tree inicial, sem misturar as
alterações anteriores do projeto.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 396 → 324 diagnósticos distintos; ainda falha |
| `unused_imports` / `unused_mut` no recorte | 62 / 10 → zero; demais contagens inalteradas |
| `cargo check --offline --locked --workspace --all-targets` | passou |
| `cargo build --offline --locked` | passou |
| Rustdoc workspace com `RUSTDOCFLAGS=-Dwarnings` | passou; encerrado em `2026-10-13T23:15:10.873771+00:00` |
| Suíte core antes e depois | 6.701 passaram, zero falhas, cinco ignorados |
| Lista de testes registrada | mesmos 6.706 nomes, sem adições ou remoções |
| Formatação e `git diff --check` | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | zero erros, 381 avisos, 1.619 notas |

A comparação de nomes desconta apenas o sufixo de apresentação
` - should panic` que o executor libtest acrescenta e `--list` omite.
A primeira comparação textual detectou essa diferença de formato;
`test-name-comparison.json` preserva a comparação bruta e a normalização.
Nenhum teste foi renomeado ou removido para ajustar o resultado.

## Pendências

Os demais diagnósticos do Clippy, avisos arquiteturais, lacunas funcionais
da auditoria original e a validação remota da matriz de release permanecem
abertos. A suíte core e a compilação local de todos os targets não equivalem
a executar a matriz Linux/Windows/macOS/i686/cross/Docker. Não houve commit,
push ou publicação nesta etapa.

A medição seguinte está em [fluxo de controle e testes efetivos](controle-fluxo-ci-2026-10-13.md).
