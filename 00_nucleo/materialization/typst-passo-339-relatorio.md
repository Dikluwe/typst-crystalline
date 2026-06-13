# Passo 339 — relatório: F-realização fatia 1 (transporte aditivo β1)

> **Fatia 1 da F-realização FECHADA.** Construído o transporte `StyledElem`-scoped
> aditivo: `#set …(numbering:)` passa a embrulhar o resto do escopo léxico num
> `Content::Styled` carregando o custom. **Aditivo** — baking permanece
> autoritativo; consumidores **não** religados (F-5); `#show` intocado (`f3s3`
> divergente); sem guards/multi-passe (Trava-Q1 = fatia 2).

Caveat de stack em todas as corridas de teste: `RUST_MIN_STACK=33554432`.

---

## Caronas

- **C0 — suíte herdada (fim do P338):** **2711** (`typst-core --lib`) em `6139e3517`.
  Workspace 3230. Reconciliação: o recon P337 projetou 2718 no início do P338;
  `triagem-47` + E0 removeram `is_numbering_active` morto, net −7. Base = **2711**.
- **C1 — prova-de-mordida do F-4:** já coberta, Δ=0. `style.rs:236`
  `styles_from_iter_last_write_wins_por_campo()` asserta `[Bold(true),Bold(false)]→false`.
- **C2 — protocolo de perf:** par back-to-back na mesma sessão (ver §Verificação).

## Estágio 0 — gatilho do spike-2

Sob a superfície da fatia 1 (transporte de escopo; **sem** multi-passe nem
`Transformation::Style`), nenhum dos 5 casos do spike-2 vira teste de paridade
**de `#show`** nesta fatia: caso 4 (escopo) só viraria sob a opção (II)
[confinar `#show`], **rejeitada** pelo dono a favor de **(I) transporte aditivo**;
casos 1/2 (composição/guard) → fatia 2; caso 3 (show-set) → fatia 3. "Medido" =
semântica da fonte com `file:line`; gatilho de 2º nível (compilar vanilla) só se a
leitura conflitar. Todos os ponteiros mantidos.

## Fase A — recon dimensionador (números exatos)

Endereços do recon P337 reverificados pós-F-4 (válidos, com shifts):
`intercept_content` `rules.rs:193`, `apply_show_rules` `:69`, `eval_show_rule`
`:507` (era 496), `show_rules=Arc::from` `:588`; baking `#set` `rules.rs:242`
(heading) / `:266` (eq) / `:313` (figure); consumo layout `layout/mod.rs:1248`;
vanilla `StyledElem` `content/mod.rs:744-752`.

- **Show-state em eval:** 34 ocorrências em 5 ficheiros; vazamento de escopo do
  `#show` = assimetria `eval/mod.rs:398-399` (CodeBlock confina) vs `:454-461`
  (ContentBlock vaza) = caso 4 / `f3s3`.
- **Testes de `#show` eager:** 20 exatos (15 `show_rule_*` + 4 `f3s2_*` + 1 `f3s3`).
- **Fixpoint externo:** não existe (eager single-pass) → fatia posterior.
- **Fatiamento confirmado:** fatia 1 = fundação do transporte; fatias 2/3 = P340/P341.
- **`f3s3`:** sob (I), **intacto** (é `#show`; a fatia toca só `#set`).

## Checkpoint da TRAVA → decisões do dono

1. **Escopo:** **(I) transporte aditivo** (não confinar `#show`; `f3s3` intacto).
2. **Trava-Q1** (guard/lifecycle) → **fatia 2**; a fatia 1 não nasce o wrapper de guard.
3. `content→elements → 0`: registado como **fora da fila atual** (decisão pendente).

## Gate L0 + recon β3 + protótipo de medição (caronas do gate)

O L0 deferia o mecanismo concreto (`style.md`/§3b.5/§3b.7 dão só o contrato).

- **Recon β3 (inviável, da fonte):** a chain já fiada **não** alcança o consumo
  carregando o custom. `self.chain` nasce fresca (`layout/mod.rs:374`), escrita só
  por `Content::Styled`/`Strong` (`:1251`/`:1293`); `engine.styles` (custom de
  eval) **não** é passada a `layout()`/`layout_with_introspector()` (`:2644`/`:2668`)
  — morre na fronteira eval→layout. E os 3 consumidores leem o **campo assado**
  (`layout:714`, `:812`, `introspect.rs:817`), não a chain. β3 colapsa no binário β1/β2.
- **Protótipo de medição β1 vs β2 (maquinaria real, instrumento descartável):**
  - β1 paridade: layout `plain_text` idêntico, introspect `kind_index` idêntico;
    única diferença `Content::PartialEq` (wrapper visível) — **sem dependente em
    produção** (`Content ==` só em `source.rs:38` por id+hash; `#show` casa por selector).
  - β1 perf: ≈2.8µs/wrap/passe; β2 proxy (1 walk extra) ≈17.6µs (~6×).
  - β1 superfície: precisa do caminho `Styles`→custom (ausente) + wrap nos 3 sítios;
    **0** arms de consumidor novos. β2: módulo de walk novo.
  - **Veredito: β1** (paridade limpa, ~6× mais barato, superfície menor). **Selado.**
- **L0 §3a.8 redigido** (f_fronteira_e1.md +91) + `Styles::push_custom` em `style.md`;
  `--fix-hashes` re-selou 4 ficheiros (commit `c8b5e12b9`).

## Fase B — diff por estágio

| Estágio | Commit | Conteúdo |
|---|---|---|
| **T** caracterização | `6b041efa5` | `f339t_caracterizacao_saida_preservada` (layout dos 3 docs: "1. A 2. B" / "x = 1 (1)" / "A") + `f339t_sem_set_nao_embrulha` (guarda) + helper `find_custom_in_styled` |
| **C** transporte | `fc7bfbf78` | `Styles::push_custom` (style.rs); wrap em `eval_markup` (snapshot dos 3 customs → embrulha o tail do escopo); 6 testes (3 wrap + 2 paridade chain≡assado + 1 transparência) |
| **M** consumidores | — (no-op) | **0 religados** — fatia aditiva; baking autoritativo até F-5. Confirmado pela caracterização verde |
| **F** f3s3 + linhagem | — | `f3s3` intacto (passa); 0 ficheiros novos; `eval.md` não descreve `eval_markup` (não fica stale); spec é §3a.8 (referenciada inline). Lint 0/0 |

**Mecanismo (C).** `eval_markup` tira snapshot dos customs `heading/equation/figure.numbering`
à entrada do corpo; ao detectar um `#set` local (custom novo vs snapshot), marca o
início do escopo; ao fim, embrulha `parts[start..]` num `Content::Styled(seq, Styles
com os customs)`. Só o `#set` muta `engine.styles` no loop (strong/emph/heading usam
`local_styles`). **Caminho duplo** chain↔assado com paridade testada (disciplina
anti-morto S5b/C1); gatilho de remoção = F-5.

**Transparência (medida):** os helpers de teste F-2 (`find_heading_numbered`,
`collect_*`) **já** recorriam `Content::Styled` — escritos a antecipar este wrap.
Zero asserção existente alterada.

## Verificação (gates)

- **build:** limpo a cada estágio. ✅
- **suíte:** C0 **2711 + 8 = 2719** (`typst-core`); workspace **3238 passed, 0 failed**.
  **Zero** asserções existentes alteradas; zero regressões (o wrapper é transparente
  a layout/introspect). ✅
- **lint:** `crystalline-lint .` = 0 violations, 0 warnings. ✅
- **lente** (`tekt-cargo-dsm@98d8f9e`, so-referência):

  | | P338 | P339 | §3a.8 previu |
  |---|---|---|---|
  | módulos | 219 | 219 | — |
  | arestas | 675 | **676** (+1) | delta de aresta no megaciclo ✅ |
  | ciclos | [90,4] | **[90,4]** | não é queda de ciclos ✅ |
  | content→elements | 66 | **66** | permanece 66 ✅ |
  | elem→elem | 0 | **0** | permanece 0 ✅ |

  O +1 é a aresta `eval→style` (uso de `Styles`), dentro do megaciclo. ✅
- **perf (C2):** o protótipo back-to-back na mesma sessão mediu o único custo novo
  — ≈2.8µs por wrap/passe, paridade limpa. O wrap só dispara com `#set` numbering
  presente; docs sem ele têm caminho de código **idêntico** (custo zero). Sem
  regressão no caminho quente dos nativos. ✅

## Contabilidade do F

- **F-4** (P338) ✅ · **F-realização fatia 1** (P339) ✅ **FECHADA**.
- Restam: **fatias 2/3** da F-realização (P340 composição/multi-passe + Trava-Q1;
  P341 show-set), depois **F-5** (de-bake — religa os 3 consumidores à chain e
  remove o baking) e **F-6** (folhas).
- **`content→elemento → 0`**: registado **fora da fila** F-1…F-6 + F-realização —
  decisão pendente do dono (marco pós-F-6 vs lacuna do plano). O baseline o espera.

## git log (commits isoláveis)

```
fc7bfbf78  Passo 339 — F-realização fatia 1: transporte (β1)
6b041efa5  Passo 339 — F-realização fatia 1: testes de caracterização
c8b5e12b9  Passo 339 — L0 §3a.8: fundação do transporte aditivo β1
```

(Caronas C0/C1 não geraram commit de código — C0 medição, C1 já coberta.)
