# P577 — Confirmar `cargo test --workspace` e medir posições depois do alinhamento RTL

## Proveniência desta medição

Seguindo a regra escrita depois de P575 (`00_nucleo/regra-proveniencia-medicao.md`):

- **Estado do código:** working tree **não commitado** sobre o commit
  `76f73904c2d9c8433e95376d24c672d19861049c`
  ("P576: actualiza Prompts L0 para alinhamento dir: rtl e sincroniza hashes").
- **Ficheiros alterados nesse working tree** (`git diff HEAD --stat`):
  ```
  00_nucleo/diagnosticos/paridade-producao-p576.md | 121 +++++++++++------------
  01_core/src/entities/layout_types.rs             |   3 +
  01_core/src/entities/style_chain.rs              |   1 +
  01_core/src/entities/value.rs                    |   5 +
  01_core/src/engine/eval/mod.rs                    |   7 ++
  01_core/src/engine/eval/repr.rs                   |   1 +
  01_core/src/engine/eval/rules.rs                  |   7 ++
  01_core/src/engine/layout/cursor.rs               |  33 +++++++
  01_core/src/engine/layout/mod.rs                  |   2 +
  01_core/src/engine/layout/text.rs                 |   5 +
  10 files changed, 122 insertions(+), 63 deletions(-)
  ```
  Isto é a implementação de P576 (a parte L0 + sync de hashes já está no
  commit acima; a implementação L1 em si nunca foi commitada — "kimi code"
  parou antes desse commit e antes de confirmar `cargo test --workspace`).
- **Binário usado:** `target/release/typst`, compilado às 14:03 de hoje —
  posterior à última modificação de qualquer ficheiro `.rs` listado acima
  (14:02:41), logo reflecte exactamente este working tree.
- **Hora da medição:** 2026-07-05, 14:27–14:33 (sessão contínua, sem
  alterações ao código entre os passos abaixo).

---

## Parte 1 — `cargo test --workspace`

Corrido até ao fim (não "em background", ao contrário do que o relatório de
P576 registava na secção 6).

```
typst_core:   3568 passed; 0 failed; 0 ignored
typst_infra:   591 passed; 0 failed; 5 ignored
typst_shell:    24 passed; 0 failed; 0 ignored
typst (bin):     2 passed; 0 failed; 0 ignored
tests/cli.rs:   21 passed; 0 failed; 0 ignored
crystalline_lint.rs: 2 passed; 0 failed; 0 ignored
doc-tests:       0 passed; 0 failed; 3 ignored
------------------------------------------------
TOTAL:        4208 passed; 0 failed; 8 ignored
```

Reconfirmado também `crystalline-lint .` neste exacto working tree:
`✓ No violations found`.

### Critério de fecho da Parte 1

- [x] Resultado completo obtido, não pendente: **4208 passed, 0 failed, 8
      ignored**.
- [x] Sem falhas — não há necessidade de comparar com o estado anterior a
      P576.

---

## Parte 2 — Medir posições depois do alinhamento

### Documento de referência (o mesmo usado em P563/P564/P566/P567/P569/P574)

```typst
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
```

(Note-se a diferença face a P567/P569/P574: aqueles passos testavam este
documento **sem** `dir:` explícito, porque a propriedade ainda não existia.
Este passo adiciona `dir: rtl`, que é precisamente o que P576 implementou.)

```bash
./target/release/typst /tmp/p577-rtl.typ /tmp/p577.pdf
pdftotext -tsv /tmp/p577.pdf -
lab/typst-original/target/release/typst compile /tmp/p577-rtl.typ /tmp/p577-vanilla.pdf
pdftotext -tsv /tmp/p577-vanilla.pdf -
```

### Vanilla (hoje, com `dir: rtl` explícito)

| Texto (visual) | left (pt) | top (pt) | width (pt) |
|----------------|----------:|---------:|-----------:|
| ةلواطلا        |     73.21 |    65.19 |     168.00 |
| ىلع            |    251.21 |    65.19 |      72.00 |
| 42             |    333.21 |    61.43 |      37.20 |
| باتكلا         |    380.41 |    65.19 |     144.00 |

Idêntico, número a número, ao que P567/P574 já tinham medido para o
vanilla (sem `dir:` — o vanilla já alinhava à direita automaticamente por
detecção de script). **Confirma que o vanilla é estável e não foi tocado.**

### Cristalino — referência histórica (P567, antes de `dir:rtl` existir)

| Texto (visual) | left (pt) | top (pt) | width (pt) |
|----------------|----------:|---------:|-----------:|
| ةلواطلا        |     70.87 |    49.90 |     168.00 |
| ىلع            |    248.72 |    49.90 |      72.00 |
| 42             |    330.56 |    49.90 |      40.00 |
| باتكلا         |    380.41 |    49.90 |     144.00 |

Uma linha só, ordem crescente e correcta (`table < on < 42 < book`), só
faltava o deslocamento para a margem direita — exactamente o que P576 se
propôs a fazer.

### Cristalino — hoje, com a implementação de P576 (`dir: rtl` explícito)

| Texto (visual) | left (pt) | top (pt) | width (pt) | linha (`pdftotext`) |
|----------------|----------:|---------:|-----------:|:--------------------|
| ىلع            |    238.41 |    49.90 |      72.00 | 0 |
| 42             |    320.41 |    49.90 |      40.00 | 0 |
| ةلواطلا        |    346.41 |    66.03 |     168.00 | 0 |
| باتكلا         |    370.41 |    49.90 |     144.00 | **1** |

**Isto não é o resultado esperado.** Três problemas, todos verificáveis nos
números acima:

1. **Ordem quebrada**: em P567 a ordem esquerda→direita era
   `table(70.87) < on(248.72) < 42(330.56) < book(380.41)` — crescente e
   limpa. Hoje é `on(238.41) < 42(320.41) < table(346.41) < book(370.41)`
   — `table` e `on` trocaram de posição relativa. Um deslocamento uniforme
   da linha (o que `align_current_line_rtl()` devia fazer) preserva a
   ordem relativa; isto não preserva.
2. **`top` inconsistente**: `table` fica a 66.03pt, os outros três a
   49.90pt — uma diferença de 16.13pt, mais do que ruído de arredondamento.
3. **`pdftotext` regista duas linhas**, não uma, para o que devia ser um
   único parágrafo de uma linha.

### Confirmação visual (`mutool draw`, 150 dpi)

- Vanilla: uma linha limpa, `الكتاب 42 على الطاولة` correctamente
  espaçado, sem sobreposição.
- Cristalino: **sobreposição visual real de glifos** entre `الطاولة`
  (mesa) e `الكتاب` (livro), com `42` espremido entre eles e um glifo
  isolado (`|`) destacado à direita. Não é um artefacto de extracção de
  texto — é um defeito de composição visível a olho nu.

### Isolamento da causa: só ocorre com direcção mista na mesma linha

Testado o documento mais simples que P576 usou na sua própria validação
(sem o `42`):

```typst
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب على الطاولة
```

| Texto (visual) | left (pt) | top (pt) | width (pt) |
|----------------|----------:|---------:|-----------:|
| ةلواطلا        |    110.41 |    49.90 |     168.00 |
| ىلع            |    288.41 |    49.90 |      72.00 |
| باتكلا         |    370.41 |    49.90 |     144.00 |

Uma linha só, ordem crescente, sem sobreposição — **correcto**. Testado
também com uma palavra latina em vez do dígito
(`الكتاب ABC على الطاولة`): mesmo padrão de quebra que com `42`
(`ىلع@196.16, ABC@278.16, ةلواطلا@346.41/top 66.03, [linha 2] باتكلا@370.41`).

**Conclusão do isolamento**: o defeito não depende do dígito em
particular — depende de haver **uma run de direcção diferente (LTR/neutra)
embutida no meio do texto RTL**. P576 só validou o próprio passo com
frases 100% árabes (ver `paridade-producao-p576.md`, secção 2.2, nenhum
dos quatro testes tem conteúdo misto); o documento de referência usado
desde P563/P567/P569/P574 — que tem sempre o `42` embutido — nunca foi
testado por P576 antes de declarar "CONCLUÍDO".

### Inferência sobre o mecanismo (marcada como inferência, não confirmada por instrumentação)

`align_current_line_rtl()` (`01_core/src/engine/layout/cursor.rs:156-176`)
aplica um deslocamento **uniforme** a todos os itens de `current_line` no
momento em que é chamada — isso por si só preserva ordem relativa. A
existência de **duas** marcações `###LINE###` no `pdftotext` para o que
devia ser uma única linha sugere que `flush_line()` (que chama
`align_current_line_rtl()` internamente, `cursor.rs:214`) foi invocado
**duas vezes** para este parágrafo — uma vez para `[on, 42, table]`, outra
para `[book]` — cada uma alinhada independentemente à margem direita, daí
a sobreposição. Candidato mais provável: o teste de quebra de linha em
`layout_chunk`/`layout_word` (`right_margin = width - margin`, um cálculo
pensado para LTR) reage de forma diferente quando a run muda de direcção
a meio do texto, disparando um `flush_line()` prematuro que o caso
100%-árabe nunca exercita.
**O que refutaria isto**: instrumentar `flush_line()`/`align_current_line_rtl()`
com contagem de chamadas para este documento — não feito neste passo
(fora do âmbito de "Verificação directa"; ficaria para o passo de
correcção).

### Critério de fecho da Parte 2

- [x] Tabela de posições construída e comparada com o vanilla e com P567,
      com números.
- [ ] **Confirmado que o deslocamento para a direita não desfez a
      correcção de posições relativas já feita antes** — **NÃO
      confirmado. O oposto foi encontrado**: o deslocamento desfaz a
      ordem e sobrepõe glifos quando a linha contém uma run de direcção
      mista.

---

## Critério de fecho do passo

- [x] Parte 1: `cargo test --workspace` confirmado, não pendente — 4208
      passed, 0 failed, 8 ignored.
- [x] Parte 2: posições medidas com a mesma tabela já usada antes na
      sequência — **e a comparação falha**.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p577.md`,
      com o hash do commit da medição (ver secção de proveniência).
- [ ] **P576 NÃO passa de "a confirmar" para "fechado".** O critério
      literal do próprio P576 ("posições relativas entre palavras... sem
      regressão") falha para o documento de referência padrão da
      sequência RTL, que é precisamente o caso que expõe o problema. A
      Parte 1 (testes) está genuinamente fechada; a Parte 2 não.

---

## Decisão (ADR-0108 — decisão escrita, não adiamento vago)

**P576 fica com estado revisto para "implementação parcial — regressão
confirmada em texto RTL com direcção mista".** Não é aceitável fechar como
"CONCLUÍDO": o próprio documento-padrão da sequência RTL (usado desde
P563) expõe sobreposição visual de glifos, não apenas uma diferença de
poucos pontos.

Não corrigido neste passo, por ser "Verificação directa" (âmbito
explicitamente de confirmação, não de implementação) e por o mecanismo
exacto (Parte 2, "Inferência sobre o mecanismo") ainda não estar
confirmado por instrumentação — corrigir às cegas aqui arriscaria mais
uma hipótese errada, o que a P576 já cita como risco conhecido nesta área
(P566).

**Recomendação**: abrir um passo dedicado, nomeado explicitamente (não
"corrigir no futuro" vago): instrumentar `flush_line()` para o documento
de referência com run mista, confirmar quantas vezes é chamado e com que
conteúdo em `current_line` de cada vez, e só depois decidir a correcção
em `align_current_line_rtl()`/`layout_chunk`. Prioridade alta: o defeito
é visualmente grave (sobreposição de glifos) e afecta o caso mais comum
de texto RTL real (números, nomes próprios em latim, código inline
misturados com árabe).

## Actualização do relatório de P576

`00_nucleo/diagnosticos/paridade-producao-p576.md` tinha `Status:
CONCLUÍDO`. Foi actualizado para reflectir este achado (ver esse
ficheiro) — não fica "CONCLUÍDO" sem a confirmação que este passo devia
dar e que, na prática, refutou.
