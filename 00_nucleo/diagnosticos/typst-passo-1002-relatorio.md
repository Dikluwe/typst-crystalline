# Passo 1002 — Relatório: Protótipo de fatiamento `operators.rs` (hub + 5 nós)

**Tipo**: Protótipo — primeiro caso real da ontologia hub/nó.
**Data**: 2026-08-12.
**Estado de medição (ADR-0121)**: commit base `00dc94966`. No início, o working tree
tinha **zero modificações em ficheiros rastreados** e untracked só documentação dos
passos 999–1002 (relatórios, materializações, `prompts/auditar-spec.md`) e 3 PDFs de
teste na raiz. Desvio registado face à pré-condição "git status limpo": os untracked são
o resíduo normal do fluxo de passos; nenhum código ou prompt estava modificado.
Todos os números abaixo vêm desse estado + as alterações deste passo (listadas no fim).

---

## Fase A — Confirmação por leitura (os 4 critérios com evidência)

### A.0 — A lista de funções do Passo 1000 estava incompleta

Leitura integral de `operators.rs` (1148 linhas): **13 funções**, não 9. O grep do
P1000 (`^pub fn |^fn `) não apanhou `pub(crate) fn` — ficaram de fora
`eval_binary_op`, `eval_unary_op`, `vanilla_type_name` e `join`. Confirmado que
`eval_binary_op`/`eval_unary_op` (citados por `decimal-arithmetic.md`) existem no
ficheiro. **Lição para o método**: inventários futuros têm de incluir `pub(crate)`.

### A.1 — `decimal-arithmetic.md` (o órfão): sem deriva substantiva

Lido na íntegra e confrontado com o código: tudo o que especifica está implementado
(aritmética homogénea `Decimal`, gate de div-zero unificado, ordenação, `Neg`;
igualdade via braço genérico). Deriva apenas formal: título com path antigo
(`rules/eval/...`), campo `Hash do Código` próprio, e orfandade de linhagem (P1001).
Conteúdo absorvido pelo nó `arithmetic.md` — incluindo a divergência registada face ao
vanilla (que coage `Decimal ↔ Int`; o cristalino não — scope-out medido, mantido).

### A.2 — Critério 1 (isolamento de teste): válido, mas não discriminou

Todas as 13 funções são `Value → Value` puras; qualquer divisão por tópico é testável
sem tipos/estado dos outros nós. Os 9 testes co-localizados existentes chamam o
dispatcher com `Value`s construídos à mão. **Para este ficheiro o critério é
vacuo** — qualquer corte o passa. Registado como limitação do método: em ficheiros
puros, o critério 1 não discrimina cortes bons de maus (ver «Avaliação do método»).

### A.3 — Critério 2 (pureza vs estado): confirmado por leitura integral

Nenhuma função toca `EvalContext`, `Scope` ou estado fora dos parâmetros; os imports
são só `entities::*`. Os 5 nós caem todos do lado **Declarativo**, como previsto no
reconhecimento anterior.

### A.4 — Critério 3 (co-mudança histórica): o mais informativo

Medição: 23 commits com alterações às funções, atravessando o rename
`rules/`→`engine/` (`6636c5ea6`), por hashing do corpo de cada função por commit.

- `eval_binary_op` mudou em **20 dos 23** commits — o `match` gigante é a superfície
  de mudança, e dentro dele os braços co-mudam **por tópico** (P404 só Decimal;
  P713 só `Length/`; P720 só `Array/Dict +`; etc.). Suporta o corte por tópico.
- `value_eq` + `values_eq`: mudaram juntas em P818 e nunca mais; **nunca** com
  `sanitize_length_nan` (só no `cargo fmt` global de P797 — ruído mecânico,
  descontado). Confirma o corte equality/arithmetic proposto.
- `binary_mismatch` + `vanilla_type_name`: criados juntos em P842, nunca separados —
  nó `error_formatting` coeso.
- `long_type_name`: criado em P843 **para a mensagem de `join`** — pertence a `join`,
  não a `error_formatting`. Refinamento que a tabela hipotética não via.
- Ordenação e igualdade nasceram no mesmo commit (P818, lote), mas nunca co-mudaram
  desde então — evidência neutra a favor da separação.

### A.5 — Critério 4 (correspondência vanilla): divergência registada

O vanilla mistura tudo num `ops.rs` de 152 linhas. O cristalino fatia por tópico de
propósito: o ficheiro cresce por feature e o custo de manutenção por IA é proporcional
ao que cada sessão lê (ADR-0104). Paridade é com a língua, não com a forma do ficheiro
(ADR-0107). Razão escrita no hub (`operators.md`, secção «Divergência deliberada»).

### A.6 — Pergunta do passo: múltiplos `@prompt` por ficheiro — NÃO é suportado

Medido directamente: um header com 6 pares `@prompt`/`@prompt-hash` produz
**erro V15** — "um ficheiro, um prompt — dividir o ficheiro ou remover as linhagens
extra; --fix-hashes é indefinido com multi-@prompt". A nota de P772o sobre
`fallback_fonts.rs` referia-se ao **par de linhas** (um `@prompt` + um
`@prompt-hash`), não a dois prompts — `fallback_fonts.rs` tem um só `@prompt`.
**Conclusão: a divisão do `.rs` era necessária** para o 1:1, e foi feita (Fase B).

### A.7 — Correcção da divisão proposta (a evidência mandou)

A divisão hipotética (arithmetic, equality, ordering, **coercion**, error_formatting)
foi corrigida antes de materializar, como o passo manda:

- **`coercion.md` eliminado** — não há função nem região que a possua: a coerção
  `Int↔Float` vive inline nos braços de cada tópico; zero sinal de co-mudança próprio.
  Falhava os critérios 1 e 3. A política de coerção fica documentada em cada nó onde
  actua.
- **`join.md` adicionado** — `join` + `long_type_name` não cabiam em nenhum dos 5
  cortes (estavam invisíveis na lista do P1000); a co-mudança mostra-os como unidade
  coesa com consumidores próprios (`eval/mod.rs`, `control_flow.rs`).

Resultado: 5 nós — `arithmetic`, `equality`, `ordering`, `error_formatting`, `join`.

---

## Fase B — Materialização

### Prompts (L0 primeiro)

| Prompt | Papel |
|--------|-------|
| `engine/eval/operators.md` | hub — tabela de despacho + invariantes, zero lógica |
| `engine/eval/operators/arithmetic.md` | `+` `-` `*` `/`, unários, `and`/`or`, gate div-zero, `sanitize_length_nan` |
| `engine/eval/operators/equality.md` | `==` `!=`, `in`/`not in`, `values_eq`/`value_eq` |
| `engine/eval/operators/ordering.md` | `<` `<=` `>` `>=`, `value_cmp` e helpers |
| `engine/eval/operators/error_formatting.md` | `binary_mismatch`, `vanilla_type_name` (verbatim vanilla) |
| `engine/eval/operators/join.md` | `join`, `long_type_name` |

Escritos **sem nenhuma referência a passo** (regra nova de P999 cumprida à nascença),
conservando as medições (`file:line` do vanilla, comportamentos medidos, scope-outs).
Absorvidos e removidos: `engine/eval/ops.md` (o agregador anterior) e
`engine/eval/decimal-arithmetic.md` (o órfão). Ponteiros actualizados:
`engine/eval.md:131` → `operators/join.md`; `entities/value.md:134` →
`operators/arithmetic.md`.

### Código (forçado pela V15, autorizado condicionalmente pelo passo)

`operators.rs` (1148 linhas) → módulo `operators/`:

| Ficheiro | Linhas | Conteúdo |
|----------|-------:|----------|
| `mod.rs` | 47 | shell de `eval_binary_op` (despacho por variante de `BinOp`) + re-exports |
| `arithmetic.rs` | 683 | gate div-zero + braços aritméticos/booleanos, `eval_unary_op`, `sanitize_length_nan`, 8 testes |
| `ordering.rs` | 168 | braços de ordenação + 4 helpers |
| `error_formatting.rs` | 143 | `binary_mismatch`, `vanilla_type_name`, 1 teste |
| `equality.rs` | 122 | braços `Eq`/`Neq`/`In`/`NotIn`, `values_eq`, `value_eq` |
| `join.rs` | 93 | `join`, `long_type_name` |

Preservação de comportamento: o texto dos braços foi **cortado e colado** (não
reescrito); a ordem dentro de cada grupo de operador é a original; a fronteira de cada
sub-`match` chama o mesmo `binary_mismatch`; o dispatcher só particiona por variante
de `BinOp` (disjuntas). Único braço novo: fronteira defensiva em `ordering.rs`
(inalcançável via o dispatcher — exigência de exaustividade do compilador).

## Fase C — Validação

- **`crystalline-lint .` → exit 0**, zero erros; só os 2 warnings V7 pré-existentes
  da baseline (`auditar-spec.md`, `package_version_resolution.md`). Zero órfãos novos
  (os 6 prompts são citados exactamente uma vez cada), zero V5/V6/V15.
- **`cargo test --workspace`: 5828 passados, 0 falhados** (4959 em typst-core, onde
  os 9 testes co-localizados foram movidos para o ficheiro do seu nó e correm).
- Baseline de comparação: antes do passo, lint exit 0 com os mesmos 2 warnings
  (após renomear `temp/p999/audita_prompts.py` para `.py.txt` — ficheiro de rascunho
  de P999 que o V8 apanhava; registado aqui por transparência).

## Avaliação do método (o passo pede explicitamente)

- **Critério 3 (co-mudança)** foi o único que corrigiu o plano: derrubou `coercion`,
  posicionou `long_type_name`, confirmou os pares coesos. Manter.
- **Critério 1 (isolamento de teste)** não discriminou — em ficheiro puro, qualquer
  corte o passa. Em ficheiros com estado (`eval.md`/`bindings.rs`, o alvo seguinte)
  deve morder; manter com essa expectativa calibrada.
- **Critério 2 (pureza)** idem: aqui só confirmou; é o critério decisivo para ficheiros
  que tocam `EvalContext`. Manter.
- **Critério 4 (vanilla)** é registo, não decisão — cumpre o papel.
- **Lacuna encontrada no próprio reconhecimento**: o grep de inventário do P1000 sem
  `pub(crate)` escondeu 4 das 13 funções — incluindo as duas principais. Corrigir o
  padrão de inventário antes de aplicar o método a `eval.md`/`bindings.rs`.
- **V15 muda o custo do padrão hub/nó**: cada nó exige um ficheiro `.rs` próprio —
  o fatiamento de prompts é inseparável do fatiamento de código. Para `eval.md`
  (11 ficheiros já separados) isto é mais barato do que foi aqui; para `bindings.rs`
  (um ficheiro só, 2235 linhas) será outro split destes.

## Alterações deste passo (para revisão)

- Novos: 6 prompts (`operators.md` + `operators/*.md`); 6 ficheiros
  `01_core/src/engine/eval/operators/*.rs`.
- Removidos (absorvidos): `prompts/engine/eval/ops.md`,
  `prompts/engine/eval/decimal-arithmetic.md`, `01_core/src/engine/eval/operators.rs`.
- Editados: `prompts/engine/eval.md` e `prompts/entities/value.md` (ponteiro cada);
  resselo de hash (`--fix-hashes`) nos 13 ficheiros que citam esses dois prompts
  (`eval/*.rs` ×11, `entities/value.rs`, e os headers dos 6 novos).
- `temp/p999/audita_prompts.py` → `.py.txt` (fora da topologia do linter).
