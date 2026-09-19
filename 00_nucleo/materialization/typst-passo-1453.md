# Passo 1453 — refinar a projeção pública de `html.frame` e fechar o denominador P1422

**Estado:** EXECUÇÃO AUTORIZADA — refinamento diagnóstico sem mudança de produto
**Data da medição:** 2026-09-18T20:55:21-03:00
**Commit medido:** `4fd9f556a9c125c45a4a1f5cfd7205e1548559c4`
**Árvore medida:** `2114aca05777ea298cf083e28d9ef04c3b678dfb`, limpa
**Regime:** `executed_without_isolation_attestation`

## 1. Causa focal

P1452 repetiu as 66 rotas P1420 sob identidades congeladas. Sessenta e cinco
formas qualificadas ficaram `MATCH`; somente `html.frame` ficou `DIFFERENCE`,
com vetor idêntico em direct, direct-repeat, reverse e repetição integral.

O witness não é uma diferença de linguagem. Ambos os produtos emitiram uma
ilha SVG inline de `5.39pt × 7.238pt`, `viewBox="0 0 5.39 7.238"` e corpo
visual `x`. O vanilla converteu o glyph em `use/defs/path`; o cristalino emitiu
`<text>`. A projeção genérica P1420 comparou toda a árvore SVG e, portanto,
incluiu detalhes privados que o L0 P1424 exclui expressamente.

## 2. Medição antes da decisão

As fontes normativas vigentes dizem:

- `prompts/compiler/layout/html_frame.md`: texto possui dimensões intrínsecas
  positivas; nesting é geometricamente idêntico; IDs/bytes SVG não pertencem à
  unidade;
- `prompts/infra/export/svg.md`: raiz SVG transparente, dimensões físicas,
  `viewBox`, `overflow: visible`, `display: block`, geometria, paint e texto são
  observáveis; IDs, paths privados, ordem de atributos e bytes não são.

No vanilla ratificado, `(repr(html.frame), repr(html.frame([x])))` produz
`["frame","frame(body: [x])"]`; body ausente produz
`missing argument: body`. P1424 mediu ainda rect vermelho 10pt×5pt, nesting
geometricamente idêntico e `html.elem("frame")` como tag literal, não ilha.

Identidades congeláveis permanecem as de P1452, sem rebuild ou alteração de
produto:

- vanilla `586e1bd43`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release SHA-256
  `4c55affaa6486683165d91af507b03f99e0041f18ce5a4fc062cbc6e2b2d05de`;
- resultados P1452 SHA-256
  `769cd0feba8293a7bc030c1b511e48b9338aef30b80bafd1c0b42ffd1040b6f4`.

## 3. Projeção decidida antes da nova execução

A projeção P1453 combina sondas independentes e fail-closed:

1. **identidade/carrier:** comparar exatamente binding e repr do carrier
   `frame(body: [x])`, além do diagnóstico de body ausente;
2. **ilha simples:** exigir exatamente um SVG inline e nenhuma tag `frame`;
   largura/altura `pt` positivas; `viewBox` com origem zero e dimensões iguais
   às físicas dentro de 0,001pt; style contendo `overflow: visible` e
   `display: block`; ausência de fundo branco; payload visual não vazio;
3. **nesting:** a projeção geométrica da ilha aninhada é idêntica à simples em
   cada produto e bilateralmente;
4. **rect:** `10pt × 5pt`, `viewBox="0 0 10 5"`, paint vermelho e transparência;
5. **controle literal:** `html.elem("frame")` conserva uma tag `frame`, não
   contém SVG e nunca recebe crédito pela rota qualificada.

Para payload visual, `<text>`, `<use>`, `<path>`, `<rect>` e outros primitives
de pintura são representações privadas alternativas. A projeção registra
somente presença de pintura; a identidade textual vem da sonda separada do
carrier. Saída ilegível, raiz ambígua, dimensões ausentes, identidade divergente,
timeout, OOM, swap ou truncamento resultam em `Unknown`, nunca `Preserved`.

## 4. Regime segregado e ataques

Aplicar o protocolo completo. Contrato/oráculos e mutantes são congelados antes
do runner; o implementador não os edita; o verificador não corrige o candidato.
Como a sessão e o filesystem são compartilhados, declarar “executado sem
atestação de isolamento”.

O gate discriminatório deve rejeitar pelo menos: raiz ausente/duplicada, tag
`frame` indevida, width/height/viewBox alterados, origem não zero, display ou
overflow removidos, fundo branco, payload visual vazio, paint vermelho perdido,
nesting divergente, carrier/repr adulterado, diagnóstico promovido, identidade
antiga, transporte assimétrico e promoção de `Unknown`. Exigir score `1.0`,
positivos `Preserved`, opacos `Unknown` e vetor igual em três ordens.

Budget: no máximo duas revisões focais do projetor e uma campanha completa mais
uma repetição independente. Duas revisões sem ganho na mesma causa interrompem
o passo como insuficiente.

## 5. Composição e critério de fecho

Executar apenas o fragmento afetado: cinco sondas bilaterais, três ordens e uma
repetição independente. Não repetir as outras 65 rotas, pois seu resultado
P1452 está protegido, usa os mesmos binários e nenhuma camada de produto muda.

Se `html.frame` for `Preserved`, compor esse recibo com as 65 rotas `MATCH` de
P1452 e declarar o denominador 66/66 fechado somente sob as projeções
registradas. Se houver `Violated` ou `Unknown`, manter `INCONCLUSIVE`.

Executar ainda `python3 -m py_compile`, validação de hashes, gate adversarial,
`cargo fmt --all -- --check`, `crystalline-lint .` e `git diff --check`.

## 6. Gate ADR-0127

O passo não altera contrato público, comportamento padrão, fase de pipeline,
compatibilidade, L0 ou produto. Ele corrige apenas a lente diagnóstica para
obedecer a ADR-0107 e ao L0 P1424 vigente; segue em fluxo contínuo sem parada.

## 7. Limite da alegação

O fecho eventual cobre as 66 rotas P1420 mediante a composição P1452+P1453 e
somente os observáveis explicitados. Não prova equivalência geral de HTML, SVG,
math ou Typst.
