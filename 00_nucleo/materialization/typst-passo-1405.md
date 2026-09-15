# Passo 1405 — `outline.entry.indented()`

## Estado congelado

- baseline cristalino: `a159b5f18ad42eab12393b4dd101c0d6f9c1b542`, árvore limpa;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário cristalino: SHA-256 `144267dc07596f4f0938881c38644d54955d4190128daf48da395c268950bfba`;
- binário vanila: SHA-256 `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- medição iniciada em `2026-09-14T23:46:34-03:00`;
- regime: protocolo completo como ensaio, executado sem atestação de isolamento.

## Medição antes da decisão

Sondas do vanila com teto virtual de 512 MiB, timeout de 15 s e saída menor
que 64 KiB demonstraram:

- `repr(outline.entry.indented)` → `indented`;
- assinatura `indented(prefix, inner, gap: 0.5em)`, com `prefix` aceitando
  Content ou `none`, `inner` exigindo Content e `gap` exigindo Length;
- fora de `context`, erro `can only be used when context is known`; dentro de
  contexto sem parent outline, a fonte exige `must be called within the
  context of an outline`;
- com `outline(indent: auto)`, dois níveis e prefixo `[P]`, os insets foram
  `13.2pt` e `26.4pt`; cada corpo contém `prefix-info`, `h(-13.2pt)`,
  `pdf-marker-tag([P])`, `h(7.25pt)` e `[I]`;
- com `indent: 2em`, os insets foram `11.45pt` e `11.45pt + 2em`, sem
  `prefix-info`; `11.45pt = largura([P]) + 0.5em` e o segundo `h` é `5.5pt`;
- com `indent: n => n * 3em`, os insets foram `11.45pt` e
  `11.45pt + 3em`, provando argumento 0-based para a função;
- com `prefix:none` e auto, os corpos são somente `[I]`; os insets dos dois
  níveis são `0pt` e `13.2pt`, usando fallback `1.2em` para nível anterior
  sem informação;
- `gap:1em` mudou o hanging de `11.45pt` para `16.95pt`, sem alterar a
  largura medida do prefixo (`5.95pt`);
- sob RTL o inset lógico aparece em `right`, não `left`;
- chamada qualificada coincide com método; argumento extra, argumentos
  ausentes, tipos errados e erro da função de indentação são propagados.

Fonte ratificada:
`lab/typst-original/crates/typst-library/src/model/outline.rs:524-628,798-833`.
Fixtures canónicas de dois níveis: auto SHA-256
`573a010d978e9493caa8411b5d3980a4280b1d79f629e3593d5471d60535d51f`;
fixa `bf0f79616b932f1642c1c7d8fd48d33c058d5017c517a7459d3bfe671bf7cfbb`;
função `f79543247ebbe04d4d19dfab24a0acfca23c2b09f19510b9abd662b1ed7fcab5`;
sem prefixo `6e792f89b54d8def7402ddbcf389d6c6db5ea612f276685dac35480d20e04911`;
RTL `a73ca15d5f91b153578c58bba5d100d0ebd760b2dffa5411780382f914e74cde`.

No cristalino, a identidade e o método não existem. Além disso,
`#show outline.entry` ainda é rejeitado antes do helper, lacuna reservada a
P1406; portanto nenhum sucesso via show rule pode receber crédito em P1405.

## Decisão proposta

1. expor a função contextual estática `outline.entry.indented` e o método
   `entry.indented(prefix, inner, gap: 0.5em)`;
2. exigir contexto conhecido e o parent outline fornecido pela realização da
   entry; não fabricar parent para entry direta ou consulta fora do outline;
3. medir a largura real do prefixo com as métricas e estilos efetivos; definir
   `prefix_inset = prefix_width + gap`;
4. para indent fixo, usar `base = (level − 1) × indent`; para função, chamá-la
   uma vez com `level − 1` e exigir Length; `Bool(true)` cristalino segue auto
   e `Bool(false)` segue indent fixo zero, preservando a extensão já pública;
5. para auto, alinhar por nível: `base` é a soma dos máximos de
   `prefix_inset` dos níveis anteriores, usando `1.2em` quando não observados;
   o hanging atual é o máximo entre seu `prefix_inset` e o máximo/fallback do
   nível corrente;
6. quando há prefixo, emitir no body: carrier interno `prefix-info` somente em
   auto, `h(-hanging)`, label semântica PDF em torno do prefixo,
   `h(hanging-prefix_width)` e `inner`; sem prefixo, body é apenas `inner`;
7. devolver Block com inset no lado lógico inicial igual a `base + hanging`;
   LTR usa esquerda, RTL usa direita;
8. preservar prefixo e inner como Content. Nenhuma medida observada vira
   constante, exceto o fallback linguístico `1.2em` medido na fonte.

P1406 permanece responsável por aceitar `outline.entry` como selector de show
rule, transportar o parent outline para o contexto de cada entry e integrar o
show default. P1405 não antecipa essa integração nem altera o layout legado de
`Content::Outline`.

## Contrato discriminatório e ataques

Positivos: identidade; auto em dois níveis; fixo; função 0-based; prefixo
ausente; gap customizado; RTL; chamada qualificada. Negativos: fora de
contexto; contexto sem parent; prefixo/inner/gap errados; argumentos ausentes e
extra; callback que falha.

Mutantes obrigatórios: nível usado como 1-based no callback; base fixa não
multiplicada; fallback diferente de `1.2em`; largura aproximada; hanging sem
máximo global; `prefix-info` fora de auto; prefixo sem label PDF; sinais dos
dois `h` trocados; gap ignorado; prefixo `none` fabricado; inset sempre à
esquerda; Content convertido para string. O score exigido é 1.0 antes de selo.

`Unknown` permanece para execução positiva por show rule antes de P1406,
igualdade mecânica Rust, bytes/render exatos, percentuais ainda não aceitos no
contrato cristalino de `outline(indent:)` e isolamento técnico de autoria.

## Política de memória

- sondas: `ulimit -v 524288`, `timeout 15s`, uma thread e saída ≤64 KiB;
- check/build: `ulimit -v 2097152`, um job, incremental e debug metadata
  desligados, 512 codegen units e linker gold;
- não executar `cargo test -p typst-core --lib` nem `cargo check --tests` do
  core; testes unitários são escritos, mas só recebem crédito se um harness
  focal permanecer sob o teto;
- corpus focal antes de qualquer repetição; nenhum teto é elevado
  automaticamente.

## Papéis e capacidades do ensaio

- intenção/contrato: root, lendo AGENTS, ADRs, L0, baseline e fonte ratificada;
- oráculos/ataques: root, derivados das medições anteriores ao candidato;
- implementação/verificação: somente após confirmação, no mesmo filesystem;
- por não haver isolamento de leitura ou autoridade independente, qualquer
  resultado futuro será `executado sem atestação de isolamento`, nunca
  certificado segregado.

## Gate ADR-0127

P1405 acrescenta função pública e semântica contextual nova. A implementação
L1–L4 aguarda confirmação humana explícita deste contrato. Após confirmação:
atualizar primeiro os L0 internos, escrever RED/mutantes, implementar, executar
somente gates limitados, ressellar hashes, lintar e commitar atomicamente.

## Resultado da implementação

Confirmação humana recebida em `2026-09-15`. A implementação expôs a identidade
estática e o método, preservou Content, aplicou indentação fixa, booleana,
funcional 0-based e automática, mediu o prefixo real, transportou máximos por
`prefix-info`, respeitou `gap` e direção lógica e materializou o prefixo PDF
como `/Lbl`.

O ataque bilateral descobriu e rejeitou uma primeira hipótese que guardava o
parent na própria entry: ela faria uma entry consultada funcionar fora do
outline, enquanto o vanila falha. O candidato final não grava esse parent;
espera as propriedades privadas de StyleChain cuja injeção pertence a P1406.
Assim, a fixture `indented.typ` falha nos dois binários com `must be called
within the context of an outline`, enquanto `identity.typ` produz `indented`
nos dois. A execução positiva por show rule continua `Unknown`, sem receber
crédito antecipado.

No estado não commitado sobre `a159b5f18ad42eab12393b4dd101c0d6f9c1b542`,
medido em `2026-09-15T00:17:48-03:00`, oito testes dirigidos do helper e um do
exportador passaram. O build do binário terminou com teto virtual de 2 GiB,
um job e incremental desligado; sondas bilaterais usaram 512 MiB, uma thread e
timeout de 15 s. A suíte monolítica de core não foi executada. Não ocorreu
estouro de memória.
