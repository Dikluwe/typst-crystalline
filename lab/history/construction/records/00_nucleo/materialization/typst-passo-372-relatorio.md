# Relatório P372 — F-5b fatia (2): BLOQUEADO na execução (transporte P368); código revertido

> **Desfecho.** A fatia 2 (de-bake do render `#set text` pelo `custom`) foi **implementada** e a
> suíte `typst-core` ficou **verde (2747, inclusive o gate do α / caso 2 + a rede +11)** — MAS a
> execução **mediu** que o desenho está **bloqueado** por uma limitação do **transporte do P368**,
> e que a premissa "inerte em layout" do L0 era **falsa**. **Código revertido** (suíte volta ao
> 2747 do P371); o L0 (`§3a.13` ⛔) + este relatório registram a medição. **A decisão é do dono.**

**HEAD**: pós-P371 (e6da8f002). **Branch**: Tekt.

---

## O que foi implementado (e reverteu)

- `#set text(<campo>)` → canal `custom` `"text.<campo>"` (`eval_set_rule`); `#set par(leading)`
  idem. `Content::Text(EcoString, TextStyle)` → `Content::Text(EcoString)` (campo removido, cascata
  ~90 sítios). Layout `node_render` decodifica o render do `custom`. `morph_canon` inalterado
  (`custom`-Styled desce → `#set text X == X`, transparente). **Suíte `typst-core`: 2747, 0 falhas
  — α/caso 2 + rede +11 verdes.** O de-bake **funciona** no nível `typst-core`.

---

## Por que reverteu — 3 medições (a fonte/oráculo venceu o desenho)

1. **"Inerte em layout" era FALSO (medido).** O L0 §3a.13 assumiu que weight/tracking/leading/
   lang/font eram inertes (capturados, não renderizados). **Falso:** `tracking`/`leading` **são
   consumidos** — o frame guarda o `TextStyle` efetivo e os testes
   `set_text_tracking_propaga_ao_frame_passo_137` / `layout_leading_afecta_posicao_linha_passo_138`
   o checam. E o `leading` vem por **`#set par`** (não `text`), que **também** assava no
   `Content::Text`. ∴ o `node_render` precisou decodificar **todos** os campos do `custom` (incl. o
   round-trip `Value↔Lang/FontList`), e o `#set par(leading)` migrou para o `custom`. (Feito; a
   suíte typst-core passou com isto.)

2. **⛔ BLOQUEIO: o transporte do P368 é single-wrap-final-collapse — errado para `#set`
   sequenciais da mesma chave.** Medido em `03_infra`
   (`font_wiring_segunda_font_diferente_ambas_embebidas`): a fonte
   `#set text(font: A)\nOlá\n#set text(font: B)\nAdeus` produz **um** `Content::Styled` em volta da
   **cauda inteira** carregando o `custom` **final** colapsado (`font: B`) → "Olá" **e** "Adeus"
   ficam font **B** → só **1 de 2** fonts embebidas (o teste exige 2). O numbering **nunca expôs**
   isto (é por-elemento; mesma-chave-2× é raro/inerte). O `#set text` **exige** o transporte
   **aninhar** os wraps (cada `#set` embrulha a sua própria cauda, fiel ao escopo léxico) ou
   carregar **por-segmento**. **É uma correção do P368 — pré-requisito da fatia 2, fora do escopo
   medido.**

3. **Snapshots PDF golden (`p307b`, 5) regridem** (`03_infra`): bytes mudaram — ao menos em parte
   pelo bug do font (#2); golden byte-a-byte é frágil. (Não diagnosticado a fundo — depende de #2.)

---

## Gates

```
typst-core (RUST_MIN_STACK=33554432): 2747 (verde com o de-bake) — MAS 03_infra: 7 falhas
  (font_wiring ×2 + p307b ×5) → de-bake NÃO content-preserving no pipeline completo.
∴ REVERTIDO. Pós-revert: typst-core 2747, lint 0/0, sem .rs funcional (só L0 + hash-sync).
INTACTOS: α/caso 2, rede +11, fatia 1 (strong/emph), numbering, #set user-props — o revert
  restaura o estado P371.
```

---

## Recomendação medida (decisão do dono)

A fatia 2 está **bloqueada na limitação do transporte (P368 single-wrap)**, que o `#set text`
(múltiplos same-key num corpo) expõe. **Recomendo:**
1. **Primeiro um lote de correção do transporte** — `eval_markup` passa a **aninhar** os wraps
   (cada `#set` embrulha a sua própria cauda, refletindo o escopo léxico) ou carregar por-segmento.
   Conserto fiel ao vanilla (`styled_with_map` por-`#set`); beneficia numbering + user-props +
   render. Tem o seu **gate do α** (não pode reabrir o caso 2).
2. **Depois a fatia 2** (de-bake do render) **sobre** o transporte corrigido — o resto do desenho
   (`#set text` → custom, layout lê custom, remove o `TextStyle` assado) já está medido e
   funciona no `typst-core`.

Alternativa: registrar o `TextStyle` render como **divergência consciente** (DEBT-61), deixando o
item (1) da auditoria em 3,5/4 (o render do `#set text` permanece assado).

**Tocados:** `f_fronteira_e1.md` (§3a.13 + ⛔ blocker) + backings (hash-sync); este relatório.
**Nenhum `.rs` funcional** (revertido). **Termina aqui — não emendo o seguinte (Trava 5).**

## Fora de escopo

A correção do transporte (lote próprio, recomendado); o **Marco G**; DEBT-59; DEBT-60.
