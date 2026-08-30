# P1149 — fechamento contextual da superfície `counter`

**Data:** 2026-08-24
**Estado:** `EXECUTADO — GREEN FOCADO; BASELINE INTEGRAL PRESERVADO`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** vertical de contrato e glue de P1148 materializada

## 1. Objetivo

Fechar os três resíduos explicitamente deixados por P1148, sem reabrir o
reshape público já aprovado:

1. assinatura completa de `counter.display` — `numbering`, `at:` e `both:`;
2. literal label na forma estática `counter.at(c, <label>)`;
3. efeito real de `counter.update(callback)` durante o fixpoint.

O passo termina apenas quando as formas estática e de instância forem
semanticamente equivalentes nos três eixos. Descoberta do field isolada ou
igualdade de `repr(counter-update(...))` não fecha comportamento contextual.

## 2. Proveniência inicial

- Hora: `2026-08-24T21:29:51-03:00`.
- HEAD: `7b0de6be400362e4b57c10d5c5acc532b2157389`.
- Working tree: apenas `typst-passo-1147.md` e `typst-passo-1148.md` não
  rastreados.
- Fonte de paridade:
  `a51e02804:crates/typst-library/src/introspection/counter.rs:370-517`.

Antes de executar, rebaselinear com:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

Qualquer número usado para decidir o fechamento deve citar esse novo estado.

## 3. Medição literal antes da decisão

A fonte ratificada define:

```typst
counter.display(
  self,
  numbering: auto,
  at: auto,
  both: false,
)

counter.at(self, selector)
counter.update(self, update)
```

Semântica medida na fonte:

- `display` resolve `at: auto` para a localização contextual atual;
- `at:` custom deve resolver exatamente uma location;
- `both: true` combina o estado naquela location com o valor final de nível
  superior antes de aplicar o numbering;
- numbering omitido/`auto` usa o numbering do elemento contado ou o fallback
  `"1.1"`;
- callback de numbering recebe cada componente como argumento separado;
- callback de `update` recebe igualmente os componentes anteriores separados,
  não um array único;
- `at` aceita `LocatableSelector`; label e location são casos importantes,
  mas não autorizam reduzir o contrato a strings.

O cristalino atual possui `parse_counter_display_args` apenas no caminho AST
de instância; o wrapper estático recebe `Args` já avaliados e delega
diretamente a `counter_display`. `apply_counter_funcs` existe, mas P1148 ainda
não o validou end-to-end e atualmente passa o estado como array único. Essa
forma é hipótese cristalina a refutar contra o vanilla antes de mantê-la.

## 4. Fase A — sondas vanilla nos dois binários ratificados

Executar cada caso em `lab/typst-original/target/release/typst` e
`/usr/local/bin/typst`; só aceitar resultado se ambos coincidirem.

### 4.1 `display`

Num documento determinístico com counter hierárquico, medir formas estática e
de instância para:

- numbering omitido e `auto`;
- patterns `"1"`, `"1."`, `"1.1"`, romano e alfabético;
- callback com parâmetros posicionais e sink;
- `at:` com label, location e selector que casa uma ocorrência;
- selector com zero e múltiplas ocorrências;
- `both: false` e `both: true` antes do fim do documento;
- combinações `numbering + at + both`;
- named desconhecido, tipo errado e argumentos excedentes.

Registrar conteúdo/repr, mensagens, hints e spans quando observáveis.

### 4.2 `at` estático

Medir no mesmo documento:

```typst
counter.at(c, <x>)
c.at(<x>)
counter.at(c, here())
c.at(here())
```

Adicionar label inexistente e selectors com cardinalidade zero/múltipla. A
equivalência é pelo array resultante ou diagnóstico, não pelo caminho AST.

### 4.3 callback de `update`

Medir callback sobre estados de um e vários componentes:

```typst
c.update(2)
c.update(xs => ...)
c.update((a, b) => ...)
c.update((..xs) => ...)
```

Separar conteúdo inserido de conteúdo atribuído/descartado. Medir retornos
`int`, `array<int>`, vazio, negativo e tipo inválido. Confirmar a aridade real
do callback e a propagação do diagnóstico.

## 5. Fase B — auditoria cristalina

Produzir matriz com `file:line`:

```text
caso | vanilla | cristalino | língua/mecânica | L0 | decisão
```

Auditar obrigatoriamente:

- `compiler/stdlib/counter.rs`: wrapper estático e helpers contextuais;
- `compiler/eval/bindings/value_methods.rs`:
  `parse_counter_display_args`, extração de label e dispatch de instância;
- `compiler/eval/call_dispatch.rs`: intercepção AST antes de avaliação;
- `compiler/introspect/from_tags.rs::apply_counter_funcs`;
- `compiler/introspect/fixpoint.rs`: ordem relativa a state funcs e displays;
- `CounterRegistry`/`Introspector`: valores por location e valor final;
- L0s vigentes desses módulos e hashes antes de propor alteração.

Não duplicar parsing de numbering, resolução de selector ou aplicação de
callback entre wrappers. O owner de counter continua dono da semântica.

## 6. Arquitetura a confirmar

- Extrair uma representação comum dos argumentos de `display` que possa ser
  construída tanto do AST de instância quanto de `Args` estáticos.
- Preservar literal label estático por intercepção cirúrgica no call dispatch
  somente se a avaliação genérica continuar apagando a informação.
- Resolver `at:` para `Location` antes de formatar e executar callback com a
  localização contextual correta.
- Implementar `both` no owner/introspector, sem assar total no eval.
- Passar componentes do estado como posicionais separados aos callbacks de
  numbering e update, se as sondas confirmarem a fonte.
- Manter match fechado e despacho estático; sem registry reflexivo.

Essa arquitetura é inferência. É refutada se as sondas mostrarem aridade,
fallback ou contexto diferentes, ou se o L0 vigente já determinar outra forma.

## 7. Nucleação e gate ADR-0127

L0s candidatos:

- `00_nucleo/prompts/compiler/stdlib/counter.md`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`, somente se houver
  intercepção sintática;
- `00_nucleo/prompts/compiler/introspect/from_tags.md`;
- L0 de `CounterRegistry`/`Introspector` somente se ganhar operação nova.

Correção interna de paridade usando contratos aprovados segue em fluxo
contínuo: L0 primeiro, resselo, RED→GREEN. Parar novamente no ADR-0127 se
aparecer qualquer nova assinatura pública Rust, comportamento padrão não
ratificado, mudança de fase ou incompatibilidade além do contrato aprovado em
P1148.

## 8. Plano RED→GREEN

1. RED da equivalência static/instância de `display` sem argumentos;
2. RED de numbering explícito, callback e fallback `auto`;
3. RED de `at:` label/location/selector e cardinalidade inválida;
4. RED de `both: true` com valor atual diferente do final;
5. RED de `counter.at(c, <label>)` estático e método equivalente;
6. RED do callback de update com um e vários componentes;
7. RED de retorno inválido e conteúdo descartado;
8. atualizar L0s e ressellar antes do primeiro código correspondente;
9. implementar owner comum e glue mínimo;
10. repetir sondas cristalinas equivalentes às vanilla;
11. executar regressões focadas e validação final.

Confirmar cada RED antes do GREEN. Um teste que já passa entra como regressão,
não como prova do delta.

## 9. Validação e baseline das falhas integrais

Obrigatório:

```text
cargo test -p typst-core <filtros P1149/counter>
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

P1148 registrou uma execução integral com 5.219 testes: 5.179 passaram e 40
falharam por baseline/ambiente (I/O indisponível, expectativa antiga de P1147
e regressões de gradient). P1149 deve reexecutar ou comparar nominalmente essa
lista. Falha nova em counter/introspecção é bloqueante; falha preexistente não
pode ser atribuída ao passo nem usada para ocultar regressão nova.

## 10. Critério de encerramento

P1149 fecha quando:

- `display` suporta e mede `numbering`, `at:` e `both:` nas duas formas;
- `counter.at(c, <label>)` preserva o literal e equivale a `c.at(<label>)`;
- callback de update tem aridade, retorno, inserção e erro iguais ao vanilla;
- não existe lógica contextual duplicada fora do owner;
- testes focados, build, format, diff e linter estão verdes;
- diferenças integrais restantes estão reproduzidas e classificadas por nome.

Ficam fora: refactor geral do fixpoint, novos selectors, limpeza das 40 falhas
de baseline e qualquer nova família pública além de `counter`.

## 11. Execução

### Rebaseline

- Hora: `2026-08-24T21:32:12-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree inicial: apenas este passo não rastreado.

### Medições ratificadas

Os dois binários vanilla devolveram resultados idênticos:

```text
update((2,3)); update((a,b)=>(a+1,b+2)) → (3,5)
display("1.1") estático/instância → "3.5"
at(<probe>) estático/instância → (3,5)
display("1 / 1", at:<probe>, both:true) estático/instância → "3 / 4"
```

### Materialização

- `apply_counter_funcs` passa componentes separados ao callback;
- `counter_display` aceita `numbering`, `at:` e `both:` no owner comum;
- `both` anexa o primeiro componente final ao estado na location;
- labels, locations e selectors são resolvidos no owner;
- callbacks de numbering recebem componentes separados;
- formas estáticas de `at`/`display` preservam literal label antes da
  avaliação genérica;
- removido o caminho P640 que concatenava texto localmente e ignorava
  callbacks.

### Verificação

- P1149: 2/2 testes passaram;
- regressões P640 de display: 6/6 passaram;
- regressões `counter_update`: 19/19 passaram;
- documento cristalino equivalente com callback, label e `both` compilou;
- `cargo check --workspace`: passou;
- `cargo build --workspace`: passou;
- `cargo fmt --all -- --check`: passou;
- `git diff --check`: passou;
- `crystalline-lint .`: zero violations de linhagem após resselo.

Suite integral: 5.221 testes, 5.181 passaram e as mesmas 40 falhas nominais
do baseline P1148 permaneceram. Nenhuma falha nova contém `counter`, P1149 ou
introspecção de counters.
