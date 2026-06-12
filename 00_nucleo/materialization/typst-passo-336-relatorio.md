# Relatório P336 — Lote F-3 inc-2: `#show` sobre o elemento dinâmico (fecha F-3)

**Pré-condição**: F-3 inc-1 (`69d4929d2`), suíte 2710, lint 0/0. ✅
**Resultado**: o elemento de utilizador entra **na linguagem** (`#callout(...)` +
`#show callout:`) pelo **mesmo** caminho eager dos nativos. **F-3 FECHADO.**
Suíte 2710 → **2717** (+7), lint 0/0.

---

## Caronas (`067a4a59c`)

- **C1** — fóssil em `f2-progresso-passo-335.md`: a seção "S5b — adiado"
  contradizia a tabela (S5b ✅). Marcada superseded. **Δ testes: 0.**
- **C2** — removido `engine.figure_numbering` (resíduo threaded-mas-não-lido do
  F-2 S3): 16 linhas (1 decl + 10 atribuições + 5 locais-base) em todas as
  construções de `Engine` (incl. macros). O param `figure_numbering: Option<&str>`
  das nativas permanece (vem da chain). **Δ testes: 0.** Suíte 2710.

## Fase A — reconhecimento + checkpoint (premissa quebrada)

Mapa (file:line): `eval()` constrói o escopo via `make_stdlib()` (`mod.rs:217`,
nativos como `Func::native(fn_ptr)`, `:588+`); `Func`/`FuncRepr` (`func.rs:22`) =
`Closure|Native`. **O ctor do registry é uma closure** (`Arc<dyn Fn>`) — não cabe
em `Func::native`. `ElementRegistry` não era referenciado fora do próprio ficheiro
(F-1 deferiu **tudo**). `#show` guarda por `active_guards` (RuleId) + depth-64.

**Checkpoint disparado** (premissa quebrada): "rodar o vanilla, afirmar
igualdade" é impraticável p/ elemento custom (vanilla usa `#[elem]`, sem binário
pronto; o spike-2 leu a fonte, nunca rodou). **Decisão do dono (Opção 1)**:
paridade contra a **semântica medida na fonte** pelo spike-2.

## Fase B — estágios

| Estágio | Commit | Δ testes | Suíte |
|---------|--------|----------|-------|
| **S1 — registry no escopo** | `517df87c4` | **+2** (`f3s1` ×2) | 2710→2712 |
| **S2 — `Selector::DynKind` + braço** | `84de6dbda` | **+4** (`f3s2` ×4) | 2712→2716 |
| **S3+S4 — gatilho + fecho** | `677a89d11` | **+1** (`f3s3` ×1) | 2716→2717 |

**S1** — o threading registry→escopo (F-1 deferiu): `FuncRepr::Element(ElementFunc)`
+ `Func::element` (`func.rs`); `ElementCtor` ganha `+ Send + Sync` (`Func`/`Value`/
`Content` são Send+Sync, fronteira E1); `ElementRegistry::ctor(name)`; arm em
`apply_func` (`closures.rs`) → `Value::Content(ctor(args))`; `eval()` ganha param
`registry` e define cada elemento como `Func::element` no escopo base. Callers:
pipeline (vazio) + `eval_for_test_with_registry` (novo) + 16 calls diretos.
Testes: `#callout(...)` resolve+renderiza; desconhecido = erro do catálogo.

**S2** — `#show callout:` intercepta `Content::Dynamic` pelo **mesmo**
`apply_show_rules` (NodeKind | DynKind na MESMA travessia), guard por **RuleId** +
depth-64. `eval_show_rule` casa o selector de elemento por `Func::element_name`
(sem fn-ptr) → `Selector::DynKind`. Fixture `BadgeElem` (kind `badge`). Testes:
transform aplica; **anti-recursão termina** (teste-contrato); callout não pega
badge; dinâmico+nativo coexistem.

**S3 (o gatilho executado)** — veredito por caso do spike-2:

| Caso | Veredito | Evidência |
|------|----------|-----------|
| 2 recursão/guard | **PARIDADE ✅** | `f3s2_..._anti_recursao_termina`; vanilla `typst-realize:472-474` |
| 5 nativo+dyn | **PARIDADE ✅** | `f3s2_dyn_e_nativo_coexistem` |
| **4 escopo** | **DIVERGÊNCIA — gatilho DISPARADO** | `f3s3_..._divergencia_registrada` (assere o estado atual: o eager **não confina** `#show` ao bloco; vanilla `content/mod.rs:744-752`) |
| 1 composição | falta-superfície (multi-passe) | registrado |
| 3 show-set | falta-superfície (`Transformation::Style`) | registrado |

O caso 4 falha porque o eager muta `engine.show_rules` da declaração em diante —
**propriedade pré-existente** do modelo eager (afeta nativos também, **não**
introduzida pelo dyn). Conforme (c): **não consertado inline**.

**S4 (registro + fecho)** — L0 §3b.6 grava o veredito; a **`#show` léxica +
realização multi-passe vira LOTE concreto** (`F-realização`) na fila
(`f-plano-lotes-passo-333.md`); **F-3 marcado FECHADO** (inc-1 DEBT C2 + inc-2
linguagem). F-4 `Styled` é o próximo do roteiro de estilo.

## Verificação

- **Suíte** `2717 passed; 0 failed` (2710 + 7: +2 S1, +4 S2, +1 S3).
- **Lint** `0 violations, 0 warnings`. **Workspace** build limpo.
- **Árvore limpa**; **commit por estágio** (caronas/S1/S2/S3+S4); **F-3 fechado**
  na fila; o doc retomável do F-2 teve o **fóssil S5b** consertado (C1).
- **Caveat de stack**: `RUST_MIN_STACK=33554432`.

## Contabilidade do F

- **F-3 FECHADO** — a fronteira E1 entra na linguagem (`#name`/`#show <dyn>`),
  eager, caminho único com os nativos; DEBT C2 fechado (inc-1).
- **Lote novo (gatilho disparado): `F-realização`** — `#show` léxico (caso 4) +
  composição (caso 1) + show-set (caso 3) = a realização multi-passe do spike-2.
  Toca o modelo eager dos **nativos** também.
- **Próximo do roteiro de estilo: F-4 `Styled`** (a 2ª StyleChain do Layouter
  colapsa na chain única).
