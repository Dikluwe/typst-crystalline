# L0 — compiler/layout/grid/row_groups
Hash do Código: 3c41fca4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/row_groups.rs`. A lógica original estava em `01_core/src/compiler/layout/grid.rs:185-282` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Normalize Grid/Table header and footer groups into whole rows, detect explicit body-cell conflicts with header rows, preserve diagnostic and hint, and assemble cells in header/body/footer order. Return the conflict through the existing caller error channel without altering cell_align restoration. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
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

Normalização lógica de grupos inteiros/roles/conflictos continua exclusiva, disponibilizada por helper puro ao estágio Table; output lógico não é Frame. Physical path preparado recebe ranges já conhecidos, não concatena/pad grupos novamente. Grid legada conserva helper/diagnósticos. Casos ricos de header/níveis/linhas conservam limites medidos anteriores, sem nova promessa PDF/UA.

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

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Conflitos com header usam y/rowspan efetivos de Cell e o mesmo source_span; Grid permanece no contrato anterior. Grupos recebem clones com props já preparadas pelo owner de realização, não leem estilos nem executam receitas. Preservar mensagens/hints e fonte lexical.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/grid/row_groups.rs` nas linhas 82 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar a extração/cálculo de rowspan que não participa da decisão de conflito com o header. No modelo splice, header ocupa [0, header_rows), y é não negativo e o span normalizado é positivo: a interseção equivale a y < header_rows. Preservar GridCell/TableCell, posição y explícita e a seleção do primeiro conflito; não mudar placement nem resolver o scope-out do footer.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.


## Tabelas: exaustividade, origem e argumentos — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC/status/git diff HEAD --stat, fontes/L0/configuração e hashes em `lab/results/construction/diagnostics/tekt-v16-table-cells-2026-10-17/before.json.gz` (snapshot inicial de dois owners preservado). contexts.json identifica dez V16 e as projeções latentes, antes do código. public-before.json usa APIs públicas reais com rlib/probe/binário/SHA/argv: Source(Cell) perde x/colspan/rowspan, kind e span no header; footer perde Source; níveis distintos são rejeitados somente quando seus membros levam Source; grupo de footer Source muda células a/b para ab/vazio. snapshot-before.json mede context_floor 77→0 sob Source nas duas restaurações. cli-before.json/cli-extra-before.json/cli-controls-before.json registram sondas bilaterais com binários SHA completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. Footer com um argumento [a#strong[b]] gera duas células a/b no Tekt contra ab/vazio no vanilla; show table:none é indevidamente rejeitado. Controles [] ocultam header/footer sem executar panic de Cell; erros de colspan com alias conservam mensagem/caret. Header/footer colspan e grupos de nível distinto também expõem divergências de linhas, que este lote não dará por resolvidas. É semântica/morfologia/origem/erro, não obrigação de igualar estrutura Rust, bytes ou inferir intenção upstream.

### Prescrição exclusiva

Apenas no caminho table_groups, projetar Header/Footer Source e seus bodies para uma vista logística normalizada, em helper privado table_group_view com complemento nominal. Clonar os structs existentes, preservando repeat; Source continua na fonte original da Table. Usar essa vista nas chamadas existentes de combine_cells. Não alterar a política/algoritmo/padding de ocupação neste lote, nem o caminho Grid. As divergências spanning/níveis já medidas são pendências explícitas, não uma ratificação do algoritmo atual.

- **C01** — `01_core/src/compiler/layout/grid/row_groups.rs:76`, `combine_cells`: Somente GridCell/TableCell fornecem y de conflito; outros conteúdos não têm posição explícita nessa projeção. A política Grid não muda. Excluir somente `GridCell, TableCell`; guardados no complemento `(nenhum)`.

- **C02** — `01_core/src/compiler/layout/grid/row_groups.rs:129`, `table_groups`: Somente a Cell com placement.y correspondente fornece source_span no erro; todos os outros continuam None, inclusive Cell quando o guard falha. Excluir somente ``; guardados no complemento `TableCell`.

### Aceitação e pendências

L0 antes de testes/código. Fluxo contínuo ADR-0127: correção interna de Source e casts da linguagem, sem campo/tipo/assinatura/trait público, nova flag/default, fase ou compatibilidade externa. Constructor só assenta Cell crua: não chama receita, with_defaults, placement, fontes ou layout. A ordem Table show→defaults/placement→Cell show→body permanece. Grupos V19 ficam justificados por caso e visíveis; não provar cobertura por executar um braço. Não introduzir allow/expect, captura @ ou if-let que esconda a exaustividade nova. Preservar erros/casts legados não alterados, guards que falham, callbacks e restauração em erro/sucesso.

Regressões RED→GREEN nos owners: propriedades/presence/kind/origem de Cell explícita e Source, casts de footer por argumento e não por fragmento, formas cruas não preparadas, groups Source e duas restaurações de snapshot, negativos de tipos/Auto/None e shadow de chave local inválida, None em transformação show sem abrir aceitação de outros valores. Revalidar efeitos ocultados/late-rule pelo CLI e observáveis estruturais do pin; não reduzir testes ou normalizar whitespace para fechar paridade.

Sondas Content/Value/TableCellKind com variantes futuras reais em workspace descartável, cfg(test) compilado: os 18 matches finais devem exigir E0004, contando apenas uma vez cada match. Inputs e erro-testemunha cfg(test) preservados. Cargo target privado, nunca compartilhar artefatos mutantes/medições com o target principal. Gates: build, suíte workspace completa, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito e reversa, replay, diferencial SARIF completo e diff-check. V16 esperado 141→131; V19 só dos grupos prescritos; V20 sem crescimento, remover profundidade das duas projeções opcionais. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais. Sem commit/push/merge/release ou pin CI novo.

Incompletude explícita: header/footer spanning e limites entre níveis ainda exigem o pacote table-row-group-contract, com medição/L0 próprios. Nível do grupo e kind/nível semântico da Cell não são o mesmo conceito; não ratificar a inferência atual como contrato completo. Não corrigir ocupação com comprimento de Vec nem usar Source/Styles como marcador de fronteira. Preservar Grid legada. Demais casts/paint/insets ricos e compromissos históricos não são certificados por fechar V16. O objetivo geral, math-stretch-composite e as duas obrigações de matching permanecem abertos.
