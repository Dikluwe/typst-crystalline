# Prompt L0 — `compiler/eval/operators` — hub dos operadores do eval
Hash do Código: 39f8eec5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/mod.rs` (dispatcher)
**Papel**: hub — só tabela de despacho e invariantes; zero lógica própria.
**ADRs**: ADR-0025 (dois sistemas de igualdade), ADR-0104 (atomicidade para agentes), ADR-0107 (paridade língua, não mecânica), ADR-0108 (medir antes de decidir)

---

## Contexto

`operators/` implementa o dispatcher de operadores do eval Typst
(`eval_binary_op`, `eval_unary_op`) e os helpers puros que o servem. A
especificação e o código estão fatiados em cinco nós declarativos; este hub
é apenas o mapa. A divisão é por tópico semântico da linguagem, não por
função — medido no histórico git: cada alteração ao antigo ficheiro único
tocava só os braços do seu tópico, logo o tópico é a unidade de co-mudança
real. A trava V15 do linter (um ficheiro, um prompt) obriga a que cada nó
seja também um ficheiro `.rs` próprio.

## Invariantes do módulo inteiro

1. **Pureza total**: nenhuma das 13 funções recebe ou toca `EvalContext`,
   `Scope`, ou qualquer estado fora dos parâmetros (confirmado por leitura
   integral do código antes do fatiamento). Toda a semântica é
   `Value → Value`.
2. **Ordem dos braços é semântica**: dentro de cada nó, os braços
   específicos vêm antes dos genéricos (`Eq`/`Neq` dedicados antes do braço
   `values_eq`; os braços específicos de `<`/`<=`/`>`/`>=` antes do braço
   combinado `value_cmp`), e a fronteira é sempre o último braço. O
   dispatcher de `mod.rs` divide por variante de `BinOp` (disjuntas), logo a
   ordem entre grupos é irrelevante; a ordem dentro de cada grupo é
   preservada da versão monolítica.
3. **Gate de divisão por zero pré-match**: verificado antes do `match` de
   `arithmetic.rs` para `Int`/`Float`/`Decimal`/`Length`/`Relative`/`Ratio`/
   `Angle` (paridade com o `is_zero()` genérico do vanilla,
   `foundations/ops.rs:344-359`).

## Tabela de despacho (tópico → nó)

| Tópico | Nó (prompt) | Ficheiro | Funções |
|--------|-------------|----------|---------|
| Despacho por variante de `BinOp` | este hub | `operators/mod.rs` | `eval_binary_op` (shell); re-exports |
| Aritmética (`+` `-` `*` `/`), unários, `and`/`or`, gate de div-zero | `operators/arithmetic.md` | `operators/arithmetic.rs` | `apply_binary` (braços `Add`/`Sub`/`Mul`/`Div`/`And`/`Or`); `eval_unary_op`; `sanitize_length_nan` |
| Igualdade (`==` `!=`) e pertença (`in`/`not in`) | `operators/equality.md` | `operators/equality.rs` | `apply_binary` (braços `Eq`/`Neq`/`In`/`NotIn`); `values_eq`; `value_eq` |
| Ordenação (`<` `<=` `>` `>=`) | `operators/ordering.md` | `operators/ordering.rs` | `apply_binary` (braços específicos + combinado); `value_cmp`; `cmp_arrays`; `length_partial_cmp`; `rel_partial_cmp` |
| Mensagens de fronteira | `operators/error_formatting.md` | `operators/error_formatting.rs` | `binary_mismatch`; `vanilla_type_name` |
| Combinação sequencial de valores (`join`) | `operators/join.md` | `operators/join.rs` | `join`; `long_type_name` |

## Consumidores fora deste ficheiro

- **Short-circuit de `and`/`or`**: implementado no dispatcher central
  (`eval/mod.rs`, braço dedicado antes do dispatch genérico de
  `Expr::Binary`; L0: `compiler/eval.md`). Os braços `And`/`Or` de
  `eval_binary_op` só recebem `Bool` (paridade vanilla
  `typst-eval/src/ops.rs:52-66`).
- **`join` em code block e entre iterações de `for`/`while`**: a acumulação
  vive em `eval/mod.rs` (`Expr::CodeBlock`) e `control_flow.rs`; a tabela de
  combinação é deste ficheiro (nó `operators/join.md`).
- **Literal percentual**: o mapeamento `Unit::Percent → Value::Ratio` vive
  em `eval/mod.rs` (L0: `compiler/eval.md`).

## Divergência deliberada da forma vanilla

O vanilla mistura toda esta semântica num único `foundations/ops.rs`
(152 linhas no crate eval, mais as impls por tipo na library). O cristalino
fatiado por tópico **de propósito**:: o módulo cresce por feature (1148
linhas à data do fatiamento) e o custo de manutenção por IA é proporcional
ao que cada sessão precisa de ler (ADR-0104). A paridade é com a linguagem
(semântica dos operadores), nunca com a organização do ficheiro fonte do
vanilla (ADR-0107).

## Critérios de Verificação

- Cada nó tem os seus próprios critérios; este hub só exige que a tabela
  acima cubra as 13 funções do módulo (sem sobras nem faltas) e que
  `crystalline-lint .` reporte zero violations para a linhagem dos 6 prompts
  (cada ficheiro `.rs` cita exactamente um — V15).

## Resultado Esperado

- `operators/mod.rs` com o dispatcher `eval_binary_op` (shell que delega por
  variante de `BinOp`) e os re-exports (`eval_unary_op`, `join`), citando
  este hub.
- Um ficheiro por nó (`arithmetic.rs`, `equality.rs`, `ordering.rs`,
  `error_formatting.rs`, `join.rs`), cada um citando o seu prompt.
- Comportamento observável idêntico ao do ficheiro monolítico anterior
  (suite de testes inalterada).
