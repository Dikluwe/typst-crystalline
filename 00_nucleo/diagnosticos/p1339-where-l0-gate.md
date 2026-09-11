# P1339 — proposta pública de `function.where` para aprovação

## O que está pronto para decidir

Foram redigidas minutas L0 para a fronteira pública e a semântica de
`function.where`. Nenhum código produtivo foi alterado. A solicitação
«Faça» autorizou preparar e apresentar esse desenho, não atravessar o gate
ADR-0127. Todos os L0 desta rodada estão marcados como minutas pendentes.

A proposta acrescenta exatamente duas variantes:

```rust
// entities::selector::Selector — valor público da linguagem
Element {
    function: Func,
    fields: EcoVec<(EcoString, Value)>,
}

// entities::show::Selector — base interna das regras de show
NativeElement(Func)
```

A primeira preserva função e grupo ordenado de filtros, inclusive vazio.
A segunda permite aplicar a seleção a nós nativos; os filtros viajam pelos
Where já existentes na representação de show. Não há duplicação do grupo
no segundo enum. Nenhum novo campo/variante de Func, Value, Content,
ElementKind, NodeKind ou trait está incluído. Não há registry, vtable, novo
modo padrão ou mudança de fase do pipeline.

São enums Rust públicos: adicionar variantes exige adaptar matches
exaustivos, inclusive de clientes externos. Esse efeito de compatibilidade
é a razão concreta do gate, embora o objetivo seja paridade de linguagem.

## Por que esse desenho

As medições anteriores `p1339-full-*` demonstram que where deve produzir
seletores de strong/emph/text, além de heading/figure/raw/table, e que filtro
vazio é um valor distinto do elemento nu. Não cabe representá-lo por Kind,
And vazio ou regex. O matcher atual não projeta Text.text/Raw como o acesso
público exige; a minuta prevê adaptação na camada compiler, sem colocar
lógica do compilador nas entidades.

A sonda focal `p1339-where-l0-{manifest,vanilla,crystalline}.json` acrescentou
evidência de ordem e igualdade antes da redação. No vanilla: reordenar campos
muda igualdade e repr; spread mantém a primeira posição da chave e o último
valor; filtro vazio difere do selector nu; aliases mantêm identidade;
campos Int/Float equivalentes comparam iguais; NaN não ganha reflexividade.
Por isso o grupo é ordenado, mas a igualdade da linguagem continua no owner
do avaliador, sem alterar PartialEq/Hash estruturais de Func/Value/Selector.

Proveniência dessas medições: HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, diff tracked vazio antes de L0;
vanilla UTC `2026-09-10T00:04:21.279738+00:00`–`00:04:21.402736+00:00`,
cristalino UTC `00:04:21.473703+00:00`–`00:04:23.009904+00:00`.
Binário vanilla ratificado `a51e02804`; manifesto e recibos fixam os SHA-256
dos binários, fontes, comandos, estado Git e canais integrais.

## L0 preparados nesta rodada

Paths relativos a `00_nucleo/prompts/`; cada um mantém seu único consumer.

| L0 | Decisão específica |
|---|---|
| `entities/selector.md` | Variante Element, dados ordenados e filtro vazio, preservação do enum anterior. |
| `entities/show.md` | Variante NativeElement, sem duplicar os filtros nem trocar o significado de Text/DynKind. |
| `compiler/eval/bindings/value_methods.md` | Helper comum estático/ligado; reconhecimento de elemento, Args, campos e diagnósticos. |
| `compiler/eval/selector_matching.md` | Conversão e matching da base nativa; projeção/igualdade nova limitada aos filtros com essa base. |
| `compiler/eval/operators/equality.md` | Igualdade pública de Element, recursiva e sensível à ordem, mantendo derives e pares legados. |

Os hashes finais, arquivos alterados, validações e preservação do produto
constam de `p1339-where-l0-gate-receipt.json`. Não foi executado
`--fix-hashes`: atualizar headers agora anteciparia a aprovação da extensão.

## Inventário de integração — ainda não autorizado para código

O parecer `p1339-where-l0-review.md` inventariou os dispatches por busca de
Selector/QuerySelector/ShowSelector e leitura focal. Os pontos abaixo exigem
atualizações proprietárias de L0 na continuação de P1339 antes de materializar.
A aprovação das duas variantes não dispensa esse trabalho.

| Consumer | Pendência antes de código |
|---|---|
| `entities/introspector.rs:634` | Novo braço exaustivo; usar decisão explícita de filtragem/locatability. Já há Content e snapshot em elements: não justificar impossibilidade com comentário antigo de ElementPayload. |
| `compiler/eval/repr.rs:1040` | Novo braço exaustivo para grupo único, vazio e ordem. |
| `compiler/stdlib/counter.rs:92` | Decidir tratamento do selector em CounterKey; não descartar filtros por copiar fallback legado. |
| `compiler/stdlib/foundations/selector.rs:263` e `query.rs` | Preservar o valor através do parser e definir consumo público pertinente; sem fallback vazio novo. |
| `compiler/eval/bindings/field_access.rs` | Descoberta estática de function.where, nome curto e preservação de extração ligada inválida. |
| `compiler/eval/call_dispatch.rs` | Encaminhamento único, avaliação do receiver uma vez e prioridade observável de falhas. |
| `compiler/eval/rules.rs:476,833,2403` | Preservar caminhos especiais como Par e distinguir elemento text de padrão textual; nenhuma aplicação dupla. |

Os owners correspondentes são `entities/introspector.md`,
`compiler/eval/repr.md`, `compiler/stdlib/counter.md`,
`compiler/stdlib/foundations/selector.md`, o owner de query,
`compiler/eval/bindings/field_access.md`, `compiler/eval/call_dispatch.md`
e `compiler/eval/rules.md`. Não foi alegado que esses L0 já estão atualizados
por esta rodada. Constantes Kind e transportadores em layout/introspect,
CounterKey e Value precisam de regressão; o ParsedSelector de L3 é outro
enum e não requer automaticamente as novas variantes.

## Revisão e validações

Autor L0: `/root`. Revisor: `/root/p1339_where_l0_review`, com escrita somente
em seus pareceres; não escreveu L0/candidato/oráculos. A skill
`tekt-materializacao-segregada` orientou a separação. Regime: executado sem
atestação de isolamento; este é parecer de desenho, não selo de contrato.

A revisão detectou e a redação corrigiu: comparação/projeção nova não deve
alterar todos os Where legados; o helper semântico de where é exceção
explícita ao limite histórico AST de value_methods. O adendo final identifica
os hashes julgados e o alcance do parecer.

V15/V26 limpos; V5 acusa os owners cujos L0 estão em minuta e headers ainda
apontam ao antecedente. Isso é estado pendente, não lint global verde.
Não houve build/teste de candidato, selo, mutation score ou commit. O NaN de
Angle segue a resolução anterior, não foi reaberto nem convertido em sucesso.

## Alcance da aprovação solicitada

Aprovar a extensão pública **Element + NativeElement** e a direção das
minutas acima. Depois completar os L0 de integração e os gates restantes
do P1339 antes do código. Essa decisão não aprova mudança pública adicional,
dispensa de teste, perda de filtros, redução das dez rotas ou conclusão de
paridade geral de selector/show/query.
