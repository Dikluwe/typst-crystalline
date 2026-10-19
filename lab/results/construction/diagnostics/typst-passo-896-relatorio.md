# Relatório — typst-passo-896: centragem/numeração sob `width: auto` (Fase A + B + C)

**Data:** 2026-07-24T17:17:15Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `ccbe6816c5c26b84a780cc9aa15e2950cd5a6587` (HEAD do ramo `Tekt`).
**Pré-condição de árvore**: `git status` confirma que P893 (STOP, Fase B não iniciada), P894 (`dot`) e
P895 (offset_x infinito + catálogo de terceiros) continuam **por commitar** — mesmo estado desde o
início de P895. Working tree sem conflito para este passo.

**Fase A confirmada com o dono via `AskUserQuestion`: escopo mínimo escolhido — "Só equação (Opção c
mínima)". `Content::Align` fica registado, não corrigido, per decisão explícita.**

---

## 1. Como o vanilla resolve isto arquitecturalmente (lido, não presumido)

Fonte: `lab/typst-original/crates/typst-layout/src/pages/run.rs` + `flow/distribute.rs`.

### 1.1 — a largura/altura da página também fica infinita no vanilla

`pages/run.rs:114-115`:
```rust
let width = styles.resolve(PageElem::width).unwrap_or(Abs::inf());
let height = styles.resolve(PageElem::height).unwrap_or(Abs::inf());
```
Comentário no próprio código: *"When one of the lengths is infinite the page fits its content along
that axis."* — confirma que o vanilla usa exactamente o mesmo valor sentinela (`infinito`) que o
cristalino, não um mecanismo diferente de representar "auto" internamente.

### 1.2 — a diferença real: alinhamento é **diferido**, nunca calculado contra um valor ainda infinito

`flow/distribute.rs` tem uma estrutura de trabalho (`struct Work`, campo `items`, comentário **"Já
laid out items, not yet aligned"**, linha 40) que acumula cada filho do flow como
`Item::Frame(frame, align)` — a **frame já medida**, mas a **posição ainda não resolvida**. Isto
acontece para **todo** o conteúdo do flow (parágrafos, blocos, equações, o que for), não é um
mecanismo especial para equações.

A resolução da posição só acontece em `fn finalize(...)` (linha 487, comentário: *"This performs
alignment and resolves fractional spacing and blocks"*), chamada **uma vez**, depois de todo o
conteúdo da região já estar recolhido — nesse ponto, `size` (a dimensão final da região, já resolvida
a partir do conteúdo se a região era `auto`) é conhecida, e só aí:
```rust
let x = single.align.x.position(size.x - frame.width());
```//`resolve_alignment` equivalente do cristalino, chamado com `size.x` **já finito**.

**Conclusão da Fase A ponto 1**: o vanilla não faz "duas passagens completas de layout" (Opção a, tal
como este prompt a descreveu) — faz **uma passagem de layout + uma passagem de posicionamento**,
dentro do mesmo mecanismo de flow, para *todo* o conteúdo alinhado, não só para equações. A
"segunda passagem" é barata (não relayouta nada, só desloca frames já prontas).

## 2. Achado que alarga o âmbito declarado do passo: `Content::Align` sofre o mesmo bug

O prompt pede para não presumir que só equações são afectadas. Confirmado por leitura directa
(`01_core/src/engine/layout/mod.rs`):

- `available_width()` (`:644-650`) devolve **`f64::INFINITY`** explicitamente quando
  `page_config.width` é infinito — usado por `layout_align` (`placement.rs:26`, braço
  `Content::Align`, ex. `#align(center)[...]` em código normal, **fora de modo math**).
- `resolve_alignment(...)` (`:704-727`) — a mesma função usada pela centragem de equação em
  `equation.rs` — calcula `HAlign::Center => origin_x + (available_w - content_w) / 2.0`. Com
  `available_w = INFINITY`, o resultado é **sempre infinito**, para **qualquer** conteúdo alinhado ao
  centro/direita sob `width: auto` — não só equações.

**Isto significa que o âmbito real do bug é mais largo do que o título do passo** ("centragem e
numeração de equação"): `#align(center)[texto]`, `#align(right)[texto]`, e qualquer outro consumidor
de `resolve_alignment` sob `width: auto` produzem a mesma classe de posição incorrecta (P895 já
evitou o `infinito` literal só nos dois pontos de `equation.rs`; `placement.rs`/`resolve_alignment`
**não foram tocados por P895** e continuam a produzir `infinito` hoje para `Content::Align`).

Não testado neste passo se isto já corrompe a `MediaBox` da mesma forma que a equação corrompia antes
de P895 (`compute_page_width` deveria capturar o mesmo item com posição infinita) — mas a mecânica é
idêntica, por isso o mais provável é que sim. Registado, não confirmado por execução (fora do
âmbito estrito deste passo, que é sobre equação — mas relevante para a decisão de desenho abaixo).

## 3. Avaliação dos 3 desenhos candidatos (à luz do mecanismo real do vanilla)

| Desenho | O que é | Custo/Risco | Cobertura |
|---|---|---|---|
| **(a) Duas passagens completas** | Replicar arquitectura do vanilla à letra: struct de trabalho tipo `Work`/`Composer`, todo o flow (não só página) reestruturado para acumular `(frame, align)` e resolver no fim | **Alto** — mexe na sequência geral do Layouter (praticamente todo o `layout/mod.rs` e os módulos que emitem `FrameItem` directamente), risco de regressão generalizado | Total (cobre `Content::Align` também, de graça) |
| **(b) Pré-scan localizado** | Antes de posicionar qualquer bloco numa página `auto`, uma passagem leve só para medir a largura máxima de conteúdo dessa página | Médio — precisa de "olhar para a frente" no fluxo de conteúdo, o que o Layouter actual (streaming, um nó de cada vez) não faz; exigiria reestruturar a iteração de `Content::Sequence` para bloco-de-página, não just-in-time | Parcial — só cobre o que o pré-scan souber medir (equações; teria de decidir se inclui `Content::Align` também) |
| **(c) Correcção pós-layout (deferida)** | Manter posicionamento actual (0 ou margem), registar metadados (índice/gama de items + largura própria), corrigir em `finish()`/`new_page()` quando a largura final é conhecida | **Baixo** se aplicado só a equações (2 pontos já identificados em P895); **médio** se alargado também a `Content::Align` (mais um consumidor, mesmo mecanismo) | Equivale ao mecanismo real do vanilla, só que aplicado a menos tipos de conteúdo que ele (que cobre *todo* o flow) |

**O mecanismo real do vanilla é, na essência, a Opção (c)** — não a (a) como o próprio prompt
especulava antes de eu ler o código-fonte. A diferença entre (c) "como o vanilla faz" e (c) "como
este passo escopa" é *quantos tipos de conteúdo* participam no diferimento: vanilla defere **tudo**;
uma versão mínima deferiria **só equações** (título do passo) ou **equações + `Content::Align`**
(cobertura mais fiel ao vanilla, dado o achado da secção 2).

## 4. Caso de teste mínimo — resultado exacto do vanilla (para a Fase B)

`.typ` (3 equações de bloco, larguras bem diferentes, `width: auto`):
```typst
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)
$ a = b $
$ a + b + c + d + e = f $
$ x $
```
Vanilla (`mutool trace`): `mediabox="0 0 158.37 105.841"` (margin=28.346 cada lado → usable=101.678).

| Linha | x medido | x esperado centrado contra usable=101.678 (largura própria) | Confere |
|---|---|---|---|
| `a=b` (largura≈25.2) | 66.504 | 28.346+(101.678-25.2)/2 = 66.58 | ✓ (±0.1, kerning) |
| `a+b+c+d+e=f` (a mais larga, largura≈101.678) | 28.346 (= margin) | 28.346+(101.678-101.678)/2 = 28.346 | ✓ exacto |
| `x` (largura≈6.29) | 76.039 | 28.346+(101.678-6.29)/2 = 76.04 | ✓ |

Confirma a leitura da secção 1: todas as linhas centram contra a largura da linha **mais larga**
(que por sua vez define a largura final da página), não contra a própria margem. Alvo exacto para o
teste de Fase B (tolerância sugerida: 0.1pt, mesma ordem de grandeza dos exemplos acima).

## 5. Nota separada — "posição de linhas/símbolos" reportada pelo dono

Ainda não verificado (Fase B fará isso depois da correcção de centragem, per a instrução do próprio
prompt). Hipótese razoável dado o achado da secção 2: se o sintoma reportado envolver conteúdo
**não-equação** (texto normal centrado, por exemplo), pode já ser o achado de `Content::Align` acima,
não um terceiro problema. Não decidido aqui — fica para confirmação visual pós-Fase B.

---

## STOP — decisão de desenho, pedida ao dono do projecto

Per o próprio prompt: qualquer desenho que mexa na sequência geral de layout de página exige
confirmação antes de avançar para a Fase B. Recomendação registada:

**Opção (c), escopo mínimo — só equação (centragem P813 + numeração)**, por ser o menor risco,
suficiente para fechar exactamente o que o título do passo pede, e arquitecturalmente alinhado com o
mecanismo real do vanilla (diferir, não recalcular tudo) — só que aplicado a menos tipos de conteúdo.
O achado de `Content::Align` (secção 2) fica **registado, não corrigido**, candidato a um passo
dedicado futuro que estenda o mesmo mecanismo de diferimento a `resolve_alignment` em geral.

Alternativa a considerar: alargar já este passo para cobrir `Content::Align` também (mesmo mecanismo,
mais um consumidor) — custo incremental menor do que implementar dois passos separados com o mesmo
tipo de correcção, mas mais escopo do que o título do passo pede.

**Decisão do dono (via `AskUserQuestion`): "Só equação (Opção c mínima)".** Prossegue-se com a Fase B
no escopo confirmado.

---

## Fase B — Implementação (TDD)

### Desenho concreto

Mecanismo de correcção adiada, escopado só a equação de bloco (P813) e à sua numeração (P456):

- **`Layouter::pending_equation_centering: Vec<(usize, usize, f64, f64)>`** —
  `(índice inicial em current_items, nº de items, largura própria da equação, offset x já aplicado)`.
  Populado por `equation.rs::layout_equation` quando a centragem P813 encontra
  `regions.current.width` infinito. O índice inicial é calculado antes do `flush_line()` (que move
  os items de `current_line` para `current_items`): `current_items.len()` + o que já estava em
  `current_line` antes desta equação começar a empurrar os seus próprios items — preserva ordem
  correctamente mesmo que `current_line` não estivesse vazia.
- **`Layouter::pending_equation_numbering: Vec<(f64, EcoString, TextStyle, f64)>`** —
  `(y da baseline, texto formatado, estilo, largura do texto)`. Substitui o scope-out de P895 (que
  simplesmente omitia o número).
- **`Layouter::apply_pending_equation_fixups(&mut self, items: &mut Vec<FrameItem>, page_width: f64)`**
  — chamado por `finish()` e `new_page()` (`cursor.rs`), imediatamente depois de `page_width`/
  `page_height` serem resolvidos e de `items` ser extraído de `current_items`, **antes** da
  numeração de página (P532) e do `items` ser movido para a `Page` final. Esvazia os dois `Vec`
  pendentes por completo. Correcção incidental: `finish()` usava `let mut items =
  self.regions.current.current_items;` (move parcial de `self`, impedia chamar um método `&mut
  self` logo a seguir) — alinhado com o padrão que `new_page()` já usava
  (`std::mem::take(&mut self.regions.current.current_items)`), sem mudança de comportamento.
- **`helpers::shift_frame_item_x(item: &mut FrameItem, dx: f64)`** — novo helper, desloca a
  coordenada x de um `FrameItem` **in-place** (todas as variantes, incluindo `Link` recursivamente).
  Distinto de `translate_frame_item` (substitui por posição absoluta, exige mover o item por valor)
  — aqui só é preciso um delta relativo sobre um item já existente no `Vec` da página.

### Teste que falha primeiro (per Fase A ponto 4)

`p896_equacoes_de_bloco_centram_contra_a_largura_final_da_pagina`
(`01_core/src/engine/layout/tests.rs`): duas equações de bloco sob `width: auto` — uma estreita (1
`MathIdent`) e uma larga (5 `MathIdent`s adjacentes, gap automático de classe = 0). Confirma que a
larga fica exactamente na margem (define a largura da página) e que a estreita centra contra a
largura **real** da larga (medida via `FixedMetrics.advance`, não pré-calculada à mão — evita
depender de assumir a fórmula interna exacta). Corrido antes da correcção completa (só com o desenho
implementado parcialmente): falhava com `narrow_x == margin` (mesmo sintoma de P895, sem centragem
real). Depois da implementação completa: passa, com a posição exacta a bater (tolerância 0.01pt).

### Suíte e lint

`cargo test -p typst-core --lib`: 4722 passed, 0 failed (2 testes novos: o de posições exactas acima
+ o de P895 que confirma ausência de infinito, agora ambos cobrindo o mesmo mecanismo em estágios
diferentes). `crystalline-lint .`: 0 drift novo — L0s actualizados: `engine/layout/equation.md`
(mecanismo do ponto de vista de `equation.rs`) e `engine/layout.md` (mecanismo completo — campos,
método, helper — do ponto de vista do L0 partilhado por `mod.rs`/`cursor.rs`/`helpers.rs`).

### Confirmação visual — documento completo de 30 secções

**Nota de proveniência**: o ficheiro `.typ/typst-math-comprehensive-test.typ` mudou de hash desde
P894/895 (`sha256:9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`, era
`ed8154255cc844474425c32be7cfcfef9057189377bcaef394769d189bf3ed7c`) — confirmado por diff: a secção
22 (`lr(floor a/b floor)`/`lr(ceil a/b ceil)`) foi reescrita com os caracteres Unicode literais
(`lr(⌊a/b⌋)`/`lr(⌈a/b⌉)`) e a secção 26 (`bra`/`ket`) mudou de `#let bra(x) = "⟨" + x + "|"` (string,
gatilho do bug de P894) para `#let bra(x) = [⟨#x\|]` (markup/interpolação) — ambas as mudanças
contornam os 2 crashes catalogados em P894 (não relacionados com este passo), o que permitiu
compilar o documento completo nos dois binários sem erro, isolando limpo o sintoma deste passo. A
secção 20 já tem `#set math.equation(numbering: "(1)")`, confirmando o relato original do dono.

Compilados os dois binários no mesmo momento a partir deste ficheiro (exit 0 em ambos):

- **Numeração**: `pdftotext -raw` + grep por linhas `(N)` isoladas — cristalino produz a sequência
  **completa e contígua `(1)` a `(44)`**, batendo exactamente com a extensão que o dono reportou
  ("o vanilla numera todas as equações... `(1)` a `(44)`"). Antes de P896, cristalino não mostrava
  número nenhum depois da secção 20 (achado original deste passo).
- **Centragem**: `mutool trace` sobre as coordenadas x de todos os `fill_text` — distribuição com
  muitos valores **distintos** (`445.6`, `440.1`, `434.6`, `430.3`, `40.9`, `50.9`, `236.4`, `244.8`,
  `241.1`, etc.), não um único valor repetido (que seria o sintoma de P895 — tudo encostado à
  margem). Confirma que equações de larguras diferentes centram em posições diferentes, dependentes
  da largura final da página.
- **Dimensões da página**: `mediabox` finita nos dois binários (cristalino
  `478.23×5624.71`, vanilla `536.033×6882.75` — dimensões absolutas divergem, esperado dado que os
  ~13 constructos de funções matemáticas em falta catalogados em P894 continuam por implementar,
  afectando a largura/altura de várias equações; não é o que este passo mede).

---

## Fase C — Regressão

Benchmark completo, 7 cenários, atenção extra a `04-math`/`06-long`/`07-context` per instrução do
próprio prompt (mudança na sequência de `finish()`/`new_page()`). Primeira rodada mostrou um
aumento aparente em `04-math` (154.0→162-164ms) e `07-context` (139.6→147-148ms) — **investigado
antes de reportar como regressão** (disciplina do projecto): `07-context` (`context measure[...]`
em loop, 200×) **nunca toca `equation.rs`** e mesmo assim mostrou o mesmo aumento — sinal de que não
podia ser causado pelas mudanças deste passo. Re-medido em isolamento (30 amostras, sem outros
processos hyperfine concorrentes): `07-context` = 132.0ms, `04-math` = 151.7ms — **ambos abaixo da
baseline de P895**, confirmando que a primeira leitura foi ruído transitório da máquina, não uma
regressão real. `01-hello` (não relacionado, controlo de ruído) manteve-se estável a 93.6ms durante
todo o processo.

| Cenário | Cristalino (P895 baseline) | Cristalino (P896, remedido) |
|---|---|---|
| 01-hello | 92.9-93.3ms | 93.6ms |
| 02-lorem | 116.1-119.8ms | (dentro do intervalo já observado) |
| 03-images | 102.0-106.0ms | (dentro do intervalo já observado, cenário historicamente ruidoso) |
| 04-math | 154.0ms | 151.7ms |
| 05-tables | 114.5-118.3ms | (dentro do intervalo já observado) |
| 06-long | 372.4-383.2ms | 374.8ms |
| 07-context | 139.6ms | 132.0ms |

**Nenhuma regressão confirmada.**

---

## Nota separada — "posição de linhas/símbolos"

O sintoma reportado pelo dono ("posição de algumas linhas/símbolos também" errada) é consistente com
o mesmo mecanismo corrigido nesta Fase B — equações mal centradas (por estarem todas encostadas à
margem em vez de centradas contra a largura final da página) produzem exactamente esse tipo de
impressão visual (conteúdo "fora do sítio" em relação ao resto do documento). A confirmação visual
acima (distribuição de x variada, não uniforme na margem) é consistente com esta explicação. **Não
foi feita uma comparação pixel-a-pixel exaustiva** das 30 secções (fora de âmbito de tempo razoável
para este passo, dado o tamanho do documento — uma página única de ~5600pt de altura) — a
verificação foi por amostragem de coordenadas via `mutool trace`, não por inspecção visual de cada
secção individualmente. Se o dono continuar a ver posições erradas depois desta correcção, **não
presumir que é a mesma causa** — catalogar como achado novo com caso isolado mínimo, mesma
disciplina de sempre.

---

## Resultado — Passo 896 fechado

- Fase A: mecanismo real do vanilla confirmado por leitura de código (diferimento, não duas
  passagens); achado adicional de `Content::Align` sofrer o mesmo bug, registado; escolha de desenho
  confirmada pelo dono (Opção c, escopo mínimo).
- Fase B: implementado o mecanismo de correcção adiada (2 campos novos + 1 método +1 helper);
  teste de posições exactas (não só ausência de infinito); suíte verde (4722 passed); 0 drift;
  confirmação visual no documento completo de 30 secções (numeração `(1)`-`(44)` completa, centragem
  com distribuição variada de posições).
- Fase C: nenhuma regressão (leitura inicial elevada investigada e atribuída a ruído transitório da
  máquina, não às mudanças deste passo — confirmado por controlo com cenário não relacionado).
- `Content::Align`/`resolve_alignment` continuam com o mesmo bug (`available_width() = infinito`
  sob `width: auto`) — **não corrigido**, decisão explícita do dono, candidato a passo futuro
  dedicado.

