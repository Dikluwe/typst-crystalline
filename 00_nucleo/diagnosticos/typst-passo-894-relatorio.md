# Relatório — typst-passo-894: triagem de `typst-math-comprehensive-test.typ` (30 secções)

**Data:** 2026-07-24T16:16:39Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `ccbe6816c5c26b84a780cc9aa15e2950cd5a6587` (HEAD do ramo `Tekt`; P891/P892 já
reconciliados nesse commit ou sem código alterado; P893 é STOP puro — L0s editados, hashes
sincronizados, Fase B **não iniciada**, nada em conflito com este passo).
**Working tree no início:** `M .gitignore` (não relacionado), relatórios/materializações untracked
de P892/893/894 (não relacionados), dois PDFs soltos (`output_vanilla.pdf`, `test_vanilla.pdf`, não
relacionados, não tocados). Nada pendente que afecte este passo.

**Ficheiro-fonte**: `.typ/typst-math-comprehensive-test.typ` (30 secções, já presente no repositório
de trabalho). Hash: `sha256:ed8154255cc844474425c32be7cfcfef9057189377bcaef394769d189bf3ed7c`. Todos
os binários deste relatório foram recompilados no início do passo a partir do commit acima; todas as
comparações vanilla-vs-cristalino usam o **mesmo ficheiro `.typ`** compilado **no mesmo momento** nos
dois binários (metodologia obrigatória do prompt, para não repetir o erro já registado 3 vezes neste
projecto — P885 achados 1/4, P891 Parte B).

Existe um segundo ficheiro `.typ/typst-math-comprehensive-test_vanilla.typ` no repositório (variante
que evita sintaxe suspeita de ser incompatível, ex. `math.op(...)` em vez de string, `epsilon.alt` em
vez de `ϵ`) — **não usado neste passo**, por instrução explícita do prompt (um único ficheiro para os
dois lados). Ele é, coincidentemente, uma pista de que alguém já suspeitava de parte dos achados
abaixo antes deste passo.

---

## Prioridade 0 — os "2 crashes": ambos isolados, causa real ≠ hipótese original

### Crash 1 (secção 22) — hipótese original refutada; causa real: `floor`/`ceil` em falta

A hipótese do prompt (delimitador escapado `lr(\]a/b\[)`) foi testada isolada — **não falha em
nenhum dos binários**. A secção completa 22 falha no cristalino em **`floor`/`ceil`** (linha
`lr(floor a/b floor)`, `unknown variable: floor`), duas linhas **antes** da linha com `\]a/b\[`.
Sem `floor`/`ceil`, o resto da secção (incluindo a linha do prompt) compila igual nos dois. Vanilla
compila a secção inteira sem erro.

Causa raiz (achado maior, ver Prioridade 3): `floor`/`ceil` não existem como funções nativas de
matemática no cristalino (só como `calc.floor`/`calc.ceil`, numéricas — `01_core/src/engine/
stdlib/calc.rs:61-62`). No vanilla são funções `#[func]` de `math/lr.rs` que envolvem o conteúdo em
`⌊⌋`/`⌈⌉` (mesma família de `abs`/`norm`/`round`).

**Nota de género**: exit 1 com mensagem de erro limpa e hints — **não é um panic/crash real**, é um
erro de compilação normal. O prompt usa "crash" para "não produz PDF", não para pânico do processo.

### Crash 2 (secção 26) — confirmado real, mas com causa dupla (uma delas é paridade, não bug)

Isolada a secção inteira (definições `bra`/`ket`/`hbar`/`expval` por concatenação de string + uso).
Cristalino falha: `error: cannot add string and content`. Vanilla compila sem erro.

**Investigação revelou dois mecanismos distintos, só um deles é bug real:**

1. **Identificadores de 1 letra nunca chamam função vinculada em modo math** (`f(phi)` com `#let
   f(x)=...` de 1 letra) — testado exaustivamente (`f`, `g`, `h`, `ab`, `abc`, `xyz`, `foo`, `bra`,
   `notbra`): **nomes de 1 letra nunca invocam a função** (renderizam como justaposição literal
   `𝑓(𝜑)`, itálico + parêntese, sem chamar o `#let`); **nomes de ≥2 letras invocam correctamente**
   (`ab(x)=x+x` chamado com `phi` produz `φφ`, confirmando a chamada real). **Confirmado que isto é
   paridade com o vanilla, não bug do cristalino** — o mesmo teste (`f(x)=x+x`, `$f(phi)$`) produz
   **exactamente o mesmo resultado** (`𝑓(𝜑)`, função NÃO chamada) nos dois binários. Este achado
   **refuta a hipótese original de Prioridade 1** ("identificador+parêntese" como causa da perda de
   conteúdo) — ver secção própria abaixo.
2. **`"string" + content` dentro de uma função de ≥2 letras invocada em modo math é o bug real**:
   `#let bra(x) = "⟨" + x + "|"` seguido de `$ bra(phi) $` — `bra` (3 letras) É chamado, `x` é
   vinculado a `Content` (a partir de `phi`), e a soma `"⟨" + Content` falha no cristalino
   (`cannot add string and content`) mas **funciona no vanilla** (produz `⟨𝜑|`). **Confirmado bug
   real e específico do cristalino** — falta suporte para `Str + Content → Content` (ou coerção
   equivalente) no operador `+` quando usado dentro de modo math.

**Não implementada correcção para nenhum dos dois** (Prioridade 0 é só diagnóstico, per o prompt).

---

## Prioridade 1 — "perda sistemática de conteúdo": hipótese original refutada; causa real confirmada e muito mais grave

### As 5 hipóteses isoladas do prompt — todas refutadas (sem perda de conteúdo)

Testados isoladamente (sem `#set page(width: auto, ...)`, com página normal): `n(n+1)`,
`f(x) = f'(x)`, `f(x) dif x`, `a^2+b^2=c^2`, `(sin x)/x`, `1/x`. **Nenhum mostra perda de conteúdo**
em nenhum dos binários — extracção de texto idêntica em substância (só diferenças triviais de
espaço na extracção, ex. `𝑓′(𝑥)` vs `𝑓 ′ (𝑥)`, sem significado). A hipótese unificadora
"identificador+parêntese" está **refutada** (ver Prioridade 0, item 1 acima — o comportamento é
paridade com vanilla, não uma ambiguidade errada do cristalino).

### A causa real, confirmada por leitura de código e visualmente: `#set page(width: auto)` + equação de bloco produz `offset_x = +infinito`

Ao reproduzir a secção 1 **exactamente como está no ficheiro** (incluindo `#set page(width: auto,
height: auto, margin: 1cm)`, linha 6 do ficheiro-fonte, activa para o documento inteiro), a extracção
de texto do cristalino fica **vazia** — não porque o conteúdo desapareceu, mas porque o PDF fica
**malformado**:

```
strings sec1.pdf | grep MediaBox
<< ... /MediaBox [0 0 inf 249.81] ... >>
```

**Confirmado por render visual** (150dpi): todas as equações da secção ficam esmagadas e sobrepostas
numa faixa minúscula no fundo de uma página de tamanho substituto (612×792, "US Letter" — o
`mutool`/`pdftotext` substituem silenciosamente este tamanho quando encontram `inf` na `MediaBox`,
mascarando a verdadeira natureza do bug). O mesmo reproduz-se identicamente para a secção 4.

**Causa exacta, isolada por bisecção binária e confirmada por leitura de código**:

- `page_config.width` fica `f64::INFINITY` quando `width: auto` (`01_core/src/engine/layout/
  set_page.rs:34`, comportamento intencional — o valor final é calculado depois via
  `compute_page_width()`, que **funciona correctamente isoladamente**, confirmado com `height: auto`
  sozinho).
- **O gatilho é especificamente `width: auto` + pelo menos uma equação de bloco.** Confirmado:
  `height: auto` sozinho funciona perfeitamente (`mediabox="0 0 595.28 191.04"`); `width: auto`
  sozinho ou os dois juntos sempre produzem `inf`; `width: auto` com **só equação inline** (sem
  bloco) também funciona (`mediabox="0 0 238.27 64.20"`).
- **Ponto exacto do bug**: `01_core/src/engine/layout/equation.rs:116-117`, a centragem horizontal
  de equações de bloco (P813):
  ```rust
  let usable = self.regions.current.width - 2.0 * self.page_config.margin;
  offset_x = Pt(self.page_config.margin + (usable - ext.width) / 2.0);
  ```
  `self.regions.current.width` é `f64::INFINITY` quando `width: auto` (sincronizado de
  `page_config.width` em `set_page.rs:79`). `usable = INFINITY`, logo `offset_x = INFINITY` para
  **toda** equação de bloco. Esta posição propaga para todos os itens da equação
  (`abs_pos.x = offset_x + pos.x`), que por sua vez tornam `compute_page_width()`
  (`line_content_right()`) infinito — dali para a `MediaBox` como o literal inválido `"inf"`.

**Impacto**: qualquer documento com `#set page(width: auto, ...)` **e** pelo menos uma equação de
bloco fica corrompido desta forma — não é um problema pontual das secções 1/4, é uma única causa que
provavelmente afecta o ficheiro **inteiro** (30 secções), já que a directiva está no topo e vale para
todo o documento. **Muito mais grave que a hipótese original** (que teria implicado corrigir uma
heurística de desambiguação; a causa real implica NaN/infinito a propagar-se por todo o layout de
página quando `width: auto` coexiste com o mecanismo de centragem de blocos).

**Não implementada correcção** (Prioridade 1 é só diagnóstico, per o prompt).

---

## Prioridade 2 — mapeamento de caractere

### `dot` — confirmado e corrigido (TDD, suíte verde)

Confirmado via `codex` (a base de dados de símbolos do vanilla, crate `codex-0.3.0`, `sym.txt`):
`dot` bare = variante `.op` = `⋅` (U+22C5, DOT OPERATOR); `dot.c` = `·` (U+00B7, MIDDLE DOT) — uma
variante **distinta**. `01_core/src/engine/stdlib/sym.rs:32` tinha `("dot", '·')` — o valor da
variante `.c`, não o valor bare. **Corrigido** (TDD: 2 testes escritos e confirmados a falhar antes
da correcção): `("dot", '⋅')` + nova entrada `("dot.c", '·')` para preservar o valor antigo como
variante nomeada. Suíte `typst-core` completa: 4703 passed (0 failed, +2 dos testes novos). `crystalline-lint
.`: 0 drift novo (correcção de dado dentro de mecanismo já documentado em `sym.md` — não enumera
símbolos individuais, não precisou de secção nova). Confirmado end-to-end: `$ a dot b $` →
`⋅` no PDF.

**Achado incidental, não relacionado, não corrigido**: `#sym.dot.c` (e `#sym.eq.not`, testado como
controlo) falham com `unknown symbol modifier` — limitação pré-existente do mecanismo de acesso a
variantes nomeadas via field-access fora do `sym_lookup` interno, não introduzida por esta correcção
(reproduzida com uma entrada já existente antes deste passo). Fora de âmbito.

### `partial` — confirmado divergente, mas **não é uma correcção simples de tabela** (não corrigido)

Confirmado (`mutool trace`): `$ partial / (partial x) f(x,y) $` produz `∂` (U+2202, upright) no
cristalino e `𝜕` (U+1D715, itálico) no vanilla. **Não é um erro de tabela** — a entrada
`"partial" => Some("∂")` (`01_core/src/engine/math/symbols.rs:72`) está correcta segundo o codex
(`partial ∂`, sem variantes — o **valor base** já é upright U+2202). A itálicização observada no
vanilla acontece **depois** da resolução do símbolo, num passo de "aplicar itálico matemático por
omissão" que o cristalino implementa (`is_single_letter_var`, `01_core/src/engine/math/
symbols.rs:203-206`) só para identificadores de **origem** com 1 letra ASCII — `"partial"` tem 7
letras na fonte, por isso nunca passa por esse teste, independentemente do símbolo resolvido ter só 1
glifo. Corrigir isto exigiria mudar o mecanismo para decidir itálico pelo **glifo resultante**, não
pelo nome digitado — mudança de arquitectura, não uma entrada de tabela. **Registado como achado
separado para um passo dedicado futuro** (per a condição explícita do prompt: só fechar Prioridade 2
neste passo se for mesmo uma correcção de tabela simples — não é o caso aqui).

---

## Prioridade 3 — catálogo das secções 5–30

### Achado dominante: classe inteira de funções matemáticas nativas em falta

`01_core/src/engine/eval/math.rs`, despacho de chamadas em modo math (`match name.as_str()`)
implementa **só 6 nomes**: `frac`, `sqrt`, `root`, `vec`, `cases`, `mat`. Confirmado por leitura
directa (grep exaustivo no match arm) e por teste isolado + `mutool trace` (não só extracção de
texto) para cada um: **`hat`, `tilde`, `bar`, `dot` (forma de acento), `abs`, `norm`, `floor`,
`ceil`, `round`, `binom` (math), `underbrace`, `overbrace`, `underbracket`, `overbracket` renderizam
como texto literal** (`hat(𝑥)` em vez de `𝑥̂`, `abs(𝑥)` em vez de `|𝑥|`, etc.) — a função nunca é
reconhecida/chamada, o nome + parênteses ficam como conteúdo matemático normal (identificador
seguido de grupo). Confirmado que `binom` só existe como `calc.binom` (numérico), não como notação
matemática — mesma classe do achado de `floor`/`ceil` na Prioridade 0.

Contagem de ocorrências no ficheiro completo: `hat` ×7, `dot` ×3, `underbrace`/`ceil`/`floor`/
`overbrace` ×2 cada, `bar`/`binom`/`norm`/`abs`/`round`/`underbracket`/`overbracket` ×1 cada — **26
ocorrências no total**, atravessando pelo menos as secções **5, 6, 7, 10, 12, 18, 26** directamente
(secção 26 já coberta em Prioridade 0). Este é o achado de maior impacto de todo o passo — maior
alcance que o bug de `width: auto` (Prioridade 1), que afecta o documento inteiro por uma única
directiva mas só corrompe a geometria da página; este afecta o **conteúdo semântico** renderizado em
pelo menos 7 secções distintas.

### Achado novo: `sqrt`/`root` com radicando multi-glifo — barra do radical atravessa o conteúdo em vez de ficar por cima

Secção 14, confirmado visualmente (render 150dpi + comparação directa com vanilla, mesmo momento):
`sqrt(a^2 + b^2)` no cristalino desenha a barra do radical **a meio da altura** do conteúdo
(atravessando os glifos, como um traço/strikethrough), não por cima como no vanilla. Não confirmado
por texto (a extracção não capta posição vertical) — **confirmado só visualmente**, conforme exigido
pela metodologia. Causa não investigada (fora de âmbito de triagem — candidato a Fase A de um passo
dedicado, provavelmente no cálculo de `radical_vertical_gap`/posicionamento da regra do radical em
`math/layout/root.rs`, mas isto é hipótese, não confirmado por leitura de código neste passo).

### Achado descartado após dupla-verificação: itálico de `theta` (falso alarme)

Suspeita inicial (da imagem renderizada a 150dpi): `cos theta` mostraria `θ` upright no cristalino em
vez de `𝜃` itálico. **Refutado** ao verificar directamente os codepoints via `mutool trace` no PDF
real (não a imagem): o cristalino produz `unicode="𝜃"` (itálico) correctamente, 3 ocorrências na
secção 14 — a impressão de "upright" na imagem era erro de leitura visual a essa resolução (o itálico
grego é sutil), não uma divergência real. Registado aqui precisamente para documentar que a
verificação dupla (trace, não só olhar a imagem) evitou um falso positivo — disciplina do próprio
prompt.

### Observação não confirmada, baixa prioridade: possível ausência de espaço fino antes do argumento de funções trig/log nomeadas

Impressão visual (não medida rigorosamente): `sin x`, `cos theta` parecem renderizar sem o pequeno
espaço entre o nome da função e o argumento que o vanilla usa (`sin x` vs `sin𝑥`/`sinx` aparente).
**Não confirmado com rigor** (medição de deltas x um a um, controlando runs de texto comparáveis,
não foi feita) — registado como observação de baixa confiança para eventual confirmação futura, não
como achado fechado.

### Secção 8 (alinhamento de equações multi-linha `&= ... \\`) — comportamento idêntico nos dois binários, não é divergência

Verificado visualmente: a secção 8 (equações com `&=`/`\\`/`&&`) renderiza de forma aparentemente
estranha (tudo numa linha, tamanho de fonte a encolher progressivamente, `\` literal a aparecer como
texto) — **mas de forma idêntica no vanilla**. Não é uma divergência cristalino-vs-vanilla; é
provavelmente uma limitação de sintaxe do próprio `.typ` de teste (equações alinhadas multi-linha
podem exigir um mecanismo específico não usado aqui) ou um comportamento genuíno do Typst fora do
âmbito desta comparação. **Não investigado mais** — sem divergência, não há o que corrigir aqui.

### Restante catálogo (secções não detalhadas acima)

Secções 9, 11, 13, 15–17, 19–21, 23–25, 27–30: compiladas com sucesso nos dois binários (página
fixa, não `auto`), texto extraído e comparado por palavra. As diferenças residuais encontradas são
predominantemente reordenação/espaçamento da extracção de texto (ruído já conhecido de
`pdftotext -layout` em conteúdo matemático denso, não uma fonte fiável de veredicto per a
metodologia) ou instâncias adicionais dos dois achados já catalogados acima (funções em falta,
constructos usando `partial`/`dot`). Nenhuma tentativa adicional de achado novo, isolado e confirmado
visualmente foi feita para estas secções neste passo, por disciplina de tempo — ficam registadas como
"sem achado novo confirmado" nesta rodada, não como "confirmadas correctas".

---

## Resumo e resultado

| Item | Veredicto | Estado |
|---|---|---|
| Crash 1 (`lr(\]a/b\[)`) | Hipótese refutada; causa real = `floor`/`ceil` em falta | Diagnosticado, não corrigido |
| Crash 2 (`bra`/`ket`) | Confirmado — 2 mecanismos: 1 é paridade (não-bug), 1 é bug real (`Str + Content`) | Diagnosticado, não corrigido |
| 5 casos isolados (Prioridade 1) | Todos refutados individualmente | Hipótese unificadora descartada |
| `width: auto` + eq. de bloco | **Achado maior, confirmado por código + visual** | Diagnosticado, não corrigido |
| `dot` símbolo | Confirmado, tabela simples | **Corrigido (TDD, suíte verde)** |
| `partial` itálico | Confirmado, **não** é tabela simples | Diagnosticado, não corrigido (achado à parte) |
| Funções math em falta (`hat`/`abs`/etc.) | **Achado maior, 26 ocorrências, 7+ secções** | Diagnosticado, não corrigido |
| `sqrt` barra mal posicionada | Confirmado visualmente | Diagnosticado, não corrigido |
| Itálico de `theta` | Falso alarme, descartado após verificação | Fechado, sem acção |
| Secção 8 (alinhamento) | Paridade com vanilla, sem divergência | Fechado, sem acção |
| Secções restantes (9,11,13,15-21,23-25,27-30) | Sem achado novo confirmado nesta rodada | Catalogado para revisão futura |

**Suíte**: `cargo test -p typst-core --lib` → 4703 passed, 0 failed (inclui os 2 testes novos de
`dot`). `crystalline-lint .`: 0 drift novo. Nenhuma correcção de Prioridade 0/1/3 implementada, per
instrução explícita do prompt — só Prioridade 2 (`dot`) fechou nesta passagem.

**Candidatos priorizados para passos futuros** (ordem sugerida por impacto observado, não uma
decisão do dono):
1. `#set page(width: auto)` + equação de bloco → `offset_x = infinito` (Prioridade 1) — corrompe
   potencialmente o documento inteiro sempre que a combinação ocorre.
2. Funções matemáticas nativas em falta (`hat`/`tilde`/`bar`/`dot`-acento/`abs`/`norm`/`floor`/
   `ceil`/`round`/`binom`/`underbrace`/`overbrace`/`underbracket`/`overbracket`) — maior alcance de
   conteúdo afectado (26 ocorrências, 7+ secções).
3. `Str + Content` dentro de função de math (causa real do crash 2).
4. `sqrt`/`root` barra do radical mal posicionada com radicando multi-glifo.
5. `partial` (e possivelmente outros símbolos "multi-letra→1-glifo") sem itálico por omissão —
   mudança de arquitectura em `is_single_letter_var`.
