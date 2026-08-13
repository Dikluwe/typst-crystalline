# Passo 1032 — Fatiamento hub/nó `stdlib::foundations`

**Tipo**: Aplicação do método consolidado (`00_nucleo/prompts/auditar-fatiamento.md`) a
`compiler::stdlib::foundations` — maior hub stdlib por fatiar (~89KB, "monolítico", per
achado lateral do Passo 1004). Resolve também um débito em aberto: o Passo 1022 registou
que o nó `regex` (em `stdlib/text/`) está no domínio errado — o tipo real vive junto de
`foundations/str.rs` no vanilla — e nomeou "o passo que fatiar `foundations`" como dono
dessa decisão.
**Pode correr em paralelo com os Passos 1030/1031** — ficheiro e prompts distintos, sem
sobreposição.
**Pré-condição**: `git status` limpo.

---

## Ler o método primeiro

`00_nucleo/prompts/auditar-fatiamento.md` — ordem obrigatória: critério-zero → inventário
genérico de visibilidade → critério 3 como decisor → critério 4 como hipótese → critério
2 em 3 classes → verificar órfãos → materializar → validar → avaliar. Incorporar também a
regra mais recente: **par de co-mudança sem mecanismo plausível é para verificar, não
para explicar** (achado de segunda ordem, Passo 1023).

## Contexto específico

`stdlib/foundations.rs` foi caracterizado no Passo 1004 (por amostra, **não** inventário
completo — lição repetida em `structural`/`bindings`, não presumir que a amostra aqui é
diferente) como: `type`, `len`, `repr`, constructors de cores, e — **misto/imperativo** —
`state`/`counter`/`query`/`here`/`locate`. O P1004 já notou que, no vanilla, isto está
dividido entre `typst_library::foundations` e `typst_library::introspection`.

**Ligação ao Passo 1018**: `CounterKey`/`CounterRegistry` foram alterados nesse passo.
Confirmar se `native_counter_at`/`native_counter_final` (citados no relatório do P1018
como estando em `stdlib/foundations.rs`) continuam lá ou já se moveram — o inventário
deste passo pode encontrar o ficheiro diferente do que o P1004 descreveu.

## Fase A — Inventário completo

```
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' 01_core/src/compiler/stdlib/foundations.rs
```
Contar também `enum`/`struct` de topo relevantes, mesma disciplina do P1013/P1014/P1022.

## Fase B — Os 4 critérios, com evidência

1. **Critério-zero** — confirmar sem `trait`.
2. **Critério 2** (3 classes, medido por `file:line`, não por assinatura) — hipótese a
   confirmar: `type`/`len`/`repr`/constructors de cor são declarativos; `state`/`counter`/
   `query`/`here`/`locate` são stateful (tocam `Engine`/introspector). Confirmar caso a
   caso — o P1013 já mostrou que a assinatura sozinha engana.
3. **Critério 3** (co-mudança) — usar `tools/analysis/cochange_metrics.py` (versão
   corrigida, pós P1022/P1023).
4. **Critério 4** (vanilla) — `typst_library::foundations` vs `typst_library::
   introspection` como candidato de fronteira principal; confirmar com critério 3 antes
   de aceitar. Localizar também onde `str.rs`/`Regex` vive exactamente no vanilla, para
   decidir se o nó `regex` de `stdlib/text/` deve mover-se para aqui.

## Fase C — Resolver o débito do nó `regex`

Com a fronteira de `foundations` decidida: absorver `stdlib/text/regex.rs`/`.md` para
dentro da estrutura nova, se o critério 3/4 sustentar essa colocação — ou declarar
explicitamente por que não (mesma disciplina exigida no Passo 1022 para este item).

## Fase D — Verificar órfãos e excepções mortas antes de materializar

Confirmar se algum prompt órfão remanescente (do Passo 1001 ou posterior) cobre pedaços
de `foundations.rs`. Verificar `crystalline.toml` por excepções apontando para ficheiros
já removidos por fatiamentos anteriores (mesma verificação que apanhou o resíduo de
`decimal-arithmetic.md` no Passo 1014).

## Fase E — Materializar, validar, avaliar

```
crystalline-lint .
cargo test --workspace
```
Prova item a item (corte e cola, não reescrita) — mesma disciplina de sempre. Actualizar
`auditar-fatiamento.md` com esta aplicação (que critério decidiu, se algum foi vácuo, se
alguma hipótese inicial foi corrigida).

---

## Resultado esperado

`stdlib::foundations` fatiado com evidência dos 4 critérios. Débito do nó `regex`
resolvido (absorvido ou declarado por que não). Nenhuma referência a passo nos L0s novos.

---

## Relatório

O relatório completo de execução — inventário, medição dos 4 critérios, métricas,
hipóteses corrigidas e validação — está em:

`00_nucleo/diagnosticos/typst-passo-1032-relatorio.md`

## Validação rápida

```text
cargo build -p typst-core     -> ok (28 warnings pré-existentes, 0 erros)
cargo test --workspace        -> ok (~5855 passed, 0 failed)
crystalline-lint .            -> 0 violations (3 warnings V7 pré-existentes)
crystalline-lint --fix-hashes . -> 0 drift warnings
```
