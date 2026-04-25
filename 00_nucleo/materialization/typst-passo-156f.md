# Passo 156F — skew via extensão Content::Transform (Layout Fase 1 sub-passo 4)

**Série**: 156F (passo **substantivo escopo S+ a M**;
materialização Layout, quarta sub-fase granular).
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → P156C (pad+hide) → P156D (h+v) → P156E
(pagebreak) → **P156F (skew via Transform)**.

**Precondição**: Passo 156E encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1214 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 50% (9/18 implementado puro pós-P156E — halfway
point); cobertura user-facing total 57%.

**Numeração**: P156F segue P156E na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations).

**Natureza**: passo **substantivo escopo S+ a M** (1 feature
adicionada via **refactor** de variant existente; ~10-18
testes adicionados estimados; **risco de regressão > 0** —
primeira vez na sequência granular pela natureza
modificadora; sem crates novas; sem ADRs novas; sem DEBTs
novos esperados).

**Decisão arquitectural P156F** (per resposta humana
2026-04-25): **estender `Content::Transform` existente** via
novo enum interno `TransformKind { Move, Rotate, Scale, Skew }`
em vez de adicionar variant `Content::Skew` separado.

Razão: P156B inventário registou que `move`, `rotate`,
`scale` estão "implementado via Content::Transform unificado".
Adicionar Skew separado quebraria a unificação existente.
Vanilla expõe os 4 como elementos separados (`MoveElem`,
`RotateElem`, `ScaleElem`, `SkewElem`); cristalino unifica
via TransformKind.

**Particularidade material face a P156C/D/E**: este é o
**primeiro passo da sequência granular que modifica struct
existente**. P156C/D/E adicionaram variants novos sem tocar
código existente. P156F **toca consumers de Transform já em
produção desde P78**. **Risco de regressão > 0** pela
primeira vez. Mitigação: tests existentes de move/rotate/
scale têm que continuar a passar.

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita
  modificações via decisão deste passo.
- **ADR-0033**: paridade funcional para skew.
- **ADR-0036**: atomização — consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded.
- **ADR-0061** (PROPOSTO): plano de Layout. Este passo
  aplica-o pela quarta vez.

---

## Contexto

P156E fechou pagebreak (50% cobertura Layout — halfway
point). Próximo natural era continuar Fase 1 com skew, que
no inventário 148 está classificado como ausente.

**`skew(content, ax, ay, origin)` em vanilla**:
- Atributo posicional `body: Content`.
- Atributos nomeados `ax: angle` e `ay: angle` (ângulos de
  distorção horizontal e vertical; defaults 0).
- Atributo nomeado `origin: align` (ponto de origem da
  transformação; default center+horizon).
- Semantic: aplica matriz de skew ao body durante render.

**Estado actual cristalino confirmar empiricamente**
(hipótese a verificar em 156F.1):
- `Content::Transform { body, ... }` existe desde P78.
- Forma actual provável (a confirmar): pode ter campos
  específicos para move (dx, dy), rotate (angle), scale
  (sx, sy) — possivelmente todos opcionais, ou via enum
  já existente, ou via matriz cm directa.
- Refactor para `TransformKind { Move, Rotate, Scale, Skew }`
  pode ser invasivo dependendo da forma actual.

**Hipóteses a confirmar empiricamente** (não compromisso):

- Forma actual de `Content::Transform`: a inventariar em
  156F.1.
- Refactor para enum `TransformKind` não muda comportamento
  externo (move/rotate/scale continuam a funcionar
  identicamente).
- Tests existentes de move/rotate/scale passam sem
  modificação ou com modificação mínima de constructor
  apenas.
- Layouter pode usar matriz cm composta para todos os 4
  kinds (já existe per P78 para curves+rotate+scale).
- `origin` para skew aplica análogo a rotate/scale (ponto
  de pivot).

---

## Objectivo

Ao fim do passo:

1. **Enum `TransformKind`** criado em
   `01_core/src/entities/transform_kind.rs` (novo) ou
   inline em `content.rs` (decisão diferida 11):

   ```rust
   pub enum TransformKind {
       Move  { dx: Length, dy: Length },
       Rotate { angle: Angle, origin: Align },
       Scale { sx: Ratio, sy: Ratio, origin: Align },
       Skew  { ax: Angle, ay: Angle, origin: Align },
   }
   ```

   Forma final ajustada consoante 156F.1 inventário.

2. **`Content::Transform`** refactorizado para usar
   `TransformKind`:

   ```rust
   Transform {
       body: Box<Content>,
       kind: TransformKind,
   }
   ```

3. **`native_skew`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#skew(body, ax: ?, ay: ?, origin: ?)`.

4. **`native_move`, `native_rotate`, `native_scale`
   actualizados** para construir Transform com TransformKind
   adequado. Sem mudança de comportamento externo.

5. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content::Transform` — actualizar
   para handling de TransformKind:
   - `entities/content.rs::is_empty()`.
   - `entities/content.rs::plain_text()`.
   - `entities/content.rs::PartialEq::eq`.
   - `entities/content.rs::map_content`.
   - `entities/content.rs::map_text`.
   - `rules/introspect.rs::materialize_time`.
   - `rules/introspect.rs::walk`.
   - `rules/layout/mod.rs::layout_content`.
   - `rules/layout/mod.rs::measure_content_constrained`.

6. **Layouter skew**: aplica matriz cm composta com skew_x
   e skew_y per origin. Reusa lógica de matrix composition
   existente para rotate/scale.

7. **Testes** unit + eval (~10-18 testes adicionados
   estimados):
   - `TransformKind` enum + variantes.
   - `eval_skew` defaults (ax=0, ay=0).
   - `eval_skew` com ax+ay.
   - `eval_skew` com origin.
   - `eval_skew` rejeita named arg desconhecido.
   - **Tests existentes de move/rotate/scale continuam a
     passar** (regressão zero é critério).
   - Layouter skew aplica matriz correcta.
   - Construtor refactorizado de Transform.

8. **L0 prompts** + hashes propagados:
   - Possível `00_nucleo/prompts/entities/transform_kind.md`
     (se for ficheiro próprio).
   - `00_nucleo/prompts/entities/content.md` ganha secção
     "Refactor `Content::Transform` + variant skew —
     Passo 156F".
   - Hash `entities/content.rs` recomputado.
   - Headers `@updated`: data execução.

9. **Inventário 148 actualizado**:
   - Tabela A.5 Layout: linha `skew` ausente →
     `implementado`.
   - Cobertura Layout: 9/18 → **10/18 = 56%**.
   - Tabela A linha "Layout": `9/0/3/6/0=18` →
     `10/0/3/5/0=18`.
   - Total user-facing: 57% → **~58%**.
   - Tabela B Content variants: 48 (sem mudança em count;
     refactor não adiciona variant nem remove).
   - **Possível atualização**: nota sobre refactor
     Transform → TransformKind.
   - §7 entrada 7: actualizar progresso Layout (P156F
     cumprido; restantes 5 entradas Layout).

10. **README dos ADRs actualizado**:
    - Tabela "Estado por ADR": linha ADR-0061 mantém-se
      PROPOSTO.
    - Distribuição inalterada.
    - Total inalterado.
    - Entrada nova em "Passos-chave" para P156F.

11. **ADR-0061 NÃO actualizada** (per decisão humana).

12. **Sem DEBTs criados/fechados** (esperado).

13. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156f-relatorio.md`.

Este passo **não**:

- Toca outras features Layout além de skew.
- Implementa show rules `#show skew: ...` ou
  `#show <transform>: ...`.
- Toca série paridade.
- Modifica ADR-0061.
- Implementa column flow ou outras features Fase 3.

---

## Decisões já tomadas

1. **Estender Content::Transform via TransformKind enum**
   (per resposta humana 2026-04-25). Não adicionar variant
   separado.

2. **Granularidade**: 1 feature adicionada via refactor.
   Conservador no que toca a tests adicionados (~10-18)
   mas com risco de regressão > 0 pela primeira vez.

3. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`
   per P156C/D/E.

4. **Assinatura natives**: 5-param canónica.

5. **Sem mudança de comportamento externo** para move/
   rotate/scale. Tests existentes critério bloqueante.

6. **Tests adicionados**: alvo 10-18 (ajustável).

7. **ADR-0061 NÃO anotada**.

8. **Show rules adiadas**: candidato a passo agregado
   futuro.

## Decisões diferidas (resolvidas neste passo)

9. **Forma exacta de TransformKind**: dependerá de 156F.1
   inventário. Se Transform actual já tem matriz cm directa
   (sem campos por tipo), TransformKind pode ser
   simplesmente:
   ```rust
   pub enum TransformKind {
       Move, Rotate, Scale, Skew  // sem fields
   }
   ```
   E os parameters viverem em `Content::Transform { body,
   kind, params: TransformParams }`. Decisão consoante o
   código actual.

10. **`origin` para skew**: por defeito `align(center +
    horizon)` (ponto de origem ao centro). Coerente com
    vanilla.

11. **Localização TransformKind**: ficheiro próprio
    `transform_kind.rs` ou inline em `content.rs`?
    **Default**: ficheiro próprio se >30 linhas; inline
    caso contrário.

12. **Pattern-match sobre TransformKind**: em todos os
    arms existentes que pattern-match Content::Transform,
    actualizar para também match `kind`. Cuidado especial
    em layouter que aplica matriz.

13. **Tests existentes de move/rotate/scale**: se
    constructor mudou, ajustar inicialização nos tests.
    **Critério**: zero regressão; tests passam ou são
    actualizados para nova forma sem mudar semantic.

14. **`extract_angle` helper**: pode existir ou precisar
    criar. Análogo a `extract_length` (P156C),
    `extract_weak` (P156D), `extract_parity` (P156E).

15. **`extract_align` helper**: pode existir (align é tipo
    fundamental usado em `place`, `align`); reusar.

16. **Layouter matriz cm**: P78 já estabeleceu padrão
    (matriz cm + AABB). Skew adiciona termos `tan(ax)` e
    `tan(ay)` na matriz. Reusar lógica existente.

---

## Escopo

**Dentro**:

- Possível criação de `01_core/src/entities/transform_kind.rs`
  ou modificação inline em `content.rs`.
- Modificação de `01_core/src/entities/content.rs`
  (refactor variant Transform + arms cobertura).
- Modificação de `01_core/src/rules/introspect.rs`.
- Modificação de `01_core/src/rules/layout/mod.rs`.
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_skew` novo + actualização de `native_move`,
  `native_rotate`, `native_scale`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo `skew` em `make_stdlib`).
- Tests novos em
  `01_core/src/rules/stdlib/mod.rs::tests`,
  `01_core/src/rules/layout/tests.rs`.
- **Possível ajuste** de tests existentes de move/rotate/
  scale (se constructor mudou; critério: semantic
  inalterada).
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Implementação de outras features Layout.
- Show rules.
- Crates externas.
- ADRs novas.
- DEBTs novos.
- Modificação de ADR-0061.
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156F.1 — Inventariar Content::Transform actual

**Crítico** para este passo (mais que para P156C/D/E porque
está-se a refactorar struct existente).

```bash
view 01_core/src/entities/content.rs   # confirmar 48 variants pós-P156E
grep -nE "Content::Transform" 01_core/src/  # localizar todos os usages
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
view 01_core/src/entities/transform.rs 2>/dev/null  # se existe
view 01_core/src/rules/layout/mod.rs   # arm Transform em layout_content
view 01_core/src/rules/stdlib/layout.rs  # native_move/rotate/scale
```

Documentar em 156F.1 do diagnóstico:
- **Forma exacta** de `Content::Transform`.
- **Lista de consumers** que pattern-match Transform
  (esperado: ~9 sítios análogos a outros variants).
- **Forma de natives** existentes: como move/rotate/scale
  constroem Transform.
- **Tests existentes** que mencionam Transform: nomes e
  localizações.

**Decisão sub-condicional**: se forma actual for
**radicalmente diferente** do esperado (ex: um variant
separado por tipo de transform), reavaliar plano:
- Pausa.
- Documentar discrepância.
- Discutir com humano se refactor vale o trabalho.

### 156F.2 — Definir TransformKind

Consoante 156F.1, decidir entre:

**Opção A** (se Transform actual usa matriz cm):
```rust
pub enum TransformKind {
    Move,
    Rotate,
    Scale,
    Skew,
}
// Content::Transform { body, kind: TransformKind, matrix: AffineMatrix }
```

**Opção B** (se Transform actual tem fields por tipo):
```rust
pub enum TransformKind {
    Move   { dx: Length, dy: Length },
    Rotate { angle: Angle, origin: Align },
    Scale  { sx: Ratio, sy: Ratio, origin: Align },
    Skew   { ax: Angle, ay: Angle, origin: Align },
}
// Content::Transform { body, kind: TransformKind }
```

**Opção C** (compromisso): kind enum com fields comuns
fora; campos específicos via Optional:
```rust
pub enum TransformKind {
    Move, Rotate, Scale, Skew
}
pub struct TransformParams {
    pub move_offset: Option<(Length, Length)>,
    pub rotate_angle: Option<Angle>,
    pub scale_factor: Option<(Ratio, Ratio)>,
    pub skew_angles: Option<(Angle, Angle)>,
    pub origin: Align,
}
// Content::Transform { body, kind: TransformKind, params: TransformParams }
```

**Default**: opção B (mais idiomática Rust; less waste).
Consoante 156F.1, ajustar.

### 156F.3 — Refactor Content::Transform

Edição de `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... outros variants
    Transform {
        body: Box<Content>,
        kind: TransformKind,
    },
}
```

Variant count: 48 (inalterado; refactor de variant
existente).

**Crítico**: actualizar todos os 9+ arms que match Transform
para também unpack kind.

### 156F.4 — Actualizar natives existentes

`native_move`, `native_rotate`, `native_scale` actualizados
para construir Transform com TransformKind adequado:

```rust
pub fn native_move(...) -> SourceResult<Value> {
    // ... extracção de dx, dy
    Ok(Value::Content(Content::Transform {
        body: Box::new(body),
        kind: TransformKind::Move { dx, dy },
    }))
}
// Análogo para native_rotate e native_scale.
```

**Sem mudança de comportamento externo**: tests existentes
verificam outputs idênticos. Se algum teste verifica
estrutura interna de Transform, ajustar para nova forma
mantendo semantic.

### 156F.5 — Adicionar native_skew

```rust
pub fn native_skew(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::Text(s.clone(), Style::default()),
        Some(_) => return Err(...),
        None => return Err(missing_argument("body")),
    };

    let mut ax = Angle::zero();
    let mut ay = Angle::zero();
    let mut origin = Align::default();  // center + horizon

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "ax" => ax = extract_angle(value)?,  // helper a verificar
            "ay" => ay = extract_angle(value)?,
            "origin" => origin = extract_align(value)?,  // helper a verificar
            other => return Err(unexpected_named(other)),
        }
    }

    Ok(Value::Content(Content::Transform {
        body: Box::new(body),
        kind: TransformKind::Skew { ax, ay, origin },
    }))
}
```

Registo em `make_stdlib`:

```rust
scope.define("skew",
    Value::Func(Func::native("skew", native_skew)));
```

Re-export em `stdlib/mod.rs` (adicionar `native_skew` à
lista).

Stdlib funcs: 37 → **38** (+1).

### 156F.6 — Layouter skew

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    Content::Transform { body, kind } => {
        match kind {
            TransformKind::Move { dx, dy } => {
                // lógica existente para move
            }
            TransformKind::Rotate { angle, origin } => {
                // lógica existente para rotate
            }
            TransformKind::Scale { sx, sy, origin } => {
                // lógica existente para scale
            }
            TransformKind::Skew { ax, ay, origin } => {
                // novo: matriz cm com tan(ax) e tan(ay)
                let pivot = origin.resolve(...);
                let sx_term = ax.tan();
                let sy_term = ay.tan();
                // applies skew matrix to body
                // ...
            }
        }
    }
    // ... outros arms
}
```

A matriz de skew (em torno da origem):
```
| 1       tan(ax)   0 |
| tan(ay)  1        0 |
| 0        0        1 |
```

Aplicada ao body em torno do pivot definido por origin.

### 156F.7 — Tests adicionados (alvo 10-18)

| Ficheiro | Testes |
|----------|--------|
| `01_core/src/entities/content.rs::tests` | (1) transform_kind_partial_eq variants; (2) transform com skew kind |
| `01_core/src/rules/stdlib/mod.rs::tests` | (3) `native_skew` defaults (ax=0, ay=0); (4) `native_skew` com ax; (5) `native_skew` com ay; (6) `native_skew` com ax+ay; (7) `native_skew` com origin; (8) `native_skew` rejeita named arg desconhecido; (9) `native_skew` sem body → Err; (10) **regression test**: native_move continua a produzir Transform com TransformKind::Move; (11) regression test rotate; (12) regression test scale |
| `01_core/src/rules/layout/tests.rs` | (13) layout_skew_aplica_matriz_correcta; (14) layout_skew_origin_default_centro |

**Total**: ~14 tests novos. **Crítico**: tests 10-12 são
**regression tests** que verificam que move/rotate/scale
continuam a funcionar com refactor.

Tests cumulativos: **1214 → ~1228**.

### 156F.8 — L0 prompts + hashes

Consoante 156F.2 decisão (TransformKind ficheiro próprio
ou inline):

- Se ficheiro próprio: criar
  `00_nucleo/prompts/entities/transform_kind.md`.
- Se inline: actualizar
  `00_nucleo/prompts/entities/content.md` apenas.

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Refactor `Content::Transform` + variant
skew — Passo 156F" após secção P156E pagebreak:

```markdown
## Refactor Content::Transform + Skew (Passo 156F)

`Content::Transform { body, kind: TransformKind }`:
- refactor de variant existente (P78);
- TransformKind unifica Move/Rotate/Scale/Skew em enum.

`TransformKind::Skew { ax, ay, origin }`:
- adicionado em P156F;
- aplica matriz de skew com tan(ax) e tan(ay).

Tests existentes de move/rotate/scale continuam a passar
(regressão zero é critério).
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

Verificar:
- `entities/content.rs`: hash novo (era `b632e841` pós-
  P156E).
- `entities/transform_kind.rs` (se ficheiro próprio): hash
  novo.

### 156F.9 — Inventário 148 actualizado

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A.5 Layout**:
- linha `skew`: ausente → **implementado**.

**Tabela A linha "Layout"**:
- antes: `9 | 0 | 3 | 6 | 0 | 18`
- depois: `10 | 0 | 3 | 5 | 0 | 18`
- cobertura Layout: 50% → **56%**.

**Total user-facing**:
- antes: `59 | 21 | 22 | 37 | 2 | 141`.
- depois: `60 | 21 | 22 | 36 | 2 | 141`.
- cobertura user-facing: (60+21)/141 = **57%** (mantém-se;
  arredondamento; 60/141 = 42.5%).

**Tabela B Content variants**:
- inalterado em count (48); nota nova sobre refactor
  Transform → TransformKind interno.

**§7 entrada 7**: actualizar progresso Layout (P156F
cumprido; restantes 5 entradas Layout: block, box, stack,
repeat, columns/colbreak).

### 156F.10 — README ADRs actualizado

- Tabela "Estado por ADR": linha ADR-0061 mantém-se
  PROPOSTO.
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova:
  ```
  P156F: aplicação quarta de ADR-0061 — skew via refactor
  Content::Transform com TransformKind enum. Cobertura
  Layout 50% → 56%. Primeira modificação de variant
  existente na sequência granular; zero regressão.
  ```

### 156F.11 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156f-relatorio.md`.

Secções (modelo P156E):
1. Sumário executivo.
2. Inventário pré-materialização (resumo 156F.1; **crítico**
   neste passo).
3. `TransformKind` enum — forma final + diff.
4. Refactor `Content::Transform` — diff.
5. Cobertura exaustiva de arms (todos os 9+ sítios
   actualizados).
6. `native_skew` + actualizações de move/rotate/scale —
   assinaturas + registo.
7. Layouter — diff (skew matriz + reuso de lógica
   existente).
8. Tests adicionados (lista + contagens; **incluir
   regression tests**).
9. L0 prompts + hashes propagados.
10. Inventário 148 actualizado.
11. README ADRs actualizado.
12. Próximo passo (P156G = block).
13. Limitações registadas.
14. Verificação final.
15. **Análise de risco de regressão** (novo neste passo
    — pela primeira vez na sequência granular).

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1228 passed
   (1214 → +14); zero falhas. **Tests existentes de
   move/rotate/scale TODOS passam**.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes.
5. ✅ `Content::Transform` refactorizado para usar
   TransformKind.
6. ✅ `TransformKind` enum em produção.
7. ✅ Stdlib `#skew(...)` invocável (37 → 38 funcs).
8. ✅ Stdlib `#move(...)`, `#rotate(...)`, `#scale(...)`
   continuam invocáveis com semantic inalterada.
9. ✅ Cobertura arms exaustiva.
10. ✅ Inventário 148 reflecte cobertura aumentada
    (50% → 56%).
11. ✅ README ADRs entrada P156F.
12. ✅ Sem ADR criada / revogada / revisada.
13. ✅ Sem DEBT criado / fechado.
14. ✅ ADR-0061 inalterada.
15. ✅ **Sem regressão em tests de move/rotate/scale**
    (critério bloqueante; explicitar em relatório).
16. ✅ Sem regressão geral.
17. ✅ Relatório do passo escrito.

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `TransformKind` enum compila + tests passam | ✅ |
| 2 | `Content::Transform` refactorizado compila | ✅ |
| 3 | Stdlib `#skew(body, ax: ?, ay: ?, origin: ?)` invocável | ✅ |
| 4 | move/rotate/scale continuam invocáveis | ✅ |
| 5 | Layouter skew aplica matriz correcta | ✅ |
| 6 | **Regression tests de move/rotate/scale passam** | ✅ |
| 7 | Inventário 148 reflecte cobertura 56% Layout | ✅ |
| 8 | Próximo passo (156G = block) tem âncora | ✅ |
| 9 | Sem regressão geral | ✅ |
| 10 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

- **Forma actual de Content::Transform muito diferente do
  esperado**: descoberto em 156F.1. Se for refactor maior
  do que esperado, **pausar** e consultar humano. Decisão
  default: continuar se cabe em ~50% mais escopo; pausar
  se exigir XL.

- **Tests existentes de move/rotate/scale falham após
  refactor**: indicativo de mudança de semantic não
  intencional. Investigar; corrigir refactor; documentar
  causa em relatório. **Critério bloqueante**: zero
  regressão.

- **`extract_angle` ou `extract_align` helpers não existem**:
  criar análogos a `extract_length` (P156C),
  `extract_parity` (P156E). Adicionar nestes locais.

- **Origin para skew comporta diferente de rotate/scale**:
  vanilla pode aplicar pivot diferentemente. Confirmar;
  registar divergência consciente se necessário.

- **Matriz cm composição falha em corner cases**:
  ex, skew + rotate aninhado. Tests devem cobrir; se
  falha, registar limitação.

- **Volume tests excede 18**: aceitável; ajustar relatório.

- **Volume tests inferior a 10**: investigar; skew tem
  vários edge cases (ax sozinho, ay sozinho, ambos,
  origin custom, valores grandes).

- **Modificação de natives existentes quebra tests
  internos do stdlib/mod**: provável; ajustar tests
  para nova forma sem mudar semantic exposta.

- **Pattern-match exaustivo sobre TransformKind nas 9+
  arms**: Linter V2 alerta se omitido. Cuidadoso em
  todos os locais; verificar build clean.

- **`origin` opcional em vanilla mas obrigatório em
  TransformKind**: divergência. Default em cristalino
  resolve via `Align::default()`. Coerente.

- **Regression test design**: tests novos devem **explicitamente
  reproduzir** comportamento de tests existentes (mesmo
  inputs, mesmos outputs esperados) para ter confiança
  alta de zero regressão.

- **Refactor expõe DEBTs implícitos**: se Transform actual
  tem dívida arquitectural (ex: campos confusos), refactor
  pode revelar. Aceitar; documentar; abrir DEBT só se
  bloqueante.

---

## Notas operacionais

- **Padrão "passos granulares" — quarta aplicação**.
  P156C+P156D+P156E mantiveram cadência limpa.
  P156F **modifica** estrutura existente — primeira
  diferença material na sequência. Risco de regressão > 0
  pela primeira vez. Hipótese da decisão humana 2026-04-25
  ainda não testada com modificações; este passo é teste
  natural.

- **Mitigação de risco de regressão**:
  1. Sub-passo 156F.1 expande inventário pré-materialização
     (mais tempo a verificar antes de modificar).
  2. Tests existentes critério bloqueante.
  3. Regression tests explícitos para move/rotate/scale.
  4. Pausa-e-consulta se discrepância radical em 156F.1.

- **TransformKind unificação**: análoga a outros casos
  vanilla onde elementos separados são unificados em
  cristalino (Quote/Divider/Terms em P154-155). Padrão
  arquitectural emergente.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.

- **Variants count**: 48 (inalterado; refactor não
  adiciona/remove variant). Após P156G (block): 49 ou
  50. Após P156I: 51-52.

- **Stdlib funcs**: 37 → **38** (+1, skew). Após P156I:
  ~40.

- **Pós-156F**:
  - 7 features Layout implementadas total (pad, hide,
    h, v, pagebreak, skew + align/move/rotate/scale via
    Transform unificado).
  - Cobertura Layout: 50% → **56%**.
  - Cobertura user-facing total: 57% → ~57-58%.
  - **Próximo**: P156G (block) ou alternativa humana.

- **Hipótese da granularidade — N=4 testa risco de
  regressão**: P156C+D+E foram aditivos. P156F é
  modificador. Se zero regressão, hipótese empírica
  reforçada para incluir refactors moderados. Se
  regressão emergir, refinar hipótese para distinguir
  passos aditivos de passos modificadores.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Slope cumulativo Fase 1**: P156C +11%, P156D +11%,
  P156E +6%, P156F +6% = 34% cumulativo em 4 passos.
  Restantes 16 pontos percentuais para 72% target em 3
  passos (P156G/H/I) = ~5-6% por passo. Realista
  consoante features Fase 2 (containers ricos block/box/
  stack).
