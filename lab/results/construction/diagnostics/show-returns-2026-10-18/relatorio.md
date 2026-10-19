# Retorno de callbacks show — investigação R3-T2

As sondas confirmam rejeição indevida de valores produzidos por funções de show.
Não houve implementação ou alteração de L0 vigente neste diagnóstico.

Medição: HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree não commitado;
UTC, status, `git diff HEAD --stat`, comandos, fontes e hashes de ambos os
executáveis em [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/show-returns-2026-10-18/before.json.gz). Oracle vanilla pinado
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Tekt SHA-256
`57154005875bca986a1c4eb44038c8bd9ad2b3580e5534059574895c94e62f82`. Fontes/L0 completas no freeze do
[diagnóstico textual](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/before-sources.json.gz),
conferidas intactas no encerramento; fixture importada return-module.typ
preservada em outputs e no manifesto deste diagnóstico.

São **66 sondas**, 60 aceitas somente
no vanilla, 6 com aceitação e texto extraído
igual em ambos. São rotas/valores cobertos, não essa quantidade de bugs.
A igualdade de texto não comprova morfologia, estilo ou geometria.

## O que a medição estabelece

- As três rotas literal/regex/elemento aceitam no vanilla int/float/decimal,
  none/bool/auto, array/dict, length/ratio, version/duration/bytes, state/counter,
  label/type, symbol, func, Content e String. Tekt aceita neste corpus somente
  Content/String; os demais são rejeitados antes de emitir o retorno.
- None produz conteúdo vazio; 1.0 exibe 1; decimal preserva 1.50; Version exibe
  1.2.3. State/Counter exibem suas representações, sem efetivar uma atualização.
- O módulo retornado emite MODULE. A fixture importada integra a proveniência;
  não é legítimo substituir seu conteúdo por repr(module).
- Com show raw, retorno bool produz RAW no vanilla. Com show literal de 1,
  retorno inteiro produz NUM. Corrigir somente o erro de tipo e emitir um Text
  genérico perderia esses comportamentos. Realização do produto depende do
  trabalho de precedência/guards/ordem R3-T1, cuja revisão está pendente.

## Fonte e classificação

`lab/typst-original/crates/typst-library/src/foundations/styles.rs:497–517`
chama Value.display depois do callback; `foundations/value.rs:191–207` trata
None, números, String, Version, Symbol, Content e Module, usando Raw typc nos
outros valores. Não há requisito de copiar mecânica de buffers ou enum Rust.

No Tekt há cinco conversões parciais em
`01_core/src/compiler/eval/rules/apply.rs:229–246,410–422,496–508,624–635`
e `rules/textual/matching.rs:206–222`. A primeira faixa é o ramo node legado;
a última em apply é a realização produtiva de localizáveis. Os ramos regex e
literal legados não podem ficar com comportamento distinto. A coerção precisa
ter um owner comum, sem copiar tabelas diferentes por ramo.

O helper atual `markup_stream::value_to_display_content`, em :494–562,
não pode ser reutilizado sem auditar o contrato: omite None/Counter, converte
State em elemento e usa texto para vários valores que deveriam ser Raw. Essa
é evidência da implementação, não nova medição de todas as inserções comuns.
A mudança de apresentação geral de markup é uma frente adicional; não presumir
que basta corrigir callbacks para declarar todo Value.display em paridade.

Symbol continua a diferença morfológica R3-Symbol já inventariada: texto α
igual não prova um elemento symbol. Um contrato público novo exigiria revisão
própria. LocatedContent/query e diagnóstico de recursão também continuam fora
desta matriz de retornos. A sonda anterior de heading retornando a própria
query não é coercion GREEN e não entra nos 66 casos daqui.

## Próximo lote e validação

Atualizar primeiro os L0s owners, escrever testes RED reais das famílias e
implementar uma conversão compartilhada, preservando Content/Location existentes,
Raw versus Text e None real. Corrigir a aceitação de valores é paridade interna;
qualquer campo/variante/API pública adicional exige seu próprio gate. Coordenar
com R3-T1 para não sobrescrever os L0s candidatos nem duplicar lógica de produto.
Não reduzir a aceitação aos valores mais fáceis e não declarar Symbol encerrado.

Revalidar rotas legadas e produtivas, elementos locatáveis, campos/morfologia,
show raw/text subsequente, erros reais da função e efeitos uma vez. PDF/HTML,
suítes/build, Clippy/Rustdoc e linhagem/SARIF continuam gates da implementação;
não foram executados de novo para código candidato inexistente.

Teto efetivo 12 GiB, High8 GiB, swap zero/jobs1/debug0/incremental0; pico deste
cgroup **33746944 bytes (32.18 MiB)**, sem eventos high/max/OOM.
[Recursos](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/show-returns-2026-10-18/before-resources.json) e [resumo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/show-returns-2026-10-18/summary.json) preservam
proveniência. Este pico é de sondas nos binários congelados, não de compilação
nem comparação de desempenho. Sem commit, publicação ou encerramento do objetivo.

## Matriz medida

| Caso | Tekt | Vanilla |
|---|---|---|
| literal-none | error: show rule deve retornar Content ou String, recebeu none | (vazio) |
| literal-int | error: show rule deve retornar Content ou String, recebeu int | 42 |
| literal-float | error: show rule deve retornar Content ou String, recebeu float | 1 |
| literal-decimal | error: show rule deve retornar Content ou String, recebeu decimal | 1.50 |
| literal-bool | error: show rule deve retornar Content ou String, recebeu bool | true |
| literal-auto | error: show rule deve retornar Content ou String, recebeu auto | auto |
| literal-array | error: show rule deve retornar Content ou String, recebeu array | (1, 2) |
| literal-dict | error: show rule deve retornar Content ou String, recebeu dictionary | (a: 1) |
| literal-length | error: show rule deve retornar Content ou String, recebeu length | 2pt |
| literal-ratio | error: show rule deve retornar Content ou String, recebeu ratio | 50% |
| literal-version | error: show rule deve retornar Content ou String, recebeu version | 1.2.3 |
| literal-duration | error: show rule deve retornar Content ou String, recebeu duration | duration(seconds: 2) |
| literal-bytes | error: show rule deve retornar Content ou String, recebeu bytes | bytes(2) |
| literal-state | error: show rule deve retornar Content ou String, recebeu state | state("s", 3) |
| literal-counter | error: show rule deve retornar Content ou String, recebeu counter | counter(page) |
| literal-label | error: show rule deve retornar Content ou String, recebeu label | <x> |
| literal-type | error: show rule deve retornar Content ou String, recebeu type | int |
| literal-content | OUTPUT | OUTPUT |
| literal-string | OUTPUT | OUTPUT |
| literal-symbol | error: show rule deve retornar Content ou String, recebeu symbol | α |
| literal-func | error: show rule deve retornar Content ou String, recebeu function | (..) => .. |
| regex-none | error: show rule regex deve retornar Content ou String, recebeu none | (vazio) |
| regex-int | error: show rule regex deve retornar Content ou String, recebeu int | 42 |
| regex-float | error: show rule regex deve retornar Content ou String, recebeu float | 1 |
| regex-decimal | error: show rule regex deve retornar Content ou String, recebeu decimal | 1.50 |
| regex-bool | error: show rule regex deve retornar Content ou String, recebeu bool | true |
| regex-auto | error: show rule regex deve retornar Content ou String, recebeu auto | auto |
| regex-array | error: show rule regex deve retornar Content ou String, recebeu array | (1, 2) |
| regex-dict | error: show rule regex deve retornar Content ou String, recebeu dictionary | (a: 1) |
| regex-length | error: show rule regex deve retornar Content ou String, recebeu length | 2pt |
| regex-ratio | error: show rule regex deve retornar Content ou String, recebeu ratio | 50% |
| regex-version | error: show rule regex deve retornar Content ou String, recebeu version | 1.2.3 |
| regex-duration | error: show rule regex deve retornar Content ou String, recebeu duration | duration(seconds: 2) |
| regex-bytes | error: show rule regex deve retornar Content ou String, recebeu bytes | bytes(2) |
| regex-state | error: show rule regex deve retornar Content ou String, recebeu state | state("s", 3) |
| regex-counter | error: show rule regex deve retornar Content ou String, recebeu counter | counter(page) |
| regex-label | error: show rule regex deve retornar Content ou String, recebeu label | <x> |
| regex-type | error: show rule regex deve retornar Content ou String, recebeu type | int |
| regex-content | OUTPUT | OUTPUT |
| regex-string | OUTPUT | OUTPUT |
| regex-symbol | error: show rule regex deve retornar Content ou String, recebeu symbol | α |
| regex-func | error: show rule regex deve retornar Content ou String, recebeu function | (..) => .. |
| node-none | error: show rule deve retornar Content ou String, recebeu none | (vazio) |
| node-int | error: show rule deve retornar Content ou String, recebeu int | 42 |
| node-float | error: show rule deve retornar Content ou String, recebeu float | 1 |
| node-decimal | error: show rule deve retornar Content ou String, recebeu decimal | 1.50 |
| node-bool | error: show rule deve retornar Content ou String, recebeu bool | true |
| node-auto | error: show rule deve retornar Content ou String, recebeu auto | auto |
| node-array | error: show rule deve retornar Content ou String, recebeu array | (1, 2) |
| node-dict | error: show rule deve retornar Content ou String, recebeu dictionary | (a: 1) |
| node-length | error: show rule deve retornar Content ou String, recebeu length | 2pt |
| node-ratio | error: show rule deve retornar Content ou String, recebeu ratio | 50% |
| node-version | error: show rule deve retornar Content ou String, recebeu version | 1.2.3 |
| node-duration | error: show rule deve retornar Content ou String, recebeu duration | duration(seconds: 2) |
| node-bytes | error: show rule deve retornar Content ou String, recebeu bytes | bytes(2) |
| node-state | error: show rule deve retornar Content ou String, recebeu state | state("s", 3) |
| node-counter | error: show rule deve retornar Content ou String, recebeu counter | counter(page) |
| node-label | error: show rule deve retornar Content ou String, recebeu label | <x> |
| node-type | error: show rule deve retornar Content ou String, recebeu type | int |
| node-content | OUTPUT | OUTPUT |
| node-string | OUTPUT | OUTPUT |
| node-symbol | error: show rule deve retornar Content ou String, recebeu symbol | α |
| node-func | error: show rule deve retornar Content ou String, recebeu function | (..) => .. |
| output-raw-rule | error: show rule deve retornar Content ou String, recebeu bool | RAW |
| output-text-rule | error: show rule deve retornar Content ou String, recebeu int | NUM |
| output-module | error: show rule deve retornar Content ou String, recebeu module | MODULE |
