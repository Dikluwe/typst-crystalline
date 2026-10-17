# Grupos aninhados e caminhos de recorte no PDF

Corrigidos dois defeitos do código que contrariavam a geometria prevista
nos L0: grupos aninhados ignoravam posição/matriz/clip, e os caminhos de
recorte invertiam Y novamente. Os prompts receberam a especificação
precisa do reparo antes da implementação; o núcleo compartilhado
pdf-group-local-space fixa coordenadas e escopo gráfico.

## Proveniência e medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot
inicial em `2026-10-14T01:25:36.513475+00:00`. before.json.gz contém fonte/L0 de draw,
status e diffstat; clip-before.json.gz preserva fonte/L0 do helper e a
falha que revelou sua segunda reflexão. O binário anterior tem SHA-256
`9163155b83268d800cafbcd31adba2a4b8024494df71433f17ffa83d77107637`. Os snapshots RED e first-validation
preservam tentativas intermediárias e seus resultados, sem substituir
falhas históricas por sucesso posterior.

A origem foi a investigação de conic documentada em
[conic-cobertura-2026-10-13.md](conic-cobertura-2026-10-13.md).
O Group de página aplicava posição/matriz/clip em draw.rs:530-565;
o Group local em :947-952 apenas visitava os filhos. O helper de clip
em shape.rs:215-259 ainda negava os valores Y. O teste histórico de
texto aninhado verificava presença de Tj, não sua posição.

## Correções e provas causais

- Group local salva/restaura estado, compõe matriz e posição sem nova
  reflexão de página, aplica o clip e pinta os filhos uma vez.
- Rect e Path de clip usam Y-down; RoundedRect conserva os quatro
  cantos físicos por permutação vertical dos raios no formatter Y-up.
  Ellipse e Line possuem braços explícitos, sem fallback de caminho vazio.
- Dois testes de desenho interpretam operadores e verificam pontos
  calculados independentemente, composição não comutativa, recorte e
  restauração de irmãos. Três testes de caminhos verificam vértices,
  controles, extremos e identidade de cantos assimétricos.

Os dois primeiros testes falharam antes da implementação. O primeiro
candidato fez passar a composição, mas expôs o recorte deslocado. Os
três testes de caminho também falharam antes do reparo do helper.
Nenhum esperado foi relaxado. Os cinco novos testes permanecem na suíte.
A primeira validação detectou um aviso novo de complexidade no retorno
do interpretador de testes; uma struct de resultado nomeada o removeu.
A primeira tentativa de resselo rejeitou pins fora de ordem; os pins
foram ordenados e o gate passou, sem alteração do linter.

## Verificação visual independente

Fontes Typst compararam composição dupla com operação equivalente:
translação, rotação e escala passaram de diferenças para identidade de
pixels em canvas inteiro a 144 dpi. Os valores e comandos estão em
before-renders.json e after-renders.json. Não se usou vanilla para
definir os esperados.

O controle fonte→PDF block(clip:true) continua falhando. No stream,
seu grupo interno recebe largura zero e o filho ainda tem coordenadas
externas. Controles adicionais mostram também block direto deslocado
e box com pintura fora do grupo de clip. Isso exige investigação do
produtor dos frames; a correção de exportação não fecha esses casos.
clip-source-controls.json conserva fontes e streams completos.

Para isolar o exportador, uma receita Rust forneceu frames com caixas
e posições conhecidas. Nos dois modos Compact/Verbose, RoundedRect,
Ellipse e Path recortados coincidem pixel a pixel com formas equivalentes;
o irmão azul fora do recorte permanece. Rect tem 12 pixels distintos
na coluna da borda direita em cada modo, diferença máxima de canal 32.
Os streams e o teste geométrico colocam a borda no mesmo ponto. Ao
repetir somente esse controle com -aaVector no, a diferença desaparece
nos dois modos. Evidência compatível com suavização de clip versus fill
no Poppler; não foi alterada a tolerância para alegar identidade.
Os 16 PDFs dos frames passaram qpdf --check.

## Gates e limites

Validação estática em `2026-10-14T01:32:54.167832+00:00`: build,
check workspace/all-targets, formato, Rustdoc -D warnings, linhagem
estrita, linhagem reversa e diff check passaram. Arquitetura:
0 erros, 379 avisos, 1614 notas.
Clippy mantém **228 diagnósticos**; o inventário com cap-lints warn
não equivale ao gate CI -D warnings, que continua aberto.

Suíte infra concluída em `2026-10-14T01:34:36.867547+00:00`:
**1321 passaram, 0 falhas, 2 ignorados**.
Cargo offline/locked, execução serial, tmpfs isolado, linter publicado
pinado e vanilla de proveniência verificada apenas onde a suíte já o usa.
workspace-pre.json.gz registra comandos, ambiente, HEAD/estado e hashes;
workspace-result.json confirma fontes inalteradas durante os testes.
Não foi executada a suíte completa do workspace neste lote.

Binário final das sondas fonte→PDF:
`ddb376464df76ebe6052072a824de3ffffb5782805a526316665c851d1db09e5`. As bibliotecas e o executável do controle
por frames têm hashes próprios em frame-renders.json. Sem commit,
push, publicação, mudança de contrato público ou fase do pipeline.

Conic, os produtores de frames de block/box, os demais itens funcionais
da auditoria e a matriz remota de CI/release permanecem abertos.
Esses resultados não estabelecem a distribuição global de falhas entre
código e prompts: os dois defeitos corrigidos aqui são do código, mas
outros L0 ainda possuem cláusulas históricas de rascunho a revisar.

## Evidências

Os snapshots .json.gz preservam estados. evidence.json.gz reúne logs,
inventários, diffs, fontes Typst, PDFs/PNGs, receitas Python/Rust e seus
hashes, conferidos após descompressão. As imagens de fonte e frames são
corpora distintos; não misturar seus critérios nem usar contagens como
prova de paridade geral.
