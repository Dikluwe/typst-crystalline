# L0 — compiler/realize/table_cells
Hash do Código: 00000000

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/realize/table_cells.rs`

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

Possuir somente a realização de Table e células na inserção. Ordem: Table show-root (via apply owner) → resolução lógica pelo placement owner → Cell show-root (via apply owner) → realização do produto/corpo (via owners existentes). Restaurar contexto causal em todo retorno. Elementos/receitas ocultados não são visitados, inclusive heading original. Tabela substituída não constrói plano. Planos completos são novos por inserção de fonte crua; contribuição já aceita/visual completo é reutilizada sem efeitos extras. Respeitar aliases globais tardios e join local: constructor cru não assa OLD; inserção local do CodeBlock pode produzir LOCAL antes de entrar como body de nova célula sob OUTER. Não distinguir esses casos por nome de variável/AST ad hoc. Resolver herança tipada: x/y/spans do plano, inset/align/fill/stroke e breakable conforme presence/rows; None explícito vence herança. Alignment fold por eixo, Stroke fold por componentes specified; unidades de inset permanecem Length, sem resolver largura física. Table sem rows explícitas/children tem zero células; rows explícitas fixam mínimo, gaps e última linha recebem vazios exceto slots merged. Header/footer inteiros e conflitos são delegados aos owners existentes, que passam a aceitar contexto de tabela. Publicar TableGridResolved com cells row-major, visual de cada célula Complete; delegar sintaxe/selectors/terminação e localização de produtos a seus owners, sem segunda interpretação, vtable, I/O, globals ou FixedMetrics para geometria.

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
