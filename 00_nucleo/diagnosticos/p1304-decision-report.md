# P1304 — rebaseline e seleção causal do P1305

## Estado da seleção

Coorte única selecionada pelo classificador: **`repr-module-and-large-array`**.
Estado final: **`P1304_PASS_P1305_COHORT_SELECTED`**, conforme o certificado
independente pinado abaixo. Este relatório registra o veredito recebido,
não autoriza implementação e não substitui o certificado.

Gate esperado: `ADR-0127_CONTINUOUS_PARITY_CORRECTION`, limitado ao formatter
existente, sem nova entidade/API/default/fase/quebra. P1305 deverá primeiro
atualizar seu L0 e provar RED→GREEN. Se o desenho concreto precisar mudar a
representação de anonimato, assinatura ou contrato público, o gate passa a
`ADR-0127_HUMAN_GATE_REQUIRED`; P1304 não autoriza essa expansão.

Regime: A/B bilateral com medição, classificação e veredito sob autoridades
distintas, **executado sem atestação de isolamento técnico**. O filesystem é
compartilhado. Este papel escreveu exclusivamente o ledger e este relatório;
não escreveu runner, refinamentos, manifesto, certificado, L0, Rust ou testes.

## Proveniência anterior às decisões

Baseline composto: HEAD `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2` +
candidato P1303 não commitado. A matriz registra medição de
`2026-09-05T02:32:36.761423+00:00` a
`2026-09-05T02:41:25.584840+00:00`, equivalentes à noite local de 4 de setembro.
A árvore de produto nesse intervalo contém exatamente:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 562 insertions(+), 4 deletions(-)
```

O vanilla é upstream/main ratificado `a51e02804`, binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O cristalino fresco é
`/dev/shm/p1304-build.3xt5tH/release/typst`, SHA-256
`a31da4d06bf6f51e1d690a2c186c39bb78fd46a7a06de607d25168474c533668`.
Nenhum número abaixo se refere apenas ao HEAD ou à string de versão.

Entradas novas recebidas prontas antes da classificação final:

| Artefato | SHA-256 |
|---|---|
| `p1304-feature-matrix.json` | `7679f2438f59a3fdf3d17760f1398b3058b130b7c79f6234b86932fbecb810fb` |
| `p1304-repr-refinement.json` | `6b300e95e456fd36828e68f8b15889af2a8184e514ee5be57ad2c8c1c0b4f6d2` |
| `p1304-encode-readiness.json` | `156f56b9f65ad5fd4310aa7785d4667c1f2c20e13f3f5c8be43b280700e7b950` |
| `p1304-refinement.log` | `622af79aec2da88f8e1b5a3c1b2fc95eca417870a43c048790d4c940fd9f90d1` |

Os três artefatos P1303 finais e os quatro arquivos do candidato permanecem
pinados no passo e serão reconferidos no manifesto/certificado. P1299 é
comparação histórica, não oracle para a classificação semântica atual.

## Matriz nova e sentinelas

| Perfil | MATCH_VALUE | MATCH_DIAGNOSTIC | CRYSTALLINE_ONLY | VANILLA_ONLY | DIFFERENT_VALUE | DIFFERENT_DIAGNOSTIC | EXECUTION_UNKNOWN |
|---|---:|---:|---:|---:|---:|---:|---:|
| default | 390 | 121 | 42 | 62 | 12 | 0 | 0 |
| html | 461 | 6 | 42 | 106 | 12 | 0 | 0 |
| a11y | 393 | 118 | 42 | 62 | 12 | 0 | 0 |
| html+a11y | 464 | 3 | 42 | 106 | 12 | 0 | 0 |
| Total | 1708 | 248 | 168 | 336 | 48 | 0 | 0 |

São `627 × 4 = 2508` pares normais e 768 pares invertidos do subconjunto
decisório/sample. O suplemento separado de oito sentinelas contém
`8 × 4 × 2 = 64` pares. Total:
`(2508 + 768 + 64) × 2 = 6680` processos. Há zero divergências de repetição
e zero EXECUTION_UNKNOWN na matriz. Suplemento não aumenta o catálogo de 627.

O cruzamento por `id/profile` entre os 626 probes históricos e os novos
encontra exatamente 18 mudanças de classe runtime: 12 dos três aliases bare,
de CRYSTALLINE_ONLY para MATCH_DIAGNOSTIC, e seis dos três gates PDF sem
a11y, de DIFFERENT_DIAGNOSTIC para MATCH_DIAGNOSTIC. Nenhum MATCH histórico
passou a não-MATCH. Não existe prova de NEW_REGRESSION nesse corpus.

As seis rotas negativas bare/sob std permanecem rejeitadas nos quatro perfis;
`color.hsl`, `color.hsv` e `color.linear-rgb` preservam MATCH_VALUE.
`calc.nope`, `sym.nope` e `color.map.nope` têm MATCH_DIAGNOSTIC;
`std.rgb`, `calc.abs` e `sym.arrow` preservam sucesso. `color.map.turbo`
preserva sucesso e dados, mas sua repr continua diferente: sucesso do lookup
não foi convertido em igualdade textual.
Os três `pdf.*` medidos são MATCH_DIAGNOSTIC sem a11y e MATCH_VALUE com
a11y, nos quatro perfis e nas repetições pertinentes.

## Refinamento de representação: dados antes da classificação

A causa está localizada em
`01_core/src/compiler/eval/repr.rs:36` (formata todos os itens) e
`:56` (`module(name)`). O vanilla em
`lab/typst-original/crates/typst-library/src/foundations/array.rs:1188`
limita a representação a 40 itens, acrescenta a quantidade omitida e usa o
formatter multilinha. Em `foundations/module.rs:174` distingue módulo
nomeado e anônimo.

O refinamento D preserva os valores completos de `components()`, tokens
RGBA8 derivados de `to-hex()`, ambos os digests integrais e os índices
0, 1, 39, 40, penúltimo e último. Os 15 mapas coincidem bilateralmente em
kind, cardinalidade, todos os componentes, canais/ordem e digest RGBA8, nos
quatro perfis. Índices fora dos mapas pequenos são NOT_APPLICABLE, não
Unknown nem igualdade inventada.

| Mapa | Cardinalidade | SHA-256 RGBA8 bilateral, também igual ao L0 | repr |
|---|---:|---|---|
| `color.map.cividis` | 256 | `dece34103d5266311558bb44f96f02df7090edf8977744fc9a1213b2213b2f79` | diferente |
| `color.map.coolwarm` | 256 | `13161dddf6ad860268eece02ce446b1025d3ab95871cc1a484a93aa05da4aed2` | diferente |
| `color.map.crest` | 256 | `660b6381c0f53156c07b4f0f86988a280c06578f46a44b734c1f388cadeaeb98` | diferente |
| `color.map.flare` | 256 | `03f70bedc78f4fb3e22b3f7ed06a680d56d721321eca69218ab746d4c803f595` | diferente |
| `color.map.icefire` | 256 | `0de0d1c2a9dd6a8a03cd3155b872ea4a2eeaa74a5454df8f511632aa5d91e55f` | diferente |
| `color.map.mako` | 256 | `f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77` | diferente |
| `color.map.rainbow` | 256 | `74fe385a692d3da43ff8afc8f61896f76bc87a1a766f05f4897f5e8164db4ed7` | diferente |
| `color.map.rocket` | 256 | `b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b` | diferente |
| `color.map.turbo` | 256 | `7909421397bc2bef00faf5a1064d7356ad4cab112ec1fd5e290b124a13529a3e` | diferente |
| `color.map.vlag` | 256 | `d015c8a4cb87aaebc49d0416edf430e5c76392e772602ca1234ce6fe748db81d` | diferente |
| `color.map.viridis` | 9 | `3b9d02b0685ec2ae62958a287c4cdd68fdb08f642339aabaa8d96e60aeaf999b` | igual |
| `color.map.inferno` | 11 | `20de1b5440e58c3727d645e6f8444d9e6d1c73fb09f9e905a49c314dc58496da` | igual |
| `color.map.magma` | 11 | `c634397325d83383609535f10e2d964efd899be34a2cef337379a5368b09fa79` | igual |
| `color.map.plasma` | 11 | `f0ddd8a6d9e12c3c3b5d705dfa2e7c5f7929ce242c8d94ab5e4a0f6b2e0d9f2d` | igual |
| `color.map.spectral` | 11 | `1c62ea2e765ddf6d6b1c5189e772605e0203d6116bf384597df0cfa4cdab5f17` | igual |

Para cada um dos dez mapas de 256 cores, o vanilla mostra 40 cores,
`.. (216 items omitted)` e 42 quebras de linha; o cristalino mostra as
256 cores e 257 quebras. São contagens dos valores completos registrados em
D, não de snippets truncados deste relatório. Os controles pequenos conservam
repr idêntica. `range(0/1/39/40)` coincide; `range(41/42/81)` e nesting
com array longo divergem, isolando o limite de elisão de nomes de mapas.

`color.map` tem kind module e nome map nos dois lados; a diferença é
`module(map)` versus `<module map>`. `repr(std)` é
DIFFERENT_VALUE em cada perfil, `module(std)` versus `<module global>`.
Os diagnósticos de nome já confirmam global para std, conforme
`field_access.md:425` e `field_access.rs:376`; a construção interna
continua em `eval/mod.rs:324`. Logo a seleção exige projeção coerente do
nome público, preservando módulos ordinários; não presume que todo módulo
deva imprimir global.

Classificação dos 11 `color.map*` históricos: **WRONG_PUBLIC_REPR**, e não
WRONG_PUBLIC_KIND_OR_IDENTITY nem semântica de dados. `repr(std)` acrescenta
o décimo segundo path. Refutariam essa inferência diferença em qualquer
canal/digest/kind/nome ou necessidade de entidade/API nova. O L0 repr cobre o
formatter, mas ainda precisa registrar explicitamente as novas fronteiras
medidas; sua regra P1290 de formatação dos itens não foi usada como prova
de uma elisão que ela ainda não especifica.

## Encoders: presença e prontidão

Os três membros continuam VANILLA_ONLY nos quatro perfis. O vanilla expõe
kind function e repr `encode`. E contém 114 probes, 456 pares e 912
processos, com 348 VANILLA_ONLY, 12 MATCH_VALUE de controles de representação
e 96 DIFFERENT_DIAGNOSTIC. Estes 96 são chamadas inválidas cujo cristalino
falha antes, na ausência do membro; não constituem coorte diagnóstica
independente do encoder ausente.

`json.encode(value, pretty: true|false)` aceita os valores estruturados
medidos; default true preserva indentação, false produz forma compacta.
`toml.encode(dict, pretty: true|false)` exige dicionário, também usa
default true e rejeita argumentos não-dict. A diferença pretty/compact
aparece nos casos aninhados/arrays, não em todo dicionário simples.
`yaml.encode(value)` não recebe pretty. Ordem, escaping, multiline,
nesting, none/bool/int/float/string/array/dict e erros de argumentos ficam
nos outputs integrais. A cor opaca medida é serializada por sua repr
pública; isto não generaliza para Symbol/Content nem para todo serializer.

Causa observada: `stdlib/loading.rs:267` possui somente cbor.encode;
`:540` instala os decoders. `eval/mod.rs:1723` registra json/yaml/toml
como funções sem namespace, contrastando com cbor. No vanilla, os contratos
estão em `loading/json.rs:134`, `toml.rs:107` e `yaml.rs:111`.
A coorte tem um domínio semântico coeso, mas pelo menos dois owners
produtivos/L0 (`loading` e `eval`) e owner próprio de testes; reexportar
novas nativas provavelmente acrescentará `stdlib/_comum`. Nenhuma nova
variant de Value foi demonstrada necessária. Nova assinatura Rust pública,
superfície pública, default ou fase exigirá gate humano no próximo contrato.

## Ledger: conservação e contagens verificáveis

O TSV contém todos os 166 paths históricos, sem renomear/deduplicar, mais
`repr(std)`: **167 linhas de dados únicas**. A coluna `universe`
separa os 160 residuais P1299, o novo repr(std) e seis controles já fechados.
O probe ID novo usa path técnico std na matriz, mas o ledger registra
explicitamente `path=repr(std)`, `probe_id=p1304-repr-std`; não confunde
isso com um novo binding.

| Classe | P1299 integral | Residual pós-P1303 | P1304, incluindo seis fechados |
|---|---:|---:|---:|
| MISSING_LANGUAGE_MEMBER | 106 | 106 | 106 |
| INTENTIONAL_PRODUCT_EXTENSION | 42 | 42 | 42 |
| WRONG_PUBLIC_KIND_OR_IDENTITY | 11 | 11 | 0 |
| WRONG_PUBLIC_REPR | 0 | 0 | 12 |
| L0_CONTRADICTION | 3 | 0 | 0 |
| DIAGNOSTIC_SPAN_DIVERGENCE (histórica) | 3 | 0 | 0 |
| EXPECTED_FEATURE_DISABLED | 1 | 1 | 1 |
| CLOSED_CONFIRMED | 0 | 0 | 6 |
| Total | 166 | 160 | 167 |

Portanto `106 + 42 + 12 + 1 = 161` entradas atuais no universo
`160 + repr(std)`, mais seis fechadas = 167. Nenhum dos 160 residuais foi
fechado incidentalmente. Onze mudaram de classe semântica pelo novo
observável, e um path foi acrescentado. Ownership/Unknown não resolvidos no
universo principal: zero.

As extensões não são perdoadas: continuam fora de seleção automática por
regra do passo. A auditoria corrigiu owners históricos de asset, replace,
lof/lot e os seis aliases grid/table para os nós próprios. Para
`calc.deg/rad/log10`, a frase antiga “L0 explicitamente retém” era forte
demais: a intenção explícita está em `calc.rs:18,87-91`; o L0 atual é
silencioso sobre os três nomes. Registrou-se essa lacuna, sem transformar
silêncio normativo em proibição e sem inferir intenção da mera execução.

## Achados auxiliares e prioridade: sem regressão temporal inventada

Os controles D adicionais não pertencem aos 160 residuais nem são novas
regressões comprovadas. Mantém-se inventário separado de nove observações
para que nenhuma diferença desapareça do diagnóstico. IDs abaixo são eixos
observáveis, não novos bindings nem novas linhas do catálogo de 627.

| ID/observável auxiliar | Classe/estado | Evidência e elegibilidade |
|---|---|---|
| repr(calc) | WRONG_PUBLIC_REPR | `module(calc)` / `<module calc>`; mesmo formatter, controle adicional, não aumenta os 12 paths pontuados. |
| repr(sym) | WRONG_PUBLIC_REPR | `module(sym)` / `<module sym>`; mesma regra. |
| repr(pdf) | WRONG_PUBLIC_REPR | `module(pdf)` / `<module pdf>`; mesma regra. |
| repr(imported-named) | WRONG_PUBLIC_REPR | Fixture controlada: nome p1304_named e x=7 iguais; wrappers diferentes; default apenas. |
| identity(anonymous-plugin) | L0_CONTRADICTION com a paridade de anonimato | Vanilla `<module>`, cristalino `module(plugin)`; `plugin.md:145` exige nome plugin e `entities/module.md` exige String. Sem rota canônica anônima demonstrada; gate público e prioridade 6, não 2. |
| diagnostic(anonymous-plugin-name) | L0_CONTRADICTION com a paridade de anonimato | Mensagem sem nome / com plugin; mesma causa anterior e scope-out explícito `plugin.md:482`; não é regressão P1301r2. |
| diagnostic(imported-source-path) | L0_CONTRADICTION com a paridade de apresentação | Mesma mensagem/nome/span, paths `../../../tmp/...` / `/tmp/...`. `wiring.md:65-71` determina prefix stripping e fallback absoluto; `main.rs:634` cumpre. Vanilla `typst-cli/src/world.rs:153` usa pathdiff. Não há rota canônica funcional alternativa medida: prioridade 6. |
| adapter(query-deprecation-hint) | Diferença auxiliar de warning; fora do observável D | Query tem exit0 e valor válido; hint concreto vanilla / hint genérico cristalino em `main.rs:513`. D pede nome/kind/repr do módulo, não contrato de migração do subcomando. Não se promove esta emissão lateral a DIAGNOSTIC_DIVERGENCE do corpus de failures B. O stderr integral permanece registrado; não se alega igualdade de warnings. |
| adapter(panic-span) | Diferença auxiliar de âncora; causa isolada não demonstrada | O panic foi construído para expor repr e falha deliberadamente. Seu texto já diverge por repr; callee inteiro / args.span também diferem. `stdlib/panic.rs:44` usa args.span. Não há controle pareado de panic com mesma repr neste contrato; não se conta como regressão ou coorte diagnóstica validada. |

Comparabilidade dos controles finais: mesmos argv salvo binário, cwd do
repositório e fixtures pinadas. As tentativas iniciais de import/plugin via
eval não observavam o módulo; permaneceram na calibração e foram substituídas
por query/compile. O nome/payload finais são observáveis; a indisponibilidade
do adaptador inicial não foi classificada como defeito do módulo.

Contagem auxiliar: quatro controles adicionais de repr, três observações de
contradição/contrato fora do universo principal e dois efeitos laterais dos
adaptadores = nove. As três contradições não têm a rota canônica funcional
exigida pela prioridade 2. Os dois efeitos laterais não receberam um novo
contrato diagnóstico por inferência neste passo. Esta delimitação não afirma
que a CLI seja paritária: uma auditoria dedicada de warnings/spans/pathnames
pode legitimamente priorizá-los após congelar os observáveis correspondentes.
Refutariam a delimitação uma sentinela B/C divergente, uma rota canônica
anônima já funcional ou um controle diagnóstico independente já declarado e
comparável que tivesse sido omitido.

## Seleção determinística

Exame das prioridades superiores: NEW_REGRESSION = 0 no cruzamento temporal;
nenhuma L0_CONTRADICTION com rota canônica funcional permanece no universo
principal ou foi demonstrada nos controles auxiliares; DIFFERENT_DIAGNOSTIC
do corpus B = 0. Erros E decorrem de membros ausentes. Os auxiliares foram
tratados explicitamente acima, sem alterar prioridades ou política de Unknown.

| Coorte | Prioridade | Owners produtivos/L0 | API Rust nova | Default/fase nova | Paths principais por causa | Superfície de regressão | Resultado |
|---|---:|---:|---|---|---:|---|---|
| repr-module-and-large-array | 4 | 1: compiler/eval/repr | 0 demonstrada necessária | 0 | 12 no formatter: 10 elisão + 2 projeção Module | arrays/Module e serializers que consomem repr | selecionada |
| loading-encoders | 5 | pelo menos 2: loading + eval; possível reexport adicional | nativas/exportação a decidir; gate se públicas | nenhum novo default de produto demonstrado; defaults de novas funções precisam contrato | 3 | três serializers, registros, errors e opacos | perde na prioridade |
| restantes membros ausentes, por owner | 5 ou 6 conforme implementação concreta | um owner semântico mais glue quando necessário | não presumido zero | não presumido zero | 103 além dos encoders | famílias distintas; não há causa única para agregá-los | prioridade inferior a 4 |
| anonimato plugin e diagnóstico correlato | 6 | plugin + Module, lookup conforme contrato | gate de representação pública provável | contrato público de anonimato | 0 no ledger principal | construção/import/diagnóstico | não elegível automaticamente |
| apresentação de path externo em diagnóstico | 6: contradição sem rota canônica medida | wiring; L2 é auditoria de fronteira | a decidir no contrato futuro | contrato vigente explícito | 0 no ledger principal | drenagem de fontes/CLI | não supera prioridade 4 |

A prioridade 5 dos encoders decorre de um único owner semântico, loading,
e dos carriers/representação já disponíveis para o fragmento medido. O glue
em eval e a eventual reexportação ampliam owners produtivos no desempate;
não transformam sozinhos prioridade 5 em 6.

A prioridade 4 vence antes dos desempates. Nessa prioridade há uma única
coorte elegível no ledger principal. A coorte não é uma alegação de algoritmo
único para arrays e módulos: são duas projeções do mesmo owner produtivo
obrigatório, e a tabela torna isso explícito. As comparações “mais paths” e
“menor regressão” não foram usadas para vencer uma prioridade superior; não
há empate restante que exija ordenar IDs lexicograficamente. Separar as duas
projeções em entregas internas não autoriza segunda coorte P1305.

Incluídos exatamente: `repr(std)`, `color.map`,
`color.map.cividis`, `color.map.coolwarm`, `color.map.crest`,
`color.map.flare`, `color.map.icefire`, `color.map.mako`,
`color.map.rainbow`, `color.map.rocket`, `color.map.turbo`,
`color.map.vlag`. Controles de módulo ordinário e arrays pequenos devem ser
preservados; anonimato plugin é fora dessa alegação.

## Owners e pins

Cada relação abaixo foi conferida por header `@prompt` exato em L1–L4:
um único consumer por Prompt. `Hash do Código` é metadata declarada do L0,
não seu SHA-256 nem o `@prompt-hash` efetivo com Núcleos; o TSV conserva
essa distinção histórica. Lint/resselo serão validados pelo verificador.

Leitura integral deste papel: cinco owners obrigatórios, eval/tests,
field_access, calc, text/case, structural/document, plugin, panic,
shell/diagnostic e wiring. Demais owners receberam auditoria focal de claims,
headers/ownership e pin, sem alegar leitura integral. Nenhum owner adicional
é proposto para implementação P1304.

| Prompt L0 | Hash do Código declarado | SHA-256 atual | Consumer único |
|---|---|---|---|
| `00_nucleo/prompts/compiler/eval.md` | `38aa5e89` | `ce02b7d257e29742071497f112f1ea42ed734a93989e8fbb1332f1c09590f5e2` | `01_core/src/compiler/eval/mod.rs` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `2d102890` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` | `01_core/src/compiler/eval/bindings/field_access.rs` |
| `00_nucleo/prompts/compiler/eval/repr.md` | `d91654f3` | `555b6df222a931991f1a521b11fe2d8d58ee185cdcf90cd3f0eaf6d3c40c9355` | `01_core/src/compiler/eval/repr.rs` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `44131ec1` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` | `01_core/src/compiler/eval/tests.rs` |
| `00_nucleo/prompts/compiler/stdlib/calc.md` | `4f58eb82` | `0fc04b14b3cca19a203823ef392b56509d7ea2ff12a43550b2654fa2bb120f81` | `01_core/src/compiler/stdlib/calc.rs` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `356f1537` | `60de24ea6496a1c95fcc4af1883278164a2cd031351ef59278e654d7d485ff01` | `01_core/src/compiler/stdlib/color.rs` |
| `00_nucleo/prompts/compiler/stdlib/counter.md` | `d19229e2` | `a4802f75d7437152b3f738c90552db359e7a27efb71187636821eed8d407d27f` | `01_core/src/compiler/stdlib/counter.rs` |
| `00_nucleo/prompts/compiler/stdlib/foundations/float.md` | `c95f8eb6` | `f28cb58e46ced68ddb29335a35e8ade4f03a7c30a573c7e79a86b1ef3bc99107` | `01_core/src/compiler/stdlib/foundations/float.rs` |
| `00_nucleo/prompts/compiler/stdlib/foundations/selector.md` | `38a8528f` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` | `01_core/src/compiler/stdlib/foundations/selector.rs` |
| `00_nucleo/prompts/compiler/stdlib/html.md` | `33e187a1` | `df741b13ec8a3630180d65c3b606aca20385ab2b00d3d93836e85b5fbdca9ac9` | `01_core/src/compiler/stdlib/html.rs` |
| `00_nucleo/prompts/compiler/stdlib/loading.md` | `9379f85f` | `d6cf9cd85c0c25dc3f7a5f6c3a9f47cf956d9fd6e998932ca87ab5b1e58e2c72` | `01_core/src/compiler/stdlib/loading.rs` |
| `00_nucleo/prompts/compiler/stdlib/math_style.md` | `56b9e300` | `02a1dadb6c08f3ea9ae109ff7bd2dc87e2616a5da5d4a4cfbf5a1269bc4c9603` | `01_core/src/compiler/stdlib/math_style.rs` |
| `00_nucleo/prompts/compiler/stdlib/panic.md` | `277f949d` | `9e9003e0c322847298a227f78f58dda7e6f7d635f6525d1dad0d38d9d69bb012` | `01_core/src/compiler/stdlib/panic.rs` |
| `00_nucleo/prompts/compiler/stdlib/plugin.md` | `6fec5b48` | `7b8f7feb04481b909114c351c52bfe8cca8b1c7095bebd2e6cee3ee170eab58a` | `01_core/src/compiler/stdlib/plugin.rs` |
| `00_nucleo/prompts/compiler/stdlib/primitives-constructors/version.md` | `76701aae` | `6e775351938a8480df7f2fbd1658cdd3d8ae7b5908c4ee8dc34e46a778ce4e5c` | `01_core/src/compiler/stdlib/primitives_constructors/version.rs` |
| `00_nucleo/prompts/compiler/stdlib/shapes.md` | `6a9b5568` | `389f4f7905fcdaa2d5f61040475c04571c489cef0720f7ef453f14e36e61ed17` | `01_core/src/compiler/stdlib/shapes.rs` |
| `00_nucleo/prompts/compiler/stdlib/state.md` | `22d3e6d3` | `daf180d5e77f0a9d307b8b1a2b148741a6ac82a776ab17a510f107c5bff42eb1` | `01_core/src/compiler/stdlib/state.rs` |
| `00_nucleo/prompts/compiler/stdlib/structural.md` | `08c5c5f3` | `81e66889454d8f397ab5eafbd4c6a5e0f8a27993e15d2f3c81d307e804ff3b12` | `01_core/src/compiler/stdlib/structural/mod.rs` |
| `00_nucleo/prompts/compiler/stdlib/structural/document.md` | `093b7ea0` | `efeabfeb8286212f9c29ad97caf225efc38977139671cb74a6813dc744ee1a92` | `01_core/src/compiler/stdlib/structural/document.rs` |
| `00_nucleo/prompts/compiler/stdlib/structural/math.md` | `81441281` | `18b2fb0d9a3e1f6a0ff9cd8fb2e25047b4c9f2185cb6a511ae022e7a9c4e80a2` | `01_core/src/compiler/stdlib/structural/math.rs` |
| `00_nucleo/prompts/compiler/stdlib/structural/outline.md` | `01b891e9` | `e8dc3b96fd9a03ae21f87f602d85bb7b0b5b6eb95d6267dcbfe280a80d3b0632` | `01_core/src/compiler/stdlib/structural/outline.rs` |
| `00_nucleo/prompts/compiler/stdlib/structural/table_grid.md` | `d4d8c07e` | `153a22b018f247e1b37943a257afa3ad55da414b92886655a1415079118c3edd` | `01_core/src/compiler/stdlib/structural/table_grid.rs` |
| `00_nucleo/prompts/compiler/stdlib/sym.md` | `9bfa8e7f` | `42a2127215e9a37d69c90695e1be66a61320fa086ffd36b51a1ac3441c816f3d` | `01_core/src/compiler/stdlib/sym.rs` |
| `00_nucleo/prompts/compiler/stdlib/text.md` | `a8732f7f` | `1dadbba93127f2b62a8728bf0a92c5378c8f52b15bff5aa9152290faf0ac72a5` | `01_core/src/compiler/stdlib/text/mod.rs` |
| `00_nucleo/prompts/compiler/stdlib/text/case.md` | `4382da9f` | `b40155505b65b4ff0d9b41c7c9ecaff1c51b467bdfab7fc356ac808288adbc2c` | `01_core/src/compiler/stdlib/text/case.rs` |
| `00_nucleo/prompts/entities/func.md` | `c57c2de2` | `52b6f6e7ca7714027d4e3a883f6e5ae888a48e4a5c67e45651b63b86d9811908` | `01_core/src/entities/func.rs` |
| `00_nucleo/prompts/entities/module.md` | `3afc4142` | `17d75e8c05e28546b7212ae232fa5d9e0474eda2b5bd7b67740e54ec3c1659a9` | `01_core/src/entities/module.rs` |
| `00_nucleo/prompts/shell/diagnostic.md` | `0096c215` | `12f4ba4bf76930aec336bcc18965f62e61efd63f7c83dbcf0c95da82aae85575` | `02_shell/src/diagnostic.rs` |
| `00_nucleo/prompts/wiring.md` | `0d0307f2` | `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d` | `04_wiring/src/main.rs` |

## Limites e fechamento independente

A conclusão cobre a superfície medida, os refinamentos fechados e a seleção.
Não prova paridade funcional geral, serializers completos, módulos anônimos,
layout, bytes de export ou inexistência de dívida fora do corpus. Dados de
mapas não foram inferidos de repr truncada. Uma revisão focal do refinamento
corrigiu o adaptador `float(ratio)` para divisão por 100% e construiu
observações de módulos via query/compile; os recibos preservam tentativa,
causa e outputs. O classificador não alterou seus inputs nem fez uma segunda
execução completa para favorecer a seleção.

**Verificação independente concluída.** O recibo e o certificado registram
`P1304_PASS_P1305_COHORT_SELECTED` após a auditoria de baseline, hashes,
contagens, classes, sentinelas, prioridades e allowlist. Os quatro arquivos
abaixo foram reconferidos por SHA-256 antes desta atualização exclusivamente
de status/pins; a substância da seleção e o ledger permaneceram congelados.

| Predecessor final | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1304-manifest.json` | `eeeb675a6496860dc8a12a681d64b086574fb2d7264011ac162a45b43583d862` |
| `00_nucleo/diagnosticos/p1304-verification.log` | `3025741f62f1ff84f6093878e246158dc3748705eb0b96408f556aba63316fc6` |
| `00_nucleo/diagnosticos/p1304-verification-receipt.json` | `9c5b1800afcde5157905c99f71aeaad72972c735d6bca0327167d8a2a55d4a19` |
| `00_nucleo/diagnosticos/p1304-certificate.json` | `3e6aebcbedb2eb9fec44f3fed7446a68f1f63bc8b5d01c872034b3d8a97e86e4` |

A cadeia permanece detached: manifesto → recibo/certificado → relatório.
Este relatório não contém auto-hash e sua versão final não é pinada pelos
predecessores. O veredito mantém os limites de escopo e de atestação acima.

Nenhum staging ou commit foi realizado por este papel.
