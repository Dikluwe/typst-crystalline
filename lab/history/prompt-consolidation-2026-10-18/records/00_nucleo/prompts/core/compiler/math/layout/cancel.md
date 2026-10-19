# L0 — math/layout/cancel
Hash do Código: 8185f0f5


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cancel.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cancel.rs:21` locates `pub(super) fn layout_cancel(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Complete cancellation geometry

Consume MathCancelElem, measuring its body once. Preserve body width/ascent/
descent; strokes overlay the box and never expand its logical dimensions.
Resolve length against the body's diagonal, plus absolute length against
causal font size. Default length is 100% + 0.3em; auto direction derives
from the box. Center the stroke at
`(width/2, (descent-ascent)/2)` in baseline-relative coordinates.
Default thickness is 0.05em with current fill; explicit stroke preserves its
resolved paint, thickness and cap.

Inverted reflects direction; cross draws both directions and wins over
inverted. Background inserts strokes before body items, otherwise after.
For functional angles, construct/consume requests through
`00_nucleo/prompts/core/compiler/math/layout/callbacks.md`, using the occurrence
reserved by the dispatcher. Both cross requests receive the same measured
default; reflection follows each resolved result. No second execution runtime
or locator identity is introduced here.

For the 100% diagonal component, endpoints are (0, descent) and
(width, -ascent); additional length extends symmetrically. Preserve Auto,
explicit Angle and Func as distinct carrier states; a missing resolution is
provisional and cannot escape as successful compilation.

Acceptance covers baseline-relative endpoints, length, ordering, explicit
stroke, cross/inverted precedence and unchanged body dimensions.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/cancel.rs` nas linhas 21 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `layout_cancel` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O despacho produtivo usa layout_cancel_elem com carrier completo e occurrence; o wrapper conserva sua fixture histórica sem novo runtime. O import MathCancelExplicit é exclusivo desse wrapper e também deve ficar sob cfg(test); conservar MathCancelAngle e MathCancelElem produtivos.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.


## Operações numéricas puras — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/cancel.rs` nas linhas 60, 63, 96, 97 contém as chamadas sinalizadas como disallowed_methods pelo Clippy vigente. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` conserva fonte, owner, configuração, libm, compilador, UTC, status e diffstat. A sonda host-math compara os backends antes da escolha: 4968 casos, 69 diferenças de até 1 ULP em trigonometria/atan2, sem mudança de classe numérica ou sinal de zero; potências e hypot coincidem na amostra. É medição deste host, não prova universal nem nova paridade contra vanilla.

### Decisão e aceitação

Usar libm::hypot, atan2, sin e cos na diagonal, ângulo automático e projeção dos endpoints. Conservar a ordem atan2(width,height), sinais, centro baseline-relativo, length, cross/inverted/background, stroke, callback e dimensões lógicas do body. Não modificar política de não finitos nesta troca de backend numérico.

Mudança interna em fluxo contínuo ADR-0127, sem contrato público novo, default ou fase. Preservar testes de geometria/linguagem existentes e comparar os exemplos SVG medidos antes/depois, declarando diferenças. A comparação geométrica não certifica todos os casos ou plataformas. Validar build, todos os targets, core, Clippy e linhagem. Não suprimir o lint nem substituir aceitação por identidade de bytes do compilador.
