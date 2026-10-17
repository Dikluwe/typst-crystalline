# L0 — grid/measurement_cache
Hash do Código: 7afb91a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid/measurement_cache.rs`
**Governança:** ADR-0107/0108/0109/0127/0129/0130.

## Evidência antes da decisão

`grid/tracks.rs:175` constrói o subframe de altura Auto e descarta todos os itens; `grid/cell_body.rs:155` os reconstrói na emissão. `grid.rs:218–239` mede antes de placement. O diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` mediu diferenças de contexto e documentou a remoção histórica do cache P234 por reordenação. A etapa layout-inicial removeu um relayout completo, mas não esse par medição/emissão. O baseline e as fontes atuais estão preservados em `00_nucleo/diagnosticos/frames-celulas-2026-10-05/before-state.json.gz`. Fonte `paragraph_plan.rs:48–56` recusa planejamento em subframes, e `line_commit.rs:493` não consome contador raiz dentro de subframe. `word_layout.rs:21–55` emite uma palavra mantida sem callback/identidade; `sub_frame.rs:314–346` devolve produtos locais e restaura estado pai, preservando last_sub_frame_width/bottom como resultados laterais. Isso motiva um experimento limitado; não prova antecipadamente ganho.

## Unidade e fronteira

Possuir um cache consumível de produtos medidos, privado a uma invocação de grid. Despacho estático descendente, sem campos novos em Layouter/entidades, sem I/O, unsafe, globals ou retenção entre layouts/compilações. Não usar Hash/PartialEq visual de Content, endereço cru nem gerar IDs. Grid coordena o produto, tracks o captura após a medição e cell_body o consome antes de instalar região da célula; os owners delegam as regras a esta unidade.

Aceitar somente token ASCII alfanumérico não vazio em Text, Sequence com um único material Text e Empty neutro, e wrappers GridCell/TableCell de placement sequencial sem alinhamento/coords/spans especiais. Não aceitar Styled/Realized/Located/contexto/qualquer locatable nem outros carriers. Todos os cells do grid devem ter placement simples (x/y ausentes, spans ausentes ou 1); sem isso, não construir cache. Linhas Fixed/Fraction não são capturadas.

Condições ambientais de captura/consumo: fora de subframe/célula/column mode/page columns, sem smallcaps, floats pendentes, transação de sufixo ou checkpoint pendente, sem par.line.numbering ativo, e alinhamento de célula ausente. Guardar o estilo pai, margens laterais e presença do coletor de decoração na criação do cache; rejeitar mudança na captura/consumo. Comparar o TextStyle efetivo atual com o TextStyle emitido medido (igualdade de dados mecânicos de render, não identidade de linguagem).

O registro precisa identificar row/col de entrada, origem x e largura de corpo finitas/positivas, correspondência do token, altura e resultado completo do subframe. Capturar somente resultado de uma única folha FrameItem::Text cujo texto é o token e largura lateral last_sub_frame_width cabe na região (tolerância exclusivamente classificatória de 1e-9 pt para cancelamento em x absoluto, sem alterar geometria), sem alinhamentos órfãos. Conservar decoração e last_sub_frame_width/bottom. No consumo exigir row/col e spans 1 do PlacedCell, mesma origem e largura, mesmo token/estilos/ambiente. Consumir no máximo uma vez, retirando a entrada; divergência ou ausência executa o layout anterior. A pool usa índice derivado de row/col apenas sob essa prova de placement, nunca como chave geral.

Ao reutilizar, repor last_sub_frame_width/bottom e devolver o produto para o pipeline de emissão vigente: insets, rebase y, decoração, fill/stroke, clipping, overflow e caudas continuam nos owners existentes. Não emitir diretamente fora deles. O cache não recupera regiões diferentes por tradução ad hoc de x.

## Aceitação

RED→GREEN: duas células Auto de um token requerem dois subframes totais, não quatro; conteúdo e métricas laterais preservados. Fixed rows, alinhamento, posicionamento explícito e conteúdos com identidades conservam fallback. Cobrir multipágina, insets, RTL, estilos, overflow, fontes e diagnósticos com controles existentes e sondas adicionais. Captura/consumo provados pelos testes e contagens externas; ganho decidido por CLI pareada com fontes/binários/estado/timestamps. Falha de saída ou ausência de benefício refuta o experimento e deve ser documentada. Ampliações exigem nova prova, não variantes adicionadas por conveniência.

O cache é interno na mesma fase, sem API externa/default/compatibilidade: fluxo contínuo ADR-0127.

Refinamento medido na sonda: para Beta/FixedMetrics, last_sub_frame_width foi 26.400000000000006 e body_w 26.4; o critério estrito descartava o produto por cancelamento de f64 em cursor_x − origin_x. A tolerância classificatória acima permite esse carrier, mas origem/largura de entrada e o produto continuam exatamente iguais. A revalidação em `00_nucleo/diagnosticos/frames-celulas-2026-10-05/width-probe.json.gz` conserva esses números, a sonda cfg(test) e seu estado exato; o log discovery original permanece exploratório. Recusar ambiente matemático também conserva o fallback.

Provas locais do helper: origem ou largura diferentes invalidam e consomem a tentativa do registro, sem hit posterior; mudança de tracking no canal custom da chain altera o TextStyle efetivo e exige fallback mesmo quando o estilo pai achatado não mudou. Esses testes complementam a integração de grid e legitimam a guardas mecânicas, não paridade por igualdade de Content.

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

Plano preparado não prova elegibilidade sozinho. Conservar cache para o subcaso comprovado de coords sequenciais unit-span e visual ASCII token, mesmos styles/insets/origins/métricas e sem metadados/locatables/contexto/efeito; medir/consumir uma vez. Callbacks ou mudanças de corpo/ambiente forçam fallback. Acrescentar prova para a nova representação sem recertificar por igualdade de Content nem relaxar controle antigo de dois subframes; sem cache fora da invocação.

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
