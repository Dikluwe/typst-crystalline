# D08B2 — contrato de presença lexical para revisão

Ratificado pelo humano em 2026-10-07 e materializado no escopo lexical. Manifesto SHA-256 `35f73c60301e1e4b27ae8b3ed2928b6d421dbcd0f32502131c3e2458260301e6`.
Candidatos L0 completos e núcleo: `00_nucleo/materialization/table-cell-presenca-2026-10-07/`.

## Medição anterior à decisão

A coleta `00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz`
e os refutadores `presence-refuters.json` registram o pin 9f2b6e871,
HEAD/working tree/diffstat/UTC e binários. Option atual apaga diferenças
observáveis entre campo omitido, auto e none explícitos. Ver owner candidato
para file:line e classificação de linguagem.

## Contrato revisável

Adicionar ao owner da entidade os tipos públicos fechados:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CellFieldPresence { #[default] Omitted, Auto, Explicit }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TableCellPresence {
    pub x: CellFieldPresence, pub y: CellFieldPresence,
    pub colspan: bool, pub rowspan: bool,
    pub fill: CellFieldPresence, pub stroke: bool,
    pub align: CellFieldPresence, pub inset: CellFieldPresence,
    pub breakable: CellFieldPresence,
}
```

TableCellElem recebe `pub presence: TableCellPresence`; todos os campos
atuais conservam seus tipos. Body permanece sempre conhecido. Omitted não
projeta campo; Auto projeta Value::Auto; Explicit projeta o valor conservado.
Somente fill/stroke explícitos podem ter Option None e projetam Value::None.
X/y/align/inset/breakable Explicit exigem Some; Auto exige None. Colspan e
rowspan presentes exigem Some positivo. Não codificar presença no kind,
em strings, Value genérico, PropMap ou registry. map_content/map_text,
clone/Hash preservam presence. Getter desconhecido continua None.


O manifesto enumera 15 owners 1:1, seus hashes atuais e candidatos completos.
Native table.cell captura presença dos argumentos com value_span; os constructors
internos assentam Some e preservam omitidos; transformações/observação conservam o
novo carrier; CLI projeta campos conhecidos em JSON/YAML. Não há mudança de fase.
D08B3 de resolução por posição/estilo/padding permanece separado e não ratificado.
Inset relativo/dict e paints/strokes ricos além dos carriers atuais permanecem dívida.

## Gate e validação

ADR-0127 exige confirmação para campo/tipos públicos novos. Aprovação deste
manifesto autoriza instalar esses L0s, ressellar e materializar D08B2 com
RED→GREEN, coletas bilaterais, preservação da árvore anterior, suites/build/lint.
Se um owner/hash atual mudar, reconciliar o candidato e registrar recibo novo antes
da instalação. A aprovação não autoriza D08B3, NodeKind cell ou query locatable.

## Execução ratificada

Recibo humano: 00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json.
Resultado e limites: 00_nucleo/diagnosticos/table-cell-presenca-execucao-2026-10-07.md.
Manifesto/candidatos aprovados permanecem imutáveis; os refinamentos internos
medidos de casts/corpo vazio estão nos L0s canônicos e nos recibos da execução.
D08B3 continua não ratificado.
