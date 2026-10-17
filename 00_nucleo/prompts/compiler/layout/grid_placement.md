# L0 — grid placement
Hash do Código: 6667c903

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid_placement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid_placement.rs:42`, `01_core/src/compiler/layout/grid_placement.rs:64`, `01_core/src/compiler/layout/grid_placement.rs:262`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own PlacedCell construction and deterministic occupied-slot placement. Require a positive column count with the current diagnostic. Normalize spans to at least one. Place explicit GridCell x/y requests first, checking invalid columns, boundary-crossing colspan, duplicate origins and occupied-span overlap with their existing diagnostics/hints; a missing y searches the first free row at the specified/default x. Then place remaining cells in row-major free rectangles, skipping occupied slots and wrapping when the span would cross the column boundary. Preserve each original Content wrapper for later style handling and document order within each pass.

The current field extractor recognizes GridCell metadata; non-GridCell content is automatic unit-span content. Do not claim full TableCell-coordinate parity here or infer placements by appearance. No rendering, font measurement, page commit or public-contract expansion is introduced.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Possuir o único motor de ocupação e sua política fechada: Grid legada ExplicitFirst, Table DocumentOrder. Dividir adaptadores tipados, sem duplicar algoritmo. Table preenche uma origem a cada item na ordem de entrada e rejeita origem ocupada/spanned; fixup preenche somente holes reais e mínimo de rows explicitamente solicitado. Resultado ordenado row-major define execução de receitas. Usar aritmética checked e diagnóstico causal source_span, nunca alocação cega a partir de coordenadas enormes. Grid mantém API/política/regras próprias. Layout de Table preparada consome coordenadas do plano, sem executar novamente ocupação nem promover cells ao índice.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### Refutadores suplementares de ocupação — 2026-10-07

Medição anterior à decisão: diagnosticos/table-cell-realizacao-d08b3-2026-10-07/placement-refuters-red.json,
binário B2 congelado e vanilla pinado, fontes/argv/cwd/UTC/resultados completos.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:2182–2215
procura origem livre sem testar spans, avança toda a área quando colspan é todas
as colunas; :2269–2301 rejeita row cheio. :1500–1530 rejeita spans conflitantes.
É semântica de posição/erro. TableDocumentOrder escolhe primeira origem livre:
não salta origem porque colspan não cabe ou porque rowspan conflita depois.
Validar span em seguida com erro/hint causal. X-only pesquisa somente a origem
na coluna; Y-only cheio tem erro dedicado. GridExplicitFirst mantém política
legada de procurar um retângulo que caiba. Refutação: aceitação desses conflitos,
mensagem/span errados ou cursor divergente após full-span. Correção interna no
contrato ratificado, sem campo/assinatura/fase suplementar.

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Ler somente x/y/spans efetivos da Cell pelo helper nominal privado para TableDocumentOrder, incluindo estilos e snapshots preparados. Algoritmo de ocupação, ordenação/padding, tipos/assinaturas públicas e política Grid permanecem. Não criar segundo algoritmo ou consultar StyleChain/Func/Engine no placement.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/grid_placement.rs` nas linhas 283 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `00_nucleo/diagnosticos/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar o helper privado extract_cell_fields sem chamadas. Preservar a rota vigente de extração/realização usada no placement, inclusive as cláusulas ratificadas D08B3. A remoção não elimina metadata nem muda spans de células.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

## Iteração e parâmetros internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/grid_placement.rs` nas linhas 216, 217, 234, 235 contém os avisos selecionados do Clippy. O snapshot anterior à decisão, spans conferidos contra a fonte, owners, referências, binário, UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status e diffstat estão em `00_nucleo/diagnosticos/iteracao-parametros-ci-2026-10-13/before.json.gz`. A inspeção mede mecânica interna e usos; não é nova prova bilateral de paridade ou ganho de desempenho.

### Decisão e aceitação

Percorrer os slices da área validada de occupied, conservando a ordem linha→coluna. A normalização de spans, os checks aritméticos/de colunas e ensure_rows continuam antes do acesso. Enumerar offsets para reconstruir as mesmas coordenadas absolutas do primeiro conflito, mensagem, hint e span causal. A fase de verificação permanece separada da fase de marcação: erro não marca parcialmente a área. Na marcação, preencher true apenas no slice de colunas do retângulo. Não usar truncamento silencioso para compensar limites inválidos nem alterar GridExplicitFirst/TableDocumentOrder, cursor ou padding.

Refatoração interna equivalente em fluxo contínuo ADR-0127, sem contrato público, default ou fase novos. Conservar as expectativas dos testes existentes; não criar testes que apenas espelhem estes loops. Validar suíte core (incluindo conflitos, spans, fixups, marcadores e seleção diferencial), todos os targets, build, documentação, Clippy e linhagem com resselo restrito. Nenhuma supressão adicional de lint é autorizada por este lote.
