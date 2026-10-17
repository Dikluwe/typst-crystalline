# L0 — compiler/layout/grid/tracks
Hash do Código: 2f4cdda7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/tracks.rs`. Comportamento original em `01_core/src/compiler/layout/grid.rs:212-451` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own Grid/Table column and row track resolution. Keep Fixed/Auto/Fraction order, constrained cell measurement, insets, page break before fractional row allocation, auto-height finite fallback, and current cursor/style effects. Return widths, starts and row heights without changing placement or emission. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Devolver também a pool privada do owner measurement_cache e transferir para ela o produto completo descartado após usar a altura Auto. O helper decide elegibilidade; Fixed/Fraction e cálculos geométricos continuam vigentes. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.

## Sequência em coluna Auto e frame literal — 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/before.json` conserva
estado/HEAD/stat/status/fontes L0 e código/binários. O lote predecessor em
espaco-terminal-2026-10-06/verification.json.gz mede seis tabelas: ABC/123 e
árabe ficam em duas linhas no Tekt, uma no vanilla pinado 9f2b6e871.
measurement.rs:104-113 toma max das larguras de filhos e soma alturas de
Sequence, mesmo quando Styled/Text/Space formam um fluxo inline. O vanilla
inline/grid mede o frame completo (grid/layouter.rs:967-1100). Block RTL com
LTR literal terminal desloca ABC em 3.49658pt por espaço: block.rs:315 usa
line_content_right que exclui todo TextSeparator. page_geometry.rs:143-154
já distingue extensão auto de espaço literal da extensão de tinta da linha.
tracks.rs:108-113 ancora toda coluna em margin.left, inclusive uma coluna
Auto RTL; no oracle a coluna única termina na margem direita. Observáveis
de geometria/texto, não obrigação de copiar algoritmo ou bytes. Hipótese
refutável por quebra/âncora ainda divergente ou regressão de espaço fraco,
coluna fixa, estilos/linhas/parágrafos após medição de frame/âncora corretas.

### Classificação

Correção interna de paridade ADR-0127, sem API pública/fase/flag/dependência
ou unsafe novo. Não inferir intenção histórica de P80. Núcleo de extensão
intrínseca distingue dimensões do container de tinta/alinhamento de linha.

### Responsabilidade exclusiva

Para uma coluna em região finita, alinhar a âncora do grid ao End RTL da
chain quando não há override horizontal, usando available_width menos soma
das larguras resolvidas, sem alterar larguras nem reverter ordem lógica.
Override Left/Right/Center/Start/End resolve contra direção externa. Colunas
múltiplas conservam a política anterior nesta rodada: seu espelhamento e spans
precisam de medição própria. Origins/insets/linhas/bordas consomem col_starts
existentes. Larguras Auto continuam delegando à medição restrita, agora apta
para Sequence inline. Fixed/Fraction, cache de células e fases conservados.


## Origem e override horizontal da região — 2026-10-06

Medição: os controles exploratórios de Align no diagnóstico
`00_nucleo/diagnosticos/bordas-cantos-2026-10-06.md` divergem na âncora Left
RTL de uma coluna. `tracks.rs:104-116` restringe o offset a uma coluna e usa
margem da página como origem, mesmo em sub-região.

Responsabilidade exclusiva: resolver um override horizontal explícito para
a largura total das colunas em qualquer contagem, usando região finita e
a origem corrente line_start_x. Sem override, preservar o caminho de uma
coluna e a política anterior de várias colunas; não inverter ordem lógica.
O fluxo lexical que instala/restaura o override pertence ao owner canônico
`00_nucleo/prompts/compiler/layout/placement.md`. Não duplicar a tradução no
emissor de células. Fixed/Auto/Fraction, paginação e cache permanecem.
Correção interna de posição ADR-0127, sem mudança pública.

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

Na entrada preparada medir o visual Complete em coluna/linha correta, usando colspan/rowspan e coords do plano, nunca chunk de células de origem como grade nem raw body substituído. Conservar tipos de tracks, metrics/styles e fórmula/insets/cache; corrigir só adaptadores necessários. Grid legada conserva ordem anterior. Não resolver recipes aqui.

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
