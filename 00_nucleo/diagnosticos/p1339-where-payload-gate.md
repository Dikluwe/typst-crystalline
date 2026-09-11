# P1339 — aprovação registrada e extensão de indexação pendente

## Resultado desta continuação

A aprovação de `Selector::Element { function, fields }` e
`ShowSelector::NativeElement(Func)` foi registrada em
`p1339-where-approval.json` (SHA-256
`018036469451279fab3b31914c76a7a85b6fd33a81d3e4b2a5bff24f30540bf5`).
Os cinco L0 anteriores agora distinguem aprovação pública de gates de
integração ainda pendentes. Acrescentei L0 de transporte/apresentação em
field_access, call_dispatch, repr, rules e foundations/selector.
Nenhum código produtivo, header ou hash de código foi alterado; não houve commit.

A integração revelou uma dependência que a proposta anterior não havia
resolvido: preservar um seletor não basta quando o elemento selecionado ainda
não tem ocorrência no índice. Esta é uma lacuna concreta, não nova exigência
de papelada nem revogação da aprovação já dada.

## Evidência anterior à nova decisão

Medição independente, sem leitura de código/L0 cristalino pelo medidor:
`p1339-where-integration-probe.md` e seus dois recibos integrais. Referência
vanilla upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitado; cada recibo registra diff/stat, arquivos, comandos e horários.
Rodadas UTC `2026-09-10T00:18:55.842594+00:00`–`00:18:59.537177+00:00`
e `00:20:17.650961+00:00`–`00:20:19.215281+00:00`, somente default/paged.

- `query(strong.where())` e `query(emph.where())` encontram ocorrências;
  seus contadores aplicam filtros de body. A fonte declara ambos Locatable.
- `query(text.where())` e `counter(text.where())` falham com
  `text is not locatable`. Não se pode agrupar os três casos como rejeição.
- Na fixture com um heading numerado e `counter(heading).update(42)`,
  `counter(heading).get()` retorna `(42,)`, mas
  `counter(heading.where()).get()` retorna `(1,)`. Colapsar o filtro vazio
  para a chave bare perderia comportamento, além de identidade/repr.
- Filtrar heading de nível 2 produz `(0, 1)`: contagem não é simplesmente
  tamanho da query. A ação e a ordem dos eventos importam.

No antecedente, `compiler/introspect/locatable.rs:157-159` exclui
Strong/Emph, `compiler/introspect.rs:1278-1363` condiciona ocorrência a
payload, e `:1398-1401` atravessa esses nós sem registrá-los. O índice já
guarda Content/snapshot para as famílias que possuem ocorrência; não falta
um segundo mapa de campos.

## Proposta mínima adicional, ainda sem autorização

Minuta em `00_nucleo/prompts/entities/element_payload.md`:

```rust
pub enum ElementPayload {
    // variantes existentes permanecem
    NativeElement,
}
```

Somente **uma variante unit pública adicional**, limitada nesta proposta
às ocorrências nativas Strong/Emph. Representações alternativas em Styled
dependem de medição e L0 próprio de ocorrência; flags visuais ou de matching
não autorizam Locations adicionais nem dupla indexação.
Não adiciona ElementKind, campo em Content/Func/Value, método de trait,
callback ou registro reflexivo. Não promove text nem tipos futuros.

Ela permite seguir a emissão normal de Tag::Start/End e preencher o store
existente por Location. O conteúdo armazenado fornece identidade e campos;
o payload não finge ser Heading ou Metadata. Tag::End já contém o hash do
conteúdo, inclusive body, portanto não é preciso duplicá-lo no evento Start.
Hash não substitui igualdade linguística nem matching; tampouco prova
estabilidade de campos que existam somente na chain fora do Content nu.
Esse caso precisa ser medido e resolvido no owner de captura.

Alternativas examinadas na revisão `p1339-where-query-design-review*.md`:
adicionar também um bucket ElementKind seria desnecessário; usar um payload
existente como sentinela seria incorreto; indexar sem tags exigiria redesenhar
invariantes de ordem, ancestralidade, convergência e sincronização com layout.
Não foi demonstrada a suficiência desse último redesenho nem alegada sua
impossibilidade universal. A proposta mantém o modelo de eventos atual.

O único gate humano novo é a variante de ElementPayload e a promoção
delimitada que ela permite. Como o enum é público, novos matches externos
exaustivos são afetados: ADR-0127. A autorização anterior não abrangia isso.

## O que permanece antes de implementação

O L0 de integração ainda não está completo. Query/introspector precisam
reutilizar o comparador e a identidade em compiler sem dependência reversa
de entities. Uma alternativa interna identificada é transferir o impl
completo de Introspector para owner compiler, conservando o trait; ainda
exige nucleação própria, não foi executada.

CounterKey já preserva Selector, mas os históricos atuais são alimentados
sob chaves Kind. A solução precisa registrar as chaves demandadas e resolver
eventos automáticos/updates da chave exata, sem deduzir ações de diferenças
entre estados ou executar callbacks em query read-only. Isso exige L0 de
runtime; não foi declarado resolvido por este novo payload. Não se pede
mudança de trait ou de fase nesta aprovação.

Extração, locatability, walk, snapshots, parents, sincronização de Locations
e layout precisam dos respectivos L0 antes de código. Os L0 das outras rotas
do P1339 e contrato/oráculos, mutantes compilados, selo e RED continuam
pendentes. Nenhum gate ou escopo foi reduzido; não se declara paridade geral.

## Validação e revisão

Skill `tekt-materializacao-segregada`: medidor e revisor separados da autoria
L0; execução sem atestação de isolamento, ambiente compartilhado. A revisão
levantou a dependência de indexação e os riscos de colapsar chaves/históricos;
influenciou a redução da proposta a um único payload, sem novo ElementKind.
Não é veredito de implementação nem selo.

O recibo `p1339-where-integration-receipt.json` fixa L0s finais, fontes
produtivas intactas, evidências anteriores preservadas e saídas dos checks.
V15/V26 e diff-check são revalidados; V5 permanece pendente por L0 alterado
sem resselo de headers. Não houve build/teste de candidato ou lint global
de fechamento.

## Decisão solicitada

Autorizar `ElementPayload::NativeElement` para a indexação delimitada de
Strong/Emph conforme a minuta, mantendo as duas variantes já aprovadas.
A aprovação permite completar a nucleação dessa integração; não aprova
novos contratos adicionais nem dispensa os gates anteriores ao código.
