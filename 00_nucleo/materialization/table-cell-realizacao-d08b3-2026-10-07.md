# D08B3-A — resolução da tabela na inserção, antes de show da célula

Pacote ratificado pelo dono em 2026-10-07 (`comfirmo`); execução e limites registrados em `00_nucleo/diagnosticos/table-cell-realizacao-d08b3-execucao-2026-10-07.md`. Manifesto SHA-256 `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`.
Candidatos L0 completos/núcleo/owners em `00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07/`.

## Medir antes de decidir

Coletas `table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz`:
pin 9f2b6e871, binários/HEAD/working tree/diffstat/fontes/UTC e casos completos.
Antes: campo/recipe da Cell chegam cedo demais e o grid só reconhece GridCell;
upstream resolve propriedades/holes antes de Cell show. Fonte e classificação
estão nas medições anteriores à decisão dos candidatos.

Regra Table que substitui tudo não executa Cells. Cell que substitui body não
executa Heading ocultado. Colisão auto→explicit rejeita, explicit→auto passa;
metadata das células é row-major, não ordem de argumentos. Rows omitidas em
Table sem children dão zero Cells; rows explícitas geram vazios. None explícito
vence fill/stroke herdado; stroke parcial faz fold; fixed/fr rows não são breakable.
Aliases medidos: LOCAL a, OUTER LOCAL a, NEW a. HTML também observa x lógico.

## Ordem e responsabilidade

Table show-root → default da Table sobrevivente → ocupação/estilos/padding
lógicos → Cell show-root → produto/body realmente emitido → índice/HTML/layout.
No mesmo Engine causal, sem reavaliar o programa ou chamar Func no Layouter.
Algoritmo único no owner grid_placement, com política Table DocumentOrder e
Grid legacy ExplicitFirst. Geometria, medidas/fontes/tracks/paginação ficam no
layout, que recebe o plano. Preparo por inserção crua; replay/contribuição aceita
não repete efeitos. Corpo/children lexicais não são visual paralelo consultável.

## Contrato público revisável

Adicionar em entities/elements/table_cell os tipos/carrier exatos:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableCellStage { Prepared, RecipeApplied, Complete }
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellResolved {
    pub x: usize, pub y: usize,
    pub colspan: usize, pub rowspan: usize,
    pub fill: Option<Color>, pub stroke: Option<Stroke>,
    pub align: Option<Align2D>, pub inset: Sides<Length>,
    pub breakable: bool, pub visual: Content,
    pub(crate) stage: TableCellStage,
}
```

TableCellElem recebe `pub source_span: Span` e
`pub resolved: Option<Arc<TableCellResolved>>`. Os campos anteriores e presence
permanecem lexicais. Constructor cru tem resolved=None; source_span é causal,
não propriedade da linguagem. Body getter conserva o corpo original (incluindo
Sequence vazia canônica); resolved Some faz os demais nove getters projetarem
valores conhecidos. Align None resolvido é Auto conhecido, fill/stroke None
resolvidos são None conhecidos. A projeção visual usa resolved.visual, nunca
substitui body lexical. Clone/maps/hash/observação conservam o carrier e stage.
Somente stage Complete pode chegar à publicação/render; Prepared é estado de
resolução e RecipeApplied é snapshot de it cuja regra já foi consumida, mas cujo
corpo ainda deve ser realizado se emitido. Aliases desse snapshot não reaplicam
a própria receita. Essa marca é nominal, privada à crate, sem Location/registry.

Adicionar em entities/elements/table o carrier exato:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableGridResolved {
    pub cells: Vec<Content>,
    pub num_cols: usize, pub num_rows: usize,
    pub header_rows: usize, pub footer_start: usize,
    pub(crate) complete: bool,
}
```

TableElem recebe `pub source_span: Span`, `pub rows_explicit: bool` e
`pub resolved: Option<Arc<TableGridResolved>>`. Constructor cru tem resolved=None.
Rows explícitas são distinguidas do fallback [Auto] omitido; sem children e sem
rows explícitas não fabricar célula. Header/footer e children originais
permanecem como fonte lexical. Plano contém somente origens de células reais ou
vazias, não entries duplicadas dos spans; células são ordenadas row-major.
header_rows é faixa [0,n); footer_start é num_rows quando não há footer.
Getter/serialização não expõem esses carriers como campos novos da linguagem.
Travessia visual/emptiness/plain_text com resolved Some consomem cells completas
uma vez, não os children/header/footer crus em paralelo. Map/clone preservam
fontes e transportam visual; não executar receita em entidades.

Registrar descendente privado compiler/realize/table_cells com owner próprio.
Adicionar exatamente a fachada pública pura:

```rust
pub fn realize_table_insertions(
    content: Content,
    ctx: &mut crate::compiler::eval::EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> crate::entities::source_result::SourceResult<Content>;
```

Usa Engine/contexto causal recebidos e restaura modos/styles/rules/guards/file
em erro e sucesso; não reavalia fonte nem cria World/índice/layout novo. Realizar
inserções de Table nos pontos efetivos (incluindo produtos de show e context),
antes de indexação final, coordenando com bare_math sem substituir sua guarda.
Captura contextual privada existente pode conservar regras Table/Cell além de
Equation; nenhum estado resolvido é armazenado em Styles/custom. O plano só é
construído após a regra da tabela deixar uma Table para emitir; child/cell é
fronteira terminal até o seu momento, evitando a travessia bottom-up prematura.


São cinco campos públicos em entidades, dois carriers públicos novos e uma
fachada pública. Stage/private complete controlam publicação uma vez; nenhum
carriers/Source/span vira campo de linguagem/Location. Source spans não entram
em == da linguagem. Propriedades resolvidas conhecidas continuam observáveis:
raw cell != it preparado, enquanto raw cells iguais == true. JSON/getters
permanecem com seus owners; entidades não conhecem Layouter/render.

## Escopo e limites explícitos

43 owners 1:1 (um novo, privado) têm candidato completo e hashes de base.
D08B3-A busca os 17 negativos primários de realização e seus refutadores de
ordem, alias, padding, herança, contexto, conteúdo ocultado e HTML lógico.
D08B3-B completa defaults de Cell isolada e decisão de fresh Cell recursiva;
não ratificar por acidente a política antiga de terminação não-localizável.
Defaults/fields genéricos de Table e rich styles/casts/linhas/spans multipágina
ficam abertos. Nem diagnóstico nem prompt pode fechar paridade universal.

## Gate e aceitação

ADR-0127 exige confirmação para campos/tipos/função públicos e mudança de fase.
A aprovação deste manifesto autoriza instalar candidatos/núcleo e materializar
somente D08B3-A. Antes de instalação verificar hashes/reconciliar base com recibo,
nunca resetar árvore anterior. Testes RED→GREEN com efeitos/ordem/ocultação são
obrigatórios; validar fields/query/HTML e geometria real sem apagar diferenças
de whitespace. Preservar cache somente com prova de equivalência e revalidar
controles de subframes, textos, fontes e posições. Cargo build/suites/lint zero
erros e receipt de preservação restrita. Não publicar candidato provisional.
