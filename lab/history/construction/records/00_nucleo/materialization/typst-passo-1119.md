# L0 — Passo 1119: Implementar as 3 Correcções Finais da Secção 36

**Gate**: `ADR-0127` — mudança de comportamento por defeito em 3 pontos,
causa já diagnosticada e verificada com prova real (tabela de 4 matrizes
vanilla cruzadas).

---

## 1. Correcção A — sinal de `matrix.c` na matriz `cm`

**Base**: `03_infra/src/export/stream.rs`. Regra verificada contra 4
matrizes reais do vanilla (`rotate+15°`, `rotate-10°`, `skew+15°`,
`scale150%`), sem excepção:

```
(a_pdf, b_pdf, c_pdf, d_pdf) = (matrix.a, -matrix.b, matrix.c, -matrix.d)
```

`matrix.c` passa directo (sem negação); só `b` e `d` são negados. O
código actual nega os três (`-matrix.b, -matrix.c, -matrix.d`).

**Mecanismo**: remover a negação de `matrix.c` especificamente, manter
`-matrix.b`/`-matrix.d`.

## 2. Correcção B — `Td` redundante dentro do `Group`

**Base**: `sub_frame.rs` inicializa `cursor_y = ascender` (`7.645pt`);
`draw_item_local` emite uma instrução `Td` adicional com esse valor
dentro do bloco `BT...ET`, somando-se à translação já aplicada pela
matriz `cm` do `Group` — duplicação.

**Mecanismo**: confirmar exactamente onde a duplicação acontece antes de
remover — ler `draw_item_local` e o caminho de `Group` por inteiro, não
remover a instrução `Td` às cegas (pode ser necessária nalgum caminho que
não passa por `Group`, ex.: texto normal fora de transformação).

## 3. Correcção C — avanço de linha usa altura explícita do `#box`

**Base**: `cursor.rs:375` calcula o avanço da linha a partir de
`text_edges`/`leading` do estilo de texto activo, ignorando a altura
explícita (`height: 60pt`) declarada nos `#box` que compõem a linha.

**Mecanismo**: quando a linha contém itens com altura explícita maior que
a métrica de texto calculada, usar a maior das duas (mesmo princípio de
"avanço de linha = máximo entre os itens da linha", já usado noutros
mecanismos desta investigação). Confirmar contra o vanilla se é
exactamente "máximo simples" ou se há alguma composição mais específica
antes de implementar.

## 4. Verificação — as 4 caixas, tabela completa, eixo X e Y

Reaproveitar a tabela já usada nas últimas rondas (Box 1-4, todos os
glifos) — confirmar `ΔX` e `ΔY` convergindo para `0.0000pt` (±0.0005pt)
em **todos** os glifos, não só o primeiro de cada caixa (mesmo cuidado que
identificou o problema da rodada anterior — erro crescente escondido
atrás de "primeiro glifo bate").

`MediaBox Height`: convergir para `162.6938pt` exacto.

## 5. Não-regressão

- Re-rodar P1086-1118 — zero regressão, `stream.rs` e `cursor.rs` já
  editados extensivamente nesta investigação inteira.
- Testar `rotate`/`scale`/`skew` fora de contexto de `#box` explícito
  (ex.: transformação directa num parágrafo) — confirmar que a Correcção
  C não regride casos onde não há altura explícita a considerar.

## Critérios de verificação

1. As 4 matrizes reais (rotate±, skew, scale) com sinal `c` correcto.
2. Nenhum `Td` duplicado — posição vertical correcta sem dupla
   translação.
3. `MediaBox Height` exacto.
4. Tabela completa (todos os glifos, não só o primeiro) a `0.0000pt`.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- As 3 correcções implementadas com o código real lido antes de editar
  (não só a partir da descrição já dada).
- Tabela final glifo a glifo completa, não amostra parcial.
- Item 7 (ordem de palavras, ainda pendente desde P1116-1117) remedido
  neste passo ou registado explicitamente como ainda pendente — não
  silenciado.
