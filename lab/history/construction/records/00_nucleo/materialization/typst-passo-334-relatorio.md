# Relatório P334 — Lote F-1: a fronteira de extensão E1 no produto

**Pré-condição**: P333 fechado — L0 `f_fronteira_e1.md` aprovado, plano de lotes,
baseline da lente, suíte 2708, lint 0. ✅ Verificado.
**Tipo**: **Lote F-1** — o primeiro código de produto do F. Aditivo;
content-preserving; os 65 nativos NÃO mudam.
**Commits** (2, isoláveis): `515b99cbc` aprovação da Trava (registro) ·
`41af58496` lote F-1.

---

## Parte 0 — aprovação da Trava (onde mora)

`ADR-0106 §Aprovação da Trava` grava as 6 respostas do dono (L0 aprovado;
Trava-Q1 guard na realização; Trava-Q2 `dyn_kind_name`+`get_field`; `Value::Custom`
fora; plano F-1→F-2; R4 da lente em paralelo). O L0 `f_fronteira_e1.md` §0 passou
a **APROVADO/ativo em F-1**; o código declara `@prompt entities/f_fronteira_e1.md`
e o **warning V7 (órfão) limpou**. Hashes sincronizados (`--fix-hashes`).

## Fase A — plano de toque (real vs previsto)

Previsto ~15–25 sites; **real ~13 sites de produto** + 3 ficheiros novos + 12
testes. **Dentro da faixa.** 3 ambiguidades **mecânicas** (decididas e
registradas):

- **A1** — `dyn_hash` desnecessário: `content_hash::hash_content` serializa por
  `format!("{:?}")` (Debug), não match → `Content::Dynamic` faz hash pelo `Debug`
  do `dyn`. L0 §3a.2/§3a.3/§0 ajustados; `dyn_hash` removido do trait.
- **A2** — `layout_content` exaustivo (108 arms, sem `_`) → F-1 usa arm **no-op**
  `Content::Dynamic(_) => {}` (layout real = realização F-2; só fixtures hoje).
- **A3** — escopo do registro: F-1 entrega o tipo `ElementRegistry` + a trava +
  a fixture do caminho Rust; threading no eval do CLI é F-2+ (bounded pelo passo).

**Descoberto na Fase B (mecânico, registrado)**: o **blanket** torna todo
`Element` também `DynElement`; nomes coincidentes davam **E0034** (ambiguidade) e
o `Content` em contexto `Send+Sync` (introspector) exigiu **E0277** → resolvidos
com **nomes `dyn_*` distintos** + supertrait **`Send + Sync`** no `DynElement`. L0
§3a.2 atualizado para o **como-construído**.

## Fase B — o código (diff por item)

1. **`DynElement` + blanket** (`entities/elements/dynamic.rs`, L0 próprio):
   object-safe; `impl<T: Element + Send + Sync + 'static> DynElement for T`
   bridga `map_*<F>` → `dyn_map_*(&mut dyn FnMut)` (reborrow local resolve o
   `Sized`); `dyn_eq`+`as_any` (downcast); sem `dyn_hash`.
2. **`Element::dyn_kind_name`** defaultado `""` (`mod.rs` + `_comum.md` §A.1.5):
   os **65 ficheiros nativos intocados** (herdam o default); o utilizador
   sobrepõe (a fixture devolve `"callout"`).
3. **`Content::Dynamic(Arc<dyn DynElement>)`** + construtor `Content::dynamic` +
   **9 arms**: `plain_text`/`is_empty`/`eq`(`dyn_eq`)/`get_field`/`map_content`/
   `map_text` (hub); `locatable`/`extract_payload`; `materialize_time`+`walk`
   (introspect, terminal/leaf); `layout` (no-op, A2). Enum continua **fechado**.
4. **`ElementRegistry`** (`entities/element_registry.rs`): nome→construtor,
   **injetado** (sem global — pureza L1); elemento desconhecido = **erro
   declarado, não panic**.
5. **A trava ADR-0105 cláusula 3** (`teste-varre-registro`): para cada nome
   registado, constrói `Content::Dynamic`, despacha pelos 6 métodos, round-trip
   de `get_field`. Repõe a verificação que o compilador deixa de dar.
6. **Fixture `callout`** (`test_callout.rs`, `#[cfg(test)]`; fora dos 65):
   caminho Rust (`impl Element` → registra → constrói). `#set`/`#show` = F-2+.
7. **Testes de object-safety + blanket**: `Arc<dyn DynElement>` constrói e
   despacha; um nativo (`DividerElem`) atravessa o `DynElement` **com o mesmo
   resultado** do caminho estático; `dyn_eq` por downcast; hub eq/map_content.
8. **B3** (`world_types::Styles(())` stub morto): F-1 **não** toca a superfície
   → fica para **F-2** (registrado).

## Verificação (gates do lote)

- **`cargo build`** limpo (workspace inteiro); **suíte `2720 passed; 0 failed`**
  (2708 + **12 novos**: 6 `dynamic` + 2 `element_registry` + 4 `test_callout`);
  **zero asserção existente alterada**.
- **`crystalline-lint .`** = **✓ 0 violations, 0 warnings** (V7 órfão limpou; V2
  do fixture resolvido com bloco `#[cfg(test)]` próprio).
- **Lente** (`tekt-cargo-dsm@98d8f9e`, mesmo comando do baseline P333):
  **219 módulos** (217 + 2 esperados: `dynamic`, `element_registry`),
  **3 ciclos** (P333: 3 — **não pioram**); a porta nova `content→elements::dynamic`
  aparece; os 65 mantêm independência mútua.
- **Perf** (mesmo corpus 70 030 linhas/método P330): **F-1 ≈ baseline**. O
  baseline absoluto **derivou por ambiente** entre sessões (P330 0.6518 s →
  re-medido 0.719 s same-machine); a prova de não-regressão é **back-to-back na
  mesma sessão**: **baseline P333 0.7188 s vs F-1 0.7150 s** (F-1 marginalmente
  mais rápido, dentro do ruído). `size_of::<Content>` = **136 bytes** em ambos
  (o `Arc<dyn>` de 16 bytes não cresce o enum) — confirma zero regressão
  estrutural de perf.
- **Caveat de stack**: `RUST_MIN_STACK=33554432`.

## Contabilidade do F

- **F-1 fechado** — a fronteira de extensão E1 está no produto (aditiva). Os dois
  públicos: caminho Rust completo; caminho typst (construção por nome via
  registro) pronto; `#set`/`#show` na linguagem ficam para F-2.
- **Próximo: F-2** — o canal único das `Set*` (~108 sites, válvula `SetPage`
  declarada) + a realização/guards/`#show` (S2–S6) + B3 + de-bake gradual.
- **Sessão R4 da lente**: abre em paralelo (não bloqueou F-1); sem novidade aqui.

## git

- `41af58496` Passo 334 — lote F-1 · `515b99cbc` Passo 334 — aprovação da Trava.
- `git status` limpo fora de `lab/typst-original/` (cruft) e dos specs de passo
  (`typst-passo-33{3,4}.md`, untracked por convenção).
