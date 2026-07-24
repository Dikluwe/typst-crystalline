# Passo 898 — `Content::Align`/`Content::Place` no eixo vertical, sob `height: auto`

**Precede este passo**: `typst-passo-897-relatorio.md`, secção "Fora de âmbito", item 1 — achado já
confirmado por leitura de código, não testado, não corrigido. Ler antes de começar.

**Pré-condição de árvore**: `git status`. P893 continua parado no gate. P897 — confirmar se já
commitado.

---

## Protocolo de TDD em dois agentes (novo a partir deste passo)

Nos últimos passos (P887, P888, P897), a ordem estrita "teste falha primeiro, depois código" foi
quebrada mais de uma vez — sempre declarado, nunca escondido, mas sempre por causa da mesma situação
(sessão retomada com implementação já em curso, ou algoritmo complexo demais para especificar o
teste em abstrato antes de escrever código). Para este passo, em vez de confiar na disciplina de
quem executa sozinho, usar dois agentes/sessões separados, sem que um veja o trabalho do outro até
ao ponto certo:

1. **Agente A (testes)**: recebe só a secção "O bug" e "Fase A" abaixo (a especificação do que deve
   e não deve acontecer) — **não recebe acesso a nenhuma implementação de correção, nem a P896/897
   já commitados além do necessário para saber que o mecanismo existe**. Escreve o(s) teste(s) que
   codificam o comportamento esperado (posição finita, alinhamento correto contra a altura final da
   página). Corre os testes, confirma que falham (vermelho), e pára — não implementa nada além do
   teste.
2. **Agente B (implementação)**: recebe o(s) teste(s) já escritos pelo Agente A (sem poder editá-
   los) mais a Fase A completa, e implementa até os testes passarem. Não pode alterar os testes para
   os fazer passar — só o código de produção.
3. Registar no relatório, explicitamente, que os dois papéis foram separados (mesmo que executados
   pela mesma ferramenta em invocações diferentes, sem contexto partilhado da implementação) — isto
   substitui a "confirmação por reversão temporária" usada nos passos anteriores, não é preciso
   repetir esse passo extra se esta separação for seguida correctamente.

---

## O bug (simétrico ao corrigido em P896/897, eixo diferente)

`available_height()` (`01_core/src/engine/layout/mod.rs:684`) e `page_bottom_limit()` (`:697`)
devolvem `f64::INFINITY` quando `page_config.height` é infinito (`height: auto`), mesmo padrão de
`available_width()` já corrigido para o eixo horizontal em P896 (equações) e P897 (`Align`/`Place`).
`VAlign::Horizon` (centro vertical) e `VAlign::Bottom` calculam posição usando esse valor
potencialmente infinito, produzindo `offset_y = infinito` (ou `NaN`, se a fórmula envolver subtração
de dois infinitos — confirmar qual dos dois na Fase A) nos mesmos moldes do bug horizontal original
de P895.

## Fase A — confirmar antes de reaproveitar

1. Confirmar a fórmula exacta usada para `VAlign::Horizon`/`VAlign::Bottom` (equivalente vertical de
   `resolve_alignment`) e se produz `infinito` ou `NaN` sob `height: auto` — as duas têm implicações
   diferentes para o teste (comparação com `NaN` nunca é verdadeira, `assert!(x.is_finite())` cobre
   os dois, mas o sintoma no PDF final pode diferir).
2. Confirmar se `Content::Equation` tem alguma centragem vertical vulnerável ao mesmo bug, além do
   que P896 já corrigiu (P896 tratou centragem **horizontal** de equação; confirmar se há algum
   caso de posicionamento vertical de equação — por exemplo, equações centradas verticalmente numa
   célula ou região — que também dependa de `available_height()`).
3. Confirmar quais consumidores de `available_height()`/`page_bottom_limit()` existem, mesmo padrão
   do grep exaustivo que P897 fez para o eixo horizontal — não presumir que são só
   `layout_align`/`layout_place`, confirmar.
4. Desenhar a correcção: mesmo padrão de 3 peças de P896/897 (campo `pending_*` + método de
   correcção chamado de `finish()`/`new_page()` + ajuste de coordenada Y, provavelmente um
   `shift_frame_item_y` simétrico a `shift_frame_item_x` — confirmar se já existe ou precisa de ser
   criado) — ou reaproveitamento directo se a função de resolução vertical for suficientemente
   parecida com `resolve_alignment` para o mesmo truque de P897 (gravar `origin_y (+ dy, se
   aplicável)` no tuplo pendente).

## Fase B — Implementação (protocolo de dois agentes acima; TDD per `CLAUDE.md`)

1. Agente A escreve teste(s): `.typ` mínimo com `height: auto` + `#align(horizon)[...]`/
   `#align(bottom)[...]`, confirmando posição Y finita e correcta contra a altura final da página
   (mesmo padrão de posições exactas de P896/897, não só ausência de infinito). Incluir também um
   caso com múltiplos blocos de alturas diferentes, mesmo padrão do teste de P896 que confirmou
   contribuição para o tamanho final da página.
2. Agente A confirma vermelho, entrega ao Agente B.
3. Agente B implementa conforme o desenho da Fase A, sem tocar nos testes.
4. Suíte completa verde, discriminada por crate.
5. Recompilar o `.typ` de 30 secções (mesmo hash de P896/897) e confirmar visualmente/geometricamente
   sem regressão (mesmo que este ficheiro não use `height: auto` explicitamente com `#align`
   vertical — confirmar se algum caso já existe nele antes de assumir que não).
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários. **Nota**: P896 e P897 mostraram os dois primeira-leitura inflada
exigindo remedição isolada com controlo. Se isto se repetir uma terceira vez seguida, considerar
aumentar o warmup padrão do benchmark (`hyperfine --warmup 3` → maior) para os passos seguintes,
em vez de investigar o mesmo "falso alarme" a cada passo — decisão a registar no relatório, não
para implementar sem mais.

## Resultado esperado

- Relatório com: separação explícita dos dois agentes documentada, veredicto da Fase A (infinito ou
  NaN, outros consumidores, se `Content::Equation` também é afectado no eixo vertical), testes
  novos, suíte verde, confirmação visual, benchmark completo.
- Se a Fase A ponto 2 confirmar que equações também têm um caso vertical vulnerável: corrigir aqui
  também (mesmo mecanismo), não abrir passo separado só para isso.
