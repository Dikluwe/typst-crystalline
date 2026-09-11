# P1339 — nucleação das rotas restantes, entrega ao integrador

Autor: `/root/p1339_remaining_l0`, papel exclusivamente de autoria L0.
Regime: protocolo completo, executado sem atestação de isolamento. Ambiente
compartilhado e allowlist declarada; não há sandbox individual atestado.
Contexto recebido: escopo P1339, estado anterior sem candidato, autorização
do dono para completar o passo e alterações necessárias no escopo, com gates
e segregação preservados. Não escrevi Rust, testes, contrato, ataques, selo,
recibos de execução do produto ou veredito. Não fiz resselo nem commit.

## Entradas e proveniência

Inspeção de fonte antecedente em HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`; marco UTC da entrega textual
`2026-09-10T02:26:47Z`. As medições de linguagem citadas são as rodadas já
congeladas, não novas execuções: seus horários, comandos, canais integrais,
status Git e `diff_stat` estão dentro dos recibos. Nas rodadas final/boundaries
o diff tracked era vazio. O baseline vanilla é somente `a51e02804`, binário
`/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

| Entrada | SHA-256 |
|---|---|
| `p1339-full-final-manifest.json` | `a7afdbf4e5bea1cc897adf330bb307a162b17cd7024806d8b45111619e477336` |
| `p1339-full-final-vanilla-runs.json` | `a7280e76e03c3b98b4681259f6a7e11daca28ce821e2a1bd9b30394458118953` |
| `p1339-full-boundaries-manifest.json` | `b1f66474b9a0bb716b591529a2b528c97cd13a7b6d0a41908afa91738a4336d8` |
| `p1339-full-boundaries-vanilla-runs.json` | `c3d34f69494caac1ecb808a5c8b83bba2603716e63324f56ee530e8097638cd7` |
| L0 float antes | `b8826f091cdd330bb92a32848784fc14cfb5f693d26793252aa12384724c4081` |
| L0 constructor version antes | `6e775351938a8480df7f2fbd1658cdd3d8ae7b5908c4ee8dc34e46a778ce4e5c` |
| L0 entidade version antes | `b28c5144e36183c5c9d4eaa7dc4fcbbe38a9b7c202a1049d3ce661e477745b64` |
| L0 layout_types lido, não editado | `60cde357984c8b75f7e580ee26e1cb2f826b98e48f4b6cf364b8b5fde160b0fd` |
| L0 func lido, não editado | `909cd5a85e4968d42a381b33d343e50a7a37e875817d581a6a2ebe9076057a1a` |
| L0 call_dispatch recebido | `b1c768f0475590103e10ff8f6fafeb12841d02ddf6661fd36c56bae84d9fcbe0` |
| L0 field_access recebido | `f6f9454be8b20ca19330610fcb3f6c8e0425bbb7e339a3db3dfd1092d31d72e6` |
| L0 value_methods recebido | `36e902e07918b4e6c854105fd23ae62e5aa88f673dea1ccd0eb3a5812d883b14` |

Li CLAUDE raiz/L1, skill `tekt-materializacao-segregada` e ambas referências,
ADRs 0107/0108/0127/0129, o passo explicitamente autorizado
`00_nucleo/materialization/typst-passo-1339.md`, A.2, resolução/revisão NaN,
L0 dos owners e fontes pertinentes. Não listei nem varri materialization/context.
A ausência de ADR local de segregação foi verificada por busca em adr.

A árvore recebida tinha somente os seguintes prompts tracked alterados;
`git diff HEAD --stat` registrado antes destas edições:

```text
 00_nucleo/prompts/compiler/eval.md                 | 173 +++++++++++++++++++++
 .../prompts/compiler/eval/bindings/field_access.md |  43 +++++
 .../compiler/eval/bindings/value_methods.md        | 134 ++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  76 +++++++++
 .../prompts/compiler/eval/operators/equality.md    |  67 ++++++++
 00_nucleo/prompts/compiler/eval/repr.md            |  38 +++++
 00_nucleo/prompts/compiler/eval/rules.md           |  34 ++++
 .../prompts/compiler/eval/selector_matching.md     |  78 ++++++++++
 00_nucleo/prompts/compiler/introspect.md           | 123 +++++++++++++++
 .../prompts/compiler/introspect/extract_payload.md |  27 ++++
 .../prompts/compiler/introspect/from_tags.md       |  78 ++++++++++
 .../prompts/compiler/introspect/locatable.md       |  24 +++
 00_nucleo/prompts/compiler/layout.md              |  27 ++++
 00_nucleo/prompts/compiler/stdlib/counter.md       |  80 ++++++++++
 .../prompts/compiler/stdlib/foundations/query.md  |  66 ++++++++
 .../compiler/stdlib/foundations/selector.md        |  35 +++++
 00_nucleo/prompts/compiler/stdlib/state.md         |  45 ++++++
 .../prompts/entities/counter_registry.md          |  35 +++++
 .../prompts/entities/element_payload.md           |  88 +++++++++++
 .../prompts/entities/elements/emph.md             |  25 +++
 .../prompts/entities/elements/strong.md           |  26 ++++
 .../prompts/entities/introspector.md              |  31 ++++
 00_nucleo/prompts/entities/selector.md             |  82 ++++++++++
 00_nucleo/prompts/entities/show.md                 |  48 ++++++
 00_nucleo/prompts/infra/pipeline.md                | 114 ++++++++++++++
 25 files changed, 1597 insertions(+)
```

## Saídas L0 escritas

- `compiler/stdlib/foundations/float.md`: sucede scope-outs das cinco rotas;
  constantes, sinal, binary32/64, defaults, casts estritos, rejeições, hints,
  spans e fronteira ligada de from-bytes. SHA-256
  `784c12e7f9f515c2b05cdc9077f375d8e4049be6c083afbda78e4aa099b36c15`.
- `compiler/stdlib/primitives-constructors/version.md`: adapter único de at,
  parser causal, delegação à entidade e preservação do constructor. SHA-256
  `071a755307a5a4f7f2bdc08faab4d98ca5b916a35a57c72594de931e00532a8b`.
- `entities/version.md`: owner textual único; fórmula vigente reutilizada;
  sucede dispensa antiga de paridade diagnóstica e localização histórica do
  dispatch. SHA-256
  `a41f7e3ad44eddc8bdb18126e673822ac4a6afec57d1b38467b85c756e85286f`.

Esses hashes identificam a redação pré-resselo. `Hash do Código` não foi
alterado. Nenhum dado sobre where/show/observações foi editado.

## Medição que antecede o desenho de integração

1. `entities/layout_types.rs:1277–1290` já possui Angle privado em radianos,
   constructors e conversões `to_rad`/`to_deg`. O L0 layout_types vigente não
   nomeia Angle; foi lido antes da decisão. A fonte ratificada
   `layout/angle.rs:142–151` declara conversões para float. Sondas
   `angle.deg-static-*`, `angle.rad-static-*` e pares ligados fixam graus,
   radianos, sinal e infinitos. Não há necessidade medida de novo módulo.
2. `call_dispatch.rs:549–654` possui descoberta de wrappers internos;
   `:775–1054` contém dispatch fechado; `:1263–1328` encaminha Float/Version
   ligados. `field_access.rs:708–801` já delega Float à stdlib e várias
   famílias ao dispatcher. Esse é precedente de glue sem fórmula nova.
3. `call_dispatch.rs:1369–1383` já faz With ligado; `:510–515` aplica With e
   `:1057–1068` concatena ocorrências pre/new. `entities/func.md` P1307-R3
   especifica o Args integral e lazy invocation; fonte vanilla
   `foundations/func.rs:372–374,395–409` confirma essa intenção. Sondas
   `with-*-chain`, `with-*-spread`, `with-*-panic-*` confirmam ordem/diagnóstico.
4. `value_methods.rs:623–661` tem parser exclusivo do at ligado; a entidade
   já resolve índices em `version.rs:103–120`. Logo extrair o adapter de
   argumentos para o owner de stdlib elimina duplicação sem migrar fórmula.
5. `from-bytes-bound-float` mede `expected bytes, found float` ancorado no
   receiver; `from-bytes-bound-bytes` mede `type bytes has no method
   \`from-bytes\``. Não presumir que uma API estática simplesmente desaparece
   do lookup de métodos float: os erros são parte do contrato observado.
6. A.2 foi sucedido em sua conclusão bloqueante NaN por
   `p1339-nan-resolution.md` e `p1339-nan-review-r2.md`: Angle NaN continua
   Unknown condicional sem crédito positivo. Não reabrir nem normalizar
   aritmética Angle fora do escopo para fabricar um receiver bilateral.

## Propostas de texto para integração nos owners compartilhados

Esta seção é rascunho de integração entregue ao autor raiz, não Prompt L0 e
não legitima consumer. Antes de contrato/selo/código, promover cada cláusula
pertinente ao seu owner e congelar os hashes efetivos.

### `entities/layout_types.md` — acrescentar causa Angle vigente

Após a medição 1 acima, explicitar: Angle conserva unidade interna em
radianos; `to_rad` devolve float nessa unidade e `to_deg` devolve o equivalente
em graus. São as causas existentes a reutilizar por `angle.rad`/`angle.deg`;
os constructors Rust `Angle::rad/deg` não são essas funções públicas.
Preservar finitos, zeros com sinal e infinitos; não alterar representação,
assinaturas nem operações/construção de Angle. Não criar novo owner/módulo
angle para copiar estas fórmulas. A precisão pública das entradas testadas
deve coincidir; um resultado divergente refuta a suficiência da fórmula e
exige reabertura medida, não arredondamento arbitrário de repr.

### `compiler/eval/call_dispatch.md` — Angle, With e transporte

Após as medições 1–3, estender o lookup fechado com wrappers internos de
`angle.deg`, `angle.rad`, `function.with`; os nomes/repr públicos são `deg`,
`rad`, `with`. Wrappers Angle e sua intercepção ligada chamam um parser
interno único, que consome positional `self` estritamente Angle, rejeita
sobras e delega somente a `Angle::to_deg`/`to_rad`. Resultado é Float.
Inteiro/float sem unidade são rejeitados: `expected angle, found <tipo>`.
Missing é `missing argument: self`, na chamada inteira; named `self` sem
positional é `the argument \`self\` is positional` com hint de remover
`self:`; outro named não preenche self. Extra e named desconhecido ancoram
primeira ocorrência sobrante. Cast ancora valor; named positional ancora
ocorrência completa. Métodos Angle sem argumentos só são válidos sobre Angle.

`function.with(f, ..preargs)` e `f.with(..preargs)` compartilham helper de
construção que chama `Func::with` com Args restante integral, sem invocar f
ou validar sua assinatura. Static consome `self` Function (sem converter
Type/namespace/element-name), então guarda todos os restantes posicionais e
named. Missing/self-named/type seguem respectivamente
`missing argument: self`, `the argument \`self\` is positional` com hint,
`expected function, found <tipo>`, e as âncoras anteriores. Todos os named
restantes são pré-argumentos, não keywords desconhecidas deste helper.

Preservar a concatenação causal pre/new de `merge_with_args` e encadeamento
antigo/novo/chamada, sem reduzir ocorrências a mapas: posicionais pré-aplicados
antecedem novos; named posterior projeta a última ocorrência, mas nativas
mantêm autoridade para validar ocorrências anteriores conforme seu contrato.
Duplicado literal permanece rejeitado; duplicado por spreads é transportado.
Func/Args/representação não precisam mudar. Closure, nativa e elemento são
receivers; With de With é permitido. A criação é lazy quanto ao corpo.

Na forma estática, avaliar argumentos em ordem antes do cast do receiver:
`function.with(1,panic("argument"))` produz panic. Na forma ligada, resolver
receiver e método antes de avaliar args: `(1).with(panic("argument"))` produz
`type integer has no method \`with\`` no field access. Receiver inválido não
deve cair numa segunda avaliação do alvo ou dos argumentos. O corpo nunca
é executado para descobrir a função ou sua namespace.

Float: encaminhar `signum`, `to-bytes` e rejeição medida de `from-bytes` ao
owner float, uma vez por chamada. Inserir receiver com seu span real quando
retido na AST; não usar detached onde erro de cast exige origem. Não capturar
`Int.signum` no dispatcher Float. Para Version ligada, avaliar Args uma vez
e encaminhar ao adapter at da stdlib por value_methods.

Transportar span integral às identidades resolvidas destas nativas (também
aliases e With delas) para missing/erros de domínio. Transportar ocorrências
inalteradas para casts/extras; não selecionar erros por string da mensagem
nem identificar nativa pelo nome público curto, que pode colidir. Métodos
ligados transportam o call inteiro e a origem do receiver. Scope fechado:
nenhuma extensão automática a outras nativas ou famílias.

### `compiler/eval/bindings/field_access.md` — descoberta e campos sem chamada

Após a medição 2, o match de Value::Type descobre Angle deg/rad e Function
with pelos wrappers de call_dispatch, Float pelas nativas/constantes do
owner float, Version at pelo adapter do owner de stdlib version. Constantes
Float permanecem valores, funções têm nome curto. Sem fórmula ou parser
de domínio neste owner. Um membro desconhecido conserva erro anterior.

Não expor `.deg`/`.rad` sobre valor Angle como função ligada: erro
`cannot access fields on type angle` no field; Float signum/to-bytes idem
com tipo float. `calc.pow.with` sem chamada continua
`function \`pow\` does not contain field \`with\`` no nome with.
`version(...).at` sem chamada conserva `unknown version component`; não
incluir at entre major/minor/patch nem mudar controles de campos ausentes.
Não instalar namespace reflexivo em instâncias; não modificar where aqui.

### `compiler/eval/bindings/value_methods.md` — retirar parser at duplicado

Após a medição 4, `eval_version_method_value` torna-se adapter AST→Args;
avalia args uma vez, conserva spans/ocorrências, insere receiver e delega ao
adapter de `compiler/stdlib/primitives_constructors/version.rs`. O parser
fechado e diagnóstico estão nesse owner; a fórmula continua `Version::at`.
Não conservar tolerância a named nem mensagem genérica anterior de aridade.
Não modificar outros helpers de state/counter/color/selector.

## Pendências concretas

O integrador deve incorporar os textos compartilhados, incluindo adendo
Angle no owner layout_types; auditar reexports/hubs caso se tornem necessários
para visibilidade, com L0 local anterior ao código; executar V15/V26 antes de
resselo; congelar L0 e seguir contrato, ataques, selo, RED e implementação por
autoridades próprias. Os adendos não alteram API pública nem PARITY_VERSION.
Nenhum gate funcional foi executado por este papel. Não há alegação de
paridade concluída, score de mutação ou independência atestada.

Integração coordenada com o autor raiz: o L0 version explicita os helpers
internos `version_type_field` e `dispatch_version_method`, mantendo o módulo
privado; os reexports cabem aos hubs proprietários que o integrador atualizará.
Para transporte de span Float, comparar identidade obtida de
`float_type_field` para `signum`/`from-bytes`/`to-bytes` permite conservar
callbacks privadas. O integrador escolhe nomes privados de wrappers Angle/With
sem mudar o contrato público curto já fixado.

Verificação documental desta entrega: `git diff --check` dos três prompts
terminou com exit 0; `git diff --name-only -- 01_core 02_shell 03_infra
04_wiring` estava vazio. Esses checks não substituem nenhum gate P1339.
