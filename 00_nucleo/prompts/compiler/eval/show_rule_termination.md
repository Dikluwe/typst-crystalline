# L0 — compiler/eval/show_rule_termination
Hash do Código: cf143c42

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/show_rule_termination.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/show_rule_termination.rs:28` — `type ShowRuleLoopResult`.
- `01_core/src/compiler/eval/show_rule_termination.rs:30` — `fn run_show_rule_loop`.
- `01_core/src/compiler/eval/show_rule_termination.rs:97` — `fn show_rule_cycle_error`.
- `01_core/src/compiler/eval/show_rule_termination.rs:113` — `fn show_rule_depth_exceeded_error`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/docs/content/reference/language/context.typ:13`.

## Contract

Own the pure one-node termination strategy, not selector matching or rule
evaluation. Keep ShowRuleLoopResult = (Content, bool) and
run_show_rule_loop<F>(initial, apply_one_step, max_depth, full_error), where the
callback returns SourceResult<Option<(Content, RuleId)>>. The bool reports whether
any rule was applied. Propagate callback errors unchanged.

No match ends successfully. A transformation with the same morph_canon is a
morphological fixed point. Otherwise retain canonical forms and RuleId transitions;
a repeated form establishes a cycle and identifies the participating rules.
The depth limit is a backstop, not the sole termination mechanism. Preserve
`show rule cycle detected` and its rule-identifying hint separately from
`maximum show rule depth exceeded`, whose hints are exactly
`maybe a show rule matches its own output` and
`maybe there are too deeply nested elements`. full_error adds only the existing
cyclic/nonconvergent classification hint and associated history; fixed-point and
cycle detection are independent of that flag.

The one-step callback is supplied by
00_nucleo/prompts/compiler/eval/rules.md, which owns innermost-first precedence,
active RuleId guards and at most one transformation per call. This module does
not inspect Engine, EvalContext, selectors, styles or document realization.

Termination must not rely on a claim that state reads in show rules are forbidden:
show rules establish style context and locatable rules may establish location
context in the language. CounterUpdate/StateUpdate remain in canonical morphology,
but their presence alone is not a proof that all contextual observations are
stable. Context-sensitive evaluation and post-show introspection require their
own coordinating contracts and revalidation. Do not change eval/layout phase to
repair such behavior inside this loop.

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

O loop mantém tipo e assinatura privados. Para entrada Cell preparada (standalone ou positioned), um passo callback e o indicador applied são retornados à realização, que decide guards/profundidade/visita do produto; igualdade de morph_canon não encerra uma nova geração de Cell. O módulo não usa Engine/ctx/styles ou aplica receitas. Todos os demais elementos conservam ponto-fixo/ciclo/limite atuais. Não substituir essa exceção nominal por política global de elementos não-localizáveis.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.
