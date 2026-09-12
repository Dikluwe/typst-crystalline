# Prompt L0 — `compiler/eval/operators/join` — combinação sequencial de valores
Hash do Código: bc05bdad

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/join.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0107 (paridade língua)

---

## Contexto

`join` é a combinação dos valores produzidos pelas expressões de um code
block e dos corpos de `for`/`while` (paridade `ops::join`,
`foundations/ops.rs:24-45`; consumidores: `typst-eval/code.rs:57`,
`typst-eval/flow.rs:86,132`). No cristalino a **acumulação** vive nos
consumidores (`eval/mod.rs` para `Expr::CodeBlock`, `control_flow.rs` para
os loops — L0 `compiler/eval.md`); este nó possui só a **tabela de
combinação** e o seu erro.

## Instrução

### `join(lhs, rhs)` — tabela

| Combinação | Resultado |
|---|---|
| `(a, None)` / `(None, b)` | `a` / `b` — `None` é identidade nos dois lados |
| `Str + Str` | concatenação |
| `Symbol + Symbol`, `Str ↔ Symbol` | `Str` (grapheme clusters integrais concatenados) |
| `Bytes + Bytes` | concatenação de bytes |
| `Content + Content`, `Content ↔ Str`, `Content ↔ Symbol` | `Content` sequência (`Content::sequence`/`Content::text`) |
| `Array + Array` | concatenação (ordem preservada) |
| `Dict + Dict` | merge — direita vence, posição da primeira ocorrência preservada |
| `Args + Args` | merge de `items` e `named` |
| qualquer outra | **erro** `"cannot join {a} with {b}"` |

Medições vanilla: `{ "a"; "b" }` → `"ab"`; `{ none; (1,) }` → `(1,)`;
`{ (:); (a: 1) }` → `(a: 1)`; `{ 1; none }` → `1`; `{ 1; 2 }` → erro
"cannot join integer with integer".

### Nomes longos no erro de join — `long_type_name`, **não é deste nó**

O erro usa os nomes **longos** de tipo (paridade `Type::long_name` via
`mismatch!`; medido: `(1, 2).join("-")` no vanilla → "cannot join integer
with string"). Difere do nome curto só em três casos: `int → integer`,
`str → string`, `bool → boolean`; o resto delega em `type_name()`.

**P1015** — este nó **não possui** `long_type_name`. A redacção anterior
descrevia-a como se fosse a única implementação, quando existia uma cópia
privada aqui e outra em `stdlib/foundations.rs`, ambas duplicando a
`pub(crate)` de `compiler/eval/bindings/access.md`. As cópias foram
removidas; `join` importa
`crate::compiler::eval::long_type_name`. O dono da função é
**`compiler/eval/bindings/access.md`**.

## Restrições Estruturais

- L1 puro (ver hub).
- `join` não avalia — só combina `Value`s já avaliados. A ordem de
  avaliação e a acumulação são responsabilidade dos consumidores.

## Critérios de Verificação

```
join(Str("a"), Str("b"))                  == Str("ab")
join(Int(1), Int(2))                      == Err("cannot join integer with integer")
join(Array[1], Array[2])                  == Array[1, 2]
join(Dict{a:1}, Dict{b:2})                == Dict{a:1, b:2}
join(Args(..), Args(..))                  == merge de items e named
join(Content([a]), Content([b]))          == Content([a b])
join(none, x) / join(x, none)             == x

// via eval (consumidores)
eval("#let x = { (1,); (2,) }; #repr(x)") == "(1, 2)"
eval("#let x = { \"a\"; \"b\" }; #repr(x)") == "\"ab\""
eval("#let x = { 1; 2 }")                 == Err (cannot join)
eval("#let x = for i in (1,) { (1,); (2,) } #repr(x)") == "(1, 2)"
eval("#let x = for i in (1,) { 1; 2 }")   == Err (cannot join)
eval("#for i in (1, 2) [x]")              == Content("xx")  // sem regressão
join(Str("a"), Symbol("♥️"))              == Str("a♥️")
join(Content([a]), Symbol("👩‍💻"))         == Content("a👩‍💻")
```

## Resultado Esperado

- `join` e `long_type_name` conforme a tabela; testes unitários no ficheiro
  e E2E nos consumidores; zero regressão na suite do eval.

## P1307-R3 — Args+Args não é a fusão With

### Medição anterior à decisão

No baseline HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais P1306,
`01_core/src/compiler/eval/operators/join.rs:73-76` funde views e mantém
o span esquerdo. A fonte ratificada
`lab/typst-original/crates/typst-library/src/foundations/args.rs:468-482`
remove TODAS as ocorrências named esquerdas cujo nome aparece à direita,
concatena os sobreviventes com RHS e define span agregado detached.
Já With em `foundations/func.rs:372-374` concatena tudo, sem essa remoção.
Essa leitura refina explicitamente a hipótese simples de concatenação da
auditoria R2; não se atribui intenção histórica ao comportamento.

### Decisão

Somente o braço Args+Args passa a operar as sequências de `entities/args.md`:
obter cada `occurrence_sequence`, retirar da esquerda named com nome presente
em qualquer ocorrência direita, conservar os demais em ordem e acrescentar
a sequência direita inteira. Reconstruir com
`Args::from_occurrences(Span::detached(), sequence)`. Os spans individuais
das ocorrências sobreviventes continuam intactos, inclusive entre Sources.
A presença de fragmento sintético None não apaga âncoras do outro lado.

A view named resultante é regenerada, portanto em colisões sua posição pode
mudar em relação ao IndexMap::extend anterior. Span agregado detached e ordem
são mudanças explicitamente submetidas ao gate; igualdade legada pode ser
afetada pelo span agregado resultante, sem mudança de PartialEq neste nó.
Não usar este algoritmo em With nem concatenação With aqui.

As outras linhas da tabela de join, inclusive Dict+Dict e None-identidade,
não mudam. O owner não avalia argumentos ou callbacks, não valida encoder e
não consulta World. Não se impõe representação Rust idêntica ao vanilla.

Aceitação futura: RHS nome repetido elimina todas as ocorrências correspondentes
LHS; named sem colisão e posicionais sobrevivem em ordem; spans sobreviventes
resolvem nas Sources originais; Args+None continua identidade sem destacar
span. Comparar controle pareado With que DEVE conservar o named antigo
inválido contra join que DEVE removê-lo. A fonte fundamenta a proposta;
medição binária adicional dos casos de join é obrigação, não resultado já PASS.

### P1307-R4 — distinguir helper de join do operador público

Medição focal `args.join-duplicates` encerrada em
`2026-09-07T17:37:05.017565+00:00`, baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
conservada em `00_nucleo/diagnosticos/p1307-r4-contract-refinement.json`, mostra
que o operador + ainda rejeita Args+Args no baseline. A fonte
`01_core/src/compiler/eval/operators/mod.rs:40-41` encaminha Add para
arithmetic; a presença do braço neste helper não prova disponibilidade da
rota da linguagem. O owner `compiler/eval/operators/arithmetic.md` deve
delegar somente o par Add/Args/Args a este helper. A regra causal permanece
única aqui; não duplicar a fusão no dispatcher. None-identidade continua
regra de join; novos pares de Add dependem do owner arithmetic. P1308 autoriza
ali Args/None e None/Args como identidade direta, sem alterar este helper.
Sem mudança de API Rust.
