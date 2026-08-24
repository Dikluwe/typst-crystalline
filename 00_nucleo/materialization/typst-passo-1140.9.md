# Passo 1140.9 — `repr` fiel dos primitivos de espaçamento e quebra

**Estado:** executado
**Data:** 2026-08-24
**Natureza:** paridade de linguagem + preservação de presença de argumentos
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127
**Predecessor:** P1140.8

## 1. Objetivo

Corrigir a representação observável de quatro elementos que hoje caem em
stubs adjacentes de `compiler/eval/repr.rs`:

- `h(amount, weak: false)`;
- `v(amount, weak: false)`;
- `pagebreak(weak: false, to: auto)`;
- `colbreak(weak: false)`.

O lote inclui os quatro porque têm a mesma causa mecânica: `repr_content`
descarta os campos e devolve somente `hspace`, `vspace`, `pagebreak` ou
`colbreak`. Além disso, todos precisam distinguir `weak` omitido de
`weak: false` explicitamente fornecido.

O passo não altera layout, colapso de `weak`, distribuição de `fr`, quebra de
página/coluna nem parsing dos construtores. A paridade buscada é somente o
observável `repr` e a informação mínima necessária para produzi-lo.

## 2. Proveniência da medição inicial

Medição em `2026-08-24T11:06:45-03:00`:

- commit base: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- working tree não commitado;
- antes deste documento, `git diff HEAD --stat`: **16 ficheiros alterados, 93
  inserções e 41 remoções**;
- `git status --short | wc -l`: **17 entradas**;
- vanilla: `/usr/local/bin/typst`, baseline ratificado `a51e02804`;
- cristalino: `./target/debug/typst` construído a partir da árvore de trabalho.

Repetir e registar estas informações no fecho. Os números desta secção não são
prova do estado final.

## 3. Medição anterior à decisão

### 3.1 Espaçamento horizontal e vertical

| expressão | vanilla ratificado | cristalino atual |
|---|---|---|
| `repr(h(1pt))` | `h(amount: 1pt)` | `hspace` |
| `repr(h(1em))` | `h(amount: 1em)` | `hspace` |
| `repr(h(1fr))` | `h(amount: 1fr)` | `hspace` |
| `repr(h(0pt))` | `h(amount: 0pt)` | `hspace` |
| `repr(h(1pt, weak: true))` | `h(amount: 1pt, weak: true)` | `hspace` |
| `repr(h(1pt, weak: false))` | `h(amount: 1pt, weak: false)` | `hspace` |
| `repr(v(1pt))` | `v(amount: 1pt)` | `vspace` |
| `repr(v(1em))` | `v(amount: 1em)` | `vspace` |
| `repr(v(0pt))` | `v(amount: 0pt)` | `vspace` |
| `repr(v(1pt, weak: true))` | `v(amount: 1pt, weak: true)` | `vspace` |
| `repr(v(1pt, weak: false))` | `v(amount: 1pt, weak: false)` | `vspace` |

`amount` é posicional no vanilla e no cristalino. `h(amount: 1pt)` e
`v(amount: 1pt)` são rejeitados por ambos; alinhar texto de diagnóstico não é
parte deste lote.

### 3.2 Quebras

| expressão | vanilla ratificado | cristalino atual |
|---|---|---|
| `repr(pagebreak())` | `pagebreak()` | `pagebreak` |
| `repr(pagebreak(weak: true))` | `pagebreak(weak: true)` | `pagebreak` |
| `repr(pagebreak(weak: false))` | `pagebreak(weak: false)` | `pagebreak` |
| `repr(pagebreak(to: "odd"))` | `pagebreak(to: "odd")` | `pagebreak` |
| `repr(pagebreak(weak: false, to: "even"))` | `pagebreak(weak: false, to: "even")` | `pagebreak` |
| `repr(colbreak())` | `colbreak()` | `colbreak` |
| `repr(colbreak(weak: true))` | `colbreak(weak: true)` | `colbreak` |
| `repr(colbreak(weak: false))` | `colbreak(weak: false)` | `colbreak` |

As aspas externas impressas por `typst eval` apenas indicam que `repr` devolve
string e foram omitidas das células.

### 3.3 Classificação

- O texto devolvido por `repr` é semântica observável da linguagem.
- A forma dos structs Rust diverge legitimamente do vanilla, desde que consiga
  preservar a informação observável (ADR-0107).
- O valor final `weak == false` não permite saber se o argumento foi omitido ou
  explicitamente fornecido. O vanilla distingue esses casos no `repr`.
- Portanto, alterar somente os quatro braços de `repr_content` é insuficiente:
  produziria paridade para um caso e regressão para o outro.

## 4. Auditoria L0 e gate obrigatório

Ler e atualizar, antes do código:

- `00_nucleo/prompts/compiler/stdlib/foundations/repr.md`;
- `00_nucleo/prompts/compiler/stdlib/layout.md`;
- `00_nucleo/prompts/entities/elements/h_space.md`;
- `00_nucleo/prompts/entities/elements/v_space.md`;
- `00_nucleo/prompts/entities/elements/pagebreak.md`;
- `00_nucleo/prompts/entities/elements/colbreak.md`.

Os quatro structs expõem campos públicos. Preservar a presença explícita de
`weak` exige acrescentar ou alterar estado no contrato público. Logo:

1. redigir a decisão nos L0;
2. ressellar os hashes documentais aplicáveis;
3. **parar e obter confirmação do dono antes de escrever L1**, conforme
   ADR-0127.

O restante deste documento especifica a implementação posterior à confirmação.

## 5. Forma recomendada da preservação de presença

Preservar o campo funcional existente e acrescentar metadado explícito é a
mudança menos invasiva:

```rust
pub weak: bool,
pub weak_explicit: bool,
```

Aplicar a `HSpaceElem`, `VSpaceElem`, `PagebreakElem` e `ColbreakElem`.

Regras:

1. `weak` continua sendo o valor usado por layout e igualdade funcional.
2. `weak_explicit` participa de `repr`, `PartialEq` e `Hash`, pois dois valores
   com `repr` diferente são distinguíveis na linguagem.
3. O parser de `Args` deve conservar simultaneamente valor e presença, sem
   tentar inferir presença a partir de `false`.
4. Os construtores usados pelas nativas recebem a presença explicitamente ou
   delegam a um construtor interno que a recebe.
5. APIs ergonómicas existentes não devem mudar de significado silenciosamente.
   O L0 deve declarar se chamadas Rust existentes tratam `weak: false` como
   omitido; a recomendação é preservar o comportamento histórico e reservar a
   forma explícita para o caminho de eval.
6. Não usar mapa genérico de propriedades, `dyn`, vtable ou importação reversa;
   os elementos permanecem atomizados nos seus módulos (ADR-0109).

Se a auditoria encontrar uma forma já existente de preservar field presence na
StyleChain ou no payload sem ampliar estes structs, medi-la e preferi-la apenas
se não introduzir acoplamento reverso nem perder a informação após eval. Não
adotar a alternativa por analogia sem uma prova end-to-end de `repr`.

## 6. Formatação canônica de `repr`

Implementar free helpers privados em `compiler/eval/repr.rs`, mantendo o match
exaustivo e estático:

### 6.1 `HSpace`

- `h(amount: <spacing>)` sempre inclui `amount`;
- `Spacing::Absolute` usa o helper canônico `repr_length` já existente;
- `Spacing::Fractional(x)` usa a mesma formatação numérica canônica dos outros
  valores e sufixo `fr`;
- acrescenta `, weak: true|false` somente quando `weak_explicit` for verdadeiro.

### 6.2 `VSpace`

- `v(amount: <length>)` sempre inclui `amount` via `repr_length`;
- mesma regra de presença para `weak`.

### 6.3 `Pagebreak`

- sem campos explícitos: `pagebreak()`;
- ordem canônica: `weak` antes de `to`;
- `weak` aparece somente quando explicitamente fornecido;
- `to: Some(Even|Odd)` aparece como string Typst escapada:
  `to: "even"` ou `to: "odd"`.

### 6.4 `Colbreak`

- sem `weak` explícito: `colbreak()`;
- com presença: `colbreak(weak: true|false)`.

Não usar `Debug` de Rust para `Length`, `Spacing`, `Parity` ou booleanos.

## 7. RED → GREEN

### 7.1 Testes antes do código

Adicionar testes end-to-end de `#repr(...)` para todas as linhas das tabelas do
§3. Testes isolados de `repr_content` devem cobrir:

- `Length` em `pt`, `em` e forma combinada;
- `Spacing::Fractional`, incluindo valor decimal;
- `weak` omitido, explícito `true` e explícito `false`;
- `pagebreak` sem argumentos, somente `to`, somente `weak` e ambos;
- `Parity::Even` e `Parity::Odd`;
- `colbreak` omitido/true/false.

Confirmar RED por causa das strings atuais e, para os casos de presença,
confirmar que o RED demonstra a perda de informação — não somente pontuação.

### 7.2 Implementação

1. preservar presença no extrator comum de `weak` sem duplicar parsing nos
   quatro construtores;
2. propagar o bit somente pelos elementos afetados;
3. atualizar construtores, matches e testes estruturais de forma exaustiva;
4. implementar os quatro braços canônicos em `repr_content`;
5. manter layout e consumidores funcionais lendo `weak`, não
   `weak_explicit`;
6. ressellar todos os headers depois da atualização final dos L0.

## 8. O que pode acompanhar este lote

Pode entrar se a medição confirmar a mesma causa e sem novo contrato:

- formatação de `h` com `Length` absoluto+`em` usando o helper já existente;
- formatação de fração decimal e zero;
- escaping canônico de `to` usando helper de string já existente;
- testes relacionais de `PartialEq`/`Hash` para a presença explícita;
- atomização dos helpers de repr por família dentro de `eval/repr.rs`, se o
  match permanecer magro e estático.

Não entra:

- implementação do colapso `weak` no layout;
- `v(1fr)`;
- percentuais em `h`/`v`;
- correção geral de mensagens de argumento;
- outros stubs de `repr` sem esta mesma estrutura de campos;
- mudanças em paginação ou multi-column flow.

## 9. Achado adjacente explicitamente separado

`repr(linebreak())` produz `linebreak()` no vanilla, enquanto o cristalino
falha antes do `repr` com `unknown variable linebreak`. Isso não é outro braço
de repr: é ausência de binding/construtor público e mudança de comportamento do
produto. Deve receber passo e L0 próprios. Não o esconder neste lote apesar da
proximidade nominal com `pagebreak`/`colbreak`.

## 10. Validação

Após a confirmação ADR-0127 e implementação:

```sh
cargo test -p typst-core p1140_9
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo build --workspace
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Reexecutar também todas as sondas do §3 nos dois binários e registar comando,
saída, commit/working tree e hora.

## 11. Critérios de aceitação

O passo fecha somente quando:

- todas as formas medidas no §3 coincidem exatamente no nível observável;
- omitido e `weak: false` continuam distinguíveis depois de eval;
- `h(1fr)` preserva o valor e usa representação canônica;
- a ordem de campos de `pagebreak` coincide com o vanilla;
- layout de spacing e break não muda;
- igualdade e hash continuam coerentes com o novo estado observável;
- nenhum despacho dinâmico ou import reverso é introduzido;
- L0, headers de linhagem e hashes estão coerentes;
- suítes L1 e L3, build, fmt, lint e diff-check passam;
- o relatório final enumera achados restantes, inclusive `linebreak`, sem os
  classificar como resolvidos.

## 12. Próximo passo provável

Depois deste lote, medir e especificar o binding público `linebreak`, incluindo
`justify`, comportamento em markup (`\\`) e `repr`. Só depois retomar a
expansão maior de papéis semânticos do PDF tagueado, salvo nova prioridade do
dono.

## 13. Gate L0 executado

Em 2026-08-24 foram atualizados os seis L0 listados no §4. A decisão adotada é
o campo público aditivo `weak_explicit: bool`, com os construtores históricos
preservados e construtores com presença explícita reservados ao eval.

Hashes L0 calculados pelo linter, ainda não escritos nos headers L1:

- `stdlib/foundations/repr.md`: `8aca8bb3`;
- `entities/elements/h_space.md`: `6a12841f`;
- `entities/elements/v_space.md`: `a72fc045`;
- `entities/elements/pagebreak.md`: `2e8926a7`;
- `entities/elements/colbreak.md`: `8f4f8844`.

`stdlib/layout.rs` aponta para `stdlib/layout.md`, mas não possui
`@prompt-hash`; o L0 foi atualizado e essa ausência preexistente não deve ser
contornada com hash inventado. Nenhum ficheiro L1 foi alterado nesta fase.

Paragem obrigatória: o dono deve confirmar os L0 e os hashes antes dos testes
RED e da implementação.

## 14. Resultado da execução

Gate confirmado pelo dono. Os testes RED reproduziram os quatro stubs; a
implementação adicionou `weak_explicit` aos quatro elementos, preservou as APIs
ergonómicas existentes, propagou presença real no eval e tornou os braços de
`repr_content` canônicos. As 19 sondas do §3 e suas variantes de cobertura
ficaram idênticas ao vanilla ratificado.

Validação final: `typst-core` **5147/5147**, `typst-infra` **828/828**, build do
workspace, fmt, lint cristalino e diff-check passaram. Relatório:
`00_nucleo/diagnosticos/typst-p1140.9-repr-spacing-breaks.md`.
