# Tiling: sobreposição vetorial e alpha sólido PDF — 2026-10-14

PDF e SVG agora compõem explicitamente um período de tiling, referenciando uma
célula vetorial única e recortada. O PDF conserva alpha sólido de formas, linhas,
texto/glifos, canvas e células aninhadas, com recursos ExtGState e restauração
de estado gráfico. O transporte público aprovado permanece intacto.

**Progresso integrado e verificado; T2/T3 ainda incompletos.** O controle PDF de
quatro cores revelou marcas nas junções periódicas; gradientes PNG, alpha
variável de gradientes e guardas extremos continuam abertos. Não é certificação
de produção, fechamento da meta geral ou autorização de release.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, working tree não commitado.
`before.json.gz`, `candidate-probe.json.gz`, `candidate-final.json.gz`,
`workspace-pre.json.gz`, `pre-validation.json.gz` e `final-state.json.gz`
registram fontes/owners, SHA-256, UTC, status e diffstat. O manifesto aprovado
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` e seus candidatos
foram revalidados sem alteração. O recibo humano continua no pacote aprovado.

Pin vanilla: `9f2b6e8715237cb086899a42873660fe744622e8`; árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Binário Tekt final: `8069a856f2ffb19820272e18b3e85405d2d5ee5fff2fcee8b4b04afe6b9748db`.
Os registros de renders contêm comandos, fontes e hashes dos binários. PNG:
144 PPI; SVG: librsvg 72 DPI ×2; PDF: pdftoppm 144 DPI. Nenhuma saída foi
realinhada por pixels. A máscara analítica é a caixa autoral, não uma seleção
pelo erro.

## Causa e correção

O código PDF descartava alpha. O L0 de tiling também tinha uma hipótese
insuficiente: confiar no overlap do Pattern PDF e em overflow visible do SVG.
A medição anterior mostrou PDF opaco e SVG com apenas uma contribuição.
Portanto esta frente tinha defeitos no código e na especificação do mecanismo;
isso não permite estimar a maioria dos defeitos de todo o projeto.

L0 atualizado primeiro, em fluxo interno ADR-0127, com novo Núcleo de alpha
compartilhado e ownership 1:1. O helper de tiling enumera as células que cruzam
um período, na ordem y/x crescente. PDF guarda o corpo num Form XObject com
BBox=size; o Pattern tem BBox=pitch e referencia o Form. SVG guarda um g
recortado e usa referências traduzidas dentro do pattern. Não há clones L1
nem grade proporcional ao viewport.

Recursos PDF coletam pares fill/stroke de alpha f32 sem quantização u8, inclusive
em células. Os dispatchers isolam folhas translúcidas; o número de restaurações
pendentes mantém ActualText antes de Q. O prefixo verboso compartilhado cobre
runs fundidos. Os controles opacos conservam seus operadores.

## Verificação

- Suíte completa de `2026-10-14T16:42:35.716694+00:00` a `2026-10-14T16:48:07.159581+00:00`: **8546
  aprovados, 0 falhas, 7 ignorados**. Hashes das
  fontes verificados antes/depois; execução com TMPDIR exclusivo em /dev/shm.
- Build, Rustdoc com -D warnings, all-targets, formato, diff-check, linhagem
  estrita v1/v5/v15/v26 e linhagem reversa passaram.
- Clippy mantém **181 avisos**, sem aumento. Foi inventário
  com cap-lints warn; o gate de produção -D warnings não está liberado.
  Arquitetura: **0 erros, 382 avisos,
  1637 notas**.
- RED→GREEN: padrão com quatro contribuições/corpo único; alpha e restauração
  antes de folha opaca seguinte, com ActualText em top/local e compacto/verboso.
  Interseções parciais do período também têm teste analítico.
- A primeira montagem de testes tinha tipos de mapas/cap incorretos; red.log
  conserva esse erro de fixture. red-2.log e red-alpha.log são as falhas
  funcionais anteriores à implementação. A expectativa SVG foi ajustada de
  atributo fill-opacity para o CSS rgba que o exporter já usa, sem mudar alpha.
- Todos os renders Tekt válidos de initial-renders e final-renders são idênticos;
  a preparação final alterou apenas formato/linhagem e fortaleceu os testes.
  Os PDFs do corpus passaram qpdf.
- Oito controles reais em semantic-v2 cobrem compacto/verboso, tagging padrão,
  desligado, UA-1 e seleção da página2. qpdf e pdftotext passaram: CELL não vira
  texto de página; seleção preserva PAGE TWO sem PAGE ONE. Isso não certifica
  conformidade integral PDF/UA. A fixture inicial usava title como content e
  foi rejeitada antes de exportar; semantic/fixture-error.json conserva o erro.

## Resultados e refutador visual

Na célula8×8 com pitch4×4 e vermelho alpha50%, quatro contribuições dão
alpha93,75%. Nos **4.800 pixels** da caixa autoral, PDF/SVG/PNG produzem
(255,15,15), erro máximo0,9375 contra composição analítica em canais de8 bits.
O vanilla pinado mantém apenas uma contribuição; essa divergência do Tekt é
intencional e correta segundo o contrato aprovado, não um defeito a copiar.

O controle de ordem usa quadrantes amarelo→verde→azul→vermelho a50%. O valor
analítico final é (159,375;63,75;79,6875). SVG e PNG são uniformes (159,63,79),
erro máximo0,75. PDF dá (158,62,78) nos pontos internos, mas a inspeção da
**caixa inteira** encontra junções: erro máximo7,75 e p95=4,75; nenhum pixel
supera16. O número16 é somente a métrica descritiva já usada no corpus,
**não um novo limite de aceitação**. O controle não está fechado como uniforme.
A origem exata das junções ainda precisa de isolamento; não se atribui intenção
nem se relaxa o contrato para aprovar a imagem.

O PDF do controle rotate-offset agora coincide em todos os pixels com o pin;
antes tinha1.070 pixels com diferença>16. Os demais controles antigos sem
overlap preservam os renders anteriores. Os novos controles solid-alpha,
text-alpha e text-alpha-cell coincidem com o pin em PDF. Comparações completas
estão em final-comparison.json e final-analytic.json.

## Fronteiras ainda abertas

- Junções PDF do período multicolorido descritas acima.
- Gradientes PNG continuam em cor representativa; no controle anterior há
  4.500 pixels com diferença>16. A rotação PNG conserva451 pixels>16. Esses
  números vêm de tiling-nativo/final-comparison e do corpus desta etapa,
  não de certificação de qualidade.
- Alpha variável de gradientes PDF exige trabalho próprio; não usar alpha de
  uma parada como substituto. Os perfis de conformidade continuam com o escopo
  limitado já declarado pelo owner PDF.
- Ranges extremos, underflow de size/pitch, singularidade e alocação ainda
  precisam diagnóstico integrado. As chamadas expect do PDF e do enumerador
  não constituem uma política de erro de produção. Não há truncamento do padrão
  nem fallback sólido para fingir sucesso.
- Continuam os demais itens de T3: transformações/contornos complexos,
  fractional Box/Block fragmentado, qualidade subpixel e performance física.
  O erro CLI --root já medido, os avisos de CI e o restante da meta geral
  permanecem abertos.
- O SVG vanilla de texto de célula contém XML inválido; o PNG vanilla subpixel
  entra em panic. Erros externos preservados, sem alterar o oráculo.

Nenhum commit, push, PR ou publicação. A aprovação cobre implementação/testes;
release continua excluída.

## Evidência recuperável

`tiling-overlap-2026-10-14/evidence.json.gz` contém **545 entradas**
{path,sha256,base64}, reabertas e verificadas também contra os arquivos atuais.
SHA-256: `9f545a03c08829ada4fd0305ccc8700926da94cf1e4fced6d6f5203d57bbbd35`.
