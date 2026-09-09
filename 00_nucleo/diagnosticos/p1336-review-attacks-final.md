# P1336 — veredicto adversarial final

Veredicto: seis famílias produtivas válidas rejeitadas pela suíte independente congelada, score 6/6 = 1.0 no fragmento. Controle C preservado. Nenhuma equivalência geral ou atestação de isolamento é inferida.

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`; agregado efetivo `p1336-attacks-final.json`, SHA `05073308aed67b16600a1090e9480fede71d7bd77ce20855166c05d165d698f3`; fonte candidata `2fa98b5a5d5c5ed05a5d6a2541a64a42452279b172a6ba9468edebbad614321d`. O checker próprio `p1336-review-attacks-audit.cjs` reavaliou recibos originais, fontes, hashes, stdout/stderr e testemunhas; resultados detalhados em `p1336-review-attacks-final.json`, com UTC por execução e pins. Nenhum artefato do adversário foi corrigido pelo reviewer.

| Família | Evidência efetiva | Discriminação observada |
|---|---|---|
| M1 | primeira rodada, recompilada | somente controles Int puro/AST falham no nome int/integer |
| M2 | R2 recompilada | somente controles Str puro/AST falham no nome str/string |
| M3 | R2 recompilada | somente Int AST falha no span 2..13 versus 6..13 |
| M4 | R2 recompilada | somente Str AST falha no span 2..12 versus 5..12 |
| M5 | R2 recompilada | somente controle Bool falha no span 7..11 versus 2..11 |
| M6 | R2 recompilada | somente controle métodos/namespaces falha na rejeição artificial de valores-tipo |

Cada M registra compilação de typst-core no workspace exclusivo e exit 101 com o sintoma esperado; controles não afetados passam. C passa os cinco testes. Os 30 pins do agregado foram conferidos, assim como os sete executáveis preservados em caminhos próprios: todos correspondem a seus hashes e possuem identidades distintas. C foi retido do cache principal intocado após execução; M1 foi retido antes da substituição por M2. Esse timing consta do recibo suplementar, sem inventar uma medição ex-ante de seus hashes.

As cinco execuções antigas M2–M6 sem recompilação permanecem Unknown instrumental nos recibos preservados e no addendum de invalidação. Elas não entram como rejeições. A rodada focal corrigiu apenas mtimes/observabilidade e concluiu sem mudar os seis snapshots, o plano, o módulo de testes ou suas expectativas. Não restou Unknown obrigatório efetivo. O custo de todas as tentativas e o histórico de invalidação permanecem no agregado.

O incidente demonstrou por que contar exit 101 sem comprovar o mutante executado era insuficiente. A revisão final usa classes discriminatórias e origens de execução, não o score automático da rodada inválida. O produto principal permaneceu com a fonte candidata congelada.
