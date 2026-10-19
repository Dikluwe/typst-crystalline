# Passo 109 — Materializar `Engine<'a>` em L1

**Série**: 109 (passo de construção; 6ª aplicação da ADR-0036,
em forma inversa — consolidação em vez de extracção).
**Precondição**: Passo 108 encerrado; análise produziu recomendação
Candidato 5; 803 L1 + 184 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0026 (divergência de Content — precedente
de paridade funcional sem paridade de implementação), ADR-0036
(atomização — base para inversão), ADR-0037 (coesão por domínio),
ADR-0042 (Sink), ADR-0043 (canal Sink).
**ADR nova**: ADR-00NN "Engine<'a> como agregador de estado de
eval em L1" — `PROPOSTO` em 109.B, `EM VIGOR` em 109.E.

---

## Objectivo

Criar `Engine<'a, ...>` em L1 como struct agregadora dos campos que
hoje são passados como parâmetros individuais às funções `eval_*`.
Substituir `N` parâmetros individuais por 1 parâmetro `&mut Engine`.

Decisões de design já tomadas (contexto anterior ao enunciado):

1. **Âmbito**: subconjunto compatível em lifetime. Decisão final em
   109.A com base no inventário. Campos com lifetimes conflituantes
   ficam fora e continuam como parâmetros individuais.
2. **Nomes dos campos**: paridade com vanilla (`world`, `route`,
   `sink`, `styles`, `show_rules`, `active_guards`, `current_file`,
   `figure_numbering`). Campos que o vanilla tem mas o cristalino
   ainda não materializou (`introspector`, `routines`, `traced`)
   ficam como comentários `// stub futuro`, **não** como stubs
   reais neste passo.
3. **Ordem dos campos**: coesa por domínio (ADR-0037). Agrupamento
   sugerido: handle externo (`world`), fluxo de eval (`route`,
   `styles`, `show_rules`, `active_guards`, `current_file`,
   `figure_numbering`), efeitos laterais (`sink`).
4. **`world` move de `EvalContext` para `Engine`**. `EvalContext`
   passa de 4 campos Regra 4 para 3 (`loop_iterations`,
   `max_loop_iterations`, `next_rule_id`).
5. **Forma da migração** (big-bang vs incremental-por-campo vs
   incremental-por-função): decidir em 109.A com base em inventário
   de lifetimes.

---

## O que este passo **não** faz

- Não materializa `Introspector`, `Routines`, `Traced`. Esses ficam
  como comentários no Engine.
- Não muda assinatura pública de `eval()` (`TrackedMut<Sink>`
  continua a entrar por valor — ADR-0043 mantida).
- Não absorve campos do `EvalContext` além de `world`.
- Não altera lógica de eval. Apenas refactoring de assinaturas.
- Não mexe em comentários `// DEBT-XX` existentes.

---

## Sub-passos

### 109.A — Inventário agressivo

O Passo 108 deu visão macro. 109.A precisa de visão fina: lifetimes
exactos, sítios de construção, conflitos potenciais.

**Parte 1 — Lifetimes exactos de cada campo**:

Grep nas assinaturas actuais de `eval_*` para cada dos 10
parâmetros. Registar:

| Parâmetro | Tipo actual exacto | Lifetime |
|-----------|---------------------|----------|
| `world` | `Tracked<'_, dyn TrackedWorld + '_>` | 1 ou 2 lifetimes |
| `route` | `Tracked<'r, Route<'r>>` | `'r` |
| `styles` | `&mut StyleChain` | elidido |
| `show_rules` | `&mut Arc<[ShowRule]>` | elidido |
| `active_guards` | `&mut Vec<RuleId>` | elidido |
| `current_file` | `FileId` | nenhum |
| `figure_numbering` | `&mut Option<String>` | elidido |
| `sink` | `&mut TrackedMut<'_, Sink>` | 1 |

(Tabela a preencher com os valores reais exactos — a amostra acima
é baseada no que o Passo 107 registou.)

**Parte 2 — Conflitos**:

Para cada par de campos, verificar se os lifetimes são compatíveis.
Dois cenários possíveis:

- **Todos compatíveis**: Engine tem 1 ou 2 lifetimes (`'a` para
  tudo, ou `'a` + `'r`). Big-bang viável.
- **Alguns incompatíveis**: ex: `TrackedMut<Sink>` com lifetime
  próprio que colide com `Tracked<Route>`. Engine absorve os
  compatíveis; os outros ficam como parâmetros avulsos.

**Parte 3 — Sítios de construção**:

Quem constrói os parâmetros que vão para dentro do Engine? Grep em
`eval()` público. O Engine vai ser construído nesse mesmo sítio, a
partir dos mesmos valores.

**Parte 4 — Forma da migração**:

Avaliar três alternativas com base nas partes 1-3:

- **Big-bang**: se todos os lifetimes compatíveis e < 30 call sites
  (ex: 10 funções × média de 3 chamadas), refactor de uma vez.
- **Incremental-por-campo**: se lifetimes delicados, fazer 1 campo
  de cada vez. 8 sub-passos, cada um verificável. Mais lento mas
  reduz risco.
- **Incremental-por-função**: Engine criado com todos os campos de
  uma vez, mas funções migram uma a uma. Requer que funções antigas
  e novas coexistam durante transição. Desfeito no fim.

**Gate**: se a Parte 2 revelar mais de 2 lifetimes distintos no
Engine, ou conflitos que obrigam a subconjuntos pequenos (< 4
campos), o passo não traz valor suficiente. Parar e reportar.
Alternativa: materializar só 2-3 campos num Engine mínimo, adiar
o resto.

**Escrever em**
`00_nucleo/diagnosticos/inventario-engine-passo-109.md`:

```
Lifetimes:
  [tabela completa]

Compatibilidade:
  Grupo 1 (compatíveis entre si): [...]
  Grupo 2 (incompatíveis): [...]
  Decisão: Engine contém Grupo 1; Grupo 2 continua avulso.

Sítios de construção:
  eval() público em <ficheiro>:<linha>

Forma decidida:
  big-bang | incremental-por-campo | incremental-por-função
  Razão: [...]
```

### 109.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-engine-agregador.md` com
`PROPOSTO`.

Conteúdo:

- **Contexto**: ADR-0036 extraiu campos do `EvalContext` por 5
  passos para diminuir acoplamento. Consequência: 10 parâmetros
  + `ctx` nas assinaturas `eval_*`. Passo 107 registou o limite
  visual. Passo 108 recomendou consolidação (Candidato 5).
- **Decisão**: `Engine<'a, ...>` em L1 agrega os N campos
  identificados como compatíveis em 109.A. Funções `eval_*`
  passam a receber `engine: &mut Engine<'_>` em vez dos N
  parâmetros.
- **Relação com ADR-0036**: este passo é a **inversão
  controlada** da ADR-0036. Não invalida extracções anteriores
  — reconhece que, depois de extraídas, a volta a agregar num
  tipo explícito é ganho de legibilidade.
- **Paridade com vanilla**: nomes dos campos batem com os do
  `typst-library::engine::Engine`. Ordem segue ADR-0037
  (coesão por domínio), não a ordem vanilla. Tipos seguem o
  cristalino (ex: `&mut StyleChain` em vez do tipo vanilla).
- **Campos omitidos**: `introspector`, `routines`, `traced` —
  vanilla tem, cristalino ainda não materializou. Comentários
  `// stub futuro` no Engine documentam a divergência; entram
  em passos dedicados quando necessários.
- **`world` move de `EvalContext` para `Engine`**: razão
  arquitectural documentada. `EvalContext` fica com 3 campos
  Regra 4 (contadores/limites/alocadores monotónicos).
- **Alternativas rejeitadas**:
  - **Manter 10 parâmetros**: ruído cresce; próxima
    propagação adiciona 11º parâmetro.
  - **Materializar Introspector/Routines/Traced antes do
    Engine**: passo 108 mostrou que cada um é um passo
    arquitectural por si; consolidar o que já existe é valor
    independente.
- **Assinatura pública de `eval()`**: mantém-se. `TrackedMut<Sink>`
  continua a entrar por valor como ADR-0043 estabelece. Dentro
  de `eval()`, construção do Engine a partir dos parâmetros.

Promover a `EM VIGOR` em 109.E.

### 109.C — Implementação

Sequência depende da forma decidida em 109.A. Abaixo está o plano
para **big-bang**; para incremental, repetir por campo.

**109.C.1 — Definir `Engine<'a>`**:

`01_core/src/entities/engine.rs`:

```rust
/// Agregador de estado de eval em L1.
///
/// Consolida campos antes passados como parâmetros individuais
/// pelas funções `eval_*` (ver ADR-0036 e ADR-00NN).
pub struct Engine<'a /* demais lifetimes conforme 109.A */> {
    // Handle externo
    pub world: Tracked<'a, dyn TrackedWorld + 'a>,
    
    // Fluxo de eval
    pub route: Tracked<'r, Route<'r>>,  // se 'r necessário
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,
    pub figure_numbering: &'a mut Option<String>,
    
    // Efeitos laterais
    pub sink: &'a mut TrackedMut<'a, Sink>,
    
    // Stubs futuros (não materializar neste passo):
    // pub introspector: Introspector,     // Passo dedicado
    // pub routines: &'a Routines,          // Passo dedicado
    // pub traced: Tracked<'a, Traced>,     // Passo dedicado
}
```

Exactos lifetimes, tipos e ordem saem de 109.A.

**109.C.2 — Remover `world` de `EvalContext`**:

```rust
// Antes
pub struct EvalContext<'a> {
    world: Tracked<'a, dyn TrackedWorld + 'a>,
    loop_iterations: usize,
    max_loop_iterations: usize,
    next_rule_id: u32,
}

// Depois
pub struct EvalContext {
    loop_iterations: usize,
    max_loop_iterations: usize,
    next_rule_id: u32,
}
```

Se o lifetime `'a` era só por causa do `world`, desaparece da
struct. Se outros sítios usavam `ctx.world`, passam a usar
`engine.world`.

**109.C.3 — Construir `Engine` dentro de `eval()`**:

```rust
// Em eval() público
pub fn eval(
    _routines: &Routines,
    world: Tracked<'_, dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    // ... inicialização de styles, show_rules, etc.
    
    let mut engine = Engine {
        world,
        route,
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file,
        figure_numbering: &mut figure_numbering,
        sink: &mut sink,
    };
    
    eval_markup(&mut ctx, &mut engine, root)?;
    
    // ... drenagem de styles no Module, etc.
}
```

**109.C.4 — Migrar assinaturas `eval_*`**:

Para cada função que hoje recebe N parâmetros individuais,
substituir:

```rust
// Antes
pub(super) fn eval_set_rule<'r>(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value>

// Depois
pub(super) fn eval_set_rule(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,  // ← N params consolidados
) -> SourceResult<Value>
```

Dentro da função, `route` vira `engine.route`, `styles` vira
`engine.styles` (ou `&mut engine.styles`), etc.

**109.C.5 — Migrar call sites**:

Cada call site passa `engine` em vez dos N parâmetros:

```rust
// Antes
eval_set_rule(set, scopes, ctx, route, styles, show_rules,
              active_guards, current_file, figure_numbering, sink)?;

// Depois
eval_set_rule(set, scopes, ctx, engine)?;
```

**Regra crítica de borrow-checker**: se uma função chamada precisa
de mutar um campo do engine **e** de outros parâmetros, o borrow
checker pode recusar (`engine.styles` borrowed junto com
`engine.show_rules`). Se acontecer: desestruturar o engine
localmente no call site, ou usar disjoint borrows. Documentar em
109.D se o pattern aparecer.

**109.C.6 — Compilação incremental**:

Depois de cada ficheiro migrado, correr `cargo check -p typst-core`.
Se a forma for incremental-por-campo (109.A), cada campo é um
check completo antes do próximo.

### 109.D — Testes

**Não adicionar testes novos de funcionalidade** neste passo. É
refactoring puro — os 803 + 184 testes existentes são o critério.

Se algum teste regride:
- Compilação falha → erro de refactoring, corrigir.
- Teste runa e falha → erro semântico, investigar. Provavelmente
  uma referência a campo esquecida (ex: `ctx.world` não trocado
  para `engine.world` num sítio).

Se a forma foi incremental-por-campo, `cargo test` passa depois de
cada sub-passo. Se big-bang, passa no fim.

Adicionar **1 teste estrutural** opcional: em `entities/engine.rs`,
`#[cfg(test)]` verifica que `Engine` pode ser construído com os
campos mínimos (smoke test). Sem asserções de comportamento.

### 109.E — Encerramento

1. Grep: `eval_*` já não têm parâmetros individuais para
   `world`/`route`/`styles`/`show_rules`/etc. As assinaturas
   têm `ctx: &mut EvalContext, engine: &mut Engine<'_>`.
2. Grep: `ctx.world` retorna zero matches (world só no Engine).
3. `cargo test --workspace`: **igual à linha de base**
   (803 L1 + 184 L3 + 6 ignorados). Nenhum teste novo ou
   removido.
4. `crystalline-lint` zero violations.
5. ADR promovida a `EM VIGOR`.
6. **DEBT-45** potencialmente actualizável: o relatório de
   continuidade dizia que 2/4 `check_*_depth` aguardavam
   `Engine<'a>`. Verificar se algum passa a ser trivialmente
   integrável agora. Se sim, **não integrar neste passo** —
   registar para passo seguinte.
7. Relatório `typst-passo-109-relatorio.md`:
   - Forma de migração escolhida (big-bang / incremental) e
     razão.
   - Subconjunto real dos campos que entraram (pode ser < 8).
   - Conflitos de borrow-checker encontrados (se houver) e
     como resolvidos.
   - Número de call sites migrados.
   - Contagem de parâmetros `eval_*` antes/depois (era 10 +
     ctx; passa a 1 + ctx se big-bang, ou intermédio se
     subconjunto).
   - Campos omitidos e razão (ex: "traced omitido — vanilla
     tem, cristalino não materializou; entra em passo
     dedicado").
   - Actualização DEBT-1 se aplicável.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 109.A escrito.
2. ADR-00NN criada e promovida.
3. `Engine<'a>` definido em `entities/engine.rs`.
4. `world` movido de `EvalContext` para `Engine`. `EvalContext`
   passa a 3 campos Regra 4.
5. Funções `eval_*` recebem `engine: &mut Engine<'_>` em vez
   dos parâmetros individuais consolidados.
6. Call sites actualizados.
7. `cargo test --workspace` com contagem **exactamente igual** à
   linha de base.
8. `crystalline-lint` zero violations.
9. Relatório 109.E escrito.

---

## O que pode sair errado

- **Lifetimes explodem**. Se o Engine precisa de 3+ lifetimes
  distintos, a assinatura vira `Engine<'a, 'b, 'r>` — legível
  mas incómoda. Gate em 109.A.2. Alternativa: limitar ao
  subconjunto que cabe em `'a` + `'r` (2 lifetimes).
- **Borrow checker recusa call interno**. Ex: função que recebe
  `engine: &mut Engine` e também chama função que lê
  `engine.sink` enquanto `engine.styles` está borrowed. Solução
  padrão: **disjoint borrows** — desestruturar o engine para
  obter `&mut` separados dos campos. Pode exigir helpers.
  Registar em 109.D se aparecer.
- **EvalContext sem lifetime**. Se `EvalContext` perde o
  lifetime `'a` (por `world` sair), todos os call sites que o
  constroem simplificam. Bom sinal, mas verificar se nenhuma
  assinatura passa a ser ambígua.
- **Testes regressam por esquecimento**. Um `ctx.world` não
  trocado para `engine.world` compila (se `EvalContext` ainda
  tivesse `world` por erro) mas semanticamente liga o campo
  errado. Grep explícito em 109.E.
- **ADR-0036 aparenta contradição**. Este passo consolida
  depois de 5 extracções. A ADR nova tem de explicar que não
  é contradição — é reconhecimento de que extrair primeiro e
  agregar depois é padrão válido.
- **Pressão para materializar Introspector de uma vez**. Se o
  executante sente que "já que estou aqui, faço também o
  Introspector", resistir. É Passo 110 ou mais tarde.
- **Forma big-bang fica maior do que parecia**. Se 109.A sugere
  big-bang mas no meio da implementação o diff é gigante,
  reverter para incremental-por-campo a meio é aceitável —
  registar a razão.

---

## Notas operacionais

- Este passo é a **6ª aplicação da ADR-0036**, em forma inversa
  (consolidação). Documentar essa inversão na ADR-00NN.
- `Engine<'a>` não é `#[comemo::track]`. Seria complexo e não
  traz valor hoje — o tracking já é feito pelos `Tracked<World>`,
  `TrackedMut<Sink>` e `Tracked<Route>` individuais dentro do
  Engine. Registar.
- O subtipo exacto dos lifetimes sai de 109.A. Se der para
  uniformizar em `'a`, fazer. Se forçar `'a` + `'r`, aceitar.
  Evitar `'a` + `'b` + `'c` — se apontar para isso, gate
  109.A.2 dispara.
- Campos públicos `pub` em vez de getters: simplicidade. L1 não
  tem razão para encapsular campos do Engine — ele é agregador
  transparente. Getters podem vir depois se aparecer razão.
- Construção do Engine fica toda dentro de `eval()`. Outros
  sítios (testes, helpers) constroem os parâmetros primeiro e
  depois montam o Engine — padrão equivalente.
- Se `EvalContext` fica sem lifetime, o nome pode ser revisto
  (sem `<'_>`). Não tocar no nome neste passo — mudança
  cosmética com raio amplo, fora do escopo.
