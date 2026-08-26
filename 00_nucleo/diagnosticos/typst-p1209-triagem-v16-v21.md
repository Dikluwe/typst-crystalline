# P1209 — triagem V16–V21

## Estado

Execução concluída por triagem semântica, sem perseguir zero artificial.

## Lote V21 produtivo

Os 9 avisos produtivos foram auditados contra os owners e o vanilla ratificado
`a51e02804`:

| Owner | Classificação | Proveniência |
|---|---|---|
| `compiler/layout/boxed.md` | `CITE` | calibração empírica P1119/P1120 normalizada de 11pt para em |
| `compiler/layout/stack.md` | `CITE` | fallback cristalino explícito de altura de linha 1.2em |
| `math/layout/cancel.md` | `CITE` | vanilla `math/cancel.rs:93`, default 0.05em |
| `math/layout/matrix.md` | `CITE` | vanilla `math/matrix.rs:15-16` e `math/table.rs:14,36-42` |
| `math/layout/_comum.md` | `CITE` | vanilla `math/run.rs:15`, `TIGHT_LEADING=0.25em` |
| `math/layout/underover.md` | `CITE` | fallback geométrico P1132n/p no centro do frame |

Resultado: V21 produtivo=0. Permanecem 15 V21 em fixtures de teste; são
`TEST-FIXTURE` e não receberam comentários redundantes neste lote.

## V18

- `export/builder.rs`, `0x00..=0x1f | 0x7f`: `ACCEPT-BOUNDARY`; classe de
  caracteres de controle escapados em literal PDF.
- `export/oracle.rs`, `b'0'..=b'9' | b'-'`: `ACCEPT-BOUNDARY`; léxico de
  inteiro no parser do oráculo PDF.

Enumerar cada byte reduziria legibilidade sem aumentar cobertura semântica.
Os dois sinais permanecem como inventário informativo.

## V19/V20

Baseline ratcheted mantido em V19=349 e V20=600. São métricas de condensação e
profundidade, não metas de zero. Outliers serão tocados apenas quando
coincidirem com um V16/V17 acionável no mesmo bloco.

## V16/V17 — classificação de fecho

Baseline: V16 produtivo=136/testes=74; V17 produtivo=26/testes=10. A maior
concentração produtiva de V16 está em `math/layout/attach.rs` (7),
`eval/math.rs` (6), `entities/content.rs` (6), `entities/value.rs` (6),
`layout/grid.rs` (5) e `stdlib/layout.rs` (5). Não se aplicou expansão global:
cada owner requer leitura integral e prova de enum fechado ou fronteira aberta.

A ADR-0017 do linter determina categoricamente que V16 **nunca silencia** por
citação ou exceção: o sinal vigia evolução futura do enum, e a taxonomia
N16 α/β/γ serve à revisão humana sem desligá-lo. Portanto, sua presença após
classificação não é dívida automaticamente acionável nem falha de fecho.
Wildcards que produzem erro/mensagem já são barreiras; defaults neutros ficam
como pontos permanentes de vigilância e só devem virar braços explícitos no
passo do owner que acrescentar ou reinterpretar variante.

V17 é uma heurística puramente sintática: os 36 casos usam curto-circuito como
um único predicado. Sem medição que separe diagnósticos ou resultados,
desdobrá-los aumentaria braços e poderia mudar ordem de avaliação. Ficam
`ACCEPT-EQUIVALENT`, sob ratchet; qualquer guard novo precisa justificar por
que não expressa duas decisões distintas.

Assim, não existe `REFACTOR` comprovado pendente em V16/V17 neste inventário.
Uma futura mudança de enum ou de comportamento reabre somente seus pontos de
vigilância, não todo o estoque.

## Validação do lote

- `crystalline-lint --fix-hashes .`: aplicado focalmente; reanálise com zero
  drift.
- suíte `compiler::math::layout::tests`: 230 testes GREEN.
- `cargo build`: GREEN com warnings Rust preexistentes.
- `git diff --check`: GREEN.
- Nenhuma fórmula ou comportamento foi alterado; somente L0 e proveniência.

## Fecho final

- Estruturais V5/V7/V15/V26: zero.
- V21 produtivo: zero; 15 fixtures classificadas.
- V18: 2 fronteiras de formato aceitas.
- V16=210, V17=36, V19=349 e V20=600: sinais permanentes/baselines
  informativos, não ordens de reescrita.
- Índice Git vazio.

Resultado: `GREEN POR TRIAGEM`. O projeto não ganha qualidade ao zerar estes
estoques mecanicamente; ganha ao impedir crescimento não classificado e ao
reabrir o ponto específico quando sua enum, fórmula ou fronteira evoluir.
