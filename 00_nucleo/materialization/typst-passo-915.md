# Passo 915 — introduzir "cramped" em modo matemático (adiado de P914)

**Precede este passo**: `typst-passo-914-relatorio.md` (decisão de escopo dividido, aprovada pelo
dono: extremos + gap simultâneo + kerning de 2 alturas em P914; "cramped" adiado para este passo).
`ADR-0123` (geometria tipográfica como categoria própria) — **este passo é o primeiro a aplicar essa
ADR desde que foi escrita, não só citá-la**: a Fase A é obrigatoriamente "ler a fórmula literal do
vanilla antes de escrever qualquer aritmética", não "inventar e validar depois".

**Pré-condição de árvore**: `git status`. P917 já commitado (confirmado pelo dono). Este passo mexe
em `attach.rs` (tocado por P914/P917) e provavelmente em `TextStyle`/`MathStyle` e nos call sites
que decidem estilo de conteúdo matemático (`frac.rs`, possivelmente `root.rs`) — confirmar
ausência de conflito antes de começar.

---

## O que "cramped" é (não inventar — ler o vanilla primeiro, per `ADR-0123`)

Já sabido de P914 (não redescobrir): `compute_script_shifts` (vanilla, `scripts.rs:318-382`) usa um
termo diferente para `superscript_shift_up` quando o contexto é "cramped" — mas P914 não
determinou **onde** o vanilla decide que um contexto é cramped, só que o campo existe e é
consumido.

## Fase A — ler o vanilla, não presumir (obrigatório por `ADR-0123`)

1. Localizar, no código-fonte real do vanilla (`lab/typst-original/`), **todos** os pontos que
   definem/propagam um estado "cramped" — candidatos típicos da tradição tipográfica (TeXbook,
   Apêndice G, que o OpenType MATH/vanilla também seguem): dentro do denominador de uma fração;
   dentro de certas construções de raiz; dentro de sub-scripts (não super-scripts) de forma
   recursiva. **Não assumir esta lista — confirmar cada item lendo o código real**, e registar
   quais desta lista o vanilla de facto implementa, e quais (se algum) foram invenção da memória
   humana comum sobre TeX e não se aplicam ao Typst especificamente.
2. Ler a fórmula exacta que `compute_script_shifts` usa para o termo cramped — não é só "usar
   outra constante", confirmar a fórmula completa (per P914, o não-cramped já usa `.max()` de
   3-4 termos; confirmar se cramped substitui um desses termos ou desliga outros).
3. Confirmar se cramped afecta só `attach.rs` (sub/sobrescrito) ou também outros módulos já
   atomizados (`frac.rs`, `root.rs`) — por exemplo, se o próprio conteúdo de um numerador/denominador
   também muda de tamanho de fonte/estilo por causa de cramped, isso já seria responsabilidade de
   outro módulo, não deste passo.

## Fase A.1 — desenhar a propagação (decisão de arquitetura, gate obrigatório)

1. Decidir onde o estado cramped vive: campo novo em `TextStyle`, ou um parâmetro adicional só nas
   funções de layout matemático que precisam dele (mais contido, mesmo espírito de P893 ter
   evitado mexer em mais do que o necessário). Registar a decisão com justificação.
2. Mapear **todos** os call sites que precisariam de activar/propagar cramped, confirmados pela
   Fase A ponto 1 — mesmo padrão de inventário exaustivo já usado em P899/901/906.
3. **Se decidir por campo novo em `TextStyle`/assinatura pública**: parar aqui, editar os L0s
   afectados, sincronizar hashes (`crystalline-lint --fix-hashes .`), e **aguardar confirmação do
   dono antes da Fase B** — mesmo protocolo de P893/896/906/909.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria nova; **usar métricas de
fonte real desde o primeiro teste, não `FixedMetrics`** — lição directa de P912/913/917: a lacuna
de cobertura com fonte real foi exactamente o que permitiu bugs anteriores sobreviverem)

1. Agente A escreve testes com ground-truth calculado a partir da fórmula real do vanilla (Fase A),
   usando dados de fonte real (`NewCMMath-Regular.otf`, o mesmo `rev` pinado confirmado por P917 —
   **não** `03_infra/fixtures/fonts/NewCMMath-Book.otf`, ficheiro diferente, já confundido uma vez
   nesta frente). Cobrir pelo menos: superscript em contexto cramped vs. não-cramped (mesma base,
   resultado tem de diferir), e um caso dentro de fracção (se a Fase A confirmar que denominador é
   cramped). Confirma vermelho.
2. Agente B implementa conforme o desenho da Fase A.1.
3. Revisão do orquestrador — mesmo padrão de P898/901/906/908/916: testar pelo menos um caso
   composto não coberto pelos testes do Agente A (por exemplo, cramped **e** sup+sub simultâneos ao
   mesmo tempo, já que P914 introduziu o ajuste de gap simultâneo — confirmar que os dois mecanismos
   interagem correctamente, não só cada um isolado).
4. Suíte completa verde, discriminada por crate — **números reais no relatório, não "ver cargo
   test"** (lição de P916).
5. Confirmar visualmente/via `mutool trace` um caso real (`frac(a^2, b)` ou equivalente, se
   denominador for cramped) contra o vanilla — mesma disciplina de `mutool trace` que P917 usou
   para desfazer o artefacto de "glyph ID local ao subset" (comparar avanço/posição real, não
   apenas ID de glifo entre PDFs diferentes).
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`. Nota de P917: os ficheiros canónicos dos 7
cenários não persistem no repositório (recriados por sessão) — reconstruir equivalentes, mesma
prática já estabelecida.

## Resultado esperado

- Fase A documentando a fórmula/pontos de propagação reais do vanilla, com `file:line`, per
  `ADR-0123` — não fórmula inventada e validada depois.
- Decisão de arquitectura registada (campo novo vs. parâmetro local), com gate de confirmação se
  necessário.
- Testes com fonte real desde o início, ground-truth calculado no teste.
- Confirmação visual/geométrica com o artefacto de "glyph ID entre subsets diferentes" evitado.
- Suíte completa com números reais.
- Benchmark completo.

## Achados em aberto desta frente, não tocados por este passo (registo, não esquecimento)

- Desalinhamento vertical entre elementos adjacentes numa sequência matemática (P917, pré-existente,
  não investigado).
- `assembly` não atinge a altura-alvo total em matrizes/casos de 6+ linhas (P916/917, pré-existente,
  caminho estruturalmente diferente do vanilla).
- Reconciliação de numeração de ADRs (`P910`) — ainda sem confirmação contra o repositório real.
