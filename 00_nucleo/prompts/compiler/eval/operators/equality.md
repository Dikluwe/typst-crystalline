# Prompt L0 — `compiler/eval/operators/equality` — igualdade e pertença
Hash do Código: bdb7e0c7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`compiler/eval/operators/equality.rs:119–124` aplica morph_canon aos dois
payloads. Na referência, `foundations/content/raw.rs:354–361` compara
campos do elemento; R5 equality.different-numbering é false no vanilla e
true no baseline. Pares com labels distintas são true. Logo preservar só
morph_canon é insuficiente neste recorte (refuta essa frase da auditoria de
transporte, não a recomendação de separar o IR).

### Decisão proprietária

Preservar igualdade raw Content↔Content e o fallback LocatedContent None.
Quando pelo menos um operando tem snapshot Some, comparar função do elemento
e os conjuntos de campos linguísticos, excluindo label; para o operando
Some usar o mapa, para raw/None usar os campos explicitamente presentes da
projeção existente. Comparar valores recursivamente por values_eq (body
continua morfológico); não comparar Location, ordem de inserção, discriminante
do carrier ou Debug. Conjuntos diferentes de campos são diferentes: raw não
é automaticamente igual à versão realizada nem automaticamente desigual
apenas por usar outra variante de Value. Não preencher defaults no comparador.

Os braços Eq/Neq e pertença devem chegar à mesma regra, inclusive aninhados
em arrays/dicts. Não alterar o derive PartialEq de Value, Content::morph_canon
ou estilos do IR para obter o resultado. Helper de projeção de campos pode
ser interno ao eval e compartilhado estaticamente com field_access, sem
import reverso de entities para compiler; não nova API pública de Value.

Aceitação: numbering1 vs I false; labels distintas com mesmos campos true;
clone true; inline-default vs query false; Location distinta não basta para
desigualdade. Coerção Int/Float e igualdade das demais famílias não regridem.

---


**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/equality.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0025 (dois sistemas de igualdade), ADR-0107 (paridade língua — igualdade morfológica de content)

---

## Contexto

O `==` da **linguagem** Typst não é o `PartialEq` do Rust (ADR-0025): coage
`Int ↔ Float` (medido: `1 == 1.0` → `true`), compara `Content`
**morfologicamente** (texto/markup/estilo semântico, ignorando estilo de
render — via `Content::morph_canon`), e propaga a coerção a elementos
aninhados de arrays/dicts. O `derive(PartialEq)` de `Value` fica para o Rust
(IndexMap, testes, estruturas de dados).

## Instrução

### Braços dedicados de `Eq`/`Neq` (antes do braço genérico)

- `Int ↔ Float` cruzado: coerção para `f64` e comparação.
- `Content == Content`: `a.morph_canon() == b.morph_canon()` — a forma
  canónica é comparada com o `==` estrutural; o estilo de render assado é
  ignorado.
- `Version == Version`: comparação directa sobre todos os componentes
  (zero-pad; não existem `pre`/`build` em Typst).
- `Ratio ↔ Relative`: igualdade quando a parte absoluta do `Relative` é zero
  e `|rel − ratio| < 1e-9`.
- Braço genérico: `(Eq, a, b) → values_eq(&a, &b)`; `Neq` é a negação.

### `values_eq` — igualdade da linguagem, recursiva

Paridade com `Value::eq` do vanilla (que **é** `ops::equal`,
`foundations/value.rs:295-299`):

- `Int ↔ Float`: coerção, em qualquer profundidade (medido:
  `(1,2) == (1.0,2.0)` → `true`; `(a: 1) == (a: 1.0)` → `true`).
- `Array`/`Dict`: elemento a elemento com `values_eq` (mesmo comprimento).
- `Length ↔ Relative`: igual quando o `Relative` tem parte relativa zero
  (`ops.rs:458-460`; medido: `10pt == (10pt + 0%)` → `true`).
- `Ratio ↔ Relative`: parte absoluta zero + tolerância `1e-9`.
- `Content`: morfológico (`morph_canon`), também em posição aninhada.
- Resto: delega no `PartialEq` derivado.

### `value_eq` e a pertença `in` / `not in`

`value_eq` delega em `values_eq` e serve os braços de pertença (medido:
`1 in (1.0, 2.0)` → `true`; `(1,) in ((1.0,), (2,))` → `true`).

| `lhs in rhs` | Resultado | Medição vanilla |
|---|---|---|
| `Str in Dict` | `Bool` — a chave existe (`contains_key`) | `"a" in (a:1,b:2)` → `true` |
| `Str in Str` | `Bool` — substring | `"ell" in "hello"` → `true` |
| `any in Array` | `Bool` — `values_eq` elemento a elemento | `(1,2) in ((1,2),(3,4))` → `true` |
| `not in` | negação lógica das combinações acima | `1 not in (1,2,3)` → `false` |
| combinação sem braço (ex.: `Int in Str`) | **erro** de fronteira | vanilla: `"cannot apply 'in' to integer and string"` |

O texto incompatível usa o spelling público entre aspas simples:
`cannot apply 'in' ...` ou `cannot apply 'not in' ...`, conforme o nó
`error_formatting.md`. Nomes Rust de variantes nunca escapam ao diagnóstico.

## Restrições Estruturais

- L1 puro (ver hub); nenhum braço toca `EvalContext`/`Scope`.
- Não tocar o `derive(PartialEq)` de `Value` — os dois sistemas coexistem
  por decisão (ADR-0025).

## Critérios de Verificação

```
eval_binary_op(Eq, Int(1), Float(1.0))          == Bool(true)
eval_binary_op(Neq, Int(1), Float(1.0))         == Bool(false)
eval_binary_op(Eq, Content(it.body), Content([a]))  // casa morfologicamente
eval_binary_op(Eq, Ratio(50%), Relative(0pt + 50%)) == Bool(true)
eval_binary_op(Eq, Length(10pt), Relative(10pt + 0%)) == Bool(true)

// coerção recursiva
eval_binary_op(Eq, Array[1,2], Array[1.0,2.0])  == Bool(true)
eval_binary_op(Eq, Dict{a:1}, Dict{a:1.0})      == Bool(true)

// pertença
eval_binary_op(In, Str("a"), Dict{a:1,b:2})     == Bool(true)
eval_binary_op(In, Str("z"), Dict{a:1,b:2})     == Bool(false)
eval_binary_op(In, Str("ell"), Str("hello"))    == Bool(true)
eval_binary_op(In, Array[1,2], Array[Array[1,2],Array[3,4]]) == Bool(true)
eval_binary_op(In, Int(1), Array[Float(1.0)])   == Bool(true)
eval_binary_op(NotIn, Int(5), Array[1,2,3])     == Bool(true)
eval_binary_op(In, Int(1), Str("hello"))        == Err("cannot apply 'in' to integer and string")
eval_binary_op(NotIn, Int(1), Str("hello"))     == Err("cannot apply 'not in' to integer and string")
```

## Resultado Esperado

- Braços `Eq`/`Neq`/`In`/`NotIn` e os helpers `values_eq`/`value_eq`
  conforme especificado; testes unitários no ficheiro e E2E no eval.

## P1339 — igualdade pública do seletor de elemento

### Medição anterior à decisão

No HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, sem diff produtivo,
`equality.rs:101-130` deixa Value::Selector no fallback estrutural. A sonda
`diagnosticos/p1339-where-l0-vanilla.json`, UTC
`2026-09-10T00:04:21.279738+00:00`–`00:04:21.402736+00:00`, mede
`heading.where(level:1) == heading.where(level:1.0)` verdadeiro no vanilla;
o antecedente cristalino mede falso. Reordenar campos é diferente; alias
é igual; campo NaN torna `s == s` falso; filtro vazio é distinto do elemento
nu. A fonte `foundations/selector.rs:75-81` usa sequência de campos, cujos
valores têm igualdade de linguagem; não se exige copiar o derive Rust.

### Decisão proposta

Adicionar tratamento de linguagem para Selector::Element no caminho de
`values_eq`. Comparar a função canônica com a igualdade Func vigente, e
comparar a sequência de campos por nomes na mesma posição e valores por
`values_eq` recursiva. Reconhecimento de um receiver como elemento é tarefa
do produtor, nunca consequência dessa comparação por Func. Preservar ordem:
não converter o grupo em comparação de conjuntos de Dict.

A regra deve propagar através de arrays, dicts e composição de seletores
que contenha a variante nova. Pares de variantes anteriores sem Element
preservam comportamento antecedente; não usar esta mudança para corrigir
outros seletores. Element vazio não é Kind; ausência de campos filtrados não
equivale a ausência do grupo. Eq, Neq e pertença seguem a mesma regra.

Não modificar PartialEq/Hash de Selector, Value, Func ou Content; os hashes
estruturais não são usados como oráculo de igualdade da linguagem. A regra
de comparação de valores necessária ao matcher pode ser reutilizada por
helper interno tipado, sem nova API pública nem import reverso de entities.
Snapshots de conteúdo continuam seguindo o Núcleo já pinado neste owner.

Aceitação: casos focais de coerção, ordem, alias, grupo vazio e NaN; mesmos
casos aninhados; preservação das famílias antigas. Inferência de suficiência
é refutada por valor de campo cuja igualdade não atravesse o helper comum,
ou por uma regressão em pares antigos sem a variante nova.

### P1339 — identidade linguística da chave de counter filtrado

Medição anterior à decisão: no HEAD acima, `values_eq` também deixa
Value::Counter no fallback derivado. O recibo
`diagnosticos/p1339-where-counter-runtime-probe-runs.json`, SHA-256
`178a8637df7eb75d21131a46ad68005b9f7e35670644c9cb9b7eeb93031d5ae3`,
registra UTC e árvore integrais: filtros level inteiro/float comparam iguais
e compartilham update; inverter ordem dos fields distingue chaves e updates;
filtro com NaN não compara igual a si próprio e não reencontra seu update.
`introspection/counter.rs:243,256` seleciona updates pela chave, separadamente
das ocorrências casadas pelo filtro. Trata-se de identidade observável da
linguagem, não de igualdade de HashMap nem de intenção inferida sobre NaN.

Para Value::Counter cuja chave contenha Selector::Element, comparar a chave
inteira usando a regra recursiva do seletor acima; delegar pares sem Element
ao comportamento anterior. Eq/Neq/pertença, inclusive aninhados, usam essa
mesma regra. O runtime pode reutilizar helper interno de comparação de
CounterKey para associar eventos manuais. Não usar Arc identity como atalho
reflexivo nem bucket de hash estrutural para excluir pares int/float iguais.
Não mudar Eq/Hash públicos de Counter, CounterKey, Value ou Selector e não
normalizar a ordem dos fields. A mudança é interna de paridade, subordinada
aos gates P1339, sem nova entidade ou assinatura pública.
