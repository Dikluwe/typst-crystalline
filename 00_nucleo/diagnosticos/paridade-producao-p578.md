# P578 — Instrumentar `flush_line()` para confirmar a causa da sobreposição RTL

## Proveniência desta medição

Seguindo `00_nucleo/regra-proveniencia-medicao.md`:

- **Estado do código:** working tree **não commitado** sobre o commit
  `76f73904c2d9c8433e95376d24c672d19861049c`
  ("P576: actualiza Prompts L0 para alinhamento dir: rtl e sincroniza hashes").
  Mesmo commit base de P577.
- **Ficheiros alterados** (`git diff HEAD --stat`, medido depois de remover
  a instrumentação temporária deste passo):
  ```
  00_nucleo/diagnosticos/paridade-producao-p576.md | 147 +++++++++++++----------
  01_core/src/entities/layout_types.rs             |   3 +
  01_core/src/entities/style_chain.rs              |   1 +
  01_core/src/entities/value.rs                    |   5 +
  01_core/src/rules/eval/mod.rs                    |   7 ++
  01_core/src/rules/eval/repr.rs                   |   1 +
  01_core/src/rules/eval/rules.rs                  |   7 ++
  01_core/src/rules/layout/cursor.rs               |  33 +++++
  01_core/src/rules/layout/mod.rs                  |   2 +
  01_core/src/rules/layout/text.rs                 |   5 +
  10 files changed, 148 insertions(+), 63 deletions(-)
  ```
  `cursor.rs` volta a ter exactamente as mesmas 33 inserções da
  implementação de P576 — a instrumentação deste passo (dois `eprintln!`
  temporários) foi adicionada e removida dentro desta sessão; o diff final
  confirma que não sobrou nada dela.
- **Binário usado:** `target/release/typst`, recompilado depois de
  remover a instrumentação; `cargo test --workspace` e
  `crystalline-lint .` corridos sobre esse mesmo binário/estado.
- **Hora da medição:** 2026-07-05, 14:40–15:09.

---

## Instrumentação e teste

Adicionados dois `eprintln!` temporários (removidos no fim deste passo,
confirmado acima pelo diff):

1. No início de `flush_line()` — número de items em `current_line` e o
   seu texto.
2. No início de `layout_word()` — `cursor_x`, largura da palavra,
   `right_margin` e se a palavra excede a margem.

### Caso misto (`الكتاب 42 على الطاولة`, o documento de referência da sequência RTL)

```
layout_word("الكتاب"): cursor_x=80.87  w=144.00 right_margin=524.41 overflow=false
layout_word("42"):     cursor_x=234.87 w=40.00  right_margin=524.41 overflow=false
layout_word("على"):    cursor_x=284.87 w=72.00  right_margin=524.41 overflow=false
layout_word("الطاولة"): cursor_x=366.87 w=168.00 right_margin=524.41 overflow=true
flush_line chamado. current_line tem 3 items: ["الكتاب", "42", "على"]
```

`flush_line()` é chamado **uma vez só**, não duas. `"الطاولة"` nunca
aparece em `current_line` no momento do flush — só é adicionada depois,
e por isso só é drenada mais tarde, em `finish()` (fim do documento),
como uma "linha" de um único item.

### Caso puro (`الكتاب على الطاولة`, sem a mistura de direcção)

```
layout_word("الكتاب"): cursor_x=80.87  w=144.00 right_margin=524.41 overflow=false
layout_word("على"):    cursor_x=234.87 w=72.00  right_margin=524.41 overflow=false
layout_word("الطاولة"): cursor_x=316.87 w=168.00 right_margin=524.41 overflow=false
```

`flush_line()` **nunca é chamado** — as 3 palavras cabem dentro de
`right_margin` (316.87 + 168.00 = 484.87 < 524.41) e ficam todas em
`current_line` até `finish()` as drenar de uma vez, como um único grupo.

---

## Refutação da hipótese de P577

A hipótese proposta em P577 era: *"o mecanismo de decidir se uma linha
está cheia... reage de forma diferente quando a direcção do texto muda a
meio, disparando um `flush_line()` antes do previsto"*.

**Refutada.** O registo mostra que:

- `flush_line()` é chamado **exactamente uma vez**, não duas.
- É disparado por um **overflow de largura absolutamente comum**
  (`01_core/src/rules/layout/cursor.rs:98`, dentro de `layout_word()`):
  depois de `"على"`, `cursor_x = 366.87`; a palavra seguinte
  (`"الطاولة"`, 168pt de largura) levaria `cursor_x` a 534.87, acima do
  `right_margin` de 524.41. Isto aconteceria com **qualquer** sequência
  de palavras (RTL, LTR, ou mista) cuja largura acumulada ultrapasse a
  margem — não há nada na condição (`cursor.rs:98`) que sequer
  observe a direcção do texto.
- O caso "puro" não dispara `flush_line()` não porque a direcção seja
  uniforme, mas porque, **por coincidência**, as 3 palavras desse teste
  cabem dentro da margem (484.87 < 524.41) enquanto as 4 do teste misto
  não cabem (534.87 > 524.41). A diferença é a largura acumulada, não a
  mistura de direcção.

---

## Nova descoberta: o bug real está em `finish()`, e é anterior a P576

Se o "flush duplo" não é causado por detecção de direcção, porque é que
a segunda "linha" (`"الطاولة"`, ou `"seven"` no teste abaixo) aparece
sobreposta à primeira, em vez de simplesmente abaixo dela como uma quebra
de linha normal?

`01_core/src/rules/layout/mod.rs:1148-1153` (`finish()`):

```rust
pub fn finish(mut self) -> PagedDocument {
    // P576 — a última linha também pode ser RTL; alinhar antes de drenar.
    self.align_current_line_rtl();
    for item in self.regions.current.current_line.drain(..) {
        self.regions.current.current_items.push(item);
    }
    ...
```

Comparar com `flush_line()` (`cursor.rs`, mesma área de código): depois
de drenar `current_line`, `flush_line()` faz
`self.regions.current.cursor_y += line_height + Pt(line_leading_pt)`.
**`finish()` não faz este avanço.** Quando um documento termina com uma
palavra "sobrante" que nunca passou por `flush_line()` (como
`"الطاولة"` ou `"seven"` abaixo), essa palavra é posicionada usando o
`cursor_y` que **já tinha sido avançado por um `flush_line()` anterior**,
mas o próprio `finish()` não adiciona avanço nenhum a mais — o resultado
observado (delta de `top` de ~16pt em vez de um `line_height` completo de
~56pt) vem de uma diferença de métricas verticais entre chamadas
(`vertical_metrics()` recalculado com contexto diferente), não de dois
avanços de linha completos.

### Confirmação de que isto é independente de RTL

Testado com um documento inteiramente latino, sem `dir:` nenhum,
desenhado só para forçar uma quebra de linha no último (e único)
parágrafo do documento:

```typst
#set text(size: 40pt)
one two three four five six seven
```

```
layout_word("one")...("six"): sempre overflow=false
layout_word("seven"): cursor_x=514.07 w=91.07 right_margin=524.41 overflow=true
flush_line chamado. current_line tem 6 items: ["one","two","three","four","five","six"]
```

TSV (`pdftotext -tsv`):

| Texto | left | top | width |
|-------|-----:|----:|------:|
| one…six (uma linha) | 80.87…457.39 | **49.90** | — |
| seven | 70.87 | **66.03** | 91.08 |

Delta de `top` = 16.13pt — **exactamente o mesmo delta** medido para
`"الطاولة"` em P577 (66.03 − 49.90). Confirmado visualmente com
`mutool draw`: `"seven"` sobrepõe-se visualmente a `"six"`/`"five"` da
linha anterior, em texto 100% LTR, sem `dir:` definido, sem qualquer
código de P576 envolvido na posição horizontal (`"seven"` fica
correctamente no `left=70.87`, a margem esquerda normal — só o
espaçamento vertical está errado).

**Isto é um bug pré-existente, independente de RTL, que afecta qualquer
documento (LTR ou RTL) cujo último parágrafo do documento quebre em duas
ou mais linhas.** P576 não o introduziu; expôs-o, porque o documento de
referência da sequência RTL calha a ter exactamente esta forma (última —
e única — linha do documento a quebrar em duas).

### O que resta específico de P576

Com o bug de `finish()` a explicar a componente vertical, resta uma
pergunta em aberto que este passo não resolve: se o espaçamento vertical
fosse corrigido, o `align_current_line_rtl()` a ser chamado
independentemente em `flush_line()` e em `finish()` (cada grupo alinhado
à margem direita por si só) ainda produziria uma ordem/posição
incorrecta para um parágrafo RTL de duas linhas, ou é esse
comportamento — cada linha a começar junto à margem direita —
exactamente o que se espera de um parágrafo RTL de várias linhas? Isto
só pode ser respondido depois de corrigir `finish()` e medir de novo.

---

## Critério de fecho

- [x] Número de chamadas a `flush_line()` confirmado para os dois
      documentos: 1 (misto), 0 (puro).
- [x] Conteúdo de `current_line` em cada chamada registado.
- [x] Hipótese de P577 **refutada**, com o registo como prova.
- [x] Localizado o ponto exacto onde a decisão de quebra de linha actua:
      `01_core/src/rules/layout/cursor.rs:98` (`layout_word`) — decisão
      correcta e comum a todo o texto, não um bug em si.
- [x] Localizado o ponto exacto do bug real:
      `01_core/src/rules/layout/mod.rs:1148-1153` (`finish()` não avança
      `cursor_y` antes de drenar a última linha).
- [x] Instrumentação removida — confirmado por `git diff HEAD --stat`
      (mesmas 33 inserções de P576 em `cursor.rs`, nada a mais).
- [x] `cargo test --workspace` reconfirmado depois de remover a
      instrumentação: **4208 passed, 0 failed, 8 ignored**.
- [x] `crystalline-lint .` reconfirmado: `✓ No violations found`.

---

## Decisão (ADR-0108 — decisão escrita, não adiamento vago)

**Não corrigir neste passo** (âmbito era sonda directa). A causa exacta
está confirmada com prova (registo + teste de isolamento LTR), não é
suposição.

**Recomendação — dois passos dedicados, nesta ordem:**

1. **Prioridade alta, geral (não é sobre RTL):** corrigir `finish()`
   (`01_core/src/rules/layout/mod.rs:1148-1153`) para avançar `cursor_y`
   /`line_height` antes de drenar a última linha, ou para reconhecer que
   não precisa de o fazer quando não há `flush_line()` anterior na
   mesma página — decidir a forma certa exige olhar para todos os
   caminhos que chamam `finish()` e `flush_line()`, não só o caso RTL.
   Este bug é mais fundamental do que a sequência RTL: afecta qualquer
   documento cujo último parágrafo quebre em 2+ linhas.
2. **Só depois do anterior estar corrigido:** re-medir o documento de
   referência RTL (`الكتاب 42 على الطاولة`) e decidir se
   `align_current_line_rtl()` chamado independentemente em
   `flush_line()` e `finish()` ainda precisa de ajuste, ou se passa a
   funcionar correctamente uma vez a componente vertical corrigida.

Não juntar as duas correcções num só passo — a primeira é um bug geral
de layout (fora do scope RTL), a segunda é especificamente sobre
`dir: rtl`; misturá-las dificultaria isolar qual das duas resolveu o quê,
o mesmo erro que este passo evitou ao não aceitar a hipótese de P577 sem
prova directa.
