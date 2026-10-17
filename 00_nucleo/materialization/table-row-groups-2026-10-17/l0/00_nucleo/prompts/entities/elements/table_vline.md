# Prompt L0 — `entities/elements/table_vline` — `TableVLineElem`
Hash do Código: 9f5ae1b3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade
**P739A** — `stroke` passou a `Option<Stroke>`: `stroke: none` compila e a linha não é desenhada (paridade vanilla, medido; zero-thickness seria hairline em PDF — achado P726). O render (`compiler/layout/grid.rs`) salta linhas com `stroke: None`.

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_vline.rs`
**Origem**: Passo 512 (linhas em grid/table). Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableVLineElem {
    pub start:    usize,
    pub end:      Option<usize>, // None = até à última linha
    pub col:      usize,          // coluna da tabela onde a vline se posiciona
    pub stroke:   Option<Stroke>, // **P739A** — None = `stroke: none` (linha não desenhada; paridade vanilla)
    pub position: EcoString,      // "left" | "right"
}
```

`Content::TableVLine(TableVLineElem)`.
Construtor ergonómico: `Content::table_vline(start: usize, end: Option<usize>, col: usize, stroke: Option<Stroke>, position: EcoString)`.

O campo `col` não é argumento público de `table.vline()`: é calculado por `native_table()` a partir da ordem dos children durante o parsing.

## `impl Element for TableVLineElem`

| método | comportamento |
|---|---|
| `plain_text` | string vazia |
| `is_empty` | `false` (linha é conteúdo visual) |
| `map_content` | devolve `Content::TableVLine(Arc::new(self.clone()))` (terminal) |
| `map_text` | devolve `Content::TableVLine(Arc::new(self.clone()))` (terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `start + end + stroke + position`.

## Critério

`plain_text` vazio; `map_*` terminal; igualdade estrutural.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Substituir pub col: usize por pub x: Option<usize> (None=auto), adicionar pub set_fields:u8. Constantes públicas TABLE_VLINE_SET_X=1, TABLE_VLINE_SET_START=2, TABLE_VLINE_SET_END=4, TABLE_VLINE_SET_STROKE=8, TABLE_VLINE_SET_POSITION=16. Outros tipos preservados. Intrínsecos x auto,start0,endNone,stroke padrão,position start; transportar alinhamentos start/end/left/right sem perder intenção lexical. Campos raw só conhecidos pelos bits, schema x,start,end,stroke,position. Terminal puro, não-locatável. Resolver direção/posição no owner de layout/planner, sem baking na origem.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
