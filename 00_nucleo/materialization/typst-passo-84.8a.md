# Passo 84.8a — ADR-0032 (política de `unsafe` em L1) e DEBTs 40-42

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/template-adr.md` — template para novos ADRs.
- `00_nucleo/adr/typst-adr-0031-early-hashing-source.md` — ADR mais
  recente; confirmar convenções de nome e formatação.
- `00_nucleo/DEBT.md` — DEBT mais recente é DEBT-39 (aberto no 84.4);
  próximo número disponível é 40.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secção 6.1 contém a análise dos 15 `unsafe` em L1 que motiva este
  passo.
- `01_core/src/rules/lexer/scanner.rs` — código com 13 ocorrências
  de `unsafe` (6 `unsafe impl Sealed<T>` + 7 `get_unchecked`).
- `01_core/src/rules/eval.rs:235` — `ImportGuard::drop` com 1
  ocorrência de `unsafe` (deref de raw pointer).

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6 ignorados
pré-existentes), zero violations. Passo 83.6 concluído. Relatório
do 83.5 persistido.

---

## Natureza deste passo

**Passo de decisão arquitectural (ADR novo) + abertura de 3 dívidas
associadas.** Não faz refactors — os refactors ficam para passos
posteriores que consumirão os DEBTs aqui abertos.

Quatro produtos concretos:

1. Criar `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md`.
2. Abrir DEBT-40 (ImportGuard::drop) na Secção 1 de `00_nucleo/DEBT.md`.
3. Abrir DEBT-41 (sealed traits no scanner) na Secção 1.
4. Abrir DEBT-42 (get_unchecked no scanner — bloqueado por benchmark)
   na Secção 1.

**Regra absoluta**: Claude Code **não altera** `01_core/src/rules/lexer/scanner.rs`
nem `01_core/src/rules/eval.rs` neste passo. Apenas escreve o ADR,
abre os DEBTs, e verifica.

---

## Decisão arquitectural do ADR-0032

Regra formalizada pelo ADR:

> **Zero `unsafe` em L1 como objectivo. Excepções apenas com prova
> de custo medido.**

Interpretação operacional em três camadas:

1. **Ideal a atingir**: L1 sem qualquer `unsafe`.
2. **Permitido temporariamente**: `unsafe` herdado de crates externas
   inlinadas (ex: scanner herdado de `unscanny` via ADR-0014) ou
   onde refactor foi escolhido para deixar como DEBT com plano
   documentado.
3. **Permitido permanentemente**: apenas se benchmark reprodutível
   demonstrar regressão inaceitável ao eliminar o `unsafe`, e se o
   ADR específico desse caso registar o número concreto.

**Proibido sem excepção (mesmo em L3)**:
- `Box::leak` (ADR-0004 estabeleceu precedente).
- `mem::transmute` arbitrário entre tipos não relacionados.
- `std::mem::zeroed` em tipos não-`Copy`.
- Casts de ponteiro bruto via `as` entre tipos de ponteiro
  (ex: `*const T as *mut T`, `usize as *const T`).

---

## Tarefa 1 — Criar ADR-0032

Criar ficheiro `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md` com o
conteúdo abaixo. Actualizar o hash do header L0 se o projecto tem
convenção de hash (ver outros ADRs recentes para padrão exacto).

```markdown
# ⚖️ ADR-0032: Política de `unsafe` em L1

**Status**: `ACCEPTED`
**Data**: 2026-04-XX

---

## Contexto

A Arquitectura Cristalina impõe pureza em L1 — sem I/O de sistema,
sem estado global mutável (V13 do linter), sem dependências externas
fora de `[l1_allowed_external]`. A regra "sem `unsafe` em L1" foi
aplicada implicitamente em toda a série de passos 84.x como
convenção cristalina, consistente com o espírito dos ADRs 0004,
0005, 0019 e 0029.

A auditoria do Passo 84.7 (Secção 6.1 do relatório) revelou que a
regra **não é literalmente seguida** — o código tem 15 ocorrências
reais de `unsafe` em L1:

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

ADR-0014 autorizou o inlining de `unscanny` em `01_core/src/rules/lexer/scanner.rs`.
O `unsafe` nesse ficheiro é, no momento da inclusão original,
**herdado** de código externo revisto pela comunidade Rust.

Esta cláusula **não** isenta o scanner da regra "zero `unsafe`" —
serve apenas para registar que o `unsafe` actual não foi
introduzido pelo projecto cristalino e tem historial de revisão
externa. A eliminação é trabalho pendente registado em DEBT-41 e
DEBT-42.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/scanner.md` | Nota sobre DEBT-41 e DEBT-42 — `unsafe` actual é temporário. |
| `00_nucleo/prompts/rules/eval.md` (se existe) | Nota sobre DEBT-40 — `ImportGuard` será refactorado. |

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
```

**Verificar após criar**:

```bash
# Ficheiro existe
ls -la 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md

# Tem cabeçalho de status
grep "^\*\*Status\*\*:" 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md
```

---

## Tarefa 2 — Abrir DEBT-40 (ImportGuard::drop)

Adicionar à Secção 1 do `00_nucleo/DEBT.md`, em ordem numérica
após DEBT-39:

```markdown
## DEBT-40 — `ImportGuard::drop` com raw pointer — EM ABERTO (Passo 84.8a)

`01_core/src/rules/eval.rs:235` tem `unsafe { (*self.stack_ptr).retain(...) }`
no `Drop for ImportGuard`. O raw pointer é usado porque a vida do
`EvalContext` não é expressível como lifetime do guard (RAII com
scope de função).

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo de eliminação zero a baixo. Resolução fica
para passo dedicado que escolha entre as opções abaixo.

### Opções de resolução (sugestões, não obrigatórias)

1. **`Rc<RefCell<Vec<FileId>>>`** no `EvalContext` + clone no
   guard. Custo: uma alocação `Rc` + borrow check runtime.
2. **Eliminar o guard**. Push/pop manual em torno da chamada
   recursiva. Perde-se RAII — se erro acontece entre push e pop,
   `pop` não corre. Aceitável se o `EvalContext` é descartado em
   erro.
3. **Índice + verificação com `len()`**. Guardar posição no
   stack em vez de ponteiro; comparar com `len()` no drop para
   confirmar que é o topo; pop se sim, erro ou no-op se não.
   Sem custo de performance, sem `unsafe`, mas perde a garantia
   estrutural de que o guard corresponde à sua entrada.

### Critério de conclusão

- Nenhuma ocorrência de `unsafe` em `eval.rs` relacionada com
  `ImportGuard`.
- Testes de `import_stack` (detecção de ciclos) continuam a passar.
- ADR específico escrito se a resolução introduzir decisão
  arquitectural não prevista.

### Dependências

Nenhuma. Pode ser atacado quando o utilizador decidir.
```

---

## Tarefa 3 — Abrir DEBT-41 (sealed traits no scanner)

```markdown
## DEBT-41 — Sealed traits no scanner usam `unsafe trait` — EM ABERTO (Passo 84.8a)

`01_core/src/rules/lexer/scanner.rs` tem 6 `unsafe impl Sealed<T>`
usando o padrão sealed-trait clássico da stdlib Rust. A palavra
`unsafe` aqui é mecanismo de encapsulamento (impedir
implementações externas), não indicação de memória não-segura.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo zero — é refactor mecânico para o padrão
"sealed via private module".

### Proposta de resolução

Substituir:

```rust
pub unsafe trait Sealed<T> { ... }
unsafe impl Sealed<char> for ... { ... }
```

Por:

```rust
mod sealed {
    pub trait Sealed<T> { ... }
}

pub trait Pattern: sealed::Sealed<char> { ... }
impl sealed::Sealed<char> for ... { ... }
```

Verificar que nenhum consumer externo das traits do scanner
depende da assinatura `unsafe trait` (uso em bounds genéricos
deveria continuar a funcionar).

### Critério de conclusão

- Zero ocorrências de `unsafe` associadas ao padrão Sealed em
  scanner.rs.
- Testes do scanner continuam a passar sem alteração.
- Nenhum impacto visível na API pública de `01_core/src/rules/lexer/`.

### Dependências

Nenhuma. Refactor trivial, pronto para atacar.
```

---

## Tarefa 4 — Abrir DEBT-42 (get_unchecked no scanner)

```markdown
## DEBT-42 — `get_unchecked` no scanner — EM ABERTO (Passo 84.8a, bloqueado)

`01_core/src/rules/lexer/scanner.rs` tem 7 ocorrências de
`unsafe { self.string.get_unchecked(start..end) }`. Herdado de
`unscanny` via ADR-0014.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito, com
excepção permitida permanentemente apenas se benchmark
reprodutível demonstrar regressão inaceitável ao eliminar o
`unsafe`, e se um ADR específico registar o número concreto.

### Bloqueio

Este DEBT depende de **infra de benchmarking reprodutível no
projecto**, que ainda não existe. Sem a infra, não é possível
aplicar o critério da ADR-0032 de forma honesta.

### Plano de resolução

1. **Pré-requisito**: criar infra de benchmark para lex e parse.
   Pode ser `criterion` crate (já usada na comunidade Rust) ou
   alternativa. ADR específico se a decisão sobre qual biblioteca
   envolver dependências novas em L1 ou se o benchmark vive fora
   de L1.

2. **Medição baseline**: com `get_unchecked` tal como está actual.
   Executar benchmark sobre conjunto de documentos representativos
   (documentos simples, documentos com muito texto, documentos
   matemáticos).

3. **Refactor experimental em branch**: substituir `get_unchecked`
   por `&self.string[start..end]` e medir.

4. **Decisão**:
   - Se regressão < 5% no tempo total de lex: eliminar `unsafe`,
     fechar DEBT.
   - Se regressão entre 5% e 20%: decisão do utilizador com base
     em contexto (documentos alvo, uso do Typst).
   - Se regressão > 20%: manter `unsafe` no scanner como excepção
     permanente, escrever ADR específico citando a medida.

### Critério de conclusão

Uma de duas:
- Zero ocorrências de `unsafe { get_unchecked(...) }` em scanner.rs.
- ADR específico escrito que autoriza permanência do `unsafe` com
  número concreto de regressão medida.

### Dependências

- Infra de benchmark — trabalho próprio, não coberto por este
  bloco de passos.
```

---

## Tarefa 5 — Verificação

```bash
# ADR criado
ls -la 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md

# Contagem de ADRs passou de 32 ficheiros para 33
ls 00_nucleo/adr/typst-adr-*.md | wc -l

# DEBT-40, 41, 42 aparecem em DEBT.md
grep "^## DEBT-40\|^## DEBT-41\|^## DEBT-42" 00_nucleo/DEBT.md

# Os três estão na Secção 1 (em aberto)
awk '/^## Secção 1/,/^## Secção 2/' 00_nucleo/DEBT.md | grep "^## DEBT-4"

# Contagem da Secção 1 aumentou em 3 (era 9 pós-84.6; agora 12)
awk '/^## Secção 1/,/^## Secção 2/' 00_nucleo/DEBT.md | grep -c "^## DEBT-"

# Nenhum ficheiro de código alterado
git status 01_core/src/ 03_infra/src/
# Esperado: zero ficheiros modificados

# Testes continuam a passar (nada de código mudou)
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] `typst-adr-0032-unsafe-em-l1.md` criado em `00_nucleo/adr/`.
- [ ] ADR-0032 segue formato dos ADRs mais recentes (0029, 0030,
  0031): status `ACCEPTED` com backticks, secções Context, Decisão,
  Prompts Afectados, Consequências, Alternativas, Referências.
- [ ] DEBT-40, DEBT-41, DEBT-42 aparecem em ordem numérica na
  Secção 1 de `00_nucleo/DEBT.md`.
- [ ] DEBT-40 (ImportGuard) tem 3 opções de resolução propostas
  como sugestões.
- [ ] DEBT-41 (sealed traits) tem proposta concreta e marca
  "refactor trivial, pronto para atacar".
- [ ] DEBT-42 (get_unchecked) marca explicitamente "bloqueado por
  infra de benchmark".
- [ ] Contagem de Secção 1: 12 DEBTs (9 pré-existentes + 3 novos).
- [ ] Contagem de Secção 2: inalterada.
- [ ] Nenhum ficheiro de código em `01_core/src/` ou `03_infra/src/`
  alterado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Confirmação de que ADR-0032 foi criado.
- Confirmação de que os 3 DEBTs foram abertos com os números
  correctos.
- Contagem da Secção 1 do DEBT.md antes e depois.
- Confirmação de que nenhum código-fonte foi tocado.
- Número de testes (esperado: 911, idêntico).
- Resultado do linter.

**Go/No-Go para o Passo 84.8b** (correcções de status nos ADRs):

- **GO**: ADR-0032 escrito, DEBTs abertos, nenhum código tocado.
  Próximo passo ataca a Prioridade ALTA do relatório do 84.7.
- **NO-GO — DEBT-42 aberto sem menção de bloqueio**: se o DEBT-42
  não identifica explicitamente "bloqueado por infra de benchmark",
  fica com aparência de dívida actionable imediatamente.
  Adicionar a dependência.
- **NO-GO — código tocado**: se `scanner.rs` ou `eval.rs` foram
  alterados (refactor prematuro), reverter. Este passo é só ADR +
  DEBTs.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório** — o produto são o ADR e os 3
DEBTs, que vivem nos seus próprios ficheiros (`adr/` e `DEBT.md`).

A regra "relatório persistente obrigatório" estabelecida no 83.6
aplica-se a passos de auditoria, diagnóstico extenso, ou
investigação. Este passo é execução directa de decisão prévia —
não gera relatório.

Os passos 84.8b a 84.8e também não produzirão relatório (são
execução), com excepção de um possível passo futuro de verificação
pós-refactors de `unsafe` (que pode precisar de relatório
comparando estado antes e depois). Esse passo, quando existir,
seguirá a convenção `00_nucleo/relatorios/relatorio-*.md`.
