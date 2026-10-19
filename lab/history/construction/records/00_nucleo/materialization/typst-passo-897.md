# Passo 897 — `Content::Align`/`resolve_alignment` sob `width: auto` (mesmo bug de P896, consumidor diferente)

**Precede este passo**: `typst-passo-896-relatorio.md`, secção 2 ("Achado que alarga o âmbito
declarado do passo") e a nota final ("`Content::Align`... não corrigido, decisão explícita do
dono, candidato a passo futuro dedicado"). Ler antes de começar — o mecanismo já está identificado
e a máquina de correção já existe, este passo é sobre reaproveitar, não descobrir do zero.

**Pré-condição de árvore**: `git status`. P893 continua parado no gate. P894/895/896 — confirmar se
já commitados antes de começar.

---

## O bug (já confirmado em P896, não redescobrir)

`01_core/src/engine/layout/mod.rs::available_width()` (`:644-650`) devolve `f64::INFINITY`
explicitamente quando `page_config.width` é infinito. `resolve_alignment(...)` (`:704-727`) — a
mesma função usada pela centragem de equação em `equation.rs`, já corrigida em P896 para esse
consumidor — calcula `HAlign::Center => origin_x + (available_w - content_w) / 2.0`. Com
`available_w = INFINITY`, o resultado é infinito para **qualquer** conteúdo alinhado ao
centro/direita sob `width: auto` — `#align(center)[...]`, `#align(right)[...]`, fora de modo
matemático. P895 não tocou nisto (só corrigiu os dois pontos de `equation.rs`); P896 confirmou o
mecanismo mas não corrigiu, por decisão explícita de escopo.

## Fase A — confirmar antes de reaproveitar

1. Confirmar se o mecanismo de correção diferida implementado em P896 (2 campos novos de metadados
   pendentes + método de resolução + `helpers::shift_frame_item_x`, todos referenciados no
   relatório de P896, secção "Fase B" — ler o diff real, não presumir a partir só do resumo) é
   directamente reaproveitável para `Content::Align`, ou se `placement.rs`/`resolve_alignment` tem
   alguma particularidade que exija adaptação (por exemplo, `Content::Align` pode aninhar-se de
   formas que uma equação de bloco não aninha — confirmar se isso complica o registo de metadados
   pendentes).
2. Confirmar se `Content::Align` contribui para a determinação da largura/altura final da página sob
   `width: auto` (ou seja, se conteúdo alinhado também pode ser a "coisa mais larga" que define o
   tamanho final) — se sim, a correção deste passo também deve mudar as dimensões finais da página
   em documentos que usem `#align` sob `width: auto`, não só reposicionar o conteúdo já medido.
   Confirmar com um caso de teste mínimo antes de assumir que não afecta.
3. Confirmar se há mais consumidores de `resolve_alignment`/`available_width()` além de
   `equation.rs` (já corrigido) e `placement.rs::layout_align` — grep exaustivo, não presumir que
   são só estes dois.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

1. Teste que falhe primeiro: `.typ` mínimo com `#set page(width: auto)` + `#align(center)[...]` de
   texto normal (fora de modo matemático), confirmando posição finita e centrada contra a largura
   final da página — mesmo padrão de teste de P896 (posições exactas, não só ausência de infinito).
   Se a Fase A ponto 2 confirmar que `Content::Align` também afecta o tamanho final da página,
   incluir um segundo teste com múltiplos blocos alinhados de larguras diferentes, mesmo padrão do
   teste de equações de P896 (`p896_equacoes_de_bloco_centram_contra_a_largura_final_da_pagina`).
2. Implementar reaproveitando o mecanismo de P896 (ou adaptando, conforme a Fase A concluir).
3. Suíte completa verde, discriminada por crate.
4. Recompilar o `.typ` completo de 30 secções (hash mais recente, o de P896 —
   `sha256:9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`, confirmar se ainda é o
   actual) e confirmar visualmente que qualquer `#align` sob `width: auto` no documento (se houver
   algum nas 30 secções — confirmar, pode não haver nenhum caso real neste ficheiro específico, o
   que não invalida o teste unitário) fica posicionado correctamente.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, mesma atenção de P896 (`04-math`, `06-long`, `07-context`). Se a
primeira leitura parecer regressão, investigar antes de reportar como tal — mesma disciplina que
P896 já demonstrou (controlo com cenário não relacionado, remedição isolada) antes de aceitar
ruído como regressão real.

---

## Achado registado, fora de escopo deste passo — espaçamento ausente em torno de texto entre aspas em modo matemático

Confirmado por comparação directa dos PDFs de seções 24 e 30 (dono do projecto, mesmo momento/
ficheiro que os achados anteriores desta frente): texto literal entre aspas dentro de `$...$`
(`"sujeito a"`, `"is natural"`, `"for all"`) não recebe espaço de nenhum dos lados no cristalino,
enquanto o vanilla mantém espaço normal antes e depois. Exemplos:
- `𝑓(𝑥)sujeito a𝑔𝑖(𝑥)` (cristalino) vs `𝑓(𝑥) sujeito a 𝑔𝑖(𝑥)` (vanilla).
- `𝑥is natural` (cristalino) vs `𝑥 is natural` (vanilla).
- `for all𝑥` (cristalino) vs `for all 𝑥` (vanilla).

Candidato a mecanismo: provavelmente o mesmo tipo de lacuna de `compute_gaps`/`spacing_between`
(`01_core/src/engine/math/layout/spacing.rs`, já tocado em P891) — texto literal (`MathClass::Text`
ou equivalente) pode não ter regra de espaçamento contra vizinhos, do mesmo jeito que script-size
não tinha antes de P891. **Não confirmado por leitura de código, é hipótese.** Fica registado para
um passo dedicado — não misturar com a correcção de `Content::Align` deste passo, mecanismos
provavelmente não relacionados (um é sobre resolução de largura de página, o outro é sobre regras
de espaçamento entre classes de conteúdo matemático).

## Resultado esperado

- Header de linhagem actualizado nos ficheiros tocados (`placement.rs` e o que mais a Fase A
  confirmar).
- Teste(s) novo(s) com posições exactas, mesmo padrão de P896.
- Relatório com: veredicto da Fase A (reaproveitamento directo ou adaptação necessária; se
  `Content::Align` afeta dimensão final de página; outros consumidores encontrados), confirmação
  visual, benchmark completo.
- Achado de espaçamento em texto citado registado, não investigado neste passo.
