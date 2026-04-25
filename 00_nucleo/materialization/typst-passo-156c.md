# Passo 156C — pad + hide (Layout Fase 1 sub-fase 1)

**Série**: 156C (passo **substantivo escopo S**;
materialização Fase 1 Layout, primeira sub-fase granular).
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → **P156C (sub-fase 1 Layout)**.

**Precondição**: Passo 156B encerrado; ADR-0061 PROPOSTO
(Layout roadmap); 1145 tests; 61 ADRs (ADR-0061 nova);
14 DEBTs abertos (DEBT-56 novo); cobertura Layout 22%
(4/18 implementado puro; per inventário 148 §A.5
actualizado em P156B); cobertura user-facing total 53%.

**Numeração**: P156C segue P156A (historiograma) e P156B
(diagnóstico Layout) na convenção de letras consecutivas.
**Não conflita** com P157 (Model Fase 2 table foundations,
renumerado por P156B).

**Natureza**: passo **substantivo escopo S** (2 features
triviais; ~10-18 testes adicionados estimados; sem
crates novas; sem ADRs novas; sem DEBTs novos esperados).

**Decisão arquitectural P156C** (per resposta humana
2026-04-25): **pad e hide como variants novos** em
`Content` enum:

- `Content::Pad { body, padding }` — body é container;
  padding é margens.
- `Content::Hide { body }` — body é container; sem
  atributos visuais.

Razão: vanilla expõe ambos como `#[elem]` proper
(`PadElem`, `HideElem` em
`lab/typst-original/crates/typst-library/src/layout/`).
Cristalino segue vanilla — ambos são Content, não Style.
Semantic clara: pad é structural (afecta layout); hide é
visual (não rende mas afecta cascata Content).

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita
  variants novos via decisão deste passo.
- **ADR-0033**: paridade funcional para pad e hide.
- **ADR-0036**: atomização — cada feature consumer
  explícito.
- **ADR-0037**: coesão por domínio — Layout permanece
  em `rules/layout/`.
- **ADR-0054**: perfil observacional graded — pad
  cumprido com aproximação aceite.
- **ADR-0061** (PROPOSTO): plano de Layout Fase X. Este
  passo aplica-o pela primeira vez. ADR-0061 mantém-se
  PROPOSTO até P156I (Layout 72%).

---

## Contexto

P156B identificou 18 entradas Layout, 4 implementadas
puro (22% cobertura). Diagnóstico priorizou pad+hide como
features Fase 1 triviais (S cada; alto valor —
fundamentais para containers user-facing).

**Pad** em vanilla:
- Atributos: `left`, `right`, `top`, `bottom`, `x`, `y`,
  `rest` (sides individual + atalhos `x`/`y`/`rest`).
- Structural: cria container que reduz área de layout
  do `body` proporcionalmente ao padding.
- `#show pad: ...` aceita customização (scope-out neste
  passo).

**Hide** em vanilla:
- Atributos: `body` apenas.
- Semantic: layout-aware (calcula tamanho do body) mas
  não rende. Útil para placeholders / equilíbrio.
- `#show hide: ...` aceita customização (scope-out neste
  passo).

**Hipóteses a confirmar empiricamente** (não compromisso):

- `padding` em cristalino pode ser `Sides<Length>`
  (per inventário 148 §A.5; tipo já existe ou candidato
  a criar).
- `body` é `Box<Content>` (consistente com Strong, Emph,
  outros containers existentes em Content).
- `Content::Pad { body, padding: Sides<Length> }` é
  forma final esperada.
- `Content::Hide { body: Box<Content> }` é forma final.
- Layouter para `Content::Pad`: ajusta cursor + área antes
  de processar body.
- Layouter para `Content::Hide`: percorre body para
  calcular dimensões + emite zero items.

---

## Objectivo

Ao fim do passo:

1. **Variant `Content::Pad { body, padding }`** adicionado
   em `01_core/src/entities/content.rs`.

2. **Variant `Content::Hide { body }`** adicionado em
   `01_core/src/entities/content.rs`.

3. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content`:
   - `entities/content.rs::is_empty()`.
   - `entities/content.rs::plain_text()`.
   - `entities/content.rs::PartialEq::eq`.
   - `entities/content.rs::map_content`.
   - `entities/content.rs::map_text`.
   - `rules/introspect.rs::materialize_time`.
   - `rules/introspect.rs::walk`.
   - `rules/layout/mod.rs::layout_content`.
   - `rules/layout/mod.rs::measure_content_constrained`.

4. **`native_pad`** em stdlib expondo
   `#pad(body, left: ?, right: ?, top: ?, bottom: ?,
   x: ?, y: ?, rest: ?)`.

5. **`native_hide`** em stdlib expondo `#hide(body)`.

6. **Tipo `Sides<Length>`** verificado/criado em
   `01_core/src/entities/`. Provavelmente já existe;
   se não, criar como struct genérica
   `pub struct Sides<T> { left: T, right: T, top: T, bottom: T }`.

7. **Testes** unit + eval (~10-18 testes adicionados
   estimados):
   - Construtor `Content::Pad` + `Content::Hide`.
   - `is_empty()`, `plain_text()`, `partial_eq`,
     `map_content`/`map_text` cobertura.
   - `eval_pad` com defaults; com `left`/`right`/etc;
     com `x`/`y` atalhos; com `rest`.
   - `eval_hide` body simples.
   - Layouter pad: padding reduz área disponível.
   - Layouter hide: emite zero items mas calcula
     dimensões.

8. **L0 prompts** + hashes propagados:
   - `00_nucleo/prompts/entities/content.md` ganha
     secção "Variants `Content::Pad` + `Content::Hide`
     — Passo 156C".
   - Hash `entities/content.rs` recomputado.
   - "Hash do Código" L0 actualizado.
   - Headers `@updated`: data execução.

9. **Inventário 148 actualizado**:
   - Tabela A.5 Layout: `pad` ausente → `implementado`;
     `hide` ausente → `implementado`.
   - Cobertura Layout: 4/18 → **6/18 = 33%**.
   - Tabela A linha "Layout": ajustar contagens
     (4/0/3/11/0=18 → 6/0/3/9/0=18).
   - Total user-facing recalculado: 53% → **~55%**.
   - §7 entrada 7: actualizar progresso Layout Fase 1.

10. **README dos ADRs actualizado**:
    - Tabela "Estado por ADR": linha ADR-0061 mantém-se
      PROPOSTO (anotação só em P156I per decisão humana).
    - Distribuição inalterada (ADR-0061 já contava em
      PROPOSTO 11 desde P156B).
    - Total inalterado (61 ADRs).
    - Entrada nova em "Passos-chave da história dos ADRs"
      para P156C: aplicação primeira de ADR-0061.

11. **ADR-0061 NÃO actualizada** neste passo (per decisão
    humana). Mantém-se PROPOSTO. Anotação cumulativa após
    P156I.

12. **Sem DEBTs criados/fechados**.

13. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156c-relatorio.md`.

Este passo **não**:

- Toca outros variants Content excepto adições.
- Toca código fora de `01_core/`.
- Materializa outras features Layout (block, box, etc.).
- Actualiza ADR-0061 com progresso (per decisão humana).
- Adiciona show rules `#show pad: ...` ou
  `#show hide: ...` — adiados a passo posterior.
- Implementa footnote area (sai do plano por decisão
  humana 2026-04-25).
- Toca série paridade (suspensa em P153).

---

## Decisões já tomadas

1. **Variants novos** (não Styled). Coerente com vanilla
   `PadElem`/`HideElem`.

2. **Pad com 7 atributos nomeados** (left/right/top/
   bottom/x/y/rest). Consistente com vanilla.

3. **Hide com body apenas**. Simplicidade.

4. **Layouter pad**: ajusta área disponível por padding;
   processa body com área reduzida.

5. **Layouter hide**: percorre body para calcular
   dimensões mas emite zero `FrameItem`s.

6. **Granularidade**: 2 features num passo. Consistente
   com decisão humana "12 passos granulares".

7. **Tests adicionados**: alvo 10-18 (ajustável). Análogo
   a P154B (9 tests para terms+divider).

8. **ADR-0061 NÃO anotada** neste passo. Anotação após
   P156I.

9. **Show rules adiadas**: candidato a passo agregado
   futuro (análogo ao adiamento de show rules Fase 1
   Model — anotação P154B).

## Decisões diferidas (resolvidas neste passo)

10. **Tipo `Sides<Length>`**:
    - Se existe em `01_core/src/entities/`, reusar.
    - Se não existe, criar como struct genérica
      `Sides<T>`. **Default**: criar como genérica
      reusable; usar `Sides<Length>` neste passo.

11. **Prioridade entre atributos pad** (left vs x vs rest):
    vanilla precedência: específico > eixo > rest. Aplicar
    mesma. Documentar em comentário código.

12. **Padding negativo**: rejeitar por agora (per ADR-0054
    perfil observacional graded). Validação em
    `native_pad`. Se vanilla aceita negativo, registar
    como divergência consciente em relatório.

13. **Hide com body vazio**: aceitável; produz zero
    items + dimensões zero. Sem comportamento especial.

14. **Layouter pad: cursor interaction**: padding interno
    afecta `(content_x, content_y)` início; padding
    externo (após body) afecta `cursor.advance` final.
    Implementar ambos em mesma função.

15. **Conflito de naming**: se houver `pad` ou `hide` em
    contexto não-Layout (improvável), preservar. Verificar
    durante execução.

---

## Escopo

**Dentro**:

- Modificação de `01_core/src/entities/content.rs`
  (2 variants novos + arms cobertura).
- Modificação de `01_core/src/entities/sides.rs` ou criação
  se não existe.
- Modificação de `01_core/src/rules/introspect.rs`
  (`materialize_time` + `walk`).
- Modificação de `01_core/src/rules/layout/mod.rs`
  (`layout_content` + `measure_content_constrained`).
- Modificação de `01_core/src/rules/eval/stdlib/structural.rs`
  (`native_pad` + `native_hide`).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo em `make_stdlib`).
- Tests em `01_core/src/entities/content.rs::tests`,
  `01_core/src/rules/eval/tests.rs`,
  `01_core/src/rules/layout/tests.rs` (se existe).
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Modificação de testes não-relacionados.
- Implementação de outras features Layout.
- Show rules `#show pad: ...` ou `#show hide: ...`.
- Footnote area.
- Crates externas.
- ADRs novas.
- DEBTs novos.
- Modificação de ADR-0061 (anotação cumulativa após
  P156I).
- Modificação de ADR-0060 (sem implicações neste passo).
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156C.1 — Verificar pré-condições

```bash
view 01_core/src/entities/content.rs   # confirmar 43 variants pós-P155
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
ls 01_core/src/entities/sides.rs 2>/dev/null  # existe Sides?
grep -rnE "Sides<" 01_core/src/entities/  # candidate location
```

Confirmar:
- Content tem 43 variants (Quote em P155 foi 43º).
- Sides<T> existe ou candidata location identificada.
- Layouter actual usa pattern-match exaustivo sobre Content.

### 156C.2 — Adicionar variants Content::Pad + Content::Hide

Edição de `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... 43 variants existentes (Text, Strong, Emph, ..., Quote)
    Pad {
        body: Box<Content>,
        padding: Sides<Length>,
    },
    Hide {
        body: Box<Content>,
    },
}
```

**Variant count**: 43 → **45** (+2).

### 156C.3 — Cobertura exaustiva de arms

Para cada função em `01_core/` que pattern-match sobre
Content, adicionar arms para `Pad` e `Hide`:

| Função | Tratamento Pad | Tratamento Hide |
|--------|----------------|-----------------|
| `is_empty()` | `body.is_empty()` | `body.is_empty()` |
| `plain_text()` | `body.plain_text()` | `String::new()` (não rende) |
| `PartialEq::eq` | comparação 2 fields | comparação 1 field |
| `map_content` | recurse body; preserva padding | recurse body |
| `map_text` | recurse body | recurse body |
| `materialize_time` | recurse body | recurse body |
| `walk` | walk body | walk body |
| `layout_content` | reduce area por padding; layout body | layout body apenas para dimensões; emite zero items |
| `measure_content_constrained` | constrain - padding * 2; recurse body | recurse body |

### 156C.4 — Sides<Length> existe ou criar

Verificar se `Sides<T>` existe em `01_core/src/entities/`.
Se não existir, criar:

```rust
// 01_core/src/entities/sides.rs (se novo)
pub struct Sides<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Clone> Sides<T> {
    pub fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self { left, right, top, bottom }
    }
    pub fn uniform(v: T) -> Self {
        Self::new(v.clone(), v.clone(), v.clone(), v)
    }
}
```

Usar `Sides<Length>` para padding em `Content::Pad`.

### 156C.5 — `native_pad` e `native_hide`

Em `01_core/src/rules/eval/stdlib/structural.rs`:

```rust
pub fn native_pad(args: &Args, _: &Engine) -> SourceResult<Value> {
    // body posicional
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::Text(s.clone(), Style::default()),
        Some(_) => return Err(unexpected_argument("body must be Content or Str")),
        None => return Err(missing_argument("body")),
    };

    // named args: left, right, top, bottom, x, y, rest
    let mut left = Length::zero();
    let mut right = Length::zero();
    let mut top = Length::zero();
    let mut bottom = Length::zero();
    let mut x_axis: Option<Length> = None;
    let mut y_axis: Option<Length> = None;
    let mut rest: Option<Length> = None;

    for (key, value) in args.named.iter() {
        let len = expect_length(value)?;
        match key.as_str() {
            "left" => left = len,
            "right" => right = len,
            "top" => top = len,
            "bottom" => bottom = len,
            "x" => x_axis = Some(len),
            "y" => y_axis = Some(len),
            "rest" => rest = Some(len),
            other => return Err(unexpected_named(other)),
        }
    }

    // precedência: específico > eixo > rest
    if let Some(r) = rest {
        if left.is_zero() { left = r; }
        if right.is_zero() { right = r; }
        if top.is_zero() { top = r; }
        if bottom.is_zero() { bottom = r; }
    }
    if let Some(x) = x_axis {
        if left.is_zero() { left = x; }
        if right.is_zero() { right = x; }
    }
    if let Some(y) = y_axis {
        if top.is_zero() { top = y; }
        if bottom.is_zero() { bottom = y; }
    }

    // validação: padding negativo rejeitado per decisão diferida 12
    if left.is_negative() || right.is_negative()
       || top.is_negative() || bottom.is_negative() {
        return Err(invalid_argument("padding cannot be negative"));
    }

    Ok(Value::Content(Content::Pad {
        body: Box::new(body),
        padding: Sides::new(left, right, top, bottom),
    }))
}

pub fn native_hide(args: &Args, _: &Engine) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::Text(s.clone(), Style::default()),
        Some(_) => return Err(unexpected_argument("body must be Content or Str")),
        None => return Err(missing_argument("body")),
    };

    if !args.named.is_empty() {
        return Err(unexpected_named("hide takes no named arguments"));
    }

    Ok(Value::Content(Content::Hide { body: Box::new(body) }))
}
```

Registo em `make_stdlib` (em `01_core/src/rules/eval/mod.rs`):

```rust
scope.define("pad", Value::Func(Func::native("pad", native_pad)));
scope.define("hide", Value::Func(Func::native("hide", native_hide)));
```

Stdlib funcs: 32 → **34**.

### 156C.6 — Layouter pad e hide

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    // ... arms existentes
    Content::Pad { body, padding } => {
        let original_x = self.cursor.x;
        let original_max_x = self.area_max_x;

        // ajusta área disponível por padding
        self.cursor.x += padding.left;
        self.cursor.y += padding.top;
        self.area_max_x -= padding.right;

        self.layout_content(body)?;

        // restaura
        self.cursor.y += padding.bottom;
        self.cursor.x = original_x;
        self.area_max_x = original_max_x;
    }
    Content::Hide { body } => {
        // calcula dimensões mas não emite items
        let saved_items = std::mem::take(&mut self.current_items);
        self.layout_content(body)?;
        self.current_items = saved_items;  // descarta items do hide
        // mantém cursor avançado pelas dimensões do body per vanilla
    }
    // ... fallback
}
```

Em `measure_content_constrained`:

```rust
match content {
    Content::Pad { body, padding } => {
        let constrained = max_width - padding.left - padding.right;
        let (w, h) = self.measure_content_constrained(body, constrained);
        (w + padding.left + padding.right, h + padding.top + padding.bottom)
    }
    Content::Hide { body } => {
        self.measure_content_constrained(body, max_width)
    }
    // ... fallback
}
```

### 156C.7 — Tests adicionados (alvo 10-18)

| Ficheiro | Testes |
|----------|--------|
| `01_core/src/entities/content.rs::tests` | (1) construtor Pad; (2) construtor Hide; (3) is_empty pad+hide; (4) plain_text pad (recurse); (5) plain_text hide (vazio); (6) PartialEq pad; (7) PartialEq hide; (8) map_content pad+hide |
| `01_core/src/rules/eval/tests.rs` | (9) `eval_pad` defaults; (10) `eval_pad` com left+right+top+bottom; (11) `eval_pad` com x+y; (12) `eval_pad` com rest; (13) `eval_pad` precedência; (14) `eval_pad` rejeita negativo; (15) `eval_hide` body simples; (16) `eval_hide` rejeita named arg |
| `01_core/src/rules/layout/tests.rs` | (17) layout pad reduz área; (18) layout hide zero items |

**Total**: ~14-18 tests adicionados. Estimativa final
após execução. Tests count cumulativo: **1145 → ~1159-
1163**.

### 156C.8 — L0 prompts + hashes

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Variants `Content::Pad` + `Content::Hide`
— Passo 156C":

```markdown
## Variants Pad e Hide (Passo 156C)

`Content::Pad { body, padding: Sides<Length> }`:
- container que aplica padding ao body durante layout;
- atributos vanilla: left, right, top, bottom, x, y, rest;
- precedência: específico > eixo > rest.

`Content::Hide { body }`:
- container que calcula dimensões mas não rende;
- útil para placeholders e equilíbrio visual.

Ambos seguem ADR-0026 perfil "variant novo" porque
vanilla expõe ambos como `#[elem]` proper.
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

Verificar:
- `entities/content.rs`: hash novo (era `8413bb8d` pós-
  P155; será diferente pós-P156C).
- `entities/content.md`: hash propagado.

Headers `@updated`:
- `content.rs`: data execução.
- `entities/content.md`: idem.

### 156C.9 — Inventário 148 actualizado

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A.5 Layout**:
- linha `pad`: ausente → **implementado** (referência:
  Passo 156C).
- linha `hide`: ausente → **implementado** (referência:
  Passo 156C).

**Tabela A linha "Layout"**:
- antes: `4 | 0 | 3 | 11 | 0 | 18`
- depois: `6 | 0 | 3 | 9 | 0 | 18` (2 ausentes → 2
  implementados).
- cobertura Layout: 22% → **33%**.

**Total user-facing**:
- antes: `54 | 21 | 22 | 42 | 2 | 141`.
- depois: `56 | 21 | 22 | 40 | 2 | 141` (2 ausentes → 2
  implementados na coluna Layout).
- cobertura user-facing: (56+21)/141 = **55%** (era 53%).

**§7 entrada 7**: actualizar progresso Layout Fase 1
(P156C cumprido).

### 156C.10 — README ADRs actualizado

- Tabela "Estado por ADR": linha ADR-0061 mantém-se
  PROPOSTO. Sem mudança.
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova:
  ```
  P156C: aplicação primeira de ADR-0061 — pad + hide
  variants. Cobertura Layout 22% → 33%.
  ```

### 156C.11 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156c-relatorio.md`.

Secções (modelo P155):
1. Sumário executivo.
2. Inventário pré-materialização (156C.1).
3. Variant `Content::Pad` — forma final + diff.
4. Variant `Content::Hide` — forma final + diff.
5. Cobertura exaustiva de arms (~7 sítios).
6. `Sides<Length>` — verificado/criado.
7. `native_pad` + `native_hide` — assinaturas + registo.
8. Layouter — diff (`layout_content` + `measure_content_constrained`).
9. Tests adicionados (lista + contagens).
10. L0 prompts + hashes propagados.
11. Inventário 148 actualizado.
12. README ADRs actualizado.
13. Próximo passo (P156D = h + v spacing).
14. Limitações registadas.
15. Verificação final.

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1159-1163 passed
   (1145 → +14-18); zero falhas.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes:
   - `entities/content.rs` ↔ `entities/content.md`.
5. ✅ `Content::Pad` e `Content::Hide` variants em
   produção.
6. ✅ Stdlib `#pad(...)` e `#hide(body)` invocáveis.
7. ✅ Cobertura arms exaustiva (sem `_ => ...` catch-all
   excepto onde já existe).
8. ✅ Inventário 148 reflecte cobertura aumentada
   (22% → 33%).
9. ✅ README ADRs entrada P156C.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (anotação só após P156I).
13. ✅ ADR-0060 inalterada.
14. ✅ Sem regressão (testes pré-P156C todos passam).
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::Pad` compila + tests unit passam | ✅ |
| 2 | `Content::Hide` compila + tests unit passam | ✅ |
| 3 | Stdlib `#pad(...)` invocável com 7 atributos | ✅ |
| 4 | Stdlib `#hide(body)` invocável | ✅ |
| 5 | Layouter pad reduz área por padding | ✅ |
| 6 | Layouter hide emite zero items mas calcula dimensões | ✅ |
| 7 | Sides<Length> usado consistentemente | ✅ |
| 8 | Inventário 148 reflecte cobertura 33% Layout | ✅ |
| 9 | Próximo passo (156D = h + v) tem âncora | ✅ |
| 10 | Sem regressão | ✅ |
| 11 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

- **`Sides<T>` já existe com forma diferente**: ajustar
  uso. Se conflito com forma esperada, criar `Sides2<T>`
  ou similar (improvável).

- **Pattern-match exaustivo causa falhas em ficheiros
  não-listados**: provável. Linter V2 pode falhar.
  Adicionar arms onde necessário; documentar.

- **Validação padding negativo conflita com vanilla**:
  vanilla pode aceitar negativo (margem invertida).
  Decisão diferida 12 rejeita. Se vanilla aceitar e isso
  for caso de teste comum, **registar como divergência
  consciente** análoga a P155 markup `"..."` →
  `Content::Text` (não Quote).

- **Layouter hide: cursor.y avança ou não?**: vanilla
  semantic é "calcula dimensões mas não rende". Cursor
  deve avançar (caso contrário próximo content sobrepõe).
  Confirmar empiricamente; se vanilla NÃO avança,
  registar divergência.

- **Layouter pad: padding atravessa border de página**:
  edge case complexo. Se body + padding excede altura
  página, pad deveria forçar pagebreak. Por agora,
  aceitar comportamento simples (pad pode ficar parcial
  entre páginas); registar limitação.

- **Tests Layout pad/hide rendering**: verificação visual
  via PDF é scope-out. Tests verificam dimensões e
  estrutura interna apenas (consistente com P140B/P141/
  P155 mesmo padrão).

- **Show rules `#show pad: ...` esperadas pelo user**:
  per decisão deste passo, scope-out. Documentar como
  limitação consciente. Show rules adiadas para passo
  agregado futuro.

- **Volume tests excede 18**: aceitável; ajustar contagem
  no relatório.

- **Volume tests inferior a 10**: investigar; padding tem
  7 atributos com precedência - testes comprehensive
  esperados.

- **Conflito "pad" naming em outro contexto**: vanilla
  não tem outro `pad`. Improvável conflito.

- **`Hide { body: Content::Hide { body: ... } }` aninhado**:
  case patológico. Layouter percorre recursivamente;
  resultado: zero items emitidos. Aceitável. Test edge
  case opcional.

- **`Pad` aninhado**: caso normal. Padding aplicado
  cumulativamente. Test esperado.

---

## Notas operacionais

- **Padrão "passos granulares 1-2 features"**: este é o
  primeiro passo da nova convenção. Modelo: 7 passos
  P156C → P156I para 72% Layout (sem footnote area, sem
  columns, sem measure/layout).

- **ADR-0061 mantém PROPOSTO**: per decisão humana
  2026-04-25. Anotação cumulativa após P156I. Risco baixo
  porque progresso é trackable via inventário 148 +
  relatórios cronológicos.

- **Sem footnote area**: per decisão humana 2026-04-25.
  Footnote area só quando Model footnote for atacado
  (passo posterior).

- **Sem columns**: trabalho L+ com refactor Layouter
  multi-region. DEBT-56 aberto em P156B. Passo dedicado
  futuro com ADR.

- **Sem measure/layout(callback)**: dependem de
  Introspection runtime (ADR-0017 adiada). Não atacar
  aqui.

- **Variants count**: 43 → 45 (+2). Após P156D (h+v):
  47. Após P156I (todas Fase 1+2 sem footnote): ~51.

- **Stdlib funcs**: 32 → 34. Após P156I: ~40.

- **Pós-156C**:
  - 2 features Layout implementadas (pad + hide).
  - Cobertura Layout: 22% → 33%.
  - Cobertura user-facing total: 53% → 55%.
  - **Próximo**: P156D (h + v spacing — 2 triviais).

- **Padrão emergente: Granularidade vs paridade rápida**:
  P156C testa hipótese de que 7 passos pequenos chegarão
  a 72% Layout sem reformulações. Se algum passo
  reformular ou descobrir mid-passo (antipadrão P102/P103
  do historiograma), reavaliar granularidade.

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.
