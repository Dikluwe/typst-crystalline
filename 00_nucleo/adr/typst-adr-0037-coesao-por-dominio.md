# ⚖️ ADR-0037: Coesão por domínio — ficheiros limitados a uma responsabilidade clara

**Status**: `EM VIGOR`
**Data**: 2026-04-22 (`PROPOSTO`) / 2026-04-22 (`EM VIGOR` após validação nos Passos 96.1–96.2)

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

**Coesão não implica isolamento** (nota adicionada no Passo 96.3,
Ajuste D). Submódulos coesos por domínio podem — e frequentemente
devem — referenciar-se mutuamente via `super::X::func()` quando a
semântica o justifica. Exemplos observados no Passo 96.2:

- `closures::eval_func_call` consulta `bindings::eval_counter_method`
  para tratar chamadas a métodos de contador (ex: `counter(x).step()`).
- `rules::eval_set_rule` chama `super::eval_expr` para avaliar os
  argumentos do `#set`.
- `markup::eval_strong`/`eval_emph`/`eval_heading` chamam
  `rules::intercept_content` para aplicar show rules ao Content
  produzido.

A divisão por domínio facilita navegação e manutenção; não cria
silos fechados. Cruzamentos entre submódulos são expectáveis e
saudáveis quando reflectem dependências semânticas reais.

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

**Paths entre submódulos** (clarificação adicionada no Passo 96.3,
Ajuste B). Submódulos acedem a funções de outros submódulos via
path relativo `super::X::func()` (sobem ao `mod.rs` do módulo pai
e descem ao submódulo destino). Paths absolutos
(`crate::rules::eval::X::func`) reservam-se para casos onde o
caminho relativo é confuso — por exemplo, `tests.rs` que acede a
funções de vários submódulos distintos.

Nota técnica: o linter V14 (`ForbiddenImport`) aceita `super::`
como path relativo padrão. Evitar `self::` — o linter V14 em
vigor no projecto trata-o como identificador externo e reporta
como violação.

**Visibilidade preferida** (clarificação adicionada no Passo 96.6).
Ao extrair código para submódulos, a preferência é a seguinte ordem:

1. **Manter privado**. Se nenhum submódulo precisa de acesso
   directo, manter sem modificador de visibilidade. Acesso
   indirecto via métodos públicos já existentes.

2. **Métodos `pub(super)` em vez de campos `pub(super)`**. Se
   submódulos precisam de operar sobre uma struct, preferir
   expor **comportamento** (métodos) em vez de **estado**
   (campos). Métodos `pub(super) fn advance(&mut self)` são
   preferíveis a campo `pub(super) pos: usize`. Isto preserva
   invariantes da struct.

3. **`pub(in path)` para escopo explícito**. Quando o escopo
   exacto é conhecido, declará-lo directamente:

   ```rust
   pub(in crate::rules::parse) fn helper(...) { ... }
   ```

   É equivalente a `pub(super)` em certos casos mas auto-documenta
   a intenção.

4. **`pub(super)` em campos apenas quando necessário**. Se
   métodos não resolvem (ex: campo que múltiplos submódulos
   precisam de ler **e** escrever, sem semântica que justifique
   método específico), usar `pub(super)`. Registar em comentário
   no código a razão.

5. **`pub(crate)` apenas quando consumido fora do módulo
   actual**. Se o item é consumido por outro módulo da crate
   (ex: `eval/closures.rs` precisa de função de `stdlib::calc`),
   `pub(crate)` é apropriado. Se só submódulos do mesmo módulo
   consomem, `pub(super)` ou `pub(in path)` são mais estritos.

6. **`pub` (público) apenas para API verdadeiramente exposta ao
   exterior**. Funções consumidas por outras crates do workspace
   ou pelo wiring.

**Anti-padrão**: `pub(super)` aplicado a todos os campos e
métodos de uma struct por conveniência (ex: bulk replace
durante reestruturação). Isto destrói invariantes e aumenta
superfície de refactor. Se uma reestruturação encontra-se
nesta situação, abrir DEBT dedicado a auditar e restringir.

### Regra 4 — Dispatchers pequenos

Funções que fazem `match` exaustivo sobre um enum grande
(dispatchers) devem **delegar** armos com lógica substancial a
funções especializadas em cada submódulo. Armos triviais
(construtores simples, literais, valores constantes) permanecem
inline.

**Revisão Passo 96.3 (Ajuste A)** — critério operacional,
validado empiricamente no Passo 96.2:

- **1–3 linhas**: inline.
- **4–7 linhas**: decisão caso a caso com base em coesão
  semântica (a lógica pertence claramente a um submódulo? →
  extrai; é glue code específico do dispatcher? → inline).
- **> 7 linhas**: extrai para submódulo.

Exemplos aceitáveis inline:

```rust
Expr::Int(n)   => Ok(Value::Int(n.get())),
Expr::Bool(b)  => Ok(Value::Bool(b.get())),
Expr::Ident(id) => resolve_ident(id, scopes),
_              => Ok(Value::None),  // fallback trivial
```

Exemplos que devem delegar:

```rust
// eval/mod.rs — dispatcher compacto
Expr::Strong(s)   => markup::eval_strong(s, scopes, ctx, route, styles, /* ... */),
Expr::SetRule(r)  => rules::eval_set_rule(r, scopes, ctx, route, styles, /* ... */),
Expr::FuncCall(c) => closures::eval_func_call(c, scopes, ctx, route, styles, /* ... */),
```

Armos que introduzem scoping cross-cutting (`CodeBlock`,
`ContentBlock` que criam `local_styles`/`local_show_rules`) podem
permanecer inline no dispatcher mesmo se excederem 3 linhas —
não pertencem a cluster específico. Esta é uma interpretação da
regra "decisão caso a caso" (faixa 4-7 linhas) e foi observada
e aceite no Passo 96.2.

### Regra 5 — Testes seguem o domínio

Testes unitários ficam preferencialmente no mesmo ficheiro que a
lógica que testam (`#[cfg(test)] mod tests` no fim do ficheiro)
ou num submódulo de testes paralelo (`eval/math/tests.rs`).

**Revisão Passo 96.3 (Ajuste C)** — excepção aceite para testes
E2E e testes transversais que exercitam múltiplos domínios: estes
podem viver em ficheiro `tests.rs` dedicado no mesmo módulo,
mesmo que exceda o limite da Regra 2. Esta é **excepção natural
reconhecida pela própria Regra 5** — não requer marca Regra 6.

Observado no Passo 96.1: o `eval/tests.rs` tem ~2100 linhas e
contém tests que cruzam markup+math+control_flow+rules+closures
via programas Typst completos. Decompor por cluster produziria
duplicação ou perda de cobertura.

O princípio operativo: testes coesos com o domínio testado são
preferidos; testes cross-cutting por natureza não se forçam a
decomposição artificial.

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

Ordem final dos sub-passos que pagam DEBT-46 (reformulada no
Passo 96.3 após introdução do 96.2):

1. **`eval.rs`** (Passo 96.1) — decomposição central em
   submódulos por domínio. **Concluído.**
2. **Delegação dos armos** (Passo 96.2) — dispatcher compacto;
   completa a aplicação da Regra 4. **Concluído.**
3. **Promoção da ADR com ajustes** (Passo 96.3) — `PROPOSTO` →
   `EM VIGOR`, 4 ajustes (A/B/C/D) validados empiricamente.
   **Concluído neste passo.**
4. **`parse.rs`** (Passo 96.4) — divisão por tipo de nó
   (markup, code, math, rules).
5. **`stdlib.rs`** (Passo 96.5) — um ficheiro por módulo da
   stdlib.
6. **`layout/mod.rs`** (Passo 96.6) — `Layouter<M, S>` com
   muitos métodos; divisão mais técnica.
7. **`math/layout.rs`** (Passo 96.7) — divisão final.
8. **`lexer/mod.rs`** (Passo 96.8) — se ainda for problemático
   após reanálise (algumas excepções da Regra 6 podem
   aplicar-se).
9. **Encerramento do DEBT-46** (Passo 96.9, ou fecho implícito
   em 96.8) — verificação final, contagem de ficheiros > 800
   linhas sem excepção Regra 6 documentada, fecha DEBT.

---

## Nota histórica

Esta ADR começou como `PROPOSTO` (2026-04-22) e foi validada
empiricamente nos Passos 96.1 (reestruturação do `eval.rs`) e
96.2 (completar delegação dos armos). Promovida a `EM VIGOR` no
Passo 96.3 com 4 ajustes:

- **Ajuste A** — Regra 4 revista com critério operacional (faixas
  1-3 / 4-7 / >7 linhas) e aceitação explícita de armos inline
  com scoping cross-cutting.
- **Ajuste B** — Regra 3 clarificada sobre paths entre
  submódulos (`super::` preferido sobre `self::` devido a
  interacção com V14 do linter).
- **Ajuste C** — Regra 5 clarificada com excepção natural para
  testes E2E cross-cutting (não requer marca Regra 6).
- **Ajuste D** — Regra 1 complementada com nota "coesão não
  implica isolamento": cruzamentos entre submódulos via
  `super::X::func()` são expectáveis e saudáveis.

Esta é a primeira ADR do projecto a usar `PROPOSTO` com plano
explícito de promoção pós-validação — o padrão pode ser
replicado em ADRs futuras que queiram ser testadas antes de
ficarem em vigor.

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
