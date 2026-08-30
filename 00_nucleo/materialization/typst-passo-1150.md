# P1150 — superfície estática completa de `content`

**Data:** 2026-08-24
**Estado:** `EXECUTADO — GREEN FOCADO; BASELINE INTEGRAL PRESERVADO`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Sentinela de origem:** `content.fields`
**Dependência:** P1149 materializado na working tree, ainda não commitado

## 1. Objetivo

Auditar e materializar a superfície pública estática completa do valor-tipo
`content`, reutilizando os cinco métodos de instância já implementados no
owner `compiler/eval/bindings/field_access.rs`:

```text
func, has, at, fields, location
```

`content.fields` é apenas a sentinela do handoff pós-P1140. O passo não pode
expor somente esse membro nem duplicar a lógica de fields num módulo genérico.

## 2. Proveniência e preservação da working tree

- Hora: `2026-08-24T21:44:44-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Baseline vanilla: `a51e02804`.
- Working tree P1149 não commitada: 11 ficheiros rastreados alterados, com
  375 inserções e 120 remoções, mais `typst-passo-1149.md` não rastreado.

P1150 deve preservar integralmente esse lote. Antes de executar, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

Se o estado mudar, rebaselinear. Não atribuir a P1150 diferenças originadas
em P1149.

## 3. Inventário literal já medido

O `#[scope] impl Content` ratificado em
`a51e02804:crates/typst-library/src/foundations/content/mod.rs:525-604` contém
exatamente cinco functions:

| Membro | Forma de linguagem |
|---|---|
| `func` | `content.func(self)` / `self.func()` |
| `has` | `content.has(self, field)` / `self.has(field)` |
| `at` | `content.at(self, field, default: ...)` / método equivalente |
| `fields` | `content.fields(self)` / `self.fields()` |
| `location` | `content.location(self)` / `self.location()` |

No cristalino, `eval_content_method` já implementa os cinco, incluindo
mensagens, fields explicitamente assentes e defaults. `method_dispatch` já
classifica os cinco como métodos de `Value::Content`. Não foi encontrado
braço `Type::Content` que exponha o scope estático.

Isso sugere glue estático sobre owner existente. É inferência até as formas
estática e de instância serem medidas caso a caso.

## 4. Fase A — sondas nos dois binários vanilla

Executar em `lab/typst-original/target/release/typst` e
`/usr/local/bin/typst`. Só ratificar quando ambos coincidirem.

### 4.1 Descoberta

Para os cinco membros:

- `type(content.member)`;
- field sem chamada e igualdade/identidade quando observável;
- field inexistente (`content.from`, por exemplo);
- receiver ausente e receiver de tipo errado.

### 4.2 Equivalência sobre conteúdos representativos

Comparar estática/instância sobre:

- texto, strong, emph e heading;
- elemento com fields opcionais assentes e não assentes;
- conteúdo styled;
- sequence e uma variante interna sem constructor chamável;
- conteúdo locatable obtido por query/show rule, quando disponível.

### 4.3 Semântica por membro

`func`:

- tipo e igualdade com o constructor global do elemento;
- chamada da função devolvida quando suportada;
- variante interna sem constructor público.

`has` e `at`:

- field assente, declarado mas não assente e inexistente;
- `default:` presente/ausente;
- field não-string, named desconhecido, positional excedente;
- mensagens, hints e spans, pois erro é observável.

`fields`:

- ordem pública das chaves;
- somente fields efetivamente assentes;
- valores preservados em nível de linguagem;
- dict vazio quando aplicável.

`location`:

- conteúdo inline sem location;
- conteúdo retornado por query/show rule com location;
- array/location/none exatos nas duas formas.

Não usar igualdade Rust, ordem interna de maps ou identidade de `Arc` como
critério de paridade.

## 5. Fase B — auditoria cristalina

Produzir matriz:

```text
caso | vanilla | cristalino | língua/mecânica | file:line |
L0 afetado | contrato/default/fase? | decisão
```

Auditar:

- `Type::Content` e global `content` no scope;
- `eval_value_field_access` para valor-tipo;
- `method_dispatch` e `eval_content_method`;
- `content_field`, `content_set_fields` e `content_elem_func`;
- chamabilidade das funções retornadas por `func`;
- metadados/location preservados ou descartados pelo domínio `Content`;
- testes P829 e regressões posteriores de fields.

Separar o delta de exposição estática das divergências preexistentes da forma
de instância. Em particular, `location()` atualmente devolve sempre `none` no
cristalino; expor a forma estática não autoriza chamar essa divergência de
paridade completa.

## 6. Nucleação e arquitetura esperada

L0s candidatos:

- `00_nucleo/prompts/compiler/eval/bindings/field_access.md`;
- `00_nucleo/prompts/compiler/eval/bindings/method_dispatch.md`, somente se o
  dispatch de instância mudar;
- `00_nucleo/prompts/entities/value.md`, para o scope de `Type::Content`;
- L0 de `Content`/location somente se a auditoria exigir mudança de domínio.

Arquitetura inicial:

- `content_type_field(field)` no owner de content/field access;
- match fechado com os cinco nomes;
- wrappers não ligados recebem `Value::Content` primeiro;
- wrappers e métodos delegam a `eval_content_method` com os mesmos `Args`;
- `eval_value_field_access` apenas descobre o field do valor-tipo;
- sem registry reflexivo, macro global de métodos ou cópia de tabelas.

Se o owner atual não puder ser chamado pelo ABI nativo sem duplicação,
extrair helper comum dentro do mesmo nó. Não mover a semântica para
`call_dispatch`.

## 7. Classificação ADR-0107/0108 e gate ADR-0127

São língua: presença dos fields, receiver, argumentos, defaults, dict/ordem
pública, função de elemento, location e diagnósticos. São mecânica: match Rust,
function pointers, representação do dict e identidade interna do content.

Exposição estática por wrappers privados sobre comportamento vigente é
correção de paridade interna: L0 primeiro, resselo e RED→GREEN contínuo.

Parar no gate ADR-0127 se surgir:

- novo método/campo/assinatura Rust pública;
- armazenamento novo de location em `Content`;
- mudança do default de `at` ou comportamento por defeito;
- transporte entre eval e introspecção/layout;
- quebra de compatibilidade.

A divergência de `location()` é o risco principal. Não ampliar o passo
silenciosamente para remodelar `Content`.

## 8. Plano RED→GREEN

1. RED da descoberta dos cinco fields em `Type::Content`;
2. RED da equivalência `func` estática/instância;
3. RED de `has` para field assente/não assente/inexistente;
4. RED de `at`, incluindo `default:` e diagnósticos;
5. RED de `fields`, valores e ordem pública;
6. RED de `location` no subset já suportado;
7. RED de receiver, aridade, named e tipos inválidos;
8. atualizar L0s e ressellar antes do código correspondente;
9. implementar tabela fechada e owner comum;
10. repetir sondas cristalinas equivalentes;
11. executar regressões focadas e validação final.

Confirmar RED antes do GREEN. Teste que já passa é regressão, não prova do
delta.

## 9. Validação

Obrigatório:

```text
cargo test -p typst-core <filtros P1150/content/P829>
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Reexecutar a suite integral ou comparar nominalmente com o baseline mais
recente. P1149 terminou com 5.181 testes verdes e 40 falhas preexistentes;
nenhuma falha nova de content/field access pode ser absorvida nessa lista.

## 10. Critério de encerramento e limites

P1150 fecha quando:

- os cinco fields estáticos existem e equivalem aos métodos no subset
  suportado;
- a sentinela `content.fields` não é a única correção;
- parsing/defaults/diagnósticos vivem num owner único;
- divergências preexistentes, especialmente location de conteúdo locatável,
  ficam explicitamente classificadas e com sucessor nomeado se permanecerem;
- testes focados, build, format, diff e linter estão verdes.

Ficam fora, salvo dependência inevitável medida:

- remodelagem geral de metadados de `Content`;
- refactor de todos os elementos/fields;
- novos constructors de variantes internas;
- limpeza das 40 falhas integrais de baseline;
- numbering por função, `Symbol` multi-codepoint e HTML.

## 11. Execução

### 11.1 Rebaseline e sondas

- Rebaseline: `2026-08-24T21:46:58-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Estado: working tree não commitada; o lote P1149 foi preservado. No início
  de P1150 havia 11 ficheiros rastreados alterados, 375 inserções e 120
  remoções, além de `typst-passo-1149.md` não rastreado.
- Vanilla ratificado: os binários
  `lab/typst-original/target/release/typst` e `/usr/local/bin/typst`
  coincidiram.

Os dois binários devolveram `function` para `func`, `has`, `at`, `fields` e
`location`. Sobre `strong[Hi]`, as formas estática e de instância coincidiram:

```text
content.fields(x) == x.fields()
content.has(x, "body") == x.has("body") == true
content.at(x, "body") == x.at("body")
content.location(x) == x.location() == none
content.func(x) == x.func() == strong
content.at(x, "missing", default: 7) == x.at("missing", default: 7) == 7
```

### 11.2 Decisão e nucleação

A medição confirmou glue estático sobre o owner existente. Não surgiu método,
campo ou assinatura Rust pública, mudança de default, mudança de fase ou quebra
de compatibilidade; portanto o gate ADR-0127 não foi reaberto.

Antes do código, foram atualizados e depois ressellados:

- `00_nucleo/prompts/compiler/eval/bindings/field_access.md` — hash
  `f8480621`;
- `00_nucleo/prompts/entities/value.md` — hash `1b4140a5`.

### 11.3 RED→GREEN e implementação

O teste `p1150_content_scope_estatico_equivale_a_instancia` falhou primeiro
com `type content does not contain field "func"`. Depois foi acrescentado em
`field_access.rs` um match fechado de `Type::Content` para os cinco nomes e
wrappers nativos privados que extraem o primeiro argumento `Content` e delegam
ao `eval_content_method` vigente. Não houve duplicação da semântica dos
métodos nem alteração de contrato público Rust.

Resultado GREEN:

- teste P1150: 1 aprovado;
- regressões P829: 23 aprovadas;
- `cargo check --workspace`: aprovado;
- `cargo build --workspace`: aprovado;
- `cargo fmt --all -- --check`: aprovado;
- `git diff --check`: aprovado;
- `crystalline-lint .`: zero violações.

### 11.4 Suite integral

Medição final em `2026-08-24T21:51:14-03:00`, no mesmo HEAD e com working tree
não commitada descrita por `git status --short`/`git diff HEAD --stat`: 5.182
testes aprovados e 40 falhas preexistentes, zero ignorados. O acréscimo de um
GREEN face ao baseline P1149 corresponde exatamente ao novo teste P1150; a
lista nominal das 40 falhas permaneceu sem falha nova de content/field access.

P1150 fica encerrado no subset suportado. A divergência preexistente de
location locatável permanece fora deste passo; tratá-la exige medição própria
e pode alcançar o gate ADR-0127 se demandar novo armazenamento público ou
mudança de fase.
