# P1215 — fechamento dos spans diagnósticos de `bytes`

**Resultado:** `BYTES DIAGNOSTIC SPANS GREEN — LANGUAGE-BYTES DECLARED-CLOSED`  
**Data:** 2026-08-26  
**Regime:** protocolo completo numa única sessão, **sem atestação de isolamento**.  
**Vanilla ratificado:** `a51e02804`.

## Veredito

Os 43 casos P1214 são byte-idênticos no `typst eval`: 22 valores e 21
diagnósticos, incluindo mensagem, path lógico, linha, coluna e extensão. Sete
sondas em ficheiros compilados também coincidem em região; uma delas desloca a
mesma chamada para a linha 4. A relação `language-bytes` foi promovida para
`declarada-fechada` e `bytes-methods` para `RESOLVED`.

O fechamento é limitado ao `Bytes` público medido: constructor/cast já
existentes e `len/at/slice`. Não declara equivalência geral da stdlib nem inclui
`world_types::Bytes`.

## Medição causal

A hipótese P1214 de span destacado era incompleta. Em `compile`, o span chegava
ao renderer, mas sempre como lista de argumentos. O vanilla exige:

- chamada inteira para ausência e out-of-bounds;
- positional específico para cast e excedente;
- named completo para argumento desconhecido.

Em `eval` havia uma segunda causa: `eval_expression` chamava `native_eval` com
`Args::positional`, portanto parseava com âncora detached, e o `SystemWorld` de
eval mantinha uma source principal vazia. O renderer L2 estava correto: omitia
labels que não podia resolver.

A correção preserva metadados AST em `CollectionCallSpans`, tipo interno
`pub(crate)`, sem alterar `Args`. `eval_expression` agora avalia uma `Source`
code numerizada; L4 fornece uma source idêntica e transitória ao formatter.
Nenhuma mensagem, valor, método público ou fase mudou.

## RED, ataques e isolamento

Antes da implementação, quatro testes focais falharam porque
`Source::span_byte_range` devolvia `None`. Depois, 4/4 passaram; as regressões
P1214 também passaram 4/4.

Os 12 mutantes dirigidos possuem testemunha em `p1215-ataques.tsv` e foram
rejeitados: `mutation_score = 12/12 = 1.0` para o conjunto declarado. Não foi
usado motor externo de mutation testing e a mesma sessão teve acesso a
contrato, implementação e veredito; portanto o score é reproduzível, mas não
independentemente atestado. Spread de argumentos fora da matriz permanece
`Unknown` e não foi convertido em sucesso.

## Proveniência

Congelamento inicial em `2026-08-26T15:11:27-03:00` sobre HEAD
`dc47c9c32b8b6769a58622c98b885cb094337508`. Durante a execução, o HEAD do
workspace avançou externamente; os gates finais em
`2026-08-26T15:26:36-03:00` foram executados sobre
`e7b962921c7fc5a13a4000da452e9dfb4469a245` + working tree P1215.

- vanilla SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino release SHA-256 `3ab23bff36e3f8391c95b53982acff62d4a26d465d5ba7cf7c28b0beaa549a4c`;
- lente SHA-256 `9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`;
- mapa SHA-256 `89f8b8c0546152a0ffd50330d3ac0c81c2964c229c8dd859da999a4b7ebae017`;
- oráculos SHA-256 `a7f02e89a29e90c88b9bb6590cc5793c41b1662050bc93f375edbf61dadc7696`;
- resultados SHA-256 `964d308b85f5843540c18579c2354fb099ef6a305afd176aa22e8be0f834e8a8`;
- pipeline causal SHA-256 `8fc25666e6a94480089bdebee0f2cdb1a5da8d4e04aba7a7168609c3f0ff5b25`;
- ataques SHA-256 `186edc5bd6ee1ab9d0267cc5cb2dcdd509c84ea388891e61ac04405fd190ffeb`.

## Gates

- P1214: 4/4; P1215: 4/4;
- A/B `eval`: 43/43 byte-idênticos no build release final; a rodada anterior
  produziu o mesmo veredito integral;
- A/B `compile`: 7/7 regiões idênticas; o controle `eval` com UTF-8 na linha
  anterior também foi byte-idêntico;
- `cargo build --workspace --quiet`: passou;
- `crystalline-lint .`: passou sem violations;
- `cargo fmt --all -- --check`: passou;
- `git diff --check`: passou;
- lente com mapa: duas execuções byte-idênticas, SHA-256
  `67c75e28ad9f5e2e2b5966f7b0b62188772a7b32aaec91fba35604fd0f8d5f6a`;
- `cargo test --workspace`: voltou a ter o timeout conhecido de 20 s em
  `p1137_watch_dependencias_recuperacao_e_filtro`; repetição isolada passou
  1/1. Nenhum teste focal ou restante suíte anterior ao teste temporal falhou.

## Próximo cluster

Retornar à fila P1213. O próximo cluster é `operator-diagnostics`: preservar
operador e representação dos operandos nas mensagens `bad_in` e comparação de
lengths já medidas em P1212.
