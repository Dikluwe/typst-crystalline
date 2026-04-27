# Diagnóstico `Content::TableHeader` + `Content::TableFooter` — Passo P157C

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
terceiro e último sub-passo Model Fase 2 declarada em
ADR-0060 §"Decisão 1" sub-passo 3. **Décima terceira aplicação
consecutiva** do padrão diagnóstico-primeiro.

**Primeira aplicação concreta de ADR-0064 Caso D em domínio
Model** (P156D weak / P156G breakable / P156J justify aplicaram-no
em Layout). Patamar empírico ADR-0064 atinge **saturação
cross-domínio cross-caso** após P157C — Casos A, B, C, D
todos validados em Layout E em Model.

---

## 1. Assinatura vanilla `TableHeader` e `TableFooter`

Fonte: `lab/typst-original/.../model/table.rs:497-536`.

### 1.1 `TableHeader` (vanilla)

```rust
#[elem(name = "header", title = "Table Header")]
pub struct TableHeader {
    #[default(true)]
    pub repeat: bool,
    #[default(NonZeroU32::ONE)]
    pub level: NonZeroU32,
    #[variadic]
    pub children: Vec<TableItem>,
}
```

### 1.2 `TableFooter` (vanilla)

```rust
#[elem(name = "footer", title = "Table Footer")]
pub struct TableFooter {
    #[default(true)]
    pub repeat: bool,
    #[variadic]
    pub children: Vec<TableItem>,
}
```

### 1.3 Subset minimal P157C

| Field vanilla | Header | Footer | Tradução cristalina P157C | Caso ADR-0064 |
|---------------|:------:|:------:|---------------------------|:--------------:|
| `repeat: bool` (default true) | ✓ | ✓ | `bool` directo (default `true`) | **D** |
| `level: NonZeroU32` (default 1) | ✓ | ✗ (não existe em Footer) | **diferido** | (futuro Caso C) |
| `children: Vec<TableItem>` (variadic) | ✓ | ✓ | **divergência cristalina**: `body: Box<Content>` per spec P157C | — |

**Subset diferido P157C** (per ADR-0054 graded):
- `level` (Header apenas): hierarquia de headers aninhados
  diferida — refino M futuro per ADR-0064 Caso A se materializado
  com `Smart<NonZeroU32>` (paridade P157B `x`/`y`).
- `repeat-rows: Smart<usize>` (vanilla suporta repetir só N
  linhas em vez de header inteiro): diferido per ADR-0054 graded.
- Algoritmo de repetição em page breaks: diferido per DEBT-56
  (multi-region).
- Children variádicos estruturados (TableItem enum): diferido —
  cristalino usa `body: Box<Content>` simplificado.

### 1.4 Divergência intencional vs vanilla — `body` em vez de `children`

Spec P157C declara `body: Box<Content>` (single child via Box,
paridade P156H Boxed / P156J Repeat / P157B TableCell). Vanilla
usa `#[variadic] children: Vec<TableItem>`.

**Decisão**: seguir spec — `body: Box<Content>` per uniformidade
com containers cristalinos existentes. Justificação:
- Cristalino não tem TableItem enum; reusar Sequence se múltiplos
  children forem necessários (`body = Sequence(vec![...])`).
- Paridade absoluta TableHeader↔TableFooter no field name e tipo.
- Simplifica pattern-match em todos os 9 sítios.

**Divergência aceite per ADR-0033** (paridade observável estrutural).

---

## 2. Comportamento observável (subset minimal)

**Vanilla**:
- `table.header[content]` renderiza `content` no topo da table;
  com `repeat=true` (default) reaparece em cada page break.
- `table.footer[content]` renderiza no fim; idem para repeat.
- Ordem semântica: header > cells > footer (footer é semanticamente
  último, não físico).

**Cristalino P157C** (per ADR-0054 graded):
- ✓ Variants existem; field `repeat` armazenado.
- ✓ `body` renderiza no contexto actual (single render).
- ✗ Sem repetição em page breaks (`repeat` armazenado mas
  `layout_grid` não consulta — DEBT-56).
- ✗ Sem ordem semântica obrigatória header/cells/footer dentro
  de Table (children são lineares per `Content::Table` P157A).

**Divergência aceite per ADR-0033 + ADR-0054**:
- Comportamento observável de P157C difere de vanilla quando
  `repeat=true` ou quando ordem header/footer é semântica.
- Tests E2E focam **renderização do body**, NÃO ordem semântica
  ou comportamento de page breaks.

---

## 3. ADR-0064 caso aplicável

### 3.1 `repeat: bool` — Caso D (primeira aplicação Model)

Vanilla `bool` com default `true` (não-`false`). Cristalino
traduz para `bool` directo (não Option) com documentação
explícita do default.

**Auto-validação cumulativa de ADR-0064 P156K — Caso D**:
- N=4 aplicação concreta de Caso D (P156D weak default false +
  P156G breakable default true + P156J justify default true +
  **P157C TableHeader/TableFooter.repeat default true**).
- **Primeira aplicação Model**. ADR-0064 Caso D ganha
  diversidade cross-domínio empírica.

### 3.2 Tabela de patamares ADR-0064 pós-P157C

| Caso | Pré-P157C | Pós-P157C |
|------|----------:|----------:|
| A | N=4 (75% Layout + 25% Model) | N=4 (inalterado) |
| B | N=1 (P156I Dir) | N=1 (inalterado) |
| C | N=3 (P156I/J + P157B; primeira variação `usize`) | N=3 (inalterado) |
| D | N=3 (Layout 100%) | **N=4** (75% Layout + 25% Model — primeira Model) |

**Saturação cross-domínio cross-caso**: após P157C, **todos os
4 casos canónicos** (A/B/C/D) têm pelo menos 1 aplicação concreta
em Layout E em Model. Padrão empírico atinge maturidade.
Candidato a passo administrativo XS futuro de "ADR-0064 caso
completion".

---

## 4. Variants Content existentes a estender

**Nenhuma**. `Content::TableHeader` e `Content::TableFooter` são
variants novos — sem encaixe em variants existentes. Análogos
estruturais a P156D HSpace/VSpace (par simétrico) e P157B
TableCell (variant rico).

---

## 5. Helpers stdlib reusáveis

### 5.1 Verificação de helpers existentes

Inspecção de `01_core/src/rules/stdlib/`:

- **`extract_weak`** em `stdlib/layout.rs:342`: **NÃO reusável**
  — específico para key `"weak"` com default `false`. P157C
  precisa de key `"repeat"` com default `true`.
- **`extract_length`**, **`extract_tracks`**: irrelevantes
  (P157C não consome Length nem TrackSizing).
- **`extract_usize_or_none_min`** (P157B): irrelevante (P157C
  consome `bool`, não `usize`).
- **`extract_dir`**, **`extract_parity`**: irrelevantes.

### 5.2 Helper novo `extract_bool_with_default`

Helper privado em `stdlib/structural.rs`:

```rust
/// Coage `Value` para `bool` com default específico per ADR-0064
/// Caso D (vanilla `bool` com default não-`false`; cristalino
/// usa `bool` directo com documentação explícita do default).
///
/// `Value::Bool(b)` → `b`.
/// `Value::None` ou ausência → `default`.
/// Outros tipos → erro hard com diagnóstico claro.
///
/// Helper privado P157C; param `default` permite reuso para
/// `repeat` (default true) e futuros bool fields com defaults
/// arbitrários (e.g. `caption-position` em P158).
fn extract_bool_with_default(
    args: &Args,
    fn_name: &str,
    field: &str,
    default: bool,
) -> SourceResult<bool>
```

**Reuso N=2 imediato no mesmo passo** (TableHeader.repeat +
TableFooter.repeat). Subpadrão emergente análogo a
`extract_usize_or_none_min` em P157B (parametrizado para
combinar usos).

**Promoção a `pub(super)` ou helper público**: diferida até
N=3-4 reuso noutros passos (e.g. P158 figure-kinds com
`caption-position` se aplicável). Política consistente.

**Decisão arquitectural alternativa rejeitada**: generalizar
`extract_weak` adicionando param `key` e `default`. Rejeitada
porque renomearia o helper existente (breaking change interno)
e mistura concerns (Layout vs Model). Helper novo dedicado
preserva separação de domínios per ADR-0037.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P157C | Refino futuro |
|---------|--------------|---------------|
| Algoritmo repetição header/footer em page breaks | ✗ scope-out | **DEBT-56** (refactor multi-region; column flow + repeat) |
| `level: NonZeroU32` (Header hierarquia) | ✗ scope-out | refino M futuro per ADR-0064 Caso A |
| `repeat-rows: Smart<usize>` (Header) | ✗ scope-out | refino futuro per ADR-0064 Caso A |
| Children variádicos estruturados (TableItem enum) | ✗ scope-out | divergência aceite — cristalino usa `body: Box<Content>` |
| Ordem semântica header > cells > footer | ✗ scope-out | depende algoritmo Grid completo (DEBT-34e + DEBT-56) |
| Variant existe + field `repeat` armazenado | ✓ implementado | — |
| Stdlib func + validações | ✓ implementado | — |

**DEBT-56 permanece aberto após P157C**. P157C contribui para
fechamento futuro armazenando `repeat: bool` necessário ao
algoritmo, mas não fecha por si.

---

## 7. Tests planeados

### 7.1 Unit tests `Content::TableHeader` + `Content::TableFooter` (~12 = 6 pares)

Em `entities/content.rs`:
1. Constructor default (`repeat=true`).
2. Constructor com `repeat=false`.
3. `is_empty()` proxy via body.
4. `plain_text()` recurse no body.
5. `PartialEq` cobertura (body + repeat).
6. `map_text` recurse + preserva `repeat`.

**Cada um repetido para TableFooter** — total ~12 tests.

### 7.2 Stdlib tests (~6-8 = 3-4 pares)

Em `stdlib/mod.rs`:
1. Defaults (`repeat=true`).
2. `repeat=false` explícito.
3. Sem body rejeitado.
4. `repeat=int` rejeitado.
5. Named arg desconhecido rejeitado.

**Cada um repetido para `native_table_footer`**.

### 7.3 Layout E2E tests (~3 = 1 par + integrativo)

Em `layout/tests.rs`:
1. `layout_table_header_renderiza_body_no_contexto_actual`.
2. `layout_table_footer_renderiza_body_no_contexto_actual`.
3. `layout_table_com_header_cell_footer_renderiza_tudo`
   (integrativo — Table com Header + Cell + Footer).

**Δ esperado**: +18 a +23 tests (range alinhado com esboço).

---

## 8. Confirmação de paridade interna TableHeader↔TableFooter

### 8.1 Paridade absoluta esperada

- ✓ Mesmos fields: `body: Box<Content>` + `repeat: bool`.
- ✓ Mesmos pattern-match em 9 sítios estruturais.
- ✓ Mesmo construtor signature `Content::table_header(body,
  repeat)` / `Content::table_footer(body, repeat)`.
- ✓ Mesmas stdlib funcs: assinatura idêntica excepto naming.
- ✓ Mesmos tests excepto naming.

### 8.2 Divergência aceite vanilla (não simétrica)

- `level: NonZeroU32` existe em Header mas **não em Footer**
  (vanilla). Em P157C, **ambos diferem este field** (paridade
  cristalina absoluta). Refino futuro pode adicionar `level`
  só a Header se necessário, quebrando paridade — documentado.

### 8.3 Tratamento simétrico em pattern-match

Cada sítio pattern-match deve listar TableHeader **imediatamente
antes** de TableFooter para tornar paridade visualmente óbvia:

```rust
Self::TableHeader { body, .. } => body.is_empty(),
Self::TableFooter { body, .. } => body.is_empty(),
```

---

## 9. Confirmação de naming flat (paridade P157B)

Decisão: `table_header` e `table_footer` flat (snake_case).

**Justificação per P157B §8** (FieldAccess actual cristalino
não suporta `Value::Func.subname`):
- Vanilla usa `table.header` / `table.footer` (com ponto).
- Cristalino usa flat per limitação técnica documentada em
  diagnóstico P157B.
- Refactor futuro pode adicionar alias `table.header` /
  `table.footer` sem breaking change.

**Divergência intencional vs vanilla** per ADR-0033 (paridade
observável estrutural preservada). Consistente com decisão
P157B `table_cell` flat.

---

## Resumo executivo

P157C materializa **par simétrico** `Content::TableHeader` +
`Content::TableFooter`:
- Variants `TableHeader { body, repeat }` e `TableFooter { body,
  repeat }` com 2 fields cada.
- Field `repeat: bool` ADR-0064 Caso D (primeira aplicação Model).
- Stdlib `native_table_header` e `native_table_footer` em
  `stdlib/structural.rs` (continuação P157A/B); naming flat
  `table_header`/`table_footer` per padrão P157B.
- Helper privado novo `extract_bool_with_default(args, fn,
  field, default)` para parse de `repeat` (paridade
  parametrizada análoga a `extract_usize_or_none_min` em P157B).
- Layouter renderiza body simples; **`repeat` armazenado mas
  ignorado em layout** per ADR-0054 graded — **DEBT-56**
  permanece aberto.

**Decisões arquitecturais P157C**:
- **Field `body: Box<Content>`** (não vanilla `children: Vec<TableItem>`):
  divergência intencional para uniformidade com containers
  cristalinos existentes per ADR-0033.
- **Paridade absoluta TableHeader↔TableFooter**: ambos diferem
  `level` (que vanilla tem só em Header).
- **Helper parametrizado `extract_bool_with_default`**: combina
  parse de bool com default arbitrário (vs `extract_weak`
  específico para key="weak" default=false).
- **Naming flat** `table_header`/`table_footer` per P157B.

**Decisões diferidas**:
- `level` (Header hierarquia): refino M futuro.
- `repeat-rows`: refino futuro.
- Algoritmo repetição em page breaks: **DEBT-56**.
- Children variádicos estruturados: divergência aceite.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido (este doc).
- ADR-0054: graded scope-out de algoritmo repetição + level +
  repeat-rows + children estruturados.
- ADR-0060: variants novos per Decisão 4 (Model Fase 2 sub-passo 3
  — **fecha "table foundations" declarado**).
- ADR-0064: **Caso D primeira aplicação Model** (cross-domínio).
  Após P157C, **todos os 4 casos canónicos** validados em Layout
  E em Model — saturação cross-domínio cross-caso.
- ADR-0065: critério #1 (naming flat) implícito + critério #6
  (divergência body vs children) explícito.

**Tests planeados**: Δ +18-23.

**Risco**: baixo. Mitigação: par simétrico aditivo análogo a
P156C/D; ADR-0064 Caso D já validado em Layout; sem nova
decisão arquitectural-chave (decisões reusadas de P157A/B).
