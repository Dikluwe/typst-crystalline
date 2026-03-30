# ⚖️ ADR-0029: Definição física de pureza em L1 — revoga ADR-0028

**Status**: `ACCEPTED`
**Data**: 2026-03-29
**Revoga**: ADR-0028 (Representação Simplificada dos Tipos Tipográficos)

---

## Contexto

A ADR-0028 autorizou representações simplificadas para os tipos
tipográficos em L1 — `Length` como `enum { Pt(f64), Em(f64) }`,
`Color` sem espaços CMYK/Oklab, etc. — com a justificação de que
a fidelidade ao original seria adiada para quando StyleChain existir.

Simultaneamente, o projecto adoptou uma definição formal de pureza
que é mais rigorosa do que a anterior: pureza é determinismo sem
I/O de sistema, não ausência de estruturas de dados de alta performance.
Esta definição tem consequências directas sobre o que é aceitável em L1.

A ADR-0028 codificou uma simplificação que a nova definição de
pureza torna incorrecta: as estruturas do Typst vanilla para
`Length`, `Abs`, `Rel`, `Angle`, `Ratio` usam `Arc` e representações
internas que são completamente puras sob a definição física — não fazem
I/O de sistema, não têm estado global mutável, não escapam da RAM.
Não há razão de domínio para simplificá-las.

---

## Definição formal de pureza e I/O em L1

### O que é I/O — proibido estritamente em L1

I/O refere-se **exclusivamente** à interacção com o mundo externo ao
processador e à memória RAM do processo actual:

| Operação | Exemplos | Proibida em L1 |
|----------|----------|----------------|
| Acesso a disco | `std::fs::read`, `File::open` | ✓ |
| Acesso a rede | HTTP, sockets, `std::net` | ✓ |
| Relógio do sistema | `SystemTime::now()`, `OffsetDateTime::now_utc()` | ✓ |
| Variáveis de ambiente | `std::env::var`, `std::env::args` | ✓ |
| Entropia do SO | `rand::thread_rng()` com seed do SO | ✓ |
| Console | `println!`, `eprintln!` em produção | ✓ |
| Estado global mutável | `static mut T`, `static Mutex<T>`, `static OnceLock<T>` | ✓ |

### O que NÃO é I/O — permitido e exigido em L1

| Operação | Exemplos | Permitida em L1 |
|----------|----------|-----------------|
| Alocação de memória RAM | `Vec::new()`, `String::new()`, `Box::new()` | ✓ |
| Contagem de referências | `Arc<T>`, `Rc<T>` em campos de struct | ✓ |
| Crates utilitárias puras autorizadas | `ecow`, `indexmap`, `rustc_hash` | ✓ |
| Processamento binário em memória | Iterar `&[u8]` já carregado | ✓ |

---

## Revogação da ADR-0028

A ADR-0028 é revogada na sua totalidade.

### O que estava errado na ADR-0028

A ADR-0028 justificou simplificações com o pretexto de que a fidelidade
ao original requeria StyleChain (DEBT-1). Isso era verdade para o
**comportamento semântico** (somas mistas `Pt + Em` → `Relative`), mas
não para a **estrutura de dados interna** de cada tipo.

`Length` no original Typst vanilla é uma struct com dois campos
(`abs: Abs`, `em: f64`) — não um enum. Essa representação é pura sob
a definição física. Simplificá-la para um enum `{ Pt(f64), Em(f64) }`
não era uma decisão arquitectural necessária — era conveniência de
implementação registada incorrectamente como decisão de domínio.

### Impacto no código existente (Passo 25)

O código do Passo 25 implementou `Length` como enum `{ Pt(f64), Em(f64) }`.
Este código **continua a funcionar** e os 341 testes continuam a passar.
A revogação da ADR-0028 não exige refactorização imediata.

O que muda é a direcção dos Passos seguintes: ao adicionar novos tipos
ou ao revisitar `Length` no contexto do Passo 30 (StyleChain), usar a
representação fiel ao vanilla — não uma simplificação.

---

## Decisão

### Regra de execução para tipos tipográficos em L1

Ao materializar `Length`, `Abs`, `Rel`, `Angle`, `Ratio`, `Color`,
e tipos similares em L1, seguir a arquitectura de memória do Typst
vanilla sem simplificações de conveniência:

1. **Diagnosticar primeiro**: verificar a estrutura real no original
   antes de definir o tipo em L1.

2. **Não simplificar sob o pretexto de adiamento**: se o tipo vanilla
   usa `struct Length { abs: Abs, em: f64 }`, usar essa estrutura.
   O facto de somas mistas precisarem de StyleChain para ser resolvidas
   é uma limitação de **operações** (eval_binary_op retorna Err), não
   de **estruturas de dados**.

3. **`Arc` é permitido e exigido quando o tipo é clonado no hot path**:
   `Arc<T>` em campos de struct é gestão de RAM, não I/O.

4. **Simplificações aceites apenas com ADR explícita** que documente:
   - Qual a diferença entre o vanilla e o cristalino
   - Qual o custo semântico da simplificação
   - Qual o critério de revisão (não "quando StyleChain existir"
     de forma vaga, mas um passo específico)

### Sobre o código do Passo 25

`Length` como `enum { Pt(f64), Em(f64) }` — manter no Passo 25.
Revisitar no diagnóstico do Passo 30 (StyleChain) quando a estrutura
real de `Length` vanilla for necessária para a implementação de
`Relative` e somas mistas.

`Ratio(f64)`, `Angle(f64)`, `Color { Rgb, Rgba }` — verificar no
diagnóstico do próximo passo que toca estes tipos se a representação
vanilla difere materialmente.

---

## O que permanece proibido (V13 sem alterações)

V13 (MutableStateInCore) permanece em vigor para estado global mutável:

```rust
// PROIBIDO — estado global mutável
static INTERNER: Mutex<HashMap<PathBuf, FileId>> = Mutex::new(HashMap::new());
static COUNTER: AtomicU16 = AtomicU16::new(1);
static CACHE: OnceLock<Vec<SyntaxKind>> = OnceLock::new();
```

```rust
// PERMITIDO — gestão de memória RAM em campos de struct
pub struct Module(Arc<ModuleInner>);     // clone O(1) em eval()
pub struct SyntaxNode(Arc<NodeData>);    // partilha do CST sem cópia
pub struct Func(Arc<FuncRepr>);          // clone O(1) de closures
```

---

## Consequências

**Positivas**:
- Elimina a pressão de tomar decisões de simplificação prematura com base
  em dívidas de DEBT que ainda não existem.
- Alinha a IA com a intenção de paridade com o vanilla onde a
  arquitectura de memória é pura.
- Remove a ADR-0028 como fonte de autorização para futuras simplificações
  não documentadas.

**Negativas**:
- O diagnóstico de cada tipo tipográfico vanilla torna-se obrigatório
  antes de materializar — mais trabalho de diagnóstico upfront.
- O código do Passo 25 tem `Length` como enum que pode não corresponder
  ao vanilla — aceitável por agora, revisão no Passo 30.

**Neutras**:
- Código existente não precisa de ser alterado agora.
- V13 não muda — a sua semântica já era correcta.

---

## Referências

- ADR-0028 — revogada por esta ADR
- ADR-0018 — precedente: "o critério não é 'é externo?' mas 'viola pureza funcional?'"
- DEBT-1 — StyleChain (Passo 30) — contexto onde Length vanilla será necessário
