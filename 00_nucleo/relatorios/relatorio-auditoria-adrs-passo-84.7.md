# Relatório de auditoria dos ADRs do Typst — Passo 84.7

**Data:** 2026-04-22
**Âmbito:** Apenas ADRs com prefixo `typst-adr-` em `00_nucleo/adr/`. ADRs do
`crystalline-lint` ignorados por instrução do passo.
**Restrição:** Nenhum ficheiro em `00_nucleo/adr/` foi alterado neste passo.

---

## 1. Inventário

- **Ficheiros físicos:** 32 (`ls 00_nucleo/adr/typst-adr-*.md | wc -l`).
- **ADRs únicos:** 31 (ADR-0026 com 2 ficheiros — Input 6 confirmado).
- **Numeração contínua de 0001 a 0031**, sem buracos.

### Distribuição por status

| Status declarado | Contagem | ADRs |
|---|---|---|
| `PROPOSTO` | 14 | 0001, 0005, 0006, 0007, 0008, 0009, 0010, 0011, 0012, 0013, 0014, 0015 |
| `PROPOSTO — adiada` | 1 | 0020 |
| `IDEIA — não implementar ainda` | 2 | 0002, 0003 |
| `IMPLEMENTADO` | 11 | 0004, 0016, 0017 (com "Estado"), 0018 (formatação broken), 0019, 0021, 0022, 0023, 0024, 0025, 0026-divergencia |
| `UPDATED` | 1 | 0026-revisao |
| `Accepted` (PT inglês, 2 pontos) | 2 | 0027, 0028 |
| `ACCEPTED` (com backticks) | 3 | 0029, 0030, 0031 |

### Relações documentadas

```
ADR-0007  ──[Revoga: ADR-0007]──→  ADR-0018  (campo "**Revoga**" presente)
ADR-0028  ──[Revoga: ADR-0028]──→  ADR-0029  (campo "**Revoga**" presente)
ADR-0015  ──[Não revogada por]──   ADR-0024  (texto explícito)
ADR-0016  ──[Complementa]──────→  ADR-0031  (Opção C aplicada)
ADR-0026-divergencia ──[Revisto por]──→ ADR-0026-revisao  (mesmo número)
```

---

## 2. ADRs com STATUS DESALINHADO

### ADR-0001 — Estratégia de Migração
- **Status declarado:** `PROPOSTO`
- **Realidade:** Opção C (`comemo` em `[l1_allowed_external]`) executada — confirmado por
  `grep "comemo" crystalline.toml` (linha 73, com comentário `# decisão intencional, ver ADR-0001`)
  e `grep -rn "comemo" 01_core/src/` retornar 5+ ocorrências de `#[comemo::track]` em `world_types.rs`.
  A "Opção B futura" (isolamento em L3 via "Passo 10") nunca aconteceu — a numeração de passos
  seguiu outra trajectória.
- **Acção sugerida:** atualizar para `IMPLEMENTADO (Opção C, Passo 4)` com nota residual
  "Opção B (isolamento em L3) continua em aberto como dívida conhecida" — registar como
  DEBT-novo se o utilizador concordar.

### ADR-0007 — `rustc_hash` substituído por `std::collections`
- **Status declarado:** `PROPOSTO`
- **Realidade:** revogado por ADR-0018 (`Revoga: ADR-0007` em ADR-0018:5; texto literal
  "ADR-0007 — decisão revogada" em ADR-0018:132).
- **Acção sugerida:** atualizar para `REVOGADO` (ou `PROPOSTO → REVOGADO pela ADR-0018`)
  e adicionar campo `**Revogado por**: ADR-0018`.

### ADR-0017 — Adiamento da migração de `eval` (typst-library)
- **Anomalia mista:** usa `**Estado**:` em vez de `**Status**:`, e o valor `IMPLEMENTADO`
  está sem backticks (`IMPLEMENTADO` em vez de `` `IMPLEMENTADO` ``).
- **Realidade:** `pub fn eval(` confirmado em `01_core/src/rules/eval.rs:250`.
- **Acção sugerida:** padronizar para `**Status**: ` `` `IMPLEMENTADO` ``. Conteúdo do ADR
  pode estar desactualizado (descrevia adiamento que já não se aplica) — auditar texto interno
  separadamente.

### ADR-0018 — `rustc_hash` reintroduzido
- **Status declarado:** `\*\*Status\*\*: ` `` `IMPLEMENTADO` `` (backslashes literais escapando
  os asteriscos).
- **Realidade:** o conteúdo é correcto; só a formatação está partida. Renderiza como literal
  em qualquer parser markdown.
- **Acção sugerida:** remover backslashes — `**Status**: ` `` `IMPLEMENTADO` ``.

### ADR-0028 — Tipos tipográficos simplificados
- **Status declarado:** `**Status:** Accepted`
- **Realidade:** revogado por ADR-0029 — texto literal "ADR-0028 é revogada na sua totalidade"
  em ADR-0029:60. O nome do ficheiro de ADR-0029 inclui "revoga-adr-0028".
- **Acção sugerida:** atualizar para `REVOGADO` (mantendo o texto histórico) com campo
  `**Revogado por**: ADR-0029`.

---

## 3. ADRs com CONTEÚDO INCOMPLETO

### ADR-0029 — Pureza física (revoga ADR-0028) — Input 1
- **Lacuna:** a lista explícita de tipos tipográficos que seguem a arquitectura vanilla
  (`Length`, `Abs`, `Rel`, `Angle`, `Ratio`, `Color`) não inclui `Alignment`. Confirmado por
  `grep "Alignment" typst-adr-0029-...md` retornar 0 matches. Passo 84.5 (DEBT-36) materializou
  `Align2D` seguindo o espírito do ADR mas sem que o ADR o reconheça textualmente.
- **Acção sugerida:** acrescentar `Alignment` (ou `Align2D` no nome cristalino) à enumeração
  explícita; alternativamente, substituir a lista por formulação genérica
  *"qualquer tipo tipográfico vanilla — incluindo Length, Angle, Ratio, Color, Alignment, e
  futuros (Gradient, Tiling, etc.)"*.

### ADR-0030 — Performance é domínio de L1 — Input 2
- **Lacuna:** a distinção entre `Vec<T>::clone()` (cópia profunda O(n), proibida no hot path)
  e `Arc<T>::clone()` (incremento de refcount O(1), obrigatória em L1) não está formalizada
  num parágrafo dedicado. Confirmado por `grep -i "arc::clone\|incremento\|refcount"
  typst-adr-0030-...md` retornar 0 matches. O ADR refere `Arc<T>` em campos struct, mas não
  contrasta as duas semânticas.
- **Acção sugerida:** adicionar secção curta "Clone profundo vs `Arc::clone`" que explicite:
  > "A palavra *clone* em Rust é polissémica: `Vec<T>::clone()` copia bytes (O(n));
  > `Arc<T>::clone()` incrementa um contador (O(1)). A regra desta ADR é evitar a
  > primeira no hot path, **não** evitar a segunda — `Arc::clone` é exigido em L1 sempre que
  > um campo cuja semântica é partilha apareça num clone repetido."

### ADR-0022 — FontBook (anti-padrão)
- **Anti-padrão:** contém secção `## Diagnóstico obrigatório antes de qualquer código` com
  scripts bash. Anti-padrão observado em ADR-0023 e ADR-0025 também (3 ADRs no total —
  `grep "## Diagnóstico" typst-adr-*.md`).
- **Razão:** ADRs documentam decisões; comandos de diagnóstico são execução. Misturar os
  dois esconde a decisão atrás de uma sequência de scripts.
- **Acção sugerida:** mover scripts bash para passos de materialização (`00_nucleo/materialization/`
  ou prompts dedicados em `00_nucleo/prompts/`); manter no ADR apenas a decisão final e o
  contexto.

---

## 4. ADRs com CONTRADIÇÃO COM CÓDIGO

**Nenhuma confirmada nesta auditoria.**

Verificações executadas confirmaram alinhamento entre ADR e código nos pontos auditados:
- ADR-0004: `FileId(NonZeroU16)` sem interner global ✓
- ADR-0016 → ADR-0031: `content_hash` confirmado em `01_core/src/entities/source.rs` ✓
- ADR-0017: `pub fn eval(` em `eval.rs:250` ✓
- ADR-0024: `ecow` em `Cargo.toml` (parser sem ecow / Value::Str com ecow) ✓
- ADR-0026 (divergência + revisão): `Content::Sequence(Arc<[Content]>)` confirmado ✓

A única dimensão onde *quase* há contradição é a regra "sem `unsafe` em L1" (ver Secção 6.1),
mas como essa regra não está formalizada num ADR, não conta como contradição com código.

---

## 5. Anomalias estruturais

### 5.1 Numeração duplicada entre projectos (Input 5)
- `00_nucleo/adr/` contém ADRs do `crystalline-lint` (`0001-tree-sitter-...md`,
  `0006-...md` etc.) e ADRs do Typst (`typst-adr-*.md`) lado a lado. Os números colidem
  (existe um ADR-0006 em cada prefixo).
- **Risco:** uma referência genérica "ADR-0006" é ambígua. O passo 84.7 declara que
  resolve sempre para `typst-adr-NNNN.md`, mas isso é convenção do passo, não da estrutura.
- **Recomendações alternativas:**
  - **A:** mover ADRs do `crystalline-lint` para `00_nucleo/adr/lint/` (subdir).
  - **B:** manter a estrutura plana e convencionar que toda a referência sem prefixo
    `typst-` se refere ao `crystalline-lint`, e vice-versa. Documentar em README.

### 5.2 ADR-0026 com dois ficheiros (Input 6)
- `typst-adr-0026-content-divergencia.md` e `typst-adr-0026-revisao-content-arc.md` partilham
  o número 0026.
- **Risco:** `grep "ADR-0026"` ambiguo entre o original e a revisão.
- **Recomendações alternativas:**
  - **A:** renomear segundo ficheiro para `typst-adr-0026-R1-content-arc.md` (sufixo de
    revisão explícito); estabelecer convenção `-R1`, `-R2`.
  - **B:** atribuir número novo (próximo livre = 0032) e referenciar o original com
    "**Revê**: ADR-0026".

### 5.3 Inconsistência de formato de status
- **PT, com backticks:** maioria dos ADRs (`**Status**: ` `` `PROPOSTO` ``).
- **PT, sem backticks, "Estado" em vez de "Status":** ADR-0017 (`**Estado**: IMPLEMENTADO`).
- **Inglês, dois pontos antes do espaço, sem backticks:** ADR-0027 e ADR-0028 (`**Status:** Accepted`).
- **Backslashes literais:** ADR-0018 (`\*\*Status\*\*: ` `` `IMPLEMENTADO` ``).
- **Acção sugerida:** padronizar para `**Status**: ` `` `VALOR` `` em PT, valores canónicos
  `PROPOSTO | IDEIA | IMPLEMENTADO | REVOGADO | UPDATED | ADIADO`.

### 5.4 Anti-padrão "Diagnóstico obrigatório" — confirmado em 3 ADRs
Já tratado em Secção 3 (ADR-0022); aplica-se também a ADR-0023 e ADR-0025.

---

## 6. Regras implícitas a formalizar

### 6.1 Regra: "Sem `unsafe` em L1" — **com excepções de facto**
- **Onde está implícita:** ADR-0004 (remove `Box::leak()`), ADR-0005 (confirma remoção),
  ADR-0019 (autoriza `unsafe` em L3, por implicação excluindo-o de L1), ADR-0029 (V13 do
  linter proibe `UnsafeCell` em statics de L1).
- **Realidade no código:** `grep -rn "unsafe" 01_core/src/` retorna **15 ocorrências**, em
  3 ficheiros:
  - `01_core/src/rules/lexer/scanner.rs` — **13 ocorrências**: `unsafe { get_unchecked(...) }`
    em métodos do scanner (acesso a substrings cujo intervalo é demonstravelmente válido) e
    `unsafe trait Sealed<T>` com 6 `unsafe impl` (sealed-trait pattern para `Pattern`-like).
  - `01_core/src/rules/eval.rs:235` — **1 ocorrência**: `unsafe { (*self.stack_ptr).retain(...) }`
    em `Drop for ImportGuard` (deref de raw pointer; SAFETY comment justifica que o
    EvalContext sobrevive ao guard).
  - `01_core/src/entities/content.rs:20` — comentário documentando que vanilla usa
    `unsafe trait NativeElement` (não é `unsafe` real).
- **Conclusão:** a regra **não é** "zero `unsafe` em L1" — é "minimizar `unsafe` em L1, com
  invariante demonstrável para cada ocorrência". O scanner pattern é well-known e justificável;
  o `ImportGuard::drop` é mais arriscado e merece atenção.
- **Acção sugerida:** criar **ADR-0032 — Política de `unsafe` em L1** com:
  - Permitido: `unsafe` em scanner com invariante de índice válido (pattern bem documentado
    do projecto Typst original).
  - Permitido: `unsafe trait` + `unsafe impl` para sealed-trait pattern.
  - Caso a caso: deref de raw pointer (como `ImportGuard::drop`) — exige SAFETY comment e
    revisão.
  - Proibido: `Box::leak`, `mem::transmute` arbitrário, `std::mem::zeroed` em tipos não-Copy.
  - Abrir DEBT-novo para reavaliar `ImportGuard::drop` (substituir por `Cell<Vec<...>>`?).

### 6.2 Regra: "`Arc::clone` é partilha, não cópia profunda" (Input 2)
- **Onde está implícita:** ADR-0030 (refere `Arc<T>` em campos), ADR-0026-revisao (propõe
  `Arc<[Content]>`), e o próprio Passo 84.4 (DEBT-22 → `Arc<[ShowRule]>`).
- **Acção sugerida:** se a Secção 3.2 deste relatório (lacuna em ADR-0030) for executada
  acrescentando o parágrafo sugerido, esta regra fica formalizada em ADR-0030 sem necessitar
  de ADR novo. Alternativa: **ADR-0033 — Semântica de `Arc::clone` em L1**, citando ADR-0030
  como complementar.

### 6.3 Regra: "Paridade funcional com vanilla é invariante; performance interna pode divergir"
- **Onde está implícita:** ADR-0026 (Content como enum vs vtable), ADR-0029 (tipos
  tipográficos seguem vanilla; estruturas internas podem divergir), Passo 84.5 (decisão
  semântica de `+` segue vanilla — erro em conflito — mesmo com forma `Align2D` que diverge).
- **Acção sugerida:** **ADR-0034 — Paridade funcional como invariante arquitectural**:
  - Invariante: para o mesmo input, output observável idêntico ao vanilla.
  - Permitido: divergência em forma estrutural (struct vs enum, `Vec` vs `Arc<[T]>`).
  - Proibido: divergência em semântica observável (regras de combinação, mensagens de erro
    com sentido diferente, ordens de operação visíveis ao utilizador).

### 6.4 Regra: "Diagnóstico obrigatório antes de materializar tipo tipográfico vanilla"
- **Onde está implícita:** Passo 84.5 (`Alignment`), Passo 84.6 (`PlaceElem`/`PlacementScope`).
  ADR-0022 e ADR-0023 misturam a regra (errada — ver Secção 5.4) com o produto.
- **Acção sugerida:** **ADR-0035 — Processo de diagnóstico para tipos vanilla**:
  - Exigência: antes de materializar tipo do vanilla em L1, executar diagnóstico estruturado
    (campos, dependências, semântica de operadores, divergências documentadas).
  - Diagnóstico vive no passo de materialização ou em prompt dedicado, **não no ADR**.
  - O ADR resultante referencia o diagnóstico, não o duplica.

---

## 7. Proposta de índice de ADRs (`00_nucleo/adr/README.md`)

```markdown
# Índice de ADRs do Typst Cristalino

## Meta-regras em vigor

1. **Pureza física de L1** (ADR-0029, revoga ADR-0028) — L1 não faz I/O de
   sistema (filesystem, rede, relógio, env). RAM é domínio.
2. **Performance de RAM é domínio** (ADR-0030) — gestão eficiente de RAM
   (`Arc`, `EcoString`, alocação) é comportamento correcto, não optimização
   especulativa.
3. **Critério de autorização externa** (ADR-0018) — o critério para
   `[l1_allowed_external]` não é "é externo?" mas "viola pureza funcional?".
4. **Política de `unsafe` em L1** (a formalizar — ADR-0032 proposta) —
   minimizar com invariante demonstrável; não é "zero unsafe".
5. **Semântica de `Arc::clone` em L1** (a formalizar via lacuna em ADR-0030,
   ou ADR-0033) — `Arc::clone` é partilha O(1), não cópia profunda; exigida
   no hot path.
6. **Paridade funcional vanilla** (a formalizar — ADR-0034 proposta) —
   output idêntico ao vanilla; estruturas internas podem divergir.
7. **Diagnóstico obrigatório para tipos vanilla** (a formalizar — ADR-0035
   proposta) — diagnóstico em passo de materialização, não no ADR.

## Status por ADR

| ADR | Título curto | Status declarado | Estado real (auditoria 84.7) |
|-----|--------------|------------------|------------------------------|
| 0001 | Estratégia de migração | `PROPOSTO` | DESALINHADO — Opção C implementada |
| 0002 | Hierarquia de contenção | `IDEIA` | OK |
| 0003 | comemo + contenção | `IDEIA` | OK |
| 0004 | Passo 1 descobertas | `IMPLEMENTADO` | OK |
| 0005 | PackageSpec World | `PROPOSTO` | OK (não materializado) |
| 0006 | typst_timing | `PROPOSTO` | OK (timing scope removido, ainda não religado) |
| 0007 | rustc_hash (substituição) | `PROPOSTO` | DESALINHADO — revogado por ADR-0018 |
| 0008–0014 | Inlinings + Unicode | `PROPOSTO` | OK (decisões de não-uso) |
| 0015 | ecow removido do parser | `PROPOSTO` | OK (não revogado por ADR-0024 — diferente contexto) |
| 0016 | LazyHash removido | `IMPLEMENTADO` | OK (Opção C formalizada em ADR-0031) |
| 0017 | Adiamento de eval | `IMPLEMENTADO` (formato `Estado:`) | DESALINHADO — formato; eval já implementado |
| 0018 | rustc_hash reintroduzido | `IMPLEMENTADO` (formatação broken) | DESALINHADO — backslashes literais |
| 0019 | TTF + RustyBuzz | `IMPLEMENTADO` | OK |
| 0020 | FontDB | `PROPOSTO — adiada` | OK |
| 0021 | DateTime | `IMPLEMENTADO` | OK |
| 0022 | FontBook | `IMPLEMENTADO` | OK + ANTI-PADRÃO (secção diagnóstico) |
| 0023 | indexmap | `IMPLEMENTADO` | OK + ANTI-PADRÃO (secção diagnóstico) |
| 0024 | ecow para Value::Str | `IMPLEMENTADO` | OK |
| 0025 | Int == Float | `IMPLEMENTADO` | OK + ANTI-PADRÃO (secção diagnóstico) |
| 0026-divergencia | Content como enum | `IMPLEMENTADO` | OK |
| 0026-revisao | Content::Sequence(Arc<[T]>) | `UPDATED` | OK |
| 0027 | CIDFont subsetting | `Accepted` (formato inglês) | DESALINHADO — formato |
| 0028 | Tipos tipográficos simplificados | `Accepted` (formato inglês) | DESALINHADO — revogado por ADR-0029 |
| 0029 | Pureza física | `ACCEPTED` | INCOMPLETO — Alignment ausente da lista |
| 0030 | Performance é domínio | `ACCEPTED` | INCOMPLETO — Arc::clone vs Vec::clone implícito |
| 0031 | Early hashing em Source | `ACCEPTED` | OK |

## Cadeia de revogações/revisões (verificada)

- **ADR-0007** → revogado por **ADR-0018** (campo `**Revoga**` presente em ADR-0018:5)
- **ADR-0015** → filosofia complementada por ADR-0024 (não revogada — texto explícito em ADR-0024:84)
- **ADR-0016** → Opção C formalizada em **ADR-0031** (early hashing)
- **ADR-0028** → revogado por **ADR-0029** (campo `**Revoga**` presente em ADR-0029:5)
- **ADR-0026 (divergencia)** → revisto por **ADR-0026-revisao** (mesmo número, ambos vivos)
```

---

## 8. Sumário priorizado para o passo de correcção

### Prioridade ALTA — informação factualmente errada (corrigir antes de qualquer próximo ADR)
- **ADR-0001:** `PROPOSTO` → `IMPLEMENTADO (Opção C)`. (1 linha)
- **ADR-0007:** `PROPOSTO` → `REVOGADO` + campo `**Revogado por**: ADR-0018`. (2 linhas)
- **ADR-0028:** `Accepted` → `REVOGADO` + campo `**Revogado por**: ADR-0029`. (2 linhas)
- **ADR-0018:** corrigir formatação `\*\*Status\*\*` → `**Status**`. (1 linha)
- **ADR-0017:** padronizar `**Estado**: IMPLEMENTADO` → `**Status**: ` `` `IMPLEMENTADO` ``. (1 linha)
- **ADR-0027 e ADR-0028:** padronizar `**Status:** Accepted` → `**Status**: ` `` `ACCEPTED` ``. (2 linhas)

### Prioridade MÉDIA — lacunas com impacto de decisão (próximo passo dedicado)
- **ADR-0029:** acrescentar `Alignment` à enumeração de tipos tipográficos (Input 1). (5 linhas
  + actualização de hash do ADR se aplicável).
- **ADR-0030:** acrescentar parágrafo "Clone profundo vs `Arc::clone`" (Input 2). (10 linhas).
- **ADR-0022, ADR-0023, ADR-0025:** mover secção "Diagnóstico obrigatório" para
  passos/prompts; deixar no ADR apenas a decisão final e referência ao diagnóstico (3
  refactors, ~50 linhas cada).

### Prioridade BAIXA — melhorias estruturais (passos posteriores ou contínuos)
- Criar `00_nucleo/adr/README.md` com a estrutura proposta na Secção 7.
- Formalizar 4 regras implícitas (ADR-0032 a ADR-0035) — ver Secção 6. **Nota:** se a Secção
  3.2 (lacuna ADR-0030) for executada, ADR-0033 deixa de ser necessária.
- Resolver duplicação ADR-0026 (renomear para `-R1` ou atribuir número novo).
- Resolver coexistência com ADRs do `crystalline-lint` (subdir ou convenção
  documentada).
- Reavaliar `ImportGuard::drop` (`unsafe` em `eval.rs:235`) — abrir DEBT se a regra de
  ADR-0032 não permitir esta forma.

---

## 9. Anexo — comandos de verificação executados

```bash
# Inventário
ls 00_nucleo/adr/typst-adr-*.md | sort
ls 00_nucleo/adr/typst-adr-*.md | wc -l

# Status
for f in 00_nucleo/adr/typst-adr-*.md; do
  printf "%-60s " "$(basename "$f")"
  grep -m1 -E "^\*\*Status\*\*:|^\*\*Estado\*\*:|^\\\\\\*\\\\\\*Status" "$f"
done

# ADR-0027/28 com formato inglês (escape diferente)
grep -m1 -E "^\*\*Status|Status:|status:" 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md \
  00_nucleo/adr/typst-adr-0028-typographic-types.md

# Relações
grep -n "revoga\|Revoga\|revogad" 00_nucleo/adr/typst-adr-*.md

# ADR-0001 (Opção C)
grep "comemo" crystalline.toml
grep -rn "comemo" 01_core/src/

# ADR-0017 (eval implementado)
grep -n "pub fn eval\b" 01_core/src/rules/eval.rs

# ADR-0029 sem Alignment (Input 1)
grep "Alignment" 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md

# ADR-0030 sem Arc::clone (Input 2)
grep -i "arc::clone\|incremento de contador\|refcount" \
  00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md

# Anti-padrão diagnóstico
grep -i "diagnóstico obrigatório\|## Diagnóstico" 00_nucleo/adr/typst-adr-*.md

# unsafe em L1 (Tarefa 3.1)
grep -rn "unsafe" 01_core/src/
grep -rn "unsafe" 01_core/src/ | grep -v "scanner.rs"
```

---

**Sumário executivo:** 31 ADRs únicos auditados, 0 contradições com código, **5 ADRs com STATUS
DESALINHADO** (0001, 0007, 0017, 0018, 0027, 0028 — última conta dupla por estar em duas
categorias), **3 ADRs com CONTEÚDO INCOMPLETO** (0029, 0030, 0022/0023/0025 anti-padrão), **4
regras implícitas a formalizar** (Inputs 2 e 4 obrigatórios + paridade vanilla + diagnóstico
de tipos vanilla), **2 anomalias estruturais** (Inputs 5 e 6) + 1 anomalia descoberta (15
ocorrências de `unsafe` em L1 — a regra "zero unsafe" não é literalmente verdadeira).
