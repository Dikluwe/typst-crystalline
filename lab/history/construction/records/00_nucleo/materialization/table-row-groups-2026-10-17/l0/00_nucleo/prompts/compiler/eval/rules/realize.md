# L0 — compiler/eval/rules/realize — Realização de parágrafos
Hash do Código: e938d7ae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab


Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/realize.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/compiler/eval/rules.md`.

## Contrato de linguagem

Realizar fluxo em nós Par antes de show quando aplicável; preservar scoped flow, estilos, containers, matemática e folhas sem duplicação ou quebra de morfologia. A geometria pertence ao layout.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).

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

Transformações de parágrafos/containers conservam novos campos/plano/etapa e respeitam fronteira de corpo ocultado; não construir entidade perdendo causalidade. Não substituir aplicação root-only Table/Cell do owner apply.

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


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-seventh/composition_show_par refuta Source como fronteira de scope/flow: classificar Source.body em Par/carriers e scoped prefix, transformar flow/inline/nó conservando origem. Não criar segundo parágrafo ou permitir estilo interno vencer show-set externo por wrapper novo; matemática segue seu owner.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## RG1 — refinamento compartilhado de fase — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Atualizar somente o pin do núcleo compartilhado. A distinção entre validação estrutural da Table e receita Cell/body substitui a leitura histórica de “Table antes de tudo”; este consumer conserva sua responsabilidade e código, sem novas operações locais, contratos próprios ou alargamento de escopo. Visual completo continua sendo a única vista publicada.

### Gate

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
