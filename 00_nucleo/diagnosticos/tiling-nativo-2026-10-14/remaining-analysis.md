# Fronteiras ainda abertas nesta migração

Medição: `comparison.json`, `native-v2/results.json.gz` e
`extended-v2b/results.json.gz`, com binários/HEAD/diffstat/UTC próprios;
fontes correspondentes em `candidate-v2.json.gz`. Os números não certificam
produção. `native-v1` é diagnóstico intermediário e não prova de fechamento.

- Negativo/alpha: no ponto (60,60) do canvas 480×280, SVG Tekt e vanilla dão
  (255,127,127); PDF Tekt (255,0,0), vanilla (255,127,127); PNG Tekt (255,15,15),
  vanilla (255,127,127). A célula tem 8×8, pitch 4×4 e alpha50%: quatro
  contribuições dão alpha93.75%, logo PNG Tekt segue a composição exigida
  pelo contrato. Não copiar o corte do vanilla. PDF ainda perde alpha;
  SVG overflow="visible" não faz librsvg compor os quatro contribuintes.
- Rotação37° com spacing(-2,4): SVG tem máximo2 e zero pixels com erro>16;
  PDF tem1070 e PNG451 pixels >16. O PDF Tekt usa BBox8×6, pitch6×10 e
  translação normalizada; o vanilla BBox6×10. Matriz linear corresponde;
  translações diferem por cinco períodos em X. A fase contínua é equivalente,
  mas a rasterização não. Não copiar offsets/erros do pin por tentativa.
- Hipótese a medir antes de implementar: PDF/SVG podem representar um período
  explicitamente, referenciando a célula vetorial única para os contribuintes
  de overlap. BBox do período seria pitch; cada ocorrência da célula mantém
  clip a size. Isso não gera grade proporcional ao viewport nem clones L1.
  Exige atualizar L0, testes RED e validação analítica de ordem/alpha antes
  de código. Refutador: fase/overlap diverge do controle analítico ou muda
  os controles simples já exatos. Não é proposta pública nova.
- PNG de gradiente continua usando cor representativa preexistente; o caso
  cell-parent-gradient tem4500 pixels>16. PDF do mesmo caso tem máximo1;
  SVG é exato. Não declarar gradiente raster corrigido.
- SVG vanilla text-cell contém caractere inválido em atributo e librsvg
  rejeita XML. PDF/PNG de texto coincidem. Erro externo fica preservado,
  sem "consertar" o oráculo. A fixture image-cell foi inicialmente rejeitada
  por ambos por root. A tentativa after-final com --root explícito revelou outro
  erro no CLI Tekt: o caminho absoluto do input vira basename na raiz. Esse caso
  permanece aberto no owner do CLI. before-cleanup/final-renders copiam tiny.jpg
  para a pasta da fixture; a imagem passa nos três targets sem esconder o erro de --root.
- Offset grande: PDF/PNG coincidem, SVG vanilla diverge. A redução periódica
  tem controle analítico no helper; não alinhar a saída por pixels nem copiar
  perda de precisão sem medir semântica.
- Testes futuros de T3: ampliar controles de imagens/subpixel, alpha/overlap,
  parent com transforms completos, seleção real de páginas com célula aninhada,
  texto externo e PDF marcado, fill-rule/contornos complexos, singularidade,
  limites de alocação e diagnóstico (PDF ainda usa expect em placement inválido).
  Não promover o pacote enquanto houver caminhos de panic/omissão silenciosa.

A correção de reuso entre páginas já tem RED/ GREEN próprio: registro global
reutilizava descendente da página excluída; agora o cache de nomes é por página.
A suíte final usa fontes posteriores a essa correção, em candidate-final.

A sonda final subpixel (período0.1pt, corpo0.05pt) conclui nos três targets Tekt;
o PNG vanilla pinado entra em panic por Pixmap de dimensão arredondada a zero.
O Tekt deve manter a amostragem coerente, sem copiar o panic. A comparação
geométrica desse caso ainda precisa de controle analítico; sucesso do processo
não basta para afirmar qualidade. A seleção real apenas da página2 passou em
qpdf e preservou PAGE TWO sem PAGE ONE; selected-page/result.json contém prova.

A limpeza posterior em libm/higiene não alterou pixels Tekt nas sondas válidas.
Memória/tempo físico de transporte ainda não têm medição final suficiente para
certificar o critério completo de performance de T1–T3.
