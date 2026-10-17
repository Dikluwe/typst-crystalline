# Gradientes PNG e raio externo dos contornos

O rasterizador agora amostra Linear, Radial e Conic em vez de usar a primeira
cor. Preserva alpha, stops, espaço cromático, foco radial, Self/Parent, raiz de
grupo/célula, transforms recebidos e paint do fundo. O raio externo de RoundedRect
com stroke também foi corrigido: a linha central desconta meia espessura.
Não é certificação de toda a paridade PNG nem conclusão da meta geral.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-final/results.json.gz
preserva UTC, status/diffstat exatos, hashes de todas as fontes/owners/núcleos
inspecionados e binários. candidate-final.json.gz e workspace-pre.json.gz registram
o estado final; verification.json prova que somente render.rs e seu L0 mudaram
entre as fontes produtivas do inventário. Diff próprio em render-change.patch.
Pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
CLI PNG com --ppi144; SVG de controle via rsvg-convert --zoom2 e DPI72.
Sem realinhamento, recorte por erro ou equivalência de PNG comprimido.

## Implementação e gates

L0 editado antes de código, correção interna de paridade ADR-0127. Sem API pública,
flag, default ou fase nova. Estado privado transporta dimensões da raiz e densidade.
Gradiente usa Pad/Nearest; tiling conserva Repeat e composição do período.
Alpha é premultiplicado antes da quantização. Retângulos negativos espelham Self.
Os raios de clips sem stroke permanecem intactos; fill fecha e rounded stroke
retorna ao início sem fechamento implícito, preservando cap/join.

red-functional-2.log registra cinco regressões funcionais; red-source.rs.txt
preserva o código. Os logs RED anteriores são tentativas de compilação das fixtures,
não evidência funcional. anisotropic-red.log isola densidade diferente por eixo;
round-red-2.log é o RED do canto externo com pixel verificado na sonda. As primeiras
fixtures before/before-valid continham stops/sharp inválidos e foram corrigidas em
before-final, com erros preservados. round-red.log usava ponto de controle inadequado;
o ponto correto está em round-red-2.log e round-red-source.rs.txt, antes da correção.
Oito testes novos estão incluídos na suíte final. Os resultados intermediários de
8.553 testes foram preservados em first-pass; não são a prova do estado final.

Suíte final UTC 2026-10-14T17:23:31.629509+00:00 → 2026-10-14T17:29:08.568224+00:00:
**8554 passaram, 0 falharam, 7 ignorados**, com source guard.
Build, rustdoc com -D warnings, check all-targets, fmt, diff e linhagem estrita
v1/v5/v15/v26 mais linhagem reversa passaram. Clippy permanece em 181 avisos,
inventário normalizado idêntico ao anterior; arquitetura em 382 avisos,
1642 notas e zero erros. Não confundir estes resultados com CI remoto
verde: cargo clippy -D warnings ainda bloqueia. Não houve commit/push/release.

## Resultado visual e limites

21 dos 28 controles principais coincidem pixel a pixel. Stops explícitos
divergem no máximo 1 nível de canal. A elipse tem diferença máxima 15 nas bordas;
o controle sólido também apresenta diferença de cobertura (máximo 13). Isso não
é aprovado por uma tolerância geral: os valores completos estão abaixo.
Contorno arredondado sólido e gradiente passaram de 396 pixels acima de 16 para
coincidência exata após o inset. Gradiente linear RGB, Oklab, LinearRGB, HSV e
Oklch coincidem nos controles registrados; HSL difere no máximo 1. CMYK permanece
divergente (máximo 114,4800 pixels acima de 16) e exige investigação de conversão.

| Controle principal | Máximo por canal | Pixels acima de 16 |
|---|---:|---:|
| cell-alpha | 163 | 4800 |
| cell-parent | 0 | 0 |
| cell-self | 0 | 0 |
| clip | 0 | 0 |
| conic-offset | 0 | 0 |
| conic | 0 | 0 |
| linear-alpha | 0 | 0 |
| linear-angle | 0 | 0 |
| linear-negative-angle | 0 | 0 |
| linear-oklab | 0 | 0 |
| linear-stops | 1 | 0 |
| linear | 0 | 0 |
| page-gradient | 0 | 0 |
| parent-box | 0 | 0 |
| parent-page | 0 | 0 |
| parent-rotate | 189 | 3598 |
| parent-scale | 185 | 3456 |
| radial-focal | 0 | 0 |
| radial | 0 | 0 |
| self-rotate | 252 | 6598 |
| self-scale | 252 | 5760 |
| sharp | 0 | 0 |
| solid | 0 | 0 |
| stroke-dash | 0 | 0 |
| stroke-ellipse | 15 | 0 |
| stroke-parent | 0 | 0 |
| stroke-rect | 0 | 0 |
| stroke-round | 0 | 0 |

O limiar 16 é inventário descritivo, não gate de aceitação nem tolerância ampliada.
Quatro casos rotate/scale ainda divergem. A mesma divergência de posição aparece nos
controles sólidos PNG e SVG; transform.rs conserva um pivô próprio. Não corrigir
essa posição distorcendo a amostragem de gradientes. O teste direto de Group afim,
Parent e siblings passa; o caminho fonte→layout de transformações continua aberto.

Overlap transparente é um caso em que não copiar o pin: typst-render/src/paint.rs:
296–308 rasteriza diretamente num canvas do período e recorta parte da célula.
No controle constante a 50%, vanilla PNG fica(255,127,127), enquanto as quatro
contribuições exigem alpha 93,75%. No gradiente variável, a composição explícita
Tekt fica a no máximo 0.833911 nível de canal do cálculo analítico.
O controle sem overlap coincide. Preservar composição autorizada no L0/núcleo de
tiling; a diferença do pin nesse caso não deve virar regressão deliberada.

## Continuidade

A meta geral permanece aberta: posição/origem de transformações, conversões CMYK,
avisos de CI e demais refutadores dos relatórios anteriores ainda precisam de
correção. Esta etapa não certifica Glyph direto, fontes afins, EXIF, alocação,
performance/caching, PageFill Tiling ou toda a produção. As junções de PDF ficam
na investigação separada tiling-juncoes-2026-10-14.md; este trabalho não mudou PDF.

## Evidência recuperável

evidence.json.gz contém 663 entradas path/sha256/base64, reabertas e
comparadas com os arquivos atuais. SHA-256 `528b4c286382d293f2e7918754d88cc6d8b364ea487c74e9b19337c44daf7515`. Inclui fontes RED/final,
scripts, comandos, imagens, resultados e logs completos. Os scripts de mutação
são registros históricos, não devem ser reexecutados sobre o estado final.
