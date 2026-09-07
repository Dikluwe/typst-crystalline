# P1308 — entrada independente para o contrato

Estado: medição anterior ao candidato; não é veredito de implementação.
Regime: executado sem atestação de isolamento técnico. Escritas deste papel
limitadas a p1308-measure*, p1308-contract* e posterior patch de testes.

## Proveniência

Medição iniciada em `2026-09-07T20:36:41.556799+00:00`, HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` mais working tree registrada
integralmente em `p1308-baseline.json`, SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`.
O recibo de medição incorpora esse state (incluindo diff/stat); não usa a
árvore candidata futura como fonte de expectativas.

- Baseline imutável `/dev/shm/p1308-baseline.XzjFgA/typst`, SHA-256
  `16aeeec4783aa8fa66821a50a3d1a1d4fdebc279602253a1bf731bcb6c8eb7be`.
- Vanilla ratificado `a51e02804`, `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Script `p1308-measure.py`, SHA-256
  `fafb16d8a6df00486e9bc3f99ce3d7c5065de88e5a73c8c5dacf3fb2fe4ec4dd`.
- Resultado `p1308-measure.json`, SHA-256
  `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`.
- Oráculo R6 reutilizado sem alteração, SHA-256
  `99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6`.

São 59 casos × 4 perfis × 2 binários × 3 ordens = 1416 processos. As ordens
normal, reversa e repetida têm zero instabilidade de exit/stdout/stderr.
Todos os casos usam `eval --format json`; fixtures, argv, saídas literais e
ranges ficam no JSON. O primeiro pedido de execução expirou na revisão de
permissão antes de criar processo; a repetição autorizada executou. Isso não
foi RED nem tentativa de adaptação semântica. Os ranges derivados são apenas
de fixtures ASCII; stderr original é autoritativo. Trace sem underline guarda
início, sem inventar fim. Não foi executada nova matriz contextual.

## Fonte medida antes da decisão

Fontes vanilla ratificadas, não candidato:

| Fonte | SHA-256 | Evidência |
|---|---|---|
| `foundations/args.rs` | `b681b149809326f2479b99966232680771f8d95a170c82180d30cbd22274849b` | 408–412 cast bool em test.span(); 455–459 Args repr integral |
| `foundations/repr.rs` | `ea8f25eaf5c3357985d023014f20b5c64cc8f0813997198cb61032f7d7216fe0` | 173–222 limite 50 bytes das peças e indentação |
| `foundations/func.rs` | `a119f7e8aae459a8358357f398d44c48bc24783b67891ff3901b8d4b043c6a38` | 380–389 span não substituído; 405–409 With preserva origem |
| `typst-eval/src/call.rs` | `cb2fa9dfa313b60d39aae320d90f161685fe9bdfbc5ac7aa71419b36447f2862` | 638 origem nos parâmetros de closure |

Os paths foundations são relativos a
`lab/typst-original/crates/typst-library/src/`; call.rs é relativo a
`lab/typst-original/crates/`. `introspection/location.rs:137–140` imprime
`location(..)`. A medição contextual anterior R6 permanece a prova binária
de Location; não substituir uma fixture de query por construção fictícia.

## Resultados e consequências refutáveis

1. Os 14 casos R6 pendentes continuam divergentes no baseline atual. Oito
   factories With/spread/filter/map e dois encoders com value_span detached
   preservam o erro primário correto, mas perdem o trace. Dois callbacks de
   panic preservam primeira ocorrência 11, porém ancoram a lista de args em
   vez da chamada. Filter não-bool perde origem e usa nome curto (`int`).
   Args+none continua rejeitado. Nenhuma expectativa R6 foi reinterpretada.
2. `filter.direct` ancora `value` em 20..25; alias, spread Args e spread Array
   ancoram `(value)` da definição em 17..24. With ancora os parâmetros
   `(prefix, value)` em 17..32. Portanto origem de uso, body e callback
   argument value_span não bastam. Um carrier privado da Func que conserve
   a origem já existente é coerente; preencher toda nativa com o span do
   identificador é refutado pelo controle `filter.native`: `str` produz
   diagnóstico bool detached, com trace filter. `filter.empty` não executa
   callback e permanece `arguments()`.
3. `panic.direct`, `.alias` e `.with` exigem chamada inteira respectivamente
   0..14, 20..33 e 34..40; wrappers em callbacks continuam apontando à chamada
   efetiva dentro do callback, não à definição/prebinding. Porém named
   inválido aponta à ocorrência individual (15..22, 34..41, 33..40) e With
   acrescenta trace da chamada final. O baseline possui dívida adicional
   medida: mensagem portuguesa/detached para named inválido. O novo L0 deve
   decidir explicitamente esse limite, não apresentar a dívida como paridade
   nem aplicar remapeamento genérico de todos os erros para whole-call.
4. None é identidade dos dois lados, inclusive Args vazio. `Args + integer`
   é controle negativo bilateral intacto. Não generalizar Add para todo o
   conjunto de join nem reconstruir por Args+Args, pois esse join destaca
   o span agregado.
5. Args longo **já é paritário no baseline**. Peças de 49/50 bytes (strings
   ASCII de 47/48 caracteres mais aspas) ficam horizontais; peça de 51 bytes
   passa a multiline, com dois espaços e trailing comma. Lista de 30 valores,
   named longos e Unicode também coincide, integralmente e sem elisão. O
   prefixo `arguments` não entra no limite. Não é obrigação alterar código
   produtivo de repr para esse ponto.
6. With repr já coincide bilateralmente: `(..) => ..`. A expectativa histórica
   da nativa original não pode ser reutilizada para sua aplicação parcial.

Classificação: mensagens, fontes/âncoras, traces e representações são
observáveis de linguagem (ADR-0107). Layout do carrier e método privado são
mecânica; não há autorização deste relatório para novo contrato público.
O dono escreve os L0s, e os testes novos serão congelados somente após leitura
dos respectivos contratos atualizados. Sem candidato, RED/GREEN ou mutantes
executados neste relatório.

## Migração de expectativas históricas

O recibo workspace R6 preservado identifica `repr_value_complex_types`
esperando `location(...)` e `p1305_args_fields_remain_integral` esperando
Args de 41 itens em uma linha. A correção independente deve manter os testes,
trocar somente os esperados por `location(..)` e lista integral multiline,
e retificar a expectativa With que seria alcançada depois da primeira falha
de `repr_value_complex_types`. Fonte ratificada e medições anteriores/current
suportam isso; não restaurar comportamento obsoleto do produto nem encurtar
Args. O patch sucessor deve declarar cada expectativa e origem.

## Suíte proposta

Reutilizar todos os 14 R6 sem editar o oráculo. Acrescentar em eval/tests
casos de identidade None bilateral com erro posterior/origem; panic
direto/alias/With e controles named; filter direto/alias/With/spreads/native/
empty; entrypoint com World sem a Source code (não reparar esse World, pois
isso esconderia o bloqueio P1308); Args fronteiras 49/50/51 bytes, UTF-8 e
long integral. Preservar severidade, hints, quantidade, ranges e trace.
Testes não dependem de API nova e não calculam expected pelo formatter.
