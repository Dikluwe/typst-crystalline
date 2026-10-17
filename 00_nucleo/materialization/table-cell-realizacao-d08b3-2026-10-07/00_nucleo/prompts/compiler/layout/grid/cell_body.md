# L0 — compiler/layout/grid/cell_body
Hash do Código: 2a676922

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:dd8c7661f2246385e2aed1737e8494d778b9f563b7ac709fc3c750a23fe50596
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/grid/cell_emission.rs:95-583`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/cell_body.rs`. Dispor uma célula posicionada, com inset, alinhamento, subframe, decoração e caudas de overflow. Preservar save/restore do estado, ordem dos itens e índices de alinhamento adiados. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Solicitar o produto medido ao helper measurement_cache antes de instalar região da célula, depois de calcular body_x/body_w e alinhamento efetivo. Sem hit executar o subframe vigente; com hit conservar exatamente os pipelines posteriores de decoração, rebase, clipping, overflow, preenchimentos e caudas. Regras de validade e métricas laterais pertencem ao helper. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.



## Célula textual certificada — refutação 2026-10-06

### Medição anterior à decisão

focused-candidate-verification.json.gz e candidate-source.json.gz em
`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/` preservam candidato
com largura/âncora corretas mas ordem ABC/123/árabe incorreta em RTL.
bidi_origin.rs:111-117 recusa todas as células/subframes; o input integral
não chega à reordenação L3. cell_body.rs:223-235 classifica Text e TextShaped
como texto para rebase de baseline, mas não TextSeparator, introduzindo
um delta de top-edge nos separadores da mesma linha. São duas causas
internas refutáveis por ordem/baseline ainda divergentes após certificação
limitada e recomposição consistente. Correção de paridade interna, sem
contrato público, fase, flags de produto ou unsafe novo.

### Responsabilidade exclusiva

Instalar/restaurar capacidade privada somente para col_starts.len()==1 e body_w finito, no trecho de layout da célula. Classificar TextSeparator junto de Text/TextShaped ao rebasear sua baseline; não acrescentar avanço, altura ou tinta. Produtos cacheados ASCII continuam com certificado de equivalência vigente; não recertificar frames por string/posição.

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

Prepared TableCell consome resolved.visual/props, incluindo fill/stroke None conhecidos que não herdam novamente do container. Alinhamento/inset conhecidos, bounds/overflow/decoração/caudas e conservação de estado permanecem. Fallback Grid/raw mantém or/fold histórico. Usar cache somente sob prova de equivalência atualizada pelo seu owner; não executar recipes.

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
