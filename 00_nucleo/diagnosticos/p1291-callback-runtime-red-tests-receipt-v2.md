# Recibo RED independente v2 — P1291.cancel-angle-runtime

## Papel e regime

- Papel: Testador A v2, após vereditos `REJECTED` comunicados pelo coordenador.
- Regime: A/B independente, com testes derivados somente dos Prompts L0 protegidos pelo selo, dos testes anteriores do Testador A e dos blockers comunicados.
- Atestação: `executed_without_technical_isolation_attestation`; o filesystem é partilhado. Não foi lido código da implementação candidata.
- Escrita restrita a `01_core/src/compiler/math/layout/tests.rs`, `04_wiring/tests/p1291_callback_runtime.rs` e este recibo.
- Nenhum código produtivo, L0, Núcleo, manifesto, selo ou adendo foi editado.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Snapshot: `2026-08-31T17:18:54-03:00`.
- Working tree partilhada e não commitada: 350 entradas em `git status --short`.
- SHA-256 da saída literal de `git status --short`: `d5c943892ee2444a1fea35362045bc3ca8a743a59ceebcdf61e4d3c803b5d621`.
- Estado dos testes no snapshot: `M 01_core/src/compiler/math/layout/tests.rs` e `?? 04_wiring/tests/p1291_callback_runtime.rs`.
- Selo consumido: `59650dfc24435e73892ee1d4346b66438349a0b3c679bc63130eb2de87082179`.
- Os hashes dos L0s relevantes continuaram iguais ao selo: callbacks `a305478928d785027e521cc9f2d7dbcedf6895fd8207d3501b595a97d32601dd`, layout `f4bd917d3c8256392359a44ba462f48c84ad2a976acebabbad9aa57a36ac865c` e pipeline `28fb5f18d4f6d454de32cb6120c0ac2919d802a5d1aa85261b466872d632df3b`.

## Ataques adicionados

| Mutação/obrigação | Teste | Resultado atual |
|---|---|---|
| `cross` entrega defaults diferentes às duas callbacks | `p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada` | **RED** |
| `cross` deixa `inverted` alterar o conjunto morfológico das duas linhas | `p1291_cross_sobrepoe_inverted_na_geometria_final` | preservado |
| store aceita mesmo ID com `Func`, default ou estilo colapsado diferentes | `p1291_store_mesmo_id_mas_func_default_ou_style_diferente_e_stale` | preservado |
| callback aninhada em script vê size léxico/default em vez do `TextStyle.size` efetivo 0.7× | `p1291_callback_aninhada_em_script_observa_size_efetivo_reduzido` | **RED** |
| erro tardio produz ficheiro ou panic | reforço de `p1291_callback_runtime_erro_preserva_span_da_chamada` | preservado: erro, sem output, sem panic |
| `Pending` transporta documento | `p1291_pending_nao_transporta_paged_document` | preservado |

A comparação de `cross` é morfológica: extrai e ordena os elementos `<line>` do SVG. A primeira versão comparava bytes integrais e foi descartada porque a ordem das duas linhas equivalentes não prova diferença de linguagem.

## RED 1 — mesmo default positivo em `cross`

```text
$ cargo test -p typst-core p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada -- --nocapture
running 1 test
thread 'compiler::math::layout::tests::p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada' panicked:
as duas chamadas de cross recebem exatamente o mesmo default positivo
test result: FAILED. 0 passed; 1 failed; 5348 filtered out
exit status: 101
```

O teste exige duas requests com mesmo equation/occurrence, linhas `[0, 1]`, ambos os defaults positivos e numericamente iguais. A candidata entrega defaults distintos às duas chamadas.

## RED 2 — style efetivo dentro de script

```text
$ cargo test -p typst-wiring --test p1291_callback_runtime p1291_callback_aninhada_em_script_observa_size_efetivo_reduzido -- --exact --nocapture
running 1 test
assertion `left == right` failed: script reduz 20pt para 14pt (0.7×) e esse TextStyle efetivo deve chegar à callback
test result: FAILED. 0 passed; 1 failed; 4 filtered out
exit status: 101
```

O fixture fixa o size léxico em 20pt, coloca `cancel` num script 0.7× e faz a callback escolher ângulos distintos para 14pt, 20pt e qualquer outro valor. O SVG observado corresponde ao ramo não efetivo, não ao controlo explícito de 14pt/0deg.

## Controles preservados, realmente selecionados

```text
$ cargo test -p typst-core p1291_store_mesmo_id_mas_func_default_ou_style_diferente_e_stale -- --nocapture
running 1 test
test result: ok. 1 passed; 0 failed; 5348 filtered out

$ cargo test -p typst-wiring --test p1291_callback_runtime p1291_cross_sobrepoe_inverted_na_geometria_final -- --exact --nocapture
running 1 test
test result: ok. 1 passed; 0 failed; 4 filtered out

$ cargo test -p typst-wiring --test p1291_callback_runtime p1291_callback_runtime_erro_preserva_span_da_chamada -- --exact --nocapture
running 1 test
test result: ok. 1 passed; 0 failed; 4 filtered out

$ cargo test -p typst-core p1291_pending_nao_transporta_paged_document -- --nocapture
running 1 test
test result: ok. 1 passed; 0 failed; 5348 filtered out
```

Uma tentativa anterior com `--exact` no filtro unitário selecionou zero testes porque o nome real inclui o módulo Rust completo; ela não foi usada como evidência. Todos os resultados acima mostram explicitamente `running 1 test`.

## Unknown preservado

- **Tipo exato da entrypoint compatível:** `Unknown`. O contrato selado exige `SourceResult<PagedDocument>`, mas não nomeia a função nem sua assinatura de argumentos. Inventar um símbolo apenas para obter erro de compilação não seria teste discriminatório. Os observáveis acessíveis foram cobertos separadamente: `Pending` não carrega `PagedDocument`; erro público não cria ficheiro e não causa panic. Isso não recebe crédito pelo tipo Rust da entrypoint nem pela conversão direta `Pending => Err`.
- **Tentativa TOC rejeitada:** `Unknown`. Os L0s não expõem um harness que force e identifique uma tentativa interna de `Layouter::new` rejeitada sem consultar/inventar API candidata. Nenhum teste alegadamente verde foi criado.
- **Teto não convergente:** `Unknown`. A linguagem normal de `cancel` não altera as métricas do body com o ângulo, portanto não há fixture pública selada capaz de forçar drift infinito. O teste exigiria seam interno não nomeado pelo selo.

`Unknown` não foi convertido em sucesso.

## Integridade final

- `01_core/src/compiler/math/layout/tests.rs`: `54aa5b5017803bb829e56d0ff632f359c6824574d5eb84e9f2f17351ac4e991a`.
- `04_wiring/tests/p1291_callback_runtime.rs`: `314937fb5a078e89d9b7b8a02cd3117ec6ac22ac07f40bd13d23fc67313d1655`.
- `git diff --check -- 01_core/src/compiler/math/layout/tests.rs 04_wiring/tests/p1291_callback_runtime.rs`: sucesso, sem saída.

## Veredito desta fase

**RED válido estabelecido em duas mutações independentes.** A candidata deve ser corrigida por outro papel. O Testador A v2 encerrou sem alterar produção e sem ampliar a API por inferência.
