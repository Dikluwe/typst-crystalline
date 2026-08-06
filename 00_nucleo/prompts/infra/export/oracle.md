# export/oracle — oráculo de paridade de operador (P980)
Hash do Código: 4c630398

**Data:** 2026-08-05 · **Camada:** L3 · **Passo:** 980 (gate confirmado
pelo dono em 2026-08-05)

## Propósito

Caminho de diagnóstico **separado** do exportador de produção: recebe o
content stream já construído pelo modo verbose normal e aplica
transformações de paridade de operador que **não** pertencem à saída
principal (não mudam posições nem têm benefício visível fora de
comparação). Activo só com a flag CLI `--oracle-pdf`.

## Regras do módulo

1. `03_infra/src/export/oracle.rs`; funções puras sobre a string do
   content stream; zero impacto no caminho normal (a saída sem a flag é
   bit-a-bit a mesma — guardado por teste).
2. Activação: `RunIntent.oracle_pdf` (L2) → `compile_to_pdf_bytes_oracle`
   (L3, aditiva) → `export_pdf_oracle` (mesma dispatch de fontes da
   emissão normal) → `PdfBuilder::with_oracle(true)` → aplica
   `collapse_trivial_tj` ao content stream de cada página (antes da
   compressão).

## Transformação 1 — `collapse_trivial_tj`

Um array `[ … ] TJ` cujos ajustes são **todos inteiros zero** (entries
`<XXXX> 0` e ajustes de fronteira `0`) é semanticamente um `Tj` puro:
reescreve-se como `<XXXXYYYY…> Tj` (hex concatenado). Arrays com qualquer
ajuste ≠ 0 ficam intocados. Números são **parseados** como inteiros (um
`-0` conta como zero). Paridade: o vanilla usa `Tj` quando não há
ajustes (medido: 92.5% dos blocos de texto do documento canónico).

## P982 — residual TJ explicado (não colapsar) e o delta q/Q vs cm

**Data:** 2026-08-05

### Parte A — o residual TJ é estrutural, não arredondamento

Medição sobre a saída do oráculo (`typst-passo-982` Fase A.1): os 263
arrays `TJ` restantes têm ajustes **reais**, não ruído — só 2 de 663
ajustes têm |a|≤3 milésimos de em. Os valores dominantes são os
**espaços de classe math**: −278 (THICK, 5/18em), −222 (MEDIUM, 2/9em),
−167 (THIN, 1/6em) — a codificação cristalina escreve o espaçamento de
classe como ajustes TJ dentro de um run; o vanilla escreve-o como
**posições de blocos separados** (cada fragmento math é um TextItem com
posição absoluta própria — daí os seus 1919 blocos contra os nossos
1293). **Conclusão**: colapsar estes arrays seria perder espaçamento
(posições mudariam) — proibido. A convergência adicional só viria de
uma transformação de **split** (partir o run nos ajustes reais e
re-posicionar absolutamente cada pedaço), que precisa dos avanços
nominais da fonte (não disponíveis na string do stream) — fica como
decisão de desenho para um passo futuro, não para esta investigação.

### Parte B — o delta q/Q (68) vs cm explicado

Os 68 pares `q…Q` sem `cm` correspondem exactamente aos 68 traços
vectoriais (`l S` — barras de fracção e afins), medido. O braço
`FrameItem::Line` emite `q {w} w {x1} {y1} m {x2} {y2} l S Q` — o `q/Q`
guarda o estado gráfico contra a mudança de line width; o `cm` não é
necessário porque as coordenadas já são absolutas. O vanilla tem delta 4
(1992 q vs 1988 cm) porque o krilla embrulha quase tudo em `q/cm/Q`
uniformemente. **Veredicto: comportamento correcto e intencional — sem
código.**

## P983 — split posicional: no oráculo, itens math não fundem runs (paridade de granularidade com o vanilla)

**Data:** 2026-08-05 · **Gate:** confirmado pelo dono em 2026-08-05
("continuar" após `typst-passo-983-faseA.md`).

**Medição decisiva** (Fase A.2, caso `$ 3x + y = 9 $`): o vanilla emite
**6 blocos `BT…ET` — um `Tj` por glifo/átomo math**, cada um com posição
absoluta própria (incluindo `3` e `x` em blocos separados apesar de não
haver espaço de classe entre eles). Ou seja: o vanilla nunca funde glifos
math — a granularidade vanilla ≈ os nossos itens L1 (a saída pré-P979).
Em prosa, o contrário: uma linha inteira é um TextItem (P979 mediu
36 = 36). Resumo da regra do vanilla: **math → um bloco por fragmento;
prosa → um bloco por linha de estilo uniforme.**

**Desenho** (muito mais simples que o temido na Fase A.1 — **não são
precisos avanços nominais**): o split não é feito por valor de ajuste nem
por cirurgia de string, mas **ao nível da emissão, por item**: no caminho
do oráculo, itens com `style.math == true` **não participam no
agrupamento de P979** (cada um é o seu bloco, como em pré-P979); itens de
prosa continuam a fundir-se (36 = 36, paridade já medida). As posições
vêm dos próprios itens (`pos.x`) — zero risco de deriva. A transformação
de P980 (`collapse_trivial_tj`) corre depois e converte os blocos
mono-glifo triviais em `Tj`, completando a semelhança com o vanilla.

**Flag**: o `oracle: bool` do `PdfBuilder` (P980) chega ao emissor via
`PageContext.with_oracle` (interno a L3; o builder aplica-o nos 3 pontos
de construção de contexto). `build_page_stream` em modo oráculo só forma
runs de P979 para itens não-math. Caminho normal: intocado (a flag está
sempre `false` fora de `--oracle-pdf`).

**Interacção com P979 (Fase A.4)**: nenhuma — P979 continua correcto na
saída principal (menos operadores, posições iguais); o oráculo diverge de
propósito para espelhar a estrutura do vanilla.
