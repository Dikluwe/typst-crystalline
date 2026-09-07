# P1307-R5 — campos de query preservados até os encoders

**Resultado desta revisão:** L0 ampliado com uma API concreta de snapshot,
sem alteração de Rust. A mudança proposta resolve a perda de campos no
transporte de `query`; não implementa silenciosamente os argumentos de
Heading que o produtor ainda descarta. A implementação depende do gate
público descrito abaixo, não de nova autorização genérica para investigar.

## Evidência que motivou a mudança

HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado.
Estado exato, diff/stat e inventário estão em `p1307-r5-baseline.json`
(SHA-256 `32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`).
As medições usam vanilla ratificado upstream `a51e02804`, não uma tag de
versão. Horários, comandos, fontes e hashes dos executáveis constam dos
recibos independentes:

- Content/igualdade/acesso: `p1307-r5-content-measurement.json`, SHA-256
  `82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`;
  interpretação em `p1307-r5-content-note.md`.
- Repr/CBOR: `p1307-r5-repr-measurement.json`, SHA-256
  `5c145ef6f5694948d34de6be2f35c873b526170d2752b34ca08e858216508567`;
  interpretação em `p1307-r5-repr-note.md`.
- Auditoria de transporte: `p1307-r5-transport-audit.md`, SHA-256
  `bfbcd3f882578df60e0bc948fe8889f2515263a71e6e20e0137f6f6c1b7a2717`.

O ponto causal é `01_core/src/compiler/introspect.rs:1350`: guarda Content
sem os estilos efetivos que ainda estavam disponíveis no walk. Apenas
implementar um encoder não recupera os dados. L3 também os perderia em
`03_infra/src/query_helpers.rs:511` e L2 os substituiria por defaults em
`02_shell/src/cli.rs:758`.

As sondas separam causas: numbering tem um pattern transportável na chain;
supplement personalizado é descartado antes, em `eval/rules.rs:970–1003`.
Não se pode atribuir ambas as perdas ao mesmo ponto. No vanilla, labels
distintas não tornam duas headings desiguais, mas numbering "1" e "I"
tornam. Isso refuta a preservação irrestrita de morph_canon-only proposta
pela auditoria de transporte; o L0 de igualdade foi corrigido explicitamente.

## Decisão proposta — o que muda de fato

| Fronteira | Contrato novo |
|---|---|
| Value | `IntrospectedContent` com árvore e mapa opcional de campos privados; getters imutáveis e conversão explícita para Content. `LocatedContent` recebe esse tipo e a Location separada. |
| Introspector | O store existente `elements` passa a guardar o carrier; não ganha um segundo store. O trait `element_at` continua devolvendo `Option<&Content>`. |
| Walk | Captura Heading antes do primeiro context, com defaults completos, numbering/string, idioma e label causal. Não antecipa callbacks. |
| Query L1 e métodos | Clonam/preservam o valor completo. Fields/has/at usam o snapshot; func e location mantêm suas fontes próprias. |
| Igualdade e repr | Igualdade compara campos realizados sem label/location. Repr de Heading realizado passa a exibir esses campos, sem mudar o IR de render. |
| Query L3 → CLI L2 | `query_elements` retorna `Vec<Value>`; `serialize_query` e `serialize_query_with_format` recebem `&[Value]`. Wiring só encaminha. |
| Encoders | Leem os dados preservados, sem executar contexto ou adivinhar estilos. CBOR continua fallback textual, com delta deliberado da repr de Heading realizado. |

O mapa usa `IndexMap<EcoString, Value, FxBuildHasher>` dentro de Arc, e não
Vec: nomes são únicos pelo tipo e a ordem continua preservada. Isso é uma
refinação mecânica da alternativa recomendada na auditoria, não um novo
modelo de render. O construtor move os dados; coerência entre body do mapa e
da árvore é obrigação testável do produtor, não alegação de validação mágica.

As assinaturas completas estão nos owners:

- `00_nucleo/prompts/entities/value.md`;
- `00_nucleo/prompts/entities/introspector.md`;
- `00_nucleo/prompts/infra/query-helpers.md`;
- `00_nucleo/prompts/shell/cli.md`.

Os demais owners alterados estão enumerados em `p1307-r5-record.py`. As
obrigações compartilhadas estão no Núcleo
`00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml`; ele não
legitima código diretamente. Seu SHA-256 físico é
`419d56209cda385cedb09205b1dde60be6719897dc467f583b637ce389df6084`;
o pin **efetivo Tekt**, incluindo framing de dependências, é
`5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24`.
Não confundir esses dois hashes. Owners permanecem 1:1.

## Obrigações mantidas, sucessoras e dívidas

Não reduzir o corpus R4 nem reescrever seus artefatos. Seus casos obrigatórios
de Content nos três encoders permanecem obrigatórios. Única supersessão
identificada nesse corpus: `r2.construct-LocatedContent`, que preservava repr
curta como dívida, passa a exigir a repr vanilla realizada em todos os seus
perfis. A fonte foi reproduzida byte a byte; o mapeamento independente está
em `p1307-r5-repr-oracle.json` (SHA-256
`49fafb6c13b48e990a04ec65732eea8cd1f371c85d0ed59d8a15f5b0e0aca34c`).

CBOR de Heading realizado continua String, não o mapa que vanilla produz.
O sucessor requer a string da nova repr e os bytes integrais medidos pelos
controles bilaterais de String. Raw Content, Symbol e demais controles CBOR
não ganham correções implícitas. Não declarar paridade geral CBOR.

Os casos R5 são entrada contratual, não aprovação automática de tudo que o
vanilla aceita. A seleção de obrigações é:

- Obrigatórios no recorte: Heading padrão consultado, numbering "1"/"I" e
  set none, idioma causal en/pt, label presente/ausente, acesso/erro/default,
  igualdade dos pares medidos, repr e o delta CBOR acima.
- Controles raw: preservar o comportamento anterior onde a obrigação R4
  já o classificava como dívida; não generalizar repr realizada ao constructor
  direto. O branch Content dos novos encoders continua total e dedicado.
- Casos com supplement Alpha/Beta/none/Func, numbering Func, set level/depth/
  offset/outlined/bookmarked/hanging-indent e precedência de numbering:none
  no constructor sob set ancestral: dívida do produtor, não objetivo de
  correção desta extensão. Permanecem visíveis na medição; não contam como
  paridade alcançada. Região não modelada também permanece dívida.
- Escape array/closure e dois corpos iguais sob estilos diferentes são
  invariantes obrigatórias do transporte. As sondas que também usam
  supplement/offset não modelados não viram falsos testes GREEN: antes do
  candidato, selar testemunhas focais só com numbering/idioma já transportados
  ou dados de domínio construídos explicitamente. Isso não apaga as sondas
  originais nem autoriza o implementador a escrever a própria expectativa.
- Nested context que não executou o marcador continua Unknown histórico;
  Selector::Where continua dívida. Nenhum dos dois foi consertado aqui.
- Estado interno gate ativo sem pattern (fixtures de Content::heading_numbered):
  snapshot expõe numbering None, sem alterar o gate/render legado. É ausência
  explícita de pattern modelado, não paridade nem reconstrução pelo prefixo.

Lowerings já existentes para render em state, math e from_tags usarão a
conversão explícita do carrier, sem apagar Values armazenados. Adaptações
nominais/testes estão inventariadas na auditoria; antes de qualquer alteração
produtiva continua obrigatória a leitura de cada owner. Se uma dessas rotas
exigir mudança semântica adicional, não tratá-la como migração mecânica.

## Gate e estado de entrega

A aprovação solicitada é destas mudanças concretas: tipo da variante pública
LocatedContent, tipo público do store, retorno Vec<Value> de query_elements,
parâmetros &[Value] da CLI e seus efeitos delimitados em fields/repr/igualdade/
CBOR. Não é autorização para ampliar todo Heading, fases ou contrato de
callbacks. O primeiro snapshot é corrigido dentro do walk puro existente.

O snapshot original dos owners está em `p1307-r5-l0-before.json`. Recibos
`p1307-r5-l0-freeze*.json` preservam as revisões documentais e validações;
os primeiros registram pins/blocos de Núcleos inválidos, corrigidos depois,
não execuções de candidato. A revisão independente final e o recibo final
identificam quais bytes estão prontos para o gate.

Regime da skill: executado sem atestação de isolamento técnico. Autoria de
fonte/transporte, medição e revisão foi segregada; não houve candidato,
RED/GREEN, mutantes Rust, certificado de implementação, commit ou push.
As alterações Rust anteriores de P1306 foram preservadas. A pausa após este
L0 decorre do gate público ADR-0127, não da ausência de uma proposta concreta.
