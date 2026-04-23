# ⚖️ ADR-0037: Coesão por domínio — ficheiros limitados a uma responsabilidade clara

**Status**: `PROPOSTO`
**Data**: 2026-04-22

---

## Contexto

A série de passos 92–95 removeu progressivamente estado partilhado
do `EvalContext` (ADR-0036), mas o ficheiro que aloja a lógica
(`01_core/src/rules/eval.rs`) continuou a crescer e tem hoje 3780
linhas. Análise realizada antes do Passo 96 revelou seis ficheiros
em `01_core/src/` acima de 1000 linhas:

| Linhas | Ficheiro |
|--------|----------|
| 3780 | `01_core/src/rules/eval.rs` |
| 2848 | `01_core/src/rules/layout/mod.rs` |
| 2255 | `01_core/src/rules/parse.rs` |
| 1806 | `01_core/src/rules/math/layout.rs` |
| 1711 | `01_core/src/rules/stdlib.rs` |
| 1250 | `01_core/src/rules/lexer/mod.rs` |

No conjunto, são 13.650 linhas em seis ficheiros. O `eval.rs`
sozinho tem 368 ocorrências de padrões `match` sobre
`Expr::`, `SyntaxKind::` e `Value::` — é na prática um dispatcher
central para toda a lógica de avaliação.

Ficheiros desta dimensão apresentam três problemas observáveis:

1. **Coesão semântica baixa**: o `eval.rs` mistura avaliação de
   markup, matemática, closures, imports, regras show/set e
   controlo de fluxo. Cada cluster é domínio distinto da
   linguagem Typst.
2. **Fricção de navegação**: encontrar "onde `Expr::MathAttach`
   é tratado" exige `grep` em vez de contexto do nome do
   ficheiro.
3. **Risco de regressão por proximidade**: alteração num armo
   pode afectar inadvertidamente outro distante no mesmo `match`,
   sem que a estrutura do módulo o denuncie.

A ADR-0036 (atomização progressiva) reduz acoplamento **dentro
das funções** ao tornar dependências explícitas. Esta ADR
complementa-a reduzindo acoplamento **entre funções** via
agrupamento por domínio em ficheiros separados.

---

## Decisão

**Ficheiros em L1 devem ter uma única responsabilidade de domínio
coerente e linha-alvo abaixo de 800 linhas.** Ficheiros acima
desse limite são candidatos naturais a decomposição por domínio.

Regras operacionais:

### Regra 1 — Coesão por domínio

Um ficheiro agrupa código que trata do mesmo domínio conceptual
da linguagem Typst ou do compilador:

- **Domínio conceptual**: markup, matemática, layout, parser,
  lexer, stdlib, introspecção.
- **Domínio técnico**: gestão de estado do eval, estruturas de
  dados fundamentais, infraestrutura de testes.

Um ficheiro `eval/math.rs` satisfaz a regra — contém tudo o que
envolve avaliação de expressões matemáticas. Um ficheiro
`eval/parte_1.rs` não satisfaz — é divisão por tamanho, não por
domínio.

### Regra 2 — Limite orientativo de 800 linhas

Ficheiros com mais de 800 linhas são **candidatos a decomposição**.
O limite é orientativo, não absoluto. Justificativas para
exceder:

- Tipo fundamental com muitos métodos coerentes (ex: `Content`
  enum com muitas variantes e impls).
- Tabela de dados extensa (ex: mapas de símbolos Unicode).
- Código gerado ou padronizado mecanicamente (ex: lexer
  gerado).

Exceder o limite sem justificativa registada em comentário no
topo do ficheiro é sinal de dívida acumulada.

### Regra 3 — Hierarquia de submódulos

Quando um ficheiro é decomposto, segue a convenção:

```
rules/eval.rs (antes, monolítico)
    ↓
rules/eval/mod.rs         (ponto de entrada, re-exports, EvalContext)
rules/eval/markup.rs      (avaliação de markup)
rules/eval/math.rs        (avaliação de matemática)
rules/eval/modules.rs     (imports, includes, Route)
rules/eval/rules.rs       (show e set rules)
rules/eval/closures.rs    (closures e chamadas de função)
rules/eval/control_flow.rs (if/while/for/let/return)
rules/eval/bindings.rs    (scopes, identificadores)
```

O `mod.rs` é o único a ter API pública para o resto do projecto.
Submódulos são `pub(super)` ou `pub(crate)` conforme a visibilidade
necessária.

### Regra 4 — Dispatchers pequenos

Funções que fazem `match` exaustivo sobre um enum grande
(dispatchers) devem **delegar** imediatamente a funções
especializadas em cada submódulo:

```rust
// eval/mod.rs
pub(crate) fn eval_expr(
    ctx: &mut EvalContext,
    /* ... */
    expr: &Expr,
) -> SourceResult<Value> {
    match expr {
        // Delegação imediata; cada armo é uma linha:
        Expr::Math(m) => eval_math::eval_math(ctx, /* ... */, m),
        Expr::Strong(s) | Expr::Emph(s) | Expr::Heading(s) =>
            eval_markup::eval_markup_node(ctx, /* ... */, s),
        Expr::Show(r) | Expr::Set(r) =>
            eval_rules::eval_rule(ctx, /* ... */, r),
        // ... etc.
    }
}
```

O dispatcher tem uma linha por armo. A lógica real vive no
submódulo correspondente.

### Regra 5 — Testes seguem o domínio

Testes unitários ficam no mesmo ficheiro que a lógica que testam
(`#[cfg(test)] mod tests` no fim do ficheiro) ou num submódulo
de testes paralelo (`eval/math/tests.rs`). Não em ficheiro
monolítico de testes que cruza domínios.

### Regra 6 — Excepções permitidas

Ficheiros fundamentais com responsabilidade única mas código
intrinsecamente extenso podem legitimamente exceder 800 linhas:

- Enums com muitas variantes e implementações derivadas (ex:
  `Content`, `Value`, `Expr`).
- Tabelas de dados (ex: símbolos matemáticos Unicode).
- Código gerado por macros ou tooling.

Estas excepções são **registadas no topo do ficheiro** com
comentário que identifica a categoria:

```rust
//! `Content` enum com 25+ variantes. Excepção à ADR-0037
//! Regra 6: enum fundamental cujas variantes têm impl coeso
//! mas numeroso. Divisão artificial reduziria clareza.
```

### Regra 7 — Reestruturação preserva comportamento

Qualquer passo de decomposição deste tipo:

- **Não altera semântica observável** (ADR-0033 preservada).
- **Não altera contagem de testes** (excepto renomeação de
  testes por convenção de submódulo).
- **Não introduz nova dependência externa**.
- **Mantém zero violations do `crystalline-lint`**.

O movimento de código de um ficheiro para outro é refactor puro
ao nível de organização. A história Git preserva via `git mv` ou
detecção automática de renomeação.

---

## Alternativas Consideradas

| Alternativa | Razão rejeitada |
|-------------|-----------------|
| Deixar ficheiros grandes como estão | Acumulação contínua; problema já sistémico (6 ficheiros > 1000 linhas). |
| Divisão mecânica por número de linhas | Produz ficheiros sem identidade, com dependências circulares. |
| Limite rígido (fail em >800 linhas) | Ignora casos legítimos de Regra 6; enforcement por linter é prematuro sem experiência. |
| Reorganização por camada arquitectural (L1/L2/...) em vez de domínio | Camadas já estão decididas (L0-L4); este ADR orienta sub-divisão dentro de cada camada. |

---

## Consequências

### Positivas

- Navegação melhorada: nome do ficheiro dá contexto do que está
  lá dentro.
- Redução de acoplamento: mudar código de matemática não obriga
  a abrir ficheiro com show rules.
- Testes mais focados: testes de cada domínio vivem junto do
  código testado.
- Onboarding de novas sessões do Claude Code: lê-se o ficheiro
  relevante sem ter de processar 3000+ linhas.
- Facilita futuras materializações: ficheiros menores são alvos
  mais tratáveis.

### Negativas

- Esforço inicial de reestruturação dos 6 ficheiros grandes
  (DEBT-46).
- Possível proliferação de ficheiros pequenos se a Regra 1 for
  aplicada excessivamente (mitigada pela Regra 2, que é limite
  orientativo, não mínimo).
- Imports mais longos entre submódulos (`use crate::rules::eval::math::eval_math`
  em vez de função local).

### Neutras

- Git detecta movimentação de código como rename; a história
  preserva-se.
- Tempo de compilação incremental pode melhorar (Rust compila
  ficheiro a ficheiro).
- A Regra 6 reconhece que alguns ficheiros legitimamente excedem
  o limite; não é fracasso, é categorização.

---

## Relação com ADR-0036

A ADR-0036 aplica-se **dentro de funções** (dependências
declaradas em assinaturas). A ADR-0037 aplica-se **entre
ficheiros** (coesão por domínio). São complementares:

- Um ficheiro pequeno mas com funções que escondem dependências
  via `&mut self` massivo viola ADR-0036 mas não ADR-0037.
- Um ficheiro grande mas com funções atomizadas viola ADR-0037
  mas não ADR-0036.
- O ideal é satisfazer ambas: ficheiros por domínio + funções
  com assinaturas explícitas.

---

## Plano de aplicação

Ordem sugerida para pagar DEBT-46 (sub-passos do Passo 96),
organizada por complexidade crescente:

1. **`eval.rs`** (Passo 96.1) — decomposição central, valida
   a ADR.
2. **Revisão da ADR** (Passo 96.2) — promover a `EM VIGOR` ou
   ajustar com base em fricções encontradas no Passo 96.1.
3. **`parse.rs`** (Passo 96.3) — divisão mais mecânica, clusters
   claros (markup, code, math, rules).
4. **`stdlib.rs`** (Passo 96.4) — um ficheiro por módulo da
   stdlib.
5. **`layout/mod.rs`** (Passo 96.5) — `Layouter<M, S>` com muitos
   métodos; divisão mais técnica.
6. **`math/layout.rs`** (Passo 96.6) — divisão final.
7. **`lexer/mod.rs`** (Passo 96.7) — se ainda for problemático
   após reanálise (algumas excepções da Regra 6 podem aplicar-se).
8. **Encerramento do DEBT-46** (Passo 96.8) — verificação final,
   contagem de ficheiros > 800 linhas, fecha DEBT se objectivos
   atingidos.

---

## Status `PROPOSTO` vs `EM VIGOR`

Esta ADR começa como `PROPOSTO` porque as regras são
**conjecturais**. Serão validadas pelo Passo 96.1 (primeira
aplicação). Se a reestruturação do `eval.rs` ocorrer sem ajustes
significativos, promove-se a `EM VIGOR` no Passo 96.2. Se
houver fricções — ex: uma regra revelar-se impraticável — o
Passo 96.2 ajusta o texto antes de promover.

Esta é a primeira ADR do projecto a usar `PROPOSTO` com plano
explícito de promoção pós-validação.

---

## Referências

- ADR-0033 (paridade funcional com vanilla) — comportamento
  observável não muda com reestruturação.
- ADR-0036 (atomização progressiva) — complementar, aplica-se
  dentro de funções; esta ADR aplica-se entre ficheiros.
- DEBT-46 — inventário concreto dos 6 ficheiros candidatos.
- Passos 92–95 — série que removeu estado partilhado do
  `EvalContext` mas não reduziu o tamanho do `eval.rs`.
- Passo 96.1 — primeira aplicação concreta (reestruturação do
  `eval.rs`).
