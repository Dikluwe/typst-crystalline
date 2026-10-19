# L0 — compiler/eval/rules
Hash do Código: ed07b107

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/rules.rs:43` — `fn type_mismatch`.
- `01_core/src/compiler/eval/rules.rs:59` — `fn expected_length_error`.
- `01_core/src/compiler/eval/rules.rs:87` — `fn edge_cast_error`.
- `01_core/src/compiler/eval/rules.rs:119` — `fn pdf_pages_found_name`.
- `01_core/src/compiler/eval/rules.rs:135` — `fn validate_pdf_page_range_element`.
- `01_core/src/compiler/eval/rules.rs:213` — `fn pdf_standard_expected_list`.
- `01_core/src/compiler/eval/rules.rs:231` — `fn pdf_standard_and_list`.
- `01_core/src/compiler/eval/rules.rs:242` — `fn pdf_version_display`.
- `01_core/src/compiler/eval/rules.rs:248` — `fn pdf_standard_version_hint`.
- `01_core/src/compiler/eval/rules.rs:262` — `fn pdf_validator_label`.
- `01_core/src/compiler/eval/rules.rs:270` — `fn pdf_validator_range`.
- `01_core/src/compiler/eval/rules.rs:290` — `fn pdf_standards_conflict_diagnostic`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `src/space_discarding.rs:53-73,194-256`.
- `text/smartquote.rs:38-89,335-419`.
- `typst-library/src/layout/page.rs:764-772`.
- `typst-library/src/layout/page.rs:813-821`.
- `typst-realize/src/spaces.rs:114-127`.

## Contract

Own set/show evaluation, argument validation, style deltas, rule application,
realization and typed interception. Preserve lexical precedence, innermost-first
application, RuleId revocation/active guards, current casts, warnings, spans and
first-error behavior. Unknown named arguments are never silently ignored.
Show-set transports Styles; transformations preserve semantic morphology.
Delegate selector semantics to
00_nucleo/prompts/compiler/eval/selector_matching.md and one-node termination to
00_nucleo/prompts/compiler/eval/show_rule_termination.md; do not duplicate either
algorithm. Native Element/Where follow the node path, not literal/regex slicing.
The special paragraph routes use the typed native-paragraph predicate without
expanding to all elements/compositions or applying text/element rules twice.

## Textual realization delegation

The descendant owner `00_nucleo/prompts/compiler/eval/rules/textual.md` specifies
causal snapshots, text-node interception, literal/regex grouping and slicing,
space normalization and Unicode nonspacing classification. This parent keeps
set/show routing and one-node application; it neither duplicates the child's
algorithm nor absorbs its consumer. The eval coordinator calls the child under
the same phase and signatures as before the extraction.

## Lexical and synthesized content

Capture immutable lexical rules/styles for an OutlineElem during the renderable
interception pass before applying outline transformations, not from a final global
stack or by inventing snapshots in the pre-show pass. realize_outlines receives
the generated entry, causal snapshot and private parent styles. Apply the normal
matcher/precedence/guards once; absent transformation, use the entry default composed
from its helpers and causal link. Each fixpoint candidate is rebuilt from its own
source, not cumulatively transformed. Do not execute callbacks in pure layout or
insert realized entries as new query occurrences.

Set outline(indent:) accepts length/function/auto and the retained crystalline bool
compatibility. Set outline.entry(fill:) accepts content/none. Store validated
values lexically; explicit constructor values win. Materialize inherited indent
on outlines and fill on direct entries only when omitted. Synthesized entries
receive effective fill. A visually neutral carrier with outline.parent.indent
and outline.parent.key preserves the exact parent for contextual content returned
by a transformation, without changing matching/query order or phase.

For RawElem, synthesize raw.line only when a currently applicable typed rule targets
those instances. Split newline preserving empty segments and ordered one-based
number/count/text/body, run the normal show engine once per line and recompose
with line boundaries. Without such a rule, preserve RawElem/highlighting/default
intact. Manual raw.line uses the same dynamic carrier and show machinery; filters
read its real fields. No alternate guard or precedence algorithm is introduced.

## Set-rule schemas and transport

Keep the existing heading/page/figure/text/par/table schemas and their defaults,
casts and warning policy. Font dictionaries delegate to
00_nucleo/prompts/compiler/eval/font_dict.md. Table numbering retains lexical
table.numbering string/none capture, not the cell-cast owner. Smartquote accepts
enabled:boolean, alternative:boolean and canonical quotes string/array/dictionary/
auto forms. Existing crystalline none compatibility normalizes to auto and is not
claimed as vanilla surface. Validate the whole delta before applying it.

Recognize par.line structurally with exactly numbering (string/function/none),
number-align (horizontal alignment/auto), number-margin (left/right/start/end),
number-clearance (length/auto), numbering-scope (document/page). Store validated
par.line.* values; omission is neutral and unknown names fail. Keep numbering
Func identity/span intact: do not call it, convert its return or fabricate a
physical-line marker during set evaluation. Late execution/request handling stays
with its pipeline/layout owners.

Math set schemas delegate domain casts to the same canonical constructor domain:

- frac.style: vertical/skewed/horizontal stored as canonical Str, with explicit
  element value above cascade above vertical default; binom does not consume it.
- accent.size/dotless: relative length/bool, defaults 100%/true.
- cases.delim/reverse/gap: delimiter pair/bool/relative, defaults {}/false/0.2em.
- mat.delim/align/augment/gap/row-gap/column-gap: canonical casts. gap feeds both
  axes; specific gaps win at the same level. Defaults are ()/center/none and
  row 0.2em/column 0.5em. Explicit augment retains its offsets while folding
  cascaded stroke; explicit none clears inherited augment. Inside augment only
  hline/vline/stroke are consumed; other inner keys are tolerated, but invalid
  consumed values and unknown outer named arguments still fail.
- stretch.size: relative length with default 100%.

For all of these, explicit element arguments win over cascade, and cascade never
sets morphological presence bits or appears as fabricated explicit fields/repr.
Unknown/duplicate names and invalid values fail before effective style mutation.

Recognize dotted format targets structurally, not through an empty text_str.
Empty rules are neutral; omitted fields never insert defaults or erase an earlier
lexical decision. Preserve independent fields, schema diagnostics and ordering:

- format.svg: optional pretty Bool -> format.svg.pretty (default false only when
  the document makes no decision).
- format.png: optional ppi Int/Float -> Float at format.png.ppi. Omission does
  not insert 144.0. Finite zero/negative values retain their measured acceptance;
  nonfinite cases with an aborting oracle do not acquire a parity claim.
- format.html: gate Html at field html before evaluating any argument. The exact
  disabled diagnostic is ``cannot access field `html` because the `html` feature
  is not enabled``, with ``try enabling the `html` feature`` and
  `see https://typst.app/help/compiler-features for more details`.
  Active calls evaluate argument expressions before ordinary schema validation;
  optional pretty Bool is stored at format.html.pretty.
- format.pdf is ungated and accepts tagged Auto/Bool, pretty Bool, pages and
  standard independently under format.pdf.*. Invalid tagged uses
  `expected boolean or auto, found <type>`, not the bool-only message. Unknown,
  duplicate or positional arguments fail normally, not as unsupported-target
  warnings; preserve absence of partial effective deltas.

PDF pages validation is complete during set evaluation. Accept positive Int,
from/to dictionary, array of Int/dictionary, or explicit None. Store the original
validated Value; None is a present reset distinct from omission and empty array
is a present empty selection. Preserve `page numbers start at one`,
`page range must not be empty`,
`unexpected key "{key}", valid keys are "from" and "to"`, and
`start of page range cannot be larger than end`. Top-level mismatch expects
integer/dictionary/array/none; array-element mismatch expects only integer/dictionary.
Nested array/None elements are not accepted. The typed reconstruction belongs to
00_nucleo/prompts/compiler/layout/format_options.md and fails closed on impossible
already-validated state.

PDF standard accepts a canonical PdfStandard string or array of strings, including
empty array, but not None. Use the exact closed name list from
00_nucleo/prompts/entities/document_format_options.md. Unrecognized strings omit
the found-type suffix; wrong-typed values include it. Array-item diagnostics omit
the array alternative and retain the final 'or' list form. Validate PdfStandards::new
and map its structured conflicts to the existing precise messages/hints; normalize
a single string to a one-item Value::Array, retain an original array, then store.
This provides language parsing/conflict validation, not proof of PDF structural
conformance. Preserve distinct conflicts for version/PDF-A/PDF-UA multiplicity,
empty intersection and explicit version outside the allowed range.

This owner does not choose top-level/container boundaries, CLI precedence,
pages/tagged/standard interactions, rasterize, format SVG or export HTML/PDF.
Those retain their format-options and pipeline owners. Feature enablement remains
independent from target selection under the pinned nucleus.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

No set text, validar hyphenate como Bool|Auto com erro "expected boolean or auto, found <type>" no span da expressão e transportar text.hyphenate via custom. No set par, validar justify Bool, transportar par.justify via custom; true avisa que a escolha global otimizada das quebras ainda não é suportada; a justificação simples por espaços está implementada, false não avisa. Validar linebreaks Auto|Str(simple|optimized); erro para outra string é `expected "simple", "optimized", or auto`, outro tipo acrescenta `, found <type>`. Valores válidos conservam o warning de scope-out da estratégia: não transportar como se existisse consumidor optimized. Preservar show-set, restauração de estilos e demais campos. A rodada-config-composicao abaixo completa a justificação simples por espaços. Estratégias globais permanecem incompletas e serão tratadas pela rodada-paragrafo-seletores depois da medição.

### Refinamento de diagnóstico medido antes da implementação

Os testes em first-policy-green.log passam transporte e política (5), mas falham quatro mensagens: type_mismatch retorna int em vez do integer medido no vanilla. O helper existente expected_length_error em eval/rules.rs:65 já usa esse vocabulário. Para estes novos casts, usar helper privado local com boolean/integer/string, sem alterar type_mismatch global nem type_name público. No constructor text, essa exceção explícita supersede somente a exigência de reutilizar type_mismatch para hyphenate. Nenhum contrato público muda.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

realize_flow trata Par e Styled(Par) como fronteiras, fechando o trecho corrente antes de inseri-los, sem embrulhar tudo num Par extra. A realização do body de Par e dos wrappers Styled inline dentro de um flow mantém Sequence inline: não fabricar Par só por entrar em bold/itálico. Implementar recursão contextual privada no owner atual, mantendo os demais bodies/containers/math em seus caminhos existentes. Corrigir o warning de justify true: justificação por espaços passa a existir, optimized permanece ausente; descrever a seleção ainda simples. Não ativar seletor por string sem consumer.

### Refinamento de precedência medido antes da implementação

No estado precedence-state.json, regression-acceptance.json registra 191 verificações passando e duas falhas no controle show-par-sets-false. O Tekt emite extraordi-/interna- onde vanilla conserva palavras sem hifenização. show-par-full-tree.log mostra SetPage e o escopo global dentro de um Par sintetizado; a recipe fica exterior ao text.hyphenate true. eval/mod.rs:1532-1560 possui a proveniência exata dos escopos #set; rules.rs:660-727 apagou essa distinção ao tratar Styled sempre como inline. Não inferir origem pelo conteúdo do Styles.

Decisão interna: o fold dos escopos #set em eval_markup transporta marcador privado eval.flow.scope=true no Styles existente. Antes de formar o primeiro parágrafo material, realize_flow reconhece esse marcador e mantém o escopo exterior à realização dos seus parágrafos, preservando os carriers prévios SetPage/Empty/Space/SpaceFromNewline em ordem, sem os embrulhar em Par. Após conteúdo inline material o escopo permanece inline dentro do parágrafo corrente; wrappers de constructors/estilos sem marcador permanecem inline. Nenhuma entidade, assinatura, comportamento válido por defeito ou fase pública muda. Pin no núcleo compartilhado paragraph-composition documenta a distinção; não recorrer a heurística por chave de estilo. O consumer do hub só produz o marcador no fold; preparação e reconhecimento são do owner rules.

### Refinamento de escopo estilizado integral

precedence-pdf/resultados.json e proveniencia.json medem oito controles no binário anterior a este refinamento. native-text-inline-override e mid-par-set-override hifenizam em ambos (diferença -/U+AD já aberta). native-text-only-override diverge: o Tekt hifeniza e o vanilla não. Portanto a hipótese "todo constructor Styled permanece inline" é insuficiente. Um Styled que contém todo o material de um fluxo ainda sem material fornece o escopo inicial de realização do parágrafo; o show-set precisa ser aplicado dentro dele. Quando há material anterior ou posterior fora desse wrapper, ele permanece inline e não altera fronteiras.

Decisão interna: além do marcador #set, reconhecer em realize_flow um wrapper Styled com prefixo e sufixo contendo apenas Empty/Space/SpaceFromNewline/SetPage. É fatoração de escopo que cobre todo o material, não inferência por campos de Styles. Conservar os carriers em ordem e realizar o body no escopo exterior existente. Não promover fragmentos com texto fora do wrapper. A regra específica integral supersede apenas a exclusão absoluta dos constructors na cláusula anterior; nenhum contrato público muda.

### Singleton de texto no escopo integral

whole-style-first-failure.log reproduz duas hifenizações no singleton integral, contra zero esperado. rules.rs:realize_node conserva Text diretamente; ao fatorar o Styled integral, esse caminho deixou de formar Par, portanto nenhuma recipe encontrou um parágrafo. O refinamento introduz realize_scoped_flow privado: atravessar Styled, envolver Text singleton em Par usando realização inline, e delegar os demais fluxos/containers a realize_node. Essa regra específica completa o texto singleton sem embrulhar blocos como headings em Par. Nenhuma fase ou assinatura pública muda.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Depois do cast válido de set par(linebreaks), transportar par.linebreaks em engine.styles, inclusive Auto. Substituir aviso genérico de propriedade ignorada por aviso factual de suporte parcial da seleção; justify true mantém aviso factual sobre conjunto ainda limitado, sem afirmar que global nunca existe. Nenhum contrato de linguagem ampliado.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `lab/results/construction/diagnostics/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

O aviso já existente sobre suporte parcial deixa de listar hifenização como recusa geral; menciona palavras latinas uniformes/largura constante e casos ainda incrementais. Preservar casts e canais.

## Remoção de hardcode funcional — 2026-09-28

### Medição anterior à decisão

`correcao-hardcode-paragrafo-2026-09-28/before-pdf/` mede 20 entradas contra
9f2b6e8715237cb086899a42873660fe744622e8 no estado `initial.json` e `before/`.
`eval/rules.rs:3483-3525` e `stdlib/text/constructor.rs:153-190` não capturam
`overhang`; auto/int/string são aceitos indevidamente. A fonte vanilla
`text/mod.rs:391-393` declara bool default true; `inline/line.rs:518-525`
condiciona a compensação ao estilo final. `paragraph_hyphen.rs:221-224`
sempre multiplica a largura por0.55. A suspeita inicial sobre `paragraph_select.rs:78-82` foi refutada antes de
implementar: vanilla `inline/linebreak.rs:551-559` usa attempt.justify, que
`inline/line.rs:138-139` desativa para mandatory. Conservar a fórmula atual
e testar a última linha natural. Isto mede comportamento, não intenção histórica.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127, sem campo ou assinatura
pública. Não confundir constantes normativas de um algoritmo com configuração
ignorada. O fator0.55, custo-base135, distância5 e incremento0.15 vêm da fonte
pinada; identificar essas constantes pelo significado e referência, sem alegar
que renomear números implementa configuração. Custos customizados continuam
escopo pendente. Conservar identidade textual literal do hífen nesta rodada:
ela requer carrier/contrato separado, não heurística sobre glyph ou posição.

Capturar overhang bool em text.overhang; rejeitar outros tipos com expected boolean e span causal do valor, usando helper de diagnóstico vigente. Não emitir warning de propriedade descartada para overhang válido. O suporte geométrico continua limitado ao subconjunto certificado; não prometer pontuação comum ou fallback completo.

## Custos textuais personalizados — 2026-09-28

### Medição anterior à decisão

`correcao-custos-paragrafo-2026-09-28/before-pdf/` registra 66 entradas no
estado `initial.json` e `before/`, contra pin 9f2b6e8715237cb086899a42873660fe744622e8.
`text/mod.rs:1507-1564` valida Dictionary, Ratio por campo e composição parcial;
`inline/mod.rs:240` lê costs do escopo compartilhado, não de cada fragmento;
`inline/linebreak.rs:29-30,961-962` aplica defaults 135/100 vezes o ratio.
O Tekt descarta costs em eval/rules e aceita sem transportar no constructor;
paragraph_hyphen e paragraph_select usam defaults fixos. Os erros e spans
estão no stderr vanilla, inclusive negativos aceitos. Não inferir intenção.

### Decisão vigente

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem novo contrato
público, flag ou fase. Módulo privado compiler/text_costs possui owner único
para cast, transporte por componente e resolução dos custos compartilhados.
Hifenização/runt globais passam a consultar a configuração. Widow/orphan são validados e transportados; sua influência é limitada ao subconjunto regional certificado descrito abaixo.
Custos de fragmentos parciais não substituem o custo compartilhado; os quatro
canais validados são neutros para glyphs na whitelist Styled. Um wrapper integral
é fatorado pela realização existente antes de formar Par. Largura variável
permanece limitada. Não deduzir custo pelo glyph. Preservar todos os defaults.

Set text costs chama parser compartilhado antes de mutar chain; empurra somente campos fornecidos, em canais canônicos. Erro usa span do dictionary. Widow/orphan fornecidos podem gerar aviso factual de influência na paginação pendente; hyphenation/runt não são descartados.

### Refinamento de custo compartilhado medido antes de implementar

after-first-resultados.json/proveniencia.json registra 66 entradas com o primeiro
binário de custos. Só restore e inline divergem entre os inputs válidos. A fonte
`inline/mod.rs:240` consulta shared.get(TextElem::costs); `eval/rules.rs` conserva
fatoração dos Styled que cobrem todo o material do fluxo antes de formar Par.
`paragraph_prepare.rs:45-78` recusa os novos canais mesmo quando eles não alteram
nenhum atributo de glyph: o fallback incremental produz escolhas divergentes.
A exclusão absoluta desses canais foi refutada; isto supersede o scope-out
anterior, sem promover fragmentos ao escopo exterior nem mudar fase de pipeline.

Permitir exclusivamente text.costs.hyphenation/runt/widow/orphan na whitelist
Styled da preparação. O custo lido pelo planejamento é o escopo compartilhado
exterior: push temporário da preparação é restaurado e não altera tal leitura.
Inline parcial não escolhe custos do parágrafo; Styles integral continua a ser
fatorado pelo mecanismo existente de realização e entra na chain antes de Par.
Não criar espaço ou fronteira de palavra por diferenças de costs. Manter
limitações físicas widow/orphan e contextos fora do subconjunto certificado.

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

Quando set text(costs:) inclui widow/orphan, atualizar o aviso factual para suporte limitado a parágrafos preparados com métricas verticais uniformes; hint declara colunas/notas/conteúdo misto ainda fora do subconjunto. Não emitir aviso de influência inteiramente ausente agora que o subconjunto foi materializado. Nenhuma nova API de Sink no constructor.

## Atomização comportamental P1499

**Medição:** a working tree não commitada em `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` contém 4754 linhas neste consumer; a suíte L1 passou com 6401 testes e 2 ignorados. Não é benchmark nem nova comparação bilateral.

**Decisão:** este owner conserva diagnósticos genéricos, bindings e testes. Os comportamentos de PDF, math set, realização, aplicação show, outline, intercepção, set e show são delegados aos owners exclusivos `rules/{pdf,math_set,realize,apply,outline,intercept,set,show}.md`. O owner `rules/textual.md` permanece independente. As cláusulas históricas acima mapeiam a superfície; cada filho especifica sua família. A separação não altera contrato público, defaults ou fase.


## Realização com avaliação única

O consumer de realização reutiliza apply_show_rules; não duplicar matching ou terminação.


## Realização com avaliação única

Serviço de grupo textual capturado é interno ao crate e reutilizável pelo consumer de realização.

## Pin do núcleo de seleção — 2026-10-18

As medições e o refinamento de forma/RTL em
lab/results/construction/diagnostics/bibliography-hanging-2026-10-18 precedem o novo pin.
Este owner recebe somente o pin do núcleo compartilhado; seu código executável,
API, fase e demais responsabilidades não mudam nesta etapa.
