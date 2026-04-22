# Passo 90 — Materializar `Route`, integrar no eval, fechar DEBT-40

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla.
- `00_nucleo/adr/typst-adr-0034-*.md` — diagnóstico obrigatório
  antes de materializar tipo vanilla.
- `00_nucleo/adr/typst-adr-0032-*.md` — política de `unsafe` em
  L1. Este passo elimina o último `unsafe` de `eval.rs`
  (o do `ImportGuard::drop`).
- `00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`
  — diagnóstico completo do `Route` do vanilla. É a base
  estrutural deste passo.
- `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
  — secção `Route` (classificação de dependências).
- `00_nucleo/DEBT.md` — entrada DEBT-40 (será movida para Secção
  2 no fim deste passo).
- `01_core/src/entities/world_types.rs` — stub actual de `Route`.
- `01_core/src/rules/eval.rs` — código actual com
  `EvalContext.import_stack`, `ImportGuard`, `enter_import`,
  detecção de ciclos.
- `lab/typst-original/crates/typst-library/src/engine.rs:251` —
  `Route` original para replicação.

Pré-condição: `cargo test` — 914 testes (740 L1 + 174 L3, 6
ignorados pré-existentes), zero violations. Passo 89 concluído
(ADR-0035 em vigor, DEBT-43 aberto).

---

## Natureza deste passo

Passo único de construção. É **grande** por natureza: materializa
`Route`, integra-o no `eval`, remove o mecanismo antigo
(`import_stack` + `ImportGuard`), e fecha o DEBT-40.

**Justificação para não decompor em sub-passos**: a política do
projecto distingue "sub-passos para pagar DEBTs" (permitidos) de
"sub-passos em construção" (proibidos). Este passo **paga o
DEBT-40** como efeito secundário, mas a actividade principal é
construção. Decompor criaria estado intermédio onde `Route`
existe em L1 mas não é usado (stub vestigial) — pior do que o
estado actual.

O passo inclui **pontos de verificação intermédios** (`cargo
check` após cada tarefa) para detectar quebras cedo sem sair do
passo.

Escolha arquitectural: `Route` como **struct concreta** com
`#[comemo::track]` em `impl` block, seguindo padrão vanilla
categoria 1 (observado no Passo 86).

---

## Decisões formalizadas neste passo

- ADR-0033 (paridade funcional) — replicar `Route` linha-a-linha
  com vanilla.
- ADR-0032 (sem `unsafe` em L1) — eliminação do último `unsafe`
  em `eval.rs` (`ImportGuard::drop`).
- ADR-0034 (diagnóstico obrigatório) — satisfeito pelo Passo 85.

---

## Diagnóstico prévio

Ver
`00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`.
Resumo factual do que vai ser materializado:

- **Struct**: `Route<'a> { outer: Option<Tracked<'a, Self>>,
  id: Option<FileId>, len: usize, upper: AtomicUsize }`.
- **Forma de propagação**: frame por valor, `outer` como
  linked list imutável via `Tracked` (lifetimes).
- **`#[comemo::track]`**: sobre `contains` e `within` (métodos
  de consulta que beneficiam de tracking granular).
- **Outros métodos**: `root`, `extend`, `with_id`, `unnested`,
  `track`, `increase`, `decrease`, 4 × `check_*_depth`.
- **Detecção de ciclo**: `contains(id)` percorre linked list via
  `outer`. Complexidade O(profundidade).

---

## Sequência de tarefas (executar em ordem)

### Tarefa 1 — Materializar `Route` em `world_types.rs`

#### 1.1 — Reler o vanilla

```bash
# Confirmar localização exacta e ver código completo:
grep -B 2 -A 40 "pub struct Route" \
    lab/typst-original/crates/typst-library/src/engine.rs

# Ver todos os métodos do impl:
grep -B 1 -A 5 "impl.*Route\|#\[comemo::track\]\|fn contains\|fn within\|fn root\|fn extend\|fn with_id\|fn unnested\|fn track\|fn increase\|fn decrease\|fn check_" \
    lab/typst-original/crates/typst-library/src/engine.rs
```

Confirmar que a forma do vanilla corresponde ao diagnóstico do
Passo 85. Se divergir em algum campo ou método, **parar** e
reportar antes de prosseguir.

#### 1.2 — Substituir o stub

Localização: `01_core/src/entities/world_types.rs`.

Substituir o stub `pub struct Route(());` + `impl Route {}` +
possível `#[comemo::track] impl Route {}` pelo tipo funcional
completo replicando vanilla.

**Importante**: a forma exacta, incluindo lifetimes, derives,
ordem dos campos, e assinatura de cada método, deve vir da
leitura em 1.1 — não inventar, não simplificar.

Nota sobre lifetimes: `Route<'a>` é o **primeiro tipo cristalino
com lifetime explícito** além dos que já existem em `Scopes<'a>`.
Se o linter tiver regra contra lifetimes em L1 (improvável, mas
verificar), seguir o vanilla e reportar o conflito — não
contornar.

#### 1.3 — Adicionar importações necessárias

`AtomicUsize` vem de `std::sync::atomic`. `Tracked` vem de
`comemo`. Ambos já autorizados (`std` implicitamente, `comemo`
via ADR-0001).

#### 1.4 — Verificação intermédia

```bash
cargo check --package typst-core 2>&1 | tail -20
```

Esperado: compila. Se não compilar, depurar antes de prosseguir
para a Tarefa 2.

---

### Tarefa 2 — Testes unitários de `Route`

Adicionar no mínimo 6 testes em `world_types.rs` (ou módulo de
testes apropriado):

- `route_root_nao_contem_nenhum_ficheiro` — `Route::root()`
  cria route sem `id`, `contains(qualquer_id)` retorna `false`.
- `route_com_id_contem_proprio_id` — `Route::root().with_id(fid)`
  retorna `true` para `contains(fid)`.
- `route_extend_adiciona_ao_stack` — cadeia de `extend` forma
  linked list observável via `contains`.
- `route_contains_detecta_ciclo` — quando o mesmo `fid` aparece
  na cadeia, `contains` retorna `true`.
- `route_increase_decrease_equilibrado` — `increase` seguido de
  `decrease` retorna o contador ao valor original.
- `route_check_depth_rejeita_profundidade_excessiva` — um dos
  `check_*_depth` retorna `Err` quando o limite é excedido.

Os nomes e semântica exactos dos testes dependem da API real
confirmada na Tarefa 1. Ajustar se a API divergir do diagnóstico.

#### 2.1 — Verificação intermédia

```bash
cargo test --package typst-core route 2>&1 | tail -10
```

Esperado: 6 novos testes passam. Contagem L1 sobe de 740 para
746. Se algum falhar, depurar antes da Tarefa 3.

---

### Tarefa 3 — Integrar `Route` no `eval`

#### 3.0 — Verificação estrutural pré-integração (bloqueante)

Antes de qualquer alteração ao eval, executar:

```bash
# Forma actual do EvalContext (tem lifetime próprio?):
grep -n "struct EvalContext\|impl.*EvalContext\|impl EvalContext" \
    01_core/src/rules/eval.rs | head -5

# Quantas funções de eval existem (dimensiona refactor):
grep -cn "^\s*fn eval_\|^\s*pub fn eval_\|^pub fn eval\b" \
    01_core/src/rules/eval.rs

# Quais check_*_depth já existem no cristalino (funcionalidade actual):
grep -n "check_.*_depth\|tick_loop\|exceeded\|too deep\|too much" \
    01_core/src/rules/eval.rs | head -20

# comemo::track já é usado noutros tipos em L1?
grep -rn "#\[comemo::track\]\|#\[track\]" 01_core/src/ --include="*.rs"
```

Classificar e reportar:

**Classe A — EvalContext com lifetime próprio já existente**
(`pub struct EvalContext<'a> { ... }`). `Route<'a>` pode ser
embedded como campo. Refactor interno ao contexto, não
transversal. **Go directo para 3.1**.

**Classe B — EvalContext sem lifetime e ≤ 5 funções de eval**.
Adicionar `&Route<'_>` como parâmetro a essas funções é aceitável.
**Go para 3.1 com Opção 2 (passar como parâmetro)**.

**Classe C — EvalContext sem lifetime e > 10 funções de eval**.
Adicionar parâmetro força alteração transversal. **Parar**.
Reportar a classe C e accionar o No-Go Parcial descrito no fim
do enunciado: manter Tarefas 1, 2, 4 concluídas; abrir DEBT-44
para integração futura; não fechar DEBT-40.

**Classe D — Caso ambíguo** (EvalContext sem lifetime e 6-10
funções): decisão humana. Parar, reportar, pedir decisão antes
de continuar.

Também reportar:
- Quais `check_*_depth` existem actualmente no cristalino (pode
  haver 0, 1, 2 ou 4). Esta informação determina se materializar
  os 4 do vanilla é **refactor** (os 4 já existem) ou
  **adição de funcionalidade** (faltam alguns).
- Se `#[comemo::track]` já aparece em outros tipos L1 (`World`,
  `Library`, `Traced` materializado no Passo 88). Se já aparece,
  adicionar em `Route` é coerente com o padrão em uso.

Sem a verificação 3.0, as tarefas seguintes podem executar-se
com premissas incorrectas.

#### 3.1 — Mapear mecanismo actual

Localizar em `01_core/src/rules/eval.rs`:

```bash
grep -n "import_stack\|ImportGuard\|enter_import\|stack_ptr" \
    01_core/src/rules/eval.rs
```

Identificar:
- Definição de `ImportGuard` e `impl Drop for ImportGuard` (local
  do `unsafe` a eliminar).
- Campo `import_stack: Vec<FileId>` em `EvalContext`.
- Método `enter_import(&mut self, id: FileId) -> ImportGuard`
  (ou assinatura equivalente).
- Pontos de chamada: provavelmente em `Expr::ModuleImport` e
  `Expr::ModuleInclude`.
- Lógica de detecção de ciclo (`self.import_stack.contains(&id)`).

#### 3.2 — Decisão arquitectural para propagação

O `Route` do vanilla é propagado **por valor** com lifetime
`'a`. O `EvalContext` cristalino actual é propagado **por
referência mutável** (`&mut self`).

Opções:
1. **Substituir `import_stack` por `route: Route<'static>`**
   mantendo o estilo actual (Route vive no EvalContext, sem
   linked list de verdade). Simples, mas perde o padrão vanilla.
2. **Passar `&Route<'a>` como parâmetro a `eval_module` e
   afins**, criando novo Route em cada recursão. Mais próximo
   do vanilla, requer mudança de assinaturas.

**Escolha recomendada pelo enunciado**: Opção 2. Justificação:
a ADR-0033 pede paridade funcional com vanilla; a Opção 1
manteria a divergência que o DEBT-40 registou. Se Claude Code
detectar que a Opção 2 impõe refactor transversal (muitas
assinaturas a mudar), **parar** e reportar — pode ser passo
próprio.

#### 3.3 — Substituir mecanismo

Remover:
- Campo `import_stack: Vec<FileId>` de `EvalContext`.
- Struct `ImportGuard` inteiro.
- `impl Drop for ImportGuard` (elimina o `unsafe`).
- Método `enter_import`.

Adicionar:
- Parâmetro `route: &Route<'_>` nas funções relevantes
  (`eval_module`, `eval_expr` ou subconjunto que precise).
- Nos pontos de `#import`/`#include`: verificação via
  `route.contains(file_id)` em vez de
  `import_stack.contains(&file_id)`.
- Criação de novo `Route` via `route.extend(file_id)` (ou
  método equivalente do vanilla) antes da recursão.

Mensagens de erro de ciclo devem ser preservadas (texto
idêntico ao actual).

#### 3.4 — Actualizar testes existentes de import cycle

Localizar:

```bash
grep -n "import.*cycle\|cyclic\|import_stack" \
    01_core/src/rules/eval.rs 01_core/tests/ 2>/dev/null
```

Os testes que exercitam detecção de ciclo devem continuar a
passar **sem alteração na asserção** — só a implementação
interna muda, não o comportamento observável.

Se algum teste accede a `EvalContext.import_stack` directamente
para inspeccionar estado interno, reescrever para usar a API
pública do `Route` ou remover se for teste de implementação
(não de comportamento).

#### 3.5 — Teste de integração end-to-end obrigatório

Adicionar ao menos **um** teste de integração que valide
comportamento observável do ciclo de imports, independentemente
da implementação interna:

```rust
#[test]
fn import_cycle_detectado_retorna_err_sem_panic() {
    // Cenário: dois módulos A e B onde A importa B e B importa A.
    // Ou variante mais simples: módulo A importa a si próprio.
    //
    // Expectativa:
    // 1. eval retorna Err (não Ok, não panic, não stack overflow).
    // 2. Mensagem de erro identifica que é ciclo de imports.
    // 3. A cadeia de imports reportada no erro contém pelo menos
    //    um dos FileId envolvidos.
    //
    // Valor: este teste é independente de Route. Valida a
    // fronteira de comportamento que a ADR-0033 exige preservar.
    // Se a materialização do Route partir esta garantia, o teste
    // falha antes do fim do passo.
}
```

O teste deve usar a API pública do `eval` (não depender de
detalhes internos como `import_stack` ou `Route`). É
complemento dos 6 testes unitários da Tarefa 2, não
substituto.

Localização sugerida: junto dos testes de import já existentes,
ou como módulo `#[cfg(test)] mod import_cycle_tests` se o
ficheiro não tiver estrutura de testes clara.

Esperado: teste adicionado passa. Contagem L1 sobe de 746 para
747.

#### 3.6 — Verificação intermédia

```bash
cargo check --package typst-core 2>&1 | tail -20
cargo test --package typst-core import 2>&1 | tail -20
```

Esperado: compila; testes de import (incluindo ciclos e o novo
E2E) passam. Se falharem, depurar antes da Tarefa 4.

---

### Tarefa 4 — Verificar que `unsafe` foi eliminado de `eval.rs`

```bash
grep -n "unsafe" 01_core/src/rules/eval.rs
```

Esperado: **zero ocorrências**. Se restar alguma, investigar —
o DEBT-40 só fecha se o `unsafe` desaparecer por completo.

Verificar também `world_types.rs` após a materialização de
`Route`:

```bash
grep -n "unsafe" 01_core/src/entities/world_types.rs
```

Esperado: zero ocorrências. `Route` com `Tracked` + `AtomicUsize`
não requer `unsafe` directo — é encapsulado por `comemo` e pela
stdlib.

---

### Tarefa 5 — Fechar DEBT-40 no `DEBT.md`

Mover a entrada DEBT-40 da Secção 1 (em aberto) para a Secção 2
(encerrados). Preservar o texto original e acrescentar linha
final:

```markdown
**Resolvido no Passo 90.** `ImportGuard` e raw pointer eliminados.
`EvalContext.import_stack` substituído por `Route<'a>`, integrado
no mecanismo de eval segundo a forma determinada pela verificação
3.0 do Passo 90 (embedded no contexto se Classe A, propagado como
referência em chamadas recursivas se Classe B). Fecha
simultaneamente divergência estrutural face ao vanilla (ADR-0033)
e elimina último `unsafe` de `eval.rs` (ADR-0032).
```

Nota ao Claude Code: após completar a Tarefa 3, substituir a
frase entre parênteses pela forma que foi efectivamente
implementada, removendo a parte não aplicável. O texto final
deve reflectir a realidade do código, não a ambiguidade
pré-verificação.

Não renumerar entradas. Inserir em Secção 2 após DEBT-41
(encerrado no Passo 85).

---

### Tarefa 6 — Verificação final

```bash
# Contagem total de testes (esperado: 747 L1 + 174 L3 + 6 ignor):
cargo test --workspace 2>&1 | tail -10

# Linter:
cargo run --package crystalline-lint 2>&1 | tail -5

# Zero unsafe em L1 (excepto o do scanner, DEBT-42 bloqueado):
grep -rn "unsafe" 01_core/src/ --include="*.rs" | grep -v "scanner.rs"
```

Esperado:
- 747 L1 + 174 L3 + 6 ignorados = 921 + ignorados testes.
- Zero violations no linter.
- `grep -v scanner.rs` retorna zero linhas (único `unsafe`
  restante em L1 é o do DEBT-42 bloqueado).

---

## Critérios de conclusão

- [ ] Stub `Route(())` substituído por `Route<'a>` funcional com
      forma vanilla.
- [ ] `#[comemo::track] impl Route { ... }` tem ao menos
      `contains` e `within`, seguindo vanilla.
- [ ] Pelo menos 6 novos testes unitários de `Route` passam.
- [ ] Um teste E2E adicional valida detecção de ciclo via API
      pública do eval.
- [ ] `ImportGuard` e `impl Drop for ImportGuard` removidos de
      `eval.rs`.
- [ ] Campo `import_stack` removido de `EvalContext`.
- [ ] Zero ocorrências de `unsafe` em `eval.rs`.
- [ ] Testes de detecção de ciclo de import continuam a passar
      (comportamento observável preservado).
- [ ] DEBT-40 movido para Secção 2 do `DEBT.md` com linha de
      resolução.
- [ ] Contagem total: 747 L1 + 174 L3 + 6 ignorados.
- [ ] `crystalline-lint` reporta zero violations.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Tarefa 1 (Route materializado):
- Linhas alteradas em `world_types.rs`.
- Campos finais de `Route` (confirmar correspondência com vanilla).
- Lifetimes usados.

Tarefa 2 (testes):
- Número de testes novos adicionados.
- Contagem L1 final.

Tarefa 3 (integração no eval):
- Opção escolhida (1 ou 2) e justificação se divergiu da
  recomendada.
- Linhas alteradas em `eval.rs`.
- Assinaturas modificadas (lista curta).

Tarefa 4 (eliminação de `unsafe`):
- Confirmação de zero `unsafe` em `eval.rs` e `world_types.rs`.

Tarefa 5 (DEBT-40):
- Confirmação de movimentação correcta.

Tarefa 6 (verificação final):
- Contagens de testes e violações.

Go/No-Go para Passo 91:
- **Go** se todas as tarefas foram concluídas e verificações
  passaram. Próximo passo provável = materializar dependências
  folha de `Styles` (`Style` e `LazyHash`), ou materializar
  `Introspection` para desbloquear `Sink`. Decisão no Passo 91
  depende do teu interesse arquitectural.
- **No-Go parcial** (reverter integração, manter Route isolado)
  se a Tarefa 3 revelar refactor transversal maior do que o
  Passo 90 comporta. Nesse caso:
  - `Route` fica materializado mas **não integrado** no eval.
  - DEBT-40 **não fecha** neste passo.
  - Abrir DEBT-44 registando "integração do `Route` no eval"
    como passo futuro dedicado.
  - Reverter parcialmente as tarefas 3 e 5; manter tarefas 1,
    2 e 4 (onde 4 vira verificação parcial).
