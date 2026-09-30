# Prompt L0 — `compiler/stdlib/structural/par` — `par` e `parbreak`
Hash do Código: 79730d57

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/par.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/par.rs` — ficheiro próprio.

**Fronteira medida**: nó das nativas de parágrafo provenientes do mesmo arquivo
vanilla `model/par.rs`. Antes de P1140.17, `native_par` tinha **1** commit de corpo real
(`c98ffc8ac`, 2026-07-21 — o commit que a criou); o único commit que a liga a `quote` ou
`footnote` é o rename de módulo `0f5575cd0`. Critério 3 mudo, critério 4 separa — e é o
critério 4 que decide, porque o 3 está em **silêncio, não em contradição**.

> **Origem desta fronteira**: `par` vinha de um nó `flow` que agrupava `par`, `quote` e
> `footnote`. Esse agrupamento estava sustentado por um cluster que **não existia** —
> artefacto de atribuição de fronteira da ferramenta de co-mudança (em `c98ffc8ac` o único
> bloco de corpo é `@@ -655,0 +656,79 @@ pub fn native_quote(`: inserção pura de `par`
> depois de `quote`, que não mudou). Medido e desfeito em 2026-08-13.

---

## Contexto

`par(...)` é a face de função do parágrafo. O ponto que este nó tem de tornar visível é
que emite o nó Content::Par existente: o body preserva conteúdo e o wrapper de estilo
transporta as opções. A correção vigente abaixo substitui o antigo parágrafo transparente.

`parbreak()` é a face de função da quebra de parágrafo já representada por
`Content::Parbreak`. Não cria outro elemento nem outro algoritmo: devolve o mesmo marker
que `SyntaxKind::Parbreak` produz a partir de uma linha vazia em markup.

## Instrução

`par(body, leading: ?, justify: ?)` — 1 posicional `Content | Str` obrigatório.

- `Content` → body de Content::Par; `Str` → convertido em texto e usado no mesmo nó.
- `leading: Length` → embrulha o body em `Content::Styled` com o custom de leading.
- `justify` → boolean validado e transportado em custom `par.justify`; consumo de
  hifenização segue a correção vigente abaixo. A composição justificada ainda é incompleta.
- Sem body → erro com a **mensagem literal do vanilla**; tipo errado →
  `expected content`; nomeado desconhecido → erro.

### `parbreak()` — P1140.17

**Medição anterior à decisão** — vanilla ratificado `a51e02804`,
`2026-08-24T13:20:36-03:00`, HEAD cristalino
`45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitado:

```text
repr(type(parbreak))       → "function"
repr(parbreak())           → "parbreak()"
parbreak(1)                → error: unexpected argument
parbreak(foo: true)        → error: unexpected argument: foo
```

No mesmo estado, o cristalino devolve `unknown variable parbreak`. A fonte vanilla é
`lab/typst-original/crates/typst-library/src/model/par.rs:697-728`; `ParbreakElem` não
tem campos nem parâmetros.

**Decisão:** adicionar `native_parbreak() -> Content`, sem posicionais e sem nomeados.
A nativa retorna exactamente `Value::Content(Content::Parbreak)`. O binding global e
`std.parbreak` têm kind `function`. A linha vazia em markup continua pelo caminho
sintático existente; múltiplas quebras continuam a colapsar no consumer vigente.

Esta adição é o contrato público e o comportamento por defeito vigente.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- A mensagem de `par()` sem body é byte-a-byte a do vanilla: aqui **a mecânica é o
  observável** (ADR-0108), logo a string conta como contrato.
- Helpers privados puros de validação são permitidos pela correção vigente abaixo.
- `native_parbreak` usa a validação normal de `Args`, mas não partilha parsing com
  `native_par`: as assinaturas não têm campos em comum.
- Nenhuma mudança na estrutura de `Content`, realização de parágrafos ou layout é
  autorizada por P1140.17. O `repr` canônico muda de `parbreak` para `parbreak()`
  conforme o observável vanilla medido, sem campo de origem.

## Critérios de Verificação

```
#par[x]                → Content::Par com body x
#par("x")              → Par com body textual
#par(leading: 2pt)[x]  → Styled com leading custom
#par(justify: true)[x] → Styled com par.justify: true; composição ainda incompleta
#par()                 → Err (mensagem literal do vanilla)
#par(1)                → Err "expected content"
#par(zz: 1)            → Err (nomeado desconhecido)
repr(type(parbreak))   → "function"
repr(parbreak())       → "parbreak()"
parbreak(1)            → Err "unexpected argument"
parbreak(foo: true)    → Err "unexpected argument: foo"
parbreak(); parbreak() → mesmo colapso semântico de duas linhas vazias
```

## P1414 — função namespaced `par.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `df196b35b`, `par` é uma nativa sem namespace. No vanila ratificado,
`repr(par.line)` é `"line"`; `ParLine` é exclusivamente configurável por set
rules e sua chamada falha com `cannot be constructed manually`.

### Decisão proposta

O owner fornece `native_par_line`, sem construção de conteúdo: qualquer chamada
devolve o diagnóstico literal `cannot be constructed manually`. O binding
principal `par` continua chamável e recebe namespace fechado contendo `line`
com nome público curto. Nenhum `ParLineElem`, marker consultável ou variante de
`Content` é criado.

Os campos públicos da função são `numbering`, `number-align`, `number-margin`,
`number-clearance` e `numbering-scope`. Sua semântica de set/layout pertence aos
owners eval e Cursor. A adição pública aguarda confirmação humana explícita.

## P1415 — `numbering` funcional (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O vanila ratificado declara `ParLine.numbering` como `Option<Numbering>` e
documenta pattern ou função de um número em `model/par.rs:791-813`. Os oráculos
P1415 confirmam retorno `Content`, retorno `none`, reinício por página e erro de
aridade. P1414 aceita somente `string | none` e declarou este subconjunto
incompleto.

### Decisão proposta

O campo público `numbering` aceita `string | function | none`. A função recebe
exatamente o número lógico da linha confirmada e seu retorno segue a conversão
canônica para conteúdo; conteúdo forte não é achatado para string. O owner não
executa a função e não cria constructor manual: `par.line()` continua falhando
com `cannot be constructed manually`.

A realização pertence aos owners de eval, layout e pipeline. Esta ampliação de
contrato público aguarda confirmação humana explícita.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

No native_par, validar justify Bool e capturar par.justify junto com o leading existente; a fronteira do body é completada pela composição vigente abaixo. Validar linebreaks Auto|Str(simple|optimized) com os mesmos diagnósticos medidos. Usar span da última ocorrência nomeada ou args.span. Não implementar justificação, optimized ou fronteira explícita de par nesta unidade. Este constructor permanece incompleto: rodada-config-composicao completa realização/fronteira e estratégia depois da preparação do parágrafo. As cláusulas antigas de justify aceito/ignorado não são mais vigentes para seu transporte/validação; a propriedade já chega à política de hifenização. Helpers privados puros são permitidos, sem mudança de assinatura pública.

### Refinamento de diagnóstico medido antes da implementação

Os testes em first-policy-green.log passam transporte e política (5), mas falham quatro mensagens: type_mismatch retorna int em vez do integer medido no vanilla. O helper existente expected_length_error em eval/rules.rs:65 já usa esse vocabulário. Para estes novos casts, usar helper privado local com boolean/integer/string, sem alterar type_mismatch global nem type_name público. No constructor text, essa exceção explícita supersede somente a exigência de reutilizar type_mismatch para hyphenate. Nenhum contrato público muda.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

native_par retorna Content::par(body), ou Styled(styles, Par(body)) quando possui opções. Preservar validação/transporte de leading/justify/linebreaks e erros; não executar render, não mudar Args nem assinatura. Esta cláusula supersede a devolução transparente do body e completa sua fronteira/representação. spacing/first-line-indent e optimized seguem declarados incompletos para seus próprios consumers; rodada-paragrafo-seletores completa estratégia após preparação global.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Transportar linebreaks válido em styles.push_custom("par.linebreaks", value.clone()), inclusive Auto. Mesmos casts/spans e Content::par; nenhuma seleção nesta unidade.
