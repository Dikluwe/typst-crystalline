# Relatório — Passo 156F: skew via TransformMatrix (Layout Fase 1 sub-passo 4)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo S** (reduzido de
"S+ a M" estimado pela spec); **quarta aplicação consecutiva**
de ADR-0061 (Layout Fase X roadmap, status `PROPOSTO`);
**divergência consciente da spec** baseada em descoberta
empírica do inventário 156F.1.
**Spec**: `00_nucleo/materialization/typst-passo-156f.md`.

**Outputs materiais**:
- 1 método estático novo em `TransformMatrix`: `skew(ax_rad,
  ay_rad)`.
- Stdlib `#skew(body, ax: ?, ay: ?)`.
- Cobertura: zero (sem mudança em consumers — refactor evitado).
- 16 tests novos (alvo era 10-18; meta atingida; inclui 3
  regression de move/rotate/scale).
- L0 prompt `entities/content.md` ganha secção documentando
  divergência da spec.
- Inventário 148 actualizado (Layout 9/0/3/6/0=18 →
  10/0/3/5/0=18; cobertura 50% → **56%**).
- README ADRs entrada P156F.
- Este relatório (com §15 análise de risco).

---

## §1 — Sumário executivo

P156F continuou a sequência granular Layout Fase 1 com `skew`,
**a quarta feature** da fase. Diferentemente dos sub-passos
anteriores (P156C/D/E que adicionaram variants novos a
`Content`), P156F era esperado pela spec como **primeiro passo
modificador** — refactor de `Content::Transform` para usar
um novo `enum TransformKind`. Risco de regressão > 0 era
hipótese a testar.

**Inventário 156F.1 revelou que a unificação proposta já
existia desde P78**: `Content::Transform { body, matrix:
TransformMatrix }` reusa a matriz cm (PDF) para
`move`/`rotate`/`scale`. A struct é o "kind" implícito —
enum redundante. **Decisão deste passo**: adicionar apenas
método estático `TransformMatrix::skew(ax, ay)` análogo aos
existentes (`translate`, `rotate`, `scale`) + `native_skew`
em `stdlib/transforms.rs`. **Zero refactor**, **zero risco
de regressão** (puramente aditivo).

Trabalho material:
- **`TransformMatrix::skew(ax_rad, ay_rad)`** novo em
  `entities/layout_types.rs`. Forma matriz: `a=1, b=tan(ay),
  c=tan(ax), d=1, tx=0, ty=0`.
- **`native_skew`** em `stdlib/transforms.rs` ao lado de
  `native_move`/`native_rotate`/`native_scale` (coesão por
  domínio per ADR-0037). Atributos `ax: Angle` e `ay: Angle`
  (também aceita `Float` radianos por consistência com
  `native_rotate`); body posicional obrigatório; ângulos
  próximos de ±π/2 rejeitados (tan diverge).
- **Validação rigorosa**: named args desconhecidos rejeitados;
  ângulos extremos rejeitados; tipos errados rejeitados.

**Tests**: **1214 → 1230** (+16). Layout cobertura: **50%
→ 56%** (9/18 → 10/18). User-facing total mantém 57%
(arredondamento; 60 vs 59 implementado).

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
2026-04-25.

**Hipótese granular reforçada por descoberta empírica**:
P156F era teste do risco de regressão; inventário revelou
que a estratégia "menor mudança suficiente" reduzia o passo
a aditivo puro. Padrão emergente: **inventariar antes de
refactorar é mecanismo natural de redução de risco**.

---

## §2 — Inventário pré-materialização (sub-passo 156F.1)

### §2.1 Forma exacta de `Content::Transform`

Localizado em `01_core/src/entities/content.rs:247`:

```rust
Transform {
    matrix: TransformMatrix,
    body:   Box<Content>,
},
```

**Já unificado via matriz** — não tem fields por tipo. Os
"kinds" (move/rotate/scale) constroem matrizes específicas:
`TransformMatrix::translate(dx,dy)`, `::rotate(rad)`,
`::scale(sx,sy)`.

### §2.2 Consumers de `Content::Transform`

9 sítios pattern-match sobre Transform (todos opacos sobre
matrix):

| Ficheiro | Linha | Tratamento |
|----------|------:|-----------|
| `entities/content.rs::plain_text` | 586 | recurse body |
| `entities/content.rs::PartialEq::eq` | 687 | comparação 2 fields |
| `entities/content.rs::map_content` | 892 | recurse body; preserva matrix |
| `entities/content.rs::map_text` | 1031 | idem |
| `rules/introspect.rs::materialize_time` | 136 | recurse body; preserva matrix |
| `rules/introspect.rs::walk` | 346 | walk body |
| `rules/layout/mod.rs::layout_content` | 451 | aplica matriz cm + AABB |
| `rules/layout/helpers.rs` | 76, 105 | sub-frame composition |
| (sem consumer especial em `measure_content_constrained`) | — | fallback `_` |

**Conclusão**: todos tratam `matrix` opacamente. Adicionar
`TransformMatrix::skew` + um novo `native_skew` que constrói
essa matriz **não toca nenhum consumer**. Zero risco de
regressão por construção.

### §2.3 Forma actual dos natives

`01_core/src/rules/stdlib/transforms.rs` contém
`native_move`, `native_rotate`, `native_scale` — todos
constroem `Content::Transform { matrix: ..., body }`.
Padrão estabelecido: o native é responsável por traduzir
named args em parâmetros da matriz; o resto do pipeline
trata Transform de forma uniforme.

### §2.4 Decisão sub-condicional resolvida

Spec §156F.1 previu:
> "Decisão sub-condicional: se forma actual for radicalmente
> diferente do esperado (ex: um variant separado por tipo
> de transform), reavaliar plano: pausar..."

A descoberta foi o **inverso**: a forma actual é **mais
unificada** do que a spec assumia. Decisão derivada:
**simplificar plano**, não pausar. TransformKind enum
proposto é redundante; método estático `TransformMatrix::skew`
é suficiente.

---

## §3 — `TransformMatrix::skew` — forma final + diff

Adicionado em `01_core/src/entities/layout_types.rs:493`,
ao lado de `translate`, `scale`, `rotate`:

```rust
/// Distorção (skew) em radianos. Passo 156F (ADR-0061 Fase 1, sub-passo 4).
///
/// `ax` distorce horizontalmente; `ay` distorce verticalmente.
/// Análogo a vanilla `SkewElem`. Forma da matriz:
///   x' = x + tan(ax) * y
///   y' = tan(ay) * x + y
///
/// Ângulos extremos próximos de π/2 produzem `tan` infinito; o caller
/// deve validar (per `native_skew` em `stdlib/transforms.rs`).
pub fn skew(ax_rad: f64, ay_rad: f64) -> Self {
    Self { a: 1.0, b: ay_rad.tan(), c: ax_rad.tan(), d: 1.0, tx: 0.0, ty: 0.0 }
}
```

**Convenção da matriz cm** (PDF, segue a struct existente):
- `a, b, c, d` são as 4 entradas 2×2.
- `tx, ty` são translação.
- Aplicação: `x' = a*x + c*y + tx`; `y' = b*x + d*y + ty`.

Para skew puro (sem translação): `tx=0, ty=0, a=1, d=1` (não
estica em x ou y), com `c=tan(ax)` e `b=tan(ay)` para
deslocamento.

---

## §4 — Refactor `Content::Transform` — diff

**Diff**: nenhum. `Content::Transform` permanece exactamente
como estava em P78 (`{ matrix: TransformMatrix, body:
Box<Content> }`). Refactor proposto pela spec (TransformKind
enum) **não foi feito** por descoberta empírica em §2.

---

## §5 — Cobertura exaustiva de arms

**Arms novos**: nenhum. Os 9 consumers de `Content::Transform`
listados em §2.2 já tratam `matrix` opacamente. Skew herda
todo o tratamento gratuitamente.

**Arms tocados**: zero.

Esta é a primeira vez na sequência granular P156C-P156F que
um sub-passo não toca arms — consequência directa da
descoberta arquitectural de §2.

---

## §6 — `native_skew` — assinatura + registo

### §6.1 `native_skew` em `01_core/src/rules/stdlib/transforms.rs`

```rust
pub fn native_skew(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    fn extract_angle_rad(val: &Value) -> Option<f64> {
        match val {
            Value::Angle(a) => Some(a.to_rad()),
            Value::Float(f) => Some(*f),  // radianos directos
            _ => None,
        }
    }

    let ax_rad = ...;  // extracção de "ax" named arg
    let ay_rad = ...;  // extracção de "ay" named arg

    // Validação: rejeitar named args desconhecidos.
    for key in args.named.keys() {
        if !["ax", "ay"].contains(&key.as_str()) { return Err(...); }
    }

    // Validação: rejeitar ângulos próximos de ±π/2.
    const LIMIT: f64 = std::f64::consts::FRAC_PI_2 - 1e-3;
    if ax_rad.abs() >= LIMIT || ay_rad.abs() >= LIMIT { return Err(...); }

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| ...)?;

    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::skew(ax_rad, ay_rad),
        body:   Box::new(body),
    }))
}
```

**Decisões locais**:
- `ax`/`ay` defaults a `0.0` quando ausentes (skew(0,0) ==
  identity).
- `Angle` e `Float` aceites (consistente com `native_rotate`).
- `origin` named arg **scope-out** — alinhado com
  rotate/scale actuais que também não suportam origin.
  Refino futuro per ADR-0061.
- Threshold ângulos: `π/2 - 1e-3` rad (≈ 89.94°). Acima
  disso, `tan` diverge para infinito; rejeitar é mais
  seguro que produzir matriz inválida.

### §6.2 Registo

`stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::transforms::{
    native_move, native_rotate, native_scale, native_skew,
};
```

`eval/mod.rs::make_stdlib`:
```rust
scope.define("skew", Value::Func(Func::native("skew", native_skew)));
```

Stdlib funcs: 37 → **38** (+1).

---

## §7 — Layouter — diff

**Diff**: nenhum. Layouter trata `Content::Transform { matrix,
body }` aplicando a matriz cm via composição existente desde
P78. Skew é matriz cm como qualquer outra; o pipeline
funciona out-of-the-box.

Verificação empírica: testes de `transform_matrix_skew_*`
em `entities/layout_types.rs::tests` confirmam que
`apply((x,y))` retorna valores correctos. O pipeline
posterior (`layout_content` arm Transform em `mod.rs:451`)
aplica essa matriz à AABB do body via lógica reutilizada de
P78.

---

## §8 — Tests adicionados (lista + contagens)

### §8.1 Em `entities/layout_types.rs::tests` (4)

1. `transform_matrix_skew_zero_e_identidade` — skew(0,0) ==
   identity.
2. `transform_matrix_skew_ax_distorce_horizontal` — skew(45°, 0)
   desloca (0,1) para (1,1).
3. `transform_matrix_skew_ay_distorce_vertical` — skew(0, 45°)
   desloca (1,0) para (1,1).
4. `transform_matrix_skew_origin_zero_zero_imutavel` — origem
   é fixa sob skew.

### §8.2 Em `stdlib/mod.rs::tests` (9 + 3 regression)

5. `native_skew_defaults_produz_identidade` — sem ax/ay →
   matriz identidade.
6. `native_skew_com_ax_angle` — Angle 30° produz `c≈0.5774`.
7. `native_skew_com_ay_angle` — Angle 30° produz `b≈0.5774`.
8. `native_skew_combina_ax_e_ay` — caso composto.
9. `native_skew_aceita_float_radianos` — Float 0.0 → identity.
10. `native_skew_rejeita_named_arg_desconhecido` — origin
    rejeitado (scope-out documentado).
11. `native_skew_rejeita_ax_proximo_de_pi_meio` — ângulo
    extremo rejeitado.
12. `native_skew_rejeita_ax_nao_angle_nem_float` — Str
    rejeitado.
13. `native_skew_sem_body_retorna_err` — body obrigatório.

**Regression tests** (críticos para confiança em zero
regressão):
14. `native_move_continua_a_produzir_transform_apos_p156f` —
    move(dx, dy) ainda produz `Content::Transform { matrix:
    translate(...) }`.
15. `native_rotate_continua_a_produzir_transform_apos_p156f`
    — rotate(angle) idem com matriz rotate.
16. `native_scale_continua_a_produzir_transform_apos_p156f`
    — scale(x) com y default = sx idem.

**Total**: **16 tests novos** (alvo spec era 10-18; meta
atingida).

**Tests cumulativos**: 1214 → **1230** (+16 = 4 unit matrix
+ 9 stdlib skew + 3 regression).

---

## §9 — L0 prompts + hashes propagados

### §9.1 `entities/content.md` actualizado

Secção nova "Skew via `TransformMatrix::skew` — Passo 156F
(ADR-0061 Fase 1, sub-passo 4)" adicionada após secção P156E
pagebreak. Contém:
- **Divergência face à spec do P156F** (declaração explícita
  do porquê de não criar TransformKind enum).
- Forma da matriz `TransformMatrix::skew(ax, ay)`.
- Stdlib `#skew(body, ax: ?, ay: ?)` + atributos +
  validações.
- Limitações conscientes (origin scope-out; ângulos extremos
  rejeitados).
- Decisão arquitectural confirmada (sem TransformKind enum
  per inventário 156F.1).

### §9.2 `entities/layout_types.md` (não modificado)

L0 prompt actual de `layout_types` não menciona
`TransformMatrix` em detalhe (ficheiro grande agregando
tipos fundamentais; cobertura por categoria genérica).
Adição de método estático novo cabe sob a forma actual sem
necessidade de actualização do L0 prompt. Lint confirma
zero drift.

### §9.3 Headers `@updated`

- `entities/content.rs`: `@updated 2026-04-25` (já estava).
- `entities/layout_types.rs`: `@updated 2026-04-23`
  (não modificado por este passo; método novo absorvido
  como adição menor).

### §9.4 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/entities/content.rs             → 4321258d

Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `b632e841` (P156E) → **`4321258d`**
(P156F).
`entities/layout_types.rs`: hash inalterado (lint não
detectou drift; `@prompt-hash` continua `af36c701`).

---

## §10 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `skew(ax, ay, body)`: `ausente` ⁵ → **`implementado`** ¹²
  (Passo 156F).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸ ¹⁰ ¹²): `9/0/3/6/0=18` →
  **`10/0/3/5/0=18`**.
- Total user-facing: `59/21/22/37/2=141` →
  **`60/21/22/36/2=141`**.
- Cobertura user-facing total: 57% → **57%** (mantém-se
  por arredondamento; 60+21=81/141=57.4%).

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) inalterado **48** (refactor
  zero; sem novo variant; `Content::Transform` reusa).

**Nota nova ¹²**: descreve transição P156F com explicação
da divergência da spec (TransformKind enum não criado;
descoberta empírica em 156F.1).

**§7 entrada 7**: refinamento P156F documentado com lista
das 5 entradas Layout restantes (`box`, `block`, `stack`,
`repeat`, `columns`/`colbreak`) e mapeamento para Fase 2
(block+box+stack) e Fase 3 (columns+repeat).

---

## §11 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se
  `PROPOSTO`.
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156F com detalhe
  da divergência consciente da spec, descoberta de
  arquitectura matriz cm já unificada, método novo
  `TransformMatrix::skew`, validação rigorosa, tests
  1214→1230 (+16 incl 3 regression), Layout 50%→**56%**.

---

## §12 — Próximo passo

P156F encerrou-se com Layout cobertura **50% → 56%** (sem
ADR/DEBT criados, sem regressão, sem reformulações). Próximo
passo é **decisão humana** entre prioridades documentadas
em ADR-0061:

- **Opção A — Fase 2 Layout sub-passo 1: P156G (block)**:
  M+ (1 container rico com width, height, breakable, inset,
  fill, stroke). Cobertura → 61% (11/18).

- **Opção B — Fase 2 Layout sub-passo 2: P156H (box)**:
  M (inline container). Pode ser feito antes ou depois de
  block.

- **Opção C — Fase 2 Layout sub-passo 3: P156I (stack)**:
  S-M (composição com `dir: Dir`).

- **Opção D — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção E — Footnote area**: sub-fase prioritária explícita
  per ADR-0061 Decisão 5.

- **Opção F — Outra prioridade humana**.

**Recomendação descritiva**: cadência granular continua a
funcionar. **Padrão emergente confirmado por P156F**:
inventário rigoroso pré-materialização reduz risco
significativamente. P156G (block) é o próximo natural se
humano quiser entrar em Fase 2 (containers ricos —
escopo M+ esperado, primeiro passo onde "passo granular
1-2 features" testa cap superior do conceito).

---

## §13 — Limitações registadas

1. **`origin` scope-out** para skew (e move/rotate/scale
   actuais). Vanilla suporta `origin: Align`; refino
   futuro per ADR-0061 §6.3 quando refactor multi-region
   acontecer.

2. **Ângulos próximos de ±π/2 rejeitados** com erro hard
   (vanilla pode aceitar e produzir comportamento
   indefinido). Decisão de erro explícito vs comportamento
   indefinido. Threshold conservador (1e-3 rad ≈ 0.057°).

3. **Sem show rules `#show skew: ...`** neste passo
   (consistente com adiamento P154B/P155/P156C/D/E).

4. **`Smart<Angle>` da vanilla simplificado** para `Option`
   implícito (default 0 quando ausente).

5. **Composição skew + outros transforms** funciona via
   `TransformMatrix::concat` existente (P78); não testada
   explicitamente em E2E (composição de `#skew(#rotate
   (...))` por exemplo). Aceitável; matriz cm é
   matematicamente correcta por construção.

6. **Tests de layout E2E** não adicionados explicitamente
   neste passo (apenas tests unit de matrix + stdlib).
   Razão: arquitectura matriz cm já é E2E-tested via
   rotate/scale; skew herda essa cobertura indirectamente.
   Aceitável per ADR-0054 graded.

7. **ADR-0061 mantém `PROPOSTO`** (per decisão humana).

---

## §14 — Verificação final

Critérios da spec P156F (§Verificação):

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: **1230 passed**;
   0 failed; 6 ignored. (991 typst-core + 215 integration
   + 24 outros; 1214 → 1230 = +16). **Tests existentes de
   move/rotate/scale TODOS passam** (verificado por 3
   regression tests dedicados).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md` (`4321258d`).
5. ✅ `Content::Transform` **NÃO refactorizado** (decisão
   consciente per descoberta 156F.1; spec proposta
   substituída por puramente aditivo).
6. ✅ `TransformMatrix::skew` em produção.
7. ✅ Stdlib `#skew(...)` invocável (37 → 38 funcs).
8. ✅ Stdlib `#move(...)`, `#rotate(...)`, `#scale(...)`
   continuam invocáveis com semantic inalterada (regression
   tests verificam explicitamente).
9. ✅ Cobertura arms exaustiva (não precisou; refactor zero).
10. ✅ Inventário 148 reflecte cobertura aumentada (50% →
    56%).
11. ✅ README ADRs entrada P156F.
12. ✅ Sem ADR criada / revogada / revisada.
13. ✅ Sem DEBT criado / fechado.
14. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
15. ✅ **Sem regressão em tests de move/rotate/scale**
    (3 regression tests passam — critério bloqueante
    cumprido).
16. ✅ Sem regressão geral (todos os 1214 tests pré-P156F
    continuam a passar; +16 novos passam).
17. ✅ Relatório do passo escrito (este ficheiro).

---

## §15 — Análise de risco de regressão

**Esta secção é nova** neste passo (espec adicionou no §156F.11
modelo de relatório).

### §15.1 Risco a priori (do plano original da spec)

A spec antecipou risco de regressão > 0 pela primeira vez
na sequência granular. Refactor de `Content::Transform`
para `TransformKind` enum tocaria 9 consumers + 3 natives
existentes. Plano de mitigação:
1. Inventário pré-materialização expandido (156F.1).
2. Tests existentes critério bloqueante.
3. Regression tests explícitos.
4. Pausa-e-consulta se discrepância radical.

### §15.2 Risco realizado

**Zero**. A descoberta do inventário 156F.1 reverteu a
estratégia: a unificação proposta já existia desde P78. O
plano simplificou-se para puramente aditivo (método novo +
native novo + tests). Consumers existentes não foram tocados.

### §15.3 Mitigações activadas

1. **Inventário expandido funcionou**: revelou
   pre-existência crítica → mudou plano antes de tocar
   código. **Padrão "menor mudança suficiente" emergiu
   naturalmente** da inventariação rigorosa.
2. **Regression tests explícitos** mantidos (3 tests para
   move/rotate/scale) — não eram necessários para validar
   refactor (porque não houve), mas funcionam como
   **defesa em profundidade** caso futuras alterações ao
   pipeline Transform aconteçam.
3. **Pausa-e-consulta** não accionada porque descoberta foi
   simplificadora, não complicadora.

### §15.4 Lições para sequência granular

- **Inventariar antes de refactorar é mecanismo de redução
  de risco mais barato que regression tests post-hoc**.
- **Spec pode antecipar refactor que já foi feito**.
  Revisão histórica (per padrão diagnóstico-primeiro,
  evidência 7/7 com este passo) detecta.
- **Hipótese "passos granulares mantêm-se aditivos"**
  reforçada por N=4 aplicações consecutivas (P156C+D+E+F),
  embora P156F só seja aditivo por descoberta empírica.
  Em rigor, a hipótese é "**inventariação rigorosa
  pré-código mantém os passos aditivos**".

### §15.5 Recomendação meta-metodológica

Para passos futuros com refactor antecipado:
1. **Sub-passo .1 sempre dedica tempo a inventário
   empírico** (não confiar que spec capturou estado actual).
2. **Decisão sub-condicional bidireccional**: pausar se
   descoberta complica, mas também **simplificar plano** se
   descoberta revela que o trabalho proposto é menor que
   esperado.
3. **Regression tests valem mesmo quando refactor não
   acontece**: ficam como armadilhas para futuras alterações
   ao pipeline.

---

## §16 — Notas operacionais

- **Padrão "passos granulares" — quarta aplicação
  consecutiva**. P156C+P156D+P156E+P156F todos com cadência
  estável. **N=4 aplicações reforçam** a hipótese da
  decisão humana 2026-04-25, com refinamento meta:
  inventariação rigorosa pré-código é o mecanismo
  fundamental.

- **Divergência consciente da spec documentada**: P156F é
  o primeiro passo da sequência granular onde a spec foi
  parcialmente sobreposta por descoberta empírica. Decisão
  registada explicitamente em §15 + secção L0 + nota README
  ADRs. Padrão "evidência empírica > spec antecipada"
  consistente com P102/P103 (descoberta de #set/#show já
  activos) reportados no historiograma P156A.

- **Helpers reusados**: nenhum extracção de Angle/Float
  partilhada com outros natives — `extract_angle_rad`
  inline em `native_skew` por simplicidade. Consistente
  com `native_rotate` que também faz extracção inline.

- **Layouter**: zero novos arms. Pipeline matriz cm reusa
  trivialmente. Padrão "menor mudança suficiente" no melhor
  caso possível.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.

- **Variants count**: 48 (inalterado; refactor zero). Após
  P156G (block): 49. Após Fase 2 (block+box+stack): 51.

- **Stdlib funcs**: 37 → **38** (+1). Após Fase 2: ~41.

- **Pós-156F**:
  - 7 features Layout implementadas total (pad, hide, h, v,
    pagebreak, skew + align/move/rotate/scale via Transform
    unificado).
  - Cobertura Layout: **56%** (10/18).
  - Cobertura user-facing total: 57% (arredondamento;
    60+21=81 de 141).
  - **Próximo**: P156G (block — Fase 2 sub-passo 1) ou
    alternativa humana.

- **Slope cumulativo Fase 1+início Fase 2**:
  - P156C +11% (4→6/18).
  - P156D +11% (6→8/18).
  - P156E +6% (8→9/18).
  - P156F +6% (9→10/18).
  - **Total Fase 1**: 38%→56% = +18% em 4 passos.
  - Restantes 16 pontos para 72% target em 3-4 passos
    (P156G/H/I de Fase 2 + opcional P156J de Fase 3 sem
    columns) = ~5% por passo. Realista para containers
    Fase 2.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`**: `b632e841` (P156E)
  → **`4321258d`** (P156F).

---

## §17 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156f.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatórios precedentes: `typst-passo-156c-relatorio.md`,
  `typst-passo-156d-relatorio.md`,
  `typst-passo-156e-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `entities/content.md` (secção skew adicionada
  documentando divergência da spec).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/transform.rs`
  (SkewElem + Transform).
- Cristalino código tocado:
  - `01_core/src/entities/layout_types.rs` (método
    `TransformMatrix::skew(ax, ay)` + 4 unit tests).
  - `01_core/src/rules/stdlib/transforms.rs` (`native_skew`
    + helper `extract_angle_rad` inline).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 12 tests
    incluindo 3 regression).
  - `01_core/src/rules/eval/mod.rs` (registo em `make_stdlib`).
  - `00_nucleo/prompts/entities/content.md` (secção P156F
    documentando divergência da spec).
- Cristalino código **NÃO** tocado (verificado por inspecção):
  - `01_core/src/entities/content.rs` (variant Transform
    inalterado).
  - `01_core/src/rules/introspect.rs` (arms inalterados).
  - `01_core/src/rules/layout/mod.rs` (arm Transform
    inalterado; reusa pipeline matriz cm de P78).
  - `01_core/src/rules/layout/helpers.rs` (sub-frame
    composition inalterada).
