# P1339 — indexação nucleada; decisão sobre execução de contador filtrado

## Resultado desta continuação

A autorização de `ElementPayload::NativeElement` foi registrada em
`p1339-where-payload-approval.json`, pinando a minuta aprovada e o recibo
anterior. Os L0 de Strong/Emph, extração, locatability, walk, snapshots,
parents, consulta e sincronização com layout agora descrevem sua integração.
Não há novo ElementKind, mapa de campos ou dependência entities→compiler.
O impl concreto de Introspector fica previsto no owner compiler existente,
com trait/struct/assinaturas preservados.

O código produtivo continua intacto. Não houve commit, resselo de headers,
build de candidato, selo ou implementação parcial escondida. P1339 ainda
está na fase B; o gate de payload foi atendido, não revogado.

## Evidências novas e o que elas demonstram

Sonda independente `p1339-where-occurrence-probe-runs.json`, vanilla ratificado
upstream `a51e02804`, 11 casos entre
`2026-09-10T01:06:52.262421+00:00` e `01:06:55.221414+00:00`:

- Calls e markup criam ocorrências próprias; nesting e reuso não colapsam.
- Bold/italic de text sozinho não é Strong/Emph.
- União de seletores preserva ordem documental, não agrupamento por função.
- Query Strong realiza delta default 300, enquanto o constructor cru pode
  omitir esse campo. Labels pertencem à ocorrência, não ao texto semelhante.

O L0 foi corrigido quanto à ordenação dos combinadores com Element e à
captura de snapshots. A revisão independente `p1339-where-index-l0-review.md`
não encontrou bloqueador substantivo no recorte/hash que examinou. Ela não
revisa as minutas de runtime adicionadas posteriormente nem é selo.
Delta explícito/set strong(delta) foi medido no vanilla, mas continua dívida
fora do lote: nenhuma correção do constructor/set ou render foi autorizada
implicitamente por essa medição.

O revisor de runtime registrou limites concretos em
`p1339-where-counter-integration-design.md`: histórico guarda estados, não
ações; Func pós-walk não recalcula Steps posteriores; trait borrowed não
pode devolver vetor temporário; o primeiro contexto precede o runtime.
Ele não executou as sondas de fase abaixo, feitas depois pelo autor L0.

### Fase de counter — medir antes de escolher

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitado; diff/stat e comandos integrais nos recibos. Binários:

- Vanilla `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Cristalino P1338 `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
  `f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.

`p1339-where-counter-phase-probe-runs.json`, UTC
`2026-09-10T01:14:23.778485+00:00`–`01:14:25.923250+00:00`, contém
seis casos por binário em query. Resultados vanilla:

- Heading A, update filtrado `n => n + 10`, Heading B: get/final filtrados
  `(12,)`; bare `(2,)`. Func seguida de Set(4) e Heading dá `(5,)`.
- Update filtrado produzido por contexto é observado por outro contexto
  como `(12,)` ao estabilizar.
- Sem ler o contador, panic do contexto é `context-first`, embora exista
  antes dele um update Func com panic `callback-first`.

No cristalino, where vazio ainda falha na construção; casos cujo where está
apenas no contexto retornaram metadata vazia. **Exit zero com array vazio
não prova que o contexto foi executado**, portanto não serve para validar
callbacks. Esta limitação do runner não foi convertida em sucesso.

O suplemento `p1339-where-counter-phase-compile-runs.json` tentou compile
via stdin: vanilla executou, cristalino respondeu `main file not found: ./-`.
São falhas de transporte, não RED de semântica.

A correção de observabilidade usa arquivos reais em
`/tmp/p1339-counter-phase.iDnXW5`, com fontes completas preservadas em
`p1339-where-counter-phase-file-runs.json`. UTC
`2026-09-10T01:15:47Z`–`01:15:50Z`:

| Fixture compile | Vanilla | Cristalino antecedente |
|---|---|---|
| assert bare após Step/Func/Step = 12 | passa | assertion failed |
| assert where vazio = 12 | passa | erro de construção where vazio |
| assert where(level: 1) = 12 | passa | assertion failed |
| Func com panic, depois contexto com panic, sem leitura | context-first | context-first |

As asserções internas exercitam a linguagem; não foi usado texto/bytes do PDF
como substituto de metadata. Os PDFs são apenas saídas descartáveis da
compilação. A falha bare é dívida preexistente, **não autorização para
corrigir counters legados neste lote**. A prova principal de causalidade
combina a asserção executada com `pipeline.rs:644-666` e
`from_tags.rs:85-136`. Ainda não é RED independente da fase D.

As rodadas foram focais. Não consumiram uma execução completa de calibração,
não modificaram contrato/oráculos protegidos nem apagaram as tentativas
insuficientes. Não há interpretação de custo/desempenho desses tempos.

## Desenho proposto e alternativa refutada

Um log privado guarda ações automáticas reais por ocorrência e updates
manuais pela chave completa. Uma consulta de contador filtrado percorre o
prefixo necessário e aplica Step/Set/Func em ordem. Filtro vazio continua
distinto do contador bare; level 2 conserva estado hierárquico.

**Proposta:** resolver Func sob demanda de get/final/at/display contextual
para chaves contendo o novo Selector::Element, usando Engine já disponível
nas chamadas estáticas e ligadas. Retorno interno owned evita mudar o trait
borrowed. O log e os campos públicos do introspector permanecem dados; não
há callback em entities, query(selector), descoberta de método ou layout.
Chaves antigas conservam o caminho anterior. Não são propostas novas
assinaturas públicas nem mudança da ordem da pipeline L3.

**Refutada:** executar todos os callbacks antes da expansão de contexto.
Isso faria `callback-first` vencer a fixture em que o vanilla produz
`context-first`. A recomendação condicional inicial do diagnóstico de desenho
é histórica e foi reavaliada com esta nova evidência, não seguida cegamente.

Minutas normativas, paths relativos a `00_nucleo/prompts/`:

- `entities/counter_registry.md`: log privado, sem callback/novo campo público.
- `compiler/introspect.md`: captura causal dos eventos e matching por ocorrência.
- `compiler/introspect/from_tags.md`: resolução sob demanda, delimitando fase.
- `compiler/stdlib/counter.md`: chave preservada e leitura interna owned.
- `compiler/eval/bindings/value_methods.md`: transporte de Engine já presente.

Há ainda investigação/contrato a completar: dependências entre callbacks,
ciclos, estabilidade de eventos gerados em contextos, identidade de chaves com
valores especiais e perfis. A aprovação da direção não torna esses casos
demonstrados; eles devem bloquear selo se permanecerem Unknown obrigatório.
Os L0 das outras rotas do P1339, contrato discriminatório, mutantes reais,+selo, RED independente e validação final também continuam pendentes.

## Decisão necessária

Autorizar a resolução **sob demanda contextual** de callbacks dos contadores
filtrados da nova rota Element, conforme as minutas, preservando contratos
públicos e chaves legadas. Isto muda onde Func executa (pós-walk → consulta
contextual), portanto exige gate ADR-0127 ponto 3. A autorização de payload
não abrangia execução de callbacks. Nenhuma implementação antes dessa decisão.

## Validação e limites

O recibo `p1339-where-index-runtime-receipt.json` fixa o estado final desta
continuação, os L0, fontes produtivas intactas, artefatos preservados e saídas
de V15/V26, V5 e diff-check. V5 é esperado enquanto L0s estão sem resselo;
não é alegação de lint global limpo.

Skill `tekt-materializacao-segregada`: revisor de desenho, medidor vanilla e
revisor L0 tiveram tarefas separadas; as sondas de fase foram exploratórias do
autor L0, não oráculos independentes. Regime **executado sem atestação de
isolamento**. A segregação orientou o registro de limites e a revisão de ordem,
mas não produziu certificado de implementação. A tentativa de reativar o
medidor para a sonda de fase foi recusada pelo limite de tarefas do ambiente;
nenhuma execução foi atribuída a ele indevidamente.
