# Baseline inicial de Box sem ascent fictício

Corrigida a origem da caixa não fracionária que inicia uma região sem
texto externo. O código instala o ascent da própria caixa, incluindo zero,
e combina a aresta real de texto se ele aparecer depois na mesma linha.
Caixas com texto menor também deixam de herdar ascent da fonte externa.

## Proveniência e medição anterior à decisão

A rodada anterior fez progresso: corrigiu composição/contornos de Box e
origem legada de subframes, com suíte verde; deixou explicitamente aberta
a caixa pequena inicial. Esta rodada mediu esse caso antes de decidir.

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshot inicial em 2026-10-14T03:05:59.614174+00:00, com fontes, owners, status e git diff
HEAD --stat em before.json.gz. before/results.json.gz registra doze fontes,
comandos, PDFs, extração, binários e UTC. O estado final está em
pre-validation.json.gz; workspace-pre.json.gz captura todos os fontes Rust
antes da suíte. Não atribuir alterações anteriores da árvore a este lote.

Caixa 8×6pt sozinha, página 100×80pt/margem 10pt: a tinta começa em pixel
[20,22] no Tekt e [20,20] no pinado a 144dpi. A mesma caixa com Q depois
começa em [20,22] nos dois lados. Sequência de caixas e caixa aninhada
reproduzem o deslocamento sem texto; baseline shift 10pt evidencia a
contribuição indevida quando a baseline própria fica negativa. Move e
rotate servem de controles que já passavam.

boxed.rs:21-38 instalava baseline de fonte antes de conhecer a caixa.
line_commit.rs:421-437 e sub_frame.rs:292-300 tratavam zero como ausência;
o fechamento da linha não considerava texto de tamanho uniforme quando
precisava elevar uma baseline inicialmente menor. A fonte pinada
inline/line.rs:559-568 acumula top/bottom dos frames presentes, começando
em zero. A geometria observável, e não o formato de buffers, fundamenta
a decisão. Os L0 foram atualizados antes da implementação.

## Reparos e testes

line_assumed_ascent agora é Option<f64> privado: None significa ainda não
instalado; Some(0) é válido. O núcleo installed-line-ascent fixa essa
distinção para os sete owners. Cursor, origem legada, Raw e checkpoint
preservam suas fórmulas e o valor instalado, inclusive durante rollback.

Box não fracionária em origem geométrica pendente compõe o corpo antes
de instalar max(baseline,0). LineCommit e a cauda do subframe acomodam
max(ascent inline, aresta do texto real), sem exigir tamanhos mistos.
O caminho fracionário e a calibração parágrafo→Box não foram migrados aqui.

A primeira execução RED teve cinco falhas e um controle verde. Uma delas
era uma fixture mal delimitada: a largura externa 20pt era toda ocupada
pela caixa interna 20pt, portanto Q quebrava legitimamente. O controle foi
corrigido para largura 40pt, mantendo os valores esperados de baseline.
Os testes finais foram repetidos contra os sete fontes produtivos anteriores:
**quatro falharam e dois controles passaram**. red-final-state.json.gz,
red-final.log e red-final-result.json preservam esse estado; a tentativa
inicial permanece arquivada e não é contada como defeito de produto.

Um sétimo teste detectou um problema no primeiro candidato: o máximo de
fonte da cauda começava no estilo exterior, aumentando indevidamente uma
fonte menor. candidate-text-red.log tem seis passes e essa falha; o
snapshot correspondente conserva código/L0. O cálculo agora parte dos
itens textuais reais e só usa o estilo exterior se nenhum existir.
font-before confirma: texto 10pt sozinho coincide, mas dentro de Box
Tekt tinha yMin 8.298pt contra 7.640pt. Depois ambos têm 7.640pt.
Os **16 testes de composição**, incluindo os nove da rodada anterior,
passaram em green.log; nenhum esperado de regressão existente foi relaxado.

A primeira suíte completa depois disso encontrou duas regressões core:
P1381 prefixo8pt e P1392 wrap em colunas. A correção da baseline usava
current_text_top já substituído pelo estilo exterior para calcular o
avanço, elevando texto menor e alterando paginação. first-candidate.json.gz
preserva os fontes e toda essa suíte falha. O reparo captura a aresta do
texto emitido antes do fallback de avanço; linhas mistas e fórmulas de
leading permanecem como especificadas. Os testes existentes mantêm suas
posições/páginas esperadas. core-corrected.log e a suíte final abaixo
revalidam a separação, sem tratar a primeira execução como aprovada.

## Sondas finais

Tekt debug SHA-256 eb5ff271f3eb0edc3986903e6771783f30f78de8794910d947a35210b49bce69; referência vanilla SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea,
pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4. Os binários anteriores dos
controles estão identificados por hash, não por uma string de versão.

Depois, a caixa sozinha começa em [20,20], a sequência e o aninhamento
conservam suas origens corretas, e o caso com texto continua em [20,22].
As doze sondas iniciais e os dois controles de texto 10pt coincidem no
raster a 144dpi com o pinado; pixel-comparison.json guarda os hashes dos
dois lados. Baselines, coordenadas e conteúdo são a aceitação; identidade
raster nestas entradas não prova paridade geral de Box ou do compilador.
Os quatro controles adicionais de altura automática mantêm as mesmas
caixas/textos e alturas de página: caixa 6pt/caixa vazia 6pt dão 26pt,
altura zero dá 20pt, e caixa 6pt seguida de Q dá 27.238pt, com margens 10pt.
auto-page-sizes.json e auto-controls preservam os dois lados.

## Gates do estado final

Em 2026-10-14T03:24:40.396843+00:00, check all-targets, build, Rustdoc -D warnings,
formato, linhagem estrita V1/V5/V15/V26, linhagem reversa e diff check
passaram. Arquitetura continua com zero erros, 382 avisos e 1620 notas;
não declarar zero violações. Clippy conserva 228 diagnósticos sem delta,
e o inventário cap-lints warn não aprova CI -D warnings.

Suíte completa workspace offline/locked, serial, tmpfs e oráculo comprovado:
**8381 passaram, 0 falhas e 7 ignorados**,
concluída em 2026-10-14T03:31:11.925941+00:00. As fontes não mudaram durante a execução;
comandos e ambiente estão nos recibos. A aprovação é local, não a matriz
remota Windows/macOS/i686/Docker nem uma release publicada.

## Continuidade do objetivo

A pendência de baseline pequena inicial do diagnóstico de composição foi
corrigida. A etapa B permanece aberta e agora tem medição própria em
[box-fracionaria-2026-10-14.md](box-fracionaria-2026-10-14.md): o sufixo
pode receber a fração correta enquanto fundo, clip e corpo mantêm largura
provisória ou escapam da linha externa. Só redimensionar o fundo não
satisfaz essa evidência. Uma sonda causal adicional em
[box-contexto-2026-10-14.md](box-contexto-2026-10-14.md) mostrou posições
locais (0,0) expostas por here().position() mesmo com inset/prefixo. Esse
rebase de metadados deve ter prioridade antes de declarar Box pronta e
antes de adiar corpos fracionários sem preservar identidade/ordem. A
baseline do context direto difere do vanilla e requer classificação
separada, sem copiar uma coordenada por mera divergência. Etapa C,
Block, conic, demais itens da auditoria, Clippy e CI/release seguem abertos.

Sem commit, push ou publicação. A correção é interna ADR-0127, com L0
primeiro e resselo limitado; nenhuma API ou fase pública nova.

## Evidências

Snapshots .json.gz preservam os estados e as tentativas. evidence.json.gz
contém logs, receitas, resultados, diffs e artefatos com SHA-256 verificado
após descompressão. O diff deste lote é relativo ao snapshot inicial;
o status global conserva separadamente a árvore anterior não commitada.
