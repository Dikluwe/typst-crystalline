# Block Auto: largura causal e regressão PDF — 2026-10-14

A regressão PDF de gradiente parent introduzida pela raiz de pintura foi
corrigida no controle reproduzido: a dimensão de Block automático agora vem
do fluxo emitido, com inset, e não da largura disponível. O PDF passou de
3792 pixels com diferença >16 contra o pin para zero, máximo de 1. Os outros 35
renders do corpus Parent/Solid permaneceram iguais aos da entrada.
Isso fecha esse refutador específico, não os recursos nativos de tiling,
o referencial afim universal nem a preparação para produção.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-source,
before-grid-owner, red-state, candidate-final, pre-validation e workspace-pre
conservam fontes, SHA-256, UTC, status/diff HEAD --stat. Grid foi incluído
mais tarde, antes de sua primeira edição; o patch preserva essa distinção.
Não houve commit, push, PR ou publicação.

Oracle pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`,
receita P1489; binário lab/typst-original/target/release/typst SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
As sondas before/after-first/parent-after-first registram ambos os binários,
fontes, comandos e estado. O corpus Parent compara com after-first do lote
paint-root anterior, cujo arquivo de evidências permanece intacto.

## Medição, decisão e implementação

Na entrada, Block com corpo de 40×24 pt e inset de 7 pt emitia fundo de 160 pt; o pin, 54 pt.
Empty e Place-only precisavam de 14 pt; Grid e Stack com larguras de 20+30 pt e espaçamento de 4 pt precisavam de 68 pt.
A projeção inicial do script buscava a cor blue errada (#0000ff). As imagens
originais não foram alteradas: blue-geometries-corrected.json extrai #0074d9
dos mesmos SVGs, com dimensões exatas dos paths/rects. Nenhuma ausência de cor
desse primeiro extrator serve como medição de comportamento.

Layouter conserva origem/largura privadas por escopo. Os produtores publicam
alocação de fluxo: Shape, target de Image, linhas antes do alinhamento visual,
tracks de Grid, composição de Stack e Columns. Block isola o corpo, restaura
o pai e publica sua alocação completa; clip/backing/paint_root usam a mesma
largura. Subframes devolvem largura causal separada do campo legado;
Place não publica sua pintura no fluxo do pai. Checkpoint conserva os novos
campos. Não há segunda execução do corpo nem mudança de API/fase/default.

O primeiro RED válido está em red-corrected.log: largura de 160 em vez de 54 pt e
perda do avanço literal terminal. red.log/red-behavior.log registram problemas
anteriores do observador de cor/import, não são apresentados como RED semântico.
A implementação revelou dois casos adicionais: gutter ignorado em grid.rs
(_gutter), e linha final de subframe que não passa por flush_line. O primeiro
agora reserva o gap horizontal antes das tracks e desloca cada origem uma vez;
o segundo usa o mesmo projetor intrínseco antes de alinhar/drenar. O teste
Grid conserva esperado de 68 pt; não foi reduzido para aceitar a ausência do gap.
O gutter vertical permanece incompleto.

Três testes novos cobrem dados transportados e geometria de containers, Literal vs Markup
e isolamento de subframe/rollback. green-third.log: três passaram. O projetor
de separadores foi tornado exaustivo após V16, sem mudar sua projeção.
Núcleo compartilhado layout/flow-frame-width.toml, pin
`9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e`;
ownership produtivo permanece 1:1 e resselo restrito ao manifesto.

## Validação bilateral e limites observados

O corpus Auto original tem 30 pares com/sem clipping; o ampliado
produz 38 pares SVG válidos. As larguras de backing coincidem em
36/38, e as dimensões width/height em 18/38.
Essas contagens são dimensões do container, não certificação da posição dos
filhos ou do render completo. comparison.json conserva cada caso. Matemática
continua com largura de 88.6315 contra 24.263 pt: centralização antecipada é tomada
como ocupação. Altura de texto, Box inline, nested Block e Stack ainda diverge.
**Regressão aberta:** Align com clipping exige realinhamento dos filhos à
dimensão Auto final. next-controls.json mede pixels vermelhos do filho:
antes 2880, depois 0, pin 3840; o clip correto de 54 pt substitui o antigo de 160 pt, mas a
âncora do filho ainda usa a região antiga. Não certificar este render nem
restaurar a largura falsa como solução. Resolver o alinhamento após conhecer
a dimensão final, preservando decorações e caminhos de metadados.
Gutter negativo é aceito pelo pin e recusado pelo Tekt nos dois controles;
eles não entram na contagem de renders bem-sucedidos nem são aceitos.
Grid com duas linhas ainda omite 4 pt de gap vertical.

Parent usa canvas inteiro de 120×96 pt em 240×192 pixels, sem corte/alinhamento; Auto
usa 160×120 pt em 320×240 pixels. Rsvg a 72 dpi e zoom 2; PDFs Parent passam por qpdf.
O controle PDF corrigido foi também inspecionado visualmente ao lado do pin.
SVG Parent inalterado não equivale a prova de paridade: sua integração ainda
precisa do contexto afim e dos recursos da célula preparada.

## Verificação geral

A primeira suíte foi encerrada externamente com status 143; o processo/filhos
foram confirmados ausentes antes de reiniciar. Log parcial preservado em
workspace-interrupted; não conta como sucesso. A repetição completa, com
fontes estáveis, terminou de 2026-10-14T13:40:42.547646+00:00 a 2026-10-14T13:45:47.122152+00:00:
**8528 passaram, 0 falharam, 7 ignorados**.
Build offline/locked, rustdoc -D warnings, all-target check, fmt/diff,
V1/V5/V15/V26 estritos e reverse lineage passaram. Clippy inventariado:
181 avisos; isso não passa o gate -D warnings de CI.
Linter completo: zero erros, 382 warnings,
1629 notes. CI remoto e release continuam abertos.

## Continuidade

Resolver alinhamento tardio e altura Auto de Block, preservar decoração e
metadados ao deslocar filhos, completar gutter vertical/validação negativa
nos owners de Grid. T1c ainda troca Shape.fill/stroke pelos carriers aprovados;
T2 implementa patterns PDF/SVG/raster e recursos recursivos; T3 fecha geometria.
Box fracionário, Block fragmentado, outset e demais pendências do objetivo
integral continuam abertas. Sem redefinir sucesso como apenas este corpus.

## Evidência

[evidence.json.gz](block-auto-width-2026-10-14/evidence.json.gz): 572 entradas
com caminho/SHA-256/base64, relidas e verificadas, incluindo fontes antes/depois,
patch, logs, arquivos de render e scripts históricos.
SHA-256 `cf3111dcca378a4fffdc3492388d59f0401b2bf4a4831182594ef9f82c6d4b5b`.
