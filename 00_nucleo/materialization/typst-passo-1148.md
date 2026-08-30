# P1148 — superfície estática completa de `counter`

**Data:** 2026-08-24  
**Estado:** `EM EXECUÇÃO — GATE APROVADO; VERTICAL DE CONTRATO E GLUE GREEN`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Sentinela de origem:** `counter.get`  
**Dependência:** P1147 materializado na working tree, ainda não commitado

## 1. Objetivo

Auditar e materializar a superfície pública estática completa do valor-tipo
`counter`, reutilizando as implementações de instância existentes no owner
`compiler/stdlib/counter.rs`.

`counter.get` é apenas a sentinela. A fonte ratificada define no mesmo scope:

```text
get, display, at, final, step, update
```

O passo não pode adicionar somente `counter.get` e declarar a família
fechada. Deve medir os seis fields, as formas estática e de instância, o
contexto exigido e a equivalência semântica entre ambas.

## 2. Proveniência inicial e preservação da working tree

- Hora: `2026-08-24T20:51:59-03:00`.
- HEAD: `2df42a6f6dbdf15b5104d5e7a6df82755f07c8c4`.
- Baseline vanilla: `a51e02804`.
- Working tree P1147 não commitada: 12 ficheiros rastreados alterados, com
  178 inserções e 28 remoções, mais três ficheiros novos — L0 de int, owner de
  int e `typst-passo-1147.md`.

P1148 deve preservar esse lote integralmente. Antes de executar, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

Se o estado tiver mudado, rebaselinear. Não usar os números acima para fechar
decisões posteriores sem registrar o novo estado.

## 3. Inventário estático já estabelecido

O bloco `#[scope] impl Counter` da fonte ratificada
`a51e02804:crates/typst-library/src/introspection/counter.rs:336-517` contém:

```text
construct, get, display, at, final, step, update
```

O constructor é a chamabilidade de `counter(...)`; os outros seis são fields
funcionais do valor-tipo. Todos recebem `self`, explícita ou implicitamente,
logo são candidatos às formas:

```typst
counter.get(c)
c.get()
```

Essa equivalência é hipótese até ser confirmada nos dois binários vanilla.

No cristalino:

- `counter` já é `Value::Type(Type::Counter)` e é chamável;
- `Value::Counter` já despacha métodos em `call_dispatch.rs`;
- `counter_get` já consulta o contexto/introspector;
- `display`, `at`, `final`, `step` e `update` já possuem implementação de
  instância;
- não foi encontrado braço `Type::Counter` em field access nem função
  `counter_type_field`.

Portanto, a divergência inicial parece ser exposição/glue, não algoritmo novo.
Isso deve ser confirmado caso a caso.

## 4. Fase A — inventário literal da linguagem

Produzir tabela:

```text
membro | assinatura Typst | contextual | static | instância |
posicional/nomeado | default | retorno | erro | owner existente
```

Extrair da fonte:

- `get(self)`: localização atual e array de inteiros;
- `display(self, numbering: auto, at: auto, both: false)`: defaults,
  numbering string/função, selector/location e contexto;
- `at(self, selector)`: resolução única de selector/location;
- `final(self)`: valor no fim do documento;
- `step(self, level: 1)`: level positivo não-zero e conteúdo produzido;
- `update(self, update)`: inteiro, array e callback;
- constructor e `CounterKey` somente para garantir que o receiver estático é
  exatamente `Value::Counter`, sem reparsear a chave.

Não inferir contrato atual apenas do L0 cristalino: ele ainda documenta
subsets históricos e alguns comportamentos graded. Comparar fonte, sondas e
implementação real com `file:line`.

## 5. Fase B — sondas nos dois binários vanilla

Executar cada sonda em
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst`. Só aceitar
baseline quando ambos coincidirem.

### 5.1 Descoberta e equivalência

Para os seis membros:

- medir `type(counter.member)` e field sem chamada;
- comparar `counter.member(c, ...)` com `c.member(...)`;
- medir receiver ausente, receiver de tipo errado, argumentos excedentes,
  named em positional e named desconhecido;
- confirmar que `counter.from`/field inexistente conserva o erro de tipo.

Usar counters string e de elemento (`counter("x")`, `counter(heading)`), sem
confundir diferenças legítimas de chave.

### 5.2 Métodos contextuais

Para `get`, `display`, `at` e `final`, construir documento determinístico com:

- counter nunca tocado;
- updates/steps antes e depois da localização consultada;
- counter hierárquico;
- label/location existente e inexistente;
- execução dentro e fora de `context`;
- formas estática e de instância no mesmo contexto.

Registrar array, conteúdo/repr, mensagens, hints e spans quando forem o
observável. `get` deve sempre retornar array, inclusive para um componente.

### 5.3 `display`

Medir:

- numbering omitido/`auto`;
- patterns `"1"`, `"1."`, romano, alfabético e múltiplos componentes;
- callback e variação de aridade hierárquica;
- `at:` como auto, label, location e selector;
- `both: false/true`;
- combinações de argumentos e tipos inválidos.

Se o cristalino ainda não suporta parte da assinatura ratificada, classificar
como divergência própria. Não fazer a forma estática delegar a um subset e
chamá-la equivalente.

### 5.4 `step` e `update`

Medir morfologia do conteúdo e efeito após introspecção:

- `step()` e `level: 1/2`, zero, negativo e tipo errado;
- `update` com inteiro, array de inteiros, callback e tipos inválidos;
- retorno inserido versus descartado;
- static/instância produzindo o mesmo efeito de linguagem.

O shape interno de `Content::CounterUpdate` é mecânica; o efeito na sequência
de valores e a exigência de inserir o conteúdo são língua.

## 6. Fase C — auditoria cristalina

Auditar com `file:line`:

- L0 e owner `compiler/stdlib/counter`;
- `Type::Counter` em `Value`, global scope, field access e chamabilidade;
- braço `Value::Counter` em `call_dispatch`;
- `eval_counter_method_value` e avaliação de args/contexto;
- `counter_get`, `counter_display`, `counter_at`, `counter_final`,
  `counter_step`, `counter_update` e nativas históricas homónimas;
- `CounterRegistry`, `Introspector` e localização corrente apenas para
  confirmar que a forma estática reutiliza o mesmo caminho;
- testes que fixem comportamento graded divergente, como label inexistente.

Matriz obrigatória:

```text
caso | vanilla medido | cristalino atual | língua/mecânica |
L0 afetado | contrato público necessário? | decisão
```

Não confundir as nativas globais históricas que recebem strings com os fields
do tipo que recebem `Value::Counter`. Não duplicar resolução de key, label,
contexto ou numbering.

## 7. Nucleação L0 e arquitetura esperada

L0s candidatos a atualização:

- `00_nucleo/prompts/compiler/stdlib/counter.md`;
- `00_nucleo/prompts/compiler/eval/bindings/field_access.md`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`, somente se o caminho de
  instância precisar ajuste;
- `00_nucleo/prompts/entities/value.md`, para fields do valor-tipo;
- L0s de introspecção/numbering apenas se a auditoria encontrar divergência
  algorítmica real.

Arquitetura inicial a confirmar:

- `counter_type_field(field)` no próprio `counter.rs`;
- funções estáticas recebem `self` como primeiro positional;
- funções com engine/contexto usam o ABI apropriado já existente;
- implementação comum recebe args já avaliados e serve ambas as formas;
- match fechado, sem registry reflexivo ou despacho dinâmico;
- nenhuma lógica nova em `field_access.rs` além da delegação do tipo.

## 8. Decisão ADR-0107/0108 e gate ADR-0127

São língua: fields, formas de chamada, contexto exigido, defaults, arrays,
numbering, localização, efeitos dos updates, mensagens e hints. São mecânica:
estrutura do registry, número de passes de introspecção, enum interno e
delegação por function pointer.

Atualizar L0 antes do código e classificar:

- wrappers/glue privados ou `pub(crate)` sobre funções já existentes, sem
  mudar contrato/default/fase: fluxo contínuo, RED→GREEN;
- novo método/campo/assinatura pública Rust, mudança de `Counter`, default,
  comportamento por defeito ou fase eval/layout: **parar no gate ADR-0127**;
- ampliar `display`, `step` ou `update` além do subset vigente pode ser
  correção de paridade interna, mas deve ter L0 primeiro e testes RED; se
  exigir contrato ou fase, parar.

A expectativa de “somente glue” é inferência. Refutação: qualquer membro cuja
forma estática não possa reutilizar o owner sem ampliar contrato público ou
transportar novo estado.

## 9. Plano RED→GREEN

1. RED de descoberta dos seis fields em `Type::Counter`;
2. RED da equivalência static/instância para cada membro;
3. RED contextual de `get`, `at`, `final` e `display`;
4. RED de todos os defaults/named args de `display`;
5. RED de `step(level:)` e `update` inteiro/array/callback;
6. RED de receiver, aridade, tipos, mensagens e hints;
7. implementar a tabela no owner e delegar ao mesmo caminho sem duplicação;
8. ressellar hashes de linhagem;
9. repetir sondas cristalinas equivalentes às do vanilla;
10. executar regressões de counter/state/context/introspecção/layout;
11. executar `cargo check --workspace`, `cargo build --workspace`,
    `cargo fmt --all -- --check`, `git diff --check` e
    `crystalline-lint .` com zero violations.

Confirmar RED antes da implementação. Testes que já passam documentam
regressão, mas não provam o delta.

## 10. Limites e encerramento

Ficam fora, salvo dependência inevitável medida:

- refactor geral do fixpoint/introspector;
- mudança de representação de `Counter`, `CounterKey` ou registry;
- novos tipos de selector ou elementos locatable;
- `content.fields` estático, numbering por função, Symbol/emoji e HTML;
- limpeza, commit ou alteração incidental do lote P1147.

P1148 encerra quando os seis fields tiverem inventário reproduzível, formas
estática/instância e contexto classificados, todas as divergências do owner
afetadas pelo glue tiverem decisão, e a validação final estiver verde. Se
algum subset permanecer graded, registrar scope-out e passo sucessor pelo
nome; não declarar a superfície `counter` completa enquanto houver membro sem
decisão.

## 11. Execução até ao gate (2026-08-24)

### Proveniência rebaselineada

- Hora de início: `2026-08-24T20:54:21-03:00`.
- HEAD: `2df42a6f6dbdf15b5104d5e7a6df82755f07c8c4`.
- Working tree: lote P1147 não commitado preservado, mais este passo e as
  atualizações L0 de P1148.
- Vanilla medido: fonte pinada `a51e02804` e os binários
  `lab/typst-original/target/release/typst` e `/usr/local/bin/typst`.

### Resultado reproduzível da descoberta

A sonda abaixo devolveu o mesmo resultado nos dois binários:

```typst
(type(counter.get), type(counter.display), type(counter.at),
 type(counter.final), type(counter.step), type(counter.update),
 repr(counter.step(counter("x"))),
 repr(counter.update(counter("x"), 5)))
```

```text
(function, function, function, function, function, function,
 counter-update(key: "x"), counter-update(key: "x"))
```

A fonte `counter.rs:336-517` confirma as seis assinaturas. Em particular:

- `step(self, level: NonZeroUsize = 1)` preserva o nível;
- `update(self, CounterUpdate)` aceita estado inteiro/hierárquico ou função;
- o enum vanilla em `counter.rs:567-576` é `Set(CounterState)`,
  `Step(NonZeroUsize)`, `Func(Func)`.

No cristalino, `01_core/src/entities/counter_update.rs` expõe publicamente
apenas `Step` e `Update(usize)`. Portanto, a implementação completa pedida
não cabe no contrato vigente.

### Decisão e paragem obrigatória

Foram nucleadas propostas em:

- `entities/counter_update.md`: `Set(Vec<usize>)`,
  `Step(NonZeroUsize)`, `Func(Func)`;
- `entities/counter_registry.md`: aplicação hierárquica e callback;
- `compiler/introspect/from_tags.md`: callback na fase de introspecção já
  equipada com `Engine + EvalContext`;
- `compiler/stdlib/counter.md`, `field_access.md` e `entities/value.md`:
  superfície estática dos seis fields.

Classificação: **mudança de contrato público**, pois substitui variantes e
payloads de `pub enum CounterUpdate`. Gate ADR-0127 obrigatório. Nenhum teste
RED nem código L1 de P1148 foi escrito. A execução só pode continuar após o
dono confirmar explicitamente o novo L0/hash.

## 12. Materialização após confirmação do gate

O dono confirmou o gate. Foram materializados:

- `CounterUpdate::{Set(Vec<usize>), Step(NonZeroUsize), Func(Func)}`;
- migração dos producers automáticos para `CounterUpdate::step()`;
- aplicação completa de `Set` e `Step(level)` no `CounterRegistry`;
- transporte e pós-walk de callbacks de counter com `Engine + EvalContext`;
- `Eq + Hash` coerentes em `Func`, necessários aos payloads hasháveis;
- os seis fields estáticos de `Type::Counter`;
- `update` com inteiro, array ou callback e `step(level:)` nas formas estática
  e de instância.

Teste P1148 GREEN: descoberta dos seis fields e igualdade morfológica de
`counter.step(c, level: 2)`/`c.step(level: 2)` e
`counter.update(c, (3, 4))`/`c.update((3, 4))`.

Validação desta vertical:

- `cargo test -p typst-core counter_update --no-fail-fast`: 19 passaram;
- teste P1148 dedicado: 1 passou;
- `cargo check --workspace`: passou;
- `git diff --check`: passou;
- `crystalline-lint --fix-hashes .`: zero drift após resselo.

A suite integral `cargo test -p typst-core --lib` executou 5.219 testes:
5.179 passaram e 40 falharam. As falhas observadas pertencem ao baseline
sujo preservado (I/O indisponível no ambiente, expectativas antigas de P1147
para `int(float)` e regressões já presentes de gradient); nenhuma falha cita
P1148 ou `CounterUpdate`.

P1148 ainda não fecha: falta ampliar e medir integralmente os named/defaults
de `display(numbering:, at:, both:)`, a forma estática de literal label em
`at`, e testes de documento para o efeito do callback durante fixpoint.
