# P1329 — parecer do L0 antes do freeze

**Veredito: apto para congelar a obrigação e preparar RED independente.**
Não há achado bloqueante no recorte P1329. Isto não libera candidato sem RED
nem substitui gates de linhagem, suíte e preservação.

Revisor `/root/p1329_review`; regime A/B executado sem atestação de isolamento,
sem refinement seal. Leitura integral de `prompts/compiler/stdlib/calc.md`;
nenhuma alteração de L0, owner, teste ou oráculo pelo revisor.

Entrada conferida em 2026-09-09T12:17:43Z, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado:

- L0 bruto SHA-256 `d32aff8975013fc7b6dc12268696317cc065cac83fa94808c4afcc4107079153`;
- norma sem a linha `Hash do Código:` SHA-256
  `3756c7998c5a81cfdc54119b6fc152ca1f186f0f74df654e01877b73bdadbe0d`;
- owner ainda byte a byte igual a `original_owner` de `p1329-baseline.json`,
  cujo SHA-256 é `d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`.

O L0 resolve os riscos preliminares: sucessão dimensional explícita de P1328,
tabela de tipos atualizada, condição OR com zero assinado para Length,
rejeição de mistos independentemente do sinal, conservação das espécies e
ausência de normalização angular/clamp. O erro usa somente a origem real de
Args e não fabrica posição; a dívida de nome no trace externo fica explícita.
Os guards, conteúdo, saturação inteira e outras funções estão preservados.

A cláusula IEEE fica coerente com a fonte medida e com a enumeração de
`guard_float` na ADR-0101. Distingue operação nativa de construção anterior
e exige testes próprios com NaN/Inf já construídos. Assim, não usa a falha
de divisão por zero como evidência de abs. A regra de Length também governa
não finitos: componente NaN não é zero; outro componente zero permite módulo.

A suficiência de calc continua inferência refutável, como exige a norma.
Não surgem campos, métodos, assinaturas públicas ou nova fase; classificar
como correção de paridade em fluxo contínuo é compatível com ADR-0127.
As expectativas antigas só podem mudar nos controles dimensionais indicados;
os arquivos históricos permanecem imutáveis. Os novos testes devem observar
tipo/magnitude e diagnóstico integral, mantendo distinção explícita das dívidas.

Snapshot de `git diff HEAD --stat -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring`:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 187 +++++-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 +-
 00_nucleo/prompts/compiler/eval/modules.md         |  47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  78 ++-
 00_nucleo/prompts/compiler/stdlib/calc.md          | 154 ++++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 198 +++++-
 00_nucleo/prompts/wiring.md                        | 104 ++-
 01_core/src/compiler/eval/bindings/field_access.rs | 731 ++++++++++++++++++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  92 ++-
 01_core/src/compiler/eval/modules.rs               |   5 +-
 01_core/src/compiler/eval/tests.rs                 | 236 ++++++-
 01_core/src/compiler/stdlib/calc.rs                | 388 ++++++++++-
 01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++++--
 04_wiring/src/main.rs                              |  50 +-
 14 files changed, 2683 insertions(+), 106 deletions(-)
```
