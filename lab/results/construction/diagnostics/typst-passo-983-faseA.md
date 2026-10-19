# Passo 983 — Fase A: split posicional no oráculo (PARADO no gate, ADR-0127)

**Data:** 2026-08-05 · **Estado da árvore:** HEAD = `08b23376f` (P982).
Nenhum código de produção alterado. L0 editado e resselado
(`oracle.md` §P983, `oracle.rs` → `c88e621c` — só hash), linter limpo.

## Respostas da Fase A

**A.1 — que dados faltam: nenhum.** O desenho final não precisa dos
avanços nominais da fonte (o receio de P982): o split acontece ao nível
da emissão, por item, com as posições `pos.x` dos próprios itens — não
por cirurgia de string nem por valores de ajuste.

**A.2 — a regra do vanilla, medida** (não só lida): `$ 3x + y = 9 $`
produz **6 blocos `BT…ET` no vanilla — um `Tj` por glifo/átomo math**,
cada um com posição absoluta (mesmo `3` e `x`, sem espaço de classe entre
eles, são dois blocos). Ou seja, o vanilla **nunca funde glifos math**; a
sua granularidade ≈ os nossos itens L1 (a nossa saída pré-P979). Em
prosa, o oposto: uma linha uniforme = um TextItem (36 = 36, medido em
P979). Regra do vanilla: **math → um bloco por fragmento; prosa → um
bloco por linha uniforme.**

**A.3 — o algoritmo**: no caminho do oráculo, itens `style.math == true`
não participam no agrupamento de P979 (cada item é o seu bloco); itens de
prosa continuam a fundir. `collapse_trivial_tj` (P980) corre depois e
converte os blocos triviais mono-glifo em `Tj` — completando a semelhança
(vanilla: `Tj` por glifo). A flag chega ao emissor via
`PageContext.with_oracle` (interno a L3; o `PdfBuilder` aplica-a nos 3
pontos de construção de contexto — o campo `oracle` já existe no builder
desde P980).

**A.4 — interacção com P979**: nenhuma na saída principal — P979 fica
intacto para quem não usa a flag; o oráculo diverge de propósito para
espelhar a estrutura de blocos do vanilla.

## Estimativa do resultado (a confirmar na Fase C)

O oráculo passará a emitir ~1 bloco por item math (≈ a contagem pré-P979
para math, ~2050) contra 1919 do vanilla — ~7% a mais, porque os nossos
itens são ligeiramente mais finos que os fragmentos do vanilla em alguns
pontos; e `Tj`% sobe para perto dos 92.5% (blocos mono-glifo triviais
colapsam). Posições: inalteradas por construção (o split usa `pos.x` dos
itens; a prova compare.py vem na Fase C).

## Gate

Parado per ADR-0127 (a transformação muda a estrutura de blocos da saída
do oráculo — superfície do oráculo já confirmada em P980, mas a regra de
split é decisão nova). À confirmação: Fase B com o protocolo de dois
agentes (testes de split/não-split e de posição primeiro), Fase C com as
contagens e a prova de posições.
