# Recon/sub-spec do Marco G (P361) — desacoplar os 65 nativos (content→elements→0)

> **Tipo**: recon/sub-spec de DESENHO, read-only (probes via leitura/grep + 1 lente; zero código de
> produto, zero L0; suíte não re-rodada; árvore limpa). Mede a **superfície de dispatch** e desenha
> o Marco G; **não executa**. Termina numa **decisão FOUNDACIONAL proposta ao dono** — porque a
> medição revelou que o **modelo** do Marco G **não está decidido** (duas leituras das ADRs/plano em
> tensão) e que o plano o marca **fora desta branch**. Saída: este doc.
>
> **Veredito (medido):** Marco G é (1) **grande** (~640 match-arms; 48 de layout exigem nova infra),
> (2) **modelo não-decidido** (α `Content::Dynamic`-vtable, do plano, **vs** β PropMap-reificação,
> da ADR-0105 — e a ADR-0026 **rejeita** o vtable), (3) **plan-deferred** (`f-plano:119,141`: "FORA
> da fila F", "pós-F-6", "spec própria", "**não é trabalho desta branch**"). → **a decisão é o
> MODELO (ADR), não código.**

**Pré-condição**: P360 fechado; HEAD `3c213179a`; suíte **2737/0**; lente `98d8f9e` (66/0).

---

## 1 — Definição (a fonte: `f-plano-lotes-passo-333.md:119-141`)

Marco G = **"converter os 65 nativos a passarem pela fronteira de extensão E1 (que o F-1 construiu),
para o núcleo deixar de importar cada um"** (`:132-134`). Métrica: `edges(content → elements::*) →
0` (`:136`). **Status no plano:** "Marco G — pós-F-6, **FORA da fila F**" (`:119`); "migração grande,
com **spec própria** quando chegar a vez. **Não é trabalho desta branch** — o P346 nomeia o marco,
não o executa" (`:140-141`). **F-6 ainda não foi feito** (adiado no P360).

---

## 2 — Superfície de dispatch, medida (`file:line`)

Os 65 variantes `Content::*(Arc<*Elem>)` são matchados em **~640** arms no workspace. Por consumidor:

| Consumidor | # arms nativos | Delegável pela trait? | Crux |
|---|---|---|---|
| **content.rs** hub (`plain_text` 75, `is_empty` 36, `map_content` 80, `map_text` 77, `get_field`, `eq` 73) | ~320 | **MAIORIA SIM** — colapsam num arm `Content::Dynamic(e) => e.dyn_*()`; `eq` via o `dyn_eq` (downcast) **que já existe**; `get_field` via `dyn_get_field` (a trait já tem). `map_text` tem ~24 ramos terminais (folhas, ficam) | baixo — a trait cobre |
| **layout/mod.rs** `layout_content` | **48** | **NÃO** — ~90% **lêem campos concretos** (`h.level`, `e.caption`, `e.kind`, `ShapeKind`, `e.dx/dy/float`…) e chamam lógica por-elemento (`layout_heading`, `layout_figure`…). O arm `Content::Dynamic` (`:538`) é **trivial** (só `body`/`plain_text`) — **perde** a lógica por-elemento | **ALTO — o crux** |
| **introspect.rs** (`walk` 36, `extract_payload` 15) | ~51 | **MAIORIA SIM** — `extract_payload` é `e.to_payload()` puro; `walk` recursa no body / usa a trait; `populate_intr` é sobre `ElementPayload` (já abstrato). Precisa do arm `Content::Dynamic` no walk (placeholder existe, `:356/1095`) | médio |
| **export** (`03_infra/export/`) | **0** | N/A — opera sobre `FrameItem` (pós-layout), **desacoplado** | nenhum |
| **registro** (`element_registry.rs`, `dynamic.rs`) | — | só **nome→construtor**; **NÃO há tabela kind→handler**; `as_any` (downcast) usado só em `dyn_eq` | infra a construir |

**O crux (medido):** o `Content::Dynamic` layout arm é trivial (`mod.rs:538-550`: layouta só o
`body`). Os 48 arms nativos carregam a lógica real (numbering de heading, caption de figura, etc.),
**lendo campos concretos**. Colapsar para `Content::Dynamic` **perderia** essa lógica — a menos que
o layout passe a despachar por **kind → handler** (downcast via `as_any` ao `*Elem` concreto). Logo
o **acoplamento RELOCA** de `content.rs` para uma **tabela de handlers de layout** (que importa os
`*Elem`); `content→elements` baixa a 0, mas `layout→elements` fica ~48. **A métrica melhora; o
acoplamento muda de morada, não desaparece.**

---

## 3 — A tensão de MODELO (a decisão foundacional, medida nas ADRs)

`content→elements→0` é alcançável por **dois modelos diferentes** — e as fontes **discordam**:

- **Modelo α — `Content::Dynamic(Arc<dyn DynElement>)` (o que o plano diz, `:132-134`).** Migrar os
  65 nativos para o `dyn`. **Mas isto é um vtable** — exatamente o que a **ADR-0026 rejeitou de
  propósito** ("`Content` como enum linear — divergência intencional do original (vtable + proc
  macros)"; "Enum pode crescer linearmente **sem vtable**"). O E1/`Dynamic` foi adicionado para
  **extensão de utilizador** (1 variante), não para substituir os nativos. α aplicado aos nativos
  **revisaria a ADR-0026**.
- **Modelo β — reificação PropMap (o que a ADR-0105 diz, `:101-106`).** "O **enum fechado
  permanece** (a exaustividade do compilador é o que D preserva e a cláusula 3 protege em F). O que
  muda é a **morada da lógica por variante**: do corpo dos matches de `content.rs` (B) → módulos de
  elemento (D) → **descritores + PropMap** (F)." Aqui o nó vira `{ kind, props: PropMap }` genérico +
  tabela const de descritores — **sem vtable**, sem importar `*Elem` concretos → `content→elements→0`
  **sem** o `dyn`. É a "F-destino" da ADR-0105.

**Os dois dão `content→elements=0`, mas são arquiteturas opostas** (vtable dinâmico vs nó-de-dados
PropMap). A ADR-0026 favorece β (ou o enum tipado); o plano nomeia α. **Esta escolha é um ADR**, não
um detalhe de lote — e governa todo o resto (a tabela de handlers, o downcast vs PropMap, o trava).

---

## 4 — A Trava (ADR-0105 cláusula 3, obrigatória ANTES de qualquer código G)

ADR-0105 §3: "F **não começa** sem repor a verificação mecânica que o compilador deixa de dar.
Mecanismo: **um teste que varre a tabela const × os backends** (para cada `(kind, field)` declarado,
assertar handler) **ou** uma **regra do `crystalline-lint`**. Erro-de-compilação **não** vira
erro-de-runtime silencioso." → ao perder a exaustividade do `match` (que o `dyn`/PropMap esconde), o
Marco G **tem de** trazer este teste/lint (kind × {layout, introspect, show} handler). Sem ele, um
kind sem handler de layout passa a falha silenciosa em runtime.

---

## 5 — Custo + slicing (medido)

- **content.rs (~320 arms):** colapso grande mas **mecânico** (trait-delegável) — ~320 → ~6 arms
  `Dynamic`. Baixo risco (a trait já existe; `dyn_eq`/`dyn_get_field` já existem).
- **layout (48 arms):** o trabalho real — construir a **tabela kind→handler** + 48 handlers (downcast
  ou PropMap) + a Trava. **Alto.**
- **introspect (~51):** arm `Dynamic` no walk + trait. Médio.
- **export:** 0 (desacoplado). 
- **Total:** ~640 arms tocados; claramente **multi-lote** (o plano diz "spec própria"). Slicing
  natural: (i) modelo+Trava (ADR); (ii) content.rs colapso; (iii) layout handler-table; (iv)
  introspect Dynamic-walk. **Não cabe num lote.**

---

## 6 — Recomendação marcada (a DECISÃO é do dono)

**Marco G não é um lote — é uma decisão arquitetural + uma migração multi-lote, e o plano marca-o
fora desta branch.** O recon mediu três bloqueios à execução imediata:
1. **O modelo (α vtable vs β PropMap) não está decidido** e as ADRs (0026/0105) **discordam** do
   plano. Decidir isto é um **ADR novo** (ou uma emenda à 0026/0105), não um lote de código.
2. **A métrica `content→elements→0` reloca o acoplamento** (para a tabela de handlers de layout) em
   vez de o eliminar — o dono deve confirmar que a métrica é o objetivo real, ou se o objetivo é
   desacoplamento substantivo (que muda o desenho).
3. **O plano põe Marco G pós-F-6, fora da branch, spec própria** — e F-6 nem foi feito.

**Recomendo:** **não escrever código de Marco G agora.** O próximo passo é **fechar o MODELO num ADR**
(α revisando a 0026, **ou** β confirmando a F-destino da 0105 — com a Trava cl.3 desenhada), e só
então abrir a spec multi-lote. Em paralelo, se o dono quiser progresso concreto nesta branch, os
candidatos **dentro** da branch continuam **DEBT-59** (pequeno) ou **F-6** (limpeza). 

**Decisão proposta ao dono:** (i) **abrir o ADR do modelo do Marco G** (α/β) como próximo passo de
desenho, **adiando a execução** (alinhado ao plano: fora da branch / pós-F-6); ou (ii) se quiser, o
dono escolhe o modelo aqui e eu redijo o ADR + a spec. Nenhum código/L0 tocado; suíte 2737; árvore
limpa.
