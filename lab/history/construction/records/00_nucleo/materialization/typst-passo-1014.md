# Passo 1014 — Fatiamento hub/nó `stdlib::structural`

**Tipo**: Aplicação do método completo (P1002) a `compiler::stdlib::structural` — maior
ficheiro stdlib (4116 linhas, per Passo 1004), último candidato confirmado da leva do
Passo 1008.
**Candidato confirmado**: P1008 — agregado (sem `trait`), fan-in real de módulo (10
ficheiros distintos chamam as suas nativas: `native_heading` em 6, `native_table` em 5,
`native_strong`/`native_emph`/`native_raw`/`native_link`/`make_math_module` em 4 cada;
fan-in baixo do DSM é efeito da facade `stdlib/mod.rs`, não ausência de consumo real).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1013 (família `eval::*` fechada).

---

## Contexto adicional (do Passo 1004, não repetir a investigação)

`structural.rs` foi identificado como "Declarativo (com grande aglomeração de domínios)":
`strong`, `emph`, `raw`, `heading`, `par`, table/grid/header/footer/cell/hline/vline,
bibliography/cite. O vanilla fragmenta isto em `typst_library::model::{table, heading,
par, quote, ...}` + `typst_library::layout::grid` — vários ficheiros pequenos.

**Ligação já conhecida ao Passo 1001**: os órfãos `grid_hline.md`, `grid_vline.md`,
`table_hline.md`, `table_vline.md` (e o já corrigido/materializado `table.md`,
`decimal-arithmetic.md` não relevante aqui) são prompts finos já escritos para pedaços
deste ficheiro, nunca activados como donos (`@prompt`) porque o hub venceu. **Verificar se
ainda existem e se o conteúdo bate com o código actual antes de escrever L0 de raiz** —
mesma disciplina que poupou trabalho em `operators.rs` (P1002, reaproveitou
`decimal-arithmetic.md`).

## Fase A — Inventário completo

```
grep -n '^pub fn \|^pub(crate) fn \|^fn ' 01_core/src/compiler/stdlib/structural.rs
```
Confirmar lista completa de nativas — o Passo 1004 não enumerou todas individualmente, só
por amostra.

## Fase B — Os 4 critérios, com evidência

1. **Isolamento de teste** — cada `native_*` provavelmente testável com `Content`/`Args`
   construídos à mão. Confirmar se alguma precisa de `Engine` real (ex.: bibliografia pode
   precisar de acesso a ficheiro/`World` — verificar, não presumir puro só por ser
   "declarativo" no sentido do P1004, mesma lição do `font_dict`).
2. **Pureza vs estado** — aplicar per `file:line` a cada nativa, não em bloco. Hipótese:
   markup simples (`strong`/`emph`/`raw`/`heading`/`par`) provavelmente puro; tabela/grid
   (com `hline`/`vline`, numeração, ver P997/P661 sobre `table_counter`) pode tocar
   `CounterRegistry`/introspecção; bibliografia (`cite`) precisa de `World`/`Engine` para
   carregar ficheiros `.bib` — confirmar.
3. **Co-mudança histórica** — mesma técnica. Hipótese a testar: markup de texto
   (`strong`/`emph`/`raw`) muda separadamente de table/grid, que muda separadamente de
   bibliografia — três (ou mais) agrupamentos de co-mudança distintos.
4. **Correspondência vanilla** — `typst_library::model::{table, heading, par, quote,
   figure, ...}` + `typst_library::layout::grid` + biblioteca separada para bib/cite
   (confirmar módulo exacto). Candidato de fronteira: `markup_elements` (strong/emph/raw/
   heading/par), `table_grid` (table/grid/header/footer/cell/hline/vline), `bibliography`
   (bibliography/cite). Confirmar com Critério 3, não aceitar às cegas.

## Fase C — Reaproveitar os órfãos antes de escrever de raiz

Para cada órfão já existente (`grid_hline.md`, `grid_vline.md`, `table_hline.md`,
`table_vline.md`): ler, confrontar com o código actual (pode ter derivado desde a
orfandade), e absorver o que estiver correcto — mesmo procedimento do P1002 com
`decimal-arithmetic.md`. Não reescrever do zero o que já está certo.

## Fase D — Materializar

Nós conforme Fase B decidir (candidato inicial: `markup_elements`, `table_grid`,
`bibliography` — ajustar por evidência real). Cada nó: L0 sem referência a passo, campo
Técnica se aplicável, ficheiro `.rs` próprio (V15).

**Atenção especial ao nó de tabela/grid**: o Passo 1001 já tinha uma dúvida em aberto sobre
`table_counter` viver local ao `Layouter` vs `CounterRegistry` partilhado (achado de Junho,
nunca confirmado como resolvido). Se este fatiamento tocar essa área, confirmar o estado
actual antes de mover — não mover um bug conhecido sem o assinalar.

## Fase E — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão.

## Fase F — Avaliação do método

Registar se os órfãos reaproveitados pouparam trabalho real (medir), e se a fronteira
vanilla bateu com a co-mudança ou foi corrigida por ela.

---

## Resultado esperado

`stdlib::structural` fatiado em nós por domínio (markup/table-grid/bibliografia ou o que a
evidência confirmar), com os órfãos do Passo 1001 absorvidos onde corretos, zero
regressão. Com este passo, a leva completa de candidatos do Passo 1008 fica tratada
(`rules`, `closures`, `bindings`, `structural`) — falta só `stdlib::text`, não incluído
nesta leva por decisão do dono (ordem de execução).
