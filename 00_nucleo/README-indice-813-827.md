# Índice — prompts 813 a 827 (resto da fila de P810)

Continuação da fila de 16 achados do handoff `handoff-novo-chat-p810.md`. Os achados #13 (math::attach residual) e a corrupção de PDF já foram fechados em P811 e P812. Este índice cobre os 15 restantes.

Caminho de relatório em todos: `00_nucleo/diagnosticos/typst-passo-XXX-relatorio.md` (convenção confirmada, não mais `materialization/`).

| Arquivo | Achado original (nº na tabela de P810) | Prioridade |
|---|---|---|
| `typst-passo-813-equacao-bloco-centragem.md` | #16 (de P808) — equação em bloco não centrada | |
| `typst-passo-814-typst-eval.md` | #1 — `#eval` sem mode/scope, span detached | |
| `typst-passo-815-eval-methods.md` | #2 — método inexistente diverge, dict-key-call sem hints | |
| `typst-passo-816-set-prop-invalida.md` | #3 — `#set` inválido vira warning silencioso (exit 0) | **alta** |
| `typst-passo-817-calc.md` | #4 — trigonometria inversa, quo, pow, decimal | **alta** |
| `typst-passo-818-ops.md` | #5 — ordenação e operadores ausentes | |
| `typst-passo-819-plugin.md` | #6 — `plugin.transition` ausente, mensagens L1, spans | |
| `typst-passo-820-scope-deprecation.md` | #7 — `Deprecation` ausente | **alta** |
| `typst-passo-821-target.md` | #8 — `#target()` fora de contexto não erra | |
| `typst-passo-822-grid-resolve.md` | #9 — mensagens divergentes, footer fora de posição (repeat-across-páginas continua débito, não corrigir aqui) | |
| `typst-passo-823-cbor.md` | #10 — mensagem de erro CBOR usa Debug interno | |
| `typst-passo-824-read-encoding.md` | #11 — `encoding:` rejeitado, não-UTF8 silencioso | |
| `typst-passo-825-math-classes.md` | #12 — classes, field access bare, fence, LeftRightAlternator | |
| `typst-passo-826-pdf-artifact-kind.md` | #14 — `pdf.artifact(kind:)` rejeitado | |
| `typst-passo-827-package-not-found.md` | #15 — mensagem de pacote não encontrado (não-preview) | |

Todos seguem a mesma estrutura: achado citado da tabela de P810, regra de execução mostrada, sonda (com sub-achados separados quando o achado original tinha vários pontos), implementação, validação, relatório.

Onde a sonda pede para "conferir o relatório de materialização de P810 antes de assumir o caso exacto", é porque o resumo da tabela de P810 é terso — o `.typ` exacto e os valores usados na medição original estão só no relatório de materialização completo, não neste índice nem nos prompts.
