# P1308 — implementação de origem diagnóstica em Func

Regime: executado sem atestação de isolamento técnico. Papel deste executor:
implementador dos consumers `01_core/src/entities/func.rs` e
`01_core/src/compiler/eval/closures.rs`; sem autoridade para contrato,
testes/oráculos independentes ou veredito. Outros consumers são implementados
pelo coordenador. Nenhum teste privado, patch de teste ou expectativa P1308
foi lido ou alterado por este executor.

## Entradas e predecessor causal

Contrato Func lido: `00_nucleo/prompts/entities/func.md`, SHA-256
`304b7768a6be5a81af507ad45e2f6d784eb5d96d571bb323655a18a713d9d289`.
Contrato closures lido: `00_nucleo/prompts/compiler/eval/closures.md`, SHA-256
`650ff91457330250d7b983c052b5cbd246061ef85da7ee5a7484d0295c702a4e`.
Baseline P1308 referenciado pelos L0: SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`.

A escrita aguardou o GO do coordenador após RED semântico e verificação de
integridade dos inputs. O GO informou congelamento de ensaio, sem selo pleno;
esta nota não converte essa limitação em atestação. Fonte ratificada conferida:
`lab/typst-original/crates/typst-library/src/foundations/func.rs:380–409` e
`lab/typst-original/crates/typst-eval/src/call.rs:638`.

## Mudança

Func conserva seu Arc no campo `.0` e acrescenta Span no segundo campo
privado. Os constructors de closure, nativas, elementos e plugin começam
detached. O constructor With copia a origem antes de mover a função para o
novo wrapper. Clone copia o Span; não clona a representação da função.

`diagnostic_span` e `with_diagnostic_span` são restritos à crate. O segundo
preenche somente quando a origem anterior está detached. Nenhum helper
consulta Source, World, AST, nomes, valores ou identidade de ponteiros.
Eq/Hash/Debug, acesso à representação, namespace, nome e set_name não foram
modificados pela implementação; seguem usando o Arc e critérios anteriores.

`eval_closure_expr` anexa exatamente `closure_expr.params().span()` à Func
recém-construída. Captura eager, defaults, binding, sink e execução do body
permanecem nos caminhos anteriores. A expressão de parâmetros continua
disponível na construção; nenhuma origem é reconstruída na chamada futura.

## Recibo de autoria

Em `2026-09-07T20:58:59Z`, sobre HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` e working tree compartilhado não
commitado, os consumers antes do resselo de linhagem pelo coordenador eram:

- Func: `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde`.
- Closures: `8bec8fa5a99a8604241ec7cae3613485cc6b9f262e42b8f661be282e07975e42`.

Executados: rustfmt focal com skip_children e `git diff --check` dos dois
consumers, ambos sem erros. O diff produtivo foi revisto para conferir os
limites descritos acima. Nenhum build integrado, commit, stage ou resselo L0
foi realizado por este executor. Os consumers foram congelados e entregues
ao coordenador para build, testes independentes e lint. Esta nota não declara
GREEN, equivalência geral nem preservação atestada.
