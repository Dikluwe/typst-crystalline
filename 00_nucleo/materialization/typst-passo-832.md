# Prompt — typst-passo-832: `layout::frame` — texto omitido silenciosamente em `move`/`rotate`/`scale` (achado #58, GRAVE) + `Neg` ausente (achado #59)

**Origem**: achados #58 e #59 de P831 (lote 5)
**Estado**: aguardando execução — **prioridade máxima da fila de 44**, junto com o achado #18 (P833)

---

## Achado #58 — GRAVE: perda silenciosa de conteúdo

**Medição de P831**: `#rotate(30deg)[Rodado sozinho]` — cristalino: página completamente em branco, **exit 0**, sem erro nem warning. Vanilla: renderiza o texto rodado. Mesma perda acontece dentro de `move`/`scale`.

**Causa já localizada por P831**: `01_core/src/engine/layout/helpers.rs:249-288` — a função `collect_items_at` só trata `Content::Shape` e `Content::Sequence`; qualquer outro tipo de conteúdo (incluindo texto) cai no braço `_ => {}` e é descartado sem aviso. Chamada a partir de `01_core/src/engine/layout/transform.rs:61`. Vanilla: `frame.rs:184,376`, `transform.rs:56`.

### Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

### Sonda (completar antes de corrigir)
1. Reproduzir o caso exacto de P831 (`#rotate(30deg)[Rodado sozinho]`) com os dois binários — confirmar exit code e ausência total de erro no cristalino.
2. Testar `move()` e `scale()` isoladamente com texto, e depois com uma combinação de texto + shape (para confirmar que shape continua funcionando dentro do mesmo wrapper).
3. Testar um caso com conteúdo aninhado mais profundo (ex.: `rotate(30deg)[#strong[Rodado] e mais texto]`, uma sequência com múltiplos tipos de nó) para mapear todos os tipos de `Content` que `collect_items_at` precisa tratar, não só texto simples.
4. Confirmar no vanilla (`lab/typst-original/`) como `frame.rs`/`transform.rs` tratam genericamente qualquer tipo de conteúdo dentro de uma transformação — é provável que o vanilla trate a coleta de itens de forma recursiva e exaustiva sobre a árvore de `Content`, sem lista fechada de variantes.

### Implementação
Reescrever `collect_items_at` para tratar todos os tipos de `Content` de forma recursiva/exaustiva (o mesmo padrão que o resto do projeto já usa em outros pontos de percurso de árvore de conteúdo), em vez de uma lista fechada com fallback silencioso. Garantir que o fallback, se ainda for necessário para algum tipo não tratado, produza pelo menos um log/debug interno detectável — não silêncio total.

### Validação
1. Recompilar. Repetir os casos da sonda — texto, shape, combinação, aninhamento — todos renderizando dentro de `move`/`rotate`/`scale`, batendo com o vanilla.
2. Teste de regressão: confirmar que `#place` e outros usos já cobertos de coleta de itens continuam funcionando (a função pode ser compartilhada com outros caminhos — verificar).
3. Suíte `typst-core` completa, comando + contagem antes/depois.

---

## Achado #59 — `Neg` ausente para `Angle` (e `Ratio`/`Fraction`/`Duration`)

**Medição de P831**: `-15deg` — cristalino `error: cannot apply Neg to angle` (exit 1); vanilla compila. Faltam braços `Neg` para `Angle`/`Ratio`/`Fraction`/`Duration` em `01_core/src/engine/eval/operators.rs:748-766`. Vanilla: `foundations/ops.rs:80`.

### Sonda
Testar `-15deg`, `-50%`, `-1fr`, e negação de `Duration` (confirmar sintaxe/constructor do vanilla para duration antes de testar) nos dois binários.

### Implementação
Adicionar os braços `Neg` para os quatro tipos em `operators.rs`, replicando a semântica do vanilla (negação numérica simples para cada um).

### Validação
Os quatro casos batendo com o vanilla. Suíte completa, comando + contagem antes/depois.

---

## Relatório

`00_nucleo/diagnosticos/typst-passo-832-relatorio.md`, uma seção por achado (#58, #59), cada uma com: medição antes, código identificado, diff, medição depois, contagem de testes. O achado #58 precisa de nota explícita confirmando que a correção cobre conteúdo arbitrariamente aninhado, não só o caso mínimo testado.
