# Passo 1404 — `outline.entry.inner()`

## Estado congelado

- baseline cristalino: `efd82c1eb08bf40c4849df53bdca8c763cecdaa4`, árvore limpa;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário cristalino: SHA-256 `9f19d7e449d8adffe2b15dce81839c7de8d6d6ddd9c360e4512c3ca64c58dbfe`;
- binário vanila: SHA-256 `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- medição: `2026-09-14T23:18:11-03:00`;
- regime: protocolo completo como ensaio, executado sem atestação de isolamento.

## Medição antes da decisão

Com teto virtual de 512 MiB, timeout de 15 s e saída inferior a 64 KiB, o
vanila devolveu:

- `repr(outline.entry.inner)` → `inner`;
- entry numerada com fill default → `sequence([H], [ ], box(width: 1fr,
  body: repeat(body: [.], gap: 0.15em)), [ ], [U+2060], [1])`;
- a mesma entry com `fill:none` → `sequence([H], h(amount: 1fr),
  [U+2060], [1])`;
- body forte continua `strong(body: [H])`, sem achatamento para texto;
- fill customizado `[-]` permanece dentro da caixa fracionária;
- argumento posicional extra falha com `unexpected argument`;
- elemento `strong` falha com `cannot outline strong` antes de fabricar inner;
- heading direto sem Location falha com `heading must have a location` e dica
  de customização por show rule.

Fixtures: P1402 principal SHA-256
`8203d182c1a5fbbda3fc7d04217b5725455107ef8314d1750431063e7c891bed`;
fill none `5ef0fd0d578879677db201a244e62cedaa4c93db5259d214c63d9388f0e5838c`;
body forte `f975c76d7526bf3e906d2c813e860b2c54647e99a1feab4b449ebf6bd3d273a3`;
fill customizado `fa55b0e85977c31b70f3a7a831bdb4ad603c5077beaf211e2cc90e096d810651`.
Fonte ratificada:
`lab/typst-original/crates/typst-library/src/model/outline.rs:629-746`.

No cristalino, a mesma chamada falha com `element entry has no method
\`inner\``. Além disso, a sonda independente `repr(box(width: 1fr)[x])`
falha porque o carrier `BoxedElem.width` aceita somente `Length`; o vanila
devolve `box(width: 1fr, body: [x])`.

## Decisão proposta

1. expor a identidade estática contextual `outline.entry.inner` e o método
   `entry.inner()`, ambos sem argumentos;
2. obter body pela mesma capacidade outlinable de `body()` e página pelo
   helper contextual P1403, sem duplicar ou contornar sua leitura causal;
3. com fill, devolver Content na ordem body, espaço, caixa inline de largura
   `1fr` contendo o fill, espaço, U+2060 e página;
4. sem fill, devolver body, `h(1fr)`, U+2060 e página;
5. sob direção RTL efetiva, envolver somente o body entre U+202B e U+202C;
6. preservar Content/markup de body, fill e página; não converter para string;
7. adicionar ao carrier interno de `BoxedElem` a largura fracionária necessária
   à morfologia retornada, mas manter `native_box(width: 1fr)` fora do escopo:
   P1404 não amplia a assinatura pública geral de `box`.

O constructor continua deliberadamente incompleto: P1405 completa
`indented`; P1406 integra as entries geradas a `#show outline.entry`. P1404 não
antecipa alinhamento global, parent outline, selector de show ou outras
capacidades de `box`.

## Testes e ataques

- identidade qualificada e método têm repr/dispatch corretos;
- morfologias fill default, `none` e customizado coincidem bilateralmente;
- body forte continua forte;
- página 2 e numbering romano atravessam `inner()`;
- argumentos extras, elemento não outlinable e Location ausente falham;
- U+2060 é obrigatório e não pode virar espaço comum;
- RTL inclui exatamente U+202B/body/U+202C;
- mutantes: fill fora da caixa, caixa sem `1fr`, espaços ausentes, `h` absoluto,
  página constante, word joiner removido, body textualizado e controles RTL
  invertidos.

`Unknown` permanece para igualdade mecânica Rust, bytes/render exatos e
integração por show rule ainda bloqueada em P1406; nenhum desses casos recebe
crédito como `Preserved`.

## Política de memória

- sondas: `ulimit -v 524288`, `timeout 15s`, saída máxima 64 KiB;
- build/check: `ulimit -v 2097152`, um job, incremental e debug metadata
  desligados, 512 codegen units e linker gold;
- não executar `cargo test -p typst-core --lib`;
- testes unitários são escritos, mas só recebem crédito por harness focal que
  permaneça dentro do teto; sondas do binário são o gate funcional principal;
- nenhum teto é elevado automaticamente após falha.

### Achado de memória durante o GREEN

A primeira sonda positiva atingiu o teto de 512 MiB. A redução causal
`#[a\u{2060}b]` reproduziu o estouro sem outline, consulta, caixa ou fração;
os controles sem WORD JOINER terminaram. A inspeção de
`03_infra/src/shaper.rs::CandidateSet::covering_all` encontrou preenchimento
denso do vetor local até o valor absoluto de um slot global. O L0 proprietário
`prompts/infra/shaper.md` foi atualizado antes da correção: fallback passa a
internar somente slots efetivamente vistos. Esta é correção de fórmula interna
em fluxo contínuo, não expansão do contrato público P1404.

## Gate ADR-0127

P1404 acrescenta a função pública `outline.entry.inner`. A confirmação humana
explícita foi recebida em 2026-09-14. Os L0 internos precedem o código; resta
fechar o GREEN limitado, ressellar hashes, lintar e commitar atomicamente.

## Resultado da materialização

Medição final em `2026-09-14T23:42:44-03:00`, sobre o baseline
`efd82c1eb08bf40c4849df53bdca8c763cecdaa4` com working tree não commitado
(os ficheiros exatos constam no commit deste passo), binário cristalino SHA-256
`144267dc07596f4f0938881c38644d54955d4190128daf48da395c268950bfba`:

- oito oráculos `typst eval --in` terminaram em ambos os binários e foram
  byte a byte iguais: identidade, default, fill none, body forte, fill
  customizado, página 2, romano e controle RTL;
- dez compilações focais cristalinas terminaram sob 512 MiB/15 s, incluindo
  o controle mínimo U+2060 e o render direto de `inner()`;
- predicados no contexto real confirmaram U+202B/U+202C/U+2060 em RTL e
  ausência dos dois controles direcionais em LTR;
- argumentos extras, elemento não outlinable e Location ausente conservaram
  os erros medidos; `box(width: 1fr)` público continua rejeitado, como scope-out;
- `cargo check` de bibliotecas e o build do binário terminaram sob 2 GiB e um
  job. A tentativa de compilar todos os testes atingiu esse teto depois de
  apontar duas fixtures antigas sem `width_fraction`; ambas foram corrigidas,
  mas o alvo guloso não foi repetido e não recebe crédito de execução.

Veredito do ensaio segregado: `Preserved` para os oito oráculos bilaterais e
os ataques negativos observados; `Unknown` para o harness unitário não
executado, isolamento de autoria e igualdade de render/bytes. Protocolo
executado sem atestação de isolamento.
