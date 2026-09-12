# Prompt L0 — `stdlib/foundations/float` — superfície pública numérica de `float`
Hash do Código: eb2d9000

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/float.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: P1289
**Baseline**: vanilla ratificado `a51e02804`
**ADRs**: ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

Em `2026-08-31T10:35:47-03:00`–`10:40:57-03:00`, sobre HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88` e working tree não commitida, o
binário vanilla de SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
devolveu `(function, "is-infinite")` para o field estático e
`[false,false,true,true,false]` para `0.0`, `42.5`, infinito positivo,
infinito negativo e NaN. A forma ligada produziu o mesmo vetor.

As duas ordens forward/reverse produziram resultados idênticos. A obtenção
do método ligado como valor, sem chamada, falhou no vanilla com
`cannot access fields on type float`; portanto, esse acesso reflexivo não faz
parte da superfície autorizada. O cristalino pré-candidato, construído da
árvore corrente em target isolado e com SHA-256
`e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78`,
falhou por ausência do field/método.

Uma auditoria posterior das pré-condições mostrou que `float.inf` e
`float.nan` são também `MISSING_MEMBER` no cristalino
(`p1284-inventory-default.json`). Os dois lados aceitam
`float("1e999")` como infinito positivo e `float("NaN")` como NaN; estes
carriers isolam a semântica de `is-infinite` sem materializar dois fields fora
do escopo nem converter o ganho esperado de `+1` em `+3`. O primeiro selo A/B
foi invalidado e refeito a partir desta medição.

A fonte vanilla medida em
`lab/typst-original/crates/typst-library/src/foundations/float.rs:81-94`
declara `is_infinite(self) -> bool` e delega a `f64::is_infinite`. A macro
upstream é mecânica, não contrato arquitetural (ADR-0107).

## Contrato da linguagem

- `float.is-infinite` existe como `function` com `repr` exatamente
  `"is-infinite"`.
- `float.is-infinite(self)` aceita `Float` e a coerção de `Int` já medida;
  retorna `true` somente para infinito positivo ou negativo.
- Valores finitos, ambos os zeros e NaN retornam `false`; os testes usam
  `float("1e999")`/`-float("1e999")` e `float("NaN")` como carriers bilaterais.
- `(value).is-infinite()` é a forma ligada para `Value::Float` e delega à
  mesma função da forma estática, sintetizando `self` como primeiro positional.
- O acesso não chamado `(value).is-infinite` permanece ausente e mantém
  `cannot access fields on type float`.
- Aridade e named args são fechados: ausência de `self` →
  `missing argument: self`; segundo positional → `unexpected argument`;
  named desconhecido → `unexpected argument: <nome>`; tipo não coercível →
  `expected float, found <tipo>`.

## Estrutura

`float_type_field` é um match fechado que descobre somente
`is-infinite`. A nativa pura possui a fórmula `f64::is_infinite`; o dispatch
ligado insere o receiver como primeiro positional e chama a mesma nativa.
Não criar trait, registry, reflexão, fallback genérico nem duplicar a fórmula
em `field_access` ou `call_dispatch`.

As funções de integração Rust são internas à crate (`pub(crate)`), sem novo
contrato público. O constructor `float(...)` permanece no owner
`foundations/cast`; `signum`, bytes e outros fields, inclusive as constantes
`float.inf` e `float.nan`, continuam resíduos independentes do inventário.

## Verificação

Cobrir presença, `repr`, chamada estática e ligada, finitos, `±inf`, `nan`,
coerção de inteiro, missing/extra/named/type errors e ausência do valor ligado.
Os testes devem rejeitar as mutações: sempre falso, `nan` infinito, aridade
permissiva, `repr` qualificado e presença sem chamada. Finalizar com RED→GREEN,
sonda bilateral forward/reverse, `cargo build`, `cargo test --workspace`,
`git diff --check` e `crystalline-lint .` sem violations.

## `float.is-nan`

- `float.is-nan` é função de nome público curto `is-nan`.
- A forma estática aceita exatamente um Float, incluindo a coerção de Int já
  definida para funções de float, e retorna `true` somente para NaN.
- A forma ligada existe apenas para receiver Float e delega à mesma nativa.
- Obter o método ligado sem chamada continua inválido.
- Missing, positional extra, named e tipo inválido preservam mensagem e span
  da âncora sintática responsável.

`float_type_field` e o dispatch ligado estendem o match fechado já usado por
`is-infinite`. A fórmula é `f64::is_nan`; não criar registry, reflexão,
fallback ou segunda implementação.

A nativa consome `Args.span` para erros de chamada. Seleção da âncora antes da
agregação pertence a `call_dispatch`; o acesso sem chamada pertence a
`field_access`. Este owner não recompõe offsets, lê fonte ou altera
`entities::Args`.

## P1307-R3 — transporte coerente nos predicados ligados

### Medição anterior à decisão

`01_core/src/compiler/stdlib/foundations/float.rs:120,124` insere receiver
em items para is-infinite/is-nan. O helper recebe f64, sem span lexical.
Fonte preservada em HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais
working tree P1306; L0 pré-edição e diff/stat completos no baseline adicional
R3 SHA-256 `592497d1e786241b9ba121479c0c55dbad370a70732e130b4f7ac3bd709d4405`.

### Decisão subordinada ao gate público de Args

Nos dois branches, Some deve receber nova ocorrência positional do receiver,
com span e value_span detached, antes da sequência original; reconstruir
via `Args::from_occurrences`. None reconstrói views via `Args::from_parts`.
Preservar span agregado, inclusive a âncora diagnóstica já selecionada pelo
dispatch; não alterar a regra P1293.reopen-A, valores ou ordem dos originais.
Não inventar origem do f64 nem invalidar origem dos argumentos recebidos.

Não mudar fórmulas, superfície, casts, mensagens ou regra de named duplicados
destas nativas. A proibição anterior de alterar Args continua aplicável a
este consumer: a definição nova pertence somente ao owner entities/args,
com seu gate ADR-0127 ainda pendente. Este owner apenas migra transporte.
Testes devem manter os diagnósticos certificados is-nan e cobrir Some/None
com receiver presente exatamente uma vez. É inferência de suficiência,
refutada por perda de origem ou alteração das âncoras legadas.

## P1339 — constantes, sinal e conversões binárias

### Medição anterior à decisão

Referência de linguagem: vanilla ratificado `a51e02804`. Os recibos
`00_nucleo/diagnosticos/p1339-full-final-vanilla-runs.json:2` e
`p1339-full-boundaries-vanilla-runs.json:2` registram os horários, binários,
comandos e working tree sobre HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
com `git diff HEAD --stat` vazio. Os manifestos correspondentes congelam as
expressões por ID; não são um candidato de implementação.

A fonte `lab/typst-original/crates/typst-library/src/foundations/float.rs:14`
documenta float de 64 bits e coerção de inteiro; `:20,34,39` documenta e
declara as constantes. `:97–112` especifica sinal, inclusive zeros e NaN;
`:124–145` especifica binary32/binary64, comprimento e endian little;
`:162–184` especifica little, tamanho default 8 e redução para binary32.
As sondas `signum-static-*`, `to-bytes-*`, `from-bytes-*`, `bytes-cast-*` e
`named-required-float.from-bytes` confirmam valores e erros. Em particular,
`-0.0` conserva sinal, binary32 de `0.1` relido resulta em
`0.10000000149011612`, tamanho acima de u32 falha antes da validação 4/8.

O consumer antecedente `01_core/src/compiler/stdlib/foundations/float.rs:97–108`
só descobre `is-infinite` e `is-nan`; `:111–140` já possui transporte ligado
com ocorrências. É owner individual legítimo para as adições. A documentação
do constructor (`float.rs` vanilla `:41–64`) aceita mais tipos que o parâmetro
float; copiar esse cast ampliaria indevidamente estas rotas.

### Classificação e contrato proprietário

Presença, nome, tipo, valor, ordem, mensagens, hints e spans são linguagem.
IEEE bytes são resultado público desta API e devem coincidir exatamente;
não se exige identidade de bytes de render ou de representação Rust. Macros,
armazenamento e organização upstream são mecânica. A intenção de conversão,
defaults e sinal vem da documentação citada, e os detalhes diagnósticos da
medição pública. Inferência de suficiência do owner existente é refutada por
uma obrigação que não possa ser expressa por suas nativas e Args causais.

Este adendo sucede **somente** os scope-outs anteriores de `inf`, `nan`,
`signum`, `from-bytes` e `to-bytes`. Preserva predicados certificados e o
constructor no owner `foundations/cast`.

- `float.inf` é float infinito positivo, `repr` `float.inf`; `float.nan` é
  float NaN, `repr` `float.nan`, não igual a si próprio. Não são funções.
- `float.signum` é função com nome/repr curto `signum`; recebe um positional
  `self`, Float ou Int convertido para float, e retorna Float: `1.0` para
  positivos e `+0.0`, `-1.0` para negativos e `-0.0`, NaN para NaN. Os dois
  infinitos seguem seu sinal. Não aceitar booleano, string, ratio ou angle.
- `float.from-bytes` é função de nome/repr `from-bytes`; recebe um positional
  `bytes` estritamente Bytes, e named `endian` opcional (`"little"` default
  ou `"big"`). Comprimento 4 interpreta binary32 e promove a float de 64 bits;
  comprimento 8 interpreta binary64. Outros comprimentos falham com
  `bytes must have a length of 4 or 8`. Retorna Float, inclusive infinito,
  NaN, subnormal e zero com sinal quando codificados na entrada.
- `float.to-bytes` é função de nome/repr `to-bytes`; recebe positional `self`
  com o mesmo cast Float/Int e named `endian` igual ao anterior, mais `size`
  inteiro u32, default 8. Só 4 e 8 são tamanhos válidos. Size 8 representa o
  float em binary64; size 4 arredonda para binary32, incluindo subnormal,
  underflow, overflow e sinal de zero. Retorna Bytes, nunca Array. Tamanho
  u32 diferente de 4/8 falha com `size must be either 4 or 8`.

As formas ligadas Float de `signum` e `to-bytes` inserem receiver uma vez e
delegam à mesma nativa. O método inteiro preexistente `Int.signum` continua
produzindo Int; coerção estática não transfere métodos float a inteiros.
`from-bytes` em receiver Float é reconhecido pela rota medida e rejeita esse
receiver como primeiro argumento Bytes (`expected bytes, found float`);
Bytes não ganha método `from-bytes`. Não transformar nenhuma dessas rejeições
em sucesso nem erro genérico diferente. Extração de método Float sem chamada
continua `cannot access fields on type float`, ancorada no nome do campo.

### Parser e diagnósticos fechados

Avaliar argumentos uma vez antes do parser nativo. Consumir parâmetros
declarados preservando ocorrências, em ordem `self`/`bytes`, `endian`, `size`
quando aplicáveis, depois rejeitar sobras. Uma segunda fórmula ou parser
distinto para método ligado é proibido.

- Missing positional: `missing argument: self` ou `missing argument: bytes`,
  no span da chamada inteira. Named com outro nome não preenche positional.
- Required fornecido apenas por seu nome: `the argument \`self\` is positional`
  ou `the argument \`bytes\` is positional`, na ocorrência named completa,
  com hint `try removing \`self:\`` ou `try removing \`bytes:\``.
- Cast: `expected float, found <tipo>` / `expected bytes, found <tipo>` no
  value span da ocorrência. Não usar o cast amplo do constructor.
- Endian string inválida: `expected "big" or "little"`; tipo não-string:
  `expected "big" or "little", found <tipo>`, no value span named.
- Size não-inteiro: `expected integer, found <tipo>`; negativo:
  `number must be at least zero`; acima de u32: `number too large`, todos no
  value span named. Esses casts antecedem `size must be either 4 or 8`.
- Extra positional: `unexpected argument` no span dessa ocorrência; named
  não consumido: `unexpected argument: <nome>` na ocorrência named completa.
  Preservar a primeira sobra na sequência causal, inclusive após spreads.
- Erros de domínio de comprimento/tamanho ancoram a chamada inteira.

O dispatch fornece span integral mesmo por alias/With da nativa resolvida;
não selecionar comportamento pelo spelling do callee. Este owner escolhe
spans pelas ocorrências reais de Args e fallback agregado para origem
legitimamente ausente. Não buscar texto, recomputar offsets nem trocar origem
conhecida por detached. Preservar named duplicados permitidos por spreads e
sua validação causal; sintaxe literal duplicada continua erro antes da nativa.

### Limites e aceitação

Somente as cinco rotas desta família entram no incremento. Não materializar
outras rotas dos lotes sucessores de P1339 nem declarar paridade geral float.
Não mudar campos/assinaturas públicas de entidades, defaults do produto ou
fase de pipeline. Helpers novos podem ser internos à crate, com ownership 1:1.
É correção de paridade ADR-0127; permanece dependente de contrato selado, RED
independente e gates do protocolo completo antes da implementação.

Aceitação cobre ambas as formas aplicáveis, aliases, presença/nome/repr,
coerções/rejeições, finitos/±zero/±infinito/NaN, binary32/64, ambos endian,
defaults, round-trip e bytes independentes, comprimentos/tamanhos inválidos,
aridade, named, ordem, hints e spans. Os casos congelados nas duas ordens e
perfis são testemunhas, não autorização para limitar a semântica às fixtures.
Mutantes que troquem endian/size, apaguem zero negativo, tornem NaN reflexivo,
retornem Array ou ofereçam só presença devem ser rejeitados por observáveis
da linguagem. Predicados anteriores e métodos inteiros permanecem controles.
