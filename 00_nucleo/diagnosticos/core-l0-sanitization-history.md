# Core L0 source archive before sanitization

This diagnostic preserves superseded process/history and the original ownership
mixtures. It is not a materialization prompt and grants no implementation authority.

## 00_nucleo/prompts/compiler/eval/observation/content.md

# Prompt L0 — observação de `Content`
Hash do Código: 80e35408

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/content.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline identificado por `compiler/eval/observation.md`, o impl exaustivo
de `ObservationEq for Content` está embutido em `compiler/eval/mod.rs` e já
contém as adaptações não commitadas de `SetPage` e `Styled`.

## Contrato

Este owner contém exclusivamente o match exaustivo de `Content`. Variantes
diferentes são `Different`; variantes iguais comparam todos os carriers
causais pela relação observacional. `Space` e `SpaceFromNewline`, presença
explícita e ausência, e identidades matemáticas distintas não são colapsadas.
Não executa show rules, layout, callbacks ou igualdade pública da linguagem.
A extração preserva byte por byte os corpos dos braços já vigentes.

## Aceitação

Adicionar ou alterar variante de `Content` continua a exigir match nominal e
falha de compilação quando o carrier não for coberto.


## 00_nucleo/prompts/compiler/eval/observation/elements.md

# Prompt L0 — observação dos demais elementos
Hash do Código: db1012b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/elements.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, elementos não matemáticos nem
propriamente de página ocupam a maior parte dos impls intercalados no
dispatcher. `ImageElem` já inclui o campo `alt` confirmado em P1481.

## Contrato

Este owner compara cada elemento restante por identidade de variante e todos
os campos causais do carrier. Incorpora o contrato P1374 para `BlockHeight` e
P1481 para `ImageElem::alt`, distinguindo ausência, string vazia e texto.
Não compara frames derivados, não executa callbacks ou layout e não usa nome
textual como substituto de identidade. Elemento dinâmico sem garantia de
imutabilidade permanece `Unproven`.

## Aceitação

Desestruturação nominalmente exaustiva impede omissão silenciosa de novo
campo; igualdade visual ou mecânica não substitui a morfologia causal.


## 00_nucleo/prompts/compiler/eval/observation/geometry.md

# Prompt L0 — observação de geometria, cor e pintura
Hash do Código: 249e53c8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/geometry.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, comprimentos, pontos, traços,
paths, cores, gradientes, tilings e demais carriers geométricos residem no
dispatcher de eval sem depender do seu controle de fluxo.

## Contrato

Este owner compara geometria, alinhamento, medidas relativas e absolutas,
cores, pinturas, traços, paths, gradientes e tilings por variante e campos
causais. Não rasteriza, não normaliza unidades, não resolve layout e não
substitui a igualdade da linguagem. Payloads float preservam bits IEEE por
delegação ao hub.

## Aceitação

Diferença inicial ou tardia em qualquer carrier coberto produz `Different` e
nenhum passo de render participa da comparação.


## 00_nucleo/prompts/compiler/eval/observation/math.md

# Prompt L0 — observação de matemática
Hash do Código: 0dab3b94

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/math.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, os impls matemáticos e os
helpers de regressão P1436–P1451 estão intercalados com elementos não
matemáticos no dispatcher central.

## Contrato

Este owner incorpora os contratos observacionais P1425, P1426, P1427,
P1436, P1437, P1439–P1444, P1446 e P1448–P1451 antes registrados em
`compiler/eval.md`. Cada carrier matemático compara todos os campos causais,
inclusive bits de presença explícita, estilo opcional, delimitadores,
anotações e slots; não compara glifos, frames ou constantes derivadas e não
resolve contexto ou layout. Identidades matemáticas diferentes permanecem
`Different` mesmo quando a aparência possa coincidir.

Os helpers crate-internos P1436–P1451 continuam reexportados pelo hub apenas
para testes existentes; não ampliam a API pública.

## Aceitação

Os testes proprietários P1425–P1451 continuam compilando e exercendo a mesma
relação nos dois sentidos.


## 00_nucleo/prompts/compiler/eval/observation/page.md

# Prompt L0 — observação de página
Hash do Código: 00e717dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/page.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, carriers de configuração,
execução e marginais de página estão intercalados com outras famílias.

## Contrato

Este owner compara papel, dimensões, margens, binding, bleed, fill,
suplemento, numeração, alinhamento, marginais, page runs, columns e carriers
de quebra/paridade por variante e campos causais. Não pagina, não realiza
header/footer e não serializa output. O carrier `SetPage` dentro de `Content`
permanece no owner de `Content` e delega aos tipos aqui cobertos.

## Aceitação

Configurações iguais são `Same`; qualquer diferença de presença, ordem ou
payload de página coberto é `Different`.


## 00_nucleo/prompts/compiler/eval/observation/style.md

# Prompt L0 — observação de estilos e tipografia
Hash do Código: 3e88f684

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/style.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, `StyleDelta`, `Styles` e
carriers tipográficos estão dispersos entre outros impls observacionais.

## Contrato

Este owner compara estilos, listas e nomes de fontes, língua, direção e
carriers tipográficos associados. Preserva ordem, presença, valores explícitos
e deltas; não resolve cascata, fallback de fonte ou render. Campos derivados
que não integram o carrier causal não são inventados pela comparação.

## Aceitação

Estilos iguais são `Same`; qualquer diferença causal coberta é `Different`;
estado tipográfico opaco sem prova não é promovido a `Same`.


## 00_nucleo/prompts/compiler/eval/observation/value.md

# Prompt L0 — observação de valores e carriers de runtime
Hash do Código: 7c1e388e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/value.rs`
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0129

## Medição anterior à decisão

No baseline de `compiler/eval/observation.md`, `Value` e os carriers de
runtime, seleção, argumentos, diagnóstico, funções e módulos compartilham o
monólito com o dispatcher embora formem a família causal da avaliação.

## Contrato

O match de `Value` é exaustivo e compara variante, presença, ordem e payload.
Este owner inclui carriers de runtime que não pertencem às famílias de estilo,
geometria, matemática, página ou elementos. `Scope` preserva ordem e valores;
funções nativas comparam identidade de chamada, nome e namespace; closures
somente são `Same` por identidade `Arc`; plugins e serviços sem prova
permanecem `Unproven`. Diagnósticos distinguem sucesso, erro, spans e metadata
causal. Nenhuma comparação executa o valor.

## Aceitação

Casos iguais, primeira ou última diferença e containers aninhados preservam o
resultado anterior à extração, inclusive zero assinado e NaN por bits.


## 00_nucleo/prompts/compiler/eval/observation.md

# Prompt L0 — `compiler/eval/observation` — hub da relação observacional
Hash do Código: cee130bc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/observation/mod.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

No HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, com working tree não
commitado e alteração sobreposta em `01_core/src/compiler/eval/mod.rs`, a
relação, o trait, os impls de folhas/containers e todas as famílias de
carriers residem no dispatcher central. O uso produtivo ocorre na validação
de leituras contextuais; a organização física não é observável pela língua.

## Contrato

Este ficheiro é o hub e owner de `ObservationRelation`, `ObservationEq`, dos
impls genéricos de referências, boxes, `Arc`, slices, arrays, vetores,
`Option`, `Result`, pares e `IndexMap`, e das folhas escalares auditadas.
Declara os módulos `content`, `value`, `style`, `geometry`, `math`, `page` e
`elements`, sem dispatcher dinâmico ou registry. Reexporta apenas os helpers
crate-internos já consumidos pelos testes estruturais; não cria API pública.

`Same.and(x)` conserva `Different` como prova dominante e `Unproven` como
ausência de prova. Folhas escalares usam igualdade exata; `f32` e `f64` usam
bits IEEE. Containers comparam cardinalidade, ordem e cada payload.

## Microbenchmarks internos

O owner contém, sob `cfg(test)`, um runner Criterion ignorado por defeito. Ele
mede explicitamente valores iguais, diferença inicial e tardia, árvores de
`Content`, estilos, geometria, carriers matemáticos, página e elementos. O
runner usa somente o trait privado e `black_box`; não expõe função, trait ou
módulo público, não entra em builds de produção e só executa por comando
explícito em perfil release. Resultados são evidência de comparação relativa,
nunca contrato semântico nem gate absoluto de tempo.

## Aceitação

- O comportamento e os testes do eval permanecem inalterados.
- O hub contém a relação e os impls genéricos, não impls das famílias filhas.
- Cada ficheiro filho cita exatamente o seu próprio Prompt L0.
- `cargo check -p typst-core --lib` e `crystalline-lint .` fecham sem nova
  violação atribuível à extração.


## 00_nucleo/prompts/compiler/parse/parser/checkpoints.md

# Prompt L0 — parser checkpoints
Hash do Código: 18248eca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/parser/checkpoints.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Medição anterior à decisão

Working tree em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, medido em 2026-09-26T12:28:06Z: parser.rs:69-185 declara estado; :188-492 concentra consultas, consumo e lexer; :494-585 checkpoints/memo; :587-732 diagnósticos/recovery. wrap_error (:362-378) pertence aos diagnósticos apesar da posição anterior. Baseline e inventário preservados no diagnóstico parser-atomization.md. São mecanismos internos, não mudança da língua (ADR-0107).

## Decisão e contrato

Owner de MemoArena, MemoKey, Checkpoint e PartialState e métodos de checkpoint, restore e memoização. Snapshot contém node_len, cursor do lexer, SyntaxMode e clone de Token. Restore trunca nodes e restaura estado parcial; não restaura depth, balanced nem nl_mode. Memo é indexado pelo início do token; arena guarda ranges de nós. Hit acrescenta os nós memoizados sem truncar nós atuais e restaura lookahead e lexer; miss retorna chave e checkpoint. Não altera gramáticas nem política de lifetime do memo.

Cada arquivo possui este owner exclusivo. Invariantes compartilhados usam o núcleo parse/core pinado acima. Métodos movidos conservam alcance limitado a compiler::parse; não usar pub(crate).

## Aceitação

Preservar texto, trivia, offsets, CST, modos, erros e progresso observáveis. Testes locais de estado, rollback/replay e recuperação, suíte parse existente, cargo build e crystalline-lint sem violations. Nenhuma mudança de contrato público, fase ou comportamento padrão.


## 00_nucleo/prompts/compiler/parse/parser/diagnostics.md

# Prompt L0 — parser diagnostics
Hash do Código: 129a37ec

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/parser/diagnostics.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Medição anterior à decisão

Working tree em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, medido em 2026-09-26T12:28:06Z: parser.rs:69-185 declara estado; :188-492 concentra consultas, consumo e lexer; :494-585 checkpoints/memo; :587-732 diagnósticos/recovery. wrap_error (:362-378) pertence aos diagnósticos apesar da posição anterior. Baseline e inventário preservados no diagnóstico parser-atomization.md. São mecanismos internos, não mudança da língua (ADR-0107).

## Decisão e contrato

Owner de expect, expected, expected_at, warnings/hints, unexpected, trim_errors, wrap_error, diagnóstico de delimitador e proteção de profundidade. Mantém supressão de erros esperados consecutivos, remoção de erros vazios antes de unexpected, erro de palavra-chave quando Ident esperado e balanceamento persistente. Limite 256; guard aumenta e devolve depth ao sair. Recovery consome pelo menos um token, respeita balanceamento e stop-set, preserva texto num erro agregado. Warning conserva severidade fornecida e comprimento zero. Mantém mensagens existentes e não assume diagnósticos específicos das gramáticas.

Cada arquivo possui este owner exclusivo. Invariantes compartilhados usam o núcleo parse/core pinado acima. Métodos movidos conservam alcance limitado a compiler::parse; não usar pub(crate).

## Aceitação

Preservar texto, trivia, offsets, CST, modos, erros e progresso observáveis. Testes locais de estado, rollback/replay e recuperação, suíte parse existente, cargo build e crystalline-lint sem violations. Nenhuma mudança de contrato público, fase ou comportamento padrão.


## 00_nucleo/prompts/compiler/parse/parser/state.md

# Prompt L0 — parser state
Hash do Código: d07e04e0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/parser/state.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Medição anterior à decisão

Working tree em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, medido em 2026-09-26T12:28:06Z: parser.rs:69-185 declara estado; :188-492 concentra consultas, consumo e lexer; :494-585 checkpoints/memo; :587-732 diagnósticos/recovery. wrap_error (:362-378) pertence aos diagnósticos apesar da posição anterior. Baseline e inventário preservados no diagnóstico parser-atomization.md. São mecanismos internos, não mudança da língua (ADR-0107).

## Decisão e contrato

Owner de criação/finalização, consultas de lookahead, Index/IndexMut por Marker, consumo, trivia, wrapping CST e ponte ao lexer. Token corrente não integra nodes. Preserva offsets e texto integral, clamp de marker antes da trivia, transições delimitadas de SyntaxMode e AtNewline e restauração do falso End. Preserva exatamente as regras ContextualContinue, StopParBreak e RequireColumn (comparação column <= min_col), bem como preservação dos erros léxicos InvalidHexNumber e InvalidUnicodeCodepoint na saída de modo. Não produz novos diagnósticos nem implementa produções gramaticais.

Cada arquivo possui este owner exclusivo. Invariantes compartilhados usam o núcleo parse/core pinado acima. Métodos movidos conservam alcance limitado a compiler::parse; não usar pub(crate).

## Aceitação

Preservar texto, trivia, offsets, CST, modos, erros e progresso observáveis. Testes locais de estado, rollback/replay e recuperação, suíte parse existente, cargo build e crystalline-lint sem violations. Nenhuma mudança de contrato público, fase ou comportamento padrão.


## 00_nucleo/prompts/compiler/parse/parser.md

# Prompt L0 — parser hub
Hash do Código: c3f33741

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/parser.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Medição anterior à decisão

Working tree em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, medido em 2026-09-26T12:28:06Z: parser.rs:69-185 declara estado; :188-492 concentra consultas, consumo e lexer; :494-585 checkpoints/memo; :587-732 diagnósticos/recovery. wrap_error (:362-378) pertence aos diagnósticos apesar da posição anterior. Baseline e inventário preservados no diagnóstico parser-atomization.md. São mecanismos internos, não mudança da língua (ADR-0107).

## Decisão e contrato

Declara Parser, Token, Newline, AtNewline e Marker e compõe state, checkpoints e diagnostics. Conserva os campos e alcance das gramáticas; nl_mode e metadados internos de trivia permanecem privados ao motor. Sem dispatcher dinâmico, API nova ou mudança de gramática.

Cada arquivo possui este owner exclusivo. Invariantes compartilhados usam o núcleo parse/core pinado acima. Métodos movidos conservam alcance limitado a compiler::parse; não usar pub(crate).

## Aceitação

Preservar texto, trivia, offsets, CST, modos, erros e progresso observáveis. Testes locais de estado, rollback/replay e recuperação, suíte parse existente, cargo build e crystalline-lint sem violations. Nenhuma mudança de contrato público, fase ou comportamento padrão.


## 00_nucleo/prompts/compiler/stdlib/registry/contextual.md

# Prompt L0 — stdlib/registry/contextual
Hash do Código: 1300f488

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/contextual.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra metadata,state,counter,context, operações históricas state/counter,
query,selector,here,target,locate. State/counter/selector permanecem Type;
as nativas conservam ABI. Não decide target nem executa leituras contextuais.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/data.md

# Prompt L0 — stdlib/registry/data
Hash do Código: e4417b40

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/data.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra read,csv,json,yaml,toml,cbor,xml,plugin. JSON/YAML/TOML são pais
chamáveis com apenas encode de nome curto. CBOR conserva cbor.encode e plugin
conserva plugin.transition. Sem novos aliases globais de encoder, serializers
ou alteração de assinatura das nativas (P1307-R3).

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md

# Prompt L0 — stdlib/registry/diagnostics
Hash do Código: 91473bbf

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/diagnostics.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra assert com eq/ne, panic, numbering e eval. Numbering/eval conservam
ABI Engine; os outros continuam as nativas vigentes. Sem validação local.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/figure_image.md

# Prompt L0 — stdlib/registry/figure_image
Hash do Código: d6ffadd7

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/figure_image.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra figure com caption, image e Type::Tiling, preservando P1473.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/formats.md

# Prompt L0 — stdlib/registry/formats
Hash do Código: 51878b31

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/formats.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Constrói PDF e HTML exatamente uma vez, cria um único asset, fornece esses
valores por referência à factory format e registra format, asset somente com
Bundle e html. Devolve o PDF construído para a inserção posterior. Html não
ativa sua própria feature; os gates de lookup continuam nos owners de eval.
Conserva P1464/P1465/P1471/P1485, sem reconstruir funções ou catálogos.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/foundations.md

# Prompt L0 — stdlib/registry/foundations
Hash do Código: e7702def

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/foundations.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra os tipos e cinco constructors globais de cor (rgb, luma, oklab,
oklch, cmyk), repr/range e tipos primitivos. Não instala hsl, hsv, linear_rgb
ou linear-rgb bare. Em trecho final registra alinhamentos/direções. Conserva
P1300 e a chamabilidade vigente de Type, sem constructor próprio.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/layout.md

# Prompt L0 — stdlib/registry/layout
Hash do Código: 35d1672a

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/layout.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra grid com cell/header/footer/hline/vline de nomes curtos, transforms,
align/place.flush e, em trecho posterior, pad/hide/h/v/linebreak/parbreak/
pagebreak/page/block/box/stack/repeat/columns/colbreak/measure/layout/stroke.
Page usa ABI Engine; stroke permanece Type. Place.flush conserva nome flush.
Não incorpora lógica de layout ou validação (P1292/P1293).

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/math.md

# Prompt L0 — stdlib/registry/math
Hash do Código: 7f4a553a

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/math.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra os 18 globais matemáticos históricos como compatibilidade cristalina
(P1488), incluindo op com ABI Engine. Não remove extras e não cria
math.underover; cancel global não é substituído pelo qualificado.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md

# Prompt L0 — microbenchmarks internos da stdlib
Hash do Código: ec3fcf9e

**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/microbenchmarks.rs`
**Camada:** L1 test-only; não materializa API produtiva.

## Medição anterior à decisão

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitada:
`01_core/src/compiler/stdlib/registry/mod.rs:26–46` compõe o registro com
inputs/features explícitos, em função crate-private. `01_core/Cargo.toml`
já declara Criterion como dev-dependency; `compiler/eval/observation/mod.rs`
contém runner Criterion ignored. `entities/func.rs:315` permite obter o
ponteiro da nativa; `stdlib/text/case.rs:23` valida named, tipo e aridade.

## Decisão e contrato

Adicionar runner ignored descendente do registry, exclusivamente cfg(test),
sem tornar públicos factories ou helpers. Este recorte interno ADR-0127 não
altera linguagem, defaults, fase ou compatibilidade. É uma exceção localizada
à preferência de ADR-0115 por benches/: medir diretamente o registro privado
evita ampliar API. Criterion e relógio existem apenas no runner test-only.

Medir construção e destruição do Scope nos oito perfis Html/A11yExtras/Bundle;
escala de inputs vazios, 16 e 256 entradas; lookup global presente/ausente e
lookup em calc; chamadas via ponteiro previamente resolvido para calc.abs e
upper, com caminhos válidos e rejeições de tipo, aridade e named.
Fixtures, resolução das nativas, World e EvalContext ficam fora do trecho
cronometrado. World não realiza I/O e falha se fonte/arquivo for solicitado.
Não medir parser, eval dispatcher, cache, layout ou exportação sob esses nomes.
Construção de outputs/diagnósticos e sua destruição integram as chamadas.
Usar black_box para inputs e outputs, asserts semânticos antes de medir,
30 amostras, warmup 150 ms e janela 300 ms; rejeitar execução debug.

## Aceitação e limites

Controles normais verificam oito perfis distintos, 178/179 bindings e gates;
chamadas válidas têm expected fixo e inválidas retornam diagnóstico não vazio.
Executar controles RED→GREEN antes do runner release. Registrar HEAD, UTC,
diff/status completos, identidade do executável, configuração e resultados em
`00_nucleo/diagnosticos/stdlib-microbenchmarks.md`. Números iniciais não provam
melhoria, estabilidade nem comparação vanilla. Refinamentos posteriores exigem
baseline/candidato, mesma máquina e repetição; não otimizar nesta rodada.


## 00_nucleo/prompts/compiler/stdlib/registry/mod.md

# Prompt L0 — stdlib/registry/mod
Hash do Código: 108a61a6

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/mod.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Compositor único: Scope fresco por invocação, inputs e features explícitos;
chama as famílias na sequência medida e preserva exatamente a ordem vigente.
Formats retorna o PDF único, inserido mais tarde por modules. Nenhum registry
dinâmico, wildcard de import, lookup reflexivo, estado global ou wrapper de
nativa. O conjunto de claims compartilhadas de features permanece pinado
nos owners de comportamento; este módulo só transporta o argumento explícito.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Declara somente sob cfg(test) o filho microbenchmarks, cujo owner exclusivo é
`00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md`. Não contém
relógio, Criterion ou API produtiva de benchmarking.

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/modules.md

# Prompt L0 — stdlib/registry/modules
Hash do Código: f599ce8e

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/modules.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Insere sym,calc,emoji,PDF previamente construído,color,gradient,math e sys
com inputs fornecidos. Color/gradient são Type; math usa factory proprietária.
Não reconstrói PDF nem interpreta inputs.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/shapes.md

# Prompt L0 — stdlib/registry/shapes
Hash do Código: 1fc5ef05

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/shapes.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra rect,square,ellipse,circle,line,polygon.regular e curve com
move,line,cubic,quad,close. Preserva os nomes qualificados existentes (P1473).

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/structural.md

# Prompt L0 — stdlib/registry/structural
Hash do Código: 145f0e33

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/structural.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra heading/title/outline/par/raw, terms/divider/quote/document/footnote,
bibliography/cite/link/label/ref/list/enum em três trechos ordenados.
Outline.entry preserva body,prefix,indented,inner,page e seus ABIs. Par.line
usa nome curto line; raw.line mantém Func::element. List/enum/terms possuem
item; footnote possui entry. Conserva P1402–P1405/P1413/P1414/P1473.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/table_grid.md

# Prompt L0 — stdlib/registry/table_grid
Hash do Código: 13d49f9c

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/table_grid.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra table com ordem header,footer,cell,hline,vline e nomes públicos
curtos; conserva os seis aliases flat históricos de table/grid e não cria
*_hline/*_vline flat (P1293).

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/tests.md

# Prompt L0 — stdlib/registry/tests
Hash do Código: 8756d111

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/tests.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Contém controles da superfície anterior: sequência completa de bindings nos
oito perfis, namespaces e projeções de format, gates PDF e bundle, inputs
e scopes frescos. Expected congelado antes da extração; o candidato não
produz seus próprios expected. Este arquivo é declarado uma só vez: durante
o baseline como filho de eval e após a extração como filho de registry.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/registry/text.md

# Prompt L0 — stdlib/registry/text
Hash do Código: cbdc7ef8

**Camada:** L1
**Consumer exclusivo:** `01_core/src/compiler/stdlib/registry/text.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada
de 2026-09-26: `01_core/src/compiler/eval/mod.rs:2260–3144` contém
o registro monolítico. Estado exato, inventário e testes ficam em
`00_nucleo/diagnosticos/stdlib-registry-atomization.md`.

## Contrato proprietário

Registra decoração/lorem/regex em um trecho e upper/lower/replace em outro.
Preserva sub/super/strike/overline/highlight e nativas existentes.

O recorte conserva a superfície vigente, nomes públicos, função nativa ligada,
kind do Value, filhos e aliases; não implementa validação de Args, casts,
exportação ou lookup. Essas operações pertencem aos owners nativos existentes.
Sem mudança de assinatura externa, default, fase ou compatibilidade: ADR-0127
em fluxo contínuo. Os exemplos históricos em
`00_nucleo/prompts/compiler/eval.md` são proveniência, não segundo ownership.

## Aceitação

Verificar nomes e kinds, namespaces, projeções de format, ausência dos três
constructors de cor extras e sys.inputs nos oito perfis de features. Executar
os testes existentes de eval/stdlib e build; V15/V26 e hashes individuais
devem permanecer válidos. A ordem congelada é controle desta refatoração,
não alegação nova de paridade com a mecânica vanilla.


## 00_nucleo/prompts/compiler/stdlib/tests/arguments.md

# Prompt L0 — stdlib tests/arguments
Hash do Código: 27a739d0

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/arguments.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `arguments`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/assertions.md

# Prompt L0 — stdlib tests/assertions
Hash do Código: f2a47d77

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/assertions.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `assertions`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/bibliography.md

# Prompt L0 — stdlib tests/bibliography
Hash do Código: 25c2cb92

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/bibliography.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `bibliography`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/calc.md

# Prompt L0 — stdlib tests/calc
Hash do Código: c0a7dcc1

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/calc.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `calc`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/color.md

# Prompt L0 — stdlib tests/color
Hash do Código: d1263437

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/color.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `color`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/contextual.md

# Prompt L0 — stdlib tests/contextual
Hash do Código: a3f90e6f

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/contextual.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `contextual`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/eval.md

# Prompt L0 — stdlib tests/eval
Hash do Código: ec6fb7aa

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/eval.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `eval`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/figure_image.md

# Prompt L0 — stdlib tests/figure_image
Hash do Código: 6091cb22

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/figure_image.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `figure_image`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/footnote.md

# Prompt L0 — stdlib tests/footnote
Hash do Código: 9ea0d8f1

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/footnote.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `footnote`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/foundations.md

# Prompt L0 — stdlib tests/foundations
Hash do Código: 6b4c6138

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/foundations.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `foundations`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/gradients.md

# Prompt L0 — stdlib tests/gradients
Hash do Código: 7198d06c

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/gradients.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `gradients`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/introspection.md

# Prompt L0 — stdlib tests/introspection
Hash do Código: 6bf7ccec

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/introspection.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `introspection`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/layout_containers.md

# Prompt L0 — stdlib tests/layout_containers
Hash do Código: 21aed5ea

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/layout_containers.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `layout_containers`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/layout_context.md

# Prompt L0 — stdlib tests/layout_context
Hash do Código: da778bd2

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/layout_context.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `layout_context`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/layout_spacing.md

# Prompt L0 — stdlib tests/layout_spacing
Hash do Código: 45155acb

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/layout_spacing.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `layout_spacing`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/math.md

# Prompt L0 — stdlib tests/math
Hash do Código: aece2733

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/math.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `math`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/mod.md

# Prompt L0 — stdlib tests/mod
Hash do Código: c526a531

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/mod.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Declara support antes das famílias para disponibilizar null_ctx; importa
os símbolos dos namespaces e reúne os módulos test-only, sem assertions
ou lógica produtiva.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/shapes.md

# Prompt L0 — stdlib tests/shapes
Hash do Código: 2447fe7d

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/shapes.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `shapes`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/stroke.md

# Prompt L0 — stdlib tests/stroke
Hash do Código: b3232c21

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/stroke.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `stroke`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/support.md

# Prompt L0 — stdlib tests/support
Hash do Código: 26d5c505

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/support.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém somente as fixtures comuns: Args, CounterKey, NullWorld, contexto,
Engine e FileId sintéticos, além dos helpers históricos compartilhados.
Todos são privados ao módulo de testes; World continua sem I/O real.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/symbols.md

# Prompt L0 — stdlib tests/symbols
Hash do Código: b8fd4268

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/symbols.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `symbols`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/table_grid.md

# Prompt L0 — stdlib tests/table_grid
Hash do Código: c5678090

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/table_grid.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `table_grid`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/text.md

# Prompt L0 — stdlib tests/text
Hash do Código: d72b0282

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/text.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `text`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/compiler/stdlib/tests/transforms.md

# Prompt L0 — stdlib tests/transforms
Hash do Código: 30348ff3

**Camada:** L1 test-only
**Consumer exclusivo:** `01_core/src/compiler/stdlib/tests/transforms.rs`

## Medição anterior à decisão

Em HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não
commitada de 2026-09-26, a suíte inline de
`01_core/src/compiler/stdlib/mod.rs:231–14087` mistura testes de namespaces
e fixtures. A proveniência integral e o inventário ficam em
`00_nucleo/diagnosticos/stdlib-tests-atomization.md`.

## Decisão e aceitação

Contém exclusivamente as regressões históricas da família `transforms`,
extraídas da suíte inline. Usa as fixtures comuns de support.
A extração preserva nomes locais, fixtures, argumentos, assertions, mensagens,
spans e observáveis dos testes existentes. Só paths e visibilidade privada de
suporte podem mudar. Este owner não legitima código de produção nem transfere
contratos dos namespaces. Não criar testes ou expectativas a partir da saída
do candidato. Verificar descoberta bijetiva dos 742 nomes e execução da suíte.


## 00_nucleo/prompts/entities/content/construction.md

# Prompt L0 — Content construction
Hash do Código: 17826cc5

**Layer:** L1  
**Owner:** `01_core/src/entities/content/construction.rs`

## Contract

Own public `Content` constructors other than sequence normalization. Moving
them out of the model owner must preserve every signature, default, omission
state, and payload. Construction remains pure L1 and uses only closed native
variants plus the existing explicit `Dynamic` extension boundary. No registry,
property map, I/O, or rendering is allowed.

## Acceptance

All existing constructor, representation, morphology, and parity tests pass;
callers compile unchanged; the module contains simultaneous verification.


## 00_nucleo/prompts/entities/content/emptiness.md

# Content semantic emptiness
Hash do Código: c7d9184f

**Layer:** L1
**Owner:** `01_core/src/entities/content/emptiness.rs`

## Contract

Own the semantic `Content::is_empty` projection. The exhaustive static match
must preserve which variants are observable and delegate element-specific
semantics to their owners.

## Acceptance

Existing emptiness behavior and tests remain unchanged; no rendering, I/O, or
dynamic registry is introduced.


## 00_nucleo/prompts/entities/content/equality.md

# Prompt L0 — Content structural equality
Hash do Código: c18f9f0d

**Layer:** L1  
**Owner:** `01_core/src/entities/content/equality.rs`

## Contract

Own `PartialEq` and `Eq` for the closed `Content` algebra. Preserve all existing
distinctions and intentional equivalences, including
`Space == SpaceFromNewline`. Rust hashing is owned independently by
`entities/content/rust-hash.md`; language morphology is owned separately.

## Acceptance

Existing equality and representation tests pass; matches remain static and the
module contains simultaneous verification.


## 00_nucleo/prompts/entities/content/fields.md

# Content named-field projection
Hash do Código: bfc0858d

**Layer:** L1
**Owner:** `01_core/src/entities/content/fields.rs`

## Contract

Own `Content::get_field` and its transparent styled-equation lookup. Preserve
field presence, omission, style precedence, and delegation to element owners.
Unsupported fields return `None`.

## Acceptance

Existing field-access and show-rule tests pass. The implementation remains a
closed static match and does not introduce a property registry.


## 00_nucleo/prompts/entities/content/identity.md

# Prompt L0 — Content identity
Hash do Código: 12678c35

**Layer:** L1  
**Owner:** `01_core/src/entities/content/identity.rs`

## Purpose

Provide the stable diagnostic name and bounded structural debug representation
of every closed `Content` variant through `Content::elem_name` and
`Debug for Content`.

## Contract

- The implementation is a nominal, exhaustive, statically dispatched match
  over `Content`.
- It must not use a wildcard arm, dynamic dispatch, a registry, or a property
  map.
- Moving this behavior out of `entities/content.rs` must not change any
  returned string.
- `Space` and `SpaceFromNewline` both return `"space"`, because their origin
  distinction is causal and not part of the public element identity.
- The method remains public with the signature
  `pub fn elem_name(&self) -> &'static str`.
- Adding a `Content` variant must force this owner to classify its diagnostic
  name at compile time.
- Debug formatting remains depth-bounded and delegates to an exhaustive static
  match. Existing output strings and payload formatting must not change.

## Acceptance

- Existing callers compile without changes.
- Existing identity and field-access tests remain green.
- `cargo test -p typst-core --lib` passes.
- `crystalline-lint .` reports no ownership, lineage, or prompt-drift
  violation after hashes are resealed.


## 00_nucleo/prompts/entities/content/morphology.md

# Prompt L0 — Content morphology
Hash do Código: 05fe993b

**Layer:** L1  
**Owner:** `01_core/src/entities/content/morphology.rs`

## Contract

Own `Content::morph_canon`, the canonical form used for language-level
morphology. Preserve ADR-0107: rendering mechanics and baked style details do
not become language identity. Moving the implementation must not alter any
canonical form or equality outcome.

## Acceptance

Existing morphology and language-equality tests pass and the module contains
simultaneous verification.


## 00_nucleo/prompts/entities/content/page-structure.md

# Prompt L0 — Content page structure
Hash do Código: b30feba6

**Layer:** L1  
**Owner:** `01_core/src/entities/content/page_structure.rs`

## Contract

Own structural page and column segmentation and `wrap_page_columns`.
Preserve source order, nested content, page markers, column markers, page
configuration, and degenerate-case normalization. The behavior is pure and
must not perform layout, measurement, rendering, or I/O.

## Acceptance

Existing page/column tests pass unchanged and the module contains simultaneous
verification.


## 00_nucleo/prompts/entities/content/plain-text.md

# Content plain-text projection
Hash do Código: 4ce383c8

**Layer:** L1
**Owner:** `01_core/src/entities/content/plain_text.rs`

## Contract

Own recursive `Content::plain_text` projection. Preserve content order, textual
markers, transparent wrappers, and element-specific delegation without making
rendered bytes or Rust representation part of the language contract.

## Acceptance

Existing plain-text, representation, and morphology tests pass; the operation
remains pure and statically dispatched.


## 00_nucleo/prompts/entities/content/rust-hash.md

# Content Rust hashing
Hash do Código: fba6070e

**Layer:** L1
**Owner:** `01_core/src/entities/content/rust_hash.rs`

## Contract

Own the Rust `Hash` implementation for `Content`. Hashing must remain compatible
with `PartialEq`, including intentional equivalence between `Space` and
`SpaceFromNewline`, while preserving causal fields for specialized carriers.
This is mechanical Rust identity, not language morphology.

## Acceptance

Existing hash and cache tests pass; equal values produce equal hashes and the
implementation remains pure and statically dispatched.


## 00_nucleo/prompts/entities/content/sequence.md

# Content sequence normalization
Hash do Código: c07ae498

**Layer:** L1
**Owner:** `01_core/src/entities/content/sequence.rs`

## Contract

Own construction and normalization of `Content::Sequence`. Empty input becomes
`Content::Empty`, a singleton is unwrapped, and multiple items use the closed
sequence carrier without changing order.

## Acceptance

Normalization remains pure, deterministic, allocation-aware, and covered by a
local test. All callers retain the existing `Content::sequence` API.


## 00_nucleo/prompts/entities/content/tests/morphology.md

# Content morphology tests
Hash do Código: 82eb78ce

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/morphology.rs`

## Contract

Own regression coverage for language morphology identity and distinctions that must not collapse into Rust representation mechanics. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

The extracted cases retain their assertions and fixtures unchanged, execute as
part of the `Content` test suite, and introduce no productive dependency.


## 00_nucleo/prompts/entities/content/tests/projections.md

# Content projection tests
Hash do Código: 335d0915

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/projections.rs`

## Contract

Own regression coverage for semantic emptiness, plain-text projection, and named-field observations across content variants. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

The extracted cases retain their assertions and fixtures unchanged, execute as
part of the `Content` test suite, and introduce no productive dependency.


## 00_nucleo/prompts/entities/content/tests/representation.md

# Content representation tests
Hash do Código: 8afa531e

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/representation.rs`

## Contract

Own regression coverage for construction, closed-variant representation, structural equality, cloning, defaults, and payload preservation. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

The extracted cases retain their assertions and fixtures unchanged, execute as
part of the `Content` test suite, and introduce no productive dependency.


## 00_nucleo/prompts/entities/content/tests/sequence.md

# Content sequence tests
Hash do Código: 24e68dd6

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/sequence.rs`

## Contract

Own regression coverage for sequence normalization, ordering, cloning, equality, and nested sequence behavior. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

The extracted cases retain their assertions and fixtures unchanged, execute as
part of the `Content` test suite, and introduce no productive dependency.


## 00_nucleo/prompts/entities/content/tests/traversal.md

# Content traversal tests
Hash do Código: 4f658ae8

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/traversal.rs`

## Contract

Own regression coverage for bottom-up `map_content`, recursive `map_text`, terminal preservation, and payload reconstruction. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

The extracted cases retain their assertions and fixtures unchanged, execute as
part of the `Content` test suite, and introduce no productive dependency.


## 00_nucleo/prompts/entities/content/tests.md

# Content behavioral test organization
Hash do Código: de69f6e2

**Layer:** L1 test-only
**Owner:** `01_core/src/entities/content/tests/mod.rs`

## Contract

Own only the composition of the behavior-focused `Content` test modules:
representation, sequence, traversal, morphology, and projections. Productive
behavior remains owned by the corresponding prompts under
`00_nucleo/prompts/entities/content/`.

The module must not contain test logic, helpers shared with production, or a
second implementation of any `Content` behavior.

## Acceptance

All five test modules execute through the standard core test suite and remain
independently readable. Test organization does not alter production behavior.


## 00_nucleo/prompts/entities/content/traversal.md

# Prompt L0 — Content traversal
Hash do Código: 252a3e45

**Layer:** L1  
**Owner:** `01_core/src/entities/content/traversal.rs`

## Contract

Own bottom-up content transformation and text transformation through
`map_content` and `map_text`. Traversal remains exhaustive, static,
order-preserving, and pure. Variant-specific recursion delegates to the owning
element. Replacement nodes are not recursively reprocessed unless the
existing contract already requires it.

## Acceptance

Existing traversal tests pass unchanged and the module contains simultaneous
verification.


## 00_nucleo/prompts/entities/content.md

# Prompt L0 — Content
Hash do Código: 9f59fa58

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

## P1505 — Bounded `Content` representation A/B (RATIFIED; gate confirmed 2026-09-26)

### Measurement before decision

The reproducible measurement is recorded in
`00_nucleo/diagnosticos/adr-performance-quantity-analysis.md`. At repository
HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, using the release binary with
SHA-256 `55142cb384c6fbfac10ad4d617a2f5eda611a005c412c001b4883ce60a963826`,
`size_of::<Content>()` is 664 bytes. The pinned vanilla
`9f2b6e8715237cb086899a42873660fe744622e8` representation is 24 bytes.

The complete inline `SetPage` payload is also 664 bytes. Because a Rust enum
has room for its largest variant, this rare marker determines the size of
every `Content`, including `Empty`, `Space`, and `Text`. `Styles` is 328 bytes,
so `Styled(Box<Content>, Styles)` becomes the next size floor after `SetPage`
is made indirect.

A controlled release-artifact benchmark cloned a vector of 100,000 real text
`Content` values ten times. Vanilla measured 12.2 ms and 8.3 MiB peak RSS;
Tekt measured 414.3 ms and 131.8 MiB. End-to-end Tekt peak RSS reached 187.0
MiB for 200 context/measure calls and 324.2 MiB for the multilingual UTF-8
case, respectively 4.80× and 4.73× vanilla. This establishes a representation
risk large enough to justify an isolated A/B; it does not by itself attribute
all pipeline time to `Content`.

### Proposed public-contract decision

Keep `Content` as a closed enum with exhaustive static dispatch. Do not adopt
the vanilla vtable, `unsafe`, proc macros, a property map, or dynamic dispatch
for native elements. Refine only the storage of the two oversized variants:

```rust
pub struct SetPageDelta {
    // exactly the fields currently carried inline by Content::SetPage
}

pub struct StyledContent {
    pub body: Content,
    pub styles: Styles,
}

pub enum Content {
    // unchanged variants omitted
    SetPage(Arc<SetPageDelta>),
    Styled(Arc<StyledContent>),
}
```

Both payload structs remain in `entities/content.rs` and are owned by this L0;
this avoids creating a second productive consumer or violating ADR-0129.
They are immutable after construction. `Arc` provides O(1) clone and keeps the
closed-enum/static-match architecture required by ADR-0026, ADR-0106, and
ADR-0109.

This is a public Rust contract change: direct construction and pattern
matching change from struct/tuple variants to an `Arc` payload. Existing
constructor functions remain semantically unchanged, but all in-repository
direct matches must migrate atomically. Language semantics, syntax,
morphology, `repr`, `plain_text`, equality, hashing, style precedence, page
configuration, and PDF behavior must not change.

### Isolated A/B sequence

The two representation changes must not be combined into one measurement:

1. **A — baseline:** retain the P1505 diagnostic measurements from the exact
   release artifact above.
2. **B — `SetPage` only:** introduce `SetPageDelta` and
   `Content::SetPage(Arc<SetPageDelta>)`; update all matches and constructors;
   rebuild release; record `size_of::<Content>()`, the controlled clone test,
   the seven-case corpus, phase timings, peak RSS, output morphology, commit or
   uncommitted-state provenance, timestamp, and binary SHA-256.
3. **C — `Styled` only over B:** introduce `StyledContent` and
   `Content::Styled(Arc<StyledContent>)`; repeat the identical measurements.

Results for B and C belong in a new diagnostic, not in this normative prompt.
The implementation may be retained only after its own measurement is
recorded. If either stage regresses the comparable corpus beyond run-to-run
noise, preserve the evidence and revert that stage without weakening the
semantic invariants.

The completed measurements are recorded in
`00_nucleo/diagnosticos/p1505-content-representation-ab.md`. Both stages are
retained: `Content` fell from 664 to 336 to 120 bytes, the controlled clone
fell from 414.3 ms to 239.3 ms to 35.7 ms, and every end-to-end case improved
at C relative to A without a language-level regression.

### Acceptance

- Tests are written or migrated before each productive stage and demonstrate
  the expected public shape and unchanged behavior.
- Stage B changes only `SetPage` storage; Stage C changes only `Styled`
  storage. No algorithmic optimization is mixed into either comparison.
- `size_of::<Content>()` is asserted in a release-compatible size guard with a
  bound justified by the measured stage, so another rare inline payload cannot
  silently inflate every node.
- `SetPageDelta` preserves all omission distinctions, including nested
  `Option` states for numbering, background, and foreground.
- `StyledContent` preserves the exact body and `Styles`; semantic-empty style
  transparency and style precedence remain unchanged.
- The exhaustive matches remain static. `Content::Dynamic` is not widened or
  used as a replacement for either native variant.
- Acceptance is at the language level under ADR-0107. PDF byte equality is
  not a gate unless bytes are the observable under test; morphology and the
  existing parity probes are.
- `cargo build`, the relevant tests, and `crystalline-lint .` finish without
  violations after hashes are resealed.

The owner confirmed this public Rust contract after the L0 update on
2026-09-26 under the ADR-0127 stop-and-confirm gate. Productive
materialization of both isolated stages is authorized and measured.

## P1494 — proveniência de newline em espaço de markup (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, com working tree limpo
durante as sondas, o vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8
(binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea)
produz 水果 para ideograma–newline–ideograma e 水 果 para espaço explícito;
o cristalino (binário SHA-256
35a2fdea38c3e5a9b8b0c2a8a93c1cf9f4b6439965ed0103aa0e564c9ca3ca83)
produz 水 果 para ambos. O controle de igualdade da linguagem entre [newline]
e [espaço] passa em ambos. Na fonte pinada, typst-eval/src/markup.rs:98-107
conserva had_newline e typst-realize/src/spaces.rs:180-205 consome a origem.
No consumer cristalino content.rs:157-172,1275,3598,3781,4020-4034, Space
é variante unitária pública, sem origem; Debug serve de entrada ao hash.

### Decisão pública ratificada

Adicionar Content::SpaceFromNewline como variante unitária para o espaço de
markup inferido por ao menos uma quebra de linha. Content::Space mantém a
forma unitária e os callers atuais, designando espaço sem essa origem. As
duas variantes são indistinguíveis na linguagem: mesma identidade diagnóstica
`"space"` (mapeamento pertencente a `entities/content/identity.md`), Debug
`"space"`, repr `"[ ]"`, plain_text `" "`, vacuidade e PartialEq cruzado;
hashes iguais são obrigatórios. morph_canon e == da linguagem consideram-nas
iguais também dentro de sequências; Parbreak e Linebreak continuam distintos.
Travessia, clone e transporte preservam a origem até a normalização causal,
sem assá-la em Text, Styled ou estilo de render. O hash baseado em Debug
deve continuar coerente com a igualdade cruzada.
Como o resultado da realização ainda depende dessa origem, nenhuma cache
com chave Content::PartialEq/Hash pode colapsar as duas formas antes da
normalização. A chave causal privada deve incluir a origem, ou a realização
deve preceder a memoização que usa a igualdade pública.

A nova variante é contrato Rust público mesmo sem diferença observável na
linguagem. O dono confirmou especificamente este contrato pelo ADR-0127
em 2026-09-24; a materialização L1–L4 está autorizada após este gate.

## P1374 — compatibilidade do construtor `Content::block`

### Medição anterior à decisão

O construtor público vigente recebe width e height como `Option<Length>` e é
usado por callers Rust que não precisam expressar percentagem ou fração. A
mudança proposta dos campos de `BlockElem` não exige quebrar esses callers.

### Decisão proposta

Preservar a assinatura pública atual. Width absoluto é elevado para
`Rel { rel: 0, abs }`; height absoluto para
`BlockHeight::Relative(Rel { rel: 0, abs })`; ausência permanece auto. O
construtor não ganha argumento fracionário nesta fatia e não aceita sentinel.
Esta obrigação participa do gate público P1374 antes de código.

## P1293 — constructors do carrier triestatal de `MathAttach`

### Medição anterior à decisão

No baseline confirmado, `01_core/src/entities/content.rs:1589-1618` expõe
`math_attach_scripts` e `math_attach` com seis `Option<Content>` e constrói
diretamente `MathAttachElem`. O inventário read-only encontrou chamadas
produtivas nos owners de eval estrutural/sintático e duas reconstruções em
`compiler/math/layout/mod.rs:2029-2037,2242-2250`; testes e fixtures possuem
chamadas adicionais, mas não são novos owners produtivos. O registro causal
P1293 SHA-256 `80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`
prova que a assinatura binária `Option` não pode transportar os três estados.

### Decisão pública confirmada

`Content::math_attach` recebe `MathAttachSlot` em `t,b,tl,bl,tr,br` e preserva
cada carrier literalmente. `math_attach_scripts` também recebe carriers para
seus quatro slots e fixa somente `t`/`b` em `Omitted`. Toda reconstrução deve
mapear recursivamente apenas `Present(content)`, mantendo `Omitted` e
`ExplicitNone` sem normalização. A variante `Content::MathAttach` e a base não
mudam; nenhuma variante nova de `Content` é criada.

Esta assinatura pública foi confirmada em `2026-09-02T08:06:37-03:00`
(`Confirmado`) sob ADR-0127 categoria 1. Não autoriza generic `Settable`,
parser/AST/Args/Value globais, sentinel, default ou mudança de fase. Aceitação:
constructors e reconstruções conservam `Omitted`, `ExplicitNone` e
`Present(Content::Empty)` até os owners de repr/layout; qualquer colapso é
refutador e bloqueia.

## P1286 — payloads públicos propostos (GATE ADR-0127)

### Smartquote: payload público proposto

Conforme o owner `entities/elements/smartquote.md`, `SmartQuoteElem` ganha
campos opcionais `alternative` e `quotes` para que a chamada direta permaneça
na variante `Content::SmartQuote`; usar `Content::Styled` vazaria `styled` por
`content.func()`. A variante do enum e a assinatura pública
`Content::smartquote(double)` permanecem; o construtor inicializa ambos os
campos novos como `None`. Os braços exaustivos continuam delegando igualdade,
hash, plain-text e `map_*` ao elemento. A mudança de campos públicos está
bloqueada pelo mesmo gate ADR-0127 e não autoriza código antes da confirmação.

### Carriers PDF propostos

#### Medição anterior à decisão

`pdf.attach` hoje termina em erro e `pdf.artifact` perde identidade ao devolver
o body. O vanilla pinado preserva `AttachElem` até coleta global e
`ArtifactElem` até tagging. `Content` não possui carrier equivalente; Formula
já demonstra que semântica PDF precisa sobreviver a eval/layout.

#### Contrato público proposto

Adicionar, após confirmação:

```rust
Content::PdfAttach(Arc<PdfAttachElem>)
Content::PdfArtifact(Arc<PdfArtifactElem>)
```

`PdfAttachElem` é leaf invisível e transporta `path`, bytes, relationship,
MIME e description. `plain_text` é vazio; `is_empty` permanece falso;
`map_*` é terminal. `PdfArtifactElem` transporta `kind: ArtifactKind` e
`body: Content`; `plain_text`, `is_empty` e `map_*` descem no body preservando
o kind. Igualdade/hash incluem todos os campos.

`ArtifactKind` é enum público fechado com
`Header|Footer|Watermark|PageNumber|LineNumber|Redaction|Bates|Page|PaginationOther|Layout|Background|Other`;
default `Other`. `AttachedFileRelationship` é enum público fechado
`Source|Data|Alternative|Supplement`.

Campos fechados propostos: `PdfAttachElem { path: EcoString,
data: Arc<Vec<u8>>, relationship: Option<AttachedFileRelationship>,
mime_type: Option<EcoString>, description: Option<EcoString> }` e
`PdfArtifactElem { kind: ArtifactKind, body: Content }`. `path` é o nome
virtual resolvido usado como identidade de deduplicação e Filespec; bytes não
são relidos em L3.

São duas variantes e tipos públicos: paragem ADR-0127 obrigatória. Os
módulos futuros `entities/elements/pdf_attach.rs` e `pdf_artifact.rs`
receberão prompts proprietários 1:1 somente quando seus consumers forem
autorizados; não criar prompt órfão antes disso.

## P1166 — variante de nó HTML explícito

**Medição:** `Content` não preservava tag, atributos ordenados e body do
`html.elem` medido no vanilla ratificado. Adicionar
`Content::HtmlElem(Arc<HtmlElem>)`, com braços exaustivos em repr, nome,
plain-text, walkers e transformações. O body é contentor para `map_content` e
`map_text`; em layout paginado não produz frame. O exporter HTML é o consumer
semântico. O payload e os campos observáveis vivem em `entities/html.md`.

## P1140.20.2 — deltas de canvas

SetPage/PageRunElem ganham `bleed: Option<PageBleedSpec>`,
`fill: Option<PageFill>`, `background: Option<Option<Content>>` e foreground
simétrico. Option externo distingue omitido; interno distingue none. map_* só
recursa em layers presentes e body. Incompleto até .3/.4; sem page público.

## P1140.20.1 — deltas de geometria

Adicionar a `SetPage` e `PageRunElem` `paper: Option<Paper>`,
`flipped: Option<bool>` e `binding: Option<PageBinding>`. Ausência preserva;
Auto e false explícitos não são ausência. `margin` preserva modo lateral.
Paper fornece apenas eixos sem width/height no mesmo delta e não vira string
de dispatch nem campo de Page.

Payload incompleto: canvas P1140.20.2, running matter .3, supplement .4;
`page` público somente em P1140.21.

## P1140.19 — `Content::PageRun`

Adicionar `Content::PageRun(Arc<PageRunElem>)` como contentor interno de um
page-run lexical. A decisão vem depois da medição diferencial: runs aninhados
restauram dimensões em LIFO (`180×180 → 100×120 → 180×180 → 240×240`), body
vazio conserva página e runs consecutivos não criam página intermediária.

`PageRunElem` é dono de `width`, `height`, `margin`, `numbering`, `columns` e
`body`, conforme L0 `entities/elements/page_run.md`. Ele é sempre não vazio,
delega `plain_text` ao body e recursa pelo body em map_*; não é locatável nem
selecionável por show rule. O enum fechado, matches exaustivos e despacho
estático permanecem. A variante não legitima o binding global `page`, adiado
explicitamente para P1140.21.

Não substituir por pares start/end nem por modo push/pop de `SetPage`: essas
formas permitem sequências desequilibradas e misturam page-run lexical com
set-rule progressiva.

## P1140.26 — morfologia pública de `PageRun`

Medição no vanilla ratificado: `repr(page([x]))` é
`sequence(pagebreak(weak: true), flush(), [x], pagebreak(weak: true))`; com
qualquer propriedade explícita, a sequência aparece como
`styled(child: sequence(..), ..)`. Portanto `PageRun` continua sendo a
representação interna fechada, mas seu `repr` público não pode colapsar no body.

## P1140.15 — `SetPage` preserva a especificação de margem

O eval já calcula `margin-left/right/top/bottom` separadamente para a
StyleChain, mas `Content::SetPage` transporta apenas `margin: Option<f64>` e
colapsa o dicionário para `left.or(top)`.

Alterar o campo público para `margin: Option<PageMarginSpec>`. `None` externo
significa não alterar margens; `Some(spec)` significa propriedade explícita;
cada lado `None` dentro da spec significa `auto`. Mapeamento, igualdade, Debug
e hashing preservam os quatro lados.

> **P622**: adicionada variante `Parbreak` — ver secção `Parbreak`.

## Módulo
`01_core/src/entities/content.rs`

## Propósito
`Content` representa a estrutura declarativa do documento Typst produzida
por `eval()`. É puramente declarativa — não desenha, não mede, não renderiza.
Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.

## Divergência do original (Opção D)
O `Content` original (`typst-library/foundations/content/`) usa:
- `pub struct Content(raw::RawContent)` com vtable `unsafe trait NativeElement`
- Proc macros `#[elem]` que geram implementações de `NativeElement`
- Arc manual (fat pointer com ref counting customizado, não `std::sync::Arc`)
- Styles como camada separada via `StyledElem` wrapper

Replicar esta metaprogramação em L1 traria toda a complexidade de
`typst_macros` sem benefício arquitectural. O cristalino diverge
intencionalmente: usa um enum linear com variantes declarativas.

Decisão registada em ADR-0026 (a criar).

> **`file:line` da caracterização do original (P1031)** — as quatro afirmações acima sobre a
> estrutura interna do `Content` do vanilla estavam sem `file:line`. Confirmam-se todas no
> vanilla ratificado (`e0e8ca4d`):
>
> | Afirmação | `file:line` |
> |---|---|
> | `pub struct Content(raw::RawContent)` | `crates/typst-library/src/foundations/content/mod.rs:84` — literal |
> | vtable via `unsafe trait NativeElement` | `crates/typst-library/src/foundations/content/element.rs:234` — `pub unsafe trait NativeElement: …` |
> | `Arc` manual com ref counting próprio | `crates/typst-library/src/foundations/content/raw.rs:17-19` e `:63-66`, comentários literais: *"`Arc<Inner<dyn Trait>>`, but in a manual way, allowing us to have a custom [fat pointer]"* e *"The element's reference count. This works just like for `Arc`. […] we have a custom fat pointer and `Arc` wouldn't know how to drop its contents."* |
> | proc macro `#[elem]` a gerar `NativeElement` | `crates/typst-library/src/foundations/content/element.rs` + `crates/typst-macros/` (o atributo `#[elem]` aparece em todos os elementos citados neste repositório de prompts) |
>
> **Natureza**: `file:line` do vanilla, não documentação — e é o registo adequado, porque
> estas afirmações são explicitamente sobre **mecânica**, não sobre a linguagem. É esse o
> ponto da secção: a divergência é deliberada e a ADR-0107 autoriza-a. Não há aqui paridade
> a provar; o que faltava era poder verificar a caracterização do original, e isso está
> agora resolvido.

> **Convenção de citação neste documento (fixada em P1031).**
>
> O catálogo de Bloco 3 registou um segundo achado sobre este L0: *"múltiplas secções (ex.:
> P156C-J, P247-P250, P295-P298): dezenas de afirmações de paridade vanilla para variantes
> de `Content` citam caminhos de ficheiro mas não `file:line`."* Inspecção do documento
> (2026-08-13) qualifica o achado:
>
> - **21 citações já têm linha** (`content.rs:NNN`) e apontam para
>   `01_core/src/entities/content.rs` — isto é, para o **próprio cristalino**, como registo
>   de onde estava o braço absorvido, não como prova de paridade.
> - As citações **sem** linha (`rules/introspect/locatable.rs`, `compiler/layout/mod.rs`,
>   `stdlib/layout.rs`, `export.rs`, …) são igualmente **ficheiros do cristalino**, não do
>   vanilla. Referem consumidores, não fontes de verdade.
> - As menções a `vanilla` no corpo do documento são maioritariamente **declarações de
>   divergência** (ADR-0054 graded, ADR-0107) — "vanilla permite desligar globalmente;
>   cristalino diverge", "sem atributos vanilla `tight`/`separator`/…" — e essas não
>   precisam de prova de paridade, precisam de estar declaradas, e estão.
>
> **Regra que passa a valer aqui**, para não deixar o achado em aberto sem o inflacionar:
>
> 1. Referência a ficheiro **do cristalino** dispensa `file:line` quando serve de índice
>    (onde vive o consumidor); mantém-se como está.
> 2. Qualquer afirmação sobre **o que o Typst faz** exige uma de duas provas: doc comment do
>    vanilla com `file:line` (é o texto de `typst.app/docs`), ou medição contra o binário
>    ratificado com proveniência. Sem uma delas, a frase tem de ser marcada como divergência
>    declarada ou como inferência.
> 3. As afirmações de paridade que restarem neste documento são revistas **por variante**,
>    no L0 dedicado da variante (`entities/elements/*.md`), não em bloco aqui — é onde a
>    revisão tem contexto para ser verificável. Vários já foram fechados em P1029 (família
>    math) e P1031 (figure, footnote, cite, ref, metadata, columns, equation, enum_item).
>
> Achado fechado como **convenção fixada + trabalho distribuído pelos L0 das variantes**,
> não como varredura completa deste ficheiro.

## Representação
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),         // TextElem mínimo
    Space,                   // SpaceElem / espaço entre palavras
    Parbreak,                // quebra de parágrafo (linha em branco no markup)
    Sequence(Vec<Content>),  // sequência de elementos
    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, Styles),             // requer Styles real
    // Heading { level: u8, body: Box<Content> },
    // Strong(Box<Content>),
    // Emph(Box<Content>),
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Elem(Arc<dyn NativeElement>),              // vtable — Passo 20+
}
```

## Modelo D — delegação por módulo (ADR-0105, lote piloto P316)

ADR-0105 adota o modelo **D**: cada variante migra **incrementalmente** de
campos inline (`Nome { … }`) para `Nome(Arc<nome::Nome>)`, com a lógica
por-variante a morar num módulo `entities/elements/nome.rs` que implementa o
trait `Element` (ver `entities/elements/_comum.md`). Os braços dessas variantes
nos **6 matches do hub** (`plain_text`, `is_empty`, `map_content`, `map_text`,
`get_field`, e o despacho de `eq`) deixam de conter lógica e viram **dispatch de
1 linha**: `Content::Nome(e) => e.metodo(…)`.

**Lote piloto P316** (3 variantes): `Divider`, `Heading`, `MathStyled` —
ver `entities/elements/{divider,heading,math_styled}.md`.

- `Divider` → `Divider(Arc<DividerElem>)`
- `Heading { level, body }` → `Heading(Arc<HeadingElem>)` (locatável: o trait
  fornece `element_kind`/`to_payload`, absorvendo o braço Heading de
  `ElementKind`/`extract_payload`)
- `MathStyled { … }` → `MathStyled(Arc<MathStyledElem>)`

**Lote 2 P317** (família math element-shaped — **11 variantes**; ordem de
migração por **largura de uso crescente**): cada migra para
`Math*(Arc<Math*Elem>)`, com módulo `entities/elements/math_<nome>.rs` e L0
próprio. **Nenhuma é locatável** (confirmado P317: zero refs em `ElementKind`/
introspecção → `element_kind`/`to_payload` ficam no default `None`). Os braços
math em `map_text` permanecem **terminais** (math structural — não descem,
paridade `MathStyled`/`Divider`: a variante migrada fica no bloco terminal `|`
como `Content::Math*(_)` e o hub clona em bloco; o `Elem::map_text` existe pelo
contrato do trait); em `map_content` os contentores recursam.

| variante | módulo / L0 | forma | shape |
|---|---|---|---|
| `MathCases { rows }` | `math_cases` | `MathCases(Arc<MathCasesElem>)` | grelha (recurse) |
| `MathMatrix { rows, delim }` | `math_matrix` | `MathMatrix(Arc<MathMatrixElem>)` | grelha (recurse) |
| `MathAlignPoint` | `math_align_point` | `MathAlignPoint(Arc<…Elem>)` | marcador unit |
| `MathAccent { base, accent }` | `math_accent` | `MathAccent(Arc<…Elem>)` | contentor (recurse) |
| `MathCancel { body }` | `math_cancel` | `MathCancel(Arc<…Elem>)` | contentor (recurse) |
| `MathDelimited { open, body, close }` | `math_delimited` | `MathDelimited(Arc<…Elem>)` | contentor (recurse body) |
| `MathRoot { index, radicand }` | `math_root(index, radicand, index_explicit)` | `MathRoot(Arc<…Elem>)` | contentor (recurse; preserva presença P1426) |
| `MathUnderover { base, under, over }` | `math_underover` | `MathUnderover(Arc<…Elem>)` | contentor (recurse) |
| `MathFrac { num, den }` | `math_frac` | `MathFrac(Arc<…Elem>)` | contentor (recurse) |
| `MathAttach { base, tl, bl, sub, sup }` | `math_attach` | `MathAttach(Arc<…Elem>)` | contentor (recurse) |
| `MathOp { text, limits }` | `math_op` | `MathOp(Arc<…Elem>)` | contentor (recurse text) |

Os campos `Box<Content>` desboxam para `Content` no `…Elem` (paridade
`MathStyledElem`, P316); `Option<Box<Content>>` → `Option<Content>`;
`Vec<Vec<Content>>` mantém-se. Cada `…Elem` ganha construtor ergonómico
`Content::math_<nome>(…)`. `MathLayouter` (`rules/math/layout`) passa a
destructurar `Arc<Math*Elem>` — **mesma lógica**, não editado por estes L0.

**Primitivos do hub — desenho declarado (triagem DEBT-58, P329).** Estas
variantes **permanecem no hub por desenho**, não por dívida — o arm próprio é
**intencional**. Critério do dono: fidelidade ao vanilla é de *comportamento*,
não de *estrutura Rust*; migrar não compra atomicidade quando não há semântica
de utilizador nem campos por crescer.

- **Definitivos (5)** — `Sequence`, `MathSequence`, `Empty`, `Space`, `Parbreak`:
  álgebra/cola do próprio `Content` (`sequence()` constrói `Sequence`/`Empty`;
  `Space`/`Parbreak` são cola de whitespace; `MathSequence` é o contentor math
  interno). Sem campos de utilizador. **Não migram nunca.**
- **Provisórios (3)** — `Text`, `MathText`, `MathIdent`: permanecem no hub **com
  revisita marcada no diagnóstico do F** — os campos que o vanilla lhes daria
  são estilo (StyleChain), território do F; decidir agora desenharia o F por
  acidente.

Os L0 redigidos no P317 para `MathSequence`/`MathText`/`MathIdent` (em
`debt-anexos/primitivos-ast/`) ficam como **referência histórica** — não serão
materializados (a decisão é manter inline). Ver **DEBT-58** (triado) e a L0 do
trait `entities/elements/_comum.md`.

> **Fora deste conjunto** (não são primitivos): `Block`/`Boxed`/`Labelled` são
> element-shaped densos → **lotes tardios** (L14 `Labelled`+`Boxed`, L15
> `Block`); `Styled` carrega `Styles` → **diagnóstico do F** (com as `Set*`).

**Lote 3 P318** (família lista/termos — **5 variantes** element-shaped; ordem de
migração por largura de uso crescente): cada migra para `Nome(Arc<NomeElem>)`,
módulo `entities/elements/<nome>.rs` + L0 próprio. **Nenhuma é locatável**
(confirmado P318: na lista exaustiva não-locatável; não são `ElementKind`).
Diferença vs Lote 2 (math): estas são **contentores de prosa** → `map_text`
**recurse** no(s) corpo(s) (precedente Heading, não terminal).

| variante | módulo / L0 | forma | recursão |
|---|---|---|---|
| `EnumItem { number, body }` | `enum_item` | `EnumItem(Arc<EnumItemElem>)` | body (preserva `number`) |
| `Link { url, body }` | `link` | `Link(Arc<LinkElem>)` | body (preserva `url`) |
| `ListItem(Box<Content>)` | `list_item` | `ListItem(Arc<ListItemElem>)` | body |
| `TermItem { term, description }` | `term_item` | `TermItem(Arc<TermItemElem>)` | term + description |
| `Terms { items }` | `terms` | `Terms(Arc<TermsElem>)` | items |

`Box<Content>` desboxa para `Content` no `…Elem` (convenção P316). `TermsElem` e
`TermItemElem` **sobrepõem `is_empty`** (`items.is_empty()` / ambos vazios); as
outras 3 ficam no default `false`. Construtores ergonómicos preservados
(`list_item`/`enum_item`/`link`) + novos (`terms`/`term_item`).

**Excluídas as `Set*`** (`SetFigureNumbering` 5, `SetEquationNumbering` 16,
`SetPage` 8, `SetHeadingNumbering` 62): são marcadores de set-rule — a superfície
da StyleChain. O destino delas depende da decisão F (medida na Parte 2 do P318);
migrá-las agora desenharia o F por acidente.

**Lote 4 P319** (decorações de texto — **3 variantes** element-shaped; ordem por
largura crescente): `Overline`(10) · `Strike`(10) · `Underline`(31). Cada migra
para `Nome(Arc<NomeElem>)`, módulo `entities/elements/<nome>.rs` + L0. **Nenhuma
é locatável** (confirmado P319). São **contentores de prosa** (corpo + cosméticos
`stroke`/`offset`/`extent`) → `map_text`/`map_content` **recursam** no body
(precedente Lote 3/Heading); **`is_empty` delega ao body** (override). Forma
idêntica para as 3:

```rust
Nome { body, stroke: Option<Color>, offset: Option<Length>, extent: Option<Length> }
  → Nome(Arc<NomeElem>)   // body desboxado para Content
```

**Novidade `Hash` manual**: `Length` não implementa `Hash`; como o trait
`Element` o exige, os 3 `…Elem` implementam `Hash` **à mão via `Debug`**
(`format!("{self:?}").hash(state)`, paridade `content_hash::hash_content`) em vez
de `#[derive(Hash)]`. Construtores ergonómicos novos
`Content::{overline,strike,underline}(body, stroke, offset, extent)`.

**Lote 5 P320** (união dos propostos 5+6 — **9 variantes** element-shaped; ordem
por largura crescente): `GridFooter`(7) · `GridHeader`(7) · `TableFooter`(10) ·
`TableHeader`(11) · `Linebreak`(11) · `Colbreak`(12) · `VSpace`(14) ·
`HSpace`(18) · `Pagebreak`(22) = ~112 sites. **Nenhuma é locatável** (confirmado
P320). Três formas:

- **Contentores (body + `repeat`)** — `GridHeader`/`GridFooter`/`TableHeader`/
  `TableFooter`: `map_*` **recursam** no body, preservam `repeat`; **`is_empty`
  delega ao body** (override). `Box<Content>` → `Content`.
- **Espaços (`amount: Length`, `weak`)** — `HSpace`/`VSpace`: `plain_text` vazio;
  **`is_empty` = `amount.is_zero()`**; map_* **terminais**; **`Hash` manual via
  Debug** (`Length` tem `f64`; precedente Lote 4).
- **Comandos unit/leaf** — `Linebreak` (unit), `Colbreak { weak }`,
  `Pagebreak { weak, to }`: `plain_text` `"\n"`/vazio; map_* **terminais**;
  `Linebreak` is_empty default, `Colbreak`/`Pagebreak` nunca vazios (false).
  **`Pagebreak` usa `Hash` manual via Debug** (`Parity` também não implementa
  `Hash` — mesma situação do `Length`, por outra razão).

Construtores preservados (`h_space`/`v_space`/`pagebreak`/`colbreak`/
`table_header`/`table_footer`) + novos (`linebreak`/`grid_header`/`grid_footer`).
**Fora deste lote**: `Space` (triagem DEBT-58) e o bloco
`TableCell`/`Table`/`GridCell`/`Grid` (lote próprio, ~193 sites).

**Lote 6 P321** (família state/counter + `Metadata` — **7 variantes**; ordem por
largura crescente): `CounterDisplay`(15) · `Metadata`(16) ·
`CounterDisplayCallback`(19) · `State`(21) · `StateDisplay`(21) ·
`StateUpdate`(25) · `CounterUpdate`(39) = ~172 sites. **Primeiro lote locatável
desde o piloto Heading.** Todas são **leaves/markers**: `plain_text` vazio;
`is_empty` default `false`; `map_content`/`map_text` **terminais**; `get_field`
default.

**Fronteira de locatabilidade (declarada):**

- **6 locatáveis** (queryable) — absorvem o braço de `extract_payload` no trait
  (precedente Heading): `element_kind` → `Some(ElementKind::X)` e `to_payload` →
  `Some(ElementPayload::X{…})`. São `Metadata`, `State`, `StateUpdate`,
  `StateDisplay`, `CounterUpdate`, `CounterDisplayCallback`
  (kind → `CounterDisplay`, partilhado). O hub passa a despachar
  `extract_payload.rs` (`Content::X(e) => e.to_payload()`) e `locatable.rs`
  (`Content::X(_) => true`). O **consumo por `ElementPayload`** (`from_tags`/
  walk de payload) é **inalterado** — matcheia o payload, não o `Content`.
- **1 não-locatável** — `CounterDisplay { kind }` legacy (single-pass, DEBT-10):
  `element_kind`/`to_payload` ficam no default `None`.

**`Hash`/`eq` (notas content-preserving):**

- **`Hash` manual via Debug** para as que carregam tipos sem `Hash` (`Value`/
  `f64`, `Func`, `state_update::StateUpdate`): `Metadata`, `State`, `StateUpdate`,
  `StateDisplay`, `CounterDisplayCallback`. `CounterDisplay` (só `String`) e
  `CounterUpdate` (action que deriva `Hash`) derivam `Hash`. Regra do modelo.
- **Quirk de `eq` preservado**: `Metadata`, `State`, `StateUpdate` **não têm arm
  de `eq` no hub** (caem em `_ => false`, sempre desiguais — marcadores
  efectivos). **Não adicionar dispatch de `eq`** para estas 3 (content-
  preserving). As outras 4 (`CounterDisplay`/`CounterUpdate`/`StateDisplay`/
  `CounterDisplayCallback`) têm arm → despacham `(X(a), X(b)) => a == b`.

`Box<Value>` desboxa para… **não**: mantém-se `Box<Value>` no `…Elem` (paridade
`ElementPayload`, que usa `Box<Value>`). Construtores ergonómicos novos
`Content::{counter_display,metadata,counter_display_callback,state,state_display,`
`state_update,counter_update}(…)`.

**Lote 7 P322** (por largura — **5 variantes** element-shaped): `Raw`(9) ·
`Align`(11) · `Image`(17) · `Hide`(18) · `Repeat`(18) = ~73 sites. **Nenhuma é
locatável** (confirmado P322). Duas formas:

- **Folhas** — `Raw { text, lang, block }` (`plain_text` = `text`) e
  `Image { path, data, width, height }` (`plain_text` vazio): `map_*` terminais;
  `is_empty` default `false`.
- **Contentores (body)** — `Align { alignment, body }`, `Hide { body }`,
  `Repeat { body, gap, justify }`: `map_*` **recursam** no body. `is_empty`:
  `Hide`/`Repeat` delegam ao body; **`Align` fica no default `false`**
  (content-preserving — o braço atual não delega). `Box<Content>` desboxa.

**`Hash`** (regra do modelo): **derivam** `Raw` (`EcoString`/bool) e `Hide`
(`Content`); **manual via Debug** `Align` (`Align2D` sem `Hash`), `Image`
(`Value`/`PtrEqArc`), `Repeat` (`Length`/`f64`). Construtores preservados
(`raw`/`hide`/`repeat`) + novos (`align`/`image`).

**Estado misto** (esperado, ADR-0105): durante os lotes o `enum` mistura
variantes migradas (`Nome(Arc<…>)`) e por migrar (`Nome { … }`); os 6 matches
despacham as migradas para o trait e mantêm o braço inline das restantes. O hub
encolhe lote a lote (baseline P313: `content.rs` 5782 linhas, 77 variantes).

**`eq`/`hash`**: `eq` por `#[derive(PartialEq)]` no `NomeElem` (estrutural via
`Arc`); `hash` continua por `content_hash::hash_content` (Debug) — os valores
absolutos das 3 variantes migradas mudam, a **relação** preserva-se (detalhe e
trava em `entities/elements/_comum.md` §A.1.1.b).

**Dois sistemas de igualdade (ADR-0025 + ADR-0107)**: o `#[derive(PartialEq)]`
acima é **estrutural** e serve testes/coleções/`IndexMap`/`hash` — **intacto**. O
`==` da **linguagem** (eval, `eval_binary_op`) sobre `Content` é **morfológico**
via **`morph_canon(&self) -> Content`**: produz uma forma canônica que remove o
estilo de **render** (o `TextStyle` assado de `Content::Text` → `default()`; o
`Content::Styled` **semanticamente vazio** — só transporte `custom` β1 — →
transparente, desce no body; `numbering_active`/`numbering` assados da chain →
neutros) e **preserva** a morfologia (texto, markup, estilo semântico
`*bold*`/`_italic_`). Compara-se duas formas canônicas com o `==` estrutural.
Implementado com `map_content` (transform total). `it.body == [a]` casa como
consequência (Achado 2, P342). Medido contra o vanilla (P345): `#set numbering`
**não** entra na igualdade (N1=true); estilo anexado ao conteúdo entra.

**Layout**: NÃO entra no trait (topologia — `entities` não depende de `rules`);
fica em `compiler/layout` / `rules/math/layout`, com o braço a destructurar
`Arc<NomeElem>` (mesma lógica). `compiler/layout.md` não é editado neste passo.

**Destino F** (ADR-0105): os módulos de elemento do D são o continente do
candidato F (PropMap); `NomeElem` agrupa campos+defaults para que `fn
descriptor()` futuro seja natural. F entra com o DEBT StyleChain (99.E).

## Interface pública obrigatória
```rust
impl Content {
    pub fn text(s: impl Into<EcoString>) -> Self;
    pub fn empty() -> Self;
    pub fn sequence(parts: Vec<Content>) -> Self;
    pub fn is_empty(&self) -> bool;
    pub fn plain_text(&self) -> String;
}
```

### `sequence()` — normalização
- 0 partes → `Empty`
- 1 parte → desembrulha (evita `Sequence([x])`)
- n > 1 partes → `Sequence(parts)`

### `plain_text()` — para verificação em testes
- `Empty` → `""`
- `Text(s)` → `s.to_string()`
- `Space` → `" "`
- `Parbreak` → `"\n"` (separação de parágrafos)
- `Sequence(v)` → concatenação recursiva

## Método `map_content` (Passo 69 — DEBT-19)

Percorre a árvore AST de baixo para cima (bottom-up), aplicando uma closure
a cada nó após processar os seus filhos.

```rust
pub fn map_content<F>(&self, transform: &mut F) -> SourceResult<Self>
where
    F: FnMut(&Content) -> SourceResult<Option<Content>>;
```

Semântica:
- `transform` retorna `Some(new_content)` → substituir; o novo nó NÃO é reavaliado.
- `transform` retorna `None` → manter o nó processado (com filhos já transformados).
- O `match` lista explicitamente containers (recursão) e terminais (clone). Sem `_ =>`.

Containers (recursão bottom-up): `Sequence`, `Strong`, `Emph`, `Heading`,
`ListItem`, `EnumItem`, `Link`, `Label`, `Figure`, `Equation`, `MathSequence`,
`MathFrac`, `MathAttach`, `MathRoot`, `MathDelimited`, `MathMatrix`, `MathCases`,
`Outline`.

Terminais (clone directo): `Text`, `Space`, `Parbreak`, `Empty`, `Linebreak`,
`Raw`, `Ref`, `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`,
`MathAlignPoint`, `MathIdent`, `MathText`.

## Variante `Content::Parbreak` — P622

Quebra de parágrafo semântica, produzida por uma linha em branco no markup
(`SyntaxKind::Parbreak`). Distinta de `Content::Space` (espaço inter-palavras) e
de `Content::Linebreak` (`\\` explícito).

- `plain_text()` → `"\n"`.
- `is_empty()` → `false` (marker estrutural; separa parágrafos).
- `map_content` / `map_text` → terminal (clone directo).
- `fmt_content`/`repr_content` → `"parbreak()"` (P1140.17; função e linha
  vazia têm a mesma morfologia canônica).
- Layout → `flush_line()` no ponto onde ocorre. Avança `cursor_y` por
  `line_height + leading` da linha que termina, separando visualmente os
  parágrafos. Múltiplos `Parbreak` consecutivos: o primeiro drena a linha
  actual; subsequentes encontram `current_line` vazia e são no-op em termos de
  avanço vertical (sem parágrafos vazios adicionais nesta versão).
- Introspecção → não-locatable; `extract_payload` → `None`; terminal em
  `materialize_time` e `walk`.

## Variante `Content::Label` — P460 + P464

```rust
Label(Arc<LabelElem>)
```

Wrapper transparente de identidade. Representa um destino nomeado para
referências cruzadas.

```rust
pub struct LabelElem {
    pub name: EcoString,
    pub body: Content,
    pub auto: bool,
}
```

- `name` — identificador do destino (e.g. `"sec1"`, `"fig1"`).
- `body` — conteúdo associado; renderizado normalmente.
- `auto` — origem do label:
  - `false`: construído explicitamente pela API Rust interna/fixtures;
  - `true`: gerado automaticamente via sintaxe `<label>` em
    headings/figures/equations.

**Comportamentos**:
- `plain_text` / `is_empty` — delegam ao `body`.
- `map_content` / `map_text` — recursam no `body`, preservando `name` e `auto`.
- Layout — transparente; regista página + posição em
  `extracted_label_pages` / `extracted_label_positions` para `/Dests` no PDF.
- Introspecção:
  - `auto: false`: propaga a label para o body; popula
    `label_to_counter_key` quando o body é numerado.
  - `auto: true`: emite `ElementPayload::Labelled` pós-recursão com
    `resolved_text` / `figure_number` (caminho legacy P329).

**Construtores**:
- `Content::label(name, body)` — `auto: false`.
- `Content::label_auto(name, body)` — `auto: true`.

**P1140.2 — medição e correção de fronteira:** o vanilla rejeita
`label("nome", body)` e o construtor `label("nome")` produz um valor de tipo
`label`, não `Content`. Assim, a forma de dois argumentos deixa de ser
superfície pública. Os dois construtores Rust acima permanecem para o wrapper
interno; a sintaxe markup continua usando `label_auto`.

## Variante `Content::Image` (Passo 71 — DEBT-24)

```rust
Image {
    path:   String,
    data:   std::sync::Arc<Vec<u8>>,
    width:  Option<Box<Value>>,   // Box quebra ciclo Content→Value→Content
    height: Option<Box<Value>>,
},
```

Terminal — sem filhos Content. `Arc<Vec<u8>>` partilhado: clones do AST não copiam bytes.
- `plain_text` → `""`
- `is_empty` → `false`
- `map_content` → terminal: `clone()`
- `map_text` → terminal: `clone()`
- Layouter: placeholder 100×100 pt (DEBT-24b).

## Método `get_field` (Passo 68)

Acesso a campos de elementos estruturados — usado pelas show rules.
Suporta `.body` e `.level` em `Heading`, `.body` em `Figure`.

## Critérios de verificação
- `Content::text("hello").plain_text() == "hello"`
- `Content::empty().is_empty() == true`
- `Content::sequence(vec![]).is_empty() == true`
- `Content::sequence(vec![Content::text("a")]) == Content::text("a")` (desembrulha)
- `Content::sequence(vec![Content::text("a"), Content::Space, Content::text("b")]).plain_text() == "a b"`
- `Content::Parbreak.plain_text() == "\n"` e `is_empty() == false`
- `Content::Empty`, `Content::Space` e `Content::Parbreak` — clone e PartialEq funcionam

## Variantes estruturais — Passo 154B (ADR-0060 Fase 1)

Materializadas em P154B como primeira sub-fase da Fase 1 do roadmap
ADR-0060. **Sem ADR nova** — apenas adições ao enum.

### `Content::Divider`

Singleton estrutural sem dados. Representa um separador horizontal.

- `plain_text()` → `""` (sem texto; representação visual é distinta).
- `is_empty()` → `false` (singleton estrutural conta como conteúdo).
- `map_content` / `map_text` → terminal (clone directo).
- Layouter: emite `FrameItem::Shape::Line` à largura do conteúdo,
  espessura 0.5pt, traço preto.

### `Content::Terms { items: Vec<Content> }`

Lista de pares termo-descrição. Tipicamente `items` é uma sequência
de `Content::TermItem`. A ordem é preservada.

- `plain_text()` → `items.iter().map(plain_text).join("\n")`.
- `is_empty()` → `items.is_empty()`.
- `map_content` / `map_text` → container; recurse em cada item.
- Layouter: itera items, layout sequencial.

### `Content::TermItem { term: Box<Content>, description: Box<Content> }`

Par individual term/description. Surge tipicamente dentro de `Terms`,
mas pode também aparecer standalone (e.g. show rules futuras).

- `plain_text()` → `format!("{}: {}", term.plain_text(), description.plain_text())`.
- `is_empty()` → `term.is_empty() && description.is_empty()`.
- `map_content` / `map_text` → container; recurse em term e description.
- Layouter: term em negrito + ": " + description, com indent 1.5em.

### Stdlib funcs (Passo 154B)

- `terms(named: descrição, ...)` em Typst-lang produz `Content::Terms`.
  Aceita só argumentos nomeados; descrição pode ser content ou string.
  Forma: `#terms(apple: [fruit], banana: [yellow])`.
- `divider()` produz `Content::Divider`. Sem argumentos.

### Limitações conscientes (P154B)

- Sem syntax markup nova (`/ term: desc` ou `---`) — trabalho de parser
  diferido a passo separado.
- Sem atributos vanilla `tight`/`separator`/`indent`/`hanging-indent`
  para `terms` — extensíveis sem breaking change (passar a
  `Terms { items, tight, ... }`).
- Sem show rules `#show terms: ...` neste passo.

## Variant `Content::Quote` — Passo 155 (ADR-0060 Fase 1, sub-passo 2)

Materializado em P155 como segunda sub-fase da Fase 1; **fecha
ADR-0060** (`PROPOSTO → IMPLEMENTADO`).

```rust
Content::Quote {
    body:        Box<Content>,
    attribution: Option<Box<Content>>,
    block:       bool,
    quotes:      bool,
}
```

**Atributos**:
- `body` — conteúdo citado.
- `attribution` — autor/fonte opcional.
- `block: true` → parágrafo dedicado, indent + spacing; `block: false`
  → inline no parágrafo circundante.
- `quotes: true` → aspas locale-apropriadas via
  `crate::compiler::lang::quotes::localize_quotes(lang)` em torno do body.

**Comportamento `plain_text`**:
- Sem smart-quotes: usa `"` ASCII fallback (texto plano não interage com lang).
- Com attribution: `"body" — attribution`.
- Sem attribution: `"body"`.
- Se `quotes: false`: aspas omitidas.

**Renderização (layouter)**:
- Smart-quotes via `text.lang` activo (per ADR-0057).
- `block: true`: indent 1.5em à esquerda; attribution em linha separada
  prefixada por "— ".
- `block: false`: inline; attribution prefixada por " — ".

**Construtores**:
- Stdlib: `#quote(body, attribution: ?, block: false, quotes: true)`.
- Markup: `"..."` em `Mode::Markup` produz aspas localizadas
  open/close por alternância (NÃO produz `Content::Quote`; produz
  `Content::Text(glyph)`). Cristalino usa o lexer vanilla
  (1 char = 1 SmartQuote token). `Content::Quote` é exclusivamente
  para construções estruturais via `#quote(...)`.

**Tabela de smart-quotes** (per `rules/lang/quotes.rs`):
| Lang | Open | Close |
|------|------|-------|
| `pt` | `«` | `»` |
| `en` | `"` (U+201C) | `"` (U+201D) |
| `de` | `„` | `"` (U+201C) |
| `fr` | `« ` (NBSP) | ` »` (NBSP) |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| (default) | `"` ASCII | `"` ASCII |

### Limitações conscientes (P155)

- Sem show rules `#show quote: ...` neste passo.
- Sem aspas secundárias (`'...'`) em markup — produz `'` ASCII.
- Sem smart-apostrophes (`don't` → `don't`).
- Aspas aninhadas em markup não suportadas (alternância simples
  open/close).
- Markup `"..."` produz `Content::Text` com glyph localizado; **não**
  produz `Content::Quote` (esse fica reservado para `#quote()`
  estrutural). Decisão pragmática: cristalino's lexer já é
  per-character, e refactor para parear `"..."` excederia escopo P155.

## Variant `Content::SmartQuote` — Passo 287 (`P-smartquote`)

Leaf variant (não container) que representa uma chamada programática a
`#smartquote(double: bool)`. Materializa a função stdlib vanilla
`text/smartquote.rs::SmartQuoteElem` em cristalino — paralelo
arquitectural ao markup `"foo"`/`'bar'` que `eval_markup` (P155)
pré-resolve em `Content::Text(glyph, style)` directo. Diagnóstico
completo em `00_nucleo/diagnosticos/diagnostico-smartquote-passo-287.md`.

```rust
Content::SmartQuote {
    double: bool,  // true = aspas duplas; false = simples
}
```

### Consumer Layouter

`Layouter::layout_content` arm `Content::SmartQuote { double }`:
1. Se `double = true`: consulta `localize_quotes(&self.style.lang)`
   (reuso `rules/lang/quotes.rs` — single source of truth partilhada
   com markup P155); alterna `self.smartquote_double_open`.
2. Se `double = false`: emite sempre ASCII `'` (smart-apostrophes
   scope-out per P155 §A.1.2); alterna `self.smartquote_single_open`.
3. Recurse via `Content::Text(glyph, style)` — reusa word-wrap +
   hyphenation pré-existentes; **`export.rs` não é tocado**
   (hash `66cb8ac3` preserved desde P285).

### Estado per-document no Layouter (P287)

Campos novos:
- `smartquote_double_open: bool` (default `true`)
- `smartquote_single_open: bool` (default `true`)

Inicializados em `Layouter::new` — reset por cada invocação `layout()`.

### Divergência aceite vs vanilla (ADR-0054 graded)

- **Estado independente do markup**: cristalino tem 2 estados separados
  (markup `eval_markup` local + função `Layouter`). Vanilla unifica via
  `SmartQuoter` único. Caso edge raro (mistura programática + markup
  literal) pode produzir "2 opens consecutivos" — registado em
  diagnóstico §A.3.2.
- **`text.smartquotes`** (atributo `set text`) **não existe** em
  cristalino — vanilla permite desligar globalmente; cristalino diverge.
- **`alternative` / `quotes`** (custom override) — rejeitados em
  `native_smartquote` com erro educacional mencionando ADR-0054; passo
  futuro condicional.

### Leaf — não qualifica como "variant rico"

Diagnóstico §A.2.2 (honestidade epistémica): `SmartQuote { double:
bool }` é leaf com 1 campo `bool` required — **não** qualifica como
"variant rico com `body` + cosméticos opcionais". Padrão N=4 cumulativo
(P156G/H/I+P284) **inalterado** pelo P287.

## Variants `Content::Underline` + `Content::Strike` + `Content::Overline` — Passo 284 (ADR-0054 graded)

Decoração textual paralela vanilla `UnderlineElem`/`StrikeElem`/`OverlineElem`
em `text/deco.rs`. Três variants **distintos** (não tagged) por coerência
arquitectural com P156G/H/I e com vanilla. Diagnóstico completo em
`00_nucleo/diagnosticos/diagnostico-deco-passo-284.md`.

```rust
Content::Underline {
    body:   Box<Content>,
    stroke: Option<Color>,
    offset: Option<Length>,
    extent: Option<Length>,
},
// Strike e Overline com assinatura idêntica.
```

### Atributos

- `body`: conteúdo a decorar (paridade `body: Content` vanilla, `#[required]`).
- `stroke`: paint da linha (apenas Color simples — objecto Stroke rico
  vanilla scope-out per Tabela A.7 linha 201 `stroke(...)` parcial). `None`
  ↔ default `Color::rgb(0, 0, 0)`.
- `offset`: override do offset Y default. `None` ↔ default por kind
  (`+0.10/-0.25/-0.80 em` — ver `stdlib.md` §`underline/strike/overline`).
- `extent`: extensão horizontal além do body (positiva ou negativa).
  `None` ↔ `0pt`.

### Scope-out (per diagnóstico §A.1 + ADR-0054 graded)

- `evade: bool` (descender skipping) — geometria glifo-a-glifo; passo
  dedicado futuro. **Vanilla `StrikeElem` não tem este atributo** (asimetria
  intencional). `native_underline` rejeita com erro explícito mencionando
  `ADR-0054`.
- `background: bool` (z-order) — baixo valor visível neste passo.

### Emit (consumer Layouter)

`Layouter::layout_content` para os três variants:

1. Captura `start_x = cursor_x`, `baseline_y = cursor_y` antes do body.
2. `layout_content(body)` — recurse normal.
3. Captura `end_x = cursor_x`.
4. Empurra `FrameItem::Line { start: (start_x − extent_pt, line_y),
   end: (end_x + extent_pt, line_y), thickness }` em `current_line`.

`line_y = baseline_y + (offset.resolve_pt() | kind_em * font_pt)`.
`thickness = max(font_pt * 0.05, 0.4)`.

### Restrição graded

Single-line apenas: se o body fluir para linha nova entre `start_x` e
`end_x`, a decoração assume largura `(start_x, end_x)` da linha final
(visivelmente incorrecta em multi-line). Sub-passo P284.1 candidato para
`flush_line`-aware emission. ADR-0054 graded justifica adiamento.

### Hash L0 `export.rs` preservado

A decisão A.2 (helper único derivado de `FrameItem::Line` existente) garante
que `export.rs` **não é tocado** — emit reusa a função de `Line` precedente
P38. Hash `bc7b8b95` preservado (verificável em `03_infra/src/export.rs:3`).

## Variants `Content::Pad` + `Content::Hide` — Passo 156C (ADR-0061 Fase 1, sub-passo 1)

Primeira aplicação concreta da ADR-0061 (Layout Fase X
roadmap). Materializam dois containers simples user-facing:
`pad()` (margens internas) e `hide()` (placeholder
layout-aware). Análogos a vanilla `PadElem`/`HideElem` em
`lab/typst-original/crates/typst-library/src/layout/{pad,hide}.rs`.

### `Content::Pad { body, padding }`

```rust
Pad {
    body:    Box<Content>,
    padding: Sides<Length>,
}
```

**Atributos** (declarados em stdlib `pad()`, resolvidos antes
de criar o variant):
- `left` / `right` / `top` / `bottom` — específico por lado.
- `x` (cobre `left` + `right`) — atalho horizontal.
- `y` (cobre `top` + `bottom`) — atalho vertical.
- `rest` — fallback para qualquer lado não declarado.

**Precedência** (resolvida em `native_pad`): específico > eixo
> rest. Lados não declarados em qualquer nível ficam a
`Length::ZERO`.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()`.
- `plain_text` — recurse no body (transparente).
- `map_content` / `map_text` — recurse no body; padding
  preservado como `Copy`.

**Renderização (layouter)**:
- `top` adicionado ao `cursor_y` antes do body.
- `left` adicionado ao `cursor_x` (e a `line_start_x` para
  que `flush_line` reinicie indentado).
- Body é layouted com cursor ajustado.
- Após body, `flush_line` força fim de linha pendente; `bottom`
  é adicionado a `cursor_y`; `cursor_x` e `line_start_x`
  restaurados.
- **`right` é scope-out neste passo** (perfil ADR-0054 graded):
  o Layouter actual não tem mecânica de "largura útil" por arm
  — width-aware wrap vive em `flush_line`/`layout_word` que
  consultam `page_config.width`. Aceitar como aproximação;
  refino quando refactor de Layouter para multi-region acontecer
  (ADR-0078 §sub-fase b; DEBT-56 columns/colbreak fechado P221).

**Validação em `native_pad`**:
- Padding negativo é rejeitado com erro hard (perfil ADR-0054
  graded). Vanilla aceita-o (margens "negativas" produzem overlap);
  cristalino diverge intencionalmente até que layout overflow
  semantic esteja clara.
- Named args desconhecidos rejeitados (paridade com `assert`,
  `align`, `place` etc.).
- Body posicional obrigatório (Content ou Str).

### `Content::Hide { body }`

```rust
Hide {
    body: Box<Content>,
}
```

**Atributos**: apenas `body`. Sem named args.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()`.
- `plain_text` — `String::new()` (não rende; texto plano vazio).
- `map_content` / `map_text` — recurse no body (transformações
  internas aplicam-se mesmo que o body não seja renderizado;
  permite que pipelines de pré-processamento funcionem).

**Renderização (layouter)**:
- Drena `current_items` e `current_line` para buffers
  temporários.
- Layouter executa o body normalmente (cursor avança).
- Items gerados pelo body são descartados (substituídos pelos
  buffers salvos).
- Resultado: zero `FrameItem`s emitidos, mas `cursor_x`/
  `cursor_y` preservam o avanço.

**Comportamento em introspect** (`materialize_time` + `walk`):
- Ambos descem no body. Hide preserva semantic de "presence":
  labels, contadores e refs dentro de `hide(...)` continuam a
  resolver. Apenas a renderização é suprimida.

### Construtores

- Stdlib: `#pad(body, left: ?, right: ?, top: ?, bottom: ?,
  x: ?, y: ?, rest: ?)` e `#hide(body)`.
- Construtores Rust: `Content::pad(body, padding)` e
  `Content::hide(body)`.

### Limitações conscientes (P156C)

- Sem show rules `#show pad: ...` ou `#show hide: ...` neste
  passo. Adiados a passo agregado futuro (análogo a P154B/P155
  para terms/divider/quote).
- `right` padding **scope-out** em layout — ver acima. Refino
  com refactor multi-region.
- Padding negativo **scope-out** (rejeitado com erro). Refino
  quando layout overflow semantic clara existir.
- `Content::Pad` e `Content::Hide` aninhados são suportados
  (cobertura recursiva em todos os arms); padding aninhado é
  cumulativo, hide aninhado é idempotente.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variants novos (não `Content::Styled`). Rationale: ambos têm
semantic estrutural (composição + cursor advance) que excede
styling visual. Coerente com vanilla `PadElem`/`HideElem`
serem `#[elem]` proper. Coerente com modelo ADR-0060 Fase 1
para terms/divider/quote.

## Variants `Content::HSpace` + `Content::VSpace` — Passo 156D (ADR-0061 Fase 1, sub-passo 2)

Segunda aplicação consecutiva de ADR-0061. Materializam
spacing primitives horizontal e vertical, análogos a vanilla
`HElem`/`VElem` em
`lab/typst-original/crates/typst-library/src/layout/spacing.rs`.

### `Content::HSpace { amount, weak }`

```rust
HSpace {
    amount: Length,
    weak:   bool,
}
```

**Atributos** (declarados em stdlib `h(amount, weak: false)`):
- `amount` — Length posicional obrigatório.
- `weak: bool` — armazenado mas comportamento de collapse
  adiado (perfil ADR-0054 graded). Refino futuro se priorizado.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — `amount.is_zero()` (consistente com Sequence
  vazia).
- `plain_text` — `String::new()` (não rende texto).
- `map_content` / `map_text` — terminal (clone directo); leaf
  sem body.

**Renderização (layouter)**:
- Resolve `amount` em pt via `Length::resolve_pt(font_size_pt)`.
- Avança `self.cursor_x` por esse valor.
- `weak` ignorado neste passo.

**Validação em `native_h`**:
- Aceita `Length`, `Float` (interpretado em pt), `Int` (idem).
- `amount` negativo rejeitado (perfil ADR-0054 graded; vanilla
  aceita-o).
- Named arg desconhecido rejeitado.
- `weak` deve ser `Bool` (tipo errado → erro hard).

### `Content::VSpace { amount, weak }`

```rust
VSpace {
    amount: Length,
    weak:   bool,
}
```

**Atributos**: idênticos a `HSpace`.

**Comportamento `is_empty` / `plain_text` / `map_*`**: idênticos
a `HSpace`.

**Renderização (layouter)**:
- Resolve `amount` em pt.
- Se `cursor_x > line_start_x`, força `flush_line` (termina
  linha em curso para evitar texto meio-render).
- Avança `self.cursor_y` pelo valor resolvido.

**Validação em `native_v`**: idêntica a `native_h` (lógica
partilhada via helper `build_spacing` em `stdlib/layout.rs`).

### Construtores

- Stdlib: `#h(amount, weak: false)` e `#v(amount, weak: false)`.
- Construtores Rust: `Content::h_space(amount, weak)` e
  `Content::v_space(amount, weak)` (naming `_space` evita
  conflito com identificadores curtos `h`/`v` em scope Rust).

### Limitações conscientes (P156D)

- `amount` aceita apenas `Length` neste passo. Vanilla aceita
  `Fraction` (ex: `h(1fr)`) — refino futuro per ADR-0061 §6.3.
- `weak` armazenado mas semantic de collapse não implementada.
  Vanilla colapsa weak adjacentes; cristalino mantém ambos
  (over-spacing aceitável per ADR-0054 graded). Se priorizado,
  abrir DEBT.
- `amount` negativo rejeitado com erro. Vanilla aceita-o
  (gera overlap). Refino quando layout overflow semantic
  clara existir.
- `h` no fim de linha não força wrap; cursor.x apenas avança
  (pode exceder largura da página). Refino com refactor
  multi-region (ADR-0078 §sub-fase b).
- `v` no início de página/coluna não colapsa contra margem
  (vanilla colapsa). Avanço simples de cursor.y.
- Sem show rules `#show h: ...` ou `#show v: ...` neste passo
  (consistente com adiamento P154B/P155/P156C).

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variants novos (não `Content::Styled`). Rationale: spacing
primitives são structurais (afectam cursor, não rendem texto),
não estilo visual. Coerente com vanilla `HElem`/`VElem` serem
`#[elem]` proper. Coerente com modelo dos sub-passos
anteriores (terms, divider, quote, pad, hide).

## Variant `Content::Pagebreak` — Passo 156E (ADR-0061 Fase 1, sub-passo 3)

Terceira aplicação consecutiva de ADR-0061. Materializa
quebra de página manual, análoga a vanilla `PagebreakElem`
em `lab/typst-original/crates/typst-library/src/layout/page.rs`.

### `Content::Pagebreak { weak, to }`

```rust
Pagebreak {
    weak: bool,
    to:   Option<Parity>,
}
```

**Atributos** (declarados em stdlib `pagebreak(weak: false,
to: ?)`):
- `weak: bool` — armazenado mas comportamento de collapse
  adiado (consistente com P156D HSpace/VSpace).
- `to: Option<Parity>` — `None` == Auto (sem ajuste);
  `Some(Even)`/`Some(Odd)` força próxima página à paridade.

**Tipo `Parity`** novo em `01_core/src/entities/parity.rs`
(Even/Odd com método `matches(page_number)`); ver
`prompts/entities/parity.md`.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — sempre **`false`** (event observável mesmo
  sem body; cf. Divider em P154B).
- `plain_text` — `String::new()` (event sem texto).
- `map_content` / `map_text` — terminal (clone directo);
  Pagebreak é leaf sem body.

**Renderização (layouter)**:
- Reusa `Layouter::new_page` (definido em `cursor.rs:128`)
  que commits `current_items` numa nova `Page`, push para
  `pages`, e reseta cursor.
- Sequência: `flush_line` (se houver linha em curso) →
  `new_page()` → se `to: Some(parity)`, verifica
  `pages.len() + 1` (próxima página) e insere segunda
  `new_page()` se paridade não bate.
- Página inserida usa `page_config` actual (mesmas dimensões;
  sem header/footer porque Page actual não os tem).
- `weak` ignorado neste passo.

**Validação em `native_pagebreak`**:
- Sem argumentos posicionais (rejeitado com erro hard).
- Named args válidos: `weak` (Bool), `to` (Str
  `"even"`/`"odd"`). Outros named args rejeitados.
- `weak` deve ser Bool; tipo errado → erro hard.
- `to` deve ser Str `"even"` ou `"odd"`; outro valor → erro
  hard (helper `extract_parity`). `to: None` (omitido)
  produz `Option::None`.

### Construtores

- Stdlib: `#pagebreak(weak: false, to: ?)`.
- Construtor Rust: `Content::pagebreak(weak, to)`.

### Limitações conscientes (P156E)

- `weak` collapse semantic não implementado (consistente
  P156D). Vanilla colapsa weak adjacentes; cristalino
  mantém ambos.
- Página vazia inserida para ajustar paridade não tem
  cabeçalho/rodapé (porque `Page` cristalino não os tem).
  Refino futuro com Page rico (Fase 3 ADR-0061).
- `to` aceita só string em stdlib (vanilla aceita
  `Symbol::even` sem aspas). Refino se priorizado.
- Pagebreak no início absoluto do documento cria página 1
  vazia + conteúdo na página 2; aceitável (case patológico
  raro).
- Sem show rules `#show pagebreak: ...` neste passo.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variant novo (não `Content::Styled`, não `Style::PageBreak`).
Rationale: pagebreak é "event" estrutural com semantic única
(força flush + verifica paridade) que excede styling. Coerente
com vanilla `PagebreakElem` ser `#[elem]` proper. Coerente
com modelo dos sub-passos anteriores.

### Tipo `Parity` (infraestrutura paralela)

`Parity { Even, Odd }` foi criado neste passo como
infraestrutura genérica reusável, análoga ao `Sides<T>`
criado em P156C. Vive em `01_core/src/entities/parity.rs`.
Reuso futuro previsível em refino Page rico (paridade per
header/footer).

Vanilla usa `Smart<Parity>` (Auto/Custom); cristalino
simplifica para `Option<Parity>` (`None` == Auto). Sem
perda funcional; ganho em clareza idiomática Rust.

## Skew via `TransformMatrix::skew` — Passo 156F (ADR-0061 Fase 1, sub-passo 4)

Quarta aplicação consecutiva de ADR-0061. Materializa
`skew(body, ax: ?, ay: ?)` análogo a vanilla `SkewElem`.

### Divergência face à spec do P156F

A spec do P156F propôs introduzir `enum TransformKind
{ Move, Rotate, Scale, Skew }` para "unificar" os 4
elementos vanilla num só variant `Content::Transform`.
**Inventário em 156F.1 revelou que essa unificação já
existia desde P78** — `Content::Transform { body, matrix:
TransformMatrix }` reusa a matriz cm (PDF) para todos os
4 tipos. `TransformKind` enum seria redundante.

**Decisão deste passo**: skew adicionado como método
estático novo `TransformMatrix::skew(ax_rad, ay_rad)` em
`entities/layout_types.rs`, análogo a `translate`,
`rotate`, `scale` já existentes. Zero refactor de
variant; zero mudança em consumers. **Risco de regressão
zero** (puramente aditivo).

### `TransformMatrix::skew(ax_rad, ay_rad)`

Forma da matriz cm:
```
| 1        tan(ax)   0 |
| tan(ay)  1         0 |
| 0        0         1 |
```

Aplicada a `(x, y)`:
- `x' = x + tan(ax) * y`
- `y' = tan(ay) * x + y`

### Stdlib `#skew(body, ax: ?, ay: ?)`

Implementado em `01_core/src/compiler/stdlib/transforms.rs`
ao lado de `native_move`/`native_rotate`/`native_scale`
(coesão por domínio per ADR-0037). Atributos:
- `ax: Angle` — distorção horizontal (default 0).
- `ay: Angle` — distorção vertical (default 0).
- `body` posicional obrigatório.
- Aceita também `Float` em radianos (consistente com
  `native_rotate`).

**Validação**:
- Ângulos com magnitude ≥ `π/2 - 1e-3` rad (~89.94°)
  rejeitados (tan diverge); erro hard.
- Named args desconhecidos rejeitados (consistente
  pad/h/v/pagebreak).
- `origin` (ponto de pivot) **scope-out** — análogo a
  rotate/scale actuais que também não têm origin
  (refino futuro per ADR-0061 §6.3).

### Limitações conscientes (P156F)

- `origin` não suportado (alinhado com move/rotate/scale).
- Ângulos extremos rejeitados em vez de saturar (decisão
  de erro explícito vs comportamento indefinido).
- `Smart<Angle>` da vanilla simplificado para `Option`
  implícito (default 0 quando ausente).

### Decisão arquitectural confirmada

Sem TransformKind enum (per inventário 156F.1).
Arquitectura matriz cm já era a unificação correcta. P156F
**adiciona método ao tipo existente** em vez de refactorar
struct — padrão "menor mudança suficiente".

## Variant `Content::Block` — Passo 156G (ADR-0061 Fase 2, sub-passo 1)

Quinta aplicação consecutiva de ADR-0061; **primeira de
Fase 2** (containers ricos). Materializa
`block(body, width: ?, height: ?, inset: ?, breakable: true)`
análogo a vanilla `BlockElem`.

### Decisão arquitectural escolhida (Opção A modificada)

Per inventário 156G.1 + análise comparativa 156G.2:
**variant rico `Content::Block { body, width, height, inset,
breakable }`** em vez de Style cascade.

**Rationale**:
- `Style` enum cobre só **propriedades de texto** (Bold,
  Italic, Size, Fill, HeadingLevel) — vocabulário não-encaixa
  com width/height/inset/breakable que são **propriedades de
  container**.
- Coerente com `Content::Pad` (P156C) que também tem fields
  explícitos para padding (também propriedade de container).
- Reusa pattern emergente: containers com fields explícitos
  para atributos não-Style.

### `Content::Block { body, width, height, inset, breakable }`

```rust
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
}
```

**Atributos** (subset Fase 1 per ADR-0054 graded; declarados
em stdlib `block(body, ...)`):
- `body` posicional opcional (Content ou Str; ausente →
  Empty).
- `width: Length` — largura explícita; default `None` (auto).
- `height: Length` — altura explícita; default `None` (auto).
- `inset: Length` — margem interna uniforme nos 4 lados
  (refino futuro: Sides completo via dict); default zero.
- `breakable: bool` — `true` permite quebra entre páginas;
  `false` é "atómico"; default `true`. **Semantic real
  adiada** — armazenado mas layouter não impede quebra ainda.

**Atributos scope-out** (refino futuro per ADR-0054 graded):
`outset`, `fill`, `stroke`, `radius`, `clip`, `spacing`,
`above`/`below`, `sticky`. **Rejeitados em `native_block`**
com erro hard até refino futuro.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()` (atributos não
  fazem container deixar de ser vazio).
- `plain_text` — recurse no body (transparente).
- `map_content` / `map_text` — recurse no body; atributos
  preservados como Copy.

**Renderização (layouter)**:
- `flush_line` se houver conteúdo pendente (block ocupa
  nova "linha lógica").
- Aplica `inset.top` (avança cursor.y).
- Aplica `inset.left` (offset de `line_start_x`/cursor.x).
- Layout do body com cursor ajustado.
- `flush_line` no fim.
- Aplica `inset.bottom` (avança cursor.y).
- Se `height: Some(h)`, garante avanço mínimo de h vertical
  (caso body + inset_top + inset_bottom seja menor).
- Restaura `line_start_x`/cursor.x.
- `inset.right` é scope-out (Layouter actual sem largura
  útil por arm; refino com refactor multi-region per
  ADR-0078 §sub-fase b).
- `width` armazenado mas não consumido em layout actual
  (largura limitada por flush_line/word_wrap globais).
  Per ADR-0054 graded; refino futuro.

**Validação em `native_block`**:
- Width/height/inset negativos rejeitados (consistente com
  pad em P156C).
- Named arg desconhecido rejeitado com mensagem explicativa
  (incluindo lista de scope-outs).
- `breakable` deve ser Bool.
- Inset aceita `Length` uniforme apenas (refino futuro para
  dict).

### Construtores

- Stdlib: `#block(body, width: ?, height: ?, inset: ?,
  breakable: ?)`.
- Construtor Rust: `Content::block(body, width, height,
  inset, breakable)`.

#### P1373 — compatibilidade do construtor Rust

Embora `BlockElem::height` passe a `Option<Rel<Length>>`, a assinatura pública
de `Content::block` permanece com `height: Option<Length>`. O construtor eleva
o valor para `Rel { rel: 0, abs: height }`. Assim, consumidores Rust absolutos
não precisam migrar; percentagens entram pela nativa Typst, que possui o
contexto de tipo correto antes do layout. O construtor não resolve nem lê
geometria de página.

### Limitações conscientes (P156G)

- **9 atributos vanilla scope-out** (outset, fill, stroke,
  radius, clip, spacing, above, below, sticky) — refino
  futuro per ADR-0054 graded.
- `inset` aceita Length uniforme apenas. Vanilla aceita
  dict ou número. Refino futuro.
- `width` armazenado mas não impõe limite real (Layouter
  actual sem mecânica de largura útil por arm).
- `breakable: false` armazenado mas semantic real defere
  (refactor multi-region exigido).
- `inset.right` scope-out em layout (mesma razão que
  `Pad.right` em P156C).
- Sem show rules `#show block: ...` neste passo.
- Block aninhado: suportado estruturalmente; insets
  cumulativos via cursor advance.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variant rico (não Styled). Coerente com:
- vanilla `BlockElem` ser `#[elem]` proper.
- `Content::Pad` (P156C) que usa fields explícitos.
- Princípio "container com atributos não-style usa variant".

**Padrão emergente Fase 2**: containers ricos preferem
variants explícitos quando atributos não são propriedades
de texto. Box (P156H) e Stack (P156I) provavelmente seguem
mesmo modelo.

## Variant `Content::Boxed` (box inline) — Passo 156H (ADR-0061 Fase 2, sub-passo 2)

Sexta aplicação consecutiva de ADR-0061; **segunda Fase 2**.
Materializa `box(body, width: ?, height: ?, inset: ?,
baseline: ?)` análogo a vanilla `BoxElem`.

### Decisão arquitectural reusada (Opção A modificada de P156G)

Padrão **variant rico** estabelecido em P156G aplicado
directamente — sem nova decisão arquitectural. P156G §15.5
recomendou explicitamente esta reaplicação.

### `Content::Boxed { body, width, height, inset, baseline }`

```rust
Boxed {
    body:     Box<Content>,
    width:    Option<Length>,
    height:   Option<Length>,
    inset:    Sides<Length>,
    baseline: Length,
}
```

**Naming**: variant Rust é `Boxed` (não `Box`) para evitar
ambiguidade com `std::boxed::Box`; stdlib expõe `#box(...)`
(paridade vanilla); construtor Rust: `Content::boxed(...)`.

**Atributos** (subset Fase 2 per ADR-0054 graded):
- `body` posicional opcional (Content ou Str; ausente → Empty).
- `width: Length` — explícita; default `None` (content-based).
- `height: Length` — explícita; default `None` (auto).
- `inset: Length` — uniforme nos 4 lados; default zero.
- `baseline: Length` — ajuste vertical; default zero;
  **negativo aceito** (move para cima).

**Atributos scope-out** (refino futuro per ADR-0054 graded):
`outset`, `fill`, `stroke`, `radius`, `clip`, `stroke-overhang`.
**Rejeitados em `native_box`** com erro hard.

### Distinção material face a `Block` (P156G)

| Aspecto | Block | Boxed (Box) |
|---------|-------|-------------|
| Posicionamento | structural (força flush_line) | **inline** (sem flush) |
| Largura default | full page width | content-based |
| Atributo único | `breakable: bool` | `baseline: Length` |
| Layouter | flush + inset_top + offset_left + body + flush + inset_bottom + height_min | append inline + inset_left + body + inset_right (top/bottom/baseline scope-out em layout actual) |

**Comportamento `is_empty` / `plain_text` / `map_*`**:
Análogo a `Block` (proxy body; recurse).

**Renderização (layouter)**:
- **NÃO força flush_line** (box é inline).
- Aplica `inset.left` como avanço de cursor.x.
- Layout body in-place na linha actual.
- Aplica `inset.right` como avanço de cursor.x final.
- `width`/`height`/`baseline` armazenados mas semantic real
  adiada per ADR-0054 graded:
  - `width`: limitar largura útil em contexto inline exigiria
    refactor multi-region (ADR-0078 §sub-fase b).
  - `height` em contexto inline alteraria line_height —
    refino futuro.
  - `baseline` exige offset vertical mid-linha — não
    suportado por cursor.rs actual.
- `inset.top`/`inset.bottom` em contexto inline são
  complexos; armazenados mas não aplicados (refino futuro).

**Validação em `native_box`**:
- Width/height/inset negativos rejeitados (consistente Block).
- **Baseline negativo aceito** (semantic legítima — move
  box para cima).
- Named arg desconhecido rejeitado.

### Construtores

- Stdlib: `#box(body, width: ?, height: ?, inset: ?,
  baseline: ?)`.
- Construtor Rust: `Content::boxed(body, width, height,
  inset, baseline)`.

### Limitações conscientes (P156H)

- 6 atributos vanilla scope-out (outset, fill, stroke,
  radius, clip, stroke-overhang). Refino futuro.
- `inset` aceita Length uniforme apenas (refino futuro
  para dict).
- `width`/`height` armazenados mas não impõem limite real
  (refino multi-region per ADR-0078 §sub-fase b).
- `baseline` armazenado mas semantic real adiada (cursor.rs
  actual sem mecânica de offset mid-linha).
- `inset.top`/`inset.bottom` armazenados mas não aplicados
  em layout inline (alterariam line_height).
- Sem show rules `#show box: ...` neste passo.

### Padrão emergente Fase 2 (reaplicação confirma)

P156G estabeleceu padrão "variant rico para containers"; P156H
reaplica directamente sem nova decisão arquitectural. **Padrão
consolidado**: P156I (stack) provavelmente segue mesmo modelo.

## Variant `Content::Stack` — Passo 156I (ADR-0061 Fase 2, sub-passo 3; **último Fase 2**)

**Sétima aplicação consecutiva** de ADR-0061; **último
sub-passo Fase 2**; **atinge target 72% Layout** declarado
em ADR-0061 §6.2.

### Decisão arquitectural reusada (Opção A modificada de P156G/H)

Padrão variant rico estabelecido em P156G+H aplicado
directamente. Adaptação para `Arc<[Content]>` (clone O(1)
per ADR-0026 revisão, consistente com `Sequence`/`MathSequence`).

### `Content::Stack { children, dir, spacing }`

```rust
Stack {
    children: Arc<[Content]>,
    dir:      Dir,
    spacing:  Option<Length>,
}
```

**Atributos**:
- `children: Arc<[Content]>` — variádicos posicionais (Content
  ou Str na stdlib).
- `dir: Dir` — direcção de empilhamento (LTR/RTL/TTB/BTT;
  default `TTB`). Tipo `Dir` novo em `entities/dir.rs`.
- `spacing: Option<Length>` — espaço entre children;
  `None` == zero (consistente com padrão Smart→Option
  N=5 aplicações).

**Distinção material face a Block/Boxed**:

| Aspecto | Block | Boxed | Stack |
|---------|-------|-------|-------|
| Body | único | único | **Vec (Arc<[Content]>)** |
| Tipo body | `Box<Content>` | `Box<Content>` | `Arc<[Content]>` |
| Posicionamento | structural | inline | **structural** |
| Atributos próprios | breakable | baseline | **dir + spacing** |

### Comportamento `is_empty` / `plain_text` / `map_*`

- `is_empty` — `children.iter().all(|c| c.is_empty())`
  (consistente com `Sequence`).
- `plain_text` — concatena plain_text de todos os children.
- `PartialEq::eq` — comparação 3-fields (Arc deep eq).
- `map_content` / `map_text` — mapear cada child; preservar
  dir/spacing.
- `materialize_time` (introspect) — recurse em cada child.
- `walk` (introspect) — walk em cada child em ordem.

### Renderização (layouter)

- **Structural**: força `flush_line` antes (se necessário).
- **TTB/BTT**: itera children; cada um em "linha" própria
  (flush_line após cada); spacing entre via `cursor_y +=
  Pt(spacing)` antes de cada child (excepto o primeiro).
- **LTR/RTL**: itera children inline; spacing via `cursor_x
  += Pt(spacing)` entre cada.
- **BTT/RTL**: implementadas como reverse iteration
  (`children.iter().rev()`) — geometricamente similar a
  TTB/LTR mas com order visualmente invertido. Refino futuro
  pode aplicar posicionamento absoluto reverso real per
  ADR-0054 graded.

### Validação em `native_stack`

- `dir` aceita só string `"ltr"`/`"rtl"`/`"ttb"`/`"btt"`
  (helper `extract_dir`). Outros valores ou tipos rejeitados.
- `spacing` aceita Length/Float/Int (em pt); negativo
  rejeitado.
- Named arg desconhecido rejeitado (sem scope-out adicional;
  vanilla stack tem apenas estes 3 atributos — lista
  pequena).
- Children variádicos: aceita Content ou Str; outros tipos
  rejeitados (estricto).

### Construtores

- Stdlib: `#stack(dir: ?, spacing: ?, ..children)`.
- Construtor Rust: `Content::stack(children: Vec<Content>,
  dir, spacing)` (Vec interno convertido para `Arc<[T]>`
  via `into()`).

### Limitações conscientes (P156I)

- `BTT`/`RTL` implementadas como reverse iteration em vez de
  posicionamento absoluto reverso real. Per ADR-0054 graded.
- Sem alignment per-child (vanilla `StackChild` tem
  alinhamento opcional). Refino futuro.
- Sem show rules `#show stack: ...` neste passo.
- Stack aninhado suportado estruturalmente; não testado E2E.

### Tipo `Dir` (infraestrutura paralela)

`Dir { LTR, RTL, TTB, BTT }` foi criado neste passo como
infraestrutura genérica reusável, análoga a `Sides<T>`
(P156C) e `Parity` (P156E). Vive em `01_core/src/entities/dir.rs`.
Reuso futuro previsível em refino bidi shaping ou
`Content::Columns { dir, ... }` quando Fase 3 for atacada.

### Padrão emergente "Smart<T> → Option<T> ou default" — N=5

P156I aplica novamente o padrão simplificador:
- P156E `Smart<Parity>` → `Option<Parity>`.
- P156F angles default 0 (em vez de Smart).
- P156G `Smart<Rel<Length>>` para width → `Option<Length>`.
- P156H idem Box.width.
- **P156I `Smart<Rel<Length>>` para spacing → `Option<Length>`;
  `Smart<Dir>` simplificado para `Dir` directo com Default
  natural (TTB).**

**N=5 aplicações** — patamar empírico forte. Candidato a
registo formal em ADR meta futuro.

---

## Variant `Content::Repeat` — Passo 156J (ADR-0061 Fase 3, sub-passo 1; **primeira Fase 3**)

P156J adiciona **um variant** ao enum `Content` cobrindo
repetição de body para preencher espaço, análoga a vanilla
`RepeatElem`. **Primeira aplicação Fase 3** (ADR-0061; activa
caminho 1 — materializar Fase 3).

**Decisão arquitectural reusada de P156G/H/I**: variant rico
(Opção A) — atributos `gap` e `justify` são propriedades
específicas de repetição; `Style` enum cobre só propriedades
de texto. Coerente com Block/Boxed/Stack.

### `Content::Repeat { body, gap, justify }`

```rust
Repeat {
    body:    Box<Content>,
    /// Espaço entre cópias; `None` == zero (padrão Smart→Option N=6).
    gap:     Option<Length>,
    /// `true` == distribuir espaço residual aumentando gap real.
    /// Default vanilla `true`. Distribuição real adiada per
    /// ADR-0054 graded.
    justify: bool,
}
```

**Atributos** (paridade vanilla):
- `body`: conteúdo a repetir (obrigatório).
- `gap: Option<Length>`: espaço entre cópias; `None` == zero.
- `justify: bool`: default vanilla `true` (paridade).

### Stdlib `repeat`

`#repeat[.]` ou `#repeat(body, gap: ?, justify: ?)`. Processado
por `native_repeat` em `stdlib/layout.rs`. Helper `extract_length`
reusado para `gap` (sexta aplicação consecutiva — N=6).

### Layout per ADR-0054 graded (paridade estrutural)

P156J implementa **paridade estrutural** (variant + stdlib +
medição estática + layout single-render). Algoritmo dinâmico
de quantidade-para-encher (`floor(available / (body_width +
gap))`) está diferido — exige refactor inline-region não
disponível no Layouter actual (mesma razão que `Block.width` /
`Boxed.width` em P156G/H).

**Layout actual**: emite o body uma vez no contexto actual.
**Walk**: percorre o body uma vez (counters/labels resolvem;
sem multiplicação de state — vanilla também só conta uma vez).

### `is_empty` / `plain_text` / `map_*`

- `is_empty()`: proxy via body (consistente com Block/Boxed).
- `plain_text()`: recurse no body sem multiplicar (paridade
  não visível em texto plano).
- `PartialEq`: cobre todos os fields (body, gap, justify).
- `map_content` / `map_text`: recurse no body; preserva
  gap/justify (Copy primitivos).

### Limitações conscientes (P156J)

- Algoritmo dinâmico de "quantidade-para-encher" diferido per
  ADR-0054 graded. Single-render é aproximação aceite.
- `justify: true` armazenado mas distribuição real adiada.
- Erro vanilla "infinite content" (se largura disponível for
  unbounded) não implementado — requer detecção de fr context
  no Layouter.

### Padrão emergente "Smart<T> → Option<T> ou default" — N=6

P156J aplica o padrão simplificador pela sexta vez consecutiva:
- P156E `Smart<Parity>` → `Option<Parity>`.
- P156F angles default 0 (em vez de Smart).
- P156G/H `Smart<Rel<Length>>` para width → `Option<Length>`.
- P156I `Smart<Rel<Length>>` para spacing → `Option<Length>`.
- **P156J `Length` (default zero vanilla) → `Option<Length>`
  para `gap`; `bool` directo para `justify` (default vanilla
  `true`).**

**N=6 aplicações** — patamar empírico forte e crescente.
Promoção a ADR meta segue como candidato P156K-meta documentado
em ADR-0061 §"Aplicações cumulativas".

### Helper `extract_length` reuso N=6

`extract_length` em `stdlib/layout.rs` foi reusado em P156C
(pad), P156D (h+v), P156G (block.width/height/inset), P156H
(box.width/height/inset/baseline), P156I (stack.spacing) e
agora **P156J (repeat.gap)**. Sexta aplicação consecutiva —
emergiu como vocabulário canónico para coerção de Length em
named args. Promoção a helper público em release futuro
(refactor scope-out).

## Variante `Content::StateDisplay` — Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ)

```rust
Content::StateDisplay {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Render-mediated state display vanilla `state.display(callback)`.
Walk emite Tag via `extract_payload` arm; `apply_state_displays`
pós-fixpoint (paralelo `apply_state_funcs` P191B) pre-renderiza
Content via `apply_func(callback, [value], ctx, engine)` e
armazena em `Introspector.state_displays[(key, loc)]`. Layout
arm consome via `Introspector::state_display_value(key, loc)`
— Layouter permanece puro (sem Engine+ctx em signature; paridade
arquitectural estrita Opção γ vs α/β/δ P239 audit).

**`callback: None`**: state value renderiza directo
(`Value::Content(c)` passa-through; `Value::Str(s)` via
`Content::text(s)`; outros tipos fallback `Content::Empty`).
**`callback: Some(func)`**: aplicada ao value via apply_func;
resultado convertido para Content pela mesma regra.

**Primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-P229** — feature runtime nova + walk
integration merece L0 tocado partial (este bloco + bloco
`state_display` em `rules/stdlib.md` + bloco `apply_state_displays`
em `rules/introspect.md`).

## Variante `Content::CounterDisplayCallback` — Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto P240)

```rust
Content::CounterDisplayCallback {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Render-mediated counter display real walk-time vanilla
`counter.display(callback)`. **Paralelo absoluto** a
`Content::StateDisplay` P240 (mesmo pattern; mesma arquitectura
Opção γ; Layouter permanece puro).

Walk emite Tag via `extract_payload` arm; `apply_counter_displays`
pós-fixpoint (paralelo `apply_state_displays` P240) converte
`intr.counters.value_at(key, loc)` (Option<&[usize]>) para
`Value::Array(Vec<Value::Int>)` representando counter state e
chama `apply_func(callback, [array], ctx, engine)`. Resultado
Content armazenado em `intr.counter_displays[(key, loc)]`. Layout
arm consome via `Introspector::counter_display_value(key, loc)`.

**Forma do Value passado ao callback** (Decisão 4 P241): paridade
vanilla `CounterState = SmallVec<[u64; 3]>` representado como
`Value::Array(Vec<Value::Int>)`. Counter inexistente:
`Value::Array(vec![])` (vector vazio).

**`callback: None`** + counter populated: formato default "1.2.3"
via join "." (paridade `formatted_counter_at` P177).
**`callback: None`** + counter inexistente: `Content::Empty`.

**Coexiste com `Content::CounterDisplay { kind }` legacy
single-pass** — variant nova paralela preservada inalterada
(Decisão 1 P241 Opção α: variant nova vs refino legacy).

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229** —
N=1 (P240) → 2 (P241) cumulativo; pattern "L0 tocado para
features runtime novas + walk integration" promove-se a N=2.

## Refino `Content::Block.radius` + `Content::Boxed.radius` — Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5)

P231 introduziu fields `radius: Option<Length>` + `clip: bool` em
ambos os variants como scope-out P156G/H graded ("semantic adiada").
P242 promove para semantic real via:

```rust
// Refino tipo radius:
- radius: Option<Length>,      // P231 — single Length OR None
+ radius: Corners<Length>,     // P242 — per-corner (top_left/top_right/bottom_right/bottom_left)

// Default migrado:
- radius: None,
+ radius: Corners::uniform(Length::ZERO),
```

Audit C1 P242 refinou hipótese spec: assumira "5 fields → 7 fields"
mas Block/Boxed já tinham 8 fields P231; ajuste real é "refine field
type" (`Option<Length>` → `Corners<Length>`) + materialize semantic
clip. Sem `P242.div-N` formal — paridade lição N=5 cumulativo
ajustes triviais audit precedentes.

**`clip` semantic materializada P242**:
- `clip: false`: comportamento inline original preservado (radius
  armazenado sem clip-mask emit; semantic radius isolada continua
  graded).
- `clip: true` + radius zero: Layouter emite `FrameItem::Group` com
  `clip_mask: Some(ShapeKind::Rect)` (paridade DEBT-30 P79).
- `clip: true` + radius non-zero: Layouter emite `FrameItem::Group`
  com `clip_mask: Some(ShapeKind::RoundedRect { radii: radius })`;
  PDF exporter desenha Bezier 4 corners path via
  `emit_rounded_rect_ops` (kappa = 0.552_284_749_831 paridade
  Ellipse same ficheiro).

**Promoção real graded ADR-0054 P156G/H → semantic concreta P242**:
sub-padrão emergente "promoção real scope-out ADR-0054 graded"
N=1 inaugurado P242 (Categoria A.4 P231 graded → A.4 materializado
parcial). Outset/fill/stroke restantes em Block/Boxed permanecem
scope-out (refino futuro).

stdlib `block(radius:)` / `box(radius:)` aceitam:
- `Length` uniforme (paridade pre-P242; `extract_length`).
- `Dict` por canto: `top-left` / `top-right` / `bottom-right` /
  `bottom-left` / `top` / `bottom` / `left` / `right` / `rest`.
  Precedência: canto específico > eixo > rest (paridade
  `extract_sides_lengths` per ADR-0064 Caso C).

## Promoção scope-outs Pad.right / Block.width / Boxed.width — Passo 243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)

P156C declarou `Pad.right` scope-out ("Layouter actual não tem
mecânica de largura útil por arm"). P156G/H declararam
`Block.width` / `Boxed.width` semantic real adiada ("armazenado
mas não impõe limite real"). **P243 promove os 3 para semantic
real** via `regions.current.width` save/restore:

```rust
// Em layout/mod.rs Pad arm:
let saved_width = self.regions.current.width;
self.regions.current.width = (saved_width - right).max(0.0);
self.layout_content(body);
self.regions.current.width = saved_width;
```

```rust
// Em Block arm:
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    self.regions.current.width = (line_start + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

```rust
// Em Boxed arm (paralelo Block):
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    self.regions.current.width = (cursor_x + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

**Mecânica**: `layout_word` em `cursor.rs` consulta
`self.regions.current.width` para width-aware wrap; promoção
P243 garante que width efectiva reflecte constraint user-provided
durante body layout. **Save/restore LIFO** preserva semantic
cumulativo para Pad/Block aninhados (P243 test
`p243_pad_aninhado_largura_cumulativa_preservada`).

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=2
cumulativo (P242 radius/clip + P243 multi-region attrs).
**§"Limitações conscientes" P156C/G/H** secções relevantes
transitam de "scope-out" / "armazenado adiada" para
"materializado P243" (anotação cruzada).

## Promoção scope-outs Block/Boxed fill+stroke+outset — Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4)

P156G/H declararam **9 atributos vanilla scope-out** em Block e
**6 em Boxed** (per ADR-0054 graded; refino futuro). P231
materializou `outset` armazenado (semantic adiada). P242 promoveu
`radius` + `clip` para semantic real (RoundedRect + clip_mask).
**P247 promove 3 cosméticos visuais em agregação**: `outset`
semantic real activado + **2 fields novos** `fill` + `stroke`
em Block + Boxed (paridade simétrica).

```rust
// Em Content::Block (P247):
Block {
    body, width, height, inset, breakable,
    outset, radius, clip,                                  // P231/P242
    fill:   Option<Color>,                                 // P247 NOVO
    stroke: Option<Stroke>,                                // P247 NOVO
}

// Em Content::Boxed (P247; paridade simétrica):
Boxed {
    body, width, height, inset, baseline,
    outset, radius, clip,                                  // P231/P242
    fill:   Option<Color>,                                 // P247 NOVO
    stroke: Option<Stroke>,                                // P247 NOVO
}
```

**Default**: `fill: None`, `stroke: None`, `outset: Sides::ZERO`
preservam output bit-equivalente a P246 (backward compat estrita).

**Types fixados** (per audit C1 §2.2 — `Color` Copy / `Stroke`
Clone existentes em `geometry.rs:24`; `Paint` enum não existe):

- `fill: Option<Color>` — `Color` Copy directo (Stroke já usa
  Color directo). Refactor para `Paint` enum é cross-cutting
  fora de scope P247 (futuro ADR dedicada).
- `stroke: Option<Stroke>` — reuso de struct `Stroke { paint:
  Color, thickness: f64 }`.

**Layouter activação Shape + outset semantic real (Decisão 3-5)**:

Quando `fill.is_some() || stroke.is_some() || outset != ZERO`,
Layouter emite `FrameItem::Shape { pos, kind, width, height,
fill, stroke }` ANTES do body (snapshot-and-insert via
`current_items.insert(items_before, ...)`):

```
outer bound:  pos.x - outset.left, pos.y - outset.top
shape bounds: outer_bound + (width + outset.left+right,
                              height + outset.top+bottom)
body origin:  pos.x + inset.left, pos.y + inset.top
```

- `kind`: `Rect` se radius == zero; `RoundedRect { radii: radius }`
  caso contrário (reuso P242).
- **Z-order**: Shape inserido em `items_before` para que
  fill+stroke renderizem por baixo do conteúdo body (paridade
  vanilla PDF z-order natural).
- **Outset semantic** (cenário A audit §2.4-§2.5 — outset zero-uso
  pré-P247): cursor.y avança `outset.top` antes do inset.top;
  `outset.bottom` após height min; bounds Shape expandem em
  todos os lados.
- **clip=true preserva semantic P242**: body items wrapped em
  `FrameItem::Group` com clip_mask; Shape fill+stroke + Group(body)
  coexistem (Shape primeiro, Group depois — z-order natural).

**stdlib `block(fill:, stroke:)` + `box(fill:, stroke:)` (P247)**:

- `fill` aceita `Value::Color` directo; tipos inválidos rejeitados
  com erro hard (paridade pattern Grid/Table P228).
- `stroke` reusa `extract_stroke` helper pré-existente (P227
  `stdlib/layout.rs:351`): aceita `Length` (Color preto + thickness
  resolvido), `Color` (thickness default 1pt), ou `Stroke` directo.

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=2 →
**N=3 cumulativo** P247 (P242 radius+clip = N=2; **P247
outset+fill+stroke = N=3 promoções reais agregadas**). Contando
granular: 5 promoções cumulativas (P242 radius + P242 clip +
P247 outset + P247 fill + P247 stroke).

**Sub-padrão emergente "agregar promoções scope-outs cosméticos
visuais"** N=1 inaugurado P247 (3 promoções num passo único;
magnitude controlada M-L; coesão semantic forte).

**§"Limitações conscientes" P156G fechadas em P247**: 5 dos 9
scope-outs originais Block fechados cumulativamente (outset
P231→P247 + radius P242 + clip P242 + fill P247 + stroke P247);
restam 4 (spacing + above + below + sticky).

**§"Limitações conscientes" P156H fechadas em P247**: 5 dos 6
scope-outs originais Boxed fechados cumulativamente (outset +
radius + clip + fill + stroke); resta 1 (stroke-overhang).

## Promoção graded → real semantic Block.breakable + Boxed.height + TableCell overflow — Passo 248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa)

P156G declarou `Block.breakable` "semantic adiada per ADR-0054
graded — armazenado mas não impede quebra". P156H declarou
`Boxed.height` "semantic real adiada". P157B declarou
`TableCell.body` sem detecção de overflow vertical. **P248
activa as 3 semanticas em agregado** via mecanismo comum de
medição antecipada (`measure_content_constrained` puro
pré-existente, audit C1 §2.4 confirmado).

**Activação A — `Block.breakable`** (Layouter `mod.rs` Block arm):

```rust
if !*breakable {
    let avail_w = match width {
        Some(w) => w.resolve_pt(font),
        None    => self.available_width(),
    };
    let (_, body_h) = self.measure_content_constrained(body, avail_w);
    let height_min = height.map(|h| h.resolve_pt(font)).unwrap_or(0.0);
    let inner_h = body_h.max(height_min);
    let block_total_h = outset_top + inset_top + inner_h
                       + inset_bottom + outset_bottom;
    let page_usable_h = self.available_height();
    let remaining_h = self.page_bottom_limit()
                    - self.regions.current.cursor_y.0;
    if block_total_h <= page_usable_h && block_total_h > remaining_h {
        self.new_page();
    }
    // else: cabe na actual OU overlong (emit normal — paridade vanilla).
}
```

3 cenários distintos:
- Cabe na página actual → emit normal preservado.
- Cabe numa página nova mas não na actual → `new_page()`
  antecipado antes do emit.
- Overlong (excede página inteira) → emit normal (paridade
  vanilla "overlong atómico não quebra").

**Default `breakable: true` preserva comportamento P156G literal**
(zero overhead; sem medição antecipada).

**Activação B — `Boxed.height` overflow** (Layouter Boxed arm):

```rust
if let Some(h) = height {
    if *clip {
        let h_pt = h.resolve_pt(font);
        let (body_w_real, body_h_real) =
            self.measure_content_constrained(body, avail_w_box);
        if body_h_real > h_pt {
            let body_items = drain items emitidos pelo body;
            push FrameItem::Group {
                pos: top-left da caixa,
                clip_mask: Some(ShapeKind::Rect),
                inner_height: h_pt,
                items: body_items,
            };
        }
    }
}
```

- `height: None` → preservado P156H literal.
- `height: Some(h)` + body cabe → preservado.
- `height: Some(h)` + body excede + `clip: true` → wrap em Group
  com clip_mask Rect (reuso mecanismo P242).
- `height: Some(h)` + body excede + `clip: false` → emit normal
  (overflow visível; paridade vanilla default).

**Activação C — `TableCell.body` overflow clip implícito**
(Layouter `grid.rs` GridCell/TableCell arm):

```rust
let (cell_h_measured, cell_items) =
    self.layout_sub_frame_with_width(cell, body_x, body_w);
// ... translate cell_items para abs ...
let cell_overflow = cell_h_measured > body_h;
if cell_overflow {
    push FrameItem::Group {
        pos: (body_x, body_y),
        clip_mask: Some(ShapeKind::Rect),
        inner_width: body_w,
        inner_height: body_h,
        items: translated_items,
    };
} else {
    for item in translated_items { push directo; }  // P157B preservado
}
```

- Cell body cabe em `regions.cell.height` (P246) → preservado
  P157B literal.
- Cell body excede → **clip implícito ao limite cell** (paridade
  vanilla default).
- **Row break real é scope-out P248** (refino futuro per
  ADR-0054 graded; promoção candidata a passo dedicado;
  DEBT-34e preservado aberto cumulativo — distinto: DEBT-34e
  cobre colspan/rowspan placement, P248 cobre overflow Y).

**Sub-padrão "promoção graded → real semantic activação consumer"
N=1 → N=2 cumulativo P248**: P245 inaugurou N=1 (Place float
real); **P248 N=2 cumulativo agregado** (3 sub-activações
granulares em passo único: breakable + height + cell overflow).

**Sub-padrão emergente "agregar promoções graded → real
multi-consumer via mecanismo comum"** N=1 inaugurado P248:
distinto de P247 "agregar promoções cosméticos visuais"
(ortogonais aditivos) — P248 agrega semantic real com
mecanismo comum (medição antecipada).

**Promoções reais scope-outs ADR-0054 graded granular cumulativas
pós-P248**: 8 = (P242 radius + P242 clip) + (P247 outset + P247
fill + P247 stroke) + (P248 breakable + P248 height + P248 cell
overflow). Limiar conceptual sólido para ADR meta candidata
futura XS admin (N≥6 patamar atingido).

**§"Limitações conscientes" P156G/H/P157B fechadas em P248**:
- Block.breakable semantic real activada (resta 4/9 cumulativo:
  spacing + above + below + sticky).
- Boxed.height semantic real activada cumulativamente.
- TableCell overflow Y clip implícito (row break diferido).

## Promoção Block spacing + above + below + sticky — Passo 250 (M9d / M7+5; ADR-0079 Categoria A.4 Block COMPLETO; cita ADR-0082 PROPOSTO N=1 primeira aplicação citante)

P156G declarou 9 scope-outs originais Block; P247 + P248
fecharam cumulativamente 5/9 cosméticos visuais + breakable
semantic real. **P250 fecha os 4 scope-outs restantes** em
agregação (spacing + above + below + sticky) e marca **Block
A.4 COMPLETO 10/10** (incluindo breakable contado como décimo
elemento).

```rust
// Em Content::Block (P250):
Block {
    body, width, height, inset, breakable,        // P156G + P248
    outset, radius, clip,                         // P231 + P242
    fill, stroke,                                 // P247
    spacing: Option<Length>,                      // P250 NOVO
    above:   Option<Length>,                      // P250 NOVO
    below:   Option<Length>,                      // P250 NOVO
    sticky:  bool,                                // P250 NOVO
}
```

**Block fields: 10 → 14**. **Boxed fields: 10 preservado** —
estes 4 scope-outs são exclusivos Block (vanilla BlockElem
properties; BoxElem não os tem). **Asymetria intencional**;
sub-padrão "refino aditivo paralelo entre variants irmãos" N=5
P247 **não aplica P250**.

**Default values**:
- `spacing: None` (cristalina graded; vanilla `Em::new(1.2)`).
- `above: None` (fallback `spacing`).
- `below: None` (fallback `spacing`).
- `sticky: false`.

**Activação A — `spacing`/`above`/`below` cursor.y advance via
collapse semantic** (paridade vanilla CSS margin collapse):

- `above_pt = above.or(spacing).map(resolve).unwrap_or(0.0)`.
- `below_pt = below.or(spacing).map(resolve).unwrap_or(0.0)`.
- Entre Blocks consecutivos: `gap = max(prev.below, curr.above)`;
  cursor.y advance = `gap - prev.below_already_applied`.
- Primeiro Block do Sequence: `above` suprimido (`block_chain_
  active == false`).
- Non-Block intermediário quebra chain (`block_chain_active`
  reset).

**Layouter fields novos P250**:
- `prev_block_below_pending: f64` (default 0.0) — below pendente
  do prev Block para CSS-style collapse.
- `block_chain_active: bool` (default false) — chain state;
  reset entre Sequences (save/restore) + non-Block children.

**Activação B — `sticky` lookahead 1-block** (Sequence consumer):

```rust
if let Content::Block { sticky: true, .. } = part {
    if let Some(next) = iter.peek() {
        let part_h = measure_content_constrained(part, avail_w).1;
        let next_h = measure_content_constrained(next, avail_w).1;
        let combined = part_h + next_h;
        let remaining = page_bottom - cursor_y;
        let page_usable = available_height;
        if combined > remaining && combined <= page_usable {
            new_page();  // break antes do block sticky
        }
        // else: cabe OU overlong → emit normal.
    }
}
```

**Refactor Sequence consumer cross-arm P250** (pattern emergente
N=1 inaugurado):

```rust
Content::Sequence(parts) => {
    let saved_below = self.prev_block_below_pending;
    let saved_chain = self.block_chain_active;
    self.prev_block_below_pending = 0.0;
    self.block_chain_active       = false;
    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        // Sticky pre-layout lookahead.
        // ... see Activação B ...
        self.layout_content(part);
        if !matches!(part, Content::Block { .. }) {
            self.block_chain_active       = false;
            self.prev_block_below_pending = 0.0;
        }
    }
    self.prev_block_below_pending = saved_below;
    self.block_chain_active       = saved_chain;
}
```

**stdlib `block(spacing:, above:, below:, sticky:)` P250**:

- `spacing`/`above`/`below`: helper inline `extract_block_length`
  (paridade pattern P247); negativos rejeitados com erro hard.
- `sticky`: `Value::Bool` directo; tipos errados rejeitados.

**Citação ADR-0082 PROPOSTO N=1 (primeira aplicação citante)**:

Os 4 critérios operacionais ADR-0082 verificados:
1. **Storage prévio** ✓ — 4 fields scope-out P156G declarados
   originalmente.
2. **Consumer Layouter pre-promoção graded** ✓ — 4 args
   "rejeitados em `native_block` com erro hard" P156G.
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.4
   confirmou: vanilla `Em::new(1.2)` default; `above.or(spacing)`
   fallback; `max(prev.below, curr.above)` collapse; sticky
   default false.
4. **Backward compat literal** ✓ — defaults (None×3 + false)
   produzem output PDF bit-equivalente para Block sem estes
   args (sentinela `p250_block_defaults_preserva_output_pre_p250`).

**Sub-padrão "promoção real scope-out ADR-0054 graded"**
granular N=8 → **N=12 cumulativo P250** (P242 radius + P242
clip + P247 outset + P247 fill + P247 stroke + P248 breakable
+ P248 height + P248 cell_overflow + **P250 spacing + above +
below + sticky**).

**Sub-padrão "Refactor Sequence consumer cross-arm"** N=1
inaugurado P250 — primeira aplicação peekable + neighbour
context no Layouter. Pattern candidato a formalização N=3-4
futuro (hipóteses: pagebreak weak collapse; HSpace/VSpace
weak adjacent).

**§"Limitações conscientes" P156G fechadas em P250**: 10/10
scope-outs originais P156G + breakable contado = **Block A.4
COMPLETO**.

**Boxed continua 5/6 scope-outs** (resta stroke-overhang;
P250 não toca Boxed por assimetria intencional).

---

## Variant `Content::Footnote` — Passo 295 (`P-footnote-cluster` Fase 1 marker only)

```rust
Content::Footnote {
    body: Box<Content>,
}
```

**Fase 1 P295 (HE marker only)**: variant minimal `(a)` per A.2 do
diagnóstico `diagnostico-footnote-cluster-passo-295.md`. Layouter
emite apenas marker `[N]` superscript inline; body **armazenado mas
não renderizado** no rodapé nesta fase. Numeração via walker counter
simples `Layouter::footnote_counter: u32` (sem Counter/Introspector
machinery — magnitude reduzida).

### Simplifications cristalino per ADR-0054 graded vs vanilla

Vanilla `FootnoteElem` (lab/.../model/footnote.rs:62-86):

```rust
#[elem(scope, Locatable, Tagged, Count)]
pub struct FootnoteElem {
    #[default(Numbering::Pattern("1"))]
    pub numbering: Numbering,
    #[required]
    pub body: FootnoteBody,
}

pub enum FootnoteBody {
    Content(Content),
    Reference(Label),
}
```

Cristalino P295 scope-outs:

- `numbering: Numbering` (default `"1"`) **scope-out** (cosmético;
  arabic default implícito). Padrão "variant rico com cosméticos
  opcionais" N=4 cumulativo **preservado inalterado** — A.2 → (a)
  minimal.
- `FootnoteBody::Reference(Label)` **scope-out** (multi-ref
  footnotes — frente futura P295.X).

### Consumer Layouter Fase 1

```rust
Content::Footnote { body: _ } => {
    self.footnote_counter += 1;
    let n = self.footnote_counter;
    let marker = format!("[{}]", n);
    self.layout_content(&Content::text(marker));
}
```

Body é silenciosamente descartado em Fase 1 — sub-passos P295.1/P295.2
renderizam no rodapé via 2-pass layout futuro.

### Match arms exhaustive defesa compilador (8 sítios)

| Local | Operação |
|---|---|
| `content.rs:is_empty()` | `false` (marker sempre observable) |
| `content.rs:plain_text()` | recursa em `body.plain_text()` |
| `content.rs:PartialEq` | `body == body` |
| `content.rs:map_content()` | recursa em body |
| `content.rs:map_text()` | recursa em body |
| `rules/introspect.rs:materialize_time` | recursa em body |
| `rules/introspect.rs:walk` | walk em body |
| `rules/introspect/locatable.rs` | `false` (Fase 1 não-locatable) |

### Frentes pendentes pós-P295

- **P295.1** — nota corpo renderizada no rodapé da página
  correspondente (requer 2-pass layout; magnitude L).
- **P295.2** — overflow multi-página (footnote ocupa páginas
  subsequentes se rodapé não chega).
- **P295.X** — footnote reference via `#footnote(<label>)` bloqueado
  por scope methods em stdlib.

### Hash `export.rs 66cb8ac3` preservado (12º passo consecutivo)

ADR-0098 §"single source of truth" honrada — emit é agnóstico ao
variant (marker chega como `FrameItem::Text` standard).

---

## Variants `Content::MathAccent` + `Content::MathCancel` — Passo 296 (`P-math-accent-cancel`)

```rust
Content::MathAccent {
    base:   Box<Content>,
    accent: Box<Content>,
},
Content::MathCancel {
    body: Box<Content>,
},
```

**P296 (HIV + (a) minimal)**: variants minimal per A.2 do diagnóstico
`diagnostico-math-accent-cancel-passo-296.md`. Layouter math expõe
handlers dedicados `layout_accent` + `layout_cancel` em
`rules/math/layout/mod.rs` paralelo a `layout_frac` (P37).

### A.0.0 N=4 — refutação significativa Tabela A.4

Tabela A.4 linhas 118-119 marcavam `accent` e `cancel` como
**`parcial`** mas inspecção literal A.0.0 confirmou **zero hits**
em todo `01_core/src/` pré-P296. Status real era **AUSENTE**. P296
corrige classificação `ausente` → `implementado`.

**Magnitude da refutação**: média (factual-significativa) —
classificação inteira inválida. **Refuta hipótese degenerescência
§6.6 P295** — A.0.0 template valida com refutação real genuína.

### Simplifications cristalino per ADR-0054 graded vs vanilla

Vanilla `AccentElem` (2 required + 2 cosméticos `size`/`dotless`).
Vanilla `CancelElem` (1 required + 5 cosméticos
`length`/`inverted`/`cross`/`angle`/`stroke`).

Cristalino P296 scope-outs:
- `AccentElem.size`/`dotless` → scope-out (cosméticos).
- `CancelElem.length`/`angle`/`stroke` → scope-out (cosméticos).
- `CancelElem.inverted`/`cross` → scope-out (toggles funcionais;
  materialização adiada para passo P296.X candidato).

**Padrão "variant rico" N=4 cumulativo preservado inalterado** —
A.2 → (a) minimal recusa qualificação gratuita.

### Consumer Layouter (paralelo `layout_frac`)

```rust
// layout_node arm:
Content::MathAccent { base, accent } => self.layout_accent(base, accent, style),
Content::MathCancel { body }         => self.layout_cancel(body, style),

fn layout_accent(&self, base, accent, style) -> MathBox {
    // Centra accent horizontalmente sobre bbox de base.
    // dx = (base.width - accent.width) / 2
    // Empilha verticalmente: accent acima, base abaixo.
}

fn layout_cancel(&self, body, style) -> MathBox {
    // Layout body; anexa FrameItem::Line diagonal default
    // (bottom-left → top-right; "rising" per vanilla angle padrão).
}
```

### Match arms exhaustive defesa compilador (9 sítios)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | concatena base+accent / body |
| `content.rs:PartialEq` | structural |
| `content.rs:map_content()` | recurse |
| `content.rs:map_text()` | terminal (paralelo MathFrac/MathRoot) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal (math structural) |
| `rules/introspect/locatable.rs` | `false` |
| `compiler/layout/mod.rs` | fallthrough math (paralelo MathFrac) |
| `rules/math/layout/mod.rs:layout_node` | handlers dedicados |

`is_empty()` herdado via catch-all `_ => false` (math structural
sempre observable).

### Frentes pendentes pós-P296

- **P296.1** — `underover` (Tabela A.4 linha 119 vanilla
  `UnderoverElem`).
- **P296.2** — `op` (vanilla `OpElem`).
- **P296.X** — `cancel(..., inverted: true)` / `cross: true`
  toggles funcionais.

### Hash `export.rs 66cb8ac3` preservado (13º passo consecutivo)

ADR-0098 §"single source of truth" honrada — math layout produz
`FrameItem::Text`/`Glyph`/`Line` standard, sem operadores PDF novos.

---

## Variant `Content::MathUnderover` — Passo 297 (`P296.1`)

```rust
Content::MathUnderover {
    base:  Box<Content>,
    under: Option<Box<Content>>,
    over:  Option<Box<Content>>,
},
```

**P297 (HV'.a + (b) Option fields)**: variant agregado cristalino
per ADR-0054 graded. Layouter math `layout_underover` empilha
over/base/under verticalmente.

### A.0.0 N=5 magnitude alta — refutação significativa spec

Spec P297 §A.1.4 assumia vanilla `UnderoverElem { base, under?, over? }`
unificado. **Inspecção literal refutou**: vanilla
`lab/.../math/underover.rs` fragmenta em **12 elementos separados**:

| Elemento vanilla | Estrutura |
|---|---|
| `UnderlineElem` / `OverlineElem` | `{ body }` |
| `UnderbraceElem` / `OverbraceElem` | `{ body, annotation? }` |
| `UnderbracketElem` / `OverbracketElem` | `{ body, annotation? }` |
| `UnderparenElem` / `OverparenElem` | `{ body, annotation? }` |
| `UndershellElem` / `OvershellElem` | `{ body, annotation? }` |

**HV'.a justificação** (cristalino agregação):
- ADR-0054 graded vigente: cristalino aceita divergência
  consciente vs vanilla para simplificação arquitectural.
- Paralelo P296 `MathAccent` (agrega vários accents Unicode em
  1 variant).
- Future-proof: refino discriminator (`UnderoverKind::Brace`/
  `Bracket`/etc.) candidato P297.X se cluster vanilla exigir
  paridade fina.

### "Variant rico" N=5 candidato genuíno

A.2 → (b) Option `Box<Content>` estrutural — **primeira
qualificação genuína desde P287 refutação**:
- P156G/H/I bool defaults — refutados em P287.
- P284 attribs primitivos (Length/Color) — diferente categoria.
- **P297 `MathUnderover` Option estrutural** — primeira qualificação real.

**Promoção adiada per P273.17 §0** (uma ADR meta por passo;
§8.7' N=5 também qualifica). Consolidação cumulativa permitirá
promoção robusta sem arbitragem.

### Consumer Layouter `layout_underover` (paralelo P296)

```rust
fn layout_underover(&self, base, under, over, style) -> MathBox {
    let base_box = self.layout_node(base, style);
    let over_box = over.map(|c| self.layout_node(c, style));
    let under_box = under.map(|c| self.layout_node(c, style));
    let w = base_box.width
        .max(over_box.as_ref().map(|b| b.width).unwrap_or(0.0))
        .max(under_box.as_ref().map(|b| b.width).unwrap_or(0.0));
    // Empilha: over (topo) + base (meio) + under (fundo);
    // cada centrado horizontalmente.
    // ...
}
```

### Match arms exhaustive defesa compilador (9 sítios paralelo P296)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | concatena over+base+under (ordem visual) |
| `content.rs:PartialEq` | structural com Options |
| `content.rs:map_content()` | recurse condicional em Option |
| `content.rs:map_text()` | terminal (paralelo MathFrac/MathAccent) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `compiler/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | handler dedicado |

### Hash `export.rs 66cb8ac3` preservado (14º passo consecutivo)

ADR-0098 §"single source of truth" honrada — math layout produz
`FrameItem::Text/Glyph` standard.

### Frentes pendentes pós-P297

- **P297.X** — discriminator `UnderoverKind::Brace`/`Bracket`/`Paren`/`Shell`
  para paridade visual fina com vanilla (cosmético per ADR-0054 graded).
- **P296.2** — `op` (vanilla `OpElem`) — **resolvido P298**.

---

## Variant `Content::MathOp` — Passo 298 (`P296.2 — fecho cluster math 4/4`)

```rust
Content::MathOp {
    text:   Box<Content>,
    limits: bool,
},
```

**P298 (HV'' adaptado)**: variant minimal paridade vanilla
`OpElem { text: Content, limits: bool }`. Handler `layout_op`
trivial (delegate); verdadeira inovação é **cross-variant
interaction** com `MathAttach`.

### A.0.0 N=6 magnitude alta — heurística limits-style já existia

Cristalino tinha heurística limits-style **hardcoded** em
`rules/math/layout/attach.rs:55-61`:

```rust
let is_limits = self.block && match base {
    Content::MathIdent(s) | Content::MathText(s) => {
        let ch = s.chars().next().unwrap_or('\0');
        symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
    }
    _ => false,
};
```

E em `symbols.rs:170`:

```rust
pub fn is_limit_function(s: &str) -> bool {
    matches!(s, "lim" | "max" | "min" | "sup" | "inf" | "limsup" | "liminf")
}
```

**P298 estende sem substituir** — adiciona 1 arm:

```rust
Content::MathOp { limits, .. } => *limits,
```

Heurística pré-P298 preservada — `MathIdent("lim")` continua a
funcionar via fallback `is_limit_function`. Regressão bit-exact
validada por teste `p298_regressao_math_ident_lim_continua_a_funcionar`.

### Cross-variant interaction (paradigma inaugural)

P296/P297 introduziram variants com handlers **independentes**.
P298 introduz **interaction cross-variant**: `MathOp.limits`
afecta layout de `MathAttach`. Paradigma **genuinamente novo**
no cluster math:

```
#op("custom", limits: true) → MathOp{limits:true}
       │
       ▼
parent MathAttach { base: MathOp{...}, sub: ..., sup: ... }
       │
       ▼ (layout_attach)
is_limits ← detecta MathOp{limits:true} → limits-style
       │
       ▼
sub/sup empilhados ABAIXO/ACIMA da base (não lateral)
```

### Operadores vanilla pré-definidos (scope-out P298)

Vanilla define 36+ operadores no scope math via macro `ops!`:
`arccos`/`arcsin`/`cos`/`lim`/`sup`/`max`/`min`/etc. Cristalino
**scope-out P298** — passo P298.X candidato se necessário.

**Workaround actual**: `lim`/`sup`/`max` continuam a funcionar
via `MathIdent` literal + heurística `is_limit_function`. Para
operators custom com `limits` explícito, user usa `#op("...", limits: true)`.

### "Variant rico" N=5 — caso ambíguo, P298 NÃO qualifica

P297 §6.4 estabeleceu "variant rico" N=5 como qualificação
Option `Box<Content>` estrutural. P298 tem `bool limits` —
**discriminador estrutural mas não Option**.

**Decisão**: NÃO qualificar gratuitamente. `bool` é caso
intermédio; diluir o gatilho violaria anti-padrão
over-formalização P273.17 §0. Padrão N=5 candidato **adiado em
P297 e preserved em P298**.

### Sub-padrão "cluster math handler dedicado" N=3 ambíguo

Quantitativo: 3 handlers (`layout_accent`, `layout_cancel`,
`layout_underover`, `layout_op`). Qualitativo: `layout_op` é
**trivial delegate** (não estructuralmente igual a P296/P297).

**Decisão**: adiar promoção. N=3 não justifica formalização se
qualidade do 3.º caso é menor que os anteriores.

### Match arms exhaustive defesa compilador (9 sítios paralelo P296/P297)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | `text.plain_text()` (limits é layout, não texto) |
| `content.rs:PartialEq` | structural (text + limits) |
| `content.rs:map_content()` | recurse em text; preserva limits |
| `content.rs:map_text()` | terminal (paralelo cluster math) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `compiler/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | `layout_op` trivial delegate |

**+1 sítio crítico** (não match arm exhaustive, mas paradigma novo):
- `rules/math/layout/attach.rs:55-61` — modificação `is_limits`.

### Hash `export.rs 66cb8ac3` preservado (15º passo consecutivo)

ADR-0098 §"single source of truth" honrada — math layout produz
`FrameItem::Text/Glyph` standard. **Cluster math 4/4 fechado**.

### Frentes pendentes pós-P298

- **P298.X** — operadores vanilla pré-definidos (`lim`/`sin`/etc.)
  como scope module integrado.
- **P297.X** — discriminator `UnderoverKind`.
- **P296.X** — toggles `inverted`/`cross` cancel.

---

## Variant `Content::MathStyled` — Passo 311b.2 (Caminho I per P311a)

```rust
Content::MathStyled {
    kind:    Option<MathStyleKind>,  // None = inherit; Some = override
    bold:    Option<bool>,           // idem
    italic:  Option<bool>,           // idem
    body:    Box<Content>,
    cramped: Option<bool>,           // relevante para script/sscript
}
```

Wrapper de variant glyph / flags math style — implementa 12 funções
vanilla `bb`/`bold`/`cal`/`frak`/`italic`/`mono`/`sans`/`scr`/`script`/
`serif`/`sscript`/`upright`. Todos os campos override usam `Option`
para distinguir "outer não overriding" (`None`) de "outer força este
valor" (`Some(_)`). Necessário para que `bold(bb(x))` preserve
DoubleStruck inner enquanto aplica bold orthogonalmente.

### Mapping 12 funções → MathStyled fields

| Função | `kind` | `bold` | `italic` | `cramped` |
|---|---|---|---|---|
| `bb` | `Some(DoubleStruck)` | None | None | None |
| `bold` | None | `Some(true)` | None | None |
| `cal` | `Some(Chancery)` | None | None | None |
| `frak` | `Some(Fraktur)` | None | None | None |
| `italic` | None | None | `Some(true)` | None |
| `mono` | `Some(Monospace)` | None | None | None |
| `sans` | `Some(SansSerif)` | None | None | None |
| `scr` | `Some(Roundhand)` | None | None | None |
| `script` | `Some(Script)` | None | None | `Some(true)` |
| `serif` | `Some(Plain)` | None | None | None |
| `sscript` | `Some(SScript)` | None | None | `Some(true)` |
| `upright` | None | None | `Some(false)` | None |

### Hash `content.rs` quebra deliberadamente

**Marco arquitectural**: P311b.2 termina sequência **27 passos
consecutivos** com hash `82d3c47d` preservado (last drift: P282).
Quebra **explicitamente assumida** per ADR-0033 (paridade observable;
forma diverge sem afectar output). Paralelo arquitectural directo a
P298 (`MathOp`) — ambos adicionam variant Math com justificação
fundacional.

### Justificação Caminho I (Variant) vs II/III

Diagnóstico P311a §3.1 + §4 fixou:

- **Caminho II rejeitado** (Eager Unicode mapping): falha em
  bindings `bb(x)` onde `x` é `MathIdent`/binding eval-time. Não
  composicional para `bb(cal(x))`.
- **Caminho III rejeitado** (Style enum extension): activa
  anti-padrão "capture sem consumer" — `StyleChain` cristalina é
  plana (DEBT-1; ADR-0040), capturar variant em chain sem
  `MathLayouter` real consumer = estado intermédio.
- **Caminho I escolhido**: paralelo a P298 (`MathOp`), composicional
  natural via nesting, funciona com bindings.

### Composição cross-variant (P311b.4 layout_node)

Regras (P311a §3.3):

1. **Variant glyph** outer-wins. `bb(cal(x))` → outer Bb prevalece.
   Walker top-down fixa `context.kind = outer.kind` ao entrar; inner
   `MathStyled` **não sobrescreve** se context já set.
2. **Bold flag** ortogonal. `bold(bb(x))` → ambos aplicados =
   "Bold Double-Struck" (kind=DoubleStruck, bold=true).
3. **Italic flag** outer-wins (`Option<bool>`). `upright(italic(x))`
   → outer Upright prevalece (italic=Some(false)).
4. **Size variant** (Script/SScript) compõe multiplicativamente.
   `script(sscript(x))` → factor 0.7 × 0.5 = 0.35.

### Match arms exhaustive defesa compilador (5 sítios paralelo P298)

| Local | Operação |
|---|---|
| `content.rs:plain_text()` | `body.plain_text()` (transparente; wrap glyph não tem texto próprio) |
| `content.rs:PartialEq` | structural (kind + bold + italic + body + cramped) |
| `content.rs:map_content()` | recurse em body; preserva kind/flags |
| `content.rs:map_text()` | terminal (paralelo cluster math) |
| `rules/introspect.rs:materialize_time` | terminal |
| `rules/introspect.rs:walk` | terminal |
| `rules/introspect/locatable.rs` | `false` |
| `compiler/layout/mod.rs` | fallthrough math |
| `rules/math/layout/mod.rs:layout_node` | `layout_styled` (P311b.4 — context passing) |

### Família `script`/`sscript` — variant separado no mesmo enum

Per P311a §3.5, opção A: variants Script/SScript ficam dentro de
`MathStyleKind` com sub-categoria size. `MathStyleKind::is_size_variant()`
distingue. Anti-padrão "misturar variant glyph + size" mitigado por
factor multiplicativo via `MathStyleKind::size_factor()`.

### ADRs novas obrigatórias (P311b.6)

- **ADR-Math-Style-Mechanism**: formaliza Caminho I; justifica
  preferência sobre II/III; documenta interacção com DEBT-1.
- **ADR-Math-Style-Composition**: formaliza 4 regras §3.3
  (outer-wins / ortogonal / multiplicativo).

### Sub-padrão Variant Math cumulativo

P296-P298 já estabeleceram pattern "variant math agregado". P311b.2
estende com **wrapper recursivo de transformação** — primeiro variant
math sem `body` semântico próprio (wrapping puro). Pattern N=4 emerge
mas formalização adiada per P273.17 §0.

---

## Estado actual cumulativo (reconciliação P258 Cenário B1)

**P258 audit empírico Fase A** (`diagnostico-model-fase-a-passo-258.md`)
confirmou **cobertura Model ~73%** (ponderado linear; +25pp face
P154A 48%). Representação base inicial deste prompt (`Empty, Text,
Space, Sequence` + comentário "Variantes futuras") está
**desactualizada vs enum real pós-M3-M9 + P199B + P252/P257**:
o enum tem **~62 variants** cumulativos.

**Decisão arquitectural P258**: representação inicial preservada
como **histórico cumulativo** (paridade pattern ADR-0080 §"refactor
aditivo"); secções subsequentes deste prompt L0 (12+ anotações
cumulativas P154B/P155/P157A-C/P159A/P247/P250/P251/P252) cobrem
materializações reais variant-por-variant. **Não reconciliação
destructiva**.

### Sumário variants Content materializadas cumulativamente

Lista amostral (ordem aproximada de introdução):

- **Foundations** (P25-P101): `Empty`, `Text`, `Sequence`,
  `Styled`.
- **Markup básico** (P-M3): `Heading`, `Raw`, `ListItem`,
  `EnumItem`, `Link`, `Outline`.
- **Math** (P36-P40 + M3-M9): `Equation`, `MathSequence`,
  `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathRoot`,
  `MathDelimited`, `MathMatrix`, `MathCases`.
- **Introspector + Numbering** (P164-P204; P182C; P199B; P464):
  `Label`, `Ref`, `SetHeadingNumbering`,
  `SetEquationNumbering`, `SetFigureNumbering`,
  `CounterDisplay`, `CounterUpdate`.
- **Figure** (P158): `Figure { body, caption, kind, numbering }`.
- **Visualize** (P25+): `Image`, `Shape`, `Transform`.
- **Grid + Table** (P82-P83; P157A-C; P227-P234): `Grid`,
  `GridHeader`, `GridFooter`, `GridCell`, `Table`, `TableCell`,
  `TableHeader`, `TableFooter`.
- **Bibliography + Cite** (P159A-G; Bloco B Model paridade
  manual): `Bibliography`, `Cite`.
- **Layout primitives** (P81-P96; P156C-L; P217-P252): `SetPage`,
  `Align`, `Place`, `Pad`, `Hide`, `HSpace`, `VSpace`,
  `Pagebreak`, `Colbreak`.
- **Markup compositivo** (P154B; P155): `Terms`, `TermItem`,
  `Quote`, `Divider`.
- **Block + Boxed** (P156G/H + P231/P242/P247/P248/P250/P252):
  `Block { body, width, height, inset, breakable, outset,
  radius, clip, fill, stroke, spacing, above, below, sticky }`
  (14 fields), `Boxed { body, width, height, inset, baseline,
  outset, radius, clip, fill, stroke }` (10 fields).
- **Stack + Repeat + Columns** (P156I-J; P217-P220): `Stack`,
  `Repeat`, `Columns`.

**~62 variants cumulativos total** (audit P258 Bloco 1).

### Variants PENDENTES pós-P258 (ausentes empíricos confirmados)

- **`Content::Footnote`** — Layout desbloqueio P156C preservado;
  variant Content + stdlib func não materializados. Candidata
  refino P-Footnote-N futuro (M; +10-15 tests).
- **`Content::Document`**, **`Content::Title`**,
  **`Content::Asset`** — Fase 3 condicional ADR-0060 §"Fase 3
  condicional"; sem prioridade designada; scope-out formal
  preservado.

### `parcial` pendentes pós-P258

- **link**, **list**, **enum**, **par** — refinos
  atributos vanilla (`marker`/`tight`/`indent`/`leading`/etc.)
  preservados como scope-out informal P258 (cobertura útil
  via paridade observable básica preservada).

### Bloco B hayagriva — scope-out implícito P258

Bibliography + Cite cumpridas **cumulativamente via paridade
manual P159A-G** (`bib_entry.rs` 413 LoC; 16 fields universais
paridade `hayagriva::Entry` sem dependência crate real).
ADR-0062 PROPOSTO preservada; promoção a IMPLEMENTADO diferida
até consumer real exigir CSL styling completo.

### Estado agregado P258

| Estado | P154A | Audit P258 | Δ |
|--------|-------|------------|---|
| implementado | 4 | 4 | 0 |
| implementado⁺ | 4 | 10 | **+6** |
| parcial | 5 | 4 | -1 |
| ausente | 10 | 4 (footnote, document, asset, title) | **-6** |

**Cobertura ponderada linear**: P154A 48% → Audit P258 **~73%**
(Δ +25pp).

**Cenário Fase B**: ☑ **B1 (≥75% — fecho conceptual Model)**
— Bloco A massivamente materializado cumulativamente; Bloco B
scope-out implícito documentado; Fase 3 + footnote refinos
futuros candidatos.

---

## P844 (achado #47 de P831) — `get_field` de `Metadata`

- `Content::get_field` ganhou braço `(Content::Metadata(e), "value")` — paridade vanilla `MetadataElem.value`; suporta `query(<meta>).first().value` após o achado #47 (`query()` devolve content).

## P863 — `Content::Par` (parágrafo como contentor)

`Content::Par { body: Box<Content> }` é um contentor sintético introduzido para
suportar `#show par: <transformação>` como regra de elemento. No vanilla os
parágrafos são `ParElem` criados durante a realização; no cristalino são
implícitos no layout até P863.

- **Construção**: `Content::par(body)` normaliza `body` via `Content::sequence`.
- **Semântica**: transparente para layout (`layout_content` delega no `body`);
  plain_text, is_empty, map_content/map_text, materialize_time e walk recursam
  no `body`; `get_field` expõe `"body"` para show rules (`it.body`).
- **Paragraph realization**: uma passagem `realize_paragraphs` executada antes de
  `apply_show_rules` agrupa, em cada contexto de fluxo (`Sequence`), o conteúdo
  entre `Content::Parbreak`s em nós `Content::Par`. Sub-árvores matemáticas
  (`Equation`, `MathSequence` e família `Math*`) são preservadas sem wrapping.
- **Show rule**: `NodeKind::Par` casa `Content::Par`; o identificador `par` em
  `#show par: …` resolve para esse `NodeKind`. A forma legada
  `#show par: set block(spacing: ..)` mantém o warning sem registar regra.

## Variant `Content::MathLimitsOverride` — Passo 992 (`limits()`/`scripts()`)

`MathLimitsOverride(Arc<MathLimitsOverrideElem>)` — família math (Modelo D,
mesmo padrão do lote P317/P772y). Consolida os dois elementos vanilla
`ScriptsElem`/`LimitsElem` num só (ADR-0107): `scripts(body)` →
`Content::math_limits_override(body, false, true, false)` (`inline` ignorado);
`limits(body, inline:)` → `Content::math_limits_override(body, true,
inline, inline_explicit)`. Ver `entities/elements/math_limits_override.md`
para o struct e
o contrato do trait `Element`.

P1425 amplia a assinatura pública do constructor com
`inline_explicit: bool`. O hub somente transporta o bit; não decide presença
por valor, não muda enum, layout ou representação. A alteração foi confirmada
pelo dono no gate ADR-0127 do passo em 2026-09-16.

Despacho no hub, mesmo padrão de `MathClassOverride` (P772y): Debug/Display,
constructor, `plain_text`, `PartialEq`, `map_content` (recursivo),
`map_text` (terminal). **Diferença**: `apply_math_default`
(`compiler/math/layout/_comum.md` §P992) recursa no `body` — necessário para
bases de 1 letra (`limits(A)`) receberem o itálico por defeito, gap que
`MathClassOverride` tem (fora de escopo aqui, pré-existente).

## P1140.4-C — delegação de campos de Equation

### Medição antes da decisão

Embora `eval/bindings/field_access::content_field` já reconheça Equation, o
caminho normal de `it.block` chama `Content::get_field`
(`content.rs:3181-3208`). O probe E2E continuou a falhar porque esse match não
possui braço Equation.

### Decisão

`Content::get_field` delega `Content::Equation(e)` a `e.get_field`. A unidade
`EquationElem` é dona dos campos de dado `block` e `body`; nenhum campo de
style é inventado no enum. Esta delegação mantém o hub magro e permite à
callback de suplemento observar a equação recebida.
## P1140.24 — deltas de running matter

`Content::SetPage` preserva omissão e os valores de `number-align`, `header`,
`header-ascent`, `footer` e `footer-descent`. `PageRunElem` transporta os mesmos
deltas e restaura-os lexicalmente, conforme `entities/page_running.md`.
## P1140.25 — transporte de supplement e forma de referência

`Content::SetPage` e `PageRunElem` preservam o delta omitido/auto/none/content.
`RefElem` preserva `RefForm::{Normal, Page}` e a presença do supplement
explícito, conforme `entities/page_supplement.md`.

## P1157 — delta tipado de numbering em `SetPage`

### Medição antes da decisão

Alternância lexical callback/pattern e `none` exige distinguir omissão de
desativação e preservar Func até o fixpoint.

### Decisão

`Content::SetPage.numbering` passa a
`Option<Option<entities::numbering::Numbering>>`. Exterior `None` preserva;
`Some(None)` desativa; `Some(Some(_))` instala. `Content` não aplica callback.
Clone, igualdade, hash, repr e walks tratam Numbering como dado fechado; a
morfologia pública de `SetPage`/`PageRun` continua content.

## P1292 — identidades públicas de math e `place.flush` (GATE ADR-0127)

### Medição anterior à decisão

`Content::MathCancel` e seu runtime completo já estão materializados por
P1291. O enum ainda não possui identidade matemática para underline/vetor nem
sentinela de flush; `vec` é degradado a `MathMatrix`. No vanilla ratificado,
`CancelElem`, `UnderlineElem` matemático, `VecElem` e `FlushElem` são
identidades distintas; `math.underline == underline` devolve `false`, e
`repr(place.flush())` devolve `flush()`.

### Decisão pública

Preservar `Content::MathCancel(Arc<MathCancelElem>)` e seu transcript sem criar
segundo elemento/runtime, completando somente metadado de presença necessário
à exposição morfológica. Adicionar:

```rust
Content::MathUnderline(Arc<MathUnderlineElem>)
Content::MathVec(Arc<MathVecElem>)
Content::Flush(Arc<FlushElem>)
```

Construtores públicos:

- `math_cancel(body)` permanece compatível e aplica defaults omitidos; o
  construtor completo transporta valores, span e presença sem reimplementar
  callback.
- `math_underline(body)` cria exclusivamente `MathUnderlineElem`, nunca o
  `UnderlineElem` textual.
- `math_vec(children, delim, align, gap, explicit)` preserva filhos variádicos,
  os três parâmetros e a presença dos named, nunca converte em `MathMatrixElem`.
- `flush()` cria exclusivamente a sentinela zero-field; não normaliza para
  `Empty`.

Nos matches exaustivos, os três math são estruturais não-locatáveis; `Flush`
é sentinela estrutural também não-locatável e não vazia.
`plain_text` de cancel/underline delega ao body; o vetor concatena o plain text
dos filhos na ordem. `map_content` recursa nos bodies/filhos preservando todos
os demais campos; `map_text` é terminal, conforme os demais nós math
estruturais. Igualdade e hash são estruturais sobre campos da linguagem; o
`span` interno de `MathCancelElem` é preservado pelos walkers e ignorado por
igualdade/hash. `materialize_time`, `walk`,
layout genérico e defaults matemáticos devem ganhar braços explícitos; `Flush`
tem walkers terminais e `plain_text` vazio. Nenhum wildcard pode esconder
variante nova.

Prompts proprietários relacionados: `entities/elements/math_cancel.md`,
`entities/elements/math_underline.md`, `entities/elements/math_vec.md`,
`entities/elements/flush.md`,
`entities/elements/_comum.md`, `compiler/math/layout/cancel.md`,
`compiler/math/layout/underline.md`, `compiler/math/layout/vec.md` e
`compiler/math/layout/_comum.md`, além de `compiler/layout/flush.md`.

Esta alteração cria variantes/construtores públicos; permanece proibida até
a confirmação humana ADR-0127. Os seis prompts de novos consumers (entidade/layout
para underline, vec e flush) ficam
deliberadamente sem consumer e sem metadado canônico de hash durante esta pausa.

## P1344 — projeção owner-local do carrier de `CounterUpdate`

### Medição anterior à decisão

O blocker P1343 R3 refutou alojar `impl Content` dentro de `impl Func`:
`entities/func.rs` aceita ali apenas associated items de `Func`. O consumer
`entities/content.rs` já é o owner 1:1 de `Content` e contém os inherent impls
reais da unidade na mesma camada L1.

### Decisão vigente

Somente sob `cfg(p1339_observation)`, `Content` pode expor ao crate um helper
owner-local que examina a variante real `Content::CounterUpdate`, consulta o
carrier da ação já transportada e encaminha a observação H11. O helper:

- recebe por borrow o `Content` e os valores reais já calculados no walk;
- não cria, anexa, muta, reencadeia ou procura carrier por nome, output,
  igualdade ou endereço;
- não adiciona variante, campo, wrapper, trait, macro ou reexport;
- não participa de Eq, Hash, Debug, repr, plain-text, walkers normais ou
  serialização;
- desaparece integralmente sem `p1339_observation`.

O armazenamento e o writer continuam pertencendo ao carrier privado de
`entities/func.md`; esta unidade apenas faz a projeção read-only necessária ao
seu próprio enum. Isso não é API pública nem mudança de comportamento por
defeito conforme ADR-0127.

# P1402 — `Content::OutlineEntry`

Medição anterior e contrato público: `compiler/stdlib/structural/outline.md`
§P1402. O enum fechado recebe `OutlineEntry(Arc<OutlineEntryElem>)`, com despacho
estático nos hubs. A lógica da unidade fica em `entities/elements/outline_entry`;
o hub só reembrulha, compara e delega. Não usar `Content::Dynamic`.

# P1403 — construção interna da entry realizada

Além do constructor público P1402, o hub oferece constructor interno que
recebe prefixo semântico opcional e Location causal. Ele apenas encaminha à
unidade `OutlineEntryElem`; não calcula contador, página ou numbering.

# P1404 — caixa fracionária de `inner`

O hub oferece constructor interno de `BoxedElem` com
`width_fraction: Some(fr)` e defaults neutros dos demais cosméticos. Ele só
embrulha body e carrier; parsing público e distribuição de linha pertencem a
seus owners.

# P1411 — travessia do corpo realizado de `Outline`

## Medição anterior à decisão

O contrato geral de `map_content` já classifica `Outline` como container, mas
o consumer vigente o mantém no grupo terminal. A tentativa P1411 demonstrou
que delegar ao walker da unidade não basta enquanto a expansão ocorre antes da
realização; também demonstrou que a delegação é necessária para a expansão
pós-realização descobrir os `ContextBlock` no `realized_body`.

## Obrigação

`Content::map_content` delega `Content::Outline` a
`OutlineElem::map_content`, que preserva os campos não-conteúdo e percorre
`title` e `realized_body`. O hub não interpreta callbacks nem resolve contexto;
a nova posição da expansão pertence ao owner L3 `infra/pipeline.md` P1411.

Esta correção interna do walker integra a mudança de fase P1411 confirmada em
2026-09-15 e não adiciona API pública.

# P1424 — `Content::HtmlFrame` (APROVADO NO GATE ADR-0127 EM 2026-09-16)

### Medição anterior à decisão

`Content::HtmlElem` representa uma tag e portanto não pode conservar a
morfologia da primitiva `html.frame`, que vira SVG em HTML e desaparece como
wrapper no target paginado.

### Contrato público proposto

Adicionar `Content::HtmlFrame(Arc<HtmlFrameElem>)`. O enum fechado mantém
dispatch estático e braços explícitos em repr, nome, plain-text, igualdade,
hash quando aplicável, walkers e transformações. É contentor: `plain_text` e
walkers descem em `body`; `map_content`/`map_text` reconstruem o carrier sem
apagar a variante; `func()` identifica `frame`; `repr` expõe a morfologia
`frame(body: ...)`, nunca `html.elem("frame")`.

Layout paginado e pipeline HTML são consumers diferentes da mesma variante.
Nenhum fallback pode convertê-la em Empty, HtmlElem ou body antes do consumer
correto. A variante pública foi confirmada pelo dono em 2026-09-16 e deve ser
materializada somente no escopo de P1424.

# P1427 — construção completa de `MathFrac` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O constructor atual fixa somente a fração vertical com barra e apaga estilo,
explicitude e a proveniência de parênteses que o AST já consegue medir.

### Contrato público proposto

Manter wrappers verticais compatíveis para callsites existentes e adicionar
um constructor completo que recebe numerador, denominador, estilo opcional,
os dois bits de deparenthesization e `line`. Walkers, plain-text, igualdade,
hash e transformações preservam todos os campos causais. O wrapper sem barra
de `binom` continua `line=false`, sem estilo público e sem herdar set rule de
`math.frac`.

# P1429 — construção completa de `MathAccent` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O constructor P296 aceita `Content` arbitrário no acento e só transporta os
dois filhos. Isso contradiz a identidade pública normalizada e apaga os dois
campos opcionais observáveis.

### Contrato público proposto

Adicionar constructor completo que recebe `base`, `accent: char`, `size`,
`dotless` e os bits de presença. Manter wrapper interno default para callsites
que já possuem codepoint canônico, sem reintroduzir `Content` arbitrário como
acento. Enum, walkers, igualdade, hash, debug, `plain_text` e todas as
reconstruções preservam o carrier P1429. A fronteira de avaliação é a única
dona de casts e normalização de entrada.

# P1430 — construção completa de `MathCases` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O constructor atual recebe rows já separadas, par de chars obrigatório,
booleano e `Option<Length>`. Ele não consegue preservar children públicos,
`none` por lado, gap relativo nem a presença dos três named.

### Contrato público proposto

Adicionar constructor completo que recebe children, par de `Option<char>`,
`reverse`, `Rel<Length>` e `MathCasesExplicit { delim, reverse, gap }`. Manter
um wrapper compatível para callsites internos antigos, reconstruindo os
boundaries sem alterar sua geometria. Enum, walkers, igualdade, hash, debug,
`plain_text` e todas as reconstruções preservam o carrier P1430; casts e
normalização pertencem exclusivamente à fronteira de avaliação.

# P1431 — construção completa de `MathMatrix` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O constructor atual transporta rows, par obrigatório de chars, três
`Option<Length>` e um único `Option<usize>`. Não representa delimitador nulo
por lado, alinhamento, relative length, augment completo nem presença causal.

### Contrato público proposto

Adicionar constructor completo que recebe rows retangulares, par de
`Option<char>`, `HAlign`, augment tipado opcional, `Rel<Length>` por eixo e
`MathMatrixExplicit { delim, align, augment, row_gap, column_gap }`. Manter
wrapper interno compatível para callers antigos, elevando o seu subconjunto
sem criar nova superfície de linguagem. Enum, igualdade, hash, debug, walkers,
`plain_text` e todas as reconstruções preservam o carrier P1431. Casts,
normalização, shorthand `gap` e retangularização pertencem à fronteira de
avaliação; layout deriva cascata sem mutar o elemento.

# P1436 — variante pública `Content::MathLr` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

A sintaxe `lr` é hoje reduzida ao body ou a `MathDelimited`, por isso o enum
não conserva identidade, tamanho relativo nem presença. No vanilla, `LrElem`
continua observável mesmo sem delimitadores e com body vazio.

### Contrato público proposto

Adicionar `Content::MathLr(Arc<MathLrElem>)` e constructor completo que recebe
`body`, `Rel<Length>` e `size_explicit`. O match fechado mantém braços
estáticos para nome `lr`, debug, igualdade, hash, plain-text, filhos e walkers.
`map_content` desce somente no body; `map_text` conserva a variante segundo a
política estrutural math; reconstruções não podem converter `MathLr` em
`MathDelimited` nem fabricar presença por cascata. O carrier mecânico
`MathDelimited` continua variante separada.

# P1437 — variante pública `Content::MathMid` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

A forma bare degrada `mid(...)` a texto e a forma qualificada inexiste; o enum
não conserva identidade, body nem ligação ao `lr` mais próximo. O vanilla
preserva `mid` inclusive aninhado e com body vazio.

### Contrato público proposto

Adicionar `Content::MathMid(Arc<MathMidElem>)` e constructor por body. O match
fechado mantém nome, debug, igualdade, hash, plain-text, filhos e walkers.
`map_content` desce só no body; `map_text` conserva a variante. Não reduzir
ao body, converter em delimitador ou assar stretch.

# P1439 — variante pública `Content::MathOverbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

A redução atual para `MathUnderover` perde identidade e presença explícita da
anotação, embora a geometria mecânica já exista.

### Contrato público proposto

Adicionar `Content::MathOverbrace(Arc<MathOverbraceElem>)`. O match fechado
preserva nome, debug, igualdade, hash, plain-text e filhos. `map_content`
transforma body e anotação preservando `annotation_explicit`; `map_text`
mantém a variante. Não inferir `overbrace` a partir de U+23DE nem converter o
carrier público permanentemente em `MathUnderover`.

# P1440 — variante pública `Content::MathOverbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A redução atual para `MathUnderover` perde identidade e presença explícita da
anotação, embora a geometria mecânica já exista.

### Contrato público proposto

Adicionar `Content::MathOverbracket(Arc<MathOverbracketElem>)`. O match
fechado preserva nome, debug, igualdade, hash, plain-text e filhos.
`map_content` transforma body e anotação preservando `annotation_explicit`;
`map_text` mantém a variante. Não inferir `overbracket` a partir de U+23B4,
nem confundi-lo com `MathOverbrace` ou com o carrier `MathUnderover`.

## P1441 — variante pública `MathOverline` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

`Content::Overline` é decoração textual e transporta campos que não existem
no elemento matemático vanilla. O enum fechado ainda não representa a
identidade matemática body-only.

### Decisão proposta

Adicionar `Content::MathOverline(Arc<MathOverlineElem>)`. O match fechado
preserva nome, debug, igualdade, hash, plain-text e body. `map_content`
transforma o body; `map_text` mantém a variante. Não equiparar a
`MathUnderline`, `Overline` textual ou regra física de layout.

## P1442 — variante pública `MathOverparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A morfologia bare atual não produz um carrier público e a redução para
`MathUnderover` perderia identidade e presença explícita da annotation.

### Decisão proposta

Adicionar `Content::MathOverparen(Arc<MathOverparenElem>)`. O match fechado
preserva nome, debug, igualdade, hash, plain-text e filhos. `map_content`
transforma body e annotation preservando `annotation_explicit`; `map_text`
mantém a variante. Não inferir `overparen` de U+23DC nem confundi-lo com os
outros spreaders ou com `MathUnderover`.

## P1443 — variante pública `MathOvershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A morfologia bare atual não produz um carrier público e a redução para
`MathUnderover` perderia identidade e presença explícita da annotation.

### Decisão proposta

Adicionar `Content::MathOvershell(Arc<MathOvershellElem>)`. O match fechado
preserva nome, debug, igualdade, hash, plain-text e filhos. `map_content`
transforma body e annotation preservando `annotation_explicit`; `map_text`
mantém a variante. Não inferir `overshell` de U+23E0 nem confundi-lo com os
outros spreaders ou com `MathUnderover`.

## P1444 — variante pública `MathPrimes` (MATERIALIZADO)

### Medição anterior à decisão

O enum fechado não representa o elemento público `primes`; texto Unicode não
conserva count, fields, função ou igualdade nominal.

### Decisão proposta

Adicionar `Content::MathPrimes(Arc<MathPrimesElem>)` e
`Content::math_primes(count)`. Nome/debug/igualdade/hash/plain-text/get_field e
walkers preservam a identidade e o count. A variante não tem filhos e não é
inferida de U+2032/U+2033/U+2034/U+2057 ou de repetição textual.
## P1446 — variante pública `MathStretch` (PROPOSTA; gate ADR-0127)

Adicionar `Content::MathStretch(Arc<MathStretchElem>)` e constructor canônico
com `body`, `Rel<Length>` e `size_explicit`. Integrar exaustivamente Debug,
nome, igualdade, hash, `plain_text`, field dispatch e walkers de conteúdo.
`map_content` recursa pelo owner; `map_text` preserva o carrier. Nunca
normalizar `MathStretch` para texto, `MathLr` ou body nu.

## P1447 — delegação de campos de `MathLr`

`Content::get_field` delega `Content::MathLr` ao seu elemento proprietário,
como já faz para carriers tipados equivalentes. A delegação não assenta
defaults, não altera nome/igualdade/walkers e não afeta `MathDelimited`.

# P1448 — variante pública `Content::MathUnderbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A redução bare atual para `MathUnderover` perde identidade, fields e presença
explícita da annotation, embora a geometria mecânica já exista.

### Contrato público proposto

Adicionar `Content::MathUnderbrace(Arc<MathUnderbraceElem>)`. O match fechado
preserva nome, debug, igualdade, hash, plain-text e filhos. `map_content`
transforma body e annotation preservando `annotation_explicit`; `map_text`
mantém a variante. Não inferir `underbrace` a partir de U+23DF nem converter o
carrier público permanentemente em `MathUnderover`.

# P1449 — variante pública `Content::MathUnderbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A redução bare atual para `MathUnderover` perde identidade, fields e presença
explícita da annotation, embora a geometria mecânica já exista.

### Contrato público proposto

Adicionar `Content::MathUnderbracket(Arc<MathUnderbracketElem>)`. O match
fechado preserva nome, debug, igualdade, hash, plain-text e filhos.
`map_content` transforma body e annotation preservando presença; `map_text`
mantém a variante. Não inferir `underbracket` de U+23B5 nem converter o carrier
público permanentemente em `MathUnderover`.

# P1450 — variante pública `Content::MathUnderparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A rota bare atual perde identidade, fields e presença, embora a geometria
mecânica comum já exista.

### Contrato público proposto

Adicionar `Content::MathUnderparen(Arc<MathUnderparenElem>)`. Nome, debug,
igualdade, hash, plain-text, filhos e reconstruções preservam identidade e
presença; nunca inferir a variante de U+23DD.

# P1451 — variante pública `Content::MathUndershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A rota bare atual perde identidade, fields e presença, embora a geometria
mecânica comum já exista.

### Contrato público proposto

Adicionar `Content::MathUndershell(Arc<MathUndershellElem>)`. Nome, debug,
igualdade, hash, plain-text, filhos e reconstruções preservam identidade e
presença; nunca inferir a variante de U+23E1.

## P1481 — compatibilidade do constructor de imagem (IMPLEMENTADO; confirmado em 2026-09-23)

Medição: o constructor público `Content::image(path, data, width,
height, fit)` em `content.rs:2142-2157` constrói `ImageElem`
diretamente. A materialização P1481 acrescenta `ImageElem.alt` para
`image(alt:)`; não há motivo para quebrar essa assinatura pública.

Contrato: manter `Content::image` com os cinco argumentos existentes e
inicializar `alt: None`. O constructor da stdlib pode formar
`Content::Image(Arc<ImageElem>)` com `Some(alt)` sem modificar esta
API. Igualdade e demais operações observam o campo segundo o owner
`entities/elements/image.md`. O campo público foi confirmado em 2026-09-23
sob ADR-0127.

## P1485 — carriers puros de arquivo em bundle (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working
tree P1484 não commitada, `01_core/src/entities/content.rs:707-720,
2974-2986` conserva `Document` só com metadata e `Asset` com
`path,kind`; `repr` de asset omite data. O vanilla pinado,
`typst-bundle/src/format.rs:194-240`, transporta path e `Bytes`
e `typst-library/src/model/document.rs:147-239` transporta path,
body, formato e metadata. `repr(asset("x","data"))` mede path
absoluto `"/x"` e `bytes(4)`. Payload, path e body são semântica/
morfologia; enum Rust, Arc e localização de campos são mecânica.

### Decisão proprietária

Substituir o carrier público `Content::Asset { path, kind }` por
asset de bundle com path normalizado e payload binário imutável.
`kind` inferido deixa de existir como atributo de linguagem.
`repr`, fields e igualdade de linguagem observam path e data
segundo o vanilla; `plain_text` continua vazio. Clone estrutural
não copia bytes desnecessariamente; nenhum acesso a filesystem em L1.
O carrier de `Document` passa a representar arquivo de bundle com
path, body, formato opcional e metadata; o uso de `#set document`
continua metadado de estilo, não `Content::Document` sem path.
Traversal/transformação recursam no body para preservar semântica
e morfologia; asset permanece leaf. Fora de bundle, a validação
de target impede emissão, não apaga conteúdo silenciosamente.

A representação Rust pode divergir do vanilla, mas a assinatura
pública, variante e campos que consumidores externos possam ler
mudam; requerem confirmação ADR-0127. O owner
`00_nucleo/prompts/compiler/stdlib/structural/document.md` valida argumentos e
constrói os carriers; `00_nucleo/prompts/infra/pipeline.md` consome-os e nunca os
cria por inferência de extensão de arquivo-fonte.


## 00_nucleo/prompts/entities/content_hash.md

# Prompt L0 — `entities/content_hash`
Hash do Código: 59cd7e8b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/content_hash.rs`
**Criado em**: 2026-04-30 (P162 sub-passo .B — resolve pendência herdada de P161)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## P1494 — hash público neutro à origem de newline (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, com working tree
limpo na medição P1494, content_hash.rs:23-29 serializa Content por Debug
antes de formar o u128. content.rs:1255-1279 já implementa Debug manual,
não derive, e escreve Content::Space como "space". A variante nova ainda
não existia nesse baseline. O diagnóstico P1494 registra que as duas
origens são iguais no operador da linguagem e têm a mesma repr; o L0
ratificado de entities/content.md requer também PartialEq cruzado e
Debug idêntico após adicionar SpaceFromNewline.

### Decisão ratificada

hash_content(Content::Space) e hash_content(Content::SpaceFromNewline)
devem devolver o mesmo u128, inclusive quando as duas formas ocupam a
mesma posição em sequências estruturalmente iguais. O Debug manual de
Content representa ambas por "space"; hash_content continua a usar essa
serialização, sem contrato público novo ou mudança de algoritmo. A
proveniência causal pré-normalização não pode depender deste hash: o
comparador observacional específico do owner eval distingue as origens.
Testar a igualdade dos hashes diretamente nas duas formas e em sequências.
Uma eventual troca por hash recursivo deve atribuir a ambas o mesmo tag
canônico; std::mem::discriminant bruto por variante quebraria o invariante.

## Contexto

`hash_content` é a função pura que produz um identificador `u128` determinístico para qualquer `Content`. P162 materializa-a para ser consumida por `extract_payload` (preenche `ElementPayload::Heading.body_hash`) e por `walk` (preenche `Tag::End(loc, content_hash)`).

Vanilla deriva `Hash` em `Content` via proc-macro `#[elem]` + sub-types. Cristalino não usa proc-macro; `Content` é enum fechado mas contém `f64` (em `Pt`/`Length`/`Ratio`) que não implementa `Hash` automaticamente. Função manual é a alternativa.

---

## Restrições Estruturais

- Camada **L1**: função pura, sem I/O, sem estado global.
- Determinismo dentro da mesma versão do compilador: dois `Content` que sejam `==` produzem o mesmo `u128`.
- Sem dependência em ordem de iteração de `HashMap` (`Content` não usa `HashMap` directamente em variantes — fan-out apenas para tipos que também não usam).
- Floats são serializados via `Debug` (que mostra a representação decimal estável); valores f64 com bit-pattern diferente mas decimal igual produzem hashes iguais — comportamento aceite.
- Sem dependência externa nova (não autorizar `siphasher` em L1; usar `std::collections::hash_map::DefaultHasher` que existe em L1 sem precisar de allowed-list).

## Implementação escolhida (P162)

Forma minimalista: `format!("{:?}", content)` produz uma string estável recursiva (Debug manual de Content preserva a equivalência de variantes iguais). Hashar a string duas vezes com sementes distintas e concatenar os u64 resultantes para obter u128.

```rust
fn hash_content(content: &Content) -> u128 {
    let serialized = format!("{:?}", content);
    let lo = hash_with_seed(&serialized, 0);
    let hi = hash_with_seed(&serialized, 1);
    ((hi as u128) << 64) | (lo as u128)
}
```

**Fragilidade declarada**: a estabilidade do hash depende da estabilidade do output de `Debug` manual em `Content`. Mudanças cosméticas no Debug formatter de qualquer field-type (e.g. mudança de formatação f64 em Rust stdlib) podem alterar o hash. Aceitável para M1 (tags são descartadas). Quando M2/M3 começarem a consumir tags entre iterações do fixpoint, esta função pode precisar de ser substituída por hash recursivo manual sobre cada variante de Content. Decisão diferida — sem ADR nova.

Refino futuro candidato: hash recursivo manual com tag canônico por variante e hash de cada field directamente (com `f64::to_bits()` para floats). Space e SpaceFromNewline partilham o tag; demais tags podem derivar do discriminante quando isso respeitar a igualdade. Aceitável quando consumidor real exigir.

---

## Interface pública

```rust
use crate::entities::content::Content;

pub fn hash_content(content: &Content) -> u128;
```

Sem trait, sem método em `Content` (manter Content sem imports adicionais). Função top-level no módulo.

---

## Semântica

- `hash_content(c)` é determinístico dentro da mesma execução e dentro da mesma versão de Rust+std.
- Dois `Content` que são `==` produzem o mesmo `u128` (porque o Debug manual preserva a mesma serialização para estruturas iguais, incluindo as duas origens de espaço).
- Dois `Content` distintos produzem `u128` distintos com altíssima probabilidade (colisões teóricas mas extremamente improváveis em qualquer prática realista — DefaultHasher é SipHash 1-3 com 64 bits cada).
- Função é pura — sem leitura de ambiente, sem alocação além do `format!`.

---

## Invariantes

- `hash_content(&c) == hash_content(&c.clone())` para qualquer `c: Content`.
- Determinismo entre runs (verificado por test "100 chamadas, todos iguais").
- Não há "hash zero" reservado; `0u128` é resultado válido se a entrada produzir esse Debug.

---

## Tests obrigatórios (sub-passo .B P162)

- **Igualdade**: `Content::text("hello") == Content::text("hello")` produz mesmo hash.
- **Distinguibilidade**: 5 Contents construídos manualmente produzem 5 hashes distintos.
- **Determinismo**: 100 chamadas sobre o mesmo Content produzem o mesmo u128.

---

## Consumers actuais

Nenhum no momento da criação. Imediatamente consumido em P162 sub-passos .D e .E.

## Consumers planeados

- `rules/introspect/extract_payload.rs` (P162 .D) — popula `ElementPayload::Heading.body_hash` via `hash_content(body)`.
- `rules/introspect.rs` walk (P162 .E) — popula `Tag::End(loc, content_hash)` via `hash_content(content)`.
- `Introspector` em M3 — pode usar para detectar mudanças cross-iteration.

---

## Sobre paridade

Vanilla obtém hash via derive `#[derive(Hash)]` automático em `Content` (proc-macro `#[elem]` injecta o derive). Hash output é u64. Cristalino não tem proc-macro nem hash automático; produz u128 (paridade com `Tag::End(Location, u128)` vanilla via design choice).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) — vanilla `Tag::End(Location, u128)` linha 13 confirma o tipo de output.

---

## Resultado Esperado

- `01_core/src/entities/content_hash.rs` — função + 2 helpers + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P162 sub-passo .B: resolve pendência P161 — função de hash determinística sobre Content | `content_hash.rs`, `content_hash.md` |


## 00_nucleo/prompts/entities/syntax-node/errors.md

# Prompt L0 — erros sintáticos e falha de numeração
Hash do Código: 708859fd

**Camada:** L1
**Consumer exclusivo:** `01_core/src/entities/syntax_node/errors.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:08:05Z: fonte original em `01_core/src/entities/syntax_node.rs`, 1.138 linhas. Declara SyntaxNode/NodeKind (:26–38), representação e edição (:39–609), SyntaxErrorKind/SyntaxError (:611–670), LinkedNode/Side/LinkedChildren (:672–989), numbering (:991–1004), e 14 testes (:1006–1138). Hash L0 original 58836fbf e hash código 66771d34 confirmados antes da decisão. Inventário exato e SHA-256 da fonte em `00_nucleo/diagnosticos/syntax-node-atomization.md`.

## Classificação

Recorte de organização interna, ADR-0037/0109/0127 contínuo: não altera assinatura, campo público, defaults, fase ou compatibilidade. Sem nova dependência, I/O ou estado global. Paridade da linguagem não está sendo medida; preservar texto, categorias, hints, árvore e navegação é aceitação, enquanto disposição física e hashes são controles mecânicos da refatoração.

## Contrato proprietário

SyntaxErrorKind público preserva Other(default), InvalidHexNumber, InvalidUnicodeCodepoint, NoTextWithinStars e NoTextWithinUnderscores. As duas últimas categorias continuam sinalizadas para warnings pelo eval conforme P786a; não classificar por texto nem mudar severidade aqui. SyntaxError conserva campos públicos span, kind, message e hints, construtores new/with_kind, spans detached e hints vazios. spanless_eq compara kind/message/hints ignorando span; apenas sua visibilidade interna passa a pub(super) para consumo pela representação irmã.

NumberingResult permanece Result<(), Unnumberable> crate-private; Unnumberable mantém derivados, Display literal cannot number within this interval e std::error::Error. Não é adicionado tipo/variant/método público. O carrier ErrorNode, árvore e COW são exclusivos de `00_nucleo/prompts/entities/syntax-node/representation.md`.

## Aceitação

Testes locais cfg(test) verificam os cinco kinds com defaults de span/hints,
igualdade sem spans sensível a hints, e Display da falha de numeração.

Conservar declarações/impls e todas as mensagens; campos importáveis nos caminhos anteriores pelo reexport da fachada `00_nucleo/prompts/entities/syntax-node.md`. Categorias, hints, spans e spanless_eq verificados pelos controles da fachada; não mudar lexer/parser/eval.


## 00_nucleo/prompts/entities/syntax-node/linked.md

# Prompt L0 — navegação LinkedNode
Hash do Código: c4d05cae

**Camada:** L1
**Consumer exclusivo:** `01_core/src/entities/syntax_node/linked.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:08:05Z: fonte original em `01_core/src/entities/syntax_node.rs`, 1.138 linhas. Declara SyntaxNode/NodeKind (:26–38), representação e edição (:39–609), SyntaxErrorKind/SyntaxError (:611–670), LinkedNode/Side/LinkedChildren (:672–989), numbering (:991–1004), e 14 testes (:1006–1138). Hash L0 original 58836fbf e hash código 66771d34 confirmados antes da decisão. Inventário exato e SHA-256 da fonte em `00_nucleo/diagnosticos/syntax-node-atomization.md`.

## Classificação

Recorte de organização interna, ADR-0037/0109/0127 contínuo: não altera assinatura, campo público, defaults, fase ou compatibilidade. Sem nova dependência, I/O ou estado global. Paridade da linguagem não está sendo medida; preservar texto, categorias, hints, árvore e navegação é aceitação, enquanto disposição física e hashes são controles mecânicos da refatoração.

## Contrato proprietário

Único dono de LinkedNode, Side e LinkedChildren e de todos os impls de navegação. LinkedNode conserva referência ao SyntaxNode, parent Option<Rc<Self>>, index e offset privados; sem Arc no parent, sem Send artificial, sem estado global. Módulo descendente da representação `00_nucleo/prompts/entities/syntax-node/representation.md`, não irmão, para acesso privado de find ao NodeKind/InnerNode sem alargar visibilidades. Todos os tipos públicos são encaminhados até entities::syntax_node pela fachada `00_nucleo/prompts/entities/syntax-node.md`.

## Invariantes e aceitação

Teste local cfg(test) cobre travessia de folhas entre pais aninhados,
saltando trivia e conservando offsets UTF-8 e ranges em bytes.

Offsets/ranges em bytes, não caracteres; children preserva indices, parent e fronteiras front/back, inclusive intercalando next/next_back. Iterator, DoubleEndedIterator e ExactSizeIterator e size_hint mantidos. Irmãos comuns pulam trivia, variantes with_trivia não; leaf_at mantém exatamente os lados Before/After nas bordas, fins e folhas vazias. Não corrigir comportamentos legados incidentalmente. find preserva poda por numeração e compara Span completo. Manter get/Deref/Debug, parent/parent_kind, siblings/kinds, leaf_at, prev/next_leaf e leftmost/rightmost_leaf com corpos idênticos. Testes da fachada cobrem UTF-8, trivia, cursor nas bordas, interleaving e find em árvore numerada; source e AST mantêm imports antigos.


## 00_nucleo/prompts/entities/syntax-node/representation.md

# Prompt L0 — representação da CST
Hash do Código: a91e664b

**Camada:** L1
**Consumer exclusivo:** `01_core/src/entities/syntax_node/representation.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:08:05Z: fonte original em `01_core/src/entities/syntax_node.rs`, 1.138 linhas. Declara SyntaxNode/NodeKind (:26–38), representação e edição (:39–609), SyntaxErrorKind/SyntaxError (:611–670), LinkedNode/Side/LinkedChildren (:672–989), numbering (:991–1004), e 14 testes (:1006–1138). Hash L0 original 58836fbf e hash código 66771d34 confirmados antes da decisão. Inventário exato e SHA-256 da fonte em `00_nucleo/diagnosticos/syntax-node-atomization.md`.

## Classificação

Recorte de organização interna, ADR-0037/0109/0127 contínuo: não altera assinatura, campo público, defaults, fase ou compatibilidade. Sem nova dependência, I/O ou estado global. Paridade da linguagem não está sendo medida; preservar texto, categorias, hints, árvore e navegação é aceitação, enquanto disposição física e hashes são controles mecânicos da refatoração.

## Contrato proprietário

Único dono de SyntaxNode(NodeKind), NodeKind fechado Leaf/Inner/Error e LeafNode/InnerNode/ErrorNode. Manter o impl SyntaxNode inteiro, sem fragmentá-lo por tema, incluindo construtores, leituras, erro da árvore, COW Arc::make_mut, spanless_eq, edição, contagens, numeração e Debug/Default. Leaf inline; Inner/Error em Arc; texto em SyntaxText. Campos e enum internos permanecem privados; nenhum pub(crate) por conveniência.

Linked é filho privado descendente via path linked.rs, reexportando seus tipos para a fachada. Pode ler NodeKind/InnerNode privados por descendência, sem getters novos, mudança de algoritmo ou campos alargados. Diagnósticos são recebidos do owner `00_nucleo/prompts/entities/syntax-node/errors.md`; o carrier ErrorNode permanece nesta representação.

## Invariantes e aceitação

Testes locais cfg(test) verificam os caches internos após edição COW e
preservação de spans dos filhos iguais nas bordas de replace_children.

Construtores leaf/inner rejeitam kind Error em debug; placeholder Error continua panic. len conta bytes, inner agrega descendants/erroneous; textos de inner vazios nas leituras e concatenados em into_text. Erros/hints e synthesize conservam isolamento de clones; spanless_eq ignora apenas spans e mantém kind/message/hints/texto. Manter a numeração, espaços insuficientes, bordas e algoritmo replace_children/update_parent sem alterações; usar NumberingResult/Unnumberable do owner errors. Preservar corpos dos impls e derivados Clone/Eq/PartialEq/Hash. Testes históricos e controles da fachada cobrem leitura, clone, erro e find; parser/lexer/source exercitam integração.


## 00_nucleo/prompts/entities/syntax-node.md

# Prompt L0 — fachada de syntax_node
Hash do Código: dc567ae6

**Camada:** L1
**Consumer exclusivo:** `01_core/src/entities/syntax_node.rs`

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:08:05Z: fonte original em `01_core/src/entities/syntax_node.rs`, 1.138 linhas. Declara SyntaxNode/NodeKind (:26–38), representação e edição (:39–609), SyntaxErrorKind/SyntaxError (:611–670), LinkedNode/Side/LinkedChildren (:672–989), numbering (:991–1004), e 14 testes (:1006–1138). Hash L0 original 58836fbf e hash código 66771d34 confirmados antes da decisão. Inventário exato e SHA-256 da fonte em `00_nucleo/diagnosticos/syntax-node-atomization.md`.

## Classificação

Recorte de organização interna, ADR-0037/0109/0127 contínuo: não altera assinatura, campo público, defaults, fase ou compatibilidade. Sem nova dependência, I/O ou estado global. Paridade da linguagem não está sendo medida; preservar texto, categorias, hints, árvore e navegação é aceitação, enquanto disposição física e hashes são controles mecânicos da refatoração.

## Contrato proprietário

Declara errors e representation privados. Reexporta explicitamente SyntaxNode, SyntaxError, SyntaxErrorKind, LinkedNode, LinkedChildren e Side nos mesmos caminhos públicos entities::syntax_node; NumberingResult e Unnumberable mantêm pub(crate). Não possui enum interno, structs de representação, lógica de erro nem navegação produtiva. Conserva os 14 testes históricos inline nos seus caminhos e acrescenta controles da fachada, UTF-8/trivia, iteração dupla, find/numeração, igualdade sem spans e isolamento de clone.

Representação/edição pertencem exclusivamente a `00_nucleo/prompts/entities/syntax-node/representation.md`; erros a `00_nucleo/prompts/entities/syntax-node/errors.md`; navegação a `00_nucleo/prompts/entities/syntax-node/linked.md`. A documentação monolítica de 2026-03-22/04-12 e P786a é proveniência histórica; suas obrigações foram transferidas para esses owners, não duplicadas. A antiga exceção de coesão ao módulo inteiro deixa de se aplicar: impl SyntaxNode continua coeso, sem fragmentação por método.

## Aceitação

Controle estrutural RED antes da extração; todos os testes históricos e novos GREEN depois; preservar imports dos consumidores, API pública e assinaturas. Testar parser, lexer, source/AST, build produtivo, ownership V15/V26 e hashes. A fachada não torna públicos os módulos filhos.

