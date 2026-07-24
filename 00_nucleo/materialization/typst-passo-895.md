# Passo 895 — `#set page(width: auto)` + equação de bloco produz `offset_x = infinito`

**Precede este passo**: `typst-passo-894-relatorio.md`, secção "Prioridade 1", subsecção "A causa
real". Ler antes de começar — a causa já está isolada por bisecção binária e leitura de código, este
prompt é só para a Fase B/C que P894 não fez (Prioridade 0/1 daquele passo era diagnóstico puro).

**Pré-condição de árvore**: `git status`. P894 fechou com um teste novo (`dot`) já commitado ou
pendente — confirmar antes de começar. P893 continua parado no gate do L0 (STOP, Fase B não
iniciada) — as áreas de código não se sobrepõem (`equation.rs` vs `font_metrics.rs`/
`MathLayouter::new`), mas confirmar de qualquer forma antes de medir benchmark.

---

## Parte A — corrigir `offset_x = infinito` (prioridade 1 da lista de P894)

### Causa já confirmada (P894, não redescobrir)

`page_config.width` fica `f64::INFINITY` quando `width: auto` — comportamento intencional
(`01_core/src/engine/layout/set_page.rs:34`), resolvido depois via `compute_page_width()`. O
problema é que `01_core/src/engine/layout/equation.rs:116-117` usa `self.regions.current.width`
directamente para centrar equações de bloco **antes** dessa resolução acontecer:

```rust
let usable = self.regions.current.width - 2.0 * self.page_config.margin;
offset_x = Pt(self.page_config.margin + (usable - ext.width) / 2.0);
```

Com `self.regions.current.width = INFINITY`, `usable = INFINITY`, `offset_x = INFINITY`. Confirmado:
só dispara com `width: auto` **e** pelo menos uma equação de **bloco** (`height: auto` sozinho
funciona; `width: auto` com só equação inline funciona; os dois juntos ou `width: auto` sozinho com
bloco sempre produz `inf`).

### Fase A — confirmar o desenho da correção antes de implementar

1. Confirmar exactamente quando `compute_page_width()` resolve o valor final de largura, e se é
   possível chamar `equation.rs` **depois** dessa resolução (reordenar a sequência de layout) ou se
   `equation.rs` precisa de resolver o próprio valor localmente antes de usar
   `self.regions.current.width` (ex.: usar `page_config.width` já resolvido, se existir nesse ponto,
   em vez de `regions.current.width`, que parece ser o valor ainda não resolvido).
2. Confirmar como o vanilla lida com centragem de equação de bloco quando a largura da página é
   `auto` (`lab/typst-original/`) — usar como referência antes de decidir a correcção do cristalino,
   não presumir a solução.
3. Confirmar se há outros consumidores de `self.regions.current.width` no mesmo módulo
   (`equation.rs`) ou em módulos irmãos que possam ter o mesmo problema com `width: auto` (por
   exemplo, qualquer outro elemento centrado horizontalmente) — se houver, decidir se a correcção
   deve ser feita no ponto comum (evitar `INFINITY` propagar de `regions.current.width` antes de
   `compute_page_width()` resolver) em vez de só em `equation.rs`.

### Fase B — Implementação (TDD)

1. Teste que falhe primeiro: `.typ` mínimo com `#set page(width: auto)` + equação de bloco,
   confirmando que a `MediaBox`/dimensão final da página **não** é infinita e que a equação fica
   centrada corretamente (não em `offset_x = infinito`).
2. Implementar a correcção no ponto confirmado pela Fase A.
3. Suíte completa verde, discriminada por crate.
4. Recompilar as secções 1 e 4 do `.typ` de teste (mesmas que expuseram o bug em P894) e confirmar
   visualmente que as equações voltam a aparecer normalmente, com `MediaBox` finita e sensata (não
   612×792 "US Letter" mascarado, nem `inf`).
5. `cargo run -- .` — zero violations.

## Parte B — reconciliar relatório de terceiros (símbolos/modificadores em falta)

**Contexto**: existe uma tabela de outra ferramenta/agente, fora do fluxo deste projecto, alegando
13/30 secções falhando por símbolos/modificadores específicos (`.alt`, `.double`, `.big`, `.h`, `oo`,
`thin`/`med`/`thick`/`quad`/`wide`, `beth`, `prop`, `math.op`). P894 não verificou esta lista
directamente (focou nos 2 crashes reportados originalmente, que acabaram por ter causas diferentes
das alegadas nessa tabela também).

1. Para cada item da lista, um teste isolado mínimo nos dois binários (mesma disciplina — hash
   sincronizado, visual quando aplicável):
   - `epsilon.alt`, `theta.alt`, `phi.alt`, `rho.alt`, `sigma.alt` (modificador `.alt`)
   - `dot.double` (modificador `.double`)
   - `union.big`, `inter.big` (modificador `.big`)
   - `dots.h` (modificador `.h`)
   - `oo` (atalho de infinito)
   - `thin`, `med`, `thick`, `quad`, `wide` (espaçamentos nomeados)
   - `beth`
   - `prop`
   - `math.op(...)` (construtor, distinto de `op("...")` que P894 já confirmou existir de outra
     forma — confirmar qual das duas formas de sintaxe é a testada no `.typ` original, linha 9-11)
2. Catalogar cada um como confirmado ou refutado, mesmo formato usado em P894 para as 5 hipóteses de
   Prioridade 1 (a maioria delas acabou refutada — não presumir que esta lista terceira está mais
   certa sem verificar item a item).
3. **Não implementar correcção nesta parte** — só catalogar. Se algum item confirmar-se como bug
   real e simples (mesmo padrão do `dot` que P894 já corrigiu em Prioridade 2), pode fechar aqui,
   com TDD, mesma disciplina.

## Fase C — Regressão (Parte A)

A correcção da Parte A mexe em `equation.rs`, layout de página. Benchmark completo, 7 cenários,
comparar com a baseline mais recente disponível sem conflito de árvore com P893 (confirmar qual é
antes de medir).

## Resultado esperado

- Header de linhagem actualizado (`equation.rs` e outros tocados pela Parte A).
- Teste novo confirmando `width: auto` + equação de bloco não produz `infinito`.
- Catálogo da Parte B com veredicto item a item da lista de terceiros.
- Relatório com: desenho da correcção da Fase A (Parte A), resultado visual antes/depois, benchmark
  completo, catálogo da Parte B.
