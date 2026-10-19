# Alinhamento de transformações — investigação causal

Estado: diagnóstico, implementação pendente. Não é Prompt L0 nem autorização
para código sem atualizar os owners. A meta geral continua aberta.

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Cada results.json.gz conserva UTC, status/diffstat exatos, fontes e SHA-256
dos binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8; receita/árvore
P1489 e SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Sondas SVG/PNG sem realinhamento, raster 2px/pt; diferença de pixels não é
classificação automática de defeito. Lados físicos, direção, conteúdo e
alinhamento são observáveis da linguagem; a organização Rust pode divergir.

## Medições

| Corpus | Documentos | Início UTC | Fim UTC |
|---|---:|---|---|
| rtl-before | 30 | 2026-10-14T19:30:56.268895+00:00 | 2026-10-14T19:31:18.592832+00:00 |
| rtl-scope-before | 14 | 2026-10-14T19:33:09.365621+00:00 | 2026-10-14T19:33:17.516289+00:00 |
| rtl-mixed-before | 15 | 2026-10-14T19:34:36.427671+00:00 | 2026-10-14T19:34:56.316890+00:00 |

São 59 documentos, cada um compilado nos dois binários para SVG e PNG.
color-interiors.json e mixed-color-interiors.json registram posições em pontos
obtidas dos interiores de cores exatas, com hashes das imagens. Não representam
bordas antialiasadas nem substituem medida geométrica geral. A imagem de
rtl-start-child-flip foi inspecionada.

1. Transform nativo na raiz ignora o alinhamento padrão RTL: com página300,
   margem70 e shape24, o frame deveria começar em206; começa em70.
   transform.rs:587 usa sempre line_start_x. Origin Start/End e posição do
   frame são responsabilidades distintas; somar uma compensação no pivô
   mascararia a causa.
2. Align geométrico alinha o conjunto dos filhos. Em mixed-center, red24 fica
   em130, enquanto o pin o centra em138; blue40 fica em130 em ambos. Em
   mixed-right, red fica em190 em vez206. Cada frame deve receber alinhamento
   próprio. placement.rs:200–208 calcula uma largura agregada para o envelope.
3. Start/End resolvem a direção efetiva de cada filho. Em mixed-start o pin
   põe red RTL em206 e blue LTR em70; Tekt põe ambos em70. Em mixed-end o pin
   põe red em70 e blue em190. Quando o body troca RTL externo por LTR, alguns
   casos atuais coincidem por acidente. Normalizar Start/End pela direção
   externa antes de compor o body quebraria esses controles. O defeito também
   aparece em Align textual, não apenas em Transform.
4. Largura automática precisa conservar alinhamento por frame até conhecer
   a largura final. rtl-auto requer red em96 e blue em70; Tekt põe ambos em70.
   Há ainda um erro vertical independente: blue começa em92 no Tekt e99.5 no
   pin; no wrapper Align começa em86. Não classificar todo pixel diferente
   como erro horizontal nem escondê-lo alinhando as imagens.
5. Há uma dependência de dimensões relativas. Rect(width:50%) sob Rotate0
   em região160 fica com largura160 no Tekt; o pin usa80 com reflow false e40
   com reflow true. helpers.rs:427–438 descarta Ratio/Relative no fallback e
   Length só lê a parte absoluta; shape.rs:150–151 chama esse helper.
   Não tratar uma segunda composição numa região menor como correção neutra:
   a região é observável para porcentagens. Essa falha exige auditoria do L0
   de shape/helpers e controles fora de transformações antes do código.
6. Texto multilinha RTL também diverge. cursor.rs:167 alinha linhas ao limite
   da região antes da largura intrínseca final; sub_frame.rs:264 pode deixar a
   linha final sem alinhamento; transform.rs:651 usa corretamente a largura
   causal, mas não recupera por isso o alinhamento de cada linha.

## Fonte e decisão de investigação

O pin typst-library/src/layout/align.rs:335–340 e436–441 resolve Alignment
pela direção no StyleChain. flow/collect.rs:245 captura alinhamento de cada
bloco; flow/distribute.rs:682–690 posiciona cada frame pela própria largura.
Essas leituras explicam os controles; não exigem copiar o pipeline upstream.

A hipótese de correção é transportar o alinhamento lógico até o produtor da
linha/frame, resolvê-lo com a direção efetiva e aplicar a posição uma vez na
largura lógica final. O encaixe no protocolo privado de fixups deve preservar
índices, decoração, labels, recursos, rollback e referência local. Registrar
largura causal antes do deslocamento visual, para que RTL não infle o frame.
Proibir recomposição indiscriminada: percentuais e efeitos da medição são
refutadores. A hipótese ainda precisa L0, RED→GREEN e validação ampla; não é
contrato materializável nem escolha arquitetural ratificada.

Owners a auditar antes de implementar: transform, placement, cursor, sub_frame,
layout/checkpoints, shape/helpers e testes. Se houver novo contrato público ou
mudança de fase, aplicar ADR-0127; correções internas continuam com L0 primeiro.
As correções de largura lógica da etapa anterior permanecem. source-guard.json
comparou os arquivos produtivos/L0 com a validação final anterior e não encontrou
mudanças. Nenhum código ou L0 foi alterado nesta investigação, nenhuma suíte nova
foi alegada, nenhum commit/push/release foi realizado.
