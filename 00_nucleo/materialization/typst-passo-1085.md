# L0 — Passo 1085: Auditoria da Densidade de Co-mudança de `introspect.rs`

**Gate**: nenhum — auditoria de dados já produzidos pelo `cochange_metrics.py`,
sem código alterado.

**Base**: `introspect.rs` mostrou 61 clusters em 66 pares possíveis (~92%
densidade), um outlier de ~10x face a qualquer outro candidato do P1084 (o
segundo mais denso, `cursor.rs`, tem 11%). Precisa de decompor a origem desse
número antes de aceitar como "acoplamento genuíno".

---

## 1. Pedir a lista bruta de pares, não só o agregado

O relatório do P1084 deu o total (61), não a lista. Pedir o output bruto do
`cochange_metrics.py` para `introspect.rs` — os 61 pares reais (função A,
função B, nº de commits em comum), não só a contagem.

## 2. Classificar os 61 pares em duas categorias

- **Categoria "hub"**: pares onde `walk` e/ou `materialize_time` é um dos dois
  elementos.
- **Categoria "auxiliares"**: pares onde nenhuma das duas é `walk`/
  `materialize_time` — só entre as ~10 funções restantes (headings, labels,
  bib/cite, etc.).

Máximo possível na categoria "hub": `2 × 10 = 20` (cada uma das duas funções-
eixo pareada com cada uma das outras 10). Máximo possível na categoria
"auxiliares": `C(10,2) = 45`.

## 3. Interpretar conforme o resultado

- Se a maioria dos 61 vier da categoria "hub" (perto do tecto de 20, com o
  resto vindo de repetição/peso, não de pares novos) — a densidade alta é
  sobretudo efeito de `walk`/`materialize_time` aparecerem em quase todo
  commit do arquivo (esperado de um dispatcher central), **não** prova de
  acoplamento real entre as funções auxiliares. Isso favorece a proposta já
  feita (manter as duas no hub, extrair o resto) — mas por um motivo mais
  fraco do que "91% de densidade" sugeria.
- Se uma fracção substancial vier da categoria "auxiliares" (auxiliares
  co-mudando **entre si**, não só com o hub) — é sinal de acoplamento real
  entre elas, o que pode significar que não devem ser extraídas para nós
  totalmente separados (podem precisar de ficar juntas nalgum agrupamento
  intermédio, não uma por uma).

## 4. Verificar se o script conta commits ou pares únicos

Confirmar a definição exacta de "cluster" no `cochange_metrics.py` — se um
cluster é incrementado a cada commit onde as duas funções mudam juntas
(podendo o mesmo par contar várias vezes), ou se é contado uma vez por par
distinto independente de quantos commits o confirmam. Isto muda completamente
a interpretação do número "61".

## Critério de conclusão

- Lista bruta dos 61 pares obtida (não só o total).
- Classificação hub vs auxiliares, com contagem em cada categoria.
- Definição de "cluster" no script confirmada (commit-count vs par-único).
- Recomendação sobre `introspect.rs` (extrair função a função vs agrupar
  auxiliares) ajustada ao que os dados reais mostrarem, não à intuição inicial
  do P1084.
