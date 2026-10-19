# PDF: isolamento de caminhos sem pintura — 2026-10-14

Corrigido o vazamento do caminho de Shape sem fill/stroke para a pintura ou
recorte seguinte. O PDF já não pinta o backing invisível do controle de Block
nem amplia o clip de um irmão. O reparo é exclusivamente do exportador L3;
outset/overflow do layout e rejeição de polygon(stroke:none) continuam abertos.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-source,
red-state, candidate-final e workspace-pre preservam fontes, hashes, UTC,
status e git diff HEAD --stat. Suíte completa: 2026-10-14T11:56:44.848169+00:00 →
2026-10-14T12:01:48.802373+00:00. Fontes mantiveram hashes durante testes e validação.
Nenhum commit, push, PR, publicação ou release foi realizado.

Referência vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Os probes conservam argv, fontes, stdout/stderr, UTC, hash de cada binário,
artefato e imagem. A rasterização usa 2px/pt, sem recortar ou realinhar a saída.
A versão textual não substitui proveniência.

## Medição antes da decisão

O lote anterior block-regiao-2026-10-14/open-pdf flagrou um retângulo azul extra
no PDF com Block(outset) sem fill/stroke, ausente no SVG. A sonda isolada desta
rodada separou exportação de layout: Place mantém as formas fora do fluxo,
e Rect/Ellipse/RoundedRect invisíveis precedem um marcador azul. O PDF Tekt
pinta desde (20,20), em vez de somente o marcador (70,60). A falha ocorre nos
modos normal e compacto, atravessa Group e amplia a máscara posterior.

Na fonte anterior draw.rs:513–517 e :922–926, ambos os dispatchers construíam
o caminho, executavam um ramo vazio quando fill/stroke eram None e fechavam Q.
A geometria pendente entrava no operador de pintura ou clip seguinte. Os cinco
REDs observaram precisamente isso: retângulo invisível participava do conjunto
pintado ou do conjunto recortado. Não se infere intenção histórica do código.

O reparo termina o caminho sem pintar nos dois ramos, preservando envelopes,
ordem, tags, recursos, transformações e todos os ramos com pintura efetiva.
Não modifica o produtor L1 para esconder a falha L3. O owner
00_nucleo/prompts/infra/export/stream/draw.md foi atualizado antes do código;
resselo restrito ao par 1:1 com draw.rs, sem núcleo novo ou contrato público.
Correção interna ADR-0127 em fluxo contínuo.

## Resultados e limites

O corpus tem dez fontes, em PDF normal/compacto, PDF vanilla e SVG dos dois
binários. Nove fontes chegam à exportação em todas as variantes. Uma fonte,
polygon(stroke:none), é rejeitada pelo constructor Tekt antes do exportador;
essa falha permanece explícita, sem ser contada como coincidência visual.
A primeira tentativa do probe parou nesse erro; o corpus completo foi refeito
com registro das falhas. Ambos os registros estão preservados.

Entre os nove casos exportáveis, pixels PDF idênticos ao pin passaram de
**2/9 para 9/9**
no normal, e de **2/9 para
9/9** no compacto. Isso inclui fill/stroke,
clip, grupo aninhado, saída de Group, formas invisíveis e controles positivos.
Os 9 SVGs Tekt exportáveis permaneceram pixel a pixel iguais
antes/depois. Não é uma exigência de igualdade de bytes PDF nem uma prova de
paridade universal.

Os 40 PDFs finais dos dois conjuntos passaram qpdf. No controle
original de Block, o retângulo extra desapareceu: a tinta agora fica só no
marcador em (17,43), igual ao SVG Tekt. O pin usa (17,35), portanto a diferença
de fluxo de outset ainda está aberta. O clip de Box/outset agora também deixa
de ganhar a região invisível no PDF; a origem e expansão próprias de outset
continuam incorretas no layout. Overflow na raiz conserva a página vazia extra.

Cinco testes de regressão foram observados em RED e passaram em GREEN. O
interpretador de teste acompanha quais retângulos entram em pintura/clip;
não exige a string n e aceitaria outra implementação geometricamente correta.
As variantes cobrem top/local, saída de Group, clip posterior, três operações
de pintura e Compact/Verbose. As imagens finais de clip foram inspecionadas.

Suíte completa: **8515 passaram, 0 falharam,
7 ignorados**. Build offline/locked, rustdoc -D warnings,
all-target check, fmt, diff check, V1/V5/V15/V26 estritos e reverse lineage
passaram. Inventário Clippy: 181 avisos, sem aumento.
Linter completo: zero erros, 382 warnings e
1629 notes. As duas notes V19 novas estão no
interpretador de teste: alternativas f/S/B e operadores de estado sem pintura.
As três operações de pintura são exercitadas explicitamente. As notas ficam
registradas; não se afirma zero violations nem aprovação de Clippy -D warnings.

Esta falha confirmada era de código/integração do exportador. O L0 ganhou a
invariante explícita e seus refutadores antes do reparo; isso não determina a
proporção de falhas de código versus prompts no projeto inteiro. CI remoto,
release e demais lacunas de produção continuam abertos; objetivo permanece ativo.

## Evidências preservadas

[evidence.json.gz](pdf-caminho-vazio-2026-10-14/evidence.json.gz) contém
316 entradas com caminho, SHA-256 e bytes base64, relidas e verificadas.
SHA-256 do arquivo: `818f26ac3be22f3e4a0462878c26a8670c744a1fdc6d65e1f3f03547ab8f18c1`.
Inclui estados, patch, RED/GREEN, gates/SARIF, suíte completa, SVG/PDF/PNG,
resultados e scripts históricos. Não reaplicar os scripts de mutação.
