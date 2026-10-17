# L0 — compiler/eval/selector_matching
Hash do Código: b7801ed9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/selector_matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/selector_matching.rs:29` — `fn query_selector_to_show_selector`.
- `01_core/src/compiler/eval/selector_matching.rs:112` — `fn selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:203` — `fn native_element_base`.
- `01_core/src/compiler/eval/selector_matching.rs:212` — `fn is_par_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:226` — `fn native_element_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:306` — `fn native_element_field`.
- `01_core/src/compiler/eval/selector_matching.rs:324` — `fn element_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:342` — `fn values_eq_semantic`.
- `01_core/src/compiler/eval/selector_matching.rs:354` — `fn is_node_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:370` — `fn is_text_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:376` — `fn textual_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:389` — `fn textual_match_ranges`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/selector.rs:134-140`.
- `foundations/selector.rs:149-153`.
- `typst-realize/src/lib.rs:1261-1325`.
- `typst-realize/src/lib.rs:1320-1371`.

## Contract

Own conversion from query selectors to show selectors, typed node matching,
node-rule classification and literal/regex splicing. Operate on received
values/content without scope mutation, World reads or layout. Splicing invokes
only its supplied replacement closure for each matched slice; this helper does
not independently resolve or apply a user Func.

Literal/regex splicing passes only each nonempty matched text slice to the recipe,
keeps unmatched prefix/suffix, leftmost order and causal content morphology.
This helper does not concatenate descendants or form realized text groups;
00_nucleo/prompts/compiler/eval/rules.md owns group construction/normalization.
Text/Regex are not node rules, Label has its separate path, and empty And/Or do
not match. NodeKind/DynKind/Where/native Element and existing compositions retain
the closed dispatch and their current semantics.

NativeElement matching recognizes actual typed native element identity without
executing it or consulting a public spelling. Native paragraph detection is a
typed helper for rules, not a generic expansion to all compositions. Legacy
strong/emph matching uses semantic origin, not a baked render style; list sequence
recognition preserves the current uniformity requirement.

Legacy Where preserves values_eq_semantic; native Element filter groups use
recursive language equality, names and ordered values under
00_nucleo/prompts/compiler/eval/operators/equality.md. Use the existing explicit
field projection (including text/raw/lang/block/body), without reverse imports
from entities. Queried-content fields delegate to
00_nucleo/prompts/compiler/eval/bindings/field_access.md and its pinned snapshot
contract: Some is authoritative, None uses its legacy path. Label absence and
explicit None remain distinct; Location is not a linguistic filter field.

OutlineEntry matches its own entry identity, not outline or heading. raw.line
uses its real typed built-in element and current dynamic identity; arbitrary
user-defined elements with a similar name do not inherit it. before/after remain
query-only and do not become show predicates. Preserve match/miss behavior for
all pre-existing selector variants; this helper does not fabricate query data.


## Realização com avaliação única

Seletores de show projetam body/visual dos carriers internos. Carriers não são elementos novos nem alteram matching where.

## D05 — equation.where, correção interna de paridade

### Medição antes da decisão

Coleta equation-where-2026-10-06/before.json.gz registra pin
9f2b6e8715237cb086899a42873660fe744622e8, binários SHA-256, UTC, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e diff HEAD --stat/estado exato.
Tekt rejeita equation.where como função não-elemento; vanilla aceita catálogo
público, filtros block/label/body/numbering/alt, alias, composição e show.
Vanilla rejeita campos size/missing, posição, closure e aplicação parcial.
Fonte upstream: foundations/func.rs:433–462 valida field_id e mantém grupo;
math/equation.rs:58–170 separa campos públicos dos internos.
Tekt element_where.rs:63–490 omite equation; selector_matching.rs:269–310
omite Equation no matching nativo. O getter block/body e snapshot já existem.

### Decisão e aceitação

NativeElement identifica Content::Equation pelo ponteiro native_math_equation
do bridge interno. .where combina identidade e campos pelo caminho existente:
Some snapshot autoritativo, None projector legado. Preservar recursividade de
igualdade linguística, wrappers causais e filtros de outros elementos.
Não reconstruir payload, fabricar fields/defaults, inferir nome de função ou
executar constructor. Não alterar NodeKind nem introduzir nova fase.

Fluxo contínuo ADR-0127: correção de tabela interna, sem novo contrato público
ou fase. RED→GREEN dos consumers e revalidação bilateral dos controles;
preservar D04 e não realizar math bare/D06 ou morfologia de matriz/D07.

### D05 — projeção da identidade da equação consultada

Medição adicional after.json.gz: query com where válido devolve zero, enquanto
Kind devolve duas equações. from_tags/equations.rs:161–166 envolve Equation
realizada em Styled; native_element_matches não a identifica. Decisão:
Styled é transparente apenas para esta identidade de equação; wrappers causais
ShowRecipe/Located/Realized são projetados conforme o contrato existente.
Matching mantém o objeto externo para projetar fields e sua configuração;
não generalizar Styled para outros tipos ou alterar snapshots Some autoritativos.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8.
selector(table.cell) devolve cell, where() devolve cell.where(:), a comparação
é false, query/counter rejeitam cell não locatable. Alias verdadeiro medido
por CodeBlock conserva esses resultados. A fixture anterior que definiu alias
apenas no documento não disponibilizava a variável à expressão eval e não
prova alias positivo; table-aliases-before corrige a fixture.

Fonte: lab/typst-original/crates/typst-library/src/foundations/selector.rs:309–321
distingue dict None de Some; :421–450 valida locatabilidade separadamente.
No Tekt, entities/selector.rs:34–37 só representa grupo Element presente;
stdlib/foundations/selector.rs:69–77,348–354 omite a nativa cell; os demais
limites atuais e hashes estão em d08-owner-audit.json.

Classificação: formato, identidade e rejeição de consulta são linguagem;
o novo carrier fechado é mecânica escolhida para preservar esses observáveis.
A distinção do vanilla é explícita na fonte; não se infere intenção apenas
da saída. Inferência: a nova variante basta à fatia de seleção/locatabilidade;
a hipótese será refutada por um caso público cuja ausência de grupo ou
identidade não caiba nela. Não se presume correção de realização de células.

### Contrato materializável somente após ratificação

Converter QuerySelector::NativeElement(f) diretamente no Selector::NativeElement(f) de show já existente, sem NodeKind novo ou grupo Where inventado. Preservar matching nativo estático, snapshots Some autoritativos, wrappers e todas as variantes precedentes. TableCell já é reconhecido por native_element_matches: não duplicar despacho pelo nome cell, nem promover esse matching a locatabilidade. Esta minuta não autoriza preparação/posição/campos de células ou o produtor direto de show table.cell; ficam no D08B.

### Fronteira incompleta e aceitação

D08A cobre apenas carrier, construção, repr, igualdade, composição e
locatabilidade. D08 permanece incompleto: D08B completa show direto, projeção
de body/campos e preparação de células explícitas/implícitas, incluindo
posições, padding, spans, header/footer e escopo lexical. D08B precisa de
seus L0s medidos e de gate próprio caso mude contrato ou fase; esta ratificação
não o autoriza. Não esconder negativos D08B ou declarar paridade universal.

Após ratificação: promover este conjunto atomicamente; escrever testes
linguísticos e verificar RED válido antes do código; materializar com
despacho estático e Rust seguro; validar 1:1/V26 antes de ressellar somente
owners deste manifesto; build, suíte core e linter sem erros. Revalidar
selector cell bare/where/alias, igualdade true/false, query/counter diretos
e compostos, função homônima, partial/closure; preservar D05, D07 e query.
