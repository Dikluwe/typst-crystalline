# Prompt L0 — `stdlib/transforms` — módulo `transforms`
Hash do Código: 7655d1cb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3


**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/transforms.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P78 (`move`/`rotate`/`scale`) e P156F (`skew`).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded), ADR-0061
(layout roadmap).
**Convenções partilhadas**: ver `00_nucleo/prompts/core/compiler/stdlib/mod.md`.

---

## Módulo `transforms` — funções nativas de transformação

Este módulo implementa transformações geométricas 2D aplicadas a content:
translação, rotação, escala e distorção (skew). Todas produzem
`Content::Transform { matrix, body }` com uma `TransformMatrix` adequada.

Todas as funções partilham a assinatura padrão de `native_*`:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

O corpo (`body`) é sempre o primeiro argumento posicional do tipo `Content`.

---

### `native_move(dx?, dy?, body)`

**Assinatura**: `move(dx: Length?, dy: Length?, body: Content) -> Content`

**Argumentos**:
- `dx`, `dy`: deslocamento em pt (`Length`, `Float`, `Int`). Default `0.0`.
- `body`: primeiro argumento posicional `Content` obrigatório.
- Não aceita outros argumentos nomeados.

**Semântica**: Cria `Content::Transform { matrix: translate(dx, dy), body }`.

**Paridade vanilla**: Equivalente a `#move(dx: 1cm, dy: 2cm, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
move([x]) -> Transform translate(0,0)
move(dx: 5pt, [x]) -> Transform translate(5,0)
move(dx: 1cm, dy: 2cm, [x]) -> Transform translate(1cm,2cm)
move(dx: -3pt, [x]) -> Transform translate(-3,0)
move() -> Err "exige um corpo de conteúdo"
```

---

### `native_rotate(angle, body)`

**Assinatura**: `rotate(angle: Angle | Float, body: Content) -> Content`

**Argumentos**:
- `angle`: `Value::Angle` (graus → radianos) ou `Value::Float` (radianos
  directos). Pode vir como named `angle:` ou como primeiro argumento
  posicional compatível.
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: rotate(angle_rad), body }`.

**Paridade vanilla**: Equivalente a `#rotate(45deg, [body])` / `#rotate(1rad, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
rotate(45deg, [x]) -> Transform rotate(pi/4)
rotate(0rad, [x]) -> Transform rotate(0)
rotate([x]) -> Err "exige um corpo" (ou angle default 0 se body presente)
rotate(45deg) -> Err "exige um corpo de conteúdo"
```

---

### `native_scale(x?, y?, body)`

**Assinatura**: `scale(x: Float | Int?, y: Float | Int?, body: Content) -> Content`

**Argumentos**:
- `x`: factor de escala horizontal. Default `1.0`.
- `y`: factor de escala vertical. Default igual a `x` (escala uniforme).
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: scale(sx, sy), body }`.

**Paridade vanilla**: Equivalente a `#scale(2, [body])` / `#scale(x: 2, y: 0.5, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
scale(2, [x]) -> Transform scale(2,2)
scale(x: 2, y: 0.5, [x]) -> Transform scale(2,0.5)
scale([x]) -> Transform scale(1,1)
scale(2) -> Err "exige um corpo de conteúdo"
```

---

### `native_skew(ax?, ay?, body)`

**Assinatura**: `skew(ax: Angle | Float?, ay: Angle | Float?, body: Content) -> Content`

**Argumentos**:
- `ax`: ângulo de distorção horizontal (default `0`). Aceita `Angle` ou
  `Float` (radianos).
- `ay`: ângulo de distorção vertical (default `0`).
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: skew(ax_rad, ay_rad), body }`.

**Paridade vanilla**: Equivalente a `#skew(30deg, [body])` / `#skew(ax: 10deg, ay: 5deg, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).
- Ângulos com `|a| >= π/2 - 1e-3` rad rejeitados porque `tan` diverge.

**Testes canónicos**:
```
skew(30deg, [x]) -> Transform skew(pi/6, 0)
skew(ax: 10deg, ay: 5deg, [x]) -> Transform skew(10°, 5°)
skew(90deg, [x]) -> Err "ângulo demasiado próximo de ±π/2"
skew([x]) -> Transform skew(0,0)
skew(10deg) -> Err "exige um corpo de conteúdo"
```

## P953 — `scale()` aceita factor posicional e `Ratio` (`#scale(150%)`)

**Achado** (`typst-passo-953` Fase A, medido no render + trace):
`#scale(150%)[grande]` renderizava a 11pt (sem escala) — `native_scale` só lia
`x`/`y` **nomeados** e só `Float`/`Int`, ignorando o argumento posicional
`Ratio` (a matriz ficava identidade). O vanilla aceita o factor como
**posicional** (`ScaleElem.x: Smart<ScaleAmount>` com `#[positional]`,
`layout/transform.rs:113-136`), com `Ratio` directo (150% = 1.5).

**Correcção**: `native_scale` passa a aceitar o factor como posicional
(primeiro arg posicional `Float`/`Int`/`Ratio` = x; o segundo, se existir e
não for o body, = y) mantendo os nomeados `x:`/`y:` a funcionar, e aceita
`Value::Ratio` (`r.0`) em ambos os canais. O layout/export já aplica a
matriz do `Content::Transform` ao grupo (rotação provada no mesmo documento
de teste) — a correcção é só no parsing do factor.

## Transformações tipadas — contrato substitutivo

## Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

Esta seção substitui cláusulas anteriores conflitantes somente para a família
Move/Rotate/Scale/Skew; demais responsabilidades permanecem.

## Contrato completo de construção e estilo

O carrier tipado de `00_nucleo/prompts/core/entities/elements/transform.md` substitui
a redução prematura para matriz. Preservar assinaturas públicas native_*.
Validar pela sequência de ocorrências de Args e spans; não procurar apenas
valores numericamente convenientes e descartar os demais. Strings e none
convertem para Content conforme casts vigentes; números não são body. Body é
posicional obrigatório. Argumentos desconhecidos, sobras, tipos errados e
body nomeado produzem os diagnósticos medidos, sem fallback identidade/zero.

- move: dx/dy nomeados aceitam Rel<Length> (ratio, length, mistura). Defaults
  não são inseridos lexicalmente. Int/float/string não viram pontos.
- rotate: angle posicional opcional de tipo Angle, encontrado como no parse
  tipado pinado, inclusive depois de body string. Angle nomeado é rejeitado.
  Origin aceita Align2D preservando eixos ausentes; reflow aceita bool.
- scale: no máximo um fator posicional Auto/Ratio/Length, que assenta ambos os
  eixos; x/y nomeados sobrepõem somente o eixo correspondente. Sem fator, y
  omitido não copia x. Ratio/Length/Auto são distintos; int/float e relative
  length misto não são fatores válidos. Dois auto são construíveis e observáveis;
  o erro de resolução ocorre em layout, não ao inspecionar fields.
- skew: ax/ay apenas nomeados e Angle; origin/reflow como rotate. Não rejeitar
  ângulos finitos por um limiar artificial de proximidade a90 graus. Validação
  de geometria não finita cabe à resolução, sem converter erro em identidade.

O default efetivo é move zero/zero, rotate zero, scale100%/100%, skew zero/zero,
origin center+horizon e reflow false. São defaults de estilo/layout, separados
da presença lexical. set usa os mesmos casts/bindings, sem body: angle de rotate
continua posicional, fator de scale assenta x/y e sobreposições preservam eixos.
Origin faz fold por eixo: um eixo explícito sobrepõe o herdado, os demais herdam.
Set não inventa campos em fields do constructor. Aliases/with devem respeitar
identidade nativa e avaliação única dos argumentos; getters contextuais usam o
mesmo esquema. Atualizar os owners de set/field access antes de implementar.

Não resolver em/fontes/percentuais/escala absoluta em eval. Não usar Styled
artificial para esconder os argumentos autorais; o canal normal de estilos
permanece com seus owners. Parsing, campos e diagnósticos são observáveis.

## Aceitação

Reproduzir todo o corpus de queries e bindings; estender para spans, ordem,
duplicatas, aliases/with, set e leitura contextual. Fields, func, repr, equality,
show/where e encoders devem concordar sobre presença e identidade. Sem promessa
de paridade de Rust/mecânica de macros ou de número de passagens upstream.
