# ⚖️ ADR-0032: Política de `unsafe` em L1

**Status**: `EM VIGOR`
**Data**: 2026-04-22

---

## Contexto

A Arquitectura Cristalina impõe pureza em L1 — sem I/O de sistema,
sem estado global mutável (V13 do linter), sem dependências externas
fora de `[l1_allowed_external]`. A regra "sem `unsafe` em L1" foi
aplicada implicitamente em toda a série de passos 84.x como
convenção cristalina, consistente com o espírito dos ADRs 0004,
0005, 0019 e 0029.

A auditoria do Passo 84.7 (Secção 6.1 do relatório
`00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md`)
revelou que a regra **não é literalmente seguida** — o código tem 14
ocorrências reais de `unsafe` em L1 (mais 1 comentário sem efeito):

- `01_core/src/rules/lexer/scanner.rs` — **13 ocorrências**:
  - 6 `unsafe impl Sealed<T>` (sealed-trait pattern — mecanismo
    de encapsulamento, não de memória).
  - 7 `unsafe { get_unchecked(...) }` (acessos sem verificação de
    bounds em código herdado de `unscanny`, ver ADR-0014).
- `01_core/src/rules/eval.rs:235` — **1 ocorrência**: deref de raw
  pointer em `Drop for ImportGuard` (invariante documentado em
  SAFETY comment).
- `01_core/src/entities/content.rs:20` — 1 comentário documentando
  `unsafe trait NativeElement` do vanilla; não é `unsafe` real.

A regra nunca foi formalizada num ADR. O Passo 84.7 identificou
esta lacuna como Input 4 para o presente passo.

---

## Análise das três classes de `unsafe`

### Classe A — Sealed traits (6 ocorrências)

`unsafe impl Sealed<T> for ...` é padrão Rust para trait privada
com implementadores restritos. A palavra `unsafe` aqui é semântica
arbitrária da linguagem — não indica manipulação de memória, não
há invariante de segurança real a preservar.

**Alternativa sem `unsafe`**: padrão "sealed via private module" —
colocar a trait num módulo privado e exportar apenas o nome.

```rust
mod sealed {
    pub trait Sealed<T> {}
}

pub trait Pattern: sealed::Sealed<char> {}
```

**Custo de eliminação**: zero. Refactor mecânico.

### Classe B — `get_unchecked` no scanner (7 ocorrências)

Acessos a substrings sem verificação de bounds, onde o scanner
mantém invariantes próprios que garantem validade dos índices.
Herdado de `unscanny` (ADR-0014 inlinou a crate como módulo de L1).

**Alternativa sem `unsafe`**: substituir `get_unchecked(start..end)`
por `&self.string[start..end]`. Rust insere verificação dinâmica —
duas comparações e um branch por acesso.

**Custo de eliminação**: desconhecido. Potencialmente pequeno em
contexto de lex (que não domina o tempo total de compilação —
layout e shape domínam). Exige benchmark reprodutível para decisão
informada. **Não é possível decidir sem infra de benchmarking que
ainda não existe no projecto.**

### Classe C — `ImportGuard::drop` (1 ocorrência)

Deref de raw pointer no `Drop for ImportGuard` para chamar
`retain` no `import_stack` do `EvalContext`. Padrão RAII que usa
raw pointer porque a vida do `EvalContext` não é expressível como
lifetime (o guard é criado dentro de função que recebe `&mut ctx`
mas persiste até ao fim do scope da função).

**Alternativas sem `unsafe`**:

1. `Rc<RefCell<Vec<FileId>>>` no `EvalContext` + clone no guard.
   Custo: uma alocação `Rc` + borrow check runtime.
2. Eliminar o guard. Push/pop manual em torno da chamada
   recursiva. Perde-se RAII — se erro acontece entre push e pop,
   `pop` não corre. Aceitável se o `EvalContext` é descartado em
   erro.
3. Substituir raw pointer por índice + verificação com `len()` no
   drop. Sem custo de performance, sem `unsafe`, mas perde a
   garantia estrutural de que o guard corresponde à sua entrada.

**Custo de eliminação**: zero a baixo. Refactor com decisão de
design.

---

## Decisão

### Regra em vigor

**Zero `unsafe` em L1 como objectivo. Excepções apenas com prova
de custo medido.**

Interpretação:

- **Ideal a atingir**: L1 sem qualquer `unsafe`.
- **Permitido temporariamente**: `unsafe` em código inlinado de
  crate externa revista (ex: scanner herdado de `unscanny`) ou
  onde refactor foi escolhido para DEBT com plano documentado.
- **Permitido permanentemente**: apenas se benchmark reprodutível
  demonstrar regressão inaceitável ao eliminar o `unsafe`, e se
  um ADR específico para esse caso registar o número concreto.

### Aplicação às 14 ocorrências reais

| Classe | Localização | Plano |
|--------|-------------|-------|
| A (Sealed) | scanner.rs, 6 ocorrências | Eliminar. DEBT-41 aberto no Passo 84.8a. Refactor em passo dedicado. |
| B (get_unchecked) | scanner.rs, 7 ocorrências | DEBT-42 aberto no Passo 84.8a. Bloqueado por infra de benchmark. Decisão pós-medição: eliminar se regressão < X%, manter com ADR específico caso contrário. |
| C (ImportGuard) | eval.rs:235, 1 ocorrência | Eliminar. DEBT-40 aberto no Passo 84.8a. Refactor em passo dedicado com escolha entre 3 opções. |

### Proibições absolutas (mesmo em L3)

- `Box::leak` — ADR-0004 estabeleceu precedente.
- `mem::transmute` arbitrário entre tipos não relacionados.
- `std::mem::zeroed` em tipos não-`Copy`.
- Casts de ponteiro bruto via `as` entre tipos de ponteiro.

### Cláusula sobre código inlinado

ADR-0014 autorizou o inlining de `unscanny` em
`01_core/src/rules/lexer/scanner.rs`. O `unsafe` nesse ficheiro é,
no momento da inclusão original, **herdado** de código externo
revisto pela comunidade Rust.

Esta cláusula **não** isenta o scanner da regra "zero `unsafe`" —
serve apenas para registar que o `unsafe` actual não foi
introduzido pelo projecto cristalino e tem historial de revisão
externa. A eliminação é trabalho pendente registado em DEBT-41 e
DEBT-42.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/scanner.md` (se existe) | Nota sobre DEBT-41 e DEBT-42 — `unsafe` actual é temporário. |
| `00_nucleo/prompts/rules/eval.md` (se existe) | Nota sobre DEBT-40 — `ImportGuard` será refactorado. |

A actualização concreta destes prompts fica para os passos que
materializarem os refactors (DEBT-40, 41, 42), não para este passo
de decisão.

---

## Consequências

**Positivas**:
- Formaliza uma regra que era aplicada implicitamente e estava a
  ser violada sem visibilidade.
- Dá critério claro para futuras decisões: zero é o objectivo,
  excepções exigem prova concreta.
- Estabelece precedente contra ritual vazio (SAFETY comments sem
  invariante demonstrado).

**Negativas**:
- O ADR cria dependência entre refactors futuros (DEBT-41, DEBT-42)
  e infra de benchmarking que ainda não existe.
- A cláusula "permitido permanentemente com ADR específico" pode
  ser usada abusivamente se benchmark não for reprodutível — o
  critério "regressão inaceitável" é ambíguo. Resolução: cada ADR
  de excepção futura deve citar número concreto e metodologia.

**Neutras**:
- `01_core/src/entities/content.rs:20` contém comentário sobre
  `unsafe trait NativeElement` do vanilla. Não é `unsafe` real —
  não é afectado.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| "Zero `unsafe` imediatamente" | Regra mais forte | Exige refactor imediato em 14 sítios, incluindo 7 onde custo é incógnito |
| "Intermédia permanente com SAFETY comments" | Aceita a realidade actual | Normaliza `unsafe` sem plano de saída; ritual sem conteúdo próprio |
| "Zero `unsafe` com excepções permanentes pré-declaradas" | Simples | Exige listar excepções antes de medir custo — decisão cega |
| **Decisão adoptada: "Zero tendencial com DEBTs medidos"** | **Honra ambas as restrições (purismo + realismo)** | **Depende de infra de benchmark para fechar DEBT-42** |

---

## Referências

- ADR-0004 — precedente sobre `Box::leak` como "exceção silenciosa"
- ADR-0005 — remoção de `Box::leak` confirmada
- ADR-0014 — inlining de `unscanny` (fonte do `unsafe` do scanner)
- ADR-0019 — `unsafe` autorizado em L3
- ADR-0029 — definição física de pureza em L1
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md`
  Secção 6.1 — motivação empírica desta ADR
- DEBT-40, DEBT-41, DEBT-42 — trabalho pendente decorrente
