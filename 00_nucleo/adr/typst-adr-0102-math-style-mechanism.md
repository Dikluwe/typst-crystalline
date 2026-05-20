# ⚖️ ADR-0102: Mecanismo math style — variant `Content::MathStyled` (Caminho I)

**Status**: `EM VIGOR`
**Data**: 2026-05-20
**Passo promotor**: P311b.2 + P311b.4 (materialização)
**Diagnóstico**: P311a (`diagnosticos/diagnostico-math-style-passo-311a.md`)
**Categoria**: Arquitectural / Mecanismo cross-variant Content
**Cross-ref**: ADR-0026 (Content como enum fechado),
              ADR-0033 (paridade observable),
              ADR-0040 (StyleChain — relacionado mas rejeitado),
              ADR-0054 (perfil graded),
              ADR-0098 (single source of truth)

---

## Contexto

P311 implementa 12 funções math style vanilla (`bb`/`bold`/`cal`/
`frak`/`italic`/`mono`/`sans`/`scr`/`script`/`serif`/`sscript`/
`upright`). O diagnóstico P311a §3.1 avaliou três caminhos
arquitectónicos:

- **Caminho I** — Variant `Content::MathStyled` dedicado.
- **Caminho II** — Mapping Unicode eager nas funções nativas.
- **Caminho III** — Style enum extension (StyleChain).

Esta ADR formaliza a escolha do **Caminho I** e documenta porquê os
outros foram rejeitados, para que futuras decisões cross-variant
math (P296-P298 + P311b) tenham precedente arquitectónico explícito.

---

## Decisão

P311b.2 adiciona `Content::MathStyled` como variant 25º de
`Content`:

```rust
Content::MathStyled {
    kind:    Option<MathStyleKind>,  // None = inherit; Some = override
    bold:    Option<bool>,           // idem
    italic:  Option<bool>,           // idem
    body:    Box<Content>,
    cramped: Option<bool>,           // script/sscript context
}
```

`MathStyleKind` (em `entities/math_style.rs`, P311b.1) lista 7
variants glyph + 2 variants size:

```rust
pub enum MathStyleKind {
    Plain, SansSerif, Chancery, Roundhand, Fraktur, Monospace,
    DoubleStruck,     // glyph variants
    Script, SScript,  // size variants (factor 0.7 / 0.5)
}
```

P311b.4 adiciona handler `Content::MathStyled` em
`MathLayouter::layout_node` + função `apply_math_style` que aplica
recursivamente substituição char-by-char via `map_glyph` antes de
delegar ao layout do body transformado.

---

## Caminho I — Variant `Content::MathStyled` (escolhido)

### Vantagens

1. **Composicional natural**: `bb(cal(x))` representa-se como
   `MathStyled { kind: Some(DS), body: MathStyled { kind: Some(Ch),
   body: ... } }`. Aninhamento mapeia trivialmente para nesting
   sintáctico user-facing.
2. **Funciona com bindings**: `bb(x)` onde `x` é `MathIdent` ou
   binding eval-time wrap funciona — o variant preserva o body como
   `Box<Content>` e a transformação acontece em layout (após eval).
3. **Paralelo arquitectónico** directo a P298 (`Content::MathOp`)
   + outros variants math agregados (MathFrac/MathAttach/etc.).
   Pattern cumulativo "variant math agregado" consolidado.
4. **Composição outer-wins natural** via `Option::or` em
   `apply_math_style`: outer set ganha sobre inner se ambos têm
   valor; inner herda se outer é `None`.
5. **`Option<_>` em todos os fields override** distingue "outer não
   overriding este campo" de "outer força este valor". Permite
   `bold(bb(x))` preservar inner Bb enquanto aplica bold orthogonal.

### Custos

1. **Hash `content.rs` quebra** — sequência 27 passos consecutivos
   com hash `82d3c47d` termina em P311b.2 (último drift: P282).
   **Aceitável e necessário** per ADR-0033 (paridade observable;
   forma diverge sem afectar output PDF).
2. **+1 variant** em `Content` (24 → 25). 5 sítios exhaustive match
   adicionados (plain_text, PartialEq, map_content, map_text +
   3 fora de content.rs).
3. **+2 ADRs novas** (esta + ADR-0103 Composition).

---

## Caminho II — Mapping Unicode eager (rejeitado)

### Proposta

Cada função nativa substitui ASCII por codepoint variant
imediatamente:

```rust
fn native_bb(args: &Args) -> SourceResult<Value> {
    let body: Content = args.first_content();
    let new_body = walk_replace_chars(body, MathStyleKind::DoubleStruck);
    Ok(Value::Content(new_body))
}
```

### Razão de rejeição

1. **Falha em bindings**: `bb(x)` onde `x = some_expr` (binding
   eval-time) **não funciona** — a substituição ASCII só opera em
   `MathIdent`/`MathText` literais. Se o body é uma expressão que
   resolve a `MathIdent` em runtime, o walk eager não vê o conteúdo
   resolvido.
2. **Não-composicional**: `bb(cal(x))` requereria re-mapeamento de
   codepoints já transformados pelo inner — destruindo informação.
   Outer Bb veria `cal(x)` já com chars Script Unicode encoded e não
   teria como inferir intent original.
3. **Limitado a strings literais**: práticamente inútil para casos
   de uso reais que usam variáveis (`bb(P)` onde `P` é definido em
   `#let`).

---

## Caminho III — Style enum extension (rejeitado, adiado)

### Proposta

```rust
pub enum Style {
    // ... existentes
    MathVariant(MathStyleKind),
    MathBold(bool),
    MathItalic(Option<bool>),
}

// Aplicado via Content::Styled(body, [MathVariant(Bb)])
```

### Razão de rejeição/adiamento

1. **Anti-padrão "capture sem consumer"**: `StyleChain` cristalina é
   **plana** (ADR-0040; DEBT-1 ainda não materializado). Adicionar
   fields ao `TextStyle` sem `MathLayouter` consumer real seria
   capturar variant em chain sem honrar — estado intermédio, não
   solução.
2. **Acopla com DEBT-1**: P266 §"capture sem consumer" identifica
   este padrão como bloqueador. Materializar StyleChain real é
   refactor M+ separado; subordinar P311 a DEBT-1 ampliaria escopo
   significativamente.
3. **Não é definitivo rejeitado** — se DEBT-1 for materializado no
   futuro, `MathStyled` pode migrar para Style merge (paridade
   vanilla). ADR-0102 não bloqueia esta migração futura; documenta
   apenas que **enquanto StyleChain estiver plana**, Caminho I é a
   única opção viável.

---

## Consequências

### Imediatas (P311b)

1. Hash `entities/content.rs` quebra (28º consecutivo termina).
   Documentado deliberadamente em L0 `entities/content.md` §"Hash
   content.rs quebra deliberadamente".
2. `Content::MathStyled` agora existe — 12 funções stdlib P311b.3
   produzem-no; `MathLayouter` consume-o via handler P311b.4.
3. 12/12 = 100% paridade categoria math style (segunda categoria
   stdlib cristalina a fechar após calc 41/41 em P308).

### Futuras (pós-P311b)

1. **DEBT-1 StyleChain real**: quando materializado, considerar
   migração `MathStyled` → `Style::MathVariant` (ADR follow-up).
2. **Greek + dígitos completos**: cobertura inicial só Latin ASCII.
   Greek capitals (Α-Ω) + `∇`/`∂` em sub-passo dedicado se
   cobertura empírica exigir.
3. **`display`/`inline` (2 funções extras vanilla)**: fora escopo
   P311; sub-passo futuro se cobertura A.7 ampliada.

---

## Composição (ADR-0103)

A regra de composição entre wraps aninhados é tema separado,
formalizada em ADR-0103 (Math-Style-Composition). Resumo:

- Variant glyph (`kind`): outer-wins via `Option::or`.
- Bold flag: outer-wins (via Option::or; bold é capturado por
  field separado, não conflita com kind).
- Italic flag: outer-wins via `Option::or` (também `Option<bool>`).
- Size variant: outer-wins (não multiplicativo — refuta diagnóstico
  P311a §3.5 cuja recomendação foi corrigida em P311b.4).

---

## Não-objectivos desta ADR

- Não descreve mapping Unicode (esse é em `entities/math_style.rs`
  + L0 dedicado).
- Não fixa política para casos exóticos vanilla (Arabic math
  variants, etc.) — fora escopo P311.
- Não obriga StyleChain real ser materializada — apenas documenta
  porque é precondição para Caminho III.

---

## Referências

- **Diagnóstico P311a**: `00_nucleo/diagnosticos/diagnostico-math-
  style-passo-311a.md` §3.1 (matriz comparativa 3 caminhos).
- **P311b.1**: `entities/math_style.rs` + L0 dedicado.
- **P311b.2**: variant `MathStyled` adicionado em `entities/content.rs`.
- **P311b.3**: 12 funções stdlib em `rules/stdlib/math_style.rs`.
- **P311b.4**: handler + `apply_math_style` em `rules/math/layout/mod.rs`.
- **ADR-0026** — Content como enum fechado (justifica variant novo).
- **ADR-0033** — Paridade observable (math style preserva paridade
  vanilla; forma diverge).
- **ADR-0040** — StyleChain plano (DEBT-1; razão de rejeição
  Caminho III).
- **ADR-0054** — Perfil graded.
- **ADR-0093** — Meta-metodologia evolução ADRs.
- **ADR-0098** — Single source of truth (variant em content.rs como
  SSOT math style).
- **ADR-0103** — Math-Style-Composition (regras de composição;
  formalizado em separado).
