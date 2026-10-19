# Passo 1140.2 — `label` como tipo chamável e remoção da extensão homónima

**Data:** 2026-08-23  
**Origem:** P1140, último `WRONG_KIND` não-math  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** correção de paridade pública com quebra da extensão cristalina  
**Gate:** contrato e comportamento por defeito — ADR-0127

## 1. Objetivo

Corrigir o binding global `label` para o tipo chamável da linguagem Typst,
preservando a sintaxe dedicada `<nome>` e separando-a do mecanismo interno
`Content::Label` usado para anexar labels a elementos.

Ao final:

```typst
repr(type(label))       == "type"
label("x")              == <x>
type(label("x"))        == label
repr(label("x"))        == "<x>"
str(label("a b"))       == "a b"
label("x", [body])      // erro: argumento inesperado
```

P1141 continua reservado para SVG. Os dois casos math restantes de P1140
pertencem a P1140.3.

## 2. Proveniência

Medição realizada em `2026-08-23`, sobre:

- HEAD cristalino `fcbc9763f8925d5c27b3670e35597b9adc0412a0`;
- working tree não commitada, após P1140.1;
- vanilla `lab/typst-original/target/release/typst`, correspondente ao hash
  ratificado `a51e02804`;
- cristalino `target/release/typst`, rebuild de P1140.1;
- fonte vanilla
  `lab/typst-original/crates/typst-library/src/foundations/label.rs:7-99`;
- implementação cristalina
  `01_core/src/compiler/stdlib/label.rs:1-144`.

Comandos reproduzíveis:

```sh
lab/typst-original/target/release/typst eval 'repr(type(label))' --format json
lab/typst-original/target/release/typst eval 'repr(label("x"))' --format json
lab/typst-original/target/release/typst eval 'repr(label("x", [body]))' --format json
target/release/typst eval 'repr(type(label))' --format json
target/release/typst eval 'repr(label("x"))' --format json
target/release/typst eval 'repr(label("x", [body]))' --format json
```

Antes de usar os números finais, repetir a medição e registrar
`git diff HEAD --stat` e hora exata, conforme a regra de proveniência.

## 3. Medição anterior à decisão

### 3.1 Fonte normativa vanilla

O vanilla declara `Label` com `#[ty(scope, cast)]` e o construtor com
`#[func(constructor)]`. `construct(name: Str)` recebe exatamente uma string
não vazia e devolve `Label` (`label.rs:49-89`). A sintaxe `<nome>` é uma
forma dedicada do mesmo tipo (`label.rs:28-48`).

O `repr` é condicional (`label.rs:92-99`):

- identificador válido para a sintaxe literal → `<nome>`;
- string não representável pela sintaxe dedicada → `label("...")`.

### 3.2 Observáveis medidos

| Expressão | Vanilla | Cristalino antes de P1140.2 |
|---|---|---|
| `repr(type(label))` | `"type"` | `"function"` |
| `repr(label("x"))` | `"<x>"` | `"label(\"x\", [])"` |
| `repr(type(label("x")))` | `"label"` | `"content"` |
| `repr(type(<x>))` | `"label"` | `"label"` |
| `repr(<x>)` | `"<x>"` | `"<x>"` |
| `label("x") == <x>` | `true` | `false` pela morfologia dos valores |
| `str(label("a b"))` | `"a b"` | construtor ainda produz content |
| `label("")` | erro `label name must not be empty` | erro de nome vazio |
| `label("x", [body])` | erro `unexpected argument` | aceita e produz `Content::Label` |
| `label("x")[body]` | erro `unexpected argument` | aceita como a mesma extensão |

O inventário P1140.1 pós-implementação tem 798 `MATCH` e três
`WRONG_KIND`: `label`, `math.equation` e `math.sqrt`. Logo `label` é a única
divergência de kind não-math restante.

### 3.3 Uso observado da extensão

A busca por chamadas Typst `label("...", ...)` não encontrou consumidor `.typ`
no repositório. Os usos encontrados são chamadas Rust de `Content::label` em
testes de introspecção/layout/export, além do próprio `native_label` e dos L0
históricos. Isso refuta a hipótese de que remover a forma pública de dois
argumentos exige remover a entidade interna.

Inferência: a extensão pública pode ser removida sem migrar fixtures Rust.
Seria refutada por um consumidor Typst real fora da busca atual; por isso a
matriz e os fixtures de integração devem ser executados antes do fechamento.

## 4. Classificação

As diferenças são semântica e morfologia da linguagem:

- o binding é `type`, não `function`;
- o construtor produz `Value::Label`, não `Value::Content`;
- a forma de `repr` depende da validade do identificador;
- aridade dois é erro público no vanilla.

Portanto ADR-0107 exige paridade. Manter `label(name, body)` por conveniência
não é divergência mecânica: altera programa aceito e o valor produzido.

## 5. Decisão proposta

### 5.1 Binding e chamada

1. `make_stdlib()` registra `label` como `Value::Type(Type::Label)`.
2. `Type::is_callable()` inclui `Type::Label`.
3. O match fechado de `call_dispatch` delega `Type::Label` a `native_label`.
4. `native_label` aceita exatamente um `Value::Str`, não vazio, e devolve
   `Value::Label(Label(name))`.
5. Zero e dois ou mais posicionais, named args e tipos diferentes de string
   falham; o segundo argumento deixa de ser aceito.

Não criar registry, vtable, `dyn`, PropMap ou payload de função em `Type`.

### 5.2 Sintaxe dedicada

`Expr::Label` em modo de código continua produzindo `Value::Label`. Em markup,
`SyntaxKind::Label` continua anexando retroativamente a label ao elemento
precedente por `Content::label_auto`. A correção do construtor não altera
lexer, parser, associação, warnings de label órfã ou introspecção.

### 5.3 `Content::Label` interno

`Content::Label`, `LabelElem`, `Content::label` e `Content::label_auto`
permanecem em L1 porque representam o mecanismo de associação/introspecção,
não o valor `label` da linguagem. Os testes Rust podem continuar construindo
essa forma diretamente.

O significado histórico `auto: false = criado por #label(name, body)` deve ser
corrigido nos L0/comentários: passa a significar construção interna/explícita
de `Content::Label`, sem prometer sintaxe pública inexistente.

### 5.4 `repr` e `str`

`repr_value(Value::Label)` deve espelhar a morfologia vanilla:

- nomes aceitos pela gramática de label literal → `<nome>`;
- demais strings não vazias → `label(<repr da string>)`.

A regra de validade deve ter uma única função pura dona, compartilhável com
lexer/parser se já existir; não duplicar regex divergente. `str(Value::Label)`
devolve o nome interno sem delimitadores.

## 6. L0 obrigatóio antes do código

Atualizar primeiro:

- `00_nucleo/prompts/entities/label.md` — construtor, validade, identidade;
- `00_nucleo/prompts/entities/value.md` — `Type::Label` chamável;
- `00_nucleo/prompts/compiler/stdlib/label.md` — substituir a extensão pelo
  construtor real de um argumento;
- `00_nucleo/prompts/compiler/eval.md` — binding global;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md` — arm estático;
- `00_nucleo/prompts/compiler/stdlib/foundations.md` — morfologia condicional
  de `repr` (L0 vigente de `compiler/eval/repr.rs`);
- `00_nucleo/prompts/compiler/stdlib/foundations/str.md` — cast para string;
- `00_nucleo/prompts/entities/content.md` e
  `entities/elements/label.md` — retirar a promessa pública de
  `label(name, body)`, preservando o mecanismo interno.

Como há mudança de comportamento default e quebra de compatibilidade, redigir
e ressellar esses L0 e **parar no gate ADR-0127** antes do RED.

## 7. Sequência de execução

1. Repetir probes e registrar HEAD, diff stat e hora.
2. Ler integralmente os L0 donos e confirmar seus hashes vigentes.
3. Redigir as alterações L0 da seção 6; ressellar; parar para confirmação.
4. Escrever testes RED de kind, chamabilidade, igualdade, aridade, `repr` e
   `str`.
5. Alterar `native_label` para produzir `Value::Label` com aridade um.
6. Alterar binding, `is_callable` e o match de `call_dispatch`.
7. Implementar a forma condicional de `repr` e o cast `str(label)`.
8. Atualizar testes antigos de `native_label`: não apagá-los; convertê-los
   para o novo contrato e manter testes separados de `Content::label`.
9. Reexecutar inventário P1140: `WRONG_KIND` deve cair de 3 para 2, contendo
   apenas `math.equation` e `math.sqrt`.
10. Executar testes focados, matriz, `cargo build`, `crystalline-lint .` e
    `git diff --check`.

## 8. Testes obrigatórios

```typst
repr(type(label)) == "type"
repr(type(label("x"))) == "label"
type(label("x")) == label
label("x") == <x>
repr(label("x")) == "<x>"
repr(label("a b")) == "label(\"a b\")"
str(label("a b")) == "a b"
label("")                       // erro
label()                         // erro
label(1)                        // erro
label("x", [body])             // erro unexpected argument
label("x", body: [body])       // erro unexpected argument
```

Não regressão:

- `<x>` em código continua `Value::Label`;
- label de markup continua anexada ao elemento precedente;
- label órfã continua produzindo o warning vigente;
- `@x`, `query(<x>)`, `locate(<x>)` e show por label continuam funcionando;
- `Content::label`/`label_auto` continuam cobrindo introspecção e fixtures
  Rust sem reintroduzir a extensão pública.

## 9. Critérios de aceitação

- [ ] L0 mede antes de decidir e é confirmado no gate ADR-0127.
- [ ] `label` é `Value::Type(Type::Label)` chamável.
- [ ] O construtor aceita exatamente uma string não vazia.
- [ ] `label(name, body)` deixa de ser linguagem aceita.
- [ ] `Content::Label` permanece mecanismo interno distinto de `Value::Label`.
- [ ] Sintaxe `<nome>` e associação de markup não regridem.
- [ ] `repr` escolhe literal ou chamada conforme validade do nome.
- [ ] `str(label)` devolve o nome interno.
- [ ] Inventário deixa apenas os dois `WRONG_KIND` math.
- [ ] Nenhuma mudança incidental em math ou SVG.
- [ ] Testes, build, lint e diff-check passam; falhas preexistentes permanecem
      explicitamente registradas, nunca mascaradas.

## 10. Limites e próxima frente

Este passo não altera:

- estrutura de `LabelElem` ou algoritmo de introspecção;
- gramática permitida por `<nome>`, salvo consolidar sua função de validade;
- `math.equation`, `math.sqrt` ou outros membros math;
- P1141/SVG;
- metadados reflexivos `ParamInfo`.

Depois do fechamento, escrever P1140.3 para medir e corrigir separadamente
`math.equation` e a colisão `math.sqrt` função/símbolo.

## 11. Estado de execução — gate L0 (2026-08-23)

Os L0 donos foram medidos, atualizados e preparados para resselo. A execução
para obrigatoriamente antes dos testes RED e do código semântico porque este
passo remove a extensão pública `label(name, body)` e altera o kind público do
binding, classes cobertas pelo gate ADR-0127. A continuação requer confirmação
humana explícita do L0 ressellado.

## 12. Fechamento da implementação (2026-08-23)

O humano confirmou o L0 ressellado e liberou o gate. O RED focado produziu
quatro falhas esperadas: retorno `Content::Label`, aceitação do segundo
posicional, `Type::Label` não chamável e `repr` literal para nome com espaço.
Após a implementação, os sete testes P1140.2 ficaram GREEN.

Implementado:

- binding global como `Value::Type(Type::Label)`;
- despacho estático `Type::Label` → `native_label`;
- construtor de exatamente uma string não vazia para `Value::Label`;
- rejeição da antiga forma pública de dois argumentos e de named args;
- `repr` compartilhando a validade lexical da sintaxe de label;
- `str(Value::Label)` devolvendo o nome interno;
- preservação de `Expr::Label`, `Content::label` e `Content::label_auto`.

Medição de inventário às `2026-08-23T22:21:03-03:00`, HEAD
`0314efaea6a5cb4c377b9190c814e3b38a080d68`, working tree não commitada:
`MATCH = 799`, `WRONG_KIND = 2`. Os dois `WRONG_KIND` restantes são apenas
`math.equation` e `math.sqrt`. Os 22 probes passaram de 3 para 7 coincidências;
as divergências restantes pertencem às frentes posteriores já catalogadas.

A suíte integral de L1 executou 5.123 testes: 5.121 passaram e permanecem as
duas falhas preexistentes P862 sobre atomização de `Content::Text` em espaços
(`p862_content_tree_splits_plain_text_on_space` e
`p862_repr_plain_text_splits_on_space`). Nenhuma das duas atravessa `label`;
os testes focados P1140.2, do módulo `label` e de `repr` passaram.
