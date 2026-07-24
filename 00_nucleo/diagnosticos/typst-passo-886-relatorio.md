# Relatório — typst-passo-886: `#context` com valor de retorno não convertido (Fase A)

**Data:** 2026-07-24T02:57:31Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `9283b31d0f8b500060d07ff82ef51064b43ed99e` (HEAD do ramo `Tekt`)
**Working tree no início deste passo:** limpa, exceto os dois `materialization/typst-passo-88{6,7}.md`
não commitados. **Este relatório**: só editou `00_nucleo/prompts/engine/stdlib/state.md` (L0) — ver
secção 5.

---

## 0. Pré-condição de árvore (exigida no passo)

A working tree não limpa (31 ficheiros, `+1106/-405`) medida no relatório de P885 já **não existe**
no início deste passo — `git log` confirma que esse trabalho foi commitado no commit `9283b31d0`
("chore: relatórios P878 a P885..."), entre o relatório de P885 e o início deste passo. A decisão
(commitar vs. reverter) já estava tomada por terceiros quando este passo começou; não coube a este
passo decidir. Registo conforme exigido, sem outra acção necessária.

---

## 1. Fase A — Diagnóstico

### 1.1 Onde o cristalino avalia `context { ... }` e o que faz ao valor de retorno

- **Criação do bloco** (nível de eval, avaliação preguiçosa): `01_core/src/engine/eval/mod.rs:1159-
  1177` — `Expr::Contextual` constrói uma `Closure` que captura o scope actual e devolve
  `Value::Content(Content::ContextBlock(...))`. O corpo **não é avaliado aqui**.
- **Expansão** (pós-introspecção, onde o corpo é finalmente avaliado): `03_infra/src/pipeline.rs:108-
  167`, `expand_context_blocks`. Para cada `ContextBlock` encontrado no documento, chama
  `apply_func(elem.closure.clone(), ...)` (linha 157-163) para obter o `Value` de retorno do corpo, e
  depois `resolved.insert(*id, value_to_content(&result))` (linha 164) — **é aqui que o `Value` vira
  `Content`**.
- **Conversão** `Value` → `Content`: `01_core/src/engine/stdlib/state.rs:185-213`, função
  `value_to_content`. Tem braços explícitos para `Content`, `Str`, `Int`, `Float`, `Bool`, `Type`
  (P821), `Length`/`Ratio`/`Relative`/`Angle`/`Fraction` (P842), `Array` (P844) — **e um braço `_ =>
  Content::Empty` que apanha tudo o resto**, `Value::Dict` incluído.
- **Por que `Value::Dict` aparece aqui**: `measure(body)` é interceptado sintacticamente em
  `01_core/src/engine/eval/closures.rs:622-671` (P712) — não corre como função nativa genérica
  (`native_measure` em `stdlib/layout.rs:1794-1806` até devolve erro se chamada indirectamente). O
  resultado é construído directamente como dict (`closures.rs:658-668`):
  ```rust
  let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
  dict.insert("width".into(), Value::Length(...));
  dict.insert("height".into(), Value::Length(...));
  return Ok(Value::Dict(dict));
  ```
  Este `Value::Dict` sobe como valor de retorno do corpo `context measure[...]`, chega a
  `value_to_content`, e cai no braço `_ => Content::Empty`.

**Confirmação directa do sintoma** (bytes do PDF, `cristalino-07-context-p884.pdf`,
`/tmp/p872-bench/`): `4 0 obj << /Length 0 >> stream\n\nendstream` — content stream vazio, coerente
com `Content::Empty` chegando ao layout sem nada para desenhar.

### 1.2 Como o vanilla faz essa conversão

`repr_value` para `Value::Dict` já existe no cristalino e **já produz o formato certo**:
`01_core/src/engine/eval/repr.rs:43-55`:
```rust
Value::Dict(dict) => {
    if dict.is_empty() { return "(:)".to_string(); }
    let items: Vec<String> = dict.iter()
        .map(|(k, v)| format!("{}: {}", k.as_str(), repr_value(v)))
        .collect();
    format!("({})", items.join(", "))
}
```
Não abri `lab/typst-original/` para o caminho de código do vanilla em si (o `repr_value` de dict do
cristalino já bate com a medição directa do binário vanilla — não há necessidade de ler a fonte para
confirmar o formato, já está confirmado pelo output real): `pdftotext vanilla-07-context.pdf` produz
`(width: 42.85pt, height: 7.24pt)` — exactamente `{k}: {v}` por entrada, `", "` entre entradas,
parênteses a envolver, `Length` formatado como `42.85pt`. **O formato de `repr_value` já bate 1:1**
com o output medido do vanilla; falta só ligá-lo a `value_to_content` para `Value::Dict`.

### 1.3 Mesma causa raiz do achado 3 (P887, `table()` sem stroke)? — **Não, confirmado com evidência**

`01_core/src/entities/elements/table.rs`: o campo `stroke: Option<Stroke>` do `TableElem` é
inicializado como `stroke: None` nos três pontos de construção do ficheiro (linhas 143, 165, 204) —
quando `table()` é chamado sem `stroke:` explícito, o elemento fica sem stroke nenhum guardado. Isto
não passa por `value_to_content`, nem por nenhuma conversão "valor computado → `Content`" — é um
**default errado no construtor do elemento** (devia ser `Some(1pt + black)`, paridade com a
linguagem Typst onde `table()` tem stroke default e `grid()` não). Mecanismo, ficheiro e camada de
falha completamente diferentes do achado 2. **As duas hipóteses ("mesma causa" e "causas
independentes") foram comparadas com leitura de código nos dois locais — não é suposição.**

### 1.4 Regressão do achado #34/P860, ou caminho nunca coberto? — **Caminho nunca coberto, confirmado com evidência**

`00_nucleo/diagnosticos/typst-passo-860-relatorio.md` (lido nesta secção — fica em `diagnosticos/`,
não nas pastas de leitura restrita): P860 fechou o DEBT-69 corrigindo a **exactidão numérica** de
`measure_content_real` (largura E altura, via `text_edges`), validado extraindo texto de PDFs onde os
casos de teste eram `#context [...]` devolvendo **arrays/tuplas** (o relatório de P860 mostra
`(23.19pt, 7.9pt)` como output — formato de array, não de dict nomeado `(width: ..., height: ...)`).
Esse caminho já era coberto pelo braço `Array` de `value_to_content`, adicionado em **P844** (anterior
a P860). P860 nunca exercitou o retorno **directo** do dict de `measure()` sem desestruturar
(`context measure[...]` sem `let (w, h) = ...`) — não há nenhuma referência a `Value::Dict` no
relatório de P860 nem no código tocado por ele (`01_core/src/engine/layout/mod.rs`, fora de
`value_to_content`). **Conclusão: não é regressão — é um caminho de código que nunca teve cobertura**,
nem antes nem depois de P860.

---

## 2. Resumo do veredicto da Fase A

| Pergunta | Resposta | Evidência |
|---|---|---|
| Onde está o bug | `01_core/src/engine/stdlib/state.rs:212`, braço `_ => Content::Empty` de `value_to_content`, falta `Value::Dict` | Leitura directa do código + bytes do PDF (`/Length 0`) |
| Causa | `measure()` (P712) retorna `Value::Dict`; nunca foi adicionado braço para este tipo (P821/P842/P844 cobriram outros tipos, nunca Dict) | `closures.rs:668`, `state.rs:185-213` |
| Mesma causa que achado 3 (tabela)? | Não | `table.rs` — default errado no construtor, mecanismo diferente |
| Regressão de #34/P860? | Não — caminho nunca coberto | `typst-passo-860-relatorio.md`, testes de P860 usavam array não dict |
| Formato correcto já existe? | Sim — `repr_value` para `Value::Dict` já bate com o vanilla medido | `repr.rs:43-55` vs `pdftotext vanilla-07-context.pdf` |

---

## 3. Fix proposto (não implementado ainda — ver secção 5)

Em `value_to_content` (`state.rs`), adicionar antes do braço `_`:

```rust
Value::Dict(_) => Content::text(crate::engine::eval::repr::repr_value(value)),
```

Mesmo padrão já usado para `Value::Array` (P844) — reuso de `repr_value`, sem lógica nova.

---

## 4. Gate do Protocolo de Nucleação — L0 desactualizado, corrigido

`00_nucleo/prompts/engine/stdlib/state.md` é o L0 de `state.rs` (`@prompt-hash cab42f07` no
cabeçalho do ficheiro). A secção "P844 — `value_to_content` usa o repr para `Value::Array`" documenta
a cobertura de tipos actual — **mas não menciona `Value::Dict`**, e por isso está desactualizado
face ao que o Passo 886 precisa de mudar (regra de leitura do L0 vigente + Regra de Ouro do
`CLAUDE.md`: não instruir código L1 sem L0 actualizado).

**Acção tomada neste passo**: adicionada a secção `## P886 (achado 2 de P885) — value_to_content:
falta Value::Dict` ao final de `state.md`, com o sintoma medido, a causa, a correcção proposta, a
comparação com o achado 3 (secção 1.3) e com #34/P860 (secção 1.4), e os testes canónicos esperados.

---

## 5. STOP — aguardando confirmação do dono do projecto

Conforme o Protocolo de Nucleação do `CLAUDE.md`: *"Validação L0: se não existe ou está
desatualizado, a IA deve redigir o novo L0 e PARAR. Só prossegue quando o humano confirmar que
guardou o ficheiro e tem o hash."*

Este passo **pára aqui**. `00_nucleo/prompts/engine/stdlib/state.md` foi editado (secção 4). Antes
de avançar para a Fase B (testes + implementação em `state.rs`), preciso de confirmação de que:

1. O L0 actualizado foi guardado (revisto pelo dono, se necessário).
2. O hash foi recalculado — presumivelmente via `crystalline-lint --fix-hashes .`, per a tabela de
   erros do linter do `CLAUDE.md` (V5 `PromptDrift`) — e o `@prompt-hash` em `state.rs` está pronto
   para ser actualizado em conjunto com a implementação da Fase B.

Fases B (TDD: teste falha → implementação → suíte verde por crate → confirmação visual) e C
(regressão: benchmark completo dos 7 cenários) do `typst-passo-886.md` ficam pendentes desta
confirmação.

---

## 6. Confirmação do gate — L0 salvo, hash recalculado

O dono confirmou e pediu para correr o linter. `crystalline-lint --fix-hashes .` recalculou e
gravou o hash novo (`9e125978`) nos dois ficheiros que referenciam este L0 — `01_core/src/engine/
stdlib/state.rs` e `01_core/src/entities/state.rs` (ambos apontavam para o mesmo `@prompt`; só a
linha `@prompt-hash` mudou nos dois, `cab42f07` → `9e125978`). `crystalline-lint .` a seguir: **0
avisos de drift** (V5); resta só o V7 pré-existente e não relacionado (`package_version_resolution.md`
órfão, já conhecido desde antes deste passo). Gate satisfeito — Fase B autorizada a prosseguir.

---

## 7. Fase B — TDD + implementação

**Teste primeiro** (`01_core/src/engine/stdlib/state.rs`, módulo `tests`, função
`p886_value_to_content_dict_usa_repr`): constrói um `Value::Dict` com `width`/`height` (os mesmos
valores medidos no vanilla, `42.85pt`/`7.24pt`) e um dict vazio, e espera
`value_to_content(...).plain_text()` igual a `"(width: 42.85pt, height: 7.24pt)"` e `"(:)"`
respectivamente.

Confirmado que falha antes da correcção:
```
cargo test -p typst-core p886_value_to_content_dict_usa_repr
FAILED — left: "", right: "(width: 42.85pt, height: 7.24pt)"
```

**Implementação** (`state.rs`, `value_to_content`): braço novo antes do `_`, mesmo padrão do braço
`Array` (P844):
```rust
Value::Dict(_) => Content::text(crate::engine::eval::repr::repr_value(value)),
```
`@updated` do cabeçalho de linhagem actualizado para `2026-07-24`.

**Teste passa** após a correcção:
```
cargo test -p typst-core p886_value_to_content_dict_usa_repr
ok. 1 passed; 0 failed
```

**Suíte completa, discriminada por crate** (per regra reforçada do handoff pós-P884 — não aceitar
número "workspace" sem saber de onde vem):

| Crate | Passou | Falhou | Ignorado |
|---|---|---|---|
| `typst-core` | 4694 | 0 | 2 |
| `typst-infra` | 731 | 0 | 5 |
| `typst-shell` | 41 | 0 | 0 |
| `typst-wiring` (+ `tests/crystalline_lint.rs`) | 37 + 2 | 0 | 0 |

Zero falhas em qualquer crate.

**Confirmação visual E2E** (não só extração de texto): `cargo build --workspace --release`, depois
`07-context.typ` (fonte actual) recompilado nos dois binários:

- Cristalino (pós-fix): `(width: 42.93pt, height: 7.24pt)` repetido — render a 150dpi mostra texto
  visível preenchendo a página, mesmo formato e disposição do vanilla.
- Vanilla (mesma fonte, mesmo momento): `(width: 42.85pt, height: 7.24pt)` — diferença de 0.08pt na
  largura, mesma ordem de grandeza residual já registada em P860 (arredondamento sub-pixel), não um
  problema novo.
- PDF deixou de ter `/Length 0`: `cristalino-07-context-p886.pdf` tem 9893 bytes de conteúdo real
  (era 2100 bytes, página em branco, antes da correcção).

**`crystalline-lint .` final**: 0 violations novas; só o V7 pré-existente (`package_version_
resolution.md`), não relacionado com este passo.

---

## 8. Fase C — Regressão (benchmark completo, 7 cenários)

Metodologia idêntica à de P872/P884 (`hyperfine --warmup 1 --min-runs 10`, saída `/dev/null`,
comandos reconstruídos a partir dos JSON de P884: vanilla `compile --format pdf`, cristalino
`--root /tmp/p872-bench`). **Proveniência**: binário cristalino recompilado neste passo
(`cargo build --workspace --release`, working tree com as alterações desta secção — `git diff HEAD
--stat` no topo do relatório). Baseline "pré-P886" usada por cenário: JSON de P884 quando existe
(`timings-{02-lorem,04-math,06-long}-p884.json` — P884 só rebenchmarcou estes 3, os visados pela sua
frente de trabalho); para os outros 4 cenários (sem timing de P884), a baseline mais próxima
disponível é P880 (`{01-hello,03-images,05-tables,07-context}-p880.json`) — registado explicitamente
por cenário na tabela, não escondido.

| Cenário | Baseline usada | Vanilla (baseline) | Cristalino (baseline) | Razão (baseline) | Vanilla (pós-P886) | Cristalino (pós-P886) | Razão (pós-P886) |
|---|---|---|---|---|---|---|---|
| 01-hello | p880 | 274.2ms | 94.8ms | 0.35× | 274.4ms | 95.0ms | 0.35× |
| 02-lorem | p884 | 280.1ms | 118.3ms | 0.42× | 276.3ms | 118.1ms | 0.43× |
| 03-images | p880 | 6.8ms | 102.8ms | 15.22× | 6.7ms | 103.4ms | 15.35× |
| 04-math | p884 | 281.7ms | 4982.9ms | 17.69× | 278.0ms | 5140.5ms | 18.49× |
| 05-tables | p880 | 301.4ms | 114.9ms | 0.38× | 298.5ms | 113.9ms | 0.38× |
| 06-long | p884 | 297.2ms | 376.8ms | 1.27× | 294.3ms | 371.3ms | 1.26× |
| **07-context** | p880 | 293.8ms | 129.6ms | **0.44×** | 293.4ms | **137.3ms** | **0.47×** |

**Leitura**: 6 dos 7 cenários ficam dentro do ruído da baseline (diferenças ≤ ~1%, mesma ordem dos
`σ` de hyperfine) — **nenhuma regressão nova** nos cenários não tocados por este passo. `03-images`
e `04-math` continuam nas mesmas ordens de grandeza já diagnosticadas em P873 (causas raiz distintas
e não tratadas por este passo — busca de fallback não filtrada para math, dedup por identidade de
`Arc` para imagens; ver `00_nucleo/diagnosticos/typst-passo-873-relatorio.md`) — **pré-existentes,
fora do escopo deste passo**, não escondidas.

`07-context` é o único cenário que muda de forma notável: 129.6ms → 137.3ms (+6%, ainda **mais
rápido que o vanilla**, 0.47× vs 0.44×). Isto é **esperado, não uma regressão**: antes da correcção,
o cristalino calculava o dict de `measure()` 200 vezes mas descartava o resultado
(`Content::Empty`) sem nunca o converter em texto, layout ou shaping — o trabalho de emitir 200
`Content::text(...)` reais, moldá-los (shaping) e embuti-los como glifos no PDF é trabalho novo e
legítimo que antes não acontecia porque o bug escondia. O tempo subir é o custo de o bug estar
corrigido, não uma regressão a investigar.

---

## 9. Resultado — Passo 886 fechado

- Header de linhagem actualizado (`state.rs`, `@updated 2026-07-24`; `@prompt-hash` recalculado nos
  dois ficheiros que apontam para `state.md`).
- Teste novo (`p886_value_to_content_dict_usa_repr`) cobrindo o caso unitário + confirmação E2E via
  PDF (secção 7).
- Fase A: causa raiz identificada (`value_to_content` sem braço `Value::Dict`), confirmado que não
  partilha causa com o achado 3 (P887) nem é regressão de #34/P860 (secção 1).
- Fase B: TDD completo, suíte verde nas 4 crates, `crystalline-lint` limpo (secção 7).
- Fase C: benchmark completo dos 7 cenários, sem regressão nova; aumento esperado e explicado em
  `07-context` (secção 8).
- Decisão sobre a árvore não limpa: já resolvida antes do início deste passo (commit `9283b31d0`,
  secção 0).
