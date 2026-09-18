# Passo 1447 — Materializar `math.abs` sobre o carrier público `math.lr`

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-18T16:46:16-03:00  
**Commit medido:** `9e1c891fe31d732b3f0b926cb1343f63a97e07f8`  
**Árvore medida:** `84995f8ac87566833988090dbbf092cd41695d4f`, limpa
(`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

P1446 fechou `P1420-MATH-018` (`math.stretch`) e deixou explicitamente
`math.abs` e as rotas `under*` no residual. O inventário P1422 identifica
`math.abs` como `P1420-MATH-017`; ela é a próxima unidade autocontida. Este
passo não concede crédito a `floor`, `ceil`, `bar`, `underbrace`,
`underbracket`, `underparen` ou `undershell`.

## 2. Medição antes da decisão

### 2.1 Fonte e binário vanilla ratificados

No pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`crates/typst-library/src/math/lr.rs:107-125` declara `abs` como função
pública com `size: Option<Rel<Length>>` nomeado e `body: Content` posicional
obrigatório. O helper comum constrói `LrElem` com body
`sequence(|, body, |)` e só fixa `size` no elemento quando o argumento esteve
presente. `math/mod.rs:75` registra a função no módulo.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/lr.rs` | `2a762699e7d3eed9a0b4b2d1afbbf5b58649339ec47ec12a2a82907519c107e0` |
| `lab/typst-original/crates/typst-library/src/math/mod.rs` | `a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957` |

O binário `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
é byte a byte igual ao build ratificado em
`lab/typst-original/target/release/typst`.

### 2.2 Observáveis públicos medidos

| Sonda vanilla | Resultado |
|---|---|
| `repr(math.abs)` | `abs` |
| `repr(math.abs([x]))` | `lr(body: sequence([|], [x], [|]))` |
| `repr(math.abs(size: 150%, [x]))` | conserva `size: 150% + 0pt` |
| `repr(math.abs(size: 100%, [x]))` | conserva `size: 100% + 0pt` explícito |
| `repr(math.abs(size: 1em, [x]))` | conserva `size: 0% + 1em` |
| `none`, string e símbolo | convertem para conteúdo dentro das duas barras |
| nesting | conserva o `lr` interior como body do exterior |
| `.body`, `.fields()`, `.has("size")`, `.func()` sem size | sequência cercada, só body, `false`, `lr` |
| campos com `size:150%` | size presente em `size/fields/has/at`; `.func()` continua `lr` |
| bare `$ abs(x) $` | mesmo carrier `lr` dentro de `equation` |
| bare com `size: #150%` | mesmo carrier e presença explícita da chamada qualificada |

Ausência de body, segundo positional, `body:` named, named desconhecido,
`size` string/inteiro, `size` duplicado e body inteiro falham. As mensagens
primárias medidas são, respectivamente, `missing argument: body`,
`unexpected argument`, ``the argument `body` is positional``,
`unexpected argument: foo`, `expected relative length, found ...`,
`duplicate argument: size` e `expected content, found integer`. Named
desconhecido na forma bare também falha, sem descarte silencioso.

As sondas foram executadas no commit e árvore acima, às
2026-09-18T16:46:16-03:00, com `/usr/local/bin/typst eval --jobs 1`; positivas
terminaram com exit zero e negativas com exit um.

### 2.3 Estado cristalino medido

`make_math_module` não registra `abs`; a chamada qualificada falha com
`module math does not contain abs`. A forma bare cai no braço histórico
agregado, ignora named e produz `MathDelimited('|', body, '|')`; a sonda
devolve `equation(block: true, body: |x|)`, apagando identidade `lr`, presença
de `size`, cascata, campos e nesting público.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/structural/math.rs` | `41d3b0f526376fb3cfafb5d1f472f68ea1a9cb543f3c27b31ed17e857f074286` |
| `01_core/src/compiler/eval/math.rs` | `cc068bdbdd562329d39ad2556266c2d97a8efdcf1a3ef32cf2d08a4874cbc165` |

## 3. Classificação anti-deriva

- Binding, assinatura, casts, diagnósticos, presença causal de `size`,
  identidade `lr`, campos e cercas `|…|` são linguagem (ADR-0107).
- O nome do helper Rust, a montagem do vetor e a variante concreta da
  sequência são mecânica e não concedem crédito.
- Medido: `abs` retorna o mesmo `LrElem` público já materializado, e `.func()`
  devolve `lr`, não `abs`. Inferência: o cristalino deve reutilizar o carrier
  P1436 e o contrato de `size`, sem criar `MathAbs`. Refutam isso uma fonte
  diferente no pin ou observável público incompatível.
- A aparência parcial do `MathDelimited` atual não satisfaz paridade: ela
  perde função pública, `size`, campos, cascata e identidade morfológica.

## 4. Decisão L0 proposta

1. Registrar `math.abs` antes do espelho `sym → math` como função nativa.
2. Aceitar exatamente um `body` posicional convertível a conteúdo e somente
   `size: Rel<Length>` nomeado; preservar ocorrência explícita inclusive para
   `100%`, aceitar spread canônico e rejeitar duplicados ou extras.
3. Construir `Content::MathLr` com `MathSequence([|, body, |])`, default
   `100%` não explícito ou o `size` explícito. Não criar `MathAbs`, não usar
   `MathDelimited` e não executar layout no builder.
4. Fazer `abs(...)` bare avaliar cada ocorrência uma vez e delegar ao mesmo
   parser/construtor da forma qualificada, preservando ordem, spread e erros.
5. Reutilizar `repr`, campos, cascata, short-fall, layout, igualdade e walkers
   de `MathLr`; `.func()` permanece `lr`, conforme medido.
6. Manter todas as outras rotas residuais P1422 fora deste recorte.

## 5. Gate ADR-0127

A proposta adiciona função pública e muda o comportamento por defeito da forma
bare de `MathDelimited` sem identidade para `MathLr`. Incide nos itens 1 e 2
da ADR-0127. Este commit contém somente passo e Prompts L0 e **PARA**; código,
testes, artefatos de ataque, headers e resselo começam apenas após confirmação
humana explícita.

## 6. Execução após confirmação

1. Congelar manifesto, contrato e oráculos para binding, assinatura, casts,
   presença, campos, convergência bare/qualificada, nesting e `repr`.
2. Confirmar RED atribuível e GREEN bilateral dos observáveis aplicáveis.
3. Implementar builder compartilhado e registro sem novo carrier ou fase.
4. Atacar remoção/troca de barras, retorno de `MathDelimited`, perda de
   presença, aceitação de extras/named e rota bare legada.
5. Exigir mutation score `1.0` nas ordens direta, repetida e inversa; qualquer
   `Unknown` não criado explicitamente como opaco bloqueia o selo.
6. Validar fmt, diff-check, build, suítes focal/integral e
   `crystalline-lint .`, com proveniência completa e certificado separado.
