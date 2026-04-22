# Passo 84.6 — `Place` relativo ao contentor pai (DEBT-37)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — `Content::Place` (campos
  `alignment`, `dx`, `dy`, `body`). Introduzido no Passo 82.
- `01_core/src/rules/layout/mod.rs` — braço `Content::Place`
  (linha aproximada 946 segundo diagnóstico do Passo 83.5). Usa
  `line_start_x` para X e `page_config.margin` para Y.
- `01_core/src/rules/stdlib.rs` — `native_place`. Recebe `alignment`,
  `dx`, `dy`, `body`.
- `03_infra/src/integration_tests.rs` — teste `place_nao_altera_cursor_y`
  (Passo 82). Valida o comportamento actual; pode precisar de
  actualização dependendo do cenário identificado.
- `00_nucleo/DEBT.md` — DEBT-37 em Secção 1.

Pré-condição: `cargo test` — 909 testes (737 L1 + 172 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.5 concluído. DEBT-36
encerrado.

---

## Contexto do DEBT-37

A descrição actual de DEBT-37 no DEBT.md:

> "`Content::Place` ancora às margens absolutas da página. O Typst
> suporta `place` relativo ao bloco pai (ex: dentro de um grid,
> `place` ancora na célula). Resolução: passar a área de âncora
> como parâmetro ao processar `Place`."

**Imprecisão**: `Content::Place` já usa `line_start_x` no eixo X
(desde Passo 82) — só o eixo Y é absoluto à margem da página. O
texto do DEBT sugere que ambos os eixos são absolutos; não é o caso.

**Ambiguidade**: o vanilla Typst tem o parâmetro `scope` em `place`
com valores `"column"`, `"parent"`. O comportamento padrão depende
do contexto (dentro ou fora de grid, flow, etc.). A resolução
correcta deste DEBT depende de saber o comportamento exacto do
vanilla — o que nenhum passo anterior diagnosticou.

Este passo **corrige ambas as lacunas**: diagnostica o vanilla,
reconcilia com o código actual, e só então implementa.

---

## Natureza deste passo

Duas fases separadas:

**Fase de diagnóstico (bloqueante e extenso)**: Tarefa 1 — mais
detalhada que nos passos anteriores. Três perguntas bloqueantes
sobre o comportamento do vanilla.

**Fase de implementação (dependente)**: Tarefa 2 executa um dos
três cenários (A, B ou C) com base no diagnóstico.

**Regra absoluta**: Claude Code **não avança para implementação
sem reportar o diagnóstico completo ao utilizador e receber
confirmação do cenário**. Esta regra é mais estrita que nos
passos anteriores porque a decisão afecta compatibilidade com
o vanilla — errar aqui é dívida semântica silenciosa.

---

## Restrições arquiteturais

1. **ADR-0001 — Estratégia de migração**: o objectivo é paridade
   funcional com o vanilla. Divergir da semântica de `place`
   sem documentar é regressão.

2. **ADR-0029 + espírito da ADR-0026 revisão**: tipos e
   comportamentos seguem o vanilla sem simplificações de
   conveniência. Se o vanilla tem `scope`, o cristalino também
   tem.

3. **Sem `unsafe`**.

4. **Compatibilidade com testes existentes do Passo 82**: os 4
   testes L3 e os testes de Grid do Passo 83 que usam `place` ou
   `align` **devem continuar a passar** ou ser explicitamente
   actualizados com justificação. Se um teste precisa de mudar,
   é porque o comportamento que ele validava estava incorrecto
   — documentar a correcção.

---

## Tarefa 1 — Diagnóstico extenso

### 1.1 — Estrutura e parâmetros de `place` no vanilla

```bash
# Localizar o ficheiro de place no vanilla
find lab/typst-original -name "place*.rs" -type f 2>/dev/null

# Esperado: lab/typst-original/crates/typst-library/src/layout/place.rs
# ou similar. Adaptar os comandos abaixo ao path real.

# PlaceElem (ou equivalente) — definição e campos
grep -B 2 -A 40 "pub struct Place\|#\[elem\]" \
  lab/typst-original/crates/typst-library/src/layout/place.rs 2>/dev/null \
  | head -80

# Parâmetro scope — se existe, qual o tipo
grep -B 2 -A 15 "scope\|PlacementScope" \
  lab/typst-original/crates/typst-library/src/layout/place.rs 2>/dev/null \
  | head -50

# Valor default de scope (se existir)
grep -B 3 -A 10 "#\[default\]\|default =\|default()" \
  lab/typst-original/crates/typst-library/src/layout/place.rs 2>/dev/null \
  | head -30
```

**Reportar o output completo.** Identificar:

- Se existe `scope` como parâmetro.
- Quais valores pode tomar (`column`, `parent`, `page`, etc.).
- Qual é o padrão (default) se `scope` não for especificado.

### 1.2 — Comportamento de `place` dentro de contextos

```bash
# Como é que place é processado em contexto de flow (pipeline principal)
grep -B 2 -A 30 "fn layout_place\|Content::Place\|PlaceElem::layout" \
  lab/typst-original/crates/typst-library/src/layout/place.rs 2>/dev/null \
  | head -60

# Como é que o layouter trata place dentro de grid
grep -rn "place\|Place" \
  lab/typst-original/crates/typst-library/src/layout/grid.rs 2>/dev/null \
  | head -20

# Qual é o valor real do scope quando place está dentro de uma célula
grep -B 3 -A 10 "scope\|placement_scope\|container_scope" \
  lab/typst-original/crates/typst-library/src/layout/flow.rs 2>/dev/null \
  | head -40
```

**Reportar**: qual é o comportamento observável de
`grid(columns: 2, rect(...), place("bottom-right", rect(...)))` no
vanilla? O `place` ancora à célula, ao grid inteiro, ou à página?

### 1.3 — Estado actual no projecto cristalino

```bash
# Definição actual de Content::Place
grep -B 2 -A 10 "Place {" 01_core/src/entities/content.rs

# Layout de Content::Place
grep -B 3 -A 35 "Content::Place" 01_core/src/rules/layout/mod.rs

# Todas as referências em código de produção
grep -rn "Content::Place\|native_place" 01_core/src/ 03_infra/src/ \
  | grep -v "test" | head -20
```

Confirmar: no código actual, `Content::Place` usa `line_start_x`
para X mas `page_config.margin` para Y — **semi-relativo**. O DEBT-37
pede a correcção do eixo Y.

### 1.4 — Testes que validam o comportamento actual

```bash
# Teste do Passo 82 que valida place
grep -B 3 -A 40 "fn place_nao_altera_cursor_y" 03_infra/src/integration_tests.rs

# Outros testes que usam place
grep -rn "#place\|place(" 03_infra/src/ 01_core/src/ | head -20
```

Identificar:
- Quantos testes usam `place`.
- O que cada teste valida exactamente (coordenadas X/Y esperadas,
  ausência de deslocamento do cursor, etc.).

Para cada teste, classificar se a alteração proposta (ancoragem à
célula quando dentro de Grid) afecta o comportamento validado:

- **Teste neutro**: testa `place` fora de grid; comportamento
  inalterado.
- **Teste afectado**: testa `place` dentro de grid; precisa de
  actualização.

### 1.5 — Infra do Passo 83 disponível para reutilização

```bash
# cell_available_h — campo do Layouter do Passo 83
grep -n "cell_available_h\|cell_x\|row_start_y\|is_height_unconstrained" \
  01_core/src/rules/layout/mod.rs
```

Confirmar:
- `cell_available_h: Option<f64>` existe no Layouter.
- Como é definido e restaurado (padrão save/restore dentro do braço
  Grid).

Se só existe `cell_available_h`, pode ser necessário adicionar
`cell_origin_x` e `cell_origin_y` para `Place` ancorar correctamente
ao canto superior esquerdo da célula.

---

## Tarefa 1.5 — Classificação e decisão

Com base no diagnóstico, classificar:

### Cenário A — Vanilla tem `scope` com default `"column"` (ou equivalente a "célula/parent")

**Sinais**:
- `scope` existe como parâmetro.
- Default é `"column"` ou `"parent"` — ancora ao pai imediato.
- Dentro de grid, `place` sem `scope` explícito ancora à célula.

**Implicações**:
- O cristalino precisa de adicionar `scope: PlaceScope` a
  `Content::Place`.
- `PlaceScope` é enum (`Column`, `Parent`, `Page`, ou equivalente
  ao vanilla).
- Propagação via `cell_available_h`/`cell_origin_x`/`cell_origin_y`
  do Layouter.
- O comportamento padrão muda: teste `place_nao_altera_cursor_y`
  pode precisar de actualização.

### Cenário B — Vanilla sem `scope`, comportamento fixo "ancora ao pai se houver"

**Sinais**:
- Não há parâmetro `scope`.
- Comportamento é: se há contexto de célula/bloco pai, ancora a
  ele; caso contrário, ancora à página.

**Implicações**:
- `Content::Place` não precisa de novo campo.
- Só a lógica do Layouter muda: ler `cell_available_h` quando
  disponível.
- Mais simples que A.

### Cenário C — Vanilla ancora à página por padrão; `scope` é opt-in

**Sinais**:
- Vanilla ancora à página por padrão, mesmo dentro de grid.
- `scope` existe mas o default é `"page"` ou não-pai.

**Implicações**:
- DEBT-37 **descrito incorrectamente**. O comportamento actual do
  cristalino (`Y` relativo à página) já está correcto por paridade
  vanilla.
- O passo muda radicalmente: adicionar `scope` como opção, mas
  **não** alterar comportamento padrão.
- DEBT-37 pode ser encerrado com "comportamento padrão correcto;
  `scope` opcional adicionado".

### Cenário D — Diagnóstico inconclusivo

**Sinais**:
- Path `lab/typst-original/.../place.rs` não encontrado com o grep
  proposto.
- Definição de `place` usa macros (`#[elem]`) que escondem a
  estrutura.
- Comportamento depende de código que não está na crate
  `typst-library` mas sim noutra.

**Implicações**:
- Reportar ao utilizador o que foi encontrado e o que está ambíguo.
- Não avançar para implementação. Pode ser necessário abrir um
  passo de investigação mais profunda ou consultar documentação
  externa do Typst.

---

## Tarefa 2A — Implementação (cenário A)

### 2A.1 — Definir `PlaceScope` em `layout_types.rs`

```rust
/// Escopo de ancoragem de um Content::Place.
///
/// Corresponde ao parâmetro `scope` do vanilla Typst.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaceScope {
    /// Ancora ao contentor imediato (célula de grid, coluna).
    /// Default no vanilla dentro de contextos aninhados.
    Column,

    /// Ancora ao bloco pai (container próximo).
    Parent,

    /// Ancora à página.
    Page,
}

impl Default for PlaceScope {
    fn default() -> Self {
        // Adaptar ao default confirmado no diagnóstico.
        PlaceScope::Column
    }
}
```

### 2A.2 — Actualizar `Content::Place`

```rust
Place {
    alignment: Align2D,
    dx:        f64,
    dy:        f64,
    scope:     PlaceScope,  // novo campo
    body:      Box<Content>,
},
```

Cascata: todos os `match` exaustivos sobre `Content::Place`
precisam de actualização. O compilador Rust sinaliza cada um.

### 2A.3 — Actualizar `native_place`

```rust
pub fn native_place(_ctx: &mut EvalContext, args: &Args)
    -> Result<Value, String>
{
    // ... extracção existente de alignment, dx, dy, body ...

    let scope = args.named::<Value>("scope")
        .and_then(|v| match v {
            Value::Str(s) => match s.as_str() {
                "column" => Some(PlaceScope::Column),
                "parent" => Some(PlaceScope::Parent),
                "page"   => Some(PlaceScope::Page),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or_default();

    Ok(Value::Content(Content::Place {
        alignment, dx, dy, scope, body: Box::new(body),
    }))
}
```

### 2A.4 — Propagação no Layouter

Adicionar campos ao Layouter (se ainda não existem):

```rust
pub struct Layouter<'a> {
    // ... campos existentes ...
    pub cell_available_h: Option<f64>,  // Passo 83
    pub cell_origin_x:    Option<f64>,  // novo
    pub cell_origin_y:    Option<f64>,  // novo
}
```

No braço `Content::Grid`, antes de layoutar cada célula:

```rust
let prev_cell_origin_x = self.cell_origin_x;
let prev_cell_origin_y = self.cell_origin_y;
self.cell_origin_x = Some(cell_x);
self.cell_origin_y = Some(cell_y);
// ... layout do item ...
self.cell_origin_x = prev_cell_origin_x;
self.cell_origin_y = prev_cell_origin_y;
```

No braço `Content::Place`, consultar o scope:

```rust
Content::Place { alignment, dx, dy, scope, body } => {
    let sub_frame = self.layout_sub_frame_with_width(body, self.available_width());

    // Resolver origem e área com base no scope.
    let (origin_x, origin_y, avail_w, avail_h) = match scope {
        PlaceScope::Column | PlaceScope::Parent => {
            // Dentro de célula de grid: usar cell_*.
            match (self.cell_origin_x, self.cell_origin_y, self.cell_available_h) {
                (Some(cx), Some(cy), Some(ch)) => (cx, cy, /* cell_width */, ch),
                _ => (
                    // Fora de célula: cair para line_start_x/margem.
                    self.line_start_x.0,
                    self.page_config.margin,
                    self.available_width(),
                    self.available_height(),
                ),
            }
        },
        PlaceScope::Page => (
            self.page_config.margin,
            self.page_config.margin,
            self.available_width(),
            self.available_height(),
        ),
    };

    let (base_x, base_y) = self.resolve_alignment(
        *alignment,
        sub_frame.width.0,
        sub_frame.height.0,
        avail_w,
        avail_h,
        origin_x,
        origin_y,
    );

    // ... resto igual ao Passo 82 ...
}
```

**Nota sobre `cell_width`**: se o Layouter não tem campo para a
largura da célula actual, adicionar `cell_origin_w: Option<f64>`
junto com `cell_origin_x` e `cell_origin_y`.

### 2A.5 — Testes

Novo teste:

```rust
#[test]
fn place_ancora_a_celula_quando_dentro_de_grid() {
    // grid com place("bottom-right") dentro de uma célula.
    // Esperado: rect ancora ao canto inferior direito da CÉLULA,
    // não da página.
    // ...
}
```

Teste existente `place_nao_altera_cursor_y` — **avaliar
criticamente**. Pode precisar de actualização se validava o
comportamento pré-scope. Se validava coordenadas da página e o
default passa a ser `Column`, quebrou — documentar a actualização.

### 2A.6 — Encerrar DEBT-37

```markdown
## DEBT-37 — Place relativo ao contentor pai — **ENCERRADO (Passo 84.6)** ✓

**Registado no Passo 82. Resolvido no Passo 84.6.**

`Content::Place` recebeu campo `scope: PlaceScope` com variantes
`Column`, `Parent`, `Page`. Default é `Column` (paridade com
vanilla). Layouter propaga `cell_origin_x`, `cell_origin_y`,
`cell_origin_w` dentro do braço Grid. `place` dentro de célula
ancora à célula; `place` fora de célula ancora à página.

Sintaxe: `place("bottom-right", scope: "column", ...)`.
Sem `scope` explícito, usa o default `Column`.
```

---

## Tarefa 2B — Implementação (cenário B)

Similar a 2A, mas **sem** `PlaceScope`:

- `Content::Place` não ganha campo novo.
- `native_place` não aceita `scope` como argumento.
- Layouter apenas verifica `cell_*` e cai para margem da página se
  `None`.

Mais simples, menos expressivo. Se o vanilla não tem `scope`, o
cristalino também não — respeitar paridade.

### 2B.1 — Alteração mínima no Layouter

```rust
Content::Place { alignment, dx, dy, body } => {
    let sub_frame = self.layout_sub_frame_with_width(body, self.available_width());

    // Usar cell_* se disponível, senão cair para margem.
    let (origin_x, origin_y, avail_w, avail_h) =
        match (self.cell_origin_x, self.cell_origin_y, self.cell_origin_w, self.cell_available_h) {
            (Some(cx), Some(cy), Some(cw), Some(ch)) => (cx, cy, cw, ch),
            _ => (
                self.line_start_x.0,
                self.page_config.margin,
                self.available_width(),
                self.available_height(),
            ),
        };

    // ... resolve_alignment e emissão como no Passo 82 ...
}
```

### 2B.2 — Testes

Mesma estrutura do cenário A — teste que valida ancoragem à célula.
O teste antigo `place_nao_altera_cursor_y` continua a validar
ancoragem à página **porque está fora de grid**.

---

## Tarefa 2C — Implementação (cenário C)

Se o vanilla ancora à página por padrão, o cristalino não precisa
de alterar comportamento. Apenas adicionar opt-in:

### 2C.1 — Documentar que DEBT-37 está mal descrito

Actualizar a entrada de DEBT-37 no DEBT.md **antes de implementar**
qualquer código. Novo texto:

```markdown
## DEBT-37 — Place relativo ao contentor pai — reclassificado (Passo 84.6)

Diagnóstico no Passo 84.6 revelou que o vanilla Typst ancora
`place` à página por padrão, mesmo dentro de grid. O comportamento
actual do cristalino (Y relativo à margem da página) está correcto
por paridade.

A "resolução" originalmente proposta para este DEBT (ancorar ao
pai) seria **divergência do vanilla**, não paridade. O DEBT está
mal descrito.

Reclassificação: adicionar `scope: PlaceScope` como parâmetro
opt-in — `place("bottom-right", scope: "column", ...)` ancora
à célula; `scope: "page"` (default) ancora à página.
```

### 2C.2 — Implementação opt-in

Como 2A, mas com `PlaceScope::Page` como default. Comportamento
sem `scope` explícito é idêntico ao Passo 82. Teste antigo não é
afectado. Teste novo valida `scope: "column"`.

### 2C.3 — Encerrar DEBT-37

```markdown
## DEBT-37 — Place relativo ao contentor pai — **ENCERRADO (Passo 84.6)** ✓

**Registado no Passo 82. Reclassificado e resolvido no Passo 84.6.**

Diagnóstico revelou que o vanilla ancora place à página por padrão.
`Content::Place` recebeu campo opcional `scope: PlaceScope`;
default `Page` preserva comportamento do Passo 82. `scope: "column"`
activo ancoragem à célula.

Nenhum teste existente afectado.
```

---

## Tarefa 3 — Verificação

```bash
# Testes
cargo test

# Linter
crystalline-lint .

# Grep de confirmação
grep -n "cell_origin_x\|cell_origin_y\|PlaceScope" 01_core/src/
```

---

## Critérios de conclusão

- [ ] Tarefa 1 (diagnóstico extenso) executada e reportada **completa**
  antes de qualquer edição.
- [ ] Cenário classificado (A, B, C ou D) e **confirmado pelo
  utilizador**.
- [ ] Se cenário D, nenhum código alterado; passo paralisado para
  investigação adicional.
- [ ] Se cenário A ou B ou C implementado: mudanças consistentes com
  o vanilla; compatibilidade com testes existentes preservada (ou
  testes actualizados com justificação explícita).
- [ ] Nenhum uso de `unsafe`.
- [ ] `cargo test` — mesmo número de testes ou mais (teste novo).
- [ ] `crystalline-lint .` zero violations.
- [ ] DEBT-37 movido para Secção 2 com nota de resolução, OU
  reclassificado e resolvido no mesmo movimento.

---

## Ao terminar, reportar

Bloco 1 — Diagnóstico (extenso):
- Estrutura de `PlaceElem` no vanilla.
- Parâmetros (incluindo `scope` se existir), valores possíveis,
  default.
- Comportamento observável dentro de grid vs fora de grid.
- Código actual do cristalino: campos de `Content::Place`, uso de
  `line_start_x` e `page_config.margin`.
- Testes actuais que usam `place` — efeito esperado da mudança em
  cada um.
- Infra disponível do Passo 83 (`cell_available_h`, etc.).
- **Cenário classificado** com justificação explícita.

Bloco 2 — Implementação (se cenário A, B ou C):
- Ficheiros alterados.
- Campos novos (se aplicável).
- Testes novos e testes actualizados.

Bloco 3 — Verificação:
- Número total de testes.
- Resultado do linter.
- DEBT counts após o passo.

**Go/No-Go para o Passo 84.7** (auditoria dos ADRs):

- **GO — DEBT-37 resolvido ou reclassificado**: diagnóstico
  completo, cenário confirmado, implementação consistente com
  vanilla.
- **NO-GO — cenário D confirmado (diagnóstico inconclusivo)**:
  passo não avança. Abrir passo de investigação antes de tentar
  implementar.
- **NO-GO — cenário implementado sem confirmação do utilizador**:
  se Claude Code avançou para Tarefa 2 sem apresentar o diagnóstico
  completo e receber confirmação, reverter.

---

## Nota de planeamento

DEBT-37 é o último DEBT classificado como MÉDIO segundo o relatório
do Passo 83.5. Após este passo, os restantes em Secção 1 do DEBT.md
serão:

- **FÁCIL**: DEBT-9 (cobertura de paridade — tracking contínuo).
- **DIFÍCEIS com ADR prévio**: DEBT-8 restante (OpenType MATH),
  DEBT-34d (min-content/max-content), DEBT-34e (colspan/rowspan),
  DEBT-33 (AABB de Bézier).
- **FORA DE ESCOPO**: DEBT-2 (comemo), DEBT-35b (cache
  preventivo), DEBT-39 (`active_guards`).

O Passo 84.7 (auditoria dos ADRs) é o próximo após este. Depois do
84.7, vai haver um ponto de descanso natural — os DEBTs restantes
requerem decisões arquitecturais novas, não são materialização de
decisões já tomadas.
