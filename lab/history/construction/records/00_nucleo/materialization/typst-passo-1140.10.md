# Passo 1140.10 — superfície pública completa de `linebreak`

**Estado:** executado
**Data:** 2026-08-24
**Natureza:** binding público + morfologia + reflexão de content
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127
**Predecessor:** P1140.9
**Complemento obrigatório:** P1140.11 — efeito visual de `justify: true`

## 1. Objetivo

Materializar a superfície pública de linguagem do elemento `linebreak`, hoje
parcial no cristalino:

- a sintaxe markup `\` já produz `Content::Linebreak` e quebra a linha;
- o construtor global `linebreak(...)` não existe no scope;
- `repr` devolve `linebreak` em vez da chamada canônica;
- o elemento unit não consegue transportar nem refletir `justify`;
- `.func()`, `.fields()`, `.has("justify")` e acesso ao campo não têm a
  semântica do vanilla.

Este passo fecha binding, eval, entidade, `repr` e reflexão. O efeito visual de
`justify: true` fica deliberadamente para P1140.11 porque exige alteração do
motor de linha/parágrafo, não apenas do elemento. A divisão é obrigatoriamente
registrada nos L0 atualizados por este passo; `justify` não pode ser declarado
como totalmente implementado ao fechar P1140.10.

## 2. Proveniência da medição inicial

Medição em `2026-08-24T11:24:45-03:00`:

- commit base: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- working tree não commitado;
- antes deste documento, `git diff HEAD --stat`: **28 ficheiros alterados, 516
  inserções e 105 remoções**;
- `git status --short | wc -l`: **31 entradas**;
- vanilla: `/usr/local/bin/typst`, baseline ratificado `a51e02804`;
- cristalino: `./target/debug/typst` da árvore de trabalho.

Repetir a proveniência no fecho; estes números não comprovam o estado final.

## 3. Medição anterior à decisão

### 3.1 Construtor e `repr`

| expressão | vanilla ratificado | cristalino atual |
|---|---|---|
| `type(linebreak)` | `function` | erro `unknown variable linebreak` |
| `repr(linebreak())` | `linebreak()` | erro `unknown variable linebreak` |
| `repr(linebreak(justify: true))` | `linebreak(justify: true)` | mesmo erro |
| `repr(linebreak(justify: false))` | `linebreak(justify: false)` | mesmo erro |

O vanilla distingue argumento omitido de `justify: false` explícito, como a
família `weak` corrigida em P1140.9.

### 3.2 Sintaxe markup

| expressão | vanilla ratificado | cristalino atual |
|---|---|---|
| `repr([\ ])` | `sequence(linebreak(), [ ])` | `sequence(linebreak, [ ])` |
| `repr([a\ b])` | `sequence([a], linebreak(), [ ], [b])` | `sequence([a], linebreak, [ ], [b])` |

A sintaxe já cria a morfologia correta e `plain_text()` já contém newline. A
divergência aqui é o braço stub de `repr`; não se deve reescrever lexer ou
parser para corrigi-la.

### 3.3 Campos e reflexão

Medição no vanilla:

```text
linebreak().func() == linebreak              -> true
linebreak().fields()                         -> (:)
linebreak(justify: false).fields()            -> (justify: false)
linebreak().has("justify")                    -> false
linebreak(justify: false).has("justify")      -> true
linebreak().justify                           -> erro: field "justify" ... is not known
linebreak(justify: true).justify              -> true
linebreak(justify: false).justify             -> false
```

`typst eval` imprime dicts como JSON (`{}`/`{"justify":false}`) sem `repr`;
as formas `(:)`/`(justify: false)` acima expressam o valor Typst canônico para
os critérios de aceitação.

### 3.4 Fonte ratificada para `justify`

No vanilla pinado:

- `typst-library/src/text/linebreak.rs:23-37`: `LinebreakElem` declara
  `justify: bool`, default `false`, descrito como “justify the line before the
  break”;
- `typst-eval/src/markup.rs:106-110`: a sintaxe markup devolve o singleton sem
  field assente, portanto sempre uma quebra não justificada;
- `typst-layout/src/inline/collect.rs:191-195`: o collector emite U+2028 quando
  `justify` é verdadeiro e `\n` quando falso.

O cristalino não possui collector de parágrafo equivalente: o braço
`Content::Linebreak` em `compiler/layout/mod.rs` chama diretamente
`flush_line()`. Implementar a superfície não basta para produzir o efeito
visual de justificação.

## 4. Classificação

- Binding, chamada, `repr`, campos e reflexão são semântica/morfologia da
  linguagem e exigem paridade (ADR-0107).
- A representação Rust é livre, mas precisa preservar valor e presença.
- O efeito de `justify` é layout observável e não pode ser considerado fechado
  por apenas armazenar o booleano.
- Adicionar o binding muda comportamento público por defeito; mudar
  `LinebreakElem` unit para struct com campos muda contrato público. Ambos
  acionam o gate ADR-0127.

## 5. Auditoria L0 e paragem obrigatória

Antes de código, ler e atualizar:

- `00_nucleo/prompts/entities/elements/linebreak.md`;
- `00_nucleo/prompts/compiler/stdlib/layout.md`;
- `00_nucleo/prompts/compiler/stdlib/foundations/repr.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- `00_nucleo/prompts/compiler/layout.md` somente para declarar o scope-out
  nomeado P1140.11; não especificar algoritmo visual sem medição própria.

O L0 deve declarar explicitamente que P1140.10 entrega a superfície mas deixa
o efeito visual de `justify: true` incompleto até P1140.11, cumprindo a regra de
funcionalidade dividida entre passos.

Fluxo obrigatório:

1. atualizar os L0;
2. calcular os hashes novos sem editar L1;
3. parar e obter confirmação do dono;
4. somente então escrever testes RED e implementação.

## 6. Contrato público recomendado

`LinebreakElem` deixa de ser unit e passa a:

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinebreakElem {
    pub justify: bool,
    pub justify_explicit: bool,
}
```

Regras:

1. `justify` é o valor funcional; default `false`.
2. `justify_explicit` preserva se o named apareceu, inclusive
   `justify: false`.
3. `PartialEq` e `Hash` incluem ambos, pois mudam `repr`, `.fields()` e
   `.has()`.
4. `Content::linebreak()` mantém assinatura e produz `false/false`, servindo à
   sintaxe markup e aos consumidores internos históricos.
5. Acrescentar
   `Content::linebreak_with_justify_presence(justify, justify_explicit)` para o
   caminho da nativa.
6. Não usar mapa genérico, `dyn`, vtable ou import reverso.

## 7. Nativa e binding

Implementar `native_linebreak` em `compiler/stdlib/layout.rs`, pois o L0 de
`stdlib/text` já decide que `text/linebreak.rs` do vanilla pertence no
cristalino aos L0 de eval/layout, não a um novo nó do hub `text`.

Contrato:

```text
linebreak(justify: bool = false) -> Content
```

- zero posicionais;
- apenas named `justify`;
- tipo diferente de bool produz diagnóstico;
- desconhecido produz diagnóstico;
- omissão cria `justify = false, justify_explicit = false`;
- named cria `justify_explicit = true` para true ou false.

Reexportar pela fronteira stdlib já usada por `layout::*` e registrar no scope
global em `compiler/eval/mod.rs` como `Value::Func(Func::native(...))`.
`type(linebreak)` deve resultar `function`; não convertê-lo em `Value::Type`.

## 8. `repr` e reflexão

### 8.1 `repr_content`

- `false/false` → `linebreak()`;
- `true/true` → `linebreak(justify: true)`;
- `false/true` → `linebreak(justify: false)`.

Isso corrige simultaneamente a forma construtor e a sintaxe markup, pois ambas
chegam ao mesmo braço exaustivo.

### 8.2 Campos

`LinebreakElem::get_field("justify")` devolve `Value::Bool` somente quando
`justify_explicit`; quando omitido, o campo é declarado porém unset. Como
`Element::get_field` retorna apenas `Option<Value>`, o dispatcher de
`content_field` deve ter braço dedicado para distinguir `Unset` de
`Undeclared`, seguindo o precedente de Heading.

Adicionar `Linebreak` aos candidatos de `content_set_fields` na ordem única
`["justify"]`. Assim:

- `.fields()` inclui somente o valor explícito;
- `.has("justify")` acompanha presença;
- acesso direto ao campo omitido emite o erro vanilla de campo não assente;
- `.func()` resolve para a mesma função global `linebreak`.

Não tratar o default como campo assente.

## 9. RED → GREEN

Antes da implementação, adicionar testes para:

1. `type(linebreak) == function`;
2. os três `repr(linebreak(...))` do §3;
3. as duas formas markup do §3;
4. `.func() == linebreak`;
5. `.fields()` omitido/false/true;
6. `.has("justify")` omitido/false/true;
7. acesso direto omitido erra e explícito devolve bool;
8. posicionais, named desconhecido e tipo inválido são rejeitados;
9. `Content::linebreak()` preserva newline, `is_empty == false` e
   `justify == false/false`;
10. igualdade/hash distinguem omitido de false explícito.

Confirmar RED por binding ausente, stub de repr e ausência de campo. Depois
implementar a menor mudança que satisfaça a superfície, sem alterar
`flush_line()` neste passo.

## 10. Preservação de comportamento

Reexecutar os testes P584 e math P996/P997. A sintaxe `\` deve continuar:

- produzindo `Content::Linebreak` com `justify_explicit = false`;
- introduzindo newline em `plain_text`;
- separando linhas math como antes;
- chamando o mesmo `flush_line()` não justificado no layout;
- sem mudar paginação, leading ou espaçamento vertical.

Consumidores internos como bibliografia, table, figure e outline continuam a
usar `Content::linebreak()` e, portanto, quebra não justificada.

## 11. O que entra junto

Entra por compartilhar o mesmo elemento e a mesma presença de field:

- binding global;
- validação de argumentos;
- `repr` da função e da sintaxe markup;
- `.func()`, `.fields()`, `.has()` e field access;
- igualdade/hash e construtores;
- regressões de math/markup/layout não justificado.

Não entra:

- justificação visual da linha anterior;
- `par(justify:)` real;
- hifenização ou line-breaking automático;
- refatoração geral do collector de parágrafo;
- outros elementos globais ausentes;
- mensagens byte-idênticas fora dos erros próprios de `linebreak`.

## 12. P1140.11 obrigatório — efeito visual de `justify`

P1140.11 deve medir posições no vanilla e no cristalino e implementar o efeito
semântico de `linebreak(justify: true)`. O critério não pode ser apenas que o
bool chegou ao Layouter.

A auditoria deve decidir, a partir dos FrameItems reais, como distribuir o
espaço restante entre oportunidades da linha anterior. O cristalino pode ter
espaços incorporados num único item de texto por P1137; portanto, simplesmente
transladar items depois de `Content::Space` pode ser insuficiente. Antes do L0
de P1140.11, medir:

- linha com três palavras e largura fixa;
- `justify: false`, `true` e omitido;
- sintaxe markup `\` sempre não justificada;
- linha sem oportunidades de espaço;
- RTL;
- shaped text e fallback sem fonte;
- decorações e links que atravessam os gaps.

P1140.10 só fecha se esse scope-out estiver explícito nos L0; não pode declarar
paridade total de `justify`.

## 13. Validação

Após confirmação do gate e implementação:

```sh
cargo test -p typst-core p1140_10
cargo test -p typst-core p584_escape_shorthand_linebreak_em_markup_preservados
cargo test -p typst-core p996
cargo test -p typst-core p997
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo build --workspace
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Reexecutar as sondas dos §§3.1–3.3 nos dois binários com proveniência completa.

## 14. Critérios de aceitação

O passo fecha quando:

- o binding global existe e tem tipo `function`;
- construtor, markup e `repr` coincidem com o vanilla nos casos medidos;
- presença explícita de `justify: false` sobrevive ao eval;
- reflexão e `.func()` coincidem com o vanilla;
- sintaxe markup e consumidores históricos continuam não justificados;
- math P996/P997 e plain text não regridem;
- o L0 declara inequivocamente P1140.11 como complemento do efeito visual;
- L0, linhagem e hashes estão coerentes;
- suítes L1/L3, build, fmt, lint e diff-check passam;
- o diagnóstico final não afirma que `justify: true` já altera layout.

## 15. Próximo passo

Executar P1140.11 para o efeito visual de `justify`. Somente depois dos testes
posicionais RED→GREEN a família `linebreak` pode ser declarada semanticamente
completa.

## 16. Gate L0 executado

Em 2026-08-24 foram atualizados os cinco L0 do §5. A decisão adotada transforma
`LinebreakElem` em struct com os campos públicos `justify` e
`justify_explicit`, registra a nativa em `stdlib/layout`, especifica binding e
reflexão em eval, corrige `repr` e declara P1140.11 como complemento obrigatório
do efeito visual.

Hashes L0 calculados pelo linter, ainda não escritos nos headers L1:

- `compiler/eval.md`: `2604e194`;
- `compiler/layout.md`: `ecf1f566`;
- `compiler/stdlib/foundations/repr.md`: `f7f98cf1`;
- `entities/elements/linebreak.md`: `28ed11e6`.

`compiler/stdlib/layout.rs` aponta para `compiler/stdlib/layout.md`, mas não
possui `@prompt-hash`; a ausência é preexistente e nenhum hash será inventado.
Nenhum ficheiro L1 foi alterado nesta fase.

O dono confirmou o gate L0. A implementação pública só começou depois dessa
confirmação.

## 17. Execução e fecho

O teste RED foi executado contra `typst-core`: dois testes falharam com
`unknown variable linebreak`, comprovando a ausência do binding. Depois da
implementação, os três testes P1140.10 passaram.

Foram materializados:

- `LinebreakElem { justify, justify_explicit }` e os construtores de `Content`;
- `native_linebreak` e o binding global;
- `repr` canônico para omissão, `false` e `true` explícitos;
- reflexão por `.func()`, `.fields()`, `.has()` e acesso direto;
- preservação da sintaxe markup como `false/false`;
- resselo automático dos L0 de eval, layout, repr e linebreak.

Validação final em `2026-08-24T11:38:04-03:00`, commit base
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`, working tree não commitado:

- `git diff HEAD --stat`: **46 ficheiros, 789 inserções, 140 remoções**;
- `git status --short`: **50 entradas**;
- `cargo test -p typst-core`: **5150 passed, 0 failed**;
- `cargo test -p typst-infra`: **828 passed, 0 failed**;
- P584: **1 passed**; filtro P996: **3 passed**; filtro P997: **15 passed**;
- `cargo build`: passou;
- `cargo fmt --all -- --check`: passou após formatação;
- `crystalline-lint .`: exit 0, sem violations bloqueantes; avisos/info
  preexistentes continuam reportados;
- `git diff --check`: passou.

O efeito visual de `justify: true` não foi implementado nem declarado como
fechado. Ele permanece exclusivamente no P1140.11.
