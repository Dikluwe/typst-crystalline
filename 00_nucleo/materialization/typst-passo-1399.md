# Passo 1399 — opacidade morfológica de `ContextBlock`

## Objetivo e regime

Corrigir exclusivamente a representação textual de todo
`Content::ContextBlock`: a forma pública deve ser `context()`, independente do
corpo, closure, identidade ou origem. Preservar `(..) => ..` para funções e
`[x]` para conteúdo textual comum.

Aplica-se o protocolo completo da skill `tekt-materializacao-segregada`
porque o passo altera morfologia pública e exige obrigação, contrato,
oráculos, ataques, implementação e veredito sob autoridades separadas. O
workspace é compartilhado e não prova isolamento ambiental forte; a linguagem
proporcional obrigatória é **executado sem atestação de isolamento**. Este
passo e os JSONs iniciais são artefatos candidatos: não selam contrato, não
atestam isolamento e não emitem veredito.

## Medição anterior à decisão

Estado causal observado em `2026-09-14T17:54:52-03:00`: HEAD limpo
`bdec21bf935cbf7f61070756cd0f218e441b7e07`, com `git status --short` e
`git diff HEAD --stat` vazios. Identidades congeladas:

- L0 anterior `00_nucleo/prompts/compiler/eval/repr.md`, SHA-256
  `cf13a3d23c7159125732b6dddeb3f8883a08c6b5cfa2ab9a877908b2761f8e24`;
- único consumer produtivo baseline
  `01_core/src/compiler/eval/repr.rs`, SHA-256
  `c31e786562cbbcd7d874c91ef3b8884fa802cb64ca7c84f421b761bb5ee9cc52`;
- vanila ratificado, commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino baseline recém-construído, binário SHA-256
  `3ff85cea7a721adcc9be76807e04d8aeeb03e8290d60f796b850632575b7b2b4`.

O build baseline foi serial, uma job, sob `RLIMIT_DATA` de 2 GiB: terminou em
8,84 s, RSS máximo 319.792 KiB e swap zero. Entre
`2026-09-14T17:54:52-03:00` e `2026-09-14T17:55:08-03:00`, sondas
bilaterais seriais sob `RLIMIT_AS` de 512 MiB mediram:

| expressão | vanila ratificado | cristalino baseline |
|---|---|---|
| `repr(context [x])` | `context()` | `context(...)` |
| `repr(context [different])` | `context()` | `context(...)` |
| `repr(context {1 + 2})` | `context()` | `context(...)` |
| `repr(context text.lang)` | `context()` | `context(...)` |
| `repr((x) => x + 1)` | `(..) => ..` | `(..) => ..` |
| `repr([x])` | `[x]` | `[x]` |

Em
`lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:626-640`,
SHA-256
`a477eb1013d08d18a39da3c3341d666b37a4a9bab8d14d93e3327aff78674245`,
a fonte ratificada delega ao handle ou enumera somente fields públicos. Em
`lab/typst-original/crates/typst-library/src/foundations/context.rs:65-83`,
SHA-256
`259527ad0d9bea23790dd96df3d163ad39199adbe6aff68e1d5cd8bf10d7cd98`,
o único field de `ContextElem` é `func`, marcado interno. Portanto a intenção
de fonte é não expor esse payload e a forma pública do elemento não possui
argumentos: `context()`. O baseline cristalino codifica `context(...)` em
`01_core/src/compiler/eval/repr.rs:1016`.

## Classificação e contrato observável

`repr` expõe morfologia da linguagem (ADR-0107). Não se está copiando a
estrutura Rust, o mecanismo do handle, identidade de closure, passos de eval
ou layout, nem bytes renderizados. A intenção decorre da visibilidade declarada
do field somada à regra geral da fonte, não somente do comportamento observado
(ADR-0108).

Para todo `Content::ContextBlock`, emitir exatamente `context()`. A regra vale
para qualquer corpo, closure, id, span, origem sintática ou estado contextual.
É proibido:

- conservar `context(...)`, emitir o corpo ou representar a closure;
- especializar `[x]`, `[different]`, `{1 + 2}`, `text.lang` ou seus resultados;
- decidir por igualdade, hash, endereço, id, span ou origem;
- alterar entidade, construção, parser, eval, layout ou export;
- alterar a representação geral de closure, função, conteúdo ou constructors;
- usar helper geral como veículo para mudar outras variantes de `repr`.

Os controles `repr((x) => x + 1)` → `(..) => ..` e `repr([x])` → `[x]`
permanecem bilateralmente preservados.

O único owner é `00_nucleo/prompts/compiler/eval/repr.md` e o único consumer
produtivo é `01_core/src/compiler/eval/repr.rs` (ADR-0129). O delta produtivo
autorizado restringe-se ao braço `Content::ContextBlock`, expectativas locais
test-only diretamente contraditórias e metadados de linhagem. Nenhuma
assinatura pública muda.

É inferência que um braço privado basta. Refutam-na: field público encontrado
na fonte ratificada, sonda bilateral válida de qualquer `ContextBlock` cuja
forma pública não seja `context()`, ou necessidade demonstrada de mudar
entidade, construção, parser, eval, layout, closure ou helper geral. Nesses
casos a cadeia é invalidada e retorna à autoria da intenção.

Trata-se de correção interna de paridade morfológica, sem contrato Rust novo,
default de produto, mudança de fase ou quebra deliberada. Segue fluxo contínuo
ADR-0127: L0 atualizado primeiro, RED, candidato, GREEN, resselo e
revalidação, sem gate humano intermediário.

## Oráculos, ataques e gate discriminatório

O autor de oráculos deve congelar antes do candidato as quatro testemunhas
contextuais e os dois controles. Precisa ainda demonstrar generalidade por
variação de corpo e origem admissível sem inspecionar o patch. O adversário
deve cobrir, no mínimo:

- manter `context(...)`;
- corrigir somente `[x]` ou somente corpos textuais;
- decidir por corpo, closure, id, span ou origem;
- representar corpo ou closure dentro de `context(...)` ou `context(...)` com
  outra pontuação;
- mudar `(..) => ..` de funções;
- mudar `[x]` de conteúdo textual comum;
- editar helper geral e causar deriva em outra variante;
- tocar entidade, eval ou layout para obter a string.

Casos positivos válidos classificam como `Preserved`; mutações negativas
válidas, como `Violated` com testemunha pública; apenas opacidade declarada
classifica como `Unknown`. O score obrigatório é
`mutations_validas_rejeitadas / mutations_validas = 1.0`. Mutação válida
sobrevivente ou `Unknown` obrigatório impede selo. Repetição e ordem inversa
devem produzir o mesmo vetor.

## Política `Unknown` fail-closed

Classificar como `Unknown`, nunca como sucesso implícito:

- identidade de binário, HEAD, prompt, consumer ou fixture ambígua;
- parse ou construção não suportada;
- falha de build ou execução, exit inesperado ou saída ausente;
- timeout interno ou externo;
- OOM, swap não zero ou limite de RSS excedido;
- stdout ou stderr acima de 64 KiB, truncado ou ilegível;
- budget de calibração esgotado;
- evidência não bilateral ou incapaz de atribuir o delta ao recorte.

## Contenção de memória e execução

É proibido executar `cargo test`, testes amplos de workspace ou compilar o
lib-test monolítico. Oráculos executam um processo por vez com 512 MiB de
espaço de endereçamento, 10 s por processo, 60 s de timeout externo do lote e
64 KiB no máximo por stream. O harness deve matar o processo, preservar o
motivo e classificar `Unknown` se qualquer teto for alcançado.

Build, check e lint rodam sequencialmente, uma job, sob `RLIMIT_DATA` de 2
GiB. `CARGO_BUILD_JOBS=1`; qualquer ferramenta com paralelismo próprio usa uma
thread. Swap não zero ou RSS acima de 1.782.579 KiB aborta toda a sequência,
sem tentativa automática com teto maior. Cada recibo registra instante, HEAD,
`git diff HEAD --stat`, argv, cwd, exit, duração, RSS, swap, bytes de stdout e
stderr e classificação.

## Capacidades dos papéis

- **Autor da intenção/contrato (`/root/contrato_p1399`)**: lê AGENTS.md,
  skill e referências, ADRs aplicáveis, L0 inteiro, baseline congelado,
  medições fornecidas e trechos ratificados; escreve somente este passo, o L0,
  `p1399-manifest.json` e `p1399-contract.json`. Não lê patch candidato, não
  implementa, não escreve oráculos/mutantes e não sela nem julga.
- **Autor dos oráculos**: recebe manifesto, L0 e contrato candidatos; escreve
  somente suite e recibo de oráculos. Não adapta expected ao candidato.
- **Adversário**: recebe manifesto, L0, contrato e baseline; escreve somente
  mutantes/ataques e seus recibos. Não escolhe nem corrige a solução.
- **Implementador**: só começa após preseal válido; recebe entradas seladas e
  altera apenas o delta produtivo autorizado. Não edita contrato, manifesto,
  oráculos ou mutantes.
- **Verificador**: recebe artefatos selados e outputs candidatos; escreve
  recibo e veredito próprios sem editar o material julgado.

O contexto conversacional herdado por cada papel integra sua capacidade e
deve constar no recibo. Hash identifica bytes, mas não prova isolamento.

## Budget de calibração e cadeia

O budget inicial permite no máximo duas revisões malsucedidas pela mesma
`reason_code`, duas execuções focais por revisão, uma execução integral antes
do selo e uma após o candidato. Executar primeiro somente casos afetados e
controles de fronteira; executar corpus integral, repetição e ordem inversa
apenas quando o recorte focal passar. Cada revisão registra hipótese, delta de
classificação, casos corrigidos/regredidos, causa dominante e custo.

Duas revisões consecutivas com mesmo vetor e causa, regressão não monotônica
entre positivos e opacos, score já integral sem resolver controles, ausência
de informação pública necessária ou custo marginal sem nova testemunha reabre
o desenho antes de terceira tentativa. O budget é condição de parada, não
autorização para afrouxar contrato ou `Unknown`.

A cadeia é: L0 + baseline → manifesto → contrato candidato → oráculos e
mutantes → gate discriminatório → preseal → implementação e testes
independentes → gates finais → veredito. Mudança de qualquer entrada protegida
invalida o preseal a partir da primeira fase afetada.

## Scope-out

Ficam fora de P1399: representação geral de funções/closures, corpos comuns,
outras variantes de `Content`, avaliação contextual, construção e erros de
contexto, identidade/spans, layout, export, serializações estruturadas,
refactor de formatter, atomização e paridade funcional além do fragmento
morfológico congelado.
