# Passo 156I — stack (Layout Fase 2 sub-passo 3; último Fase 2)

**Série**: 156I (passo **substantivo escopo M**;
materialização Fase 2 Layout, terceira sub-fase). **Sétimo
passo consecutivo** da sequência granular Layout
(P156C+D+E+F+G+H+I). **Último passo Fase 2 — atinge target
72% Layout** declarado em ADR-0061.
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → P156C (pad+hide) → P156D (h+v) → P156E
(pagebreak) → P156F (skew) → P156G (block) → P156H (box) →
**P156I (stack)**.

**Precondição**: Passo 156H encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1271 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 67% (12/18 implementado puro pós-P156H); cobertura
user-facing total 59%.

**Numeração**: P156I segue P156H na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations). **Após P156I**: numeração reservada P157+
sem letras (volta à convenção numérica padrão).

**Natureza**: passo **substantivo escopo M** (1 feature
container compositivo com 3 atributos; ~15-22 testes
adicionados estimados; **possível tipo Dir novo**; sem
crates novas; sem ADRs novas; **possível anotação cumulativa
ADR-0061 após este passo per decisão humana**; sem DEBTs
novos esperados).

**Decisão metodológica P156I** (per resposta humana
2026-04-25): **inventariar primeiro 156I.1 curto** (4
sub-investigações focadas) e **decidir spacing consoante
viabilidade descoberta**. Decisão arquitectural genérica
(variant rico) já estabelecida por P156G/H; decisão local
sobre `spacing` pendente.

Razão: stack tem mais especificidades que Box (Vec<Content>,
Dir novo, spacing material). Modelo Block/Boxed reaplicável
mas com adaptações. Inventário identifica especificidades
antes de comprometer.

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita variants
  novos.
- **ADR-0033**: paridade funcional para stack.
- **ADR-0036**: atomização — consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded — stack
  cumprido com aproximação aceite (spacing semantic real
  pode adiar consoante 156I.1).
- **ADR-0061** (PROPOSTO): plano de Layout. **Sétima
  aplicação; última Fase 2; atinge target 72% declarado**.
  Após este passo, ADR-0061 é candidata a anotação
  cumulativa per decisão humana.

---

## Contexto

P156H fechou box (67% Layout). Próximo natural fecha
Fase 2 com **stack**, único container compositivo
(Vec<Content> em vez de body único).

**`stack(dir, spacing, ..children)` em vanilla**:
- **`dir: Dir`**: direcção de empilhamento (LTR, RTL, TTB,
  BTT). Default `TTB` (top-to-bottom).
- **`spacing: Smart<Rel<Length>>`**: espaço entre children.
  Default smart (computed em runtime).
- **`children: Vec<Content>`**: variádicos posicionais.
- Semantic: empilha children visualmente na direcção
  especificada com spacing entre eles.

**Hipóteses a confirmar empiricamente em 156I.1**:

- **`Dir` enum existe?** Provável: não. Análogo a Parity
  (P156E) — candidato a criar em `entities/dir.rs`.
- **Vec<Content> em outros variants existe?** Provável:
  `Content::Sequence(Vec<Content>)` ou similar (P22 ou
  posterior). Reusar pattern.
- **spacing layouter mecânica**: vanilla colapsa spacing
  per direcção. Cristalino layouter pode aplicar trivialmente
  como avanço cursor entre children. Verificar.
- **`Smart<Rel<Length>>` simplificação**: candidato a
  `Option<Length>` (consistente com padrão Smart→Option
  N=4). None == espaço default; Some(L) == explícito.

---

## Objectivo

Ao fim do passo:

1. **Inventário rigoroso curto** em 156I.1 (4 sub-
   investigações).

2. **Decisão local sobre `spacing`** em 156I.2 consoante
   descoberta:
   - **Se spacing trivial em layouter**: implementar real.
   - **Se spacing exige refactor**: scope-out documentado
     análogo a `breakable` (P156G).

3. **Possível tipo `Dir`** criado em
   `01_core/src/entities/dir.rs`:
   ```rust
   pub enum Dir {
       LTR,  // left-to-right
       RTL,  // right-to-left
       TTB,  // top-to-bottom (default stack)
       BTT,  // bottom-to-top
   }

   impl Dir {
       pub fn is_horizontal(self) -> bool { ... }
       pub fn is_vertical(self) -> bool { ... }
   }
   ```

4. **Variant `Content::Stack`** adicionado:
   ```rust
   Stack {
       children: Vec<Content>,
       dir:      Dir,
       spacing:  Option<Length>,  // None == default smart
   }
   ```

5. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content` (modelo Block/Boxed
   adaptado para Vec<Content>):
   - `entities/content.rs::is_empty()`: todos os children
     vazios.
   - `entities/content.rs::plain_text()`: concatenar
     plain_text de todos os children.
   - `entities/content.rs::PartialEq::eq`: comparação
     deep Vec.
   - `entities/content.rs::map_content`: mapear cada child.
   - `entities/content.rs::map_text`: idem.
   - `rules/introspect.rs::materialize_time`: recurse em
     cada child.
   - `rules/introspect.rs::walk`: walk cada child em
     ordem.
   - `rules/layout/mod.rs::layout_content`: iterar
     children + spacing entre + dir compliance.
   - `rules/layout/mod.rs::measure_content_constrained`:
     somar dimensões com spacing.

6. **`native_stack`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#stack(dir: ?, spacing: ?, ..children)`.

7. **Layouter stack**: aplica direcção (TTB default;
   horizontal vs vertical); insere spacing entre children
   se trivial; força flush_line antes/depois (structural).

8. **Testes** unit + eval (~15-22 testes adicionados
   estimados):
   - `Dir` enum + métodos.
   - Construtor `Content::Stack`.
   - `is_empty()`, `plain_text()`, `partial_eq`,
     `map_*` cobertura.
   - `eval_stack` defaults (sem args).
   - `eval_stack` com dir explícito.
   - `eval_stack` com spacing.
   - `eval_stack` com children variádicos.
   - `eval_stack` rejeita dir inválido.
   - `eval_stack` rejeita named arg desconhecido.
   - Layouter: stack TTB empilha verticalmente.
   - Layouter: stack LTR empilha horizontalmente.
   - Layouter: spacing entre children (se implementado).
   - **Regression tests** para Block/Boxed/Pad/Hide.

9. **L0 prompts** + hashes propagados:
   - Possível `00_nucleo/prompts/entities/dir.md` (novo).
   - `00_nucleo/prompts/entities/content.md` ganha secção
     "Variant `Content::Stack` — Passo 156I".
   - Hashes recomputados.
   - Headers `@updated`: data execução.

10. **Inventário 148 actualizado**:
    - Tabela A.5 Layout: linha `stack` ausente →
      `implementado`.
    - Cobertura Layout: 12/18 → **13/18 = 72%**.
      **Target atingido**.
    - Tabela A linha "Layout": `12/0/3/3/0=18` →
      `13/0/3/2/0=18`.
    - Total user-facing: 59% → **~60%**.
    - Tabela B Content variants: 50 → **51**.
    - §7 entrada 7: actualizar progresso Layout (P156I
      cumprido; **Fase 2 completa**; restantes 2 entradas
      Fase 3: repeat, columns/colbreak).

11. **README dos ADRs actualizado**:
    - Tabela "Estado por ADR": **anotação cumulativa
      candidata em ADR-0061** per decisão humana
      pós-P156I.
    - Total inalterado (61 ADRs).
    - Entrada nova em "Passos-chave da história dos ADRs"
      para P156I com **fechamento Fase 2** marcado.

12. **ADR-0061 anotação cumulativa** (per decisão humana
    2026-04-25): pós-P156I, anotar progresso Fase 1+2
    cumulativo. **NÃO** promover de PROPOSTO a IMPLEMENTADO
    (Fase 3 ainda não materializada). **Sim** anotar §
    novo "Aplicações cumulativas" listando P156C-I com
    cobertura cumulativa, padrões emergentes, e estado.

13. **Sem DEBTs criados/fechados** (esperado).

14. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156i-relatorio.md`
    com §análise de risco modelo P156F/G/H + **§análise
    cumulativa Fase 1+2** (novo).

Este passo **não**:

- Toca outras features Layout além de stack.
- Implementa Fase 3 (columns, repeat).
- Implementa atributos avançados sem suporte renderer.
- Adiciona show rules.
- Toca série paridade.
- Promove ADR-0061 a IMPLEMENTADO (Fase 3 pendente).

---

## Decisões já tomadas

1. **Variant rico** per padrão estabelecido em P156G+H.
   Sem nova decisão arquitectural genérica.

2. **3 fields**: children Vec<Content>, dir, spacing.

3. **Granularidade**: 1 container compositivo num passo.
   Escopo M.

4. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`.

5. **Assinatura natives**: 5-param canónica.

6. **Inventário 156I.1 curto** (4 sub-investigações).

7. **Decisão local spacing**: pendente em 156I.2.

8. **Atributos avançados scope-out**: vanilla stack tem
   menos atributos que Block; provável zero scope-out além
   de spacing-se-complexo.

9. **Tests adicionados**: alvo 15-22.

10. **ADR-0061 anotação cumulativa pós-P156I**: per decisão
    humana. NÃO promove a IMPLEMENTADO; SIM anota progresso
    cumulativo.

11. **Show rules adiadas**.

## Decisões diferidas (resolvidas neste passo)

12. **Forma exacta de `Dir`**: 4 variantes (LTR/RTL/TTB/BTT)
    canónicas. Decisão final consoante 156I.1 (se vanilla
    expressa diferentemente).

13. **`Dir` em ficheiro próprio vs inline**: ficheiro
    próprio se >40 linhas (com tests + métodos); inline
    se mínimo. **Default**: ficheiro próprio (análogo a
    Parity P156E).

14. **`spacing` semantic**: real se trivial; scope-out se
    complexo. Decisão consoante 156I.1.

15. **`spacing` default**: vanilla usa `Smart` (computed em
    runtime). Cristalino simplifica para `None == zero`
    (mais conservador) ou `None == leading` (mais útil)
    consoante 156I.1.

16. **Children empty Vec**: aceitável (stack vazio é
    semantically válido). Test edge case.

17. **Children single**: aceitável (stack com 1 child é
    layout idêntico ao child). Test edge case.

18. **dir aceito como string** (`"ltr"`, `"rtl"`, etc.) em
    stdlib? Vanilla aceita Symbol. Default: aceitar string;
    helper `extract_dir`.

19. **Children variádicos posicionais**: stack aceita
    `#stack(child1, child2, child3)` sem array literal.
    Implementação: iterar `args.items` após dir/spacing
    parsing.

---

## Escopo

**Dentro**:

- Possível criação de `01_core/src/entities/dir.rs`.
- Modificação de `01_core/src/entities/content.rs`
  (variant novo + arms cobertura).
- Modificação de `01_core/src/entities/mod.rs` (registo
  `pub mod dir`).
- Modificação de `01_core/src/rules/introspect.rs`.
- Modificação de `01_core/src/rules/layout/mod.rs`.
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_stack`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo).
- Tests novos.
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- **ADR-0061 anotação cumulativa**.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Implementação de outras features Layout (Fase 3).
- Show rules.
- Crates externas.
- ADRs novas (ADR-0061 anotação cumulativa NÃO é ADR nova).
- DEBTs novos.
- Promoção ADR-0061 a IMPLEMENTADO.
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156I.1 — Inventário curto pré-decisão

**Reduzido face a P156G** (decisão arquitectural genérica
estabelecida) **mas focado em especificidades de stack**.

**A.1.1 — Verificar `Dir` em entities**:

```bash
ls 01_core/src/entities/dir.rs 2>/dev/null
grep -nE "pub enum Dir|enum Direction" 01_core/src/entities/
grep "Dir::" 01_core/src/   # uso em outros locais
```

Confirmar:
- `Dir` existe? Provável: não.
- Algum tipo direccional similar (Direction, Axis)?

**A.1.2 — Verificar Vec<Content> em outros variants**:

```bash
grep -nE "Vec<Content>|Vec<Box<Content>>" 01_core/src/entities/content.rs
grep -nE "Sequence|Group" 01_core/src/entities/content.rs
```

Documentar:
- `Content::Sequence(Vec<Content>)` existe? Provável: sim
  (P22+ ou similar).
- Pattern para arms quando variant tem Vec<Content>:
  `is_empty` (todos vazios?), `plain_text` (concat),
  `map_content` (mapear cada).
- Reusar.

**A.1.3 — Verificar spacing layouter mecânica**:

```bash
grep -nE "leading|line_height" 01_core/src/rules/layout/
view 01_core/src/rules/layout/cursor.rs   # mecânica vertical
```

Determinar:
- Layouter tem mecânica trivial de "avançar cursor.y por X
  pt"? Sim — VSpace (P156D) confirma.
- spacing em stack TTB == VSpace entre children. Reusar
  `cursor_y += Pt(spacing)` directamente.
- spacing em stack LTR == HSpace entre children. Reusar
  `cursor_x += Pt(spacing)`.
- **Conclusão provável**: spacing implementável trivialmente.

**A.1.4 — Verificar children variádicos em natives**:

```bash
grep -nE "args.items.iter\(\)|args.items\[" 01_core/src/rules/stdlib/
```

Confirmar:
- Outros natives recebem N posicionais? Provável: sim
  (e.g. `text(...)` ou `array(...)`).
- Pattern para iterar `args.items` após named parsing.
- Reusar.

### 156I.2 — Decisão local sobre spacing

Consoante 156I.1.A.1.3:

**Se spacing trivial** (esperado): implementar real.
- `Content::Stack { children, dir, spacing: Option<Length> }`.
- Layouter aplica `cursor_y += Pt(spacing)` entre children
  em TTB; análogo para outras direcções.
- Default `None == zero` (conservador).

**Se spacing exige refactor**: scope-out.
- Armazenar como atributo; layouter ignora.
- Limitação documentada.

### 156I.3 — Criar `Dir` enum

Em `01_core/src/entities/dir.rs` (novo):

```rust
//! @prompt: prompts/entities/dir.md
//! @updated 2026-04-26
//! @prompt-hash <recompute>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    LTR,  // left-to-right
    RTL,  // right-to-left
    TTB,  // top-to-bottom (default stack)
    BTT,  // bottom-to-top
}

impl Dir {
    pub fn is_horizontal(self) -> bool {
        matches!(self, Dir::LTR | Dir::RTL)
    }
    pub fn is_vertical(self) -> bool {
        matches!(self, Dir::TTB | Dir::BTT)
    }
    pub fn is_reverse(self) -> bool {
        matches!(self, Dir::RTL | Dir::BTT)
    }
}

impl Default for Dir {
    fn default() -> Self { Dir::TTB }  // stack default
}

#[cfg(test)]
mod tests { ... }
```

Registar em `entities/mod.rs::pub mod dir;`.

### 156I.4 — Adicionar variant Content::Stack

```rust
pub enum Content {
    // ... 50 variants existentes (Text, Strong, ..., Boxed)
    Stack {
        children: Vec<Content>,
        dir:      Dir,
        spacing:  Option<Length>,
    },
}
```

**Variant count**: 50 → **51** (+1).

### 156I.5 — Cobertura exaustiva de arms

Modelo P156G/H **adaptado para Vec<Content>**:

| Função | Tratamento Stack |
|--------|------------------|
| `Content::is_empty()` | `children.iter().all(|c| c.is_empty())` |
| `Content::plain_text()` | concat plain_text de todos |
| `PartialEq::eq` | comparação 3-fields (Vec deep eq) |
| `Content::map_content` | mapear cada child; preserva dir/spacing |
| `Content::map_text` | idem |
| `materialize_time` | recurse em cada child |
| `walk` | walk cada child em ordem |
| `layout_content` | iterar children + spacing + dir |
| `measure_content_constrained` | somar dimensões com spacing |

### 156I.6 — `native_stack`

Em `stdlib/layout.rs`:

```rust
fn extract_dir(value: &Value) -> SourceResult<Dir> {
    match value {
        Value::Str(s) => match s.as_str() {
            "ltr" => Ok(Dir::LTR),
            "rtl" => Ok(Dir::RTL),
            "ttb" => Ok(Dir::TTB),
            "btt" => Ok(Dir::BTT),
            other => Err(invalid_argument(format!(
                "dir must be \"ltr\"/\"rtl\"/\"ttb\"/\"btt\", got {:?}", other))),
        },
        _ => Err(invalid_argument("dir must be a string")),
    }
}

pub fn native_stack(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    let mut dir: Dir = Dir::default();  // TTB
    let mut spacing: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "dir"     => dir = extract_dir(value)?,
            "spacing" => spacing = Some(extract_length(value)?),
            other => return Err(unexpected_named(other)),
        }
    }

    // Validação spacing negativo: rejeitado.

    // Children variádicos: iterar args.items.
    let children: Vec<Content> = args.items.iter()
        .filter_map(|v| match v {
            Value::Content(c) => Some(c.clone()),
            Value::Str(s) => Some(Content::text(s.as_str())),
            _ => None,  // ou erro — decidir
        })
        .collect();

    Ok(Value::Content(Content::Stack {
        children, dir, spacing,
    }))
}
```

Registo em `make_stdlib` e `stdlib/mod.rs`.

Stdlib funcs: 40 → **41** (+1).

### 156I.7 — Layouter stack

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    Content::Stack { children, dir, spacing } => {
        let font = self.font_size_pt.val();
        let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

        // Stack é STRUCTURAL: força flush_line antes.
        if self.cursor_x.0 > self.line_start_x.0 {
            self.flush_line();
        }

        match dir {
            Dir::TTB => {
                for (i, child) in children.iter().enumerate() {
                    if i > 0 { self.cursor_y += Pt(space_pt); }
                    self.layout_content(child);
                    self.flush_line();
                }
            }
            Dir::BTT => {
                // Layout reverse — mais complexo;
                // implementação simples: layout normal mas children invertidos.
                for (i, child) in children.iter().rev().enumerate() {
                    if i > 0 { self.cursor_y += Pt(space_pt); }
                    self.layout_content(child);
                    self.flush_line();
                }
            }
            Dir::LTR => {
                for (i, child) in children.iter().enumerate() {
                    if i > 0 { self.cursor_x += Pt(space_pt); }
                    self.layout_content(child);
                }
                self.flush_line();
            }
            Dir::RTL => {
                // Análogo a LTR mas reverse.
                for (i, child) in children.iter().rev().enumerate() {
                    if i > 0 { self.cursor_x += Pt(space_pt); }
                    self.layout_content(child);
                }
                self.flush_line();
            }
        }
    }
    // ... fallback
}
```

Em `measure_content_constrained` (modelo Block):

```rust
match content {
    Content::Stack { children, dir, spacing } => {
        let font = self.font_size_pt.val();
        let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

        if children.is_empty() { return (0.0, 0.0); }

        match dir {
            Dir::TTB | Dir::BTT => {
                // dimensão: max width, sum heights + (n-1) * spacing
                let mut max_w = 0.0;
                let mut sum_h = 0.0;
                for child in children {
                    let (w, h) = self.measure_content_constrained(child, max_width);
                    max_w = max_w.max(w);
                    sum_h += h;
                }
                let total_h = sum_h + (children.len() - 1) as f64 * space_pt;
                (max_w, total_h)
            }
            Dir::LTR | Dir::RTL => {
                // dimensão: sum widths + (n-1) * spacing, max height
                let mut sum_w = 0.0;
                let mut max_h = 0.0;
                for child in children {
                    let (w, h) = self.measure_content_constrained(child, max_width);
                    sum_w += w;
                    max_h = max_h.max(h);
                }
                let total_w = sum_w + (children.len() - 1) as f64 * space_pt;
                (total_w, max_h)
            }
        }
    }
    // ... fallback
}
```

### 156I.8 — Tests adicionados (alvo 15-22)

| Ficheiro | Testes |
|----------|--------|
| `entities/dir.rs::tests` | (1) dir_default_e_ttb; (2) is_horizontal_vs_vertical; (3) is_reverse |
| `entities/content.rs::tests` | (4) stack_constructor_default; (5) stack_constructor_explicit_atributos; (6) stack_is_empty_se_todos_children_vazios; (7) stack_plain_text_concatena_children; (8) stack_partial_eq |
| `stdlib/mod.rs::tests` | (9) native_stack_defaults_sem_args; (10) native_stack_aceita_dir_ltr; (11) native_stack_aceita_spacing; (12) native_stack_com_children_variadicos; (13) native_stack_rejeita_dir_invalido; (14) native_stack_rejeita_spacing_negativo; (15) native_stack_rejeita_named_arg_desconhecido; (16) native_stack_children_vazios_aceito; (17) native_block_box_pad_hide_continuam_a_funcionar_apos_p156i — **regression** |
| `layout/tests.rs` | (18) layout_stack_ttb_empilha_verticalmente; (19) layout_stack_ltr_empilha_horizontalmente; (20) layout_stack_spacing_avanca_cursor_entre_children |

**Total**: ~20 tests novos. Tests cumulativos: **1271 →
~1291**.

### 156I.9 — L0 prompts + hashes

**Possível L0 novo**: `00_nucleo/prompts/entities/dir.md`
(análogo a parity.md de P156E).

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Variant `Content::Stack` — Passo 156I
(ADR-0061 Fase 2, sub-passo 3)":

```markdown
## Variant Stack (Passo 156I)

`Content::Stack { children, dir, spacing }`:
- container compositivo (Vec<Content>);
- direcção via `Dir` enum (LTR/RTL/TTB/BTT; default TTB);
- spacing entre children;
- structural (força flush_line antes/depois).

Adaptação do template Block/Boxed para Vec<Content>:
arms iteram children em vez de recurse body único.

**Último sub-passo Fase 2 ADR-0061. Cobertura Layout 67% →
72% (target atingido).**
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

### 156I.10 — Inventário 148 actualizado

**Tabela A.5 Layout**: `stack` ausente → **implementado**.

**Tabela A linha "Layout"**: `12/0/3/3/0=18` →
**`13/0/3/2/0=18`**. Cobertura Layout: 67% → **72%**.
**Target Fase 1+2 atingido**.

**Total user-facing**: `62/21/22/34/2=141` →
**`63/21/22/33/2=141`**. Cobertura: 59% → **~60%**.

**Tabela B Content variants**: 50 → **51**.

**§7 entrada 7**: actualizar progresso Layout — **Fase 2
completa**; restantes 2 entradas Fase 3 (repeat,
columns/colbreak); marcar P156I como fechamento da série
P156C-I.

### 156I.11 — README ADRs actualizado

- Tabela "Estado por ADR": ADR-0061 mantém-se PROPOSTO
  (Fase 3 pendente). **Anotação cumulativa adicionada
  como sub-secção do próprio ADR-0061** (sem promoção
  formal).
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova marcando **fechamento
  série P156C-I**:
  ```
  P156I: aplicação sétima de ADR-0061 — stack container
  compositivo. Cobertura Layout 67% → 72%. Target Fase 1+2
  atingido. Sequência granular P156C-I completa (7 passos
  consecutivos sem reformulação).
  ```

### 156I.12 — ADR-0061 anotação cumulativa

Em `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`:

Adicionar secção **§Aplicações cumulativas** (mantém status
PROPOSTO; Fase 3 pendente):

```markdown
## Aplicações cumulativas (pós-P156I)

ADR-0061 PROPOSTO em P156B (2026-04-25). Fase 1+2
materializadas em sequência granular P156C-I:

| Passo | Feature(s) | Slope | Cobertura cumulativa |
|-------|-----------|-------|----------------------|
| P156C | pad + hide | +11% | 22% → 33% |
| P156D | h + v | +11% | 33% → 44% |
| P156E | pagebreak | +6% | 44% → 50% |
| P156F | skew | +6% | 50% → 56% |
| P156G | block | +5% | 56% → 61% |
| P156H | box | +6% | 61% → 67% |
| P156I | stack | +5% | 67% → **72%** (target) |

Total: +34 pontos percentuais Layout em 7 passos
consecutivos. Zero reformulações mid-passo. Padrões
metodológicos consolidados:
- "Inventariar primeiro" (N=4 aplicações).
- "Smart<T> → Option<T>" (N=4-5 aplicações).
- "Reuso de template Block" (N=2-3 aplicações).
- "§análise de risco no relatório" (N=4 aplicações).

Status: **PROPOSTO** (Fase 3 — repeat, columns/colbreak —
pendente per DEBT-56). Promoção a IMPLEMENTADO requer Fase 3
ou decisão humana de scope-out.
```

### 156I.13 — Relatório do passo

Ficheiro: `00_nucleo/materialization/typst-passo-156i-relatorio.md`.

Secções:
1. Sumário executivo (incluindo **target 72% atingido**).
2. Inventário 156I.1.
3. Decisão local spacing 156I.2.
4. `Dir` enum.
5. Variant `Content::Stack` — forma final.
6. Cobertura exaustiva de arms.
7. `native_stack` + `extract_dir`.
8. Layouter stack — diff (4 direcções).
9. Tests adicionados (incluindo regression).
10. L0 prompts + hashes.
11. Inventário 148 actualizado.
12. README ADRs + ADR-0061 anotação cumulativa.
13. Próximo passo (decisão humana sobre Fase 3 ou outra
    prioridade).
14. Limitações registadas.
15. Verificação final.
16. **§análise de risco de regressão** (modelo P156F/G/H).
17. **§análise cumulativa Fase 1+2** (novo neste passo —
    fechamento de série).

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1291 passed
   (1271 → +20); zero falhas.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes.
5. ✅ `Content::Stack` variant em produção (50 → 51).
6. ✅ `Dir` enum em produção (se ficheiro próprio).
7. ✅ Stdlib `#stack(...)` invocável (40 → 41 funcs).
8. ✅ Cobertura arms exaustiva (9 sítios actualizados;
   adaptados para Vec<Content>).
9. ✅ Layouter stack TTB/BTT/LTR/RTL aplicam direcção
   correctamente.
10. ✅ Inventário 148 reflecte cobertura **72% Layout**
    (target atingido).
11. ✅ README ADRs entrada P156I.
12. ✅ **ADR-0061 anotação cumulativa** adicionada (sem
    promoção a IMPLEMENTADO).
13. ✅ Sem ADR criada / revogada / revisada formalmente.
14. ✅ Sem DEBT criado / fechado.
15. ✅ ADR-0060 inalterada.
16. ✅ **Sem regressão** em containers existentes (Block,
    Boxed, Pad, Hide) — regression test verifica.
17. ✅ Sem regressão geral.
18. ✅ Relatório do passo escrito (com §análise de risco +
    §análise cumulativa Fase 1+2).

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | Inventário 156I.1 produzido | ✅ |
| 2 | Decisão local spacing justificada | ✅ |
| 3 | `Dir` enum compila + tests passam | ✅ |
| 4 | `Content::Stack` compila + tests passam | ✅ |
| 5 | Stdlib `#stack(dir, spacing, ..children)` invocável | ✅ |
| 6 | Layouter stack aplica 4 direcções | ✅ |
| 7 | Inventário 148 reflecte cobertura **72% Layout** | ✅ |
| 8 | **Target Fase 1+2 atingido** | ✅ |
| 9 | ADR-0061 anotação cumulativa adicionada | ✅ |
| 10 | Sem regressão | ✅ |
| 11 | §análise cumulativa Fase 1+2 no relatório | ✅ |
| 12 | Próximo passo (decisão humana pós-72%) tem âncora | ✅ |
| 13 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

### Cenários gerais

- **156I.1 revela `Dir` ou `Direction` já existente**:
  improvável mas verificar. Se sim, reusar; sem criar novo
  tipo.

- **`Content::Sequence` já existe e cobre stack**: improvável
  porque Sequence não tem dir/spacing. Mas se descoberto,
  considerar derivar Stack de Sequence ou simplificar.

- **`spacing` exige refactor**: scope-out documentado
  análogo a `breakable` (P156G). Aceitável.

- **Vec<Content> em arms expõe pattern não-anticipado**:
  (e.g. infinite recursion em walk). Mitigar com tests.

- **Volume tests excede 22**: aceitável; ajustar relatório.

- **Volume tests inferior a 15**: investigar.

- **Layouter BTT/RTL comportam diferente do esperado**:
  reverse children pode ter implicações sobre cursor reset.
  Tests E2E cobrem. Se complexo, BTT/RTL scope-out e
  documentar.

### Cenários específicos a Stack

- **Stack vazio (children empty)**: aceitável; (0, 0)
  dimensões; layouter no-op.

- **Stack com 1 child**: layout idêntico ao child standalone.
  Aceitável.

- **Spacing default `None`**: zero (conservador) vs leading
  (útil). Decisão consoante 156I.1. Documentar.

- **Stack aninhado** (stack de stacks): suportado
  estruturalmente; tests cobrem.

- **Dir aceito como Symbol em vez de String** em vanilla:
  cristalino apenas string per padrão P156E.

---

## Notas operacionais

- **Padrão "passos granulares" — sétima aplicação
  consecutiva**. P156C+D+E+F+G+H+I. **N=7 aplicações** sem
  reformulação. **Fechamento de série**.

- **Padrão "inventariar primeiro" — quarta aplicação**:
  P156F (defensivo), P156G (deliberado), P156H (curto),
  P156I (curto focado). **N=4 aplicações** — patamar
  empírico forte.

- **Padrão "Smart<T> → Option<T>" — quinta aplicação**:
  P156E (Parity), P156F (Angle), P156G (Block.width),
  P156H (Box.width), P156I (Stack.spacing). **N=5
  aplicações** — patamar empírico muito forte.

- **ADR-0061 anotação cumulativa** marca fechamento de
  fase mas **NÃO promove**. Promoção a IMPLEMENTADO requer
  Fase 3 ou decisão humana de scope-out.

- **Variants count**: 50 → **51** (+1, Stack). Após
  Fase 3 hipotética: 53.

- **Stdlib funcs**: 40 → **41** (+1). Após Fase 3
  hipotética: 43.

- **Pós-156I**:
  - 10 features Layout implementadas total (pad, hide, h,
    v, pagebreak, skew, block, box, stack + align/move/
    rotate/scale via Transform unificado).
  - Cobertura Layout: 67% → **72%** (target atingido).
  - Cobertura user-facing total: 59% → **~60%**.
  - **Próximo**: decisão humana pós-72%:
    - Fase 3 Layout (columns L+ via DEBT-56; repeat).
    - Footnote area (sub-fase prioritária ADR-0061).
    - P157 Model Fase 2 table.
    - Outras prioridades.

- **Slope cumulativo final**:
  - P156C +11% (4→6/18).
  - P156D +11% (6→8/18).
  - P156E +6% (8→9/18).
  - P156F +6% (9→10/18).
  - P156G +5% (10→11/18).
  - P156H +6% (11→12/18).
  - **P156I +5% (12→13/18)**.
  - **Total Fase 1+2**: 22%→72% = +50% em 7 passos.
  - Restantes 28 pontos para 100% em 2 passos Fase 3
    (repeat = +6%; columns = +22% L+).

- **Slope decreasing pattern**: +11%, +11%, +6%, +6%, +5%,
  +6%, +5%. Estável em ~+5-6% pós-halfway. Coerente com
  hipótese "features remanescentes mais complexas".

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`** (após P156H):
  `5bb6e3d2`. Após P156I: novo (a recomputar).

- **Padrões emergentes consolidados ao fechar série**:
  1. **Granularidade 1-2 features/passo**: N=7 aplicações
     consecutivas sem reformulação.
  2. **Inventariar primeiro**: N=4 aplicações.
  3. **Smart<T> → Option<T>**: N=5 aplicações — candidato
     a registo formal em ADR meta.
  4. **§análise de risco no relatório**: N=4 aplicações.
  5. **Reuso de template containers**: N=2-3 aplicações
     (Block → Boxed → Stack).
  6. **Antecipar especificidades técnicas**: N=2 aplicações
     (Boxed naming P156H; Vec<Content> arms P156I).

- **Pausa natural após P156I**: target atingido; padrões
  consolidados; ADR-0061 anotada cumulativamente. Decisão
  humana sobre próxima direcção tem máxima informação.
