# Paridade Produção — P705 — `luma()` aceita percentagem; sondagem `Int | Ratio`

**Data:** 2026-07-11
**Passo:** sem materialização prévia (`typst-passo-705.md` não existia — instruído directamente pelo utilizador na sessão, seguindo o mesmo protocolo dos passos anteriores).
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `b01565e3d` (fim de P704, detached HEAD).
**Estado:** ÂMBITO FECHADO — `luma()` aceita percentagem, com **paridade exacta** (incluindo o fallback silencioso do vanilla), corrigido em duas iterações (achado de um bug próprio a meio, corrigido antes de fechar). `cetz` mantém o progresso, novo bloqueio isolado (`in` para `Str`/`Dict`).

---

## 1. Sondagem pedida — generalizar `Int | Ratio` como tipo de componente

O utilizador pediu explicitamente para medir se valia a pena generalizar um
tipo de componente partilhado (`Int | Ratio`) entre construtores de cor,
antes de decidir o âmbito de P705.

**Medição**: `grep -rn` exaustivo a
`~/.cache/typst/packages/preview/cetz/0.5.2/src/` para `oklab`, `oklch`,
`hsl(`, `hsv(`, `cmyk(`, `linear-rgb`/`linear_rgb`, e `rgb(` com argumentos
percentuais.

**Resultado**: **nenhum** consumidor real usa qualquer uma destas formas.
O único uso é `luma(v * 1%)` em `palette.typ:107`. **Decisão**: não
generalizar — implementar o braço de percentagem **só** em `luma()`,
seguindo a mesma disciplina de P703 (que já tinha decidido não generalizar
`Ratio` para `rgb()` pela mesma razão). Confirma que a hipótese do
utilizador ("resolver vários gaps de uma vez") **não se verificou** — é
sempre 1 função, 1 gap, medido isoladamente.

---

## 2. Achado corrigido a meio da implementação — causa raiz real

**Primeira tentativa** (errada): implementei um braço `Value::Ratio(r)`,
assumindo que `50%`/`v * 1%` produziam `Value::Ratio`. Testei
manualmente e **todos os valores válidos vinham brancos** — `luma(50%)`,
`luma(90%)` davam `luma(100%)` em vez do valor certo.

**Diagnóstico**: `#type(50%)` → `length`, não `ratio`. Uma percentagem
simples neste cristalino avalia para **`Value::Relative(Rel<Length>)`**
(`rel: f64` + `abs: Length`), **unificado com `length`** (P685: `type(50%
+ 1pt) == length`). `Value::Ratio` existe mas só é produzido por outros
caminhos (ex. `Ratio * Int` em `operators.rs:215-218`); `v * 1%` (`Int *
Relative`) usa o braço `operators.rs:242-244`, que produz
`Value::Relative`, não `Value::Ratio`.

**Correcção**: o braço aceita `Value::Relative(rel)` quando `rel.abs ==
Length::ZERO` (sem parte absoluta) e `rel.rel` em `[0.0, 1.0]`. Revalidado
manualmente: todos os 6 casos de `luma(0/255/50%/90%/0%/100%)` batem
agora exactamente com o vanilla, e `luma(90 * 1%)`/`luma(42 * 1%)`
(reprodução literal do padrão de `cetz`) dão `0.9`/`0.42` correctos.

Isto é exactamente o tipo de erro que compilar um documento `.typ` real
apanha e um teste unitário com valores construídos à mão não apanharia —
os meus primeiros testes unitários usavam `Value::Ratio` directamente
(por construção), não o valor que o eval realmente produz para `50%`.
Adicionei um teste específico que constrói `Value::Relative` para não
repetir este erro.

---

## 3. Achado lateral — fallback silencioso do vanilla, decisão revista

Medido directamente (release build do vanilla): `luma("bad")`,
`luma(300)`, `luma(150%)` e `luma()` (zero argumentos) devolvem todos
`luma(100%)` (branco), **sem erro**. Mecanismo:
`args.expect(...).unwrap_or(Component(Ratio::one()))` — qualquer falha de
cast (tipo errado, valor fora de gama) ou ausência do argumento é
**engolida silenciosamente**.

**Decisão inicial** (rascunho do L0, antes de o utilizador perguntar):
não replicar — manter erro no cristalino, por "filosofia de falhar alto".

**Decisão revista** (depois de o utilizador perguntar explicitamente "por
que existe o erro, [há] solução de devolver a saída correta?"): **ADR-0107
diz que a paridade é com a língua observável, não com a preferência
própria de estilo de erro.** `luma(300) → branco` é comportamento da
língua, medido, reproduzível — recusar replicá-lo era substituir a decisão
do projecto pela minha. Corrigido: o cristalino agora replica o fallback
exactamente, incluindo a correcção retroactiva de `luma(256)` (que antes
de P705 erguia erro, uma divergência pré-existente não documentada).

---

## 4. Implementação final

### `native_luma` (`01_core/src/rules/stdlib/foundations.rs`)

- `[]` → branco.
- `[v]` → `component_to_ratio(v).unwrap_or(1.0)`, onde
  `component_to_ratio`:
  - `Int` em `[0,255]` → `/255.0`.
  - `Ratio` em `[0.0,1.0]` → directo (caminho alternativo, não exercitado
    por nenhum consumidor real, mantido por robustez).
  - `Relative` com `abs == Length::ZERO` e `rel` em `[0.0,1.0]` → `rel`
    directo — **este é o caminho real usado por `cetz`**.
  - Qualquer outro valor → `None` → fallback branco.
- `_` (2+ argumentos) → erro estrutural (`alpha` não suportado — âmbito
  de decisão de P703/P705, `Color::Luma` do cristalino não tem esse
  campo; não é o fallback silencioso, é limitação estrutural distinta).

### Testes (9 novos, `foundations.rs::tests_p705_luma_ratio`)

`Ratio` directo, `Int` sem regressão, `Int`/`Ratio` fora de gama → branco,
tipo errado → branco, zero argumentos → branco, 2+ argumentos → erro,
**percentagem via `Value::Relative`** (reprodução exacta de `cetz`),
percentagem com parte absoluta (`50% + 1pt`) → fallback (não é componente
válido).

---

## 5. Ficheiros tocados

- **L0**: `00_nucleo/prompts/rules/stdlib/foundations.md` — secção
  `native_luma` reescrita: algoritmo, causa raiz (`Value::Relative`, não
  `Ratio`), fallback silencioso replicado (com a mudança de decisão
  registada), scope-out (`luma(color)`, `alpha`).
- **Código**: `01_core/src/rules/stdlib/foundations.rs` (`native_luma` +
  9 testes).

---

## 6. Validação — âmbito de P705 confirmado

Reexecutados todos os casos medidos (release build):

- `luma(0/255/50%/90%/0%/100%)` → `0.0/1.0/0.5/0.9/0.0/1.0` — **idêntico
  ao vanilla**.
- `luma(90 * 1%)` / `luma(42 * 1%)` (padrão real de `cetz`) → `0.9`/`0.42`
  — correcto.
- `luma("bad")`, `luma(300)`, `luma(150%)`, `luma()` → todos branco
  (`1.0`) — **idêntico ao vanilla** (fallback silencioso replicado).
- `luma(0, 1, 2)` → erro (estrutural, `alpha` não suportado).

- `cargo test --workspace` → **3772 passed**, 0 failed (3763 de P704 + 9
  novos de P705); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (hash de `foundations.rs`
  realinhado com `--fix-hashes`).

---

## 7. Repetição da reprodução de P700-704 — mesmo progresso, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~7.3s** — mesma ordem de grandeza dos passos
anteriores.

**Novo bloqueio**: `error: cannot apply In to str and dictionary`. O
operador `in` (`"chave" in dict`, teste de pertença de chave num
dicionário) não está implementado para `Str`/`Dict` no cristalino.
Confirmado standalone:
```
#("a" in (a: 1, b: 2))
```
→ vanilla: `true`, exit 0. Cristalino: `error: cannot apply In to str and
dictionary`.

Não isolei ainda a linha exacta de `cetz` que usa isto (fora do âmbito de
P705); fica para a sonda do próximo passo.

### Próximo passo sugerido (P706, não iniciado)

1. Isolar onde em `cetz` o operador `in` é usado sobre um dict (`grep -rn
   " in " nos ficheiros do pacote, filtrando falsos positivos de "for x in").
2. Confirmar a semântica completa do vanilla (`in` para `Dict` testa
   chaves; para `Array` testa elementos; para `Str` testa substring —
   confirmar quais combinações faltam no cristalino, não assumir que é só
   `Str in Dict`).
3. L0 para o operador `in` (provavelmente `rules/eval/operators.md` ou
   secção nova em `rules/eval.md`).

---

## 8. Estado da cadeia P678–705

Progresso cumulativo: plugin WASM real (P699-700), `cbor.encode` (P701),
`.with()` (P702), `rgb(hex)` (P703), `range(step:)` (P704), e agora
`luma()` com percentagem e paridade exacta do fallback silencioso (P705)
— todos fechados e testados. `cetz` ainda não renderiza; o bloqueio mudou
de área (de `palette.typ`/cores para um operador de linguagem geral, `in`)
— não está mais confinado à mesma função dos 3 passos anteriores. Pausada
aqui, com P706 sugerido.

## 9. Critério de fecho do passo

- [x] Sondagem `Int | Ratio` pedida pelo utilizador — medida, decisão
      documentada (não generalizar, sem consumidor).
- [x] Causa raiz real de `luma(v * 1%)` isolada e corrigida
      (`Value::Relative`, não `Value::Ratio`) — erro próprio encontrado e
      corrigido antes do fecho, não silenciado.
- [x] Fallback silencioso do vanilla replicado, com a decisão revista
      registada (por que mudei de "erro" para "paridade exacta").
- [x] Implementado e testado (9 testes).
- [x] Sem regressão em `cargo test --workspace` (3772 passed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — próximo bloqueio identificado (`in` Str/Dict).
- [x] Relatório com resultado exacto (este ficheiro).
