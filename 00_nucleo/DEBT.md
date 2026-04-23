# Inventário de dívida técnica

> Reorganização aplicada no Passo 83.5: três secções — abertos, encerrados,
> instrumentação. Texto de cada entrada preservado tal como estava antes da
> reorganização. Numeração descontínua é intencional (DEBT-4, DEBT-5 nunca
> existiram; DEBT-24a, DEBT-34a idem).

---

## Secção 1 — DEBTs em aberto ou parcialmente resolvidos

## DEBT-1 — StyleChain — PARCIALMENTE RESOLVIDO

### Resolvido no Passo 30

- **`StyleChain`** implementada em `entities/style_chain.rs` (L1)
- **`StyleDelta { bold, italic, size }`** como delta de herança
- **`#set text(bold:, italic:, size:)`** avaliado em `eval_expr` (Expr::SetRule)
- **`EvalContext::styles: StyleChain`** — cadeia activa durante eval
  (~~removido no Passo 94: agora propagado como `&mut StyleChain`
  parâmetro; ver secção "Actualização Passo 95" abaixo~~)
- **`TextStyle::from(&StyleChain)`** — bridge para layout/export actuais
- **`Content::Text(EcoString, TextStyle)`** — estilo capturado em eval
- **Strong/Emph/Heading** em eval: push/pop de estilos correcto

### Divergência intencional

- ~~`#set` é global ao eval (não tem scoping por bloco) — DEBT menor~~
  **Desactualizada (Passo 95).** O scoping por bloco foi adicionado no
  Passo 33 via save/restore em `CodeBlock`/`ContentBlock`, e
  reafirmado arquitectonicamente no Passo 94 (atomização de `styles`
  como parâmetro: cada bloco cria `local_styles` próprio; isolamento
  por construção).
- Apenas `text` como target suportado — outros targets ignorados silenciosamente
- StyleChain não integrada com `#show` rules (Passo futuro)
- Layout usa merge de node_style + self.style para compatibilidade com testes directos

### Pendente

- Propriedades adicionais (fill, font-family, weight numérico, etc.)
- Paridade total com o sistema de styles do original
- Remover os wrappers Content::Strong/Emph do layout quando eval os tiver totalmente substituído

**Ficheiros alterados**: `entities/style_chain.rs` (novo), `entities/mod.rs`,
`entities/content.rs`, `rules/eval.rs`, `rules/layout.rs`

### Nota — actualização no Passo 84.1

Duas pendências originais ("Scoping de `#set` por bloco" e "`#show` rules")
foram riscadas por terem sido resolvidas implicitamente por outros DEBTs
(DEBT-7 e DEBT-19/20 respectivamente). A auditoria do Passo 83.5
confirmou a presença do código correspondente em `eval.rs`. As
pendências remanescentes (propriedades adicionais, paridade, wrappers)
continuam em aberto.

### Actualização Passo 95 — revisão à luz da atomização do Passo 94

Tarefa A do Passo 95 classifica as pendências do DEBT-1:

- [x] **Scoping de `#set` por bloco** — resolvido pelo Passo 33
  (save/restore) e **reforçado pelo Passo 94** (atomização de
  `styles` como `&mut StyleChain` parâmetro; cada bloco constrói
  `local_styles` por construção — scoping torna-se intrínseco em
  vez de depender de save/restore manual sobre um campo partilhado).
- [x] **Arquitectura partilhada do `styles`** (implícita no item
  "`EvalContext::styles: StyleChain`" da lista de Passo 30) —
  resolvida no Passo 94 via extracção como parâmetro (ADR-0036
  segunda aplicação).
- [ ] **Propriedades adicionais** (fill, font-family, weight
  numérico, etc.) — **continua em aberto**. Ortogonal à
  atomização; exige extensão do `StyleDelta` e do `TextStyle`
  para novos campos.
- [ ] **Paridade total com o sistema de styles do original** —
  **continua em aberto**. Inclui o item anterior mais integração
  com show rules (DEBT-19/20), constant folding, etc.
- [ ] **Remover wrappers `Content::Strong/Emph` do layout** —
  **continua em aberto, sem mudança de natureza**. A atomização
  do eval não toca em layout; a decisão arquitectural permanece
  igual ao que era antes do Passo 94.

Sumário: 2 pendências implicitamente resolvidas pela atomização
(já não exigem trabalho específico); 3 pendências permanecem
legítimas. DEBT-1 permanece na Secção 1 — o trabalho residual
(propriedades, paridade, wrappers) não foi resolvido por
atomização.

---

## DEBT-2 — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO

### Resolvido no Passo 31

- `ClosureRepr::captured` mudou de `IndexMap<String, Value>` (clone eager O(N)) para `Arc<Scope>`
- Captura no momento da definição: snapshot O(N) uma única vez, depois partilhado em O(1)
- `apply_closure` usa `Scopes::with_parent(Arc::clone(&captured))` — lookup sem clone dos valores
- `eval_let` trata `LetBindingKind::Closure`: sintaxe `#let fib(n) = ...` agora funciona
- `Expr::Closure` arm lê `closure_expr.name()` — nome propagado correctamente para recursão

### Divergência residual

- Semântica de captura: ainda eager (snapshot). `#let x=1; #let f()=x; #let x=2; f()` retorna `1`
  (snapshot), não `2` (lazy). O original via `comemo` retornaria `2`.
  **Confirmado no Passo 31**: o snapshot é uma cópia independente do scope, não uma referência
  partilhada. Divergência semântica documentada com o original. Não bloqueante.
- A integração com `comemo` para tracking semântico real aguarda `TrackedWorld` real.
- Registado como sub-DEBT se cenários avançados de shadowing forem encontrados nos testes de paridade.

### Pendente

- Integração com `comemo` para tracking semântico real
- Testes de paridade com o original para cenários avançados de shadowing

---

## DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Parcialmente resolvido no Passo 37**. Registado no Passo 34.

**Resolvido no Passo 36**:

- `MathLayouter` criado em `rules/math/layout.rs` (L1 puro)
- `MathIdent` e `MathText` → `FrameItem::Text` (sem placeholder)
- `MathFrac`, `MathAttach`, `MathRoot` → texto plano sem `[...]`
- Placeholder `[equação]` removido do layouter principal
- `Content::Equation` delega ao `MathLayouter` para inline e block

**Resolvido no Passo 39**:

- `rules/math/symbols.rs` com `ident_to_unicode`, `shorthand_to_unicode`,
  `is_math_function`, `is_single_letter_var`
- `Expr::MathIdent` em `eval_math_expr`: símbolo conhecido → `Content::MathText(unicode)`
- `MathShorthand::get()` no AST já retorna o char Unicode — sem alteração necessária
- Variáveis de uma letra → `TextStyle { italic: true }` em `MathLayouter::layout_node`
- Funções (sin, cos, lim, etc.) → `TextStyle { italic: false }`

**Resolvido no Passo 38**:

- `FrameItem::Line { start, end, thickness }` adicionado a `layout_types.rs`
- `layout_frac` emite `FrameItem::Line` para a linha horizontal entre num/den
- `MathBox::place()` e `hconcat()` propagam offsets para `FrameItem::Line`
- `export.rs` trata `FrameItem::Line` com operadores PDF `q w m l S Q`
- `frac(a, b)` em `eval_math_expr` via `Expr::FuncCall` → `Content::MathFrac`
- `MathDelimited` em `eval_math_expr` inclui `open` e `close` como `MathText`

**Resolvido no Passo 37**:

- `MathBox { width, ascent, descent, items }` como unidade de layout
- `MathFrac`: numerador acima da baseline, denominador abaixo,
  tamanho 70% do texto base. Sem linha de fracção (Passo 38+)
- `MathAttach`: sup elevado a 50% do ascender, sub baixado a 30%
  da descida, tamanho 65% do texto base
- `hconcat`: concatenação horizontal de `MathBox`es
- `MathLayouter` refactorizado para stateless (`&self`)
- `layout_equation` retorna `Vec<FrameItem>` (posições relativas)
- Integração em `layout.rs` usa `y: offset_y + pos.y` para
  posicionamento vertical correcto

**Resolvido no Passo 40**:

- `sqrt(x)` como função nativa em `eval_math_expr` → `Content::MathRoot { index: None }`
- `root(n, x)` como função nativa → `Content::MathRoot { index: Some(n) }`
- Validação de aridade: `sqrt()` e `sqrt(x,y)` retornam `Err`; `root(3)` retorna `Err`
- `MathLayouter::layout_root`: símbolo `√`, overline (`FrameItem::Line`), radicando posicionado à direita
- `offset_item` helper adicionado em `math/layout.rs`
- `sqrt` e `root` adicionados a `is_math_function` (renderizados sem itálico)

**Ainda pendente**:

- Kern matemático entre símbolos
- Fontes OpenType MATH (tabelas MATH, variantes de tamanho)
- `MathPrimes` (parseado e evaluado em `eval.rs`, sem lógica de kern/posição no layouter)
- Baseline correcta em relação ao x-height da fonte

### Nota — actualização no Passo 84.1

`MathAlignPoint` foi removido da lista de pendências — verificação no
Passo 83.5 confirmou implementação completa em `math/layout.rs`,
`eval.rs` e `layout/mod.rs`. A entrada de `MathPrimes` foi clarificada
para indicar o estado parcial (parseado mas sem lógica de layout
dedicada).

---

## DEBT-9 — Cobertura de paridade — tracking contínuo

**Estado**: Baseline estabelecido no Passo 35. Sem divergências no corpus actual.

**Descrição**: O parity_runner testa 50 inputs (40 markup/code/math gerais + 10 math
específicos do Passo 34). Todos passam. À medida que o motor de equações e novas
funcionalidades forem implementadas, adicionar casos de paridade correspondentes.

**Quando expandir**: A cada passo que adicione novo SyntaxKind ou altere semântica do parser.

**Referência**: `lab/parity/tests/parse_parity.rs`, baseline em
`00_nucleo/materialization/parity-baseline-passo-35.md`

---

## DEBT-33 — Bounding Box de curvas Bézier (Passo 79) — EM ABERTO

A bounding box de `ShapeKind::Path` é calculada verificando o min/max dos pontos
de controlo. Para `CubicTo`, a curva real pode ultrapassar a caixa delimitadora
dos pontos de controlo, causando vazamento visual subtil. Resolução futura:
cálculo analítico dos extremos da curva paramétrica B(t) para obter a AABB exacta.

---

## DEBT-34d — Auto não encolhe antes de matar fr — EM ABERTO (Passo 80)

Um Auto guloso (célula com texto longo) pode consumir todo o `safe_available`,
deixando 0pt para as colunas fr. Resolução futura: implementar min-content e
max-content para Auto, com negociação entre Auto e fr.

---

## DEBT-34e — colspan e rowspan — EM ABERTO (Passo 80)

Células que ocupam múltiplas colunas ou linhas requerem um algoritmo de
placement diferente. Resolução: passo futuro.

---

## DEBT-35b — Invalidação de cache de available_width após SetPage — EM ABERTO (Passo 81)

Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.

---

## DEBT-42 — `get_unchecked` no scanner — EM ABERTO (Passo 84.8a, bloqueado)

`01_core/src/rules/lexer/scanner.rs` tem 7 ocorrências de
`unsafe { self.string.get_unchecked(start..end) }`. Herdado de
`unscanny` via ADR-0014.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito, com
excepção permitida permanentemente apenas se benchmark
reprodutível demonstrar regressão inaceitável ao eliminar o
`unsafe`, e se um ADR específico registar o número concreto.

### Bloqueio

Este DEBT depende de **infra de benchmarking reprodutível no
projecto**, que ainda não existe. Sem a infra, não é possível
aplicar o critério da ADR-0032 de forma honesta.

### Plano de resolução

1. **Pré-requisito**: criar infra de benchmark para lex e parse.
   Pode ser `criterion` crate (já usada na comunidade Rust) ou
   alternativa. ADR específico se a decisão sobre qual biblioteca
   envolver dependências novas em L1 ou se o benchmark vive fora
   de L1.

2. **Medição baseline**: com `get_unchecked` tal como está actual.
   Executar benchmark sobre conjunto de documentos representativos
   (documentos simples, documentos com muito texto, documentos
   matemáticos).

3. **Refactor experimental em branch**: substituir `get_unchecked`
   por `&self.string[start..end]` e medir.

4. **Decisão**:
   - Se regressão < 5% no tempo total de lex: eliminar `unsafe`,
     fechar DEBT.
   - Se regressão entre 5% e 20%: decisão do utilizador com base
     em contexto (documentos alvo, uso do Typst).
   - Se regressão > 20%: manter `unsafe` no scanner como excepção
     permanente, escrever ADR específico citando a medida.

### Critério de conclusão

Uma de duas:
- Zero ocorrências de `unsafe { get_unchecked(...) }` em `scanner.rs`.
- ADR específico escrito que autoriza permanência do `unsafe` com
  número concreto de regressão medida.

### Dependências

- Infra de benchmark — trabalho próprio, não coberto por este
  bloco de passos.

---

## DEBT-43 — Linter: whitelist crate-level em vez de type-level — EM ABERTO (Passo 89)

O `crystalline.toml` usa whitelist crate-level para externos
autorizados em L1 (secção `[l1_allowed_external]`). Isto significa
que, se uma crate tem ao menos um tipo autorizado por ADR, **qualquer**
tipo dessa crate passa o linter — mesmo tipos cujo uso não foi
autorizado.

Exemplo concreto identificado no Passo 87:

- ADR-0024 autorizou `ecow::EcoString` para `Value::Str` (pontual).
- ADR-0035 autoriza `ecow::EcoVec` (Passo 89) mas **não** `ecow::EcoMap`,
  `ecow::EcoArc` ou outros tipos da crate.
- `use ecow::EcoMap` passaria hoje o linter sem reportar violação,
  porque o nome `ecow` consta do array autorizado.

A disciplina de respeitar o escopo dos ADRs type-específicos é humana
(revisão de código), não automática. Este DEBT regista o gap. O
enforcement arquitectural continua efectivo via revisão manual e pela
cultura de materialização precedida por ADR + diagnóstico
(ADR-0033/0034), mas fica explícito que o guardião automático é
incompleto.

### Proposta de resolução

Estender o formato do `crystalline.toml` para aceitar whitelisting
por tipo:

```toml
[l1_allowed_external.ecow]
types = ["EcoString", "EcoVec"]
```

Em vez do actual `ecow` estar apenas listado no array de crates
autorizadas sem granularidade.

**Escopo em dois repositórios**:

1. **`crystalline-lint` (projecto separado, guardião arquitectural)**
   — alterar o parser da configuração e a lógica de verificação para
   ler e aplicar o novo formato type-level. Sem esta alteração, o
   novo formato no `crystalline.toml` é ignorado pelo binário actual.

2. **Typst cristalino (este repositório)** — migrar o
   `crystalline.toml` para o novo formato, crate por crate, partindo
   das autorizações já estabelecidas pelos ADRs (0010-0013 para
   crates unicode, 0018 para `rustc_hash`, 0023 para `indexmap`,
   0024 para `EcoString`, 0035 para `EcoVec`).

O trabalho no `crystalline-lint` é pré-requisito do trabalho neste
repositório — o `.toml` só tem efeito depois de o binário saber
interpretá-lo.

### Dependências

- Trabalho no repositório `crystalline-lint`: alteração do parser de
  configuração e da lógica de verificação. Passo dedicado nesse
  projecto, não neste.
- Após o `crystalline-lint` aceitar type-level whitelisting: passo
  neste repositório para migrar `crystalline.toml` para o novo
  formato e adicionar teste de violação negativa (tipo da mesma
  crate não listado deve ser reportado).

### Nota sobre escopo

Este DEBT **não** afecta correcção funcional do código Typst. Afecta
apenas enforcement automático de decisões arquitecturais. É trabalho
de infraestrutura, não de domínio.

### Critério de conclusão

- [ ] `crystalline.toml` aceita whitelist type-level para externos.
- [ ] Pelo menos uma crate (sugestão: `ecow`) migrada para o novo
      formato.
- [ ] Tipo não autorizado dessa crate é reportado como violação em
      teste do `crystalline-lint`.
- [ ] Documentação actualizada no README do `crystalline-lint`.

---

## DEBT-45 — Métodos `check_*_depth` de `Route<'a>` não chamados pelo eval — EM ABERTO (Passo 91)

O Passo 90 materializou `Route<'a>` com 4 métodos de verificação de
profundidade:

- `check_call_depth` (limite `MAX_CALL_DEPTH = 80`)
- `check_show_depth` (limite `MAX_SHOW_RULE_DEPTH = 64`)
- `check_layout_depth` (limite `MAX_LAYOUT_DEPTH = 72`)
- `check_html_depth` (limite `MAX_HTML_DEPTH = 72`)

Paridade estrutural com vanilla (ADR-0033). Mas apenas
`check_call_depth` tem equivalente chamado no cristalino actual —
via `EvalContext::check_call_depth()` da era pré-Route, independente
do `Route<'a>`.

Os 3 restantes são **código não executado**:

- `check_show_depth` — seria chamado em `apply_show_rules` ou
  equivalente; actualmente não chamado.
- `check_layout_depth` — seria chamado no braço de layout recursivo;
  actualmente não chamado.
- `check_html_depth` — seria chamado no pipeline HTML (que não
  existe no cristalino ainda).

### Impacto

- Zero impacto funcional imediato — os métodos existem mas não são
  invocados; nenhuma profundidade é verificada onde não era
  verificada antes.
- Impacto latente: bugs de recursão infinita em show rules ou
  layout não são capturados pelo limite declarado em `Route`. O
  cristalino continua vulnerável aos mesmos stack overflows que o
  limite do vanilla evita.

### Critério de conclusão

- [x] `check_show_depth` chamado no ponto correspondente em
      `rules/eval.rs` ou `rules/show.rs`, consistente com o vanilla.
- [ ] `check_layout_depth` chamado no ponto correspondente em
      `rules/layout/`, consistente com o vanilla.
- [ ] `check_html_depth` chamado quando o pipeline HTML existir
      (não antes — aguarda materialização do pipeline).
- [x] `EvalContext::check_call_depth` antigo pode ser removido ou
      re-encaminhado para `Route::check_call_depth` — decisão adiada
      para o passo que atacar este DEBT.
- [x] Testes que exercitam cada limite passam sem alteração da
      asserção (comportamento observável preservado).

### Dependências

- DEBT-44 preferencialmente resolvido antes — faz mais sentido
  `EvalContext` usar `Route<'a>` estruturalmente antes dos `check_*`
  serem integrados. Resolver DEBT-45 sem DEBT-44 é possível mas cria
  dependência artificial sobre o mecanismo antigo.

### Nota sobre escopo

`check_html_depth` só é accionável quando o pipeline HTML for
materializado. Este DEBT pode ser parcialmente pago (3 limites
integrados) e manter o quarto aberto até então.

### Estado actual (após Passo 93)

**Parcialmente pago.** 2 dos 4 `check_*_depth` integrados, 1 adiado,
1 pendente do HTML:

- [x] `check_show_depth` — chamado em `apply_show_rules`
  (`01_core/src/rules/eval.rs:1428`), antes de processar as regras.
  Paridade com `typst-realize/src/lib.rs:402` do vanilla.
- [ ] `check_layout_depth` — **adiado**. A pipeline de layout do
  cristalino (`01_core/src/rules/layout/mod.rs::layout`) opera
  sobre `Content` já avaliado, sem receber `Route<'a>`. Propagar
  `route` para o `Layouter` é refactor análogo ao do Passo 92 mas
  num submódulo diferente; encaixa melhor no passo que
  materializar `Engine<'a>` (que alinha `route` + layout num só
  contexto). Fica para passo dedicado.
- [x] `check_call_depth` — chamado em `apply_closure`
  (`01_core/src/rules/eval.rs:1069`), antes de bindar parâmetros.
  Paridade com `typst-eval/src/call.rs:33`. **Opção 2** aplicada:
  `EvalContext::check_call_depth`, `enter_call`, `leave_call`, e
  os campos `depth`/`max_call_depth` foram **removidos**. Além
  disso, o helper `eval_for_test_with_limits` perdeu o parâmetro
  `max_call_depth` (4 call-sites actualizados); os testes de
  recursão continuam a passar porque Route's 80 > 50 do limite
  antigo — a assertion `is_err()` + mensagem `"depth"` cobre
  ambos os mecanismos.
- [ ] `check_html_depth` — pendente. Integração só é possível
  quando o pipeline HTML for materializado (trabalho futuro).

Mais: por motivos de arquitectura do `comemo 0.4.0`, os 4
`check_*_depth` foram refactorados de **métodos** `Route::check_*`
para **funções livres** `world_types::check_*(Tracked<Route>)`.
Razão: `Tracked<T>` só expõe métodos do bloco `#[comemo::track]`
e as verificações precisam de construir `SourceDiagnostic` (tipo
não-memoizável). As funções livres delegam a lógica ao `within`
(que é tracked) e constroem o diagnóstico externamente.

Passo 93 adicionou também o diagnóstico
`00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`
(Tarefa A) a documentar o padrão
`<T<'static> as Validate>::Constraint` descoberto no Passo 92.

Encerramento completo do DEBT-45 fica para quando
`check_layout_depth` for integrado (Passo próprio ou parte da
materialização do `Engine<'a>`) e `check_html_depth` quando o
pipeline HTML existir.

---

## DEBT-46 — Ficheiros de L1 com coesão baixa por tamanho excessivo — EM ABERTO (Passo 96)

Seis ficheiros em `01_core/src/` excedem 1000 linhas e misturam
responsabilidades de domínios distintos. A análise realizada antes
do Passo 96 revelou:

| Linhas | Ficheiro |
|--------|----------|
| 3780 | `01_core/src/rules/eval.rs` |
| 2848 | `01_core/src/rules/layout/mod.rs` |
| 2255 | `01_core/src/rules/parse.rs` |
| 1806 | `01_core/src/rules/math/layout.rs` |
| 1711 | `01_core/src/rules/stdlib.rs` |
| 1250 | `01_core/src/rules/lexer/mod.rs` |

Total: 13.650 linhas em seis ficheiros. O `eval.rs` sozinho tem
368 ocorrências de padrões `match` sobre `Expr::`, `SyntaxKind::`
e `Value::` — é um dispatcher central para toda a lógica de
avaliação, misturando markup, matemática, closures, imports,
regras show/set e controlo de fluxo.

Este DEBT documenta o inventário concreto da reestruturação
necessária pela ADR-0037 (`EM VIGOR` desde Passo 96.3 após
validação empírica nos Passos 96.1–96.2).

### Motivação

A ADR-0036 (atomização progressiva) reduziu acoplamento dentro
de funções (Passos 92–95) mas não reduziu tamanho dos ficheiros.
O `eval.rs` diminuiu parcialmente após as extracções de `route`,
`styles`, `show_rules`, `active_guards`, mas continua acima de
3700 linhas.

A ADR-0037 complementa a ADR-0036 ao orientar decomposição
**entre ficheiros**, não só dentro de funções.

### Critério de conclusão

Renumeração aplicada no Passo 96.3 após introdução do 96.2
(delegação dos armos) que deslocou os sub-passos seguintes em
uma posição:

- [x] `eval.rs` reestruturado em submódulos por domínio (math,
      operators, control_flow, closures, bindings, rules,
      markup, modules, tests). **Passo 96.1 concluído** — 7
      submódulos criados inicialmente (math/operators/
      control_flow/closures/bindings/rules), mais tests.rs em
      ficheiro separado. `markup.rs` e `modules.rs` adicionados
      no Passo 96.2.
- [x] Delegação completa dos armos longos do dispatcher
      `eval_expr`. **Passo 96.2 concluído** — `mod.rs` caiu
      para 520 linhas, dispatcher compacto com armos de 1 linha
      para casos de domínio; excepção Regra 6 removida do
      `mod.rs`. `tests.rs` mantém nota Regra 6 (cfg(test) gated).
- [x] ADR-0037 promovida de `PROPOSTO` para `EM VIGOR` com 4
      ajustes (A/B/C/D) validados empiricamente nos Passos
      96.1–96.2. **Passo 96.3 concluído neste passo.**
- [x] `parse.rs` reestruturado por tipo de nó (markup, code,
      math, rules). **Passo 96.4 concluído** — 7 submódulos
      criados: `parser` (Parser struct + tipos de apoio, 700
      linhas), `math` (~377), `markup` (~205), `code` (~263),
      `rules` (~235), `patterns` (~485); `mod.rs` caiu para
      156 linhas (só entry points + declarações de submódulos
      + testes); nenhum submódulo excede 800 linhas. L1 tests:
      746 → 748 (+2 smoke tests em parser.rs para satisfazer V2).
- [ ] `stdlib.rs` reestruturado por módulo da stdlib (text,
      layout, math, calc, etc.). (Passo 96.5)
- [ ] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). (Passo 96.6)
- [ ] `math/layout.rs` reestruturado ou marcado como excepção
      Regra 6. (Passo 96.7)
- [ ] `lexer/mod.rs` reestruturado ou marcado como excepção
      Regra 6. (Passo 96.8)
- [ ] Verificação final: `find 01_core/src -name "*.rs" | xargs wc -l |
      sort -rn | head -10` mostra ficheiros acima de 800 linhas
      só com excepções Regra 6 documentadas. (Passo 96.9 ou
      encerramento implícito em 96.8)

### Dependências

Nenhuma técnica. O trabalho é mecânico (movimentação de código
por domínio), não requer decisões arquitecturais adicionais.

### Nota sobre escopo

O DEBT aplica-se apenas a `01_core/src/`. Ficheiros em `02_shell/`,
`03_infra/`, `04_wiring/` não estão no escopo — se excederem o
limite, abrir DEBT específico por camada.

### Nota sobre encerramento

Este DEBT fecha quando os 7 ficheiros listados tiverem sido
tratados (reestruturados ou marcados como excepção). Encerramento
parcial não é aceitável — a coerência do princípio ADR-0037
requer aplicação consistente.

Se um dos ficheiros resistir à decomposição por razões técnicas
descobertas durante a execução, registar como **excepção Regra 6**
em vez de deixar como dívida pendente. A Regra 6 existe
precisamente para estes casos.

---

## Secção 2 — DEBTs encerrados

## DEBT-3 — Safety rails hardcoded — RESOLVIDO (estrutura)

### Resolvido no Passo 28

- **`while` limit**: 10.000 → 1.000.000, via `EvalContext::tick_loop()`
- **`MAX_CALL_DEPTH`**: 200 → 250, via `EvalContext::check_call_depth()`
- **Limites documentados em `EvalContext`**: não mais magia inline
- **Métodos de verificação**: `check_call_depth()` e `tick_loop()` implementados
- **Limite global de loop**: contador acumulado ao longo de toda eval, não por loop

### Resolvido no Passo 29

- **Detecção de ciclos de importação**: `EvalContext::enter_import()` + `ImportGuard`
- **`import_stack: Vec<FileId>`**: rastreamento de ficheiros em avaliação
- **`ModuleImport` e `ModuleInclude`**: retornam Err limpo (não panic)

### Pendente (não é DEBT — é feature futura)

- **Implementação de `import` completo**: Passo 33+
- **Integração com `comemo`**: tracking semântico real (aguarda TrackedWorld real)

### Ficheiros alterados

- `01_core/src/rules/eval.rs`: `EvalContext` (max_call_depth 250, import_stack), `ImportGuard`, `ModuleImport`/`ModuleInclude` handling
- `01_core/src/rules/eval.rs`: testes novos de import_stack e ModuleImport/Include

---

## DEBT-6 — eval_for_test coverage blind spot — RESOLVIDO

### Registado e resolvido no Passo 32

**Problema**: `eval_for_test` usa `MockWorld` — um mundo artificial que não passa pelo mecanismo
de tracking real (`TrackedWorld`). Os testes de L1 nunca exercitavam o caminho de código de produção.

**Resolvido no Passo 32**:

- Testes de integração em `03_infra/src/integration_tests.rs`
- Pipeline completo exercitado: `SystemWorld` → `eval` → `layout` → `export_pdf`
- `eval()` pública confirmada como genérica sobre `TrackedWorld`
- `eval_for_test` mantida para testes unitários rápidos de L1

**Cobertura adicionada**:

- `pipeline_texto_simples`: eval + layout via SystemWorld
- `pipeline_export_pdf_helvetica`: export com fallback Helvetica
- `pipeline_export_pdf_com_fonte_real`: export com fonte do sistema (ou fallback)
- `pipeline_com_set_text_bold`: StyleChain via pipeline real
- `pipeline_com_closures`: closures via pipeline real
- `pipeline_eval_retorna_err_em_sintaxe_invalida`: robustez a input inválido

---

## DEBT-7 — Merge bold em layout — RESOLVIDO

**Registado no Passo 32. Resolvido no Passo 33.**

**Resolução**:

- Save/restore de `ctx.styles` em `Expr::CodeBlock` (`#{ }`)
- Save/restore de `ctx.styles` em `Expr::ContentBlock` (`[ ]`)
- Save/restore de `ctx.styles` em `apply_closure` (body de closures)
- Merge `bold || node_style.bold` e `italic || node_style.italic` removidos de `layout.rs`
- `#set text(bold: false)` dentro de um bloco agora reverte correctamente ao sair do bloco
- `node_style` capturado em eval já inclui o estilo correcto de Strong/Emph/Heading

**Ficheiros alterados**: `rules/eval.rs`, `rules/layout.rs`

---

## DEBT-10 — Contadores em duas passagens — RESOLVIDO

**Registado no Passo 57. Encerrado no Passo 62.**

**Resolução (Passagens 1 e 2)**:

- `01_core/src/rules/introspect.rs` criado: pré-passagem analítica que percorre
  `Content` sem alocações visuais, popula `resolved_labels: HashMap<Label, String>`.
- `layout()` recebe `CounterState` externo — o orquestrador chama `introspect` primeiro.
- Forward refs (`@conclusao` antes do heading `= Conclusão <conclusao>`) resolvem.
- Backward refs continuam a funcionar.
- `Content::Figure` implementado no Passo 62: introspecção rastreia figuras na
  Passagem 1, Layouter desenha blocos numerados na Passagem 2.

**Ficheiros criados/alterados**:

- `01_core/src/rules/introspect.rs` (novo)
- `01_core/src/entities/counter_state.rs` — `resolved_labels`, `headings_for_toc`, `auto_label_counter`
- `01_core/src/rules/layout/mod.rs` — `layout()` com estado externo
- `01_core/src/rules/layout/figure.rs` — braço `Content::Figure` (Passo 62)

---

## DEBT-11 — Decomposição de `layout.rs` — RESOLVIDO

**Registado no Passo 60. Resolvido no Passo 61.**

**Resolução**:

- `layout.rs` (1408 linhas) convertido em `layout/mod.rs` (orquestrador).
- `rules/layout/counters.rs`: braços `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
- `rules/layout/references.rs`: braços `Ref` e `Labelled`.
- `rules/layout/outline.rs`: braço `Content::Outline` (TOC).
- L0 criados: `layout_counters.md`, `layout_references.md`, `layout_outline.md`.
- `layout()` passa a receber `CounterState` externo — o orquestrador chama `introspect` primeiro.

---

## DEBT-12 — Números de página na TOC — RESOLVIDO (Passo 63)

**Registado no Passo 61. Resolvido no Passo 63.**

**Resolução**: orquestração em 3 passagens no `compile_to_pdf` de L3:

- Passagem 1 (introspecção): `introspect()` popula `resolved_labels` e `headings_for_toc`.
- Passagem 2 (draft): `layout()` regista `label_pages` via `layout_labelled` e expõe-o em
  `PagedDocument::extracted_label_pages`. A TOC ainda não tem números de página.
- Passagem 3 (final): `layout()` com `initial_state.label_pages` preenchido; a TOC lê
  os números reais de `label_pages` para cada linha.

**Limitação residual**: registada como DEBT-17 (fixpoint da TOC).

---

## DEBT-13 — Efeitos colaterais duplicados na TOC — RESOLVIDO (mitigado, Passo 63)

**Registado no Passo 61. Mitigado no Passo 63.**

**Mitigação**: flag `CounterState::is_readonly` activa durante a renderização de cada
linha da TOC em `outline.rs`. Enquanto `is_readonly = true`, os métodos
`step_flat`, `step_hierarchical` e `update_flat` são no-ops — `CounterUpdate` embebido
nos clones de heading não avança os contadores.

**Limitação residual**: `CounterDisplay` ainda lê estado incorrecto na TOC (lê valores
do momento em que a TOC é renderizada, não do momento do heading real). Registado
como DEBT-18 (perda de contexto temporal em AST clonado na TOC).

---

## DEBT-14 — SetRule para `#set figure(numbering: ...)` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A numeração de figuras está activa por defeito (`CounterState::new()` insere
`"figure" = true`). O utilizador não consegue desactivá-la com
`#set figure(numbering: none)` até que o braço de SetRule para "figure" seja
adicionado ao `eval.rs`, produzindo um nó equivalente a `SetHeadingNumbering`.
Quando implementado, figuras sem numeração cujas labels forem referenciadas
mostrarão o fallback `@label` (comportamento intencional — ver braço `Labelled`
em `introspect.rs`).

---

## DEBT-15 — Campo `kind` hardcoded em `Content::Figure` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A chave `"figure"` está hardcoded em `step_flat("figure")` tanto na introspecção
como no layout. No Typst original, `#figure` aceita um argumento `kind`
(ex: `image`, `table`, `code`), e cada kind tem contador próprio —
"Tabela 1" e "Figura 1" são independentes. Com a implementação actual, tabelas
e gráficos partilham o mesmo contador.

**Resolução futura**: adicionar campo `kind: String` (default `"figure"`) a
`Content::Figure` e usar `step_flat(&kind)` em vez da string fixa.

---

## DEBT-16 — Acoplamento do Avaliador à Stdlib — RESOLVIDO (Passo 64)

**Registado no Passo 62. Resolvido no Passo 64.**

**Resolução**:

- `NativeFunc.call` mudou de `fn(&[Value])` para `fn(&Args)` — aceita positional e named args.
- `apply_func` passa `&args` directamente: `(native.call)(&args)`.
- Função auxiliar `expect_no_named()` adicionada em `stdlib.rs` — funções que não aceitam
  named args retornam `Err` semântico (não silencioso).
- `native_figure` migrada do interceptador hardcoded em `eval.rs` para `stdlib.rs`.
  `eval.rs` não contém nenhuma referência ao nome "figure".
- Cascata de 17 funções actualizadas (8 stdlib + 9 calc).
- Testes de `stdlib.rs` actualizados para usar `Args::positional(...)` em vez de `&[Value]`.

---

## DEBT-17 — Fixpoint da TOC — RESOLVIDO (Passo 65)

**Registado no Passo 63. Resolvido no Passo 65.**

**Resolução**: o algoritmo de fixpoint foi movido para dentro de `layout()` em L1.
O ciclo corre até que `extracted_label_pages` não mude entre iterações (máximo 5).
O orquestrador L3 (`compile_to_pdf`) voltou a ser linear:
`introspect()` → `layout()` → `export_pdf()`.

**Separação leitura/escrita**: `CounterState` tem dois campos distintos:

- `label_pages`: escrito por `references.rs` em cada iteração (começa vazio via `Layouter::new()`).
- `known_page_numbers`: lido por `outline.rs`, injectado pelo fixpoint com o mapa da iteração anterior.

`CounterState::has_outline`: sinalizado por `introspect()` na presença de `Content::Outline`.
Sem `has_outline`, `layout()` usa short-circuit de passagem única (sem fixpoint).

---

## DEBT-18 — Perda de Contexto Temporal em AST Clonado na TOC — RESOLVIDO (Passo 66)

**Registado no Passo 63. Resolvido no Passo 66.**

**Resolução**: materialização de AST na Passagem 1 (Introspecção).
A função `materialize_time(content, state)` em `introspect.rs` percorre o AST do
body de cada título e substitui cada `CounterDisplay` pelo seu valor em texto
estático (`Content::Text`) usando o `CounterState` actual naquele momento exacto.

`headings_for_toc.push` passou de `*body.clone()` para `materialize_time(body, state)`.

`CounterState::display_value(kind)` centraliza a lógica de leitura (hierárquico
vs plano) usada por `materialize_time` e por `layout/counters.rs`.

---

## DEBT-19 — Avaliação superficial de NodeKind — **ENCERRADO (Passo 69)**

`map_content` bottom-up implementado em `content.rs`. `apply_show_rules`
reescrito para usar `map_content` em seletores de nó — a árvore inteira
é percorrida. `NodeKind` expandido com Strong, Emph, Raw, Equation, ListItem.
`intercept_content` adicionado a `Expr::Strong` e `Expr::Emph` além de
`Expr::Heading` e `Expr::FuncCall`.

Substituição Texto→Content via Func/Content continua a gerar Err explícito.

---

## DEBT-20 — Guard anti-recursão booleano global — **ENCERRADO (Passo 70)** ✓

Substituído por `active_guards: Vec<RuleId>` no `EvalContext`. Pilha de
regras activas permite composição de regras sem stack overflow — a regra
cujo ID está na pilha é saltada, outras regras continuam a actuar.

---

## DEBT-21 — Resolução de NodeKind por string — **ENCERRADO (Passo 84.3)** ✓

**Registado no Passo 70. Mitigado no Passo 70. Desbloqueado no Passo 84.1.
Resolvido no Passo 84.3.**

A construção do selector em `eval_expr` (`Expr::ShowRule`) mapeava
`Value::Func` → `NodeKind` por comparação textual: `match f.name() {
Some("heading") => NodeKind::Heading, ... }`. Aliasing por wrapper closure
ou re-registo da mesma nativa com nome diferente não era detectado, e a
sintaxe estável de `fn_addr_eq` requeria Rust ≥ 1.85.

**Resolvido no Passo 84.3.** Substituído pelo método `Func::native_fn_addr()
-> Option<fn(...)>` (retorna `None` para closures, `Some(call)` para
nativas). A construção do selector usa `std::ptr::fn_addr_eq` para comparar
o ponteiro com cada nativa correspondente:

```rust
match f.native_fn_addr() {
    Some(addr) if fn_addr_eq(addr, native_heading as fn(_,_) -> _) =>
        Selector::NodeKind(NodeKind::Heading),
    // ... idem para figure, strong, emph, raw
    Some(_) => Err(...),  // nativa não suportada
    None    => Err(...),  // closure rejeitada explicitamente
}
```

`Func::name()` mantido para apresentação (mensagens de erro, debug) — não
mais para identidade. Sem `unsafe`. Suporte para `equation`/`list_item`
removido do match (não existem `native_*` correspondentes); pode ser
adicionado quando essas nativas forem registadas na stdlib.

Testes de regressão: `show_rule_resolve_por_identidade_nao_por_nome`
(aliasing via `#let h = heading; #show h: ...`),
`show_rule_closure_anonima_rejeitada` (closure como selector → Err
explícito).

---

## DEBT-22 — Clone de show_rules por nó — **ENCERRADO (Passo 84.4)** ✓

**Registado no Passo 68. Resolvido no Passo 84.4.**

`ctx.show_rules` migrada de `Vec<ShowRule>` para `Arc<[ShowRule]>` em
`EvalContext`. O clone em `intercept_content` (uma vez por nó AST visitado)
deixa de ser O(N) sobre os bytes da lista e passa a ser O(1) — só incrementa
o refcount do `Arc`. `Arc::clone(&ctx.show_rules)` substitui `clone()` no
hot path para sinalizar a intenção explicitamente.

Push (`#show` do utilizador) e truncate-back (saída de `Expr::CodeBlock`)
reconstroem o slice e ficam O(N), mas são caminhos frios — ordens de
magnitude menos frequentes que o clone do hot path.

Helpers em `EvalContext`:
- `push_show_rule(&mut self, ShowRule)` — encapsula a reconstrução por push.
- `truncate_show_rules(&mut self, len: usize)` — substitui `Vec::truncate`
  (`Arc<[T]>` não tem método equivalente). Reconstrói via `Arc::from(&[..len])`.

API de leitura inalterada: `Arc<[T]>` deriva `Deref<Target=[T]>`, portanto
`iter()`, `len()`, `is_empty()`, indexação e referência (`&rules`) continuam
a funcionar. `apply_show_rules(content, &rules, ctx)` aceita `&[ShowRule]`
sem alteração.

Padrão consistente com ADR-0026 revisão (`Arc<[Content]>` em
`Content::Sequence`). Novos campos com perfil semelhante (lista imutável
após construção, partilhada entre clones frequentes) devem seguir o mesmo
padrão.

`active_guards: Vec<RuleId>` ficou intencionalmente fora deste passo —
push/pop frequente sem clone é o caso onde `Arc<[T]>` regrediria em vez
de melhorar. Registado em DEBT-39 (Secção 1).

---

## DEBT-23 — Travessia múltipla em apply_show_rules — **ENCERRADO (Passo 70)** ✓

**Registado no Passo 69.**

`apply_show_rules` percorria a árvore uma vez por `ShowRule` activa: O(R×N)
com R regras e N nós. Resolução planeada: `map_content` chamado uma única
vez com todas as regras testadas por nó dentro da closure, implicando
mudança de assinatura de `map_content` para aceitar uma lista de regras em
vez de uma closure genérica.

**Resolvido no Passo 70.** `apply_show_rules` chama `map_content` uma
única vez para todas as regras `NodeKind`. Dentro da closure bottom-up,
todas as regras activas são testadas por nó antes de prosseguir — custo
reduzido de O(R×N) para O(N).

---

## DEBT-24b — Dimensões reais de imagem — **ENCERRADO (Passo 72)** ✓

Placeholder 100×100 substituído por leitura real de dimensões via trait
`ImageSizer` injectado no layouter. L3 implementa com `imagesize`;
L1 mantém-se puro (zero dependências externas adicionais).

---

## DEBT-24c — FrameItem::Image e export PDF — **ENCERRADO (Passo 73)** ✓

`FrameItem::Image` adicionado ao layouter e ao exportador PDF. JPEG embutido
com `/DCTDecode`. Deduplicação por `Arc::as_ptr`. Espaço reservado no layout
e imagem emitida no PDF para ficheiros JPEG.

---

## DEBT-25 — Resolução de caminhos relativos em stdlib (Passo 71) — **ENCERRADO (Passo 75)** ✓

`native_image` passa o caminho ao `World::read_bytes` tal como fornecido pelo
utilizador (relativo à raiz do projecto). Não há resolução relativa ao ficheiro
fonte activo. Resolução: `EvalContext` deve expor o path do ficheiro corrente
para que `native_image` construa um caminho absoluto.

---

## DEBT-26 — PartialEq O(N) sobre Arc<Vec<u8>> em Content::Image — **ENCERRADO (Passo 74)** ✓

A implementação manual de `PartialEq` para `Content::Image` compara `data`
por valor (`da == db`), o que é O(N) nos bytes da imagem. Em contextos de
teste ou deduplicação frequente, usar `Arc::ptr_eq` primeiro e cair para
comparação por valor apenas se os ponteiros diferirem.

---

## DEBT-27 — Suporte a transparência PNG no exportador PDF — **ENCERRADO (Passo 74)** ✓

PDF não suporta ficheiros PNG crus. PNG requer descodificação de píxeis
(canal RGBA separado para /SMask de transparência) antes de ser embutido.
Prova de conceito do Passo 73 usa apenas JPEG (/DCTDecode).
Resolução: Passo 74 adiciona `crate image` a L3 para descodificar PNG
em RGBA plano e passar ao gerador de PDF.

---

## DEBT-28 — Dupla leitura de cabeçalho de imagem no layouter — **ENCERRADO (Passo 74)** ✓

`calculate_dimensions` e `sizer.size()` lêem o cabeçalho da imagem separadamente.
O custo é mínimo (imagesize lê apenas o cabeçalho, não os píxeis), mas é
redundante. Resolução futura: `calculate_dimensions` retorna `ImageDimensions`
com `intrinsic_width`, `intrinsic_height` — eliminando a segunda chamada.

---

## DEBT-29 — Detecção de ColorSpace para JPEGs crus — **ENCERRADO (Passo 74)** ✓

O exportador assume `DeviceRGB` para todos os JPEGs. JPEGs Grayscale (1 canal)
ou CMYK (4 canais) com ColorSpace errado produzem lixo visual ou rejeição pelo
leitor PDF. Resolução: Passo 74 lê o marcador SOF0/SOF2 do cabeçalho JPEG para
determinar o número de canais e escolher `DeviceRGB`, `DeviceGray` ou `DeviceCMYK`.

---

## DEBT-30 — Suporte a clipping paths (clip: true) — ENCERRADO ✓ (Passo 79)

Contentores com `clip: true` requerem o operador `W` (clipping path) e a sequência
`W n` antes de desenhar o conteúdo interno. O PDF mantém um clipping path activo
por estado gráfico (`q`/`Q`), portanto `clip: true` exige um push/pop de estado
adicional. Campo `clip_mask: Option<ShapeKind>` adicionado a `FrameItem::Group`.
O exportador emite `W n` no espaço local do Group, após a matriz `cm`.

---

## DEBT-31 — Transformações afins (rotate, scale) em nós — **ENCERRADO (Passo 78)** ✓

Rotação e escala requerem a matriz `cm` com valores não-identidade:
`[cos -sin sin cos tx ty]` para rotação. A bounding box de um nó rodado não é
um rectângulo alinhado aos eixos — o layouter precisaria de calcular o
bounding box transformado. `Ellipse` usa rectângulo placeholder no exportador.
Resolução: passo futuro de transformações.

---

## DEBT-32 — Alinhamento da bounding box para linhas com deltas negativos — **ENCERRADO (Passo 77)** ✓

O exportador desenhava a linha a partir de `pos.x` sem considerar o sinal de `dx`/`dy`.
Se `dx < 0`, a linha saía para a esquerda da bounding box. A correcção mapeia
início/fim dentro da bounding box com base no sinal dos deltas.

---

## DEBT-34b — Parâmetro rows em Content::Grid ignorado — **ENCERRADO (Passo 83)** ✓

`Content::Grid` passa a respeitar `rows: Vec<TrackSizing>`. O motor de layout
implementa três passagens (Fixed → Auto → Fraction) espelhando a resolução de
colunas, com indexação cíclica `N % rows.len()` e decisão de paginação antes da
fase Fraction (evita alturas `fr` "fósseis" da página anterior).

---

## DEBT-34c — Alinhamento vertical de células no Grid — **ENCERRADO (Passo 83)** ✓

`cell_height` é agora passado como `available_h` a `resolve_alignment` para
items dentro de células (campo `cell_available_h: Option<f64>` no Layouter).
`VAlign::Bottom` ancora ao limite inferior da célula e `VAlign::Horizon` centra
verticalmente.

---

## DEBT-36 — Operadores simbólicos de alinhamento — **ENCERRADO (Passo 84.5)** ✓

**Registado no Passo 82. Resolvido no Passo 84.5.**

`Value::Align(Align2D)` adicionado ao enum `Value`. Constantes top-level
`left`, `center`, `right`, `top`, `horizon`, `bottom` registadas em
`make_stdlib()` como valores de `Align2D`. `eval_binary_op` trata
`BinOp::Add` entre dois `Value::Align` com **semântica vanilla**: combina
componentes de eixos distintos (`center + bottom` → Both) e devolve `Err`
em conflito de eixo (`center + right` → erro "cannot add two horizontal
alignments"). Esta semântica diverge da sugestão original do passo
("`b.h.or(a.h)` com sobrescrita") em favor da fidelidade ao vanilla,
conforme autorizado pelo enunciado.

Sintaxe preferida: `align(center + bottom, ...)`, `place(top + right, ...)`.

Sintaxe legacy `align("center", ...)` preservada — `Align2D::from_string`
continua a ser usada como fallback em `native_align` e `native_place` via
helper `extract_alignment(args, default)`. Remoção da sintaxe legacy e dos
4 testes L3 que ainda a usam fica para passo dedicado posterior.

Divergência estrutural não tratada: `Align2D` é struct + Option (permite
`(None, None)` "vazio"), enquanto vanilla `Alignment` é enum tagged
`{ H | V | Both }` (não permite vazio). Conversão para enum tagged está
fora do escopo deste passo.

Testes de regressão: `align_plus_combina_eixos_distintos`,
`align_plus_eixo_horizontal_repetido_falha`, `align_plus_eixo_vertical_repetido_falha`
(L1, unitários sobre `eval_binary_op`); `align_aceita_constante_simbolica`
e `align_aceita_composicao_via_plus` (L3, pipeline real).

---

## DEBT-37 — Place relativo ao contentor pai — **ENCERRADO (Passo 84.6)** ✓

**Registado no Passo 82. Resolvido no Passo 84.6.**

A descrição original do DEBT estava parcialmente incorrecta: o eixo X de
`Content::Place` já ancorava à coluna desde o Passo 81.5 (via
`line_start_x`); só o eixo Y era absoluto à margem da página.

**Diagnóstico (Passo 84.6):** o vanilla tem `enum PlacementScope { Column
(default), Parent }`. `Column` ancora ao "current container" (`regions.base()`)
— célula activa quando dentro de Grid; `Parent` ancora à página
("spans columns") e é restrito a `float: true`.

**Resolvido no Passo 84.6 (cenário A do enunciado):**

- `PlaceScope { Column (default), Parent }` adicionado em `layout_types.rs`.
- Campo `scope: PlaceScope` adicionado a `Content::Place`. Cascata em
  `PartialEq`, `map_content`, `map_text`, `introspect::materialize_time`.
- Layouter recebeu 3 campos novos: `cell_origin_x`, `cell_origin_y`,
  `cell_origin_w` (`Option<f64>`), em paralelo ao `cell_available_h` do
  Passo 83. Save/restore por célula no braço Grid.
- Braço `Content::Place` selecciona área de ancoragem por scope: Column
  com cell_* todos Some → célula; Column sem cell_* (fora de Grid) →
  página; Parent → sempre página.
- Compensação Y para evitar dupla translação no sub_frame de célula:
  quando `cell_origin_y.is_some()`, subtrai `cell_origin_y` em vez de
  `sub_origin_y` ao transferir items — anula a translação posterior do
  Grid (`row_start_y + (item.y - ascender_local)`).
- `native_place` aceita argumento nomeado `scope: "column" | "parent"`;
  default `Column`. String inválida → erro explícito.

**Divergência face ao vanilla a documentar:** o vanilla restringe `Parent`
a `float: true` (erro caso contrário, `collect.rs:309`). O cristalino não
tem `float` implementado — `Parent` é aceite incondicionalmente, com
efeito visual de ancoragem à página sem layout flutuante. Quando `float`
for adicionado, repor a restrição.

Sintaxe: `place("bottom-right", scope: "parent", rect(...))`.
Sintaxe legacy `place("bottom-right", ...)` continua a funcionar (default
Column → ancora à célula se dentro de Grid, à página caso contrário).

Testes de regressão: `place_dentro_de_grid_ancora_a_celula` (Column dentro
de Grid → coords da célula), `place_dentro_de_grid_com_scope_parent_ancora_a_pagina`
(Parent dentro de Grid → coords da página). `place_nao_altera_cursor_y`
(P82) continua a passar — place fora de Grid ancora à página por default
Column (cell_* todos None → fallback para página).

---

## DEBT-38 — Cache de sub-frames no Grid Auto — **ENCERRADO (Passo 84.2)** ✓

**Registado no Passo 83.**

A resolução de altura de linhas Auto chamava `layout_sub_frame_with_width`
para medir a altura intrínseca de cada item, descartando os FrameItems
produzidos. Quando a célula era emitida no documento, a mesma função
era chamada de novo para o mesmo item com a mesma largura, duplicando
o trabalho de layout em todas as células Auto.

**Resolvido no Passo 84.2.** Cache local `HashMap<usize, (f64, Vec<FrameItem>)>`
no braço `Content::Grid`, populado na fase de medição Auto e consumido
(via `remove`) na fase de emissão. Chave: `row_idx * num_cols + col_idx`.
Cache sai de escopo no fim do braço — sem invalidação manual.

Não usa `ptr::addr_of` nem cast de ponteiro (`unsafe` excluído pela
convenção cristalina). Não usa `Arc<[FrameItem]>` no valor (transferência
única do cache para o frame, sem partilha). `std::collections::HashMap`
é permitida em L1 por ADR-0029 (pureza física — RAM é domínio, não I/O).

---

## DEBT-41 — Sealed traits no scanner usam `unsafe trait` — **ENCERRADO (Passo 85)** ✓

`01_core/src/rules/lexer/scanner.rs` tem 6 `unsafe impl Sealed<T>`
usando o padrão sealed-trait clássico da stdlib Rust. A palavra
`unsafe` aqui é mecanismo de encapsulamento (impedir
implementações externas), não indicação de memória não-segura.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo zero — é refactor mecânico para o padrão
"sealed via private module".

### Proposta de resolução

Substituir:

```rust
pub unsafe trait Sealed<T> { ... }
unsafe impl Sealed<char> for ... { ... }
```

Por:

```rust
mod sealed {
    pub trait Sealed<T> { ... }
}

pub trait Pattern: sealed::Sealed<char> { ... }
impl sealed::Sealed<char> for ... { ... }
```

Verificar que nenhum consumer externo das traits do scanner
depende da assinatura `unsafe trait` (uso em bounds genéricos
deveria continuar a funcionar).

### Critério de conclusão

- Zero ocorrências de `unsafe` associadas ao padrão Sealed em
  `scanner.rs`.
- Testes do scanner continuam a passar sem alteração.
- Nenhum impacto visível na API pública de `01_core/src/rules/lexer/`.

### Dependências

Nenhuma. Refactor trivial, pronto para atacar.

**Resolvido no Passo 85.** Padrão "sealed via private module"
aplicado. Zero `unsafe` associadas a Sealed em `scanner.rs`.
Get_unchecked permanece (DEBT-42, bloqueado por benchmark).

---

## DEBT-40 — `ImportGuard::drop` com raw pointer — **ENCERRADO (Passo 90)** ✓

`01_core/src/rules/eval.rs:235` tem `unsafe { (*self.stack_ptr).retain(...) }`
no `Drop for ImportGuard`. O raw pointer é usado porque a vida do
`EvalContext` não é expressível como lifetime do guard (RAII com
scope de função).

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo de eliminação zero a baixo. Resolução fica
para passo dedicado que escolha entre as opções abaixo.

### Opções de resolução (sugestões, não obrigatórias)

1. **`Rc<RefCell<Vec<FileId>>>`** no `EvalContext` + clone no
   guard. Custo: uma alocação `Rc` + borrow check runtime.
2. **Eliminar o guard**. Push/pop manual em torno da chamada
   recursiva. Perde-se RAII — se erro acontece entre push e pop,
   `pop` não corre. Aceitável se o `EvalContext` é descartado em
   erro.
3. **Índice + verificação com `len()`**. Guardar posição no
   stack em vez de ponteiro; comparar com `len()` no drop para
   confirmar que é o topo; pop se sim, erro ou no-op se não.
   Sem custo de performance, sem `unsafe`, mas perde a garantia
   estrutural de que o guard corresponde à sua entrada.

### Critério de conclusão

- Nenhuma ocorrência de `unsafe` em `eval.rs` relacionada com
  `ImportGuard`.
- Testes de `import_stack` (detecção de ciclos) continuam a passar.
- ADR específico escrito se a resolução introduzir decisão
  arquitectural não prevista.

### Dependências

Nenhuma. Pode ser atacado quando o utilizador decidir.

**Resolvido no Passo 90.** `ImportGuard`, `impl Drop for ImportGuard`
e o bloco `unsafe { (*stack_ptr).retain(...) }` foram removidos.
`EvalContext.import_stack` substituído por `EvalContext.route:
Vec<FileId>` com API segura `with_route_id(id, span, f)` que empurra
antes e retira após a closure (incluindo em `Err`). Simultaneamente:
(i) fecha a divergência estrutural face ao vanilla (ADR-0033) ao
materializar `Route<'a>` em `01_core/src/entities/world_types.rs`
com a forma canónica (`outer: Option<Tracked<'a, Self>>`, `id`,
`len`, `upper: AtomicUsize`, `#[comemo::track]` em `contains`/`within`,
`check_*_depth` para as 4 categorias); (ii) elimina o último
`unsafe` de `eval.rs` (ADR-0032). A escolha entre as opções acima
foi variante (2) — eliminar o guard, push/pop explícito — porque
`EvalContext` em cristalino é mutado em vez de recriado por
recursão (Classe A do plano do Passo 90). Teste E2E
`import_cycle_detectado_retorna_err_sem_panic` valida, via API
pública de `eval`, que um ciclo `main → other → main` retorna
`Err` sem panic. O `scanner.rs` continua a ser o único `unsafe`
em L1 (DEBT-42, bloqueado por infra de benchmark).

---

## DEBT-44 — `EvalContext` não usa estruturalmente `Route<'a>` — **ENCERRADO (Passo 92)** ✓

O Passo 90 materializou `Route<'a>` em
`01_core/src/entities/world_types.rs` com paridade estrutural
completa face ao vanilla (`outer: Option<Tracked<'a, Self>>`,
linked list imutável, `#[comemo::track]` em `contains`/`within`).

No entanto, `EvalContext` (em `01_core/src/rules/eval.rs`) não usa
`Route<'a>` estruturalmente. Mantém campo `pub route: Vec<FileId>`
como projecção plana — uma lista linear que imita a cadeia mas não
é a estrutura vanilla. A API `with_route_id(id, span, f)` substitui
o `ImportGuard` antigo com push/pop explícito sobre este `Vec`, sem
`unsafe`.

### O que o Passo 90 resolveu

- `unsafe` em `ImportGuard::drop` eliminado (ADR-0032 avança).
- `Route<'a>` existe como tipo em L1 para uso futuro.
- DEBT-40 encerrado (o `unsafe` era o seu critério de fecho).

### O que o Passo 90 não resolveu

- Divergência estrutural face ao vanilla: o `eval` do vanilla
  propaga `Route<'a>` por valor entre frames (linked list imutável);
  o cristalino propaga estado partilhado num `Vec` dentro de
  `EvalContext`.
- `Tracked<'a, Route>` não é usado como parâmetro de funções
  memoizadas, mesmo quando a infraestrutura `comemo` já está
  disponível.

### Razão do adiamento

Opção escolhida no Passo 90 por decisão pragmática: integrar
`Route<'a>` estruturalmente exigiria refactor transversal de ~12
funções `eval_*` (parâmetro `route: &Route<'_>` ou campo
`route: Route<'a>` no `EvalContext<'w>`). Benefício observável
limitado enquanto `Expr::ModuleImport` e `Engine<'a>` continuam
stubs. A decisão preservou o valor principal (fechar `unsafe`,
fechar DEBT-40) e adiou o valor estrutural.

### Critério de conclusão

- [x] `EvalContext.route: Vec<FileId>` eliminado.
- [x] Mecanismo de detecção de ciclo usa `Route<'a>` directamente
      (parâmetro `route: Tracked<'r, Route<'r>>` nas 10 funções
      `eval_*`/`apply_*`/`intercept_*` que participam na recursão).
- [x] API pública do eval não muda (comportamento observável
      preservado — ADR-0033).
- [x] Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
      continua a passar sem alteração.
- [x] Nenhum novo `unsafe` introduzido.

### Dependências

Nenhuma técnica. Passo dedicado quando priorizado. O Passo 90 deixou
o cenário preparado: `Route<'a>` existe e tem API funcional; falta
apenas ligá-lo ao `EvalContext`.

### Nota sobre escopo

A integração estrutural, quando atacada, pode revelar que o escopo
real é maior do que ~12 funções — por exemplo, se o `Engine<'a>`
for materializado simultaneamente. Nesse caso, este DEBT pode
dividir-se em sub-DEBTs.

**Resolvido no Passo 92.** Primeira aplicação concreta da ADR-0036
(atomização progressiva). Mudanças efectivas:

1. `EvalContext.route: Vec<FileId>`, `route_contains`, `with_route_id`
   e o pré-push de `current_file` em `EvalContext::new` foram
   removidos. `EvalContext` deixa de carregar estado de
   detecção-de-ciclo.
2. As 10 funções que participam na recursão
   (`eval`, `eval_markup`, `eval_expr`, `eval_markup_body`,
   `eval_args`, `eval_conditional`, `eval_while`, `eval_for`,
   `eval_let`, `eval_counter_method`) ganham parâmetro
   `route: Tracked<'r, Route<'r>>` (lifetime nomeada `'r` devido
   à invariância de `Route<'_>`). `apply_func`, `apply_closure`,
   `apply_show_rules` e `intercept_content` ganham o mesmo
   parâmetro por propagação.
3. `eval_math_content`/`eval_math_expr` permanecem inalteradas
   (tier 3 — não têm caminho a `Expr::ModuleInclude`).
4. `world_types.rs::Route::outer` passou a parametrizar
   explicitamente a `Constraint` via
   `Tracked<'a, Self, <Route<'static> as Validate>::Constraint>`
   — pattern documentado pelo `comemo 0.4.0` para habilitar
   covariância em cadeias `Tracked`. Sem isto o encadeamento
   recursivo de `Route::extend` não compilava (erro de
   invariância).
5. `Expr::ModuleInclude` verifica ciclos via `route.contains(id)`
   real e cria frame filho via `Route::extend(route).with_id(id)`,
   passando `child_route.track()` à chamada recursiva — paridade
   directa com `typst-eval/src/import.rs:232` do vanilla.
6. Os 3 testes `with_route_id_*` do Passo 90 foram removidos
   (testavam o mecanismo intermédio que desapareceu). O teste
   E2E `import_cycle_detectado_retorna_err_sem_panic` passa sem
   alteração — valida apenas comportamento observável. Contagem
   L1: 744 (747 − 3).

DEBT-45 (`check_*_depth` não chamados) continua em aberto;
Passo 93 ligá-los-á agora que `route` está propagado.

---

## DEBT-39 — `active_guards` com push/pop frequente — **ENCERRADO (Passo 95)** ✓

Diagnóstico do Passo 84.4 revelou que `EvalContext.active_guards`
(`Vec<RuleId>`) tem padrão push/pop por entrada/saída de cada show rule
em `apply_show_rules`, **sem clones**. Difere de `show_rules` (resolvido
no 84.4 via `Arc<[ShowRule]>`) — `Arc<[RuleId]>` aqui forçaria
reconstrução O(n) por push e por pop, regressão face ao push/pop O(1) do
`Vec` actual.

Padrão actual (eval.rs):
- `ctx.active_guards.push(rule.id)` antes de `apply_func`.
- `ctx.active_guards.pop()` imediatamente após.
- `ctx.active_guards.contains(&rule.id)` na decisão de saltar regra.

Soluções candidatas para revisão futura:
- Manter `Vec<RuleId>` — aceitar custo zero do clone (nenhum existe) e
  push/pop O(1) como o melhor compromisso. Encerrar este DEBT como
  "registado para clarificar que não é regressão face a 84.4".
- Estrutura persistente (`im::Vector`, `rpds::List`) — push/pop O(1)
  imutável, mas introduz dependência externa não justificada hoje.

Não é candidato a resolução até surgir evidência empírica de problema
(ex: composição profunda de show rules a degradar em documentos reais).

**Resolvido no Passo 95 (Tarefa C, Opção 1).** `active_guards` deixou
de ser campo de `EvalContext` e passou a ser parâmetro
`active_guards: &mut Vec<RuleId>` propagado pelas mesmas 13 funções
que já recebem `route`/`styles`/`show_rules` desde a série de
atomização da ADR-0036 (Passos 92/94/95).

A **preocupação de performance** original do DEBT (não regredir para
`Arc<[RuleId]>`) mantém-se respeitada: o tipo continua `Vec<RuleId>`;
push/pop continuam O(1); zero clones por iteração. A mudança é
**exclusivamente de ownership**, não de representação.

Razão da decisão (Opção 1 em vez de Opção 2 "excepção da Regra 4"):
o DEBT-39 documentava preferência de TIPO (`Vec` sobre `Arc<[T]>`),
não preferência de LOCALIZAÇÃO (campo vs. parâmetro). A ADR-0036
trata da segunda dimensão. Extrair como parâmetro mantendo o tipo é
a solução que satisfaz ambas as restrições — o que o DEBT protege
(O(1) push/pop) é independente de onde o `Vec` vive.

---

## Secção 3 — Dívida de instrumentação

# Dívida de instrumentação — ADR-0006

Os seguintes pontos de timing foram removidos para manter L1 puro.
Religação prevista no Passo 10 (isolamento de comemo/infra).

| Função       | Nome do scope original |
|--------------|------------------------|
| parse()      | "parse"                |
| parse_code() | "parse-code"           |
| parse_math() | "parse-math"           |

## Como religar no Passo 10

Localizar todos os `// ADR-0006: timing removed — ver 00_nucleo/DEBT.md`
em `01_core/src/rules/parse.rs` e substituir `timing_scope!("...")` por
o mecanismo de telemetria escolhido (trait injectável ou outro).

Ver: `00_nucleo/adr/typst-adr-0006-typst-timing.md`
