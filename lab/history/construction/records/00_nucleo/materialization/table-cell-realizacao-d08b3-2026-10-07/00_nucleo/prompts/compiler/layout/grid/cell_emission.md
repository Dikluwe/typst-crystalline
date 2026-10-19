# L0 — compiler/layout/grid/cell_emission
Hash do Código: e64ab788

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/cell_emission.rs`. O comportamento original estava em `01_core/src/compiler/layout/grid.rs:276-882` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Emite as células posicionadas de Grid/Table linha a linha, incluindo quebras de página, alinhamento e inset herdados, subframes, caudas de overflow, ordem de fill/stroke e descarregamento dos segmentos de borda. Devolve os inícios das linhas emitidas para as regras visuais explícitas. Preserva o restauro do estado do Layouter, a ordem dos itens, o ajuste dos caminhos e o suporte a regiões limitadas. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Encaminhar a pool consumível de tracks a cada cell_body durante a emissão, conservando paginação e ordem. Não definir elegibilidade nem chave neste owner. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.

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

Emitir células de origem por row/col do plano, sem duplicar merged slots, em ordem pertinente. Header/footer físicos repetem visual preparado, não a recipe; preservar paginação, caudas, strokes/fill e ranges recebidos. Sem nova execução/contexto.

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
