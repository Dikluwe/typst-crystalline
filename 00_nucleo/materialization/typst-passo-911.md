# Passo 911 — auditar `attach.rs`/`matrix.rs`/`cases.rs`/`delimited.rs`/`stretchy.rs`/`assembly.rs` contra a fórmula real do vanilla

**Precede este passo**: `ADR-0123` (geometria tipográfica como categoria própria) — regra: portar a
fórmula literal do vanilla antes de escrever/aceitar geometria de layout como correta, não validar
só empiricamente depois. Este passo aplica essa regra retroativamente aos módulos de
`math/layout/` que **já estão atomizados** (não precisam do trabalho de `P909`), mas **nunca foram
auditados** contra o vanilla da mesma forma que `root.rs`/`frac.rs`/`underover.rs` acabaram sendo,
um de cada vez, só depois de já estarem errados em produção.

**Não é achado confirmado — é auditoria preventiva.** Ao contrário de P901/P905/P906 (correção de
bug já visível), este passo pode não encontrar nada errado em nenhum dos seis. Isso é um resultado
bom e válido, não motivo para inventar achado.

**Pré-condição de árvore**: `git status`. Confirmar estado de `P906`/`P907`/`P908` (se tocaram
algum destes seis ficheiros) e de `P909` (atomização de `accent`/`cancel`/`underover`/`op`, mesmo
diretório `math/layout/`, ficheiros diferentes — confirmar que não há conflito de merge).

---

## Método (mesmo para os seis, um de cada vez — não misturar achados)

Para cada módulo, nesta ordem (do mais simples ao mais provável de ter geometria não-trivial):

1. **`delimited.rs`** (`MathDelimited` — par de delimitadores fixos).
2. **`matrix.rs`** (`MathMatrix` — matrizes).
3. **`cases.rs`** (`MathCases` — chaves grandes).
4. **`attach.rs`** (`MathAttach` — sub/sobrescritos; já parcialmente auditado em P891 para
   `math_kern`, mas não para o resto da fórmula de posicionamento).
5. **`stretchy.rs`** (operadores extensíveis — já tocado por P899/905/906 indiretamente; confirmar
   se a fórmula de seleção de variante em si, não só os dados que ela consome, está correta).
6. **`assembly.rs`** (montagem de delimitadores grandes por partes — mesma nota de `stretchy.rs`).

### Fase A, por módulo

1. Ler a fórmula real do cristalino (`file:line`, não resumo).
2. Ler a fórmula real do vanilla correspondente (`lab/typst-original/`, `file:line`).
3. Comparar termo a termo. Se idênticas (mesma convenção de baseline, mesmos offsets, mesmas
   constantes de `MathConstants` consumidas): registar "auditado, sem achado", com a referência
   cruzada dos dois `file:line`, e passar ao próximo módulo.
4. Se divergirem: **não corrigir ainda dentro deste passo de auditoria** — registar o achado com o
   mesmo rigor de P901/905/906 (fórmula errada, convenção assumida, evidência), e decidir se cabe
   corrigir aqui mesmo (achado pequeno, baixo risco) ou destacar para passo dedicado (achado grande,
   mesmo critério usado nos passos anteriores desta frente).

### Fase B — só para os achados que a Fase A decidir corrigir neste mesmo passo

TDD normal (protocolo de dois agentes de P898 se o achado for cálculo geométrico novo, não só
constante trocada). Suíte completa verde. Confirmação visual/geométrica (`mutool trace`, não só
`pdftotext`) contra o vanilla real.

## Fase C — Regressão

Só se algo tiver sido corrigido na Fase B. Se os seis módulos passarem "auditado, sem achado" sem
nenhuma correção, não é preciso benchmark — nada mudou.

## Resultado esperado

- Relatório com uma linha por módulo: auditado, `file:line` dos dois lados comparados, resultado
  (sem achado / achado corrigido aqui / achado destacado para passo próprio).
- Se algum achado for encontrado e corrigido: mesma disciplina de relatório dos passos anteriores
  (causa, fórmula, teste, confirmação visual).
- Se nenhum achado for encontrado em nenhum dos seis: isso fecha, de vez, a suspeita levantada por
  `ADR-0123` de que a convenção errada de `root.rs`/`frac.rs`/`underover.rs` era um padrão
  sistemático em todo `math/layout/` — passa a ser um resultado confirmado (3 de 9 módulos tinham o
  problema, os outros 6 não), não uma suposição.
