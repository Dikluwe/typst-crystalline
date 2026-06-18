# Relatório P360 — Balanço da fila F restante (recon read-only)

> **Tipo**: recon de balanço de fila, read-only (probes via leitura/grep + 1 lente; zero código de
> produto, zero L0; suíte não re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). A F-realização
> fechou (casos 1–4). Mede **custo × demanda × dependência** dos candidatos restantes (F-5, F-6,
> Marco G, DEBT-59) para o dono escolher o próximo lote. **Não decide.** Saída detalhada:
> `00_nucleo/diagnosticos/f-recon-fila-f-passo-360.md`.
>
> **Resposta-linchpin (medida):** **nem F-6 nem Marco G precisam do chain-threading no introspect**
> que o F-5 ergueria → a infra tem **demanda NULA** → o F-5 **continua adiado** (ADR-0107). A fila
> **não reordena**. **Decisão do dono: Marco G como próximo, começando pelo recon/sub-spec.**

**HEAD**: `63aec21a3` (pós-P359). **Branch**: Tekt. Lente `tekt-cargo-dsm` `98d8f9e` (66/0). Suíte 2737/0.

---

## 1 — A linchpin, medida (`file:line`)

**F-6 passa pelo introspect? NÃO — layout-only.** As 3 folhas (`Text`/`MathText`/`MathIdent`) são
**terminais** no walk (`introspect.rs:200-211`, "clonar directamente", sem recursão) e
**não-locatáveis** (`extract_payload(Text) == None`, `extract_payload.rs:137`). O estilo chega-lhes
no **layout**, que já lê a chain: `Text` faz merge `node_style ⊕ self.style` (`layout/mod.rs:609-626`);
math recebe **`&self.style`** (a chain) por **parâmetro** (`equation.rs:37`
`layout_equation(body, &self.style)`). **Nenhuma precisa de `StyleChain` no introspect.**

**Marco G precisa do introspect ler a chain? NÃO.** Migrar os nativos pela E1 exige um arm
`Content::Dynamic` no **walk** do introspect (placeholder já existe, `introspect.rs:356/1095`) — isso
é walk de filhos pela trait, **não** `StyleChain`. O introspect lê os nativos por `to_payload` (trait)
+ match de `ElementPayload`, independente da chain.

**→ A infra de introspect-chain (o que tornaria o F-5 "fundação") tem demanda ZERO.** F-5 fica
limpeza adiada.

## 2 — Tabela: custo × demanda × dependência × observável (medido)

| Item | Largura/custo | Demanda (medida) | Dependências | Observável |
|---|---|---|---|---|
| **F-5** de-bake | 4 pontos (3 numbering + `Text` `TextStyle`) | **LIMPEZA, nula** — caminho duplo chain≡assado paridade-testada (P353) | precisa introspect-chain (só o F-5 a ergueria → circular; sem demanda externa) | interno |
| **F-6** 3 folhas (DEBT-58) | **SMALL** — `Text` 4/7, `MathText` 6/5, `MathIdent` 2/5 | **LIMPEZA** — a chain **já chega** no layout (Text baked+merge; Math via `self.style` param; Math sem campo a de-bakar) | **layout-only**; não precisa introspect-chain | **sem divergência observável medida** |
| **Marco G** | **MEDIUM-LARGE** — 65 variantes → `Content::Dynamic`; ~3 ficheiros, 30+ arms | **ESTRUTURAL** — a métrica do marco (`content→elements` 66→0); objetivo declarado | E1 feito; Dynamic walk-arm placeholder existe; não precisa introspect-chain | **interno/lente** (66→0) |
| **DEBT-59** flag CLI | **SMALL** — `cli.rs` add `--full-error` (~3 ln) + map RunIntent (campo já existe) + fio L4 (1 ln); `eval_with_full_error` já existe (P350c) | **DIAGNÓSTICO** — útil, nicho | isolado | observável (3º hint) |
| **DEBT-60 (a)** | — | **SELADO** — divergência consciente medida (P335 deliberado; confinado não-idiomático; gatilho) | — | registrado |

**Nenhum item é uma correção observável** (F-6 medido sem divergência; F-5 limpeza; Marco G interno;
DEBT-59 diagnóstico).

## 3 — Grafo de dependência

```
E1 (F-1, feito) ──► Marco G (65→Dynamic; precisa Dynamic walk-arm [placeholder existe]; NÃO chain)
                      └ a métrica do marco (content→elements 66→0) — o objetivo estrutural restante
F-5 de-bake ──(precisa)──► introspect-chain ◄──(NÃO precisa)── F-6, Marco G
   └ só o F-5 quereria a infra → demanda circular → NULA → adiado (ADR-0107)
F-6 (3 folhas) ── layout-only, sem dependência ── LIMPEZA (chain já chega)
DEBT-59 ── isolado ── pequeno
```

A infra de introspect-chain não tem consumidor a montante → o F-5 não é fundação → fica por último.
O item estrutural restante é o **Marco G** (desbloqueado: E1 feito, Dynamic walk-arm placeholder),
mas é o **maior**. F-6 e DEBT-59 são pequenos e independentes.

## 4 — Decisão do dono (P360)

**Marco G como próximo, começando pelo recon/sub-spec** (desenho, não código). F-5 e F-6 **adiados**
(limpeza sem demanda, ADR-0107). DEBT-59 fica como ganho pequeno disponível. O recon/sub-spec do
Marco G é o lote seguinte (executado no P361, `f-recon-marco-g-passo-361.md`).

> **Nota de fecho (P361):** o recon/sub-spec do Marco G mediu que ele **não é um lote** — modelo
> não-decidido (α `Content::Dynamic`-vtable vs β PropMap-reificação; ADR-0026 rejeita vtable),
> acoplamento que **reloca** (layout 48 arms → tabela kind→handler), e plano marca-o **fora desta
> branch / pós-F-6**. A próxima decisão do Marco G é um **ADR de modelo**, não código. Ver P361.

## 5 — Gates / estado

```
read-only: nenhum código de produto, nenhum L0 tocado; probes via leitura/grep/lente.
lint: crystalline-lint . = 0/0 (inalterado). suíte: 2737 (não re-rodada — read-only).
lente: tekt-cargo-dsm 98d8f9e; content→elements = 66; elemento→elemento = 0.
INTACTOS: F-realização, α/caso 2, caso 4, morph ==/morph_canon, flag P350c, Marco G — não tocados.
saída: f-recon-fila-f-passo-360.md (a tabela + o grafo + a decisão). árvore limpa fora de docs.
```

## Fora de escopo (confirmado)

A implementação de qualquer item — nasce da escolha do dono; F-5/F-6 adiados; A2/A3 do caso 1
(declinados P358); qualquer toque no α / `morph_canon` / `==` ou na flag.
