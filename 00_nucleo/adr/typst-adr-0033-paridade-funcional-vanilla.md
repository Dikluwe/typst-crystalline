# ⚖️ ADR-0033: Paridade funcional com vanilla como invariante arquitectural

**Status**: `EM VIGOR`
**Data**: 2026-04-22

---

## Contexto

A Arquitectura Cristalina é refactor do Typst vanilla com três
objectivos declarados: pureza física (ADR-0029), performance como
domínio de L1 (ADR-0030), e manutenção por LLMs (atomização,
ADRs, linter). Durante a série de passos 84.x — particularmente
84.5 (Alignment/Align2D) e 84.6 (Place/PlacementScope) — apareceu
implicitamente uma regra não escrita: **o comportamento observável
do Typst cristalino deve corresponder ao do vanilla, mesmo quando
a forma interna diverge**.

O Passo 84.5 é o caso paradigma: ao materializar `Alignment`, a
escolha foi `Align2D` (struct com dois campos) em vez de `Alignment`
(enum com dezenas de variantes). A forma diverge; a semântica de
`center + bottom` (combinação de alignments) é idêntica. Durante
o diagnóstico do passo, a decisão foi deliberada: preservar
semântica, divergir na forma quando há ganho arquitectural.

O Passo 84.7 (relatório de auditoria) identificou esta regra como
Input implícito e propôs formalização em ADR dedicado. Este é esse
ADR.

---

## Decisão

### Regra

**Para qualquer input, o output observável do Typst cristalino é
idêntico ao output do Typst vanilla.**

"Output observável" inclui:
- Resultado visual final (PDF, PNG, SVG) — bytes diferentes são
  aceitáveis apenas se a diferença for invisível (ex: ordenação
  interna de dicionários, comentários).
- Mensagens de erro — texto pode ser reescrito com mais clareza,
  mas **sentido** e **localização no código-fonte** devem ser
  idênticos.
- Ordem de avaliação observável pelo utilizador (side effects,
  outputs de `#let`, etc.).

### Divergências permitidas

Internas (não observáveis ao utilizador):
- **Forma estrutural**: struct vs enum, `Vec` vs `Arc<[T]>`,
  hierarquia vtable vs enum tagged.
- **Representação de tipos**: `Align2D { x, y }` em vez de enum
  com 9+ variantes para todas as combinações de alinhamento.
- **Optimizações de performance**: caching de sub-frames, hashing
  preventivo, partilha via `Arc`.
- **Organização de módulos**: ficheiros divididos em L0-L4, etc.

### Divergências proibidas

Observáveis ao utilizador:
- **Semântica de operadores** sobre tipos vanilla. Exemplo:
  `center + bottom` retorna `Align2D { x: Center, y: Bottom }`,
  combinando os dois alinhamentos — não retorna erro "can't add
  alignments". O operador `+` sobre `Alignment` vanilla tem esta
  semântica; cristalino preserva.
- **Mensagens de erro com sentido diferente**. Um erro de
  "expected content, found int" do vanilla não pode virar "type
  mismatch" no cristalino.
- **Regras de combinação**. Por exemplo, show rules aplicam-se por
  ordem de declaração no vanilla; cristalino preserva mesmo que
  a estrutura de dados seja diferente (`Arc<[ShowRule]>` em vez de
  `Vec<ShowRule>`).
- **Ordem visível** de operações. `#let x = { a(); b() }` avalia
  `a` antes de `b` no vanilla; cristalino preserva.

---

## Exemplos de aplicação (com divergências reais)

### Exemplo 1 — Passo 84.5 / ADR-0029: Alignment → Align2D

**Vanilla**: enum `Alignment` com ~18 variantes mais aliases
(`Center`, `Left`, `Right`, `Top`, `Bottom`, `Horizon`, combinações
especializadas, etc.). Operador `+` combina alinhamentos
unidimensionais em bidimensionais.

**Cristalino**: struct `Align2D { x: HAlign, y: VAlign }`. Operador
`+` implementado para preservar a mesma combinação (se ambos os
operandos têm o mesmo eixo, é erro; se têm eixos diferentes,
combinam-se nos dois campos).

**Observabilidade preservada**:
- `align(center)` produz o mesmo output visual.
- `align(center + bottom)` produz o mesmo output.
- `align(center + left)` produz o mesmo erro ("cannot add two
  horizontal alignments").

**Divergência**: a representação em memória é completamente
diferente. O enum vanilla tem ~18 variantes discriminadas; o struct
cristalino tem dois campos de enum mais pequenos.

### Exemplo 2 — ADR-0026 / ADR-0026-R1: Content

**Vanilla**: `Content` é tipo opaco com vtable interna (trait
`NativeElement`). Elementos são structs distintos implementando
a trait.

**Cristalino**: `Content` é enum com variantes fixas (ex:
`Content::Sequence(Arc<[Content]>)`, `Content::Text(EcoString)`,
etc.).

**Observabilidade preservada**:
- `#show "foo": it => bar` funciona igual (matching pelo tipo de
  elemento).
- Concatenação via `+` produz a mesma sequência.
- Profundidade de aninhamento e ordem de elementos preservadas.

**Divergência**: o vanilla permite adicionar novos tipos de
elemento via trait externa (`impl NativeElement for MyElement`);
cristalino exige que o tipo esteja no enum. Esta é divergência
de **extensibilidade**, aceitável porque L1 cristalino não permite
extensão em runtime (ADR-0029, pureza física).

### Exemplo 3 — Passo 84.4 / DEBT-22: ShowRule

**Vanilla**: `Vec<ShowRule>` dentro de cada scope. Clone de scope
copia todos os show rules (O(n)).

**Cristalino**: `Arc<[ShowRule]>`. Clone é `Arc::clone` (O(1),
incremento de refcount). Ver ADR-0030 secção "Clone profundo vs
`Arc::clone`".

**Observabilidade preservada**:
- Ordem de aplicação dos show rules idêntica.
- Escopo de visibilidade idêntico (rules de escope exterior são
  visíveis no interior).
- Redefinição de regra (`#show` com mesmo seletor) sobreescreve
  igual.

**Divergência**: o vanilla permite que código do utilizador mute
o `Vec` interno em certas condições; cristalino torna isso
impossível porque `Arc<[T]>` é imutável. Esta divergência **não
é observável** porque o Typst vanilla nunca expõe a mutabilidade
ao utilizador — é detalhe de implementação.

---

## Relação com outros ADRs

- **ADR-0029** (pureza física): a paridade é definida **em relação
  ao vanilla**; o vanilla faz I/O de sistema em L2/L3 (mundo do
  compilador), e o cristalino mantém essa separação. A pureza de
  L1 cristalino corresponde à pureza computacional do typst-library
  vanilla.

- **ADR-0030** (performance é domínio): optimizações internas
  (hashing, caching, `Arc::clone`) são permitidas porque a
  paridade é sobre output, não sobre ciclos de CPU.

- **ADR-0026** e **ADR-0026-R1** (Content): exemplo concreto
  de divergência estrutural permitida.

- **ADR-0034** (diagnóstico de tipos vanilla): complementar — o
  diagnóstico obrigatório garante que decisões de paridade são
  tomadas com conhecimento factual do comportamento vanilla, não
  por adivinhação.

---

## Como verificar paridade

Verificação formal é responsabilidade da suite `lab/parity/` (já
existente). Cada tipo/comportamento vanilla tem ou deve ter teste
de paridade que compare output cristalino com output vanilla para
input idêntico.

**Regra operacional**: quando um passo altera comportamento
relacionado com tipo vanilla, o passo deve:
1. Correr `lab/parity/` antes (baseline).
2. Aplicar mudança.
3. Correr `lab/parity/` depois.
4. Reportar zero regressões, ou justificar divergências aceitáveis.

DEBT-9 (cobertura incompleta de `lab/parity/`) continua em aberto
e é relevante para esta regra.

---

## Consequências

**Positivas**:
- Formaliza a regra que já estava a ser seguida implicitamente
  desde 84.5.
- Dá critério claro para decisões futuras: quando divergir do
  vanilla, é aceitável se e só se a divergência não é observável
  pelo utilizador.
- Protege contra "optimizações" que quebram comportamento sem
  intenção.

**Negativas**:
- Exige disciplina em materializações novas: qualquer mudança de
  tipo ou operador precisa de verificar se altera semântica
  observável.
- Pode bloquear refactors que seriam desejáveis por si mesmos mas
  alteram comportamento de erro (ex: mensagens mais claras).
  Resolução: refactors desse tipo requerem ADR próprio justificando
  a divergência.

**Neutras**:
- Não impede divergências puramente internas (forma de dados,
  organização de módulos, performance). A regra é sobre **output
  observável**.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| "Fork total — comportamento pode divergir livremente" | Liberdade máxima | Cristalino vira projecto diferente; perde compatibilidade com ecossistema Typst |
| "Paridade bit-a-bit em todos os outputs" | Teste simples | Impossível — ordenação interna de dicionários, timestamps de PDF, etc., divergem legitimamente |
| **Decisão adoptada: paridade semântica com divergência estrutural permitida** | **Preserva compatibilidade sem amarrar implementação** | **Exige testes de paridade (`lab/parity/`) para garantir** |
| "Paridade apenas para cenários listados no test suite" | Escopo bem definido | Paridade torna-se função do test suite — incompleto = ADR incompleto |

---

## Referências

- ADR-0029 — Pureza física em L1
- ADR-0030 — Performance é domínio de L1 (inclui secção Clone
  profundo vs Arc::clone)
- ADR-0026 + ADR-0026-R1 — Content como enum (exemplo 2)
- ADR-0034 — Diagnóstico obrigatório para tipos vanilla
  (complementar)
- Passo 84.5 — Materialização de Alignment como Align2D (exemplo 1)
- Passo 84.4 — Conversão de ShowRule para Arc<[T]> (exemplo 3)
- DEBT-9 — Cobertura incompleta de `lab/parity/` (relevante)
- `lab/parity/` — suite de testes de paridade (mecanismo de
  verificação)

---

## Anotação cumulativa P310 — Excepção IEEE 754 stdlib

**Data**: 2026-05-20.

P310 formaliza **excepção categórica à paridade observable** em
funções matemáticas escalares da stdlib (`calc.*`) via **ADR-0101
EM VIGOR**. Cristalino rejeita NaN+Inf no resultado de 12 funções
`calc.*` (9 com divergência total vs vanilla + 4 com divergência
parcial); vanilla deixa passar (sin/cos/tan/.../erf retornam `f64`
transparente). Catálogo completo de 67 sítios L1 em
`00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md`.

Status `EM VIGOR` ADR-0033 **preservado literal**. Esta anotação
documenta **excepção categorial documentada** stdlib sem revogar a
regra geral de paridade observable — `eval`/layout/operators/output
PDF preservam paridade total. A excepção concentra-se em 11 sítios
de `01_core/src/engine/stdlib/calc.rs` cobertos pelo helper
`guard_float`.

**Decisão P310**: Opção A de P309 §7.1 — "Conformidade IEEE 754
total (status quo divergente)". Custo de implementação zero;
divergência vanilla **assumida explicitamente** em ADR-0101.

**Paralelo ADR-0054 graded**: precedente já existente de "divergência
consciente documentada via ADR dedicada". P310 segue pattern para
política numérica específica.

**Sub-padrão N=1 inaugural** — *"Excepção categorial documentada via
ADR dedicada"* (P310). Candidato a observação cumulativa futura se
padrão repetir em outras categorias (N≥3).

**Sub-padrão "Anotação cumulativa em vez de ADR nova" N+1
cumulativo** (paralelo P266/P268.1/.../P273 acumuladas em ADR-0054).
P310 reusa pattern via ADR-0093 Pattern 2 (anotação preserva ADR-0033
literal; ADR-0101 é nova distinta).

Cross-references:
- **ADR-0101** — IEEE 754 restrição stdlib (criada P310; EM VIGOR).
- **P308** — `calc.erf` divergência local que motivou auditoria.
- **P309** — diagnóstico transversal IEEE 754 (catálogo 67 sítios,
  comparação vanilla obrigatória todas categorias).
- **P310** — formalização Opção A (esta anotação + ADR-0101 + drift
  L0 duplo).
- `00_nucleo/prompts/engine/stdlib.md` §"Política IEEE 754" — secção
  L0 sítio divergente.
- `00_nucleo/prompts/engine/eval.md` §"Política IEEE 754 —
  propagação silenciosa" — secção L0 sítio paridade total.

---

## Anotação cumulativa P311b — Math style mechanism + composition

P311b.2 quebra o hash `entities/content.rs` (sequência 27 passos
consecutivos termina) com adição do variant `Content::MathStyled`
para suportar 12 funções math style (`bb`/`bold`/`cal`/`frak`/
`italic`/`mono`/`sans`/`scr`/`script`/`serif`/`sscript`/`upright`).

A quebra de hash é **mudança fundacional aceitável** per esta ADR:
- Forma diverge (variant novo adicionado a enum).
- Output observable preservado (chars Unicode variant correctos
  para math style; layout PDF paritário vanilla).
- Mecanismo escolhido (Caminho I `Content::MathStyled`) preferido
  sobre Caminhos II/III rejeitados em diagnóstico P311a.

**Paridade observable**: 12/12 = 100% paridade categoria math style
vanilla — segunda categoria stdlib cristalina a fechar após `calc`
(41/41 em P308). Composição cross-variant (`bb(cal(x))` outer-wins,
`bold(italic(x))` ortogonal) testada empíricamente em P311b.4 + 6
tests E2E em P311b.5.

**Refinamento empírico vs diagnóstico**:
- Diagnóstico P311a §3.3 propôs "bold ortogonal bitwise OR + size
  multiplicativo".
- P311b.4 implementação empírica refina: regra única `Option::or`
  uniforme (outer-wins em todos os fields incluindo size).
- Refutação documentada em ADR-0103 §"Refutações ao diagnóstico
  P311a §3.3".

**Sub-padrão N=1 inaugural** — *"Refinamento empírico de diagnóstico
durante materialização"*. Candidato a observação cumulativa futura
se padrão repetir (N≥3).

**Sub-padrão "Excepção categorial documentada via ADR dedicada"
N=2 cumulativo** (P310 IEEE 754 + P311b Math style). Padrão
"divergência consciente documentada via ADR" continua a consolidar-
se.

Cross-references P311b:
- **ADR-0102** — Math-Style-Mechanism (variant `Content::MathStyled`;
  criada P311b; EM VIGOR).
- **ADR-0103** — Math-Style-Composition (4 regras outer-wins; criada
  P311b; EM VIGOR).
- **P311a** — Diagnóstico (`diagnosticos/diagnostico-math-style-
  passo-311a.md`).
- **P311b** — Materialização (6 sub-passos).
- `00_nucleo/prompts/entities/math_style.md` — L0 dedicado para
  `MathStyleKind` + `map_glyph`.
- `00_nucleo/prompts/entities/content.md` §"Variant
  `Content::MathStyled`" — drift que documenta variant 25º.
- `00_nucleo/prompts/engine/stdlib.md` §"12 funções math style" — drift
  para registar funções nativas.
- `00_nucleo/prompts/engine/math/layout.md` §"Variant
  `Content::MathStyled`" — drift para handler + apply_math_style.
