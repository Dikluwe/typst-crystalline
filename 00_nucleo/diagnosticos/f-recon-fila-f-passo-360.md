# F-recon da fila F restante (P360) — medir para escolher o próximo lote

> **Tipo**: recon read-only de balanço de fila (probes via leitura/CLI; zero código de produto, zero
> L0; suíte não re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). A F-realização fechou (casos
> 1–4). Mede **custo × demanda × dependência** dos candidatos restantes — **F-5**, **F-6**, **Marco
> G**, **DEBT-59** — para o dono escolher o próximo. **Não decide.** Saída: este doc.
>
> **Resposta-linchpin (medida):** **NEM F-6 NEM Marco G precisam do chain-threading no introspect**
> que o F-5 ergueria. → a infra de introspect-chain tem **demanda NULA** na fila → o F-5 **continua
> limpeza-sem-demanda** (adiado, P353; ADR-0107). A fila **não reordena** em torno do F-5.

**Pré-condição**: P359 fechado; HEAD `63aec21a3`; suíte **2737/0**; lint **0/0**; lente **66/0**.
**Lente**: `tekt-cargo-dsm` commit `98d8f9e`.

---

## 1 — A linchpin, medida (`file:line`)

**(F-6 passa pelo introspect?) NÃO — layout-only.** As 3 folhas (`Content::Text`, `MathText`,
`MathIdent`) são **terminais** no walk do introspect (`rules/introspect.rs:200-211`, "clonar
directamente", sem recursão) e **não-locatáveis** (`extract_payload(Text) == None`,
`extract_payload.rs:137`). O estilo chega-lhes **no layout**, que já lê a chain: `Content::Text`
merge `node_style ⊕ self.style` (`layout/mod.rs:609-626`); math recebe **`&self.style`** (a chain)
por **parâmetro** — `equation.rs:37` `math_layouter.layout_equation(body, &self.style)` →
`math/layout/mod.rs:258 layout_node(content, style)`. **Nenhuma das 3 folhas precisa de `StyleChain`
no introspect.**

**(Marco G precisa do introspect ler a chain?) NÃO.** Migrar os 68 nativos pela fronteira E1 exige um
**arm `Content::Dynamic` no walk** do introspect — que **já existe como placeholder**
(`introspect.rs:356`, `:1095`). Isso é **walk de filhos pela trait**, não `StyleChain`. O introspect
lê os nativos por `to_payload` (trait) + match de `ElementPayload` — **independente** da chain. Marco
G é topologia de import (content→elements), **não** resolução de estilo.

**→ A infra de introspect-chain (o que tornaria o F-5 "fundação") tem demanda ZERO.** F-5 fica
limpeza adiada.

---

## 2 — Tabela: custo × demanda × dependência × observável (medido)

| Item | Largura/custo (medido) | Demanda (medida) | Dependências | Observável |
|---|---|---|---|---|
| **F-5** de-bake | 4 pontos: 3 numbering (heading/eq/figure) + `Text` `TextStyle`; os numbering exigem introspect-chain; o `TextStyle` arrasta o bold do heading (`markup.rs:85-86`) | **LIMPEZA, nula** — caminho duplo chain≡assado paridade-testada (P353); ninguém precisa | **precisa introspect-chain** (e só o F-5 a ergueria → circular; sem demanda externa) | interno (paridade-testada) |
| **F-6** 3 folhas (DEBT-58) | **SMALL** — `Text` 4 prod/7 cons; `MathText` 6/5; `MathIdent` 2/5 (confirmado vs DEBT.md) | **LIMPEZA** — a chain **já chega** no layout (Text baked+merge; Math via `self.style` param); `MathText`/`MathIdent` **já** são param-styled (sem campo a de-bakar); só o `Text` `TextStyle` é redundante | **layout-only**; NÃO precisa introspect-chain | **sem divergência observável medida** (a chain alcança as 3 no render) |
| **Marco G** desacoplar nativos | **MEDIUM-LARGE** — **68** variantes `Content::*(Arc<*Elem>)` → colapsar em `Content::Dynamic`; ~3 ficheiros (content.rs, introspect.rs, layout/mod.rs), **30+** match arms | **ESTRUTURAL** — é a **métrica-gate do F** (`content→elements` 66→0); é o objetivo declarado do lote F | **E1 feito** (F-1, `dynamic.rs`); Dynamic walk-arm placeholder existe; NÃO precisa introspect-chain | **interno/lente** (66→0; não-observável no doc) |
| **DEBT-59** flag CLI | **SMALL** — `cli.rs` add `--full-error` (~3 ln) + map p/ `RunIntent` (1 ln, campo já existe `:103-117`); fio L4 (`main.rs:58`, 1 ln); `eval_with_full_error` **já existe** (P350c) | **DIAGNÓSTICO** — útil (classifica erro de recursão de `#show`), nicho | **isolado** (L2/L4 + fio interno L3) | **observável** (3º hint de erro) |
| **DEBT-60 (a)** | — | **SELADO** — divergência consciente medida (P335 deliberado; confinado não-idiomático; gatilho registrado) | — | registrado, não pendente |

---

## 3 — Grafo de dependência

```
E1 (F-1, feito) ──► Marco G (68→1; precisa Dynamic walk-arm [placeholder existe]; NÃO chain)
                      └ métrica-gate do F (content→elements 66→0) — o objetivo estrutural restante

F-5 de-bake ──(precisa)──► introspect-chain  ◄──(NÃO precisa)── F-6, Marco G
   └ a única coisa que ergueria a infra É o F-5 → demanda circular → NULA → adiado (ADR-0107)

F-6 (3 folhas) ── layout-only, sem dependência ── LIMPEZA (chain já chega)
DEBT-59 ── isolado (L2/L4 + fio L3) ── pequeno
```

**Leitura do grafo:** a infra de introspect-chain não tem consumidor a montante (só o próprio F-5 a
quereria). Logo o F-5 não é fundação de nada — fica por último. O item **estrutural** restante (a
razão do lote F) é o **Marco G**; está **desbloqueado** (E1 feito, Dynamic walk-arm placeholder), mas
é o **maior**. F-6 e DEBT-59 são pequenos e independentes.

---

## 4 — Recomendação marcada (a decisão é do dono)

Nenhum item é uma **correção** observável (F-6 medido sem divergência; F-5 limpeza; Marco G interno;
DEBT-59 diagnóstico). Logo a escolha é por **valor arquitetural** vs **custo**:

- **Recomendado: Marco G — começando por um recon/sub-spec próprio.** É o **objetivo estrutural
  restante do F** (a métrica-gate `content→elements→0`), está **desbloqueado** (a linchpin limpou o
  medo do introspect-chain; o E1 e o Dynamic walk-arm existem), e fecha o lote F de verdade. **Mas é
  o maior** (68→1, 30+ arms) — o plano sempre disse "sub-spec própria"; o próximo lote deve ser o
  **desenho** (medir o colapso 68→1, os arms, os riscos de exaustividade/ADR-0105 cl.3), não o código
  directo.
- **Alternativa (ganho pequeno isolado): DEBT-59** — expor a flag na CLI (~5 linhas L2/L4 + fio L3;
  `eval_with_full_error` já existe). Útil, isolado, baixo risco. Bom se o dono quiser um lote curto
  antes do Marco G.
- **F-5 e F-6: adiar.** Limpeza sem demanda medida (ADR-0107: não erguer/mexer à frente da demanda).
  F-5 continua o débito de de-bake disponível; F-6 idem (a chain já alcança as 3 folhas no render).
  Reabrir quando uma demanda real (ex.: um campo de estilo de folha que diverge observável) surgir.

**Decisão proposta ao dono:** o próximo lote é **(i) o recon/sub-spec do Marco G** (o trabalho
estrutural restante), **ou (ii) DEBT-59** (ganho pequeno), com **F-5/F-6 adiados**. A escolha é do
dono. Nenhum código/L0 tocado; suíte 2737; lente 66/0; árvore limpa.

---

## 5 — DECISÃO DO DONO (P360): **Marco G — começar pelo recon/sub-spec**

Escolhido o **Marco G** como o próximo trabalho, **começando pelo desenho** (recon/sub-spec próprio),
não pelo código. O próximo lote mede/desenha: o colapso das **68** variantes `Content::*(Arc<*Elem>)`
em `Content::Dynamic`; os **~30+** match arms (content.rs, introspect.rs, layout/mod.rs) que mudam; o
arm `Content::Dynamic` no walk do introspect (placeholder → real, walk de filhos pela trait); a
**Trava ADR-0105 cláusula 3** (exaustividade por elemento que o `dyn` esconde — teste/lint que varre
o registro × backends); a ordem (cabe num lote ou fatia-se). **F-5 e F-6 adiados** (limpeza sem
demanda, ADR-0107). DEBT-59 fica como ganho pequeno disponível. Este recon (P360) **fecha** aqui; o
desenho do Marco G é o lote seguinte.
