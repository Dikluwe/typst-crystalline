# Passo 26 — Correcções arquitecturais (ADR-0029, ADR-0030, ADR-0031)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — `Length`, `Ratio`, `Angle`, `Color` do Passo 25
- `01_core/src/entities/content.rs` — `Content::Sequence(Vec<Content>)`
- `01_core/src/entities/source.rs` — `Source` sem `Hash`/`Eq`
- `00_nucleo/adr/typst-adr-0029-*.md` — pureza física, revoga ADR-0028
- `00_nucleo/adr/typst-adr-0030-*.md` — performance é domínio
- `00_nucleo/adr/typst-adr-0031-*.md` — early hashing em Source
- `00_nucleo/adr/typst-adr-0026-revisao-*.md` — Content::Sequence com Arc

Pré-condição: `cargo test` — 341 testes, zero violations.

**Objectivo**: alinhar o código com as três ADRs aprovadas sem adicionar
funcionalidade nova. Este é um passo de correcção estrutural — cada
tarefa tem critério de correcção binário (passa/falha), não incremental.

**Fronteira**: não iniciar DEBT-4 (funções nativas de conversão e cálculo)
neste passo. As correcções devem ser atómicas e deixar os testes verdes.

---

## Tarefa 1 — Diagnóstico das estruturas vanilla (ADR-0029)

Antes de qualquer código, confirmar a estrutura real de `Length` e `Abs`
no Typst vanilla. A ADR-0029 exige que não se simplifique sem verificar.

```bash
# Estrutura interna de Length no original
grep -rA 20 "^pub struct Length\b" \
  lab/typst-original/crates/typst-library/src/layout/ 2>/dev/null | head -30

# Estrutura de Abs — newtype ou struct com campos?
grep -rA 15 "^pub struct Abs\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -20

# Rel — struct com ratio e length?
grep -rA 15 "^pub struct Rel\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -20

# Ratio — confirmar se é newtype f64 ou tem campos
grep -rA 10 "^pub struct Ratio\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -15

# Angle — confirmar unidade interna (radianos ou graus?)
grep -rA 10 "^pub struct Angle\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -15

# Color — enum ou struct com discriminante?
grep -rA 25 "^pub enum Color\b\|^pub struct Color\b" \
  lab/typst-original/crates/typst-library/src/visualize/ 2>/dev/null | head -30

# Confirmar que estas structs não usam I/O de sistema
grep -rn "std::fs\|std::net\|std::env\|SystemTime" \
  lab/typst-original/crates/typst-library/src/layout/length.rs \
  lab/typst-original/crates/typst-library/src/layout/angle.rs \
  2>/dev/null | head -10
```

**Parar. Reportar output antes de qualquer código.**

Questões a responder:
1. `Length` vanilla — tem dois campos `abs: Abs` e `em: f64`, ou outra estrutura?
2. `Abs` — é `Abs(f64)` newtype (unidade: pontos? unidades raw internas?) ou tem escala?
3. `Ratio` — é `Ratio(f64)` onde 1.0 = 100%, ou tem escala diferente?
4. `Angle` — usa radianos ou graus internamente?
5. `Color` — é enum com variantes por espaço de cor, ou struct com discriminante interno?

---

## Tarefa 2 — Corrigir representações se diferem do vanilla (ADR-0029)

Com base no diagnóstico, corrigir apenas onde a representação cristalina
difere materialmente da vanilla **e** a diferença é de estrutura de dados
(não de operações que requerem StyleChain).

### Regra de decisão por tipo

Para cada tipo, responder: "A representação cristalina é fiel à vanilla?"

- **Sim** → nenhuma alteração necessária neste passo
- **Não, mas a diferença é só de operações** (ex: somas mistas requerem Relative) → nenhuma alteração neste passo, registar em DEBT.md para o Passo 30
- **Não, a estrutura de dados é diferente** → corrigir agora

### Correcção esperada para `Length` (se vanilla usa `abs + em`)

Se o diagnóstico confirmar que `Length` vanilla é `struct Length { abs: Abs, em: f64 }`:

```rust
// Substituir o enum actual:
// enum Length { Pt(f64), Em(f64) }  ← ADR-0028, revogada

// Por estrutura fiel ao vanilla:
/// Comprimento tipográfico — combinação de componente absoluta e relativa.
///
/// Estrutura fiel ao Typst vanilla (ADR-0029 — revoga ADR-0028).
/// `abs`: componente absoluta em pontos tipográficos.
/// `em`: componente relativa em múltiplos do font-size actual.
///
/// Operações que combinam abs e em (ex: `1pt + 1em`) são representáveis
/// como valor. A *resolução* para pt absoluto requer font-size e acontece
/// no Layouter (L3 side), não em L1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub abs: Abs,    // componente absoluta
    pub em:  f64,    // componente em (múltiplo do font-size)
}

impl Length {
    pub const ZERO: Self = Self { abs: Abs::ZERO, em: 0.0 };

    pub fn pt(v: f64) -> Self { Self { abs: Abs::pt(v), em: 0.0 } }
    pub fn em(v: f64) -> Self { Self { abs: Abs::ZERO,  em: v   } }

    pub fn is_zero(&self) -> bool {
        self.abs.is_zero() && self.em == 0.0
    }

    /// Resolve para pontos dado um font-size em pt.
    /// `1pt + 1em` com font_size=12.0 → 13.0pt
    pub fn resolve_pt(&self, font_size_pt: f64) -> f64 {
        self.abs.to_pt() + self.em * font_size_pt
    }
}

impl std::ops::Add for Length {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { abs: self.abs + rhs.abs, em: self.em + rhs.em }
    }
}

impl std::ops::Neg for Length {
    type Output = Self;
    fn neg(self) -> Self {
        Self { abs: -self.abs, em: -self.em }
    }
}
```

E o tipo `Abs` subjacente (se vanilla o tem separado):

```rust
/// Comprimento absoluto em unidades raw internas (escala a confirmar no diagnóstico).
/// Tipicamente: 1pt = 72000 unidades raw, ou simplesmente 1pt = 1.0f64.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Abs(f64);  // unidade interna: confirmar no diagnóstico

impl Abs {
    pub const ZERO: Self = Self(0.0);

    pub fn pt(v: f64) -> Self { Self(v) }         // ajustar escala se necessário
    pub fn to_pt(self) -> f64 { self.0 }          // ajustar escala se necessário
    pub fn is_zero(self) -> bool { self.0 == 0.0 }
}

impl std::ops::Add for Abs {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl std::ops::Neg for Abs {
    type Output = Self;
    fn neg(self) -> Self { Self(-self.0) }
}
```

**Nota**: se o diagnóstico revelar que `Length` vanilla já é `enum { Pt(f64), Em(f64) }`
ou que a nossa representação é fiel, **não alterar**. Documentar no relatório.

### Se `Ratio`, `Angle`, `Color` estão fiéis ao vanilla

Não alterar. Documentar no relatório que a verificação foi feita e a
representação está correcta.

### Actualizar `Value::Length` se `Length` for alterado

Se `Length` mudar de enum para struct, actualizar:
- `Value::Length(Length)` — a variante não muda de nome
- `eval_binary_op` — a soma `Pt + Em` deixa de ser Err e passa a ser
  `Length { abs: x, em: y }` correcto
- `stdlib` — `native_rgb`, `native_luma` não são afectados
- Testes que usam `Length::pt(...)` ou `Length::em(...)` — os construtores mantêm-se

---

## Tarefa 3 — Content::Sequence com Arc (ADR-0026 revisão)

Migrar `Content::Sequence(Vec<Content>)` para `Content::Sequence(Arc<[Content]>)`.

### Diagnóstico antes da mudança

```bash
# Quantos usos de Content::Sequence existem no código
grep -rn "Sequence\b" 01_core/src/ | grep -v "test\|//\|\.md" | head -20

# Como sequence() é construído
grep -n "fn sequence\|Sequence(" 01_core/src/entities/content.rs | head -10
```

### Implementação

```rust
// Em content.rs — alterar a variante
pub enum Content {
    // ...
    Sequence(Arc<[Content]>),  // ADR-0026 revisão — clone O(1), não O(n)
    // ...
}

// Actualizar o construtor
pub fn sequence(parts: Vec<Content>) -> Self {
    match parts.len() {
        0 => Self::Empty,
        1 => parts.into_iter().next().unwrap(),
        _ => Self::Sequence(parts.into()),  // Vec<Content> → Arc<[Content]>
    }
}
```

### Actualizar PartialEq

`Arc<[Content]>` com `derive(PartialEq)` compara por ponteiro, não por conteúdo.
É necessário implementação manual:

```rust
impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty,         Self::Empty)         => true,
            (Self::Text(a),       Self::Text(b))       => a == b,
            (Self::Space,         Self::Space)         => true,
            (Self::Sequence(a),   Self::Sequence(b))   => a.as_ref() == b.as_ref(),
            // ... todas as variantes existentes ...
            _ => false,
        }
    }
}
```

**Nota**: remover `#[derive(PartialEq)]` de `Content` e implementar manualmente
é obrigatório. O `derive` existente não está errado agora porque `Vec` compara
por conteúdo — mas passa a estar errado com `Arc<[...]>`.

### Actualizar plain_text() e layout

```rust
Self::Sequence(v) => v.iter().map(|c| c.plain_text()).collect(),
```

O resto de `layout_content` que itera sobre `Sequence` muda de
`.iter()` sobre `Vec` para `.iter()` sobre `Arc<[Content]>` —
a interface de iteração é idêntica.

---

## Tarefa 4 — Early hashing em Source (ADR-0031)

Adicionar `content_hash: u64` a `SourceInner` e implementar `Hash`/`Eq` em `Source`.

### Diagnóstico antes da mudança

```bash
# Estado actual de Source — tem Hash/Eq?
grep -n "Hash\|Eq\|PartialEq\|derive" \
  01_core/src/entities/source.rs | head -15

# Como Source é usado em testes e em eval_for_test
grep -rn "Source\b" 01_core/src/rules/eval.rs | head -10
grep -rn "MockWorld\|source\b" 01_core/src/contracts/ | head -10
```

### Implementação

```rust
// Em source.rs — actualizar SourceInner
struct SourceInner {
    id:           FileId,
    text:         String,
    root:         SyntaxNode,
    content_hash: u64,   // ADR-0031 — pré-computado em new(), nunca muda
}

// Actualizar Source::new()
impl Source {
    pub fn new(id: FileId, text: String) -> Self {
        use std::hash::{Hash, Hasher};
        use rustc_hash::FxHasher;

        let mut hasher = FxHasher::default();
        text.hash(&mut hasher);
        let content_hash = hasher.finish();

        let root = crate::rules::parse::parse(&text);

        Self(Arc::new(SourceInner { id, text, root, content_hash }))
    }

    /// Hash do conteúdo — O(1), pré-computado na construção (ADR-0031).
    pub fn content_hash(&self) -> u64 {
        self.0.content_hash
    }
}

// Implementar Hash e Eq baseados em content_hash
impl std::hash::Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
        self.0.content_hash.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.content_hash == other.0.content_hash
    }
}

impl Eq for Source {}
```

**Verificar que `rustc_hash` já está em `01_core/Cargo.toml`** (ADR-0018).
Se não estiver, adicionar antes de compilar.

---

## Tarefa 5 — Testes de regressão e novos testes

### Testes para Length (se estrutura mudou)

```rust
#[test]
fn length_soma_mista_agora_funciona() {
    // Com Length vanilla (abs + em), a soma Pt + Em é representável
    let l = Length::pt(6.0) + Length::em(1.0);
    // Não é Err — é um Length com ambos os componentes
    assert_eq!(l.abs.to_pt(), 6.0);
    assert_eq!(l.em, 1.0);
    // Resolve com font-size=12pt → 6 + 12 = 18pt
    assert_approx_eq!(l.resolve_pt(12.0), 18.0);
}

#[test]
fn length_zero_constante() {
    assert!(Length::ZERO.is_zero());
    assert_approx_eq!(Length::ZERO.resolve_pt(12.0), 0.0);
}

#[test]
fn length_soma_abs_preserva_em() {
    let a = Length::pt(3.0);
    let b = Length::pt(4.0);
    let sum = a + b;
    assert_approx_eq!(sum.abs.to_pt(), 7.0);
    assert_eq!(sum.em, 0.0);
}
```

### Testes para Content::Sequence com Arc

```rust
#[test]
fn sequence_clone_e_o1() {
    // Verificar que clone de Sequence não copia o conteúdo
    let seq = Content::sequence(vec![
        Content::text("a"),
        Content::text("b"),
        Content::text("c"),
    ]);
    let clone = seq.clone();
    // PartialEq por conteúdo — não por ponteiro
    assert_eq!(seq, clone);
}

#[test]
fn sequence_partialeq_por_conteudo() {
    let s1 = Content::sequence(vec![Content::text("hello")]);
    let s2 = Content::sequence(vec![Content::text("hello")]);
    // Dois Arc distintos com mesmo conteúdo → iguais
    assert_eq!(s1, s2);
}

#[test]
fn sequence_partialeq_conteudos_diferentes() {
    let s1 = Content::sequence(vec![Content::text("a")]);
    let s2 = Content::sequence(vec![Content::text("b")]);
    assert_ne!(s1, s2);
}
```

### Testes para Source com Hash/Eq

```rust
#[test]
fn source_hash_o1_apos_construcao() {
    let s = Source::detached("hello world");
    // Múltiplas chamadas retornam o mesmo hash — pré-computado
    assert_eq!(s.content_hash(), s.content_hash());
}

#[test]
fn source_eq_mesmo_conteudo() {
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let s1 = Source::new(id, "hello".into());
    let s2 = Source::new(id, "hello".into());
    assert_eq!(s1, s2);
}

#[test]
fn source_neq_conteudo_diferente() {
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let s1 = Source::new(id, "hello".into());
    let s2 = Source::new(id, "world".into());
    assert_ne!(s1, s2);
}

#[test]
fn source_pode_ser_chave_de_hashmap() {
    use rustc_hash::FxHashMap;
    let mut map: FxHashMap<Source, &str> = FxHashMap::default();
    let s = Source::detached("test");
    map.insert(s.clone(), "value");
    assert_eq!(map.get(&s), Some(&"value"));
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar que Arc não entrou como estado global (V13 deve passar)
grep -n "static.*Arc\|static.*Mutex\|static.*OnceLock" \
  01_core/src/**/*.rs && echo "VERIFICAR V13" || echo "OK"

# Confirmar que Length (se alterado) passa nos testes de eval_binary_op
cargo test -p typst-core -- eval_binary_op 2>&1 | tail -5
```

Critérios de conclusão:
- Diagnóstico das estruturas vanilla documentado no relatório ✓
- `Length` — representação fiel ao vanilla se difere do enum actual,
  ou confirmação de que enum está correcto se vanilla for igual ✓
- `Content::Sequence` usa `Arc<[Content]>` com clone O(1) ✓
- `PartialEq` de `Content` implementado manualmente (compara por conteúdo) ✓
- `Source` tem `content_hash: u64` pré-computado em `new()` ✓
- `Source` implementa `Hash`, `PartialEq`, `Eq` ✓
- `Source` pode ser chave de `FxHashMap` ✓
- Testes de regressão do Passo 25 não regridem (341 base) ✓
- Zero violations ✓

---

## Ao terminar, reportar

**Do diagnóstico (Tarefa 1):**
- Estrutura real de `Length` no vanilla — `struct { abs, em }` ou enum ou outro?
- Se `Abs` é newtype f64 e qual a escala interna (pt directo ou unidades raw?)
- Se `Ratio(f64)` vanilla usa 0.5 = 50% ou 50.0 = 50%
- Se `Angle` usa radianos internamente — confirmado ou diferente?
- Se `Color` é enum com variantes por espaço de cor — confirmado?

**Das correcções:**
- Se `Length` foi alterado — quais testes quebraram e como foram corrigidos
- Se `Length` NÃO foi alterado — documentar porquê (vanilla é igual ao enum actual)
- Se a soma `Pt + Em` em `eval_binary_op` passou de `Err` para `Ok(Length)` — confirmar
- Se `PartialEq` manual de `Content` causou problemas com outras variantes
- Se `rustc_hash` já estava em `01_core/Cargo.toml` ou foi necessário adicionar

**Número total de testes e zero violations.**

**Go para Passo 27 — DEBT-4 continuação: funções nativas de conversão
(`str()`, `int()`, `float()`) e cálculo (`calc.abs()`, `calc.pow()`, `calc.sqrt()`).**
