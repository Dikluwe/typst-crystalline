# Passo 84.3 — Resolução de NodeKind por identidade em vez de string (DEBT-21)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — entrada DEBT-21 actualizada no Passo 84.1
  (cabeçalho "MITIGADO (Passo 70), desbloqueado (Passo 84.1)").
- `01_core/src/rules/eval.rs` — localização de `apply_show_rules`,
  `active_guards`, e o uso de `Func::name()`.
- `01_core/src/entities/show.rs` — definição de `ShowRule`, `Selector`,
  `NodeKind`, `RuleId`.
- `01_core/src/entities/func.rs` — definição de `NativeFunc` e
  `EvalContext::register`.

Pré-condição: `cargo test` — 902 testes (732 L1 + 170 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.2 concluído. DEBT-38
encerrado.

---

## Restrições arquiteturais

Três regras que condicionam este passo:

1. **Sem `unsafe` em L1** (convenção cristalina, consistente com
   ADR-0004, ADR-0005, ADR-0019, ADR-0029). `fn_addr_eq` é safe —
   permitido.

2. **ADR-0018 — `rustc_hash` em L1**: já disponível. Se a solução
   precisar de `HashSet<fn(...)>` ou similar, não há barreira
   arquitectural.

3. **ADR-0030 — Performance é domínio**: substituir comparação de
   strings por comparação de ponteiros (ou de discriminants) reduz o
   custo de `apply_show_rules` de O(L) por invocação (L = length do
   nome) para O(1). Isto é trabalho de domínio, não "optimização
   prematura".

---

## Natureza deste passo

Este passo tem **duas fases sequenciais e separadas**:

**Fase de diagnóstico (obrigatória, bloqueante)**: executar os
comandos `grep` da Tarefa 1 e reportar o output completo ao
utilizador **antes** de escrever qualquer código. A solução concreta
depende do que o diagnóstico revelar — existem três cenários possíveis
que levam a soluções tecnicamente diferentes.

**Fase de implementação (dependente)**: escolher a Tarefa 2 (cenário A,
B ou C) com base no resultado do diagnóstico e executá-la.

**Regra absoluta**: Claude Code **não escolhe um cenário sem apresentar
primeiro o diagnóstico ao utilizador**. Se o diagnóstico for ambíguo
ou misturar cenários (ex: o projecto usa `Func::name()` em alguns
sítios e `NodeKind` noutros), reportar a ambiguidade e esperar
decisão humana.

---

## Tarefa 1 — Diagnóstico obrigatório

Executar **todos** os comandos abaixo e registar o output completo
no relatório ao utilizador.

### 1.1 — Definição de `Func` e `NativeFunc`

```bash
# Como Func está representado — enum, Arc<Repr>, struct com Arc?
grep -B 2 -A 15 "pub struct Func\|pub enum Func" 01_core/src/entities/func.rs

# Como NativeFunc está declarado — tem ponteiro de função bare?
grep -B 2 -A 15 "pub struct NativeFunc\|pub enum NativeFunc" 01_core/src/entities/func.rs

# O método Func::name() — como extrai o nome?
grep -B 1 -A 10 "fn name" 01_core/src/entities/func.rs
```

### 1.2 — Definição de `NodeKind`, `Selector`, `ShowRule`

```bash
# NodeKind é um enum plano ou carrega dados?
grep -B 2 -A 20 "pub enum NodeKind" 01_core/src/entities/show.rs

# Selector — como identifica o que filtrar?
grep -B 2 -A 15 "pub enum Selector\|pub struct Selector" 01_core/src/entities/show.rs

# ShowRule — qual o campo de identidade?
grep -B 2 -A 10 "pub struct ShowRule" 01_core/src/entities/show.rs

# RuleId — o que é?
grep -B 2 -A 10 "pub struct RuleId\|pub type RuleId" 01_core/src/entities/show.rs
```

### 1.3 — Uso actual de `Func::name()` em selectors

```bash
# Todos os call sites de Func::name()
grep -rn "\.name()" 01_core/src/rules/ 01_core/src/entities/show.rs

# Comparações de string em apply_show_rules — alvo directo do DEBT-21
grep -B 2 -A 5 "name ==\|name() ==" 01_core/src/rules/eval.rs 01_core/src/entities/show.rs

# Como as show rules são construídas e registadas
grep -B 2 -A 15 "apply_show_rules\|show_rules.push\|ShowRule::new" 01_core/src/rules/eval.rs
```

### 1.4 — Relação entre `Selector` e `Content`

```bash
# Se selector é por NodeKind, como se extrai o NodeKind de um Content?
grep -B 2 -A 10 "fn node_kind\|NodeKind::from\|match.*Content.*NodeKind" 01_core/src/

# Se selector é por Func, como se associa Content a Func?
grep -rn "Content::FuncCall\|Content.*func\|func_addr" 01_core/src/entities/content.rs
```

---

## Tarefa 1.5 — Classificação do cenário

Com base no output do diagnóstico, classificar em um dos três cenários:

### Cenário A — Selector identifica `Func`

**Sinais de A**:
- `ShowRule` tem campo do tipo `Func` ou `*const ()`.
- Comparações actuais fazem `rule.func.name() == content_func.name()`.
- O caminho do show rule é: utilizador escreve `#show heading: it => ...`,
  o parser constrói selector referindo `heading` como uma `Func`,
  `apply_show_rules` compara por nome desta `Func`.

**Implicação**: `fn_addr_eq` sobre ponteiros de função nativa é a
solução correcta. Closures (show rules de utilizador) não são
comparadas entre si — são o **transformador**, não o **selector**.

### Cenário B — Selector identifica `NodeKind` (enum sem dados)

**Sinais de B**:
- `NodeKind` é enum com variantes simples: `Heading, Strong, Emph, Raw,
  Equation, ListItem, ...`.
- `ShowRule` tem campo `selector: Selector` onde `Selector::Node(NodeKind)`.
- Comparações actuais fazem `rule.kind == extract_node_kind(content)`
  ou similar.

**Implicação**: `std::mem::discriminant(&a) == std::mem::discriminant(&b)`
(se o enum carrega dados) ou `a == b` directo (se é enum plano com
`#[derive(PartialEq)]`). `fn_addr_eq` **não se aplica a este cenário**.

O DEBT-21 é menos sobre "string vs ponteiro" e mais sobre "string vs
discriminant". Neste caso, a resolução textual por `Func::name()` não
está a comparar ponteiros — está a converter `Func` → `&str` → match
contra literal. Substituir por `NodeKind` directo.

### Cenário C — Mistura

**Sinais de C**:
- Diferentes tipos de show rule usam diferentes selectors (ex:
  `#show heading: ...` usa `NodeKind`, mas `#show my_fn: ...` usa
  `Func`).
- Ou: o projecto evoluiu e tem resquícios dos dois modelos.

**Implicação**: Ambas as soluções (A e B) aplicam-se em sítios
diferentes. Requer decisão sobre unificação vs coexistência —
reportar ao utilizador antes de avançar.

---

## Tarefa 2A — Implementação (se Cenário A)

### 2A.1 — Adicionar método `fn_addr()` a `NativeFunc`

Em `01_core/src/entities/func.rs`:

```rust
impl NativeFunc {
    /// Endereço da função nativa como identidade opaca.
    ///
    /// Usado para comparar dois NativeFunc por identidade em vez de por
    /// nome — substitui `name() == "heading"` por `fn_addr_eq(...)`.
    ///
    /// Safe: retorna `fn(...)` (function pointer), não `*const c_void`.
    /// O tipo exposto é o tipo canónico da assinatura de funções nativas.
    pub fn fn_addr(&self) -> fn(&mut EvalContext, &Args) -> Result<Value, String> {
        // O campo exacto depende da representação de NativeFunc —
        // adaptar à definição real confirmada no diagnóstico.
        self.call
    }
}
```

Se `NativeFunc` é apenas `pub struct NativeFunc { pub call: fn(...) }`,
então o método pode ser simplesmente `self.call` directo sem método
auxiliar. Preferir o método para estabilizar a API — assim se a
representação interna mudar, call sites não precisam de actualização.

### 2A.2 — Substituir comparações por `fn_addr_eq`

Em `01_core/src/rules/eval.rs`, no ponto onde `apply_show_rules`
compara a identidade da função da rule com a função do conteúdo:

**Antes** (exemplo — adaptar ao código real):

```rust
if rule.func.name() == content_func.name() {
    // aplicar transformação
}
```

**Depois**:

```rust
if std::ptr::fn_addr_eq(rule.func.fn_addr(), content_func.fn_addr()) {
    // aplicar transformação
}
```

**Nota sobre closures**: `fn_addr_eq` só compara function pointers
(`fn(...)`), não closures. Closures em Rust não têm identidade estável
comparável — cada instância é um tipo único. Se o código actual já
assume que closures não são comparáveis como selectors (o utilizador
não pode escrever `#show <closure>: ...`), a substituição é directa.
Se o código tenta comparar closures via `name()`, o diagnóstico deve
ter revelado esse caminho — reportar como ambiguidade.

### 2A.3 — Manter `Func::name()` para mensagens de erro

`Func::name()` **não é removido**. Continua a ser usado para:
- Mensagens de erro legíveis ("função `heading` não aceita argumento X").
- Introspecção de debug.
- Display em traços de stack.

A mudança é apenas: `apply_show_rules` deixa de usar `name()` para
**identidade**. Name fica para **apresentação**.

### 2A.4 — Teste de regressão para aliasing

O aliasing é o bug que `fn_addr_eq` resolve e que `Func::name()`
não detectava. Adicionar um teste que exercita o caso:

```rust
#[test]
fn show_rule_resolve_por_identidade_nao_por_nome() {
    // Registar uma função nativa com nome "heading" e uma show rule
    // que selecciona essa função. Depois criar um alias local da
    // mesma função com um nome diferente.
    //
    // Com resolução por nome (DEBT-21 original): a show rule não
    // disparava para o alias porque os nomes não coincidiam — mas
    // era a MESMA função.
    //
    // Com resolução por ponteiro: a show rule dispara para ambos
    // (função original e alias) porque o ponteiro é o mesmo.
    //
    // Este teste documenta o comportamento correcto pós-Passo 84.3.
    // ...
}
```

A construção exacta do teste depende da API de registo de funções
nativas em `EvalContext` — adaptar ao padrão usado nos testes
existentes de show rules.

---

## Tarefa 2B — Implementação (se Cenário B)

### 2B.1 — Confirmar que `NodeKind` tem `PartialEq`

```bash
grep -B 2 "pub enum NodeKind" 01_core/src/entities/show.rs
```

Se o enum já deriva `PartialEq, Eq, Hash`, a substituição é directa.
Se não, adicionar os derives — `NodeKind` é enum plano sem dados
conforme o cenário B, portanto os derives são triviais.

### 2B.2 — Substituir comparações por string por igualdade de enum

Em `01_core/src/rules/eval.rs`, no ponto onde `apply_show_rules`
classifica o `Content`:

**Antes** (exemplo):

```rust
let kind_name = extract_kind_name(content);  // retorna &str
if rule.kind_name == kind_name {
    // aplicar transformação
}
```

**Depois**:

```rust
let kind = NodeKind::of(content);  // retorna NodeKind
if rule.kind == kind {
    // aplicar transformação
}
```

Onde `NodeKind::of(&Content) -> NodeKind` é a função canónica de
classificação (provavelmente já existe em `introspect.rs` ou
equivalente — o diagnóstico em 1.4 deve ter revelado).

### 2B.3 — Remover strings de `Selector` se já não forem usadas

Se `Selector::Node(&str)` existir como variante de string, substituir
por `Selector::Node(NodeKind)`. Actualizar todos os call sites.

Isto pode gerar cascata de alterações — se muitos sítios constroem
selectors por string (ex: parser ao processar `#show heading: ...`),
cada um precisa de converter string → `NodeKind` na construção.

**Critério de paragem**: se a cascata ultrapassar ~5 ficheiros ou
~30 alterações, parar e reportar. Pode ser sinal de que o passo
deve ser partido em 84.3a (infra de `NodeKind`) e 84.3b (migração
dos call sites).

### 2B.4 — Teste de regressão

O DEBT-21 no cenário B é sobre robustez a typos e a mudanças de nome.
Teste:

```rust
#[test]
fn show_rule_resolve_por_enum_nao_por_string() {
    // Verificar que uma show rule registada para NodeKind::Heading
    // dispara para um Content::Heading independentemente de qualquer
    // representação textual do tipo. Isto elimina a classe de bugs
    // onde uma mudança de case ou typo no nome ("Heading" vs "heading")
    // silenciosamente desactiva a rule.
    // ...
}
```

---

## Tarefa 2C — Implementação (se Cenário C)

Cenário C significa que o código tem os dois padrões misturados.
Antes de codificar, **escrever um mini-relatório** ao utilizador com:

1. Lista de sítios que usam padrão A (identidade de `Func`).
2. Lista de sítios que usam padrão B (comparação de `NodeKind`).
3. Proposta: unificar para A, unificar para B, ou manter os dois.

Critério de desempate (para sugestão, decisão final é do utilizador):
- Se a maioria dos selectors é `#show <funcao_nativa>: ...` (ex:
  `#show heading: ...`), o modelo natural é `Func` + `fn_addr_eq`.
- Se a maioria é `#show <tipo>: ...` (ex: ainda não existe esta
  sintaxe no projecto), o modelo natural é `NodeKind`.

**Não avançar com implementação sem confirmação explícita da direcção
unificada.** Cenário C expõe uma decisão arquitectural que merece
ADR — potencialmente ADR-0032 "Modelo de selector para show rules".

---

## Tarefa 3 — Actualizar DEBT-21

Mover DEBT-21 da Secção 1 para Secção 2, com entrada consolidada:

```markdown
## DEBT-21 — Resolução de NodeKind por string — **ENCERRADO (Passo 84.3)** ✓

**Registado no Passo 70. Mitigado no Passo 70. Desbloqueado no Passo 84.1. Resolvido no Passo 84.3.**

[texto adaptado ao cenário A, B ou C escolhido — descrever a solução
aplicada e o comportamento pós-resolução]
```

Se cenário A: mencionar `std::ptr::fn_addr_eq` e `Func::fn_addr()`.
Se cenário B: mencionar `NodeKind` + derives + função canónica `NodeKind::of`.
Se cenário C: mencionar a decisão unificada e referenciar ADR se foi
criada.

---

## Tarefa 4 — Verificação

```bash
# Testes de regressão.
cargo test

# Nenhuma nova violação de linter.
crystalline-lint .

# Clippy — apenas warnings pré-existentes permitidos.
cargo clippy --all-targets 2>&1 | grep -v "TransformMatrix\|integration test"

# Grep de confirmação — Func::name() só aparece em contextos de
# apresentação (erro, debug), nunca em apply_show_rules.
grep -n "\.name()" 01_core/src/rules/eval.rs
```

Resultado esperado: o grep de `.name()` em `eval.rs` não deve
aparecer dentro ou adjacente a `apply_show_rules`. Se aparecer,
verificar se é caminho de erro (legítimo) ou caminho de identidade
(regressão).

---

## Critérios de conclusão

Critérios comuns aos três cenários:

- [ ] Tarefa 1 (diagnóstico) executada e reportada ao utilizador
  antes de qualquer edição de código.
- [ ] Cenário classificado e reportado ao utilizador (A, B ou C).
- [ ] Se cenário C, decisão do utilizador registada antes de codificar.
- [ ] `Func::name()` mantido para mensagens de erro e debug.
- [ ] Nenhum uso de `unsafe` ou cast de ponteiro.
- [ ] Teste de regressão adicionado que demonstra robustez a
  aliasing (A) ou typos/mudança de case (B).
- [ ] DEBT-21 movido para Secção 2 com nota de resolução.
- [ ] `cargo test` mantém ou aumenta o número de testes (um novo
  de regressão).
- [ ] `crystalline-lint .` zero violations.

Critérios específicos do cenário A:

- [ ] Método `fn_addr()` adicionado a `NativeFunc`.
- [ ] `std::ptr::fn_addr_eq` usado em `apply_show_rules`.
- [ ] Teste `show_rule_resolve_por_identidade_nao_por_nome` passa.

Critérios específicos do cenário B:

- [ ] `NodeKind` deriva `PartialEq, Eq, Hash` (se ainda não derivava).
- [ ] `Selector::Node` usa `NodeKind` em vez de `String`.
- [ ] Teste `show_rule_resolve_por_enum_nao_por_string` passa.

---

## Ao terminar, reportar

Bloco 1 — Diagnóstico:
- Output dos comandos da Tarefa 1 (resumido se longo).
- Cenário classificado (A, B ou C) com justificação de uma frase.

Bloco 2 — Implementação:
- Qual Tarefa 2 foi executada (2A, 2B ou 2C).
- Ficheiros alterados.
- Número de linhas afectadas (indicação aproximada).

Bloco 3 — Testes e linter:
- Número total de testes (esperado: 733 L1 + 170 L3).
- Resultado do `crystalline-lint .`.
- Resultado do grep final de `.name()`.

**Go/No-Go para o Passo 84.4** (candidato: DEBT-22 — clone de
show_rules por nó, via `Arc<[ShowRule]>`):

- **GO — resolução por identidade funciona**: testes passam, o
  cenário foi classificado correctamente, comportamento observável
  preservado (show rules existentes continuam a disparar).
- **NO-GO — cenário classificado incorrectamente**: se o diagnóstico
  apontou A mas o código tinha padrão B, a implementação 2A vai
  falhar a compilar. Voltar à Tarefa 1, refazer a classificação.
- **NO-GO — cascata de alterações maior que prevista (cenário B)**:
  se a migração `Selector::Node(&str) → Selector::Node(NodeKind)`
  afectou mais de 5 ficheiros ou criou instabilidade em testes
  não relacionados, parar e partir o passo em 84.3a / 84.3b.
- **NO-GO — ambiguidade no cenário C não resolvida**: se o Claude
  Code avançou com implementação de C sem confirmação explícita do
  utilizador sobre a direcção unificada, reverter e pedir decisão.
