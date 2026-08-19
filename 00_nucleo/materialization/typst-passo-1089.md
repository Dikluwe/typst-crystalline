# L0 — Passo 1089: Investigação — Desvio Horizontal Acumulado por Glifo em Equações

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é só
investigação.

**Base**: achado registado em nota externa (2026-08-12, rodada seguinte ao
P1086-1088). Espaçamento vertical entre blocos já confirmado corrigido
(Δ máx 0.08pt, ruído). Achado novo: `dx` (vanilla−cristalino) cresce em
degraus ao longo de uma linha — `0.62 → 0.88 → 1.22 → 1.84pt` — não é um
glifo isolado deslocado, é acumulação progressiva. Largura de página
(260.46pt cristalino vs 262.649pt vanilla, Δ=2.19pt) bate com o desvio
acumulado na linha mais longa — mesmo mecanismo, não dois problemas.

**Instrução explícita da nota**: não corrigir a largura de página
directamente — encontrar a causa por-carácter; a página corrige-se sozinha.

---

## 1. Não presumir constante aditiva — os degraus não são uniformes

Diferenças entre degraus sucessivos: `0.88−0.62=0.26`, `1.22−0.88=0.34`,
`1.84−1.22=0.62`. Não é um Δ fixo por glifo (senão os degraus seriam iguais).
Cresce de forma acelerada. Duas hipóteses a testar, não escolher sem
verificar:

1. **Erro proporcional ao tamanho do glifo** (não aditivo por glifo) — cada
   avanço (`advance width`) está a ser calculado como uma fracção
   ligeiramente menor do valor real (ex.: erro percentual, não erro fixo em
   pt). Isto explicaria crescimento não-linear se os glifos da linha "min"
   tiverem larguras crescentes ou diferentes entre si.
2. **Erro específico a certos tipos de glifo/operador**, não a todos —
   Se só alguns caracteres da linha (ex.: operadores, itálico, símbolos
   especiais como `∈`) tiverem advance sub-calculado, e outros (letras
   simples) estiverem correctos, os degraus apareceriam exactamente nas
   posições onde esses caracteres específicos ocorrem, não uniformemente.

Mapear a posição exacta de cada degrau contra os caracteres reais da linha
`min_(x in RR) f(x)` antes de escolher a hipótese.

## 2. Ler o código real de advance width para glifos de equação

Não presumir localização — buscar:

```bash
grep -rn "fn advance\|x_advance\|advance_width" 01_core/src/compiler/math/ \
  03_infra/src/font_metrics.rs 03_infra/src/shaper.rs
```

Hipótese informada pelo padrão já encontrado em P1086-1088 (duas vezes
seguidas o bug esteve em qual implementação de `FontMetrics` era realmente
usada — `cap_height` vs `glyph_ink_bounds`, unsigned vs signed): confirmar se
o cálculo de advance em contexto matemático usa a métrica real da fonte
(glyph-based, via tabela da fonte) ou cai nalgum caminho aproximado
(`FixedMetrics`, ou uma fórmula genérica tipo `size * constante`) em
determinadas condições (itálico matemático, estilo `MathStyle`, ou glifos
fora do alfabeto latino básico).

## 3. Medição isolando variáveis, mesmo padrão do P1088

Não aceitar a primeira explicação que "bate mais ou menos". Testar:

1. Uma linha só com letras latinas simples (`a b c d`) — advance bate exacto?
2. Uma linha só com operadores matemáticos (`∈ ∇ ∑`) — mesmo teste.
3. Uma linha em itálico matemático (variável simples, `x y z`, que por
   convenção do Typst já é itálico automático em modo math) — mesmo teste.
4. Se (1) bater e (2)/(3) não — a causa é específica a um subconjunto de
   glifos, não um erro genérico de advance.

## 4. Confirmar a hipótese da largura de página, não corrigir directamente

Depois de identificar e corrigir a causa por-carácter, remedir a largura de
página do mesmo documento — deve convergir para 262.649pt sem qualquer
mudança adicional em código de página/margem. Se não convergir sozinha, a
causa por-carácter não era a única, ou não foi a causa certa.

## Critério de conclusão

- Código real de advance width em contexto matemático lido e citado.
- §1 resolvido — hipótese aditiva vs proporcional vs específica a tipo de
  glifo, confirmada com dados, não suposição.
- §3 executado — pelo menos os 3 casos isolados medidos.
- §4 confirmado — página convergindo por si, não corrigida à parte.
