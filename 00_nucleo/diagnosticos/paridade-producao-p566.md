# Paridade de Produção — P566

## Resumo

P566 foi executado como verificação directa de P565, repetindo as medições de
posições por palavra já usadas em P563/P564 e tentando medir o custo do reflow
com o mesmo rigor. A conclusão é que **P565 não está fechado**: o documento de
referência `الكتاب 42 على الطاولة` (40 pt) continua a ter `الطاولة` numa linha
separada, e a ordem visual das palavras no cristalino diverge do vanilla em
pontos, não em dezenas.

A causa raiz não está no reflow de P565 em si, mas no shaper de P484:
`bidi_runs()` em `03_infra/src/shaper.rs` usa `unicode_bidi::visual_runs()` e
agrupa os espaços nos runs adjacentes. O resultado são runs visuais
incorrectos — por exemplo, o texto lógico `الكتاب 42 على الطاولة` é emitido
pelo shaper como três runs visuais (` على الطاولة`, `42`, `الكتاب `) em vez de
quatro unidades semânticas separadas (`الكتاب`, `42`, `على`, `الطاولة`). Como o
Layouter coloca os runs da esquerda para a direita na ordem em que os recebe, a
ordem visual final já nasce errada, e a passagem posterior de `layout_bidi.rs`
não pode reconstruir a ordem lógica perdida.

## Parte 1 — Medições do documento de referência

Documento:

```typst
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
```

Comandos usados:

```bash
./target/release/typst /tmp/p566-referencia.typ /tmp/p566.pdf
pdftotext -tsv /tmp/p566.pdf /tmp/p566.tsv
lab/typst-original/target/release/typst compile /tmp/p566-referencia.typ /tmp/p566-vanilla.pdf
pdftotext -tsv /tmp/p566-vanilla.pdf /tmp/p566-vanilla.tsv
```

### Tabela de posições — Cristalino (pós-P565)

| Texto (visual) | left (pt) | top (pt) | width (pt) | linha |
|----------------|----------:|---------:|-----------:|------:|
| على            |     80.87 |    49.90 |      72.00 |     0 |
| 42             |    162.87 |    49.90 |      40.00 |     1 |
| باتكلا         |    212.87 |    49.90 |     144.00 |     2 |
| ةلواطلا        |     70.87 |    66.03 |     168.00 |     3 |

### Tabela de posições — Vanilla

| Texto (visual) | left (pt) | top (pt) | width (pt) | linha |
|----------------|----------:|---------:|-----------:|------:|
| ةلواطلا        |     73.21 |    65.19 |     168.00 |     0 |
| على            |    251.21 |    65.19 |      72.00 |     0 |
| 42             |    333.21 |    61.43 |      37.20 |     0 |
| باتكلا         |    380.41 |    65.19 |     144.00 |     0 |

### Análise

- **Número de linhas distintas:** cristalino = 2, vanilla = 1.
- **`الطاولة` na mesma linha das outras palavras?** Não. No cristalino está na
  linha 3 (`top = 66.03`); no vanilla está na linha 0 com as restantes.
- **Ordem visual (da esquerda para a direita):**
  - Vanilla: `الطاولة`, `على`, `42`, `الكتاب` (correcto para RTL).
  - Cristalino: `على`, `42`, `الكتاب`, `الطاولة` (incorrecto; o run `على` não
    deveria aparecer antes de `42` e `الكتاب`).
- **Diferença de posição de `الكتاب`:** vanilla `left = 380.41`, cristalino
  `left = 212.87` — diferença de ~167 pt, ou seja, **dezenas de pontos**, não
  poucos pontos.

Critério de fecho da Parte 1: **não satisfeito**.

## Parte 2 — Custo do reflow

O benchmark oficial `tools/perf/benchmark-p507.py` foi iniciado em P565 e não
terminou no timeout de 300 s. Como a correção de P565 não foi confirmada, o
benchmark antes/depis não é relevante para validar o fecho. Foi feita uma
medição isolada do custo da passagem bidi/reflow actual com um documento
multi-linha RTL:

```typst
#set text(lang: "ar", size: 20pt)
الكتاب المفتوح على الطاولة يحتوي على معلومات قيمة...
```

```bash
time ./target/release/typst /tmp/p566-multi-rtl.typ /tmp/p566-multi-rtl.pdf
```

Resultado: `real 0m3,975s`.

Este documento também revelou que o shaper agrupa palavras e pontuação de
forma incorrecta (ex.: `.ةميقتامولعم` em vez de `معلومات قيمة.`), confirmando
que o problema é anterior ao reflow.

## Decisão

**P565 não fica fechado.** O reflow implementado em `03_infra/src/layout_bidi.rs`
cumpre a especificação unitária que lhe foi dada (fundir blocos de linhas RTL
consecutivas quando cabem), mas essa especificação parte de uma premissa falsa:
que o shaper de P484 emite os runs na ordem visual correcta e com separação de
palavras preservada. As medições mostram que não emite.

### Causa raiz identificada

`bidi_runs()` em `03_infra/src/shaper.rs` chama
`unicode_bidi::visual_runs()` sobre o texto completo e devolve os runs na
ordem visual. Os espaços, sendo neutros, aderem aos runs adjacentes, e a ordem
visual resultante não corresponde à ordem esperada pelo Layouter. Como o
Layouter é LTR e posiciona os runs da esquerda para a direita na ordem em que
os recebe, o posicionamento final já nasce errado. A passagem posterior em
`layout_bidi.rs` só pode reordenar items dentro de uma linha ou fundir linhas;
não pode reconstruir a ordem lógica perdida pelo shaper.

### Caminho recomendado

Reabrir a sequência RTL num passo que corrija o shaper antes de reavaliar o
reflow. Duas opções arquitecturais a considerar:

1. **Fazer o shaper emitir runs na ordem lógica** (não visual), com a
   informação de direcção (`rtl`/`ltr`) preservada. O Layouter coloca os runs
   da esquerda para a direita na ordem lógica, e `layout_bidi.rs` passa a ser a
   única responsável pela reordenação visual e reflow. Esta opção é
   arquitecturalmente mais limpa e consistente com a localização da lógica bidi
   em L3.

2. **Manter o shaper visual, mas separar explicitamente os espaços e as
   palavras** de forma a que os runs visuais correspondam exactamente às
   unidades semânticas. Esta opção mantém a decisão actual de P484, mas exige
   reimplementar parte da reordenação visual no shaper para lidar com
   pontuação/espaços.

Em ambos os casos, o L0 `00_nucleo/prompts/infra/shaper.md` terá de ser
actualizado e os testes de P484/P564/P565 revalidados.

## Critérios de fecho do passo

- [x] Parte 1: posições remedidas com números.
- [ ] Parte 2: benchmark completo ou casos isolados medidos três vezes,
  antes/depois. *(Não aplicável: P565 não foi confirmado; foi feita medição
  isolada de custo.)*
- [x] Decisão registada: P565 não fechado, causa raiz no shaper de P484.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p566.md`.

---

*Relatório gerado em 2026-07-05.*
