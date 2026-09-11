# Prompt L0 — constructor primitivo `version` e adapter de `at`
Hash do Código: 64e45df5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/primitives_constructors/version.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente aceita string numérica pontuada, qualquer quantidade de
componentes inteiros posicionais não negativos, ou um array desses componentes.
Argumentos nomeados e componentes `pre`/`build` são rejeitados. P1200 preserva
integralmente esse comportamento e individualiza somente ownership e linhagem.

## Contrato

`native_version(ctx, args, world, current_file) -> SourceResult<Value>`:

- uma string isolada usa `Version::from_str`;
- um array isolado ou os próprios posicionais fornecem zero ou mais componentes
  inteiros não negativos;
- zero componentes produz `Version::default()`;
- rejeitar negativos, componentes não inteiros, strings inválidas e quaisquer
  argumentos nomeados;
- retornar `Value::Version(Arc<Version>)` e permanecer L1 puro.

## Limites e gates

- `pre` e `build` não pertencem à linguagem Typst e não são aceitos.
- Não adicionar bump, conversão para string ou novas operações; a proibição
  inicial de `.at(index)` é sucedida exclusivamente pelo adendo P1339 abaixo.
- A entrada por tipo chamável delega a esta mesma free function; mudança da
  superfície pública permanece sob ADR-0127.

## Aceitação

Cobrir strings válidas e inválidas, cardinalidades arbitrárias, vazio, array,
negativos, não inteiros e argumentos nomeados rejeitados.

## P1339 — adapter único de `version.at`

### Medição anterior à decisão

Vanilla ratificado `a51e02804`, fonte
`lab/typst-original/crates/typst-library/src/foundations/version.rs:109–134`,
documenta índice i64, extensão positiva por zero e índice negativo relativo
aos componentes explícitos. A medição congelada
`00_nucleo/diagnosticos/p1339-full-final-vanilla-runs.json:2` (UTC e
proveniência integrais no recibo), sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4` com diff/stat vazio, contém os IDs
`discovery-version.at`, `version-static-*`, `version-bound-*`,
`version-invalid-*` e `version-method-value`; o suplemento boundaries cobre
as sobras ligadas. A rota é `(function, "at")`, retorna Int, e a obtenção de
`version(...).at` sem chamada falha com `unknown version component`.

`01_core/src/entities/version.rs:103–120` já possui a fórmula completa.
`01_core/src/compiler/eval/bindings/value_methods.rs:623–661` reimplementa
argparsing só ligado, usando span agregado e tolerando named se existir
um positional. Não basta descobrir uma segunda nativa com o parser antigo.

### Decisão e contrato proprietário

Criar neste consumer um adapter nativo interno de `at`, descoberto como
`version.at`, e fazer a forma ligada chamar o mesmo adapter. Ele recebe dois
posicionais obrigatórios: `self` estritamente Version e `index` estritamente
Int/i64; não converter string em Version nem Float/Bool em Int. Remove
receiver por operação causal de Args; delega valor e erro de índice a
`Version::at`. Nenhuma segunda fórmula, registry ou nova entidade.

Integração interna nomeada: `pub(crate) fn version_type_field(field: &str)
-> Option<Value>` descobre somente `at`; `dispatch_version_method` é helper
`pub(crate)` que recebe Version, nome do método e Args já avaliados, insere
receiver causalmente e chama a mesma nativa de `version_type_field`. O módulo
`version` permanece privado; reexports internos nos hubs proprietários não
transformam helpers ou módulo em API pública externa. O dispatcher seleciona
span integral pela identidade da função retornada no lookup, inclusive alias
ou With, e não por spelling `at`.

Parser por ordem declarada: consumir/castar `self`, consumir/castar `index`,
rejeitar sobras, depois executar a operação. Required fornecido apenas por
named produz `the argument \`self\` is positional` ou
`the argument \`index\` is positional`, com hint de remover `self:`/`index:`
e span da ocorrência named completa. Ausência produz
`missing argument: self`/`missing argument: index` na chamada inteira.
Cast usa `expected version, found <tipo>`/`expected integer, found <tipo>`
no value span. Extra positional produz `unexpected argument` e named não
consumido `unexpected argument: <nome>`, na primeira ocorrência sobrante.
Mensagem de out-of-bounds da entidade é preservada literalmente e ancora
a chamada inteira. Argumentos e spreads são avaliados uma vez pelo dispatcher
antes do parser; não reconstruir ordem/origem por mapa ou AST textual.

`version(v).at(i)` aqui designa chamada ligada sobre um valor Version já
construído, não conversão adicional: o glue insere o receiver e encaminha
Args/spans ao adapter. Aliases estáticos têm a mesma semântica. Este owner
não fornece acesso a `.at` como valor ligado; isso continua no field access.

Valor, tipo, erro, hint e spans são linguagem; formato Vec, Arc e escolha de
nativa Rust são mecânica. A intenção do índice vem da documentação citada,
os diagnósticos de parsing da medição. Inferência de suficiência do adapter é
refutada por erro/ordem que perca ocorrência entre estática e ligada.

Constructor, major/minor/patch, repr, display, comparação e `PARITY_VERSION`
permanecem intactos. A tensão histórica entre documentação e campos ausentes
de versão não é corrigida por esta rota. Não fecha paridade geral de Version.
Gate de implementação: contrato selado e RED independente P1339; depois
verificar índices positivos/negativos/extremos, lista vazia, erros, aliases e
preservações, com ownership 1:1 e V5/V15/V26 limpos.
