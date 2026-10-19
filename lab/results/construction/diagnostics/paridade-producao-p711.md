# Paridade Produção — P711 — `context` herda o `StyleChain` da posição

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-711.md`
**Hash do commit (implementação):** `a9a108770`.
**HEAD base:** `f8402e1f2` (fim de P710).
**Estado:** FECHADO — `expand_context_blocks` (L3) usa agora o `StyleChain`
acumulado até à posição do bloco `context`, não uma cadeia de defaults
isolada. ADR-0114 em vigor (mecanismo central); sonda + varredura ampla
feitas antes da implementação.

---

## 1. Sonda

### 1.1 Confirmar o alcance com múltiplos casos

```typst
#set text(size: 12pt)
#context [#(10em).to-absolute()]     — cristalino (antes): Abs(110.0)
                                        (default 11pt, ignora o #set)
#set text(size: 20pt)
#context [#(2em).to-absolute()]      — cristalino (antes): Abs(22.0)
```

Vanilla: `120pt` / `40pt` (usa o `#set` real). Confirmado: qualquer
`#set text(size:)` antes de `context {...}` era silenciosamente
ignorado dentro do bloco.

### 1.2 Localização exacta — `file:line`

`03_infra/src/pipeline.rs` (L3), função `expand_context_blocks`
(linha 108-159, antes da correção):

```rust
let mut styles = StyleChain::default_chain();   // linha 131 — sempre default
```

E `collect_context_blocks` (linha 161-184, antes) descartava o delta ao
atravessar `Content::Styled`:

```rust
Content::Styled(inner, _) => {          // `_` — delta descartado
    map.extend(collect_context_blocks(inner));
}
```

Confirmado que **nenhum outro ponto do código invoca a closure** de
`ContextBlockElem` — grep exaustivo em `01_core/src` (`.closure\b`)
só encontra o `Clone` impl; `Content::ContextBlock(_) => {}` em
`layout/mod.rs:1172` é confirmado no comentário como "defensive no-op
(deve ter sido expandido antes)" — ou seja, `expand_context_blocks`
em L3 é o único ponto real de resolução, chamado no pipeline entre
`introspect` e `layout` (`compile_to_pdf_bytes_full_error`,
`pipeline.rs:348-358`).

### 1.3 Mecânica correcta confirmada no vanilla

`lab/typst-original/crates/typst-library/src/foundations/context.rs:78-82`:

```rust
pub const CONTEXT_RULE: ShowFn<ContextElem> = |elem, engine, styles| {
    let loc = elem.location().unwrap();
    let context = Context::new(Some(loc), Some(styles));
    Ok(elem.func.call::<[Value; 0]>(engine, context.track(), [])?.display())
};
```

`context {...}` é um **show rule** no vanilla — recebe o `styles:
StyleChain` **da posição no show-tree onde aparece**, exactamente
como qualquer outro show rule. Não há StyleChain "de defaults"
separada; o mecanismo de show rules já thread a chain correcta.
Confirma a direcção da correcção: `collect_context_blocks` tem de
acumular a chain (aplicando `push_styles` a cada `Content::Styled`
atravessado), não descartar o delta.

### 1.4 `measure()` dentro de `context` — também afectado?

```typst
#set text(size: 20pt)
#context [#let s = measure[Texto de teste]; #s.width]
```

Vanilla: `113.06pt`. Cristalino (antes e depois de P711): `Abs(0.0)` —
**sempre zero**, dentro e fora de `context` (testado sem `context`
directamente: vanilla dá `error: can only be used when context is
known`; cristalino aceita e devolve zero, silenciosamente). **Não é
causado pelo bug de StyleChain** — é um gap pré-existente e
independente em `measure()`, que não resolve larguras reais em
nenhum caminho de execução. Fora do escopo de P711; ver §5.

---

## 2. Achado adicional, não relacionado, encontrado durante a sonda

`01_core/src/engine/eval/repr.rs:59-66` (`repr_value`) formata
`Value::Length`/`Ratio`/`Angle`/`Color`/`Stroke`/`Align` com `{:?}`
do Rust (`format!("{:?}", l)`) em vez do repr Typst, quando estes
valores são embutidos directamente em markup via `#expr`
(`eval/mod.rs:585-590`, comentário confirma que `repr_value` é o
caminho oficial de conversão). Confirmado **independente do bug de
StyleChain** — mesmo bug de formatação dentro e fora de `context`,
só o número interno muda (`Abs(110.0)` vs `Abs(120.0)`). Não corrigido
aqui — ver §5 (P712 sugerido).

---

## 3. Implementação

### `03_infra/src/pipeline.rs`

- `collect_context_blocks(content: &Content, chain: &StyleChain) ->
  HashMap<u64, (Arc<ContextBlockElem>, StyleChain)>` — acumula a
  cadeia: parte de `StyleChain::default_chain()` (chamador,
  `expand_context_blocks`) e aplica `.push_styles(styles)` a cada
  `Content::Styled(inner, styles)` atravessado, associando a cadeia
  resultante (não só o elemento) a cada `ContextBlockElem`
  encontrado. Assinatura e contrato documentados em
  `00_nucleo/prompts/infra/pipeline.md` §`expand_context_blocks`.
- `expand_context_blocks` usa `block_chain.clone()` (a cadeia
  associada ao bloco) como `engine.styles`, em vez de
  `StyleChain::default_chain()` isolada.
- Sem alterações a `substitute_context_blocks`, `value_to_content`,
  ou a qualquer código de L1 — a correcção é inteiramente local ao
  walk de recolha em L3.

### Testes (4 novos, `pipeline.rs` `mod tests`)

- `collect_context_blocks_acumula_size_de_set_ancestral` — `#set`
  simples é reflectido na cadeia colectada.
- `collect_context_blocks_sem_set_mantem_default` — sem `#set`,
  cadeia continua nos defaults (11pt) — sem regressão.
- `collect_context_blocks_sets_aninhados_o_mais_interno_vence` —
  dois `#set` aninhados, o mais próximo do bloco vence (paridade com
  o comportamento de scoping léxico já usado por `eval_markup`).
- `collect_context_blocks_multiplos_blocos_em_sequence_cadeias_independentes`
  — dois blocos irmãos em `Content::Sequence`, um dentro de
  `Content::Styled` e outro não: cadeias não vazam entre ramos.

---

## 4. Validação

Reprodução exacta da sonda (§1.1), release build, contra o binário
corrigido:

```
#set text(size: 12pt)
#context [#(10em).to-absolute()]
#set text(size: 20pt)
#context [#(2em).to-absolute()]
```

Resultado: `Abs(120.0)` / `Abs(40.0)` — **paridade numérica com o
vanilla** (`120pt`/`40pt`); o formato de exibição (`Length {...}` em
vez de `"120pt"`) é o bug §2, inalterado por este passo (esperado).

- `cargo test --workspace` → **3798** (typst-core, inalterado) + **630**
  (typst-infra: 626 + 4 novos) passed, 0 failed.
- `crystalline-lint .` → 0 violations; `--fix-hashes .` → "Nothing to
  fix" (hash de `pipeline.md`↔`pipeline.rs` já consistente após a
  edição do L0).
- Repetido `measure()` dentro de `context` (§1.4): continua
  `Abs(0.0)` — confirma que o fix de P711 não altera (nem esconde)
  esse bug separado, como esperado.

### Reprodução `cetz`

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({ import cetz.draw: *; line((0,0),(2,1)); circle((0,0)) })
```

`error: cannot apply Div to length and length` — mesmo bloqueio de
P710 (não tocado por este passo), tempo de compilação **~52.7s**,
mesma ordem de grandeza, sem regressão.

---

## 5. Não corrigido aqui — próximos passos sugeridos, um bug por passo

- **P712** (sugerido): `repr_value` (`repr.rs:59-66`) — Debug format
  do Rust em vez de repr Typst para `Length`/`Ratio`/`Angle`/`Color`/
  `Stroke`/`Align` embutidos em markup.
- **P713** (sugerido): `measure()` devolve sempre `0pt` (dentro e fora
  de `context`); falta também o gate "can only be used when context
  is known" do vanilla.
- **Length / Length (Div)** — bloqueio actual do `cetz`, já suprido
  em P710 como sugestão de passo seguinte (não renomeado para não
  colidir com a numeração dos dois achados acima; o próximo passo
  livre para este continua a ser proposto ao utilizador).

## 6. Critério de fecho do passo

- [x] Sonda completa: alcance confirmado (`text(size:)`), localização
      exacta (`pipeline.rs:108-184`), mecânica correcta confirmada no
      vanilla (show rule com `styles` real), `measure()` confirmado
      como bug separado e não afectado pela correcção.
- [x] Corrigido, testado com múltiplos casos (`#set` simples,
      aninhado, ausente, blocos irmãos independentes).
- [x] Varredura ampla do corpus existente — 0 regressões
      (3798 + 630, era 3798 + 626).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — mesmo bloqueio de P710, sem regressão.
- [x] Relatório com resultado exacto (este ficheiro), achados fora de
      escopo registados (§2, §1.4) e não corrigidos, com passos
      sugeridos (§5).
