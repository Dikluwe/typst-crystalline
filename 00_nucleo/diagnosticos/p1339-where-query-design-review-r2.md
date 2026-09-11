# P1339 — R2: alternativas de evento nativo para query

Inspeção somente leitura de código/L0, em 2026-09-10, aproximadamente
00:23–00:27 UTC. HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, sem diff
produtivo. Este documento complementa, sem substituir nem editar,
`p1339-where-query-design-review.md`. Regime executado sem atestação de
isolamento; não houve implementação, teste funcional ou selo.

Escopo: avaliar a alternativa recebida `ElementPayload::NativeElement` unit
mais `ElementKind::NativeElement` unit, comparar variantes Strong/Emph
específicas e identificar a autoridade adicional necessária. Não amplia o
inventário de famílias cuja paridade será implementada neste passo.

## Proveniência

L0s lidos integralmente neste complemento e SHA-256 medidos:

| Owner em `00_nucleo/prompts/entities/` | SHA-256 |
|---|---|
| `element_payload.md` | `be4c48cc670a086c5920e849162d9f0f7e296f075da3c74590a951e4f83fb47a` |
| `element_kind.md` | `b9853fad937cbfc2361f32903a653b55d11d3ebfbbc46ebd5b08f121d7e5f6d1` |
| `element_info.md` | `8e1875ebd93092b19be02c6adb85a42aee4c84e41cdce0b11f3ccded9e9c17b0` |
| `tag.md` | `073fd76dd37492d135d9a90dd2775db664e0c9148ac64951f0eb8c767db5155e` |

Também lidas integralmente ADR-0068 e ADR-0069. O diff não commitado medido
durante esta revisão era documental e vinha da autoridade principal:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 43 +++++++++++
 .../compiler/eval/bindings/value_methods.md        | 89 ++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 53 +++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 44 +++++++++++
 00_nucleo/prompts/compiler/eval/repr.md             | 38 +++++++++
 00_nucleo/prompts/compiler/eval/rules.md            | 34 +++++++++
 .../prompts/compiler/eval/selector_matching.md     | 59 ++++++++++++++
 00_nucleo/prompts/entities/selector.md              | 82 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                  | 48 ++++++++++++
 9 files changed, 490 insertions(+)
```

## Medição antes da classificação

- `element_kind.md:13` define discriminador por tipo de elemento;
  `:23` exige variantes sem campos; `:124` exige inventário dedicado para
  adicionar variantes; `:125` proíbe variantes catch-all (`Other`, `Unknown`).
  Portanto o bucket genérico não é mera aplicação de uma extensão já prevista.
- `element_payload.md:13` define dados tipados por kind e `:31` evita Content
  diretamente no payload. Uma variante unit não aumenta cópias de árvore ou
  campos: o store já contém Content e snapshot (`entities/value.rs:26-45`,
  `entities/introspector.rs:372`).
- `ElementInfo` exige payload presente (`element_info.md:62`), mas não possui
  campo ElementKind. Tag::Start contém Location e ElementInfo, e Tag::End
  contém Location e hash (`entities/tag.rs:18-26`). Não existe obrigação de
  representação Rust de que todo payload tenha um Kind: a população do índice
  é manual em `compiler/introspect.rs:792-794`.
- Há precedente explicitamente documentado de payload sem ElementKind:
  ADR-0069, Decisão item 6, exclui `ElementKind::Labelled`. Isso não autoriza
  transformar Strong/Emph em Labelled nem adotar sua emissão pós-recursão;
  demonstra apenas que payload e Kind não têm bijeção estrutural obrigatória.
- A população de labels ocorre antes do match do payload
  (`introspect.rs:789-791`); a gravação da entrada completa ocorre depois do
  helper e antes de Tag::Start (`:1345-1360`). Um arm unit pode cumprir
  trabalho de marcador, sem fingir ser um elemento legado.
- Parent index discrimina explicitamente payloads reais
  (`introspect.rs:1086-1099`): qualquer promoção deve ser incluída nesse
  conjunto. Tags End existentes recebem `hash_content(content)` (`:1944-1948`),
  que inclui o discriminante e dados por Debug estrutural
  (`entities/content_hash.rs:25-29`); compute_tags_hash inclui a sequência
  inteira (`introspect/convergence.rs:29-32`). Assim, payload unit não torna
  necessariamente invisível a troca Strong↔Emph no mesmo lugar. Isso deve ser
  verificado também para wrappers de origem semântica; não é prova de
  convergência geral ou de campos derivados somente de estilo.
- `ElementKind::as_str` é público e total (`element_kind.rs:99-122`), e os
  parsers usam `from_name` (`stdlib/foundations/selector.rs:56`, `:277`).
  `eval/repr.rs:1042` imprime Kind por as_str; `03_infra/src/query_helpers.rs:178`
  expõe kind_name. Um bucket adicionado a essas rotas pode vazar para linguagem.
- ADR-0068 fixa sincronização dos Locators por `is_locatable` versus
  `extract_payload.is_some()`. O ponto real de layout está em
  `layout/mod.rs:1484-1487`; também grava Position. Strong/Emph já possuem
  braços próprios de layout (`:1894-1913`), mas há caminhos de coleta inline
  adicionais (`:2640`, `:2653`) que exigem revisão antes de alegar sincronização.

## Comparação das alternativas

| Alternativa | Extensão pública adicional | Consequência |
|---|---|---|
| Payload e Kind específicos Strong/Emph | Duas variantes de cada enum | Preserva o significado de Kind como tipo; exige repetir extensão pública ao promover outra família sem Kind. |
| Payload unit NativeElement e Kind unit NativeElement | Uma variante de cada enum | Permite índice de candidatos compartilhado, mas altera o significado de Kind para categoria e requer tratar a proibição de catch-all e vazamento de nome. |
| Somente payload unit NativeElement, sem novo Kind | Uma variante de ElementPayload | Mantém tags/Locator e Kind por tipo; consulta Element usa o store completo ordenado e o matcher por identidade real. Não há bucket nomeável por Kind. |

As contagens desta tabela descrevem o desenho proposto, não medições de patch.
Nenhuma dessas extensões está coberta pela aprovação de Selector::Element e
ShowSelector::NativeElement.

O segundo desenho é tecnicamente coerente **se** a categoria tiver membership
fechado e explicitamente aprovado: neste recorte Strong/Emph, sem aceitar
`_ => NativeElement` nem promover automaticamente todo native futuro. Não é
registro, vtable ou igualdade por nome: a função aprovada permanece no
Selector, o conteúdo na entrada indexada, e o matcher de compiler faz a
discriminação real. Mas os L0s atuais não autorizam esse novo significado de
Kind; precisaria de decisão explícita que distinguisse categoria finita de
catch-all proibido. Não o recomendo como opção padrão contra o L0 vigente.

`as_str(NativeElement)` teria necessariamente algum texto técnico, dada sua
assinatura atual. Esse texto não pode virar nome de função do elemento nem
selector parseável. `from_name` não deve reconhecê-lo; produtores da linguagem
não devem construir Kind(NativeElement); query/repr/counter novos devem usar
Element(Func, fields). É impossível prometer que o enum seja privado em Rust:
o próprio Kind é público e callers podem construí-lo diretamente. A promessa
possível é não expor o nome do bucket como identidade **na linguagem Typst**.

## Alternativa menor identificada: somente payload

Para o recorte medido, o Kind genérico não é necessário para o transporte.
A promoção poderia adicionar somente `ElementPayload::NativeElement` unit,
com admissão fechada de Strong/Emph em extract_payload e is_locatable. O walk
existente passa a produzir Start/End e a guardar a entrada canônica por
Location; o arm de população preserva labels e não inventa um Kind para ela.
O parent index inclui esse evento como nó real. Query Element percorre as
entradas em ordem de Location e aplica a identidade nativa real e filtros
por igualdade de linguagem. Não usa o nome de um bucket nem requer índice
por função. A ordem é recuperável pelo Locator monotônico vigente; não se
usa a ordem do HashMap.

Essa alternativa não viola a proibição de catch-all em ElementKind porque
não altera o enum. Ainda exige atualizar explicitamente a descrição
payload/kind do L0 para essa categoria finita, assim como a semântica do novo
marcador; não é código autorizado agora. Seu custo é examinar mais candidatos
na consulta, que é uma escolha de mecânica. Não há medição de performance
neste diagnóstico que justifique acrescentar o Kind por otimização.

Nem a opção de uma variante nem a de duas fecha o runtime de counter descrito
em R1: demandas, histórico, callbacks e identidade das chaves continuam
precisando de desenho proprietário. A unidade do payload é suficiente para
marcar a ocorrência; o código de contagem deve buscar o Content correspondente
e usar a ação estática adequada. Não pode atribuir uma identidade única ao
marcador e contar todos os candidatos como se fossem o mesmo elemento.

## Autoridade concreta e limites

Há necessidade pública efetiva de **ao menos um novo carrier de evento**
nas alternativas aqui demonstradas que preservam tags e a sincronização
atual. A opção menor identificada é uma variante unit de ElementPayload;
adicionar também o Kind é uma decisão extra, não uma necessidade demonstrada.
Uma proposta ao dono pode portanto solicitar especificamente a variante de
payload, o seu membership inicial Strong/Emph e a interpretação de marcador
cuja identidade se encontra no store canônico. Isso exige L0 novo e gate
ADR-0127 antes da implementação. Não modifica ElementInfo, Tag ou traits.

Promover outros elementos no futuro não fica autorizado por esse nome
genérico: cada nova família precisa de medição e extensão explícita do L0,
mesmo quando dispensar nova variante pública.

A alternativa sem qualquer carrier novo, alocando Locations fora das tags,
continua não demonstrada. A exceção ADR-0069 é para informação dependente do
estado após recursão, e a implementação ratificada reutiliza Location do alvo
para preservar ADR-0068; não é licença geral para acrescentar chamadas a
locator.next fora do gate compartilhado. Fazer isso apenas em introspect
deslocaria as Locations usadas pelo layout e quebraria a associação de
posição/estado aos nós seguintes. Não se pode esconder essa mudança como
adaptação interna de query.

Conclusão limitada: a aprovação dos enums de seletor permite ampliar seus
consumers internos após nucleação, mas não a variante de evento aqui
identificada. Recomendo nucleação da opção de payload único para revisão do
dono, mantendo o desenho de Kind vigente. A preferência depende de validação
posterior de identidade, wrappers, posições, parent index e convergência;
não há alegação de implementação pronta nem impossibilidade genérica de
outras arquiteturas.
