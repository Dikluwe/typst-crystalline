# Passo 1403 — `outline.entry.prefix()` e `page()`

## Estado congelado

- baseline cristalino: `b7804e4daa9d3dd823dba5d1b828e60c845cc515`, árvore limpa;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário cristalino: SHA-256 `fa9f0f346fb8a334f227bc26ab4e7820ed8cbf72f7246d8951f1cae88967f341`;
- binário vanila: SHA-256 `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- medição: `2026-09-14T22:50:19-03:00`;
- estado: executado sem atestação de isolamento.

## Medição antes da decisão

Com teto virtual de 512 MiB, timeout de 15 s e saída inferior a 64 KiB, o
vanila devolveu:

- `repr((outline.entry.indented, outline.entry.inner,
  outline.entry.page, outline.entry.prefix))` →
  `(indented, inner, page, prefix)`;
- no documento P1402 numerado,
  `query(outline.entry).first().prefix()` → `[1.]`;
- no mesmo documento, `query(outline.entry).first().page()` → `[1]`;
- entry direta sobre heading não numerado: `prefix()` → `none`;
- dentro de show rule, page com `#set page(numbering: "I")` → `[I]`;
- `inner()` com fill default preserva body, box fracionária, word joiner e
  página; com `fill:none`, usa `h(amount: 1fr)`;
- `indented([P], [I])` depende do outline pai e produz block com inset e
  alinhamento de prefixos.

A fixture conjunta tem SHA-256
`3d9d7f4df4191a57b817b405cbc85512fe82fd9478ae178e104ad3c807a14d18`.
A fonte ratificada é
`lab/typst-original/crates/typst-library/src/model/outline.rs:524-746`.

No cristalino P1402, `prefix`, `page`, `inner` e `indented` não existem. A
tentativa de usar `#show outline.entry` falha antes do helper: a função ainda
não é aceita como selector de show rule. Portanto esse bloqueio não pode ser
confundido com resultado negativo de nenhum helper.

## Decisão proposta

P1403 materializa apenas o primeiro fragmento contextual:

1. expor as identidades estáticas `outline.entry.prefix` e
   `outline.entry.page`;
2. `prefix()` sem argumentos retorna `none` quando o elemento não possui
   numbering; para heading numerado localizado, usa o contador hierárquico na
   Location preservada e retorna Content formatado pelo numbering realizado;
3. `page()` sem argumentos exige Location, consulta a página e o numbering da
   página no introspector contextual e retorna Content formatado; ausência de
   numbering usa padrão decimal `"1"`;
4. entry direta sem Location falha quando a operação precisa dela, conservando
   a dica de customização por show rule;
5. nenhum helper converte Content em string internamente.

O constructor permanece deliberadamente incompleto: P1404 completa `inner`,
P1405 completa `indented`, e P1406 integra entries geradas à aplicação de
`#show outline.entry`. Esta divisão é normativa; P1403 não pode alegar suporte
a show rules nem antecipar a mecânica de alinhamento global.

## Testes e ataques

- identidades têm repr `prefix` e `page`;
- entry consultada numerada devolve `[1.]` e `[1]`;
- heading sem numbering devolve `none` em `prefix`;
- page numbering romano devolve `[I]`;
- argumentos posicionais ou named desconhecidos falham;
- Location ausente nunca é substituída pela página corrente;
- mutantes: contador flat, nível ignorado, página constante 1, numbering de
  página ignorado, `none` convertido em vazio e coerção para string.

## Política de memória

- sondas: `ulimit -v 524288`, `timeout 15s`, saída máxima 64 KiB;
- build: `ulimit -v 2097152`, `CARGO_BUILD_JOBS=1`, incremental desligado,
  debug metadata desligado e 512 codegen units;
- não executar `cargo test -p typst-core --lib`;
- nenhum limite é elevado automaticamente após falha.

## Gate ADR-0127

P1403 acrescenta duas funções públicas. A implementação L1 aguarda confirmação
humana explícita deste contrato. Após a confirmação: L0 dos owners internos,
RED focal, implementação, resselo, sondas bilaterais e linter com zero
violações.
