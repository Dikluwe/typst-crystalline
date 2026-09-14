# P1393 — o que falta para a paridade

## Resultado direto

A paridade geral continua aberta. P1393 encontrou um defeito de linguagem
atual reproduzível, uma família pública ampla ainda sem ownership e um defeito
do harness que dispara uma tentativa de alocação de 41,6 GB no vanila.

## Fila priorizada

### 1. Precedência lexical de callee local em math — `DIFFERENCE`

Witnesses atuais:

```typst
#let abs(x) = [LOCAL-ABS]
$abs(1)$
```

```typst
#let sqrt(x) = [LOCAL-SQRT]
$sqrt(1)$
```

O vanila renderiza `LOCAL-ABS` e `LOCAL-SQRT`. O cristalino ignora as closures
locais e renderiza o operador nativo `|1|` e a raiz de 1. Os quatro processos
terminaram com exit 0, pico máximo de 39.464 KiB e zero swap.

O owner existe:
`00_nucleo/prompts/compiler/eval/math.md` →
`01_core/src/compiler/eval/math.rs`. O L0 ainda cita o pin anterior na linha 10
e deve ser atualizado antes do código. A fonte produtiva já promete prioridade
local em `math.rs:290-314` e `:382-403`, mas o caminho de chamada não cumpre a
mesma regra. A hipótese é correção interna de paridade ADR-0127; qualquer
alteração de contrato público, default ou fase refuta o recorte.

### 2. Família pública `format.*` — `ABSENT/PARTIAL`, ownership pendente

O resync introduziu 127 paths de target drift, majoritariamente `format`,
`format.html.*`, `format.pdf.*`, `format.png`, `format.svg` e aliases. P1357
não encontrou owner L0 único e corretamente não abriu código. Alguns efeitos
de CLI foram implementados em P1359–P1367, mas isso não materializa os bindings
da linguagem.

Antes de implementação é necessário individualizar owners 1:1 e decidir se o
novo comportamento altera default público. Esse gate é arquitetural, não uma
falha que possa ser escondida por alias no CLI.

### 3. 145 rotas retidas — `UNKNOWN_CURRENT_HEAD`, historicamente não-MATCH

O anexo P1357 enumera 145 paths e 492 células. As famílias principais são:

- operadores, estilos e símbolos math (`math.*`, `bb`, `cal`, `frak`, `scr`,
  `sscript`, `upright`);
- membros de introspecção (`counter_*`, `state_*`, `outline.entry.*`,
  `selector.before/after`);
- elementos model/layout (`list.item`, `enum.item`, `figure.caption`,
  `footnote.entry`, cells/header/footer);
- HTML element members;
- repr e variantes de Symbol.

Elas precisam de rerun no HEAD e classificação por semântica. Presença ausente
ou kind diferente não prova, sozinha, que toda a operação está ausente.

### 4. Diagnósticos, loaders e representação pública — `PARTIAL/DIFFERENCE`

P1335 preservou gaps não fechados posteriormente: origem em closures/loaders,
namespace XML, fallback CBOR de Symbol/Content, mensagens de arquivo/argumento,
thin space de `math.liminf/limsup` e repr de `sym.quote`/`math.quote`. Esses
itens têm causas e owners distintos; não formar um lote monolítico.

### 5. Layout fora dos certificados — `UNKNOWN/PARTIAL`

P1373–P1392 fecharam altura relativa/fracionária, columns e várias células de
prefixo inline. Permanecem explicitamente fora do crédito: terceiro fragmento,
wrapper aninhado, delta adicional, multiword, wrap interno ou múltiplo,
reserva insuficiente, bidi, footnote completa e flow complexo não exercitado.

## Lacunas do instrumento

### PNG sem PPI explícito

Nos casos `P1137-X-003` e `P1138-X-002`, o comando vanila sem `--ppi` aborta ao
tentar alocar `41.568.028.160` bytes, embora o help declare default 144. Sob o
mesmo limite, `--ppi 144` termina com pico de 43.212 KiB e gera 1191×1684.
O cristalino sem essa flag também gera 1191×1684; ele rejeita `--ppi`, que é
uma lacuna pública separada.

O próximo ajuste do harness deve acrescentar `--ppi 144` apenas ao lado vanila
para tornar explícito o default declarado. Não se aumenta memória, não se roda
o caso sem limite e não se pede ao cristalino que reproduza o crash.

### PDF font name

`P1138-X-003` coincide em página, MediaBox e texto, mas `pdffonts` mostra
`LibertinusSerif-Regular-Identity-H` no vanila e
`LibertinusSerif-Regular` no cristalino. Isso permanece `DIFFERENCE`
instrumentalmente detectada e sem classificação de defeito de língua até medir
fonte/encoding efetivos.

## Próximo passo recomendado

**P1394 — corrigir e selar o observador PNG com PPI explícito e limite de
memória.** É o menor passo necessário para remover os dois `UNKNOWN` da matriz
sem expor novamente a máquina à tentativa de 41,6 GB. Ele altera somente
manifesto/harness e testes, não produto.

Depois de P1394, o primeiro passo de produto deve tratar a precedência local
`abs`/`sqrt` em math, atualizando primeiro o L0 `compiler/eval/math.md` e
resselando sua linhagem.

## Limite do veredito

As contagens atuais provam somente os casos executados. P1393 não declara
equivalência geral, não transforma extensão cristalina em defeito e não usa
ausência de evidência como evidência de ausência.
