# Contrato público candidato — RG1

Este documento descreve a mudança Rust para revisão; não é código produtivo nem L0 proprietário. Os L0s completos candidatos estão em `l0/00_nucleo/prompts/` e só serão instalados depois da aprovação.

## Entidades e migração

| Tipo | Mudança pública | Migração externa |
|---|---|---|
| TableHeaderElem | adiciona level: NonZeroU32 e set_fields:u8 | literal precisa de nível positivo e máscara; clone/map deve conservar ambos |
| TableFooterElem | adiciona set_fields:u8 | literal informa máscara de repeat |
| TableHLineElem | row:usize substituído por y:Option<usize>; adiciona set_fields:u8 | None=auto, Some(n)=índice explícito; não usar zero como auto |
| TableVLineElem | col:usize substituído por x:Option<usize>; adiciona set_fields:u8 | mesma distinção em x |
| TableElem | remove header,footer,hlines,vlines; children guarda todos os itens em ordem | mover cada grupo/linha para a posição documental correta em children; não concatenar por papel |
| TableGridResolved | remove header_rows/footer_start; adiciona row_groups,hlines,vlines; novo is_complete() | percorrer intervalos; não inferir header como prefixo nem footer por número de entradas; ler visual só completo |

Os campos não citados mantêm tipo. Content::table/table_with_caption/table_cell/table_header/table_footer/table_hline/table_vline mantêm assinaturas. Seus adapters Rust de Header/Footer tratam repeat fornecido como explícito e Header usa nível1 omitido; linha com argumento row/col fornecido vira Some desse índice, conserva todos os cinco argumentos de propriedades assentes e normaliza a string Rust histórica auto ao lado intrínseco. A API nativa da linguagem valida alinhamentos/auto/none conforme medições; não herda a permissividade antiga de strings e argumentos ignorados.

### Cinco tipos públicos novos no módulo entities/elements/table

```rust
pub enum TableRowGroupKind { Header { level: NonZeroU32 }, Footer }
pub struct TableRowGroup {
    pub kind: TableRowGroupKind,
    pub rows: Range<usize>,
    pub repeat: bool,
}
pub enum TableLineSide { Before, After }
pub struct ResolvedTableHLine {
    pub source: TableHLineElem,
    pub row: usize,
    pub side: TableLineSide,
}
pub struct ResolvedTableVLine {
    pub source: TableVLineElem,
    pub col: usize,
    pub side: TableLineSide,
}
```

Enums são nominais fechados (Clone/Copy/Debug/PartialEq/Eq/Hash); TableRowGroup deriva Clone/Debug/PartialEq/Eq/Hash; wrappers de linhas derivam Clone/Debug/PartialEq como seus payloads. Coordenadas de plano são dados derivados; source retém propriedades/omissões originais. `rows` é intervalo half-open real e grupos ficam em ordem de origem, mesmo quando coordenadas explícitas mudam a ordem geométrica. `repeat` é elegibilidade efetiva; a fonte lexical conserva o pedido original. Header vazio/line-only reserva uma linha; Footer só pode ocupar o final real. O papel do grupo não sobrescreve Header/Data explícito da Cell. Defaults Auto em padding de Header usam nível do grupo como escolha Tekt (ver inferência em review).

### Constantes de presença

Módulo table_header: TABLE_HEADER_SET_REPEAT=1, TABLE_HEADER_SET_LEVEL=2.
Módulo table_footer: TABLE_FOOTER_SET_REPEAT=1.
Módulo table_hline: TABLE_HLINE_SET_Y=1, TABLE_HLINE_SET_START=2, TABLE_HLINE_SET_END=4, TABLE_HLINE_SET_STROKE=8, TABLE_HLINE_SET_POSITION=16.
Módulo table_vline: TABLE_VLINE_SET_X=1, TABLE_VLINE_SET_START=2, TABLE_VLINE_SET_END=4, TABLE_VLINE_SET_STROKE=8, TABLE_VLINE_SET_POSITION=16.

Demais bits reservados. `children` de Header/Footer sempre existe, inclusive vazio. Omitir repeat/level não equivale a torná-los campos raw conhecidos. Linhas distinguem índice auto explícito de omitido; end:none/stroke:none explícitos não se perdem.

### Plano antes da receita

`TableGridResolved.complete` continua crate-private. `pub fn is_complete(&self)->bool` é a nova leitura pública, sem efeitos/alocação. Some(plan) com false contém estrutura validada, mas células ainda não são visual realizado. Leitores de render/introspecção só consomem true. A receita Table vê `children` lexicais, sem Cell show/defaults assados neles. Fontes e plano são vistas distintas, nunca duas emissões. Nenhuma constante/flag CLI muda.
