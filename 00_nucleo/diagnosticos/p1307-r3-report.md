# P1307-R3 — L0 do transporte de argumentos e dos encoders

Os **18 L0 foram redigidos e revisados**. Há uma proposta concreta para
aprovação pública; não há implementação P1307 nesta rodada. A autorização
“autorizo” permitiu redigir o escopo ampliado, não aprovou antecipadamente
os campos e comportamentos que só agora ficaram definidos.

## O que muda de fato

Args recebe `occurrences: Option<Vec<ArgOccurrence>>`. Cada ocorrência guarda
nome opcional, Value, span do argumento completo e span da expressão-valor.
Isso preserva dois dados que o mapa atual apaga: **o valor named anterior**
e **sua origem**, inclusive depois de spread, sink, factory e With.

`items` e `named` continuam públicos como views. Some contém a sequência
causal autoritativa; os construtores e métodos regeneram as views. None é
síntese/legado explicitamente sem origem, não um remendo automático para dados
incoerentes. Campos públicos não oferecem garantia automática pelo sistema
de tipos: a auditoria e os testes de todos os writers são obrigatórios.

A API proposta inclui `from_parts`, `from_occurrences`,
`occurrence_sequence`, `invalidate_occurrences`, `remove_positional` e
`remove_named`; preserva assinaturas de `positional`, `len` e `is_empty`.
Adicionar o campo quebra literais Rust externos. O contrato completo está em
`entities/args.md`, relativo a `00_nucleo/prompts/`.

A nova superfície de linguagem é:

```typst
json.encode(value, pretty: true) -> str
toml.encode(value, pretty: true) -> str // dictionary no topo
yaml.encode(value) -> str
```

Os pais permanecem decoders chamáveis. Não há novo encoder csv/xml/read,
dependência, flag, feature ou fase de pipeline. Loading valida argumentos e
escolhe a âncora correta; dispatch não interpreta mensagens para escolher
spans. As strings retornadas são comparadas integralmente, sem normalização.

## Efeitos públicos que precisam ser aprovados junto

| Operação | Decisão concreta |
|---|---|
| With | Conserva todas as ocorrências. `pretty: "bad"` anterior ainda falha mesmo havendo `pretty: true` posterior. |
| Args + Args | Remove named da esquerda presentes à direita; não usa o merge de With. Span agregado detached. |
| arguments.filter/map | Percorre ordem conjunta e duplicatas; pode alterar ordem/quantidade de callbacks. Filter preserva spans individuais; map destaca apenas value_span; ambos destacam o agregado. |
| arguments.len/repr | Com Some, len da linguagem conta ocorrências e repr usa ordem causal. O len Rust continua contando posicionais. |
| Repr canônica | State mostra chave/init; Counter distingue suas chaves; Location usa `location(..)`; toda função With usa `(..) => ..`. Nome e namespace da função não mudam. |
| Fallback CBOR | As correções de repr acima propagam à string codificada. A exceção é explícita: não se promete preservação byte-a-byte dessas classes. |

Não há correção geral de igualdade Args: PartialEq mantém items/named/span
e ignora apenas o campo novo. Alterar o span agregado em join/map/filter pode
afetar o resultado dessa igualdade legada; o contrato declara isso. Também
não há correção incidental dos constructors State/Counter, selectors
compostos ou da dívida de Symbol/Content em CBOR.

## Migração por owner

Todos os paths desta tabela são relativos a `00_nucleo/prompts/` e cada L0
continua com um único consumer produtivo.

| L0 | Responsabilidade |
|---|---|
| `entities/args.md` | Definição, views, coerência e métodos de consumo. |
| `entities/func.md` | With guarda Args inteiro; remove definição duplicada de Args. |
| `compiler/eval/call_dispatch.md` | Captura AST, spreads, fusão With e delegação sem perder origem. |
| `compiler/eval/closures.md` | Consumo de parâmetros e transporte do sink restante. |
| `compiler/stdlib/collections.md` | Receiver, filter/map, projeções e len da linguagem. |
| `compiler/eval/operators/join.md` | Regra específica de Args + Args. |
| `compiler/eval/repr.md` | Projeções canônicas e repr causal de Args. |
| `compiler/stdlib/loading.md` | Encoders, classes, emissão, casts e diagnósticos. |
| `compiler/stdlib/_comum.md` | Reexports internos; sem lógica de encoder. |
| `compiler/eval.md` | Registro dos namespaces e fachada de repr inalterada. |
| `compiler/eval/tests.md` | Regressões e ataques independentes. |
| `compiler/eval/math.md` | Builders AST com ocorrências; spread math ignorado continua dívida explícita. |
| `compiler/stdlib/numbering.md` | Callbacks com valores calculados: síntese sem origem lexical inventada. |
| `compiler/stdlib/foundations/int.md` | Receiver sintético e projeção positional coerente. |
| `compiler/stdlib/gradients.md` | Receiver sintético sem apagar origens dos demais argumentos. |
| `compiler/stdlib/foundations/float.md` | Mesma migração nos predicados, preservando âncoras P1293. |
| `compiler/eval/bindings/method_dispatch.md` | Consumo positional/named pelos métodos coerentes. |
| `compiler/eval/bindings/field_access.md` | Consumo de default em Content; proteção P1306 permanece. |

Os últimos quatro owners foram encontrados na revisão. O adendo sucessor
`p1307-r3-scope-amendment.json` declarou essa ampliação documental dentro da
autorização de transporte, e `p1307-r3-scope-baseline.json` guardou seus textos
antes das edições. O preflight original não foi reescrito. A nota congelada
do autor dos seis primeiros contratos registra o instante anterior, quando
esses quatro L0 ainda faltavam; este relatório e a revisão final refinam
esse estado. Os pontos conhecidos estão atendidos documentalmente, mas isso
não prova uma auditoria exaustiva de todo writer indireto.

Os literais puramente sintéticos nos testes de shapes/plugin/structural/hub
também precisarão de adaptação de compilação após o gate. Seus owners devem
ser conferidos antes da edição; a migração não autoriza semântica nova ali.

## Verificação e proveniência

HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree P1306 não
commitado. Antes de R3, os arquivos tracked alterados eram exclusivamente
os L0 e Rust de `compiler/eval/bindings/field_access` e `compiler/eval/tests`.
Os snapshots abaixo contêm a lista exata, `git diff HEAD --stat`, diff
integral, status e horários; não se atribui ao HEAD sozinho o estado medido.

- Baseline R3: `p1307-r3-baseline.json`, SHA-256
  `b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`.
- Baseline adicional: `p1307-r3-scope-baseline.json`, SHA-256
  `592497d1e786241b9ba121479c0c55dbad370a70732e130b4f7ac3bd709d4405`.
- Linhagem documental final: `p1307-r3-lineage.json`, SHA-256
  `3b34aed0c540bac2dc0b96872596fa16859c10c6df6d69259a8fa744ddaa94eb`.
  Entre as 1.027 entradas do inventário, mudaram somente os 18 L0 declarados.
  Rust, headers, Cargo, núcleos e evidências predecessoras permanecem iguais.
  Cada par registra SHA completo do L0, header efetivo esperado pelo linter,
  hash do consumer e metadata inversa declarada/calculada.
- Revisão independente dos L0: `p1307-r3-review.md`, SHA-256
  `d311927234ad34ae6a22d8f20825f3c771619cd0a20807678b6b2679e08c0c29`.
  Nenhuma contradição substancial remanescente para apresentar a proposta.

Os números acima vêm da conferência timestampada de linhagem, não de execução
de candidato. A referência das medições reutilizadas continua vanilla
ratificado `a51e02804`, com hashes e recibos integrais citados nos L0/revisão.
Não foi necessário novo build ou nova matriz binária para redigir documentos.

`p1307-r3-gates.json` conserva comandos, horários, estado antes/depois e saídas:

- `crystalline-lint --checks v3,v4,v13,v14,v15,v26 .`: zero violações.
- `git diff --check`: passou.
- `crystalline-lint .`: exit 0 no limiar padrão de erros, **não** zero
  ocorrências: emite drift e informações preexistentes.
- `crystalline-lint --checks v5 --fail-on warning .`: exit 1, com os 18
  drifts esperados porque o L0 foi editado e os headers não foram.

Há ainda sete metadados inversos `Hash do Código` stale **já no baseline**:
call_dispatch, math, eval, repr, collections, hub _comum e numbering (este
continha `ffffffff`). Foram preservados, identificados por path e comparados
ao hash real no recibo de linhagem. Isso não é cadeia bidirecional selada.
O resselo posterior deverá usar os bytes reais da implementação autorizada;
não rodamos `--fix-hashes` nem alteramos headers para esconder o estado.

## Próximo gate

A decisão humana agora é sobre **este contrato público concreto**, incluindo
o campo/API de Args, compatibilidade de literais, encoders/defaults e os
efeitos da tabela acima. Após aprovação: completar a auditoria dos writers,
congelar expectativas independentes adicionais de ordem/filter/map/join,
With de outras nativas e deltas CBOR, calibrar ataques e só então iniciar
RED e implementação segregados. Unknown obrigatório continua bloqueante.

A skill `tekt-materializacao-segregada` separou autoria dos L0 e revisão,
preservou os artefatos predecessores e impede declarar esta rodada como selo
de implementação. Regime: **executado sem atestação de isolamento técnico**.
O revisor foi autor das medições R2 reutilizadas; não se alega uma segunda
autoria independente dessas medições. Não houve stage, commit, push ou
alteração dos passos de execução. A parada é no ADR-0127, não em uma falha
de ferramenta ou falta de temporários.
