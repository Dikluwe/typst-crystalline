# Relatório P361 — Recon/sub-spec do Marco G (desacoplar os 65 nativos)

> **Tipo**: recon/sub-spec de DESENHO, read-only (probes via leitura/grep + 1 lente; zero código de
> produto, zero L0; suíte não re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). Mede a superfície
> de dispatch e desenha o Marco G; **não executa**. Saída detalhada:
> `00_nucleo/diagnosticos/f-recon-marco-g-passo-361.md`.
>
> **Veredito.** Marco G **não é um lote** — é (1) **grande** (~640 match-arms; 48 de layout exigem
> infra nova), (2) **modelo não-decidido** (α `Content::Dynamic`-vtable do plano **vs** β
> PropMap-reificação da ADR-0105; a ADR-0026 **rejeita** o vtable), (3) **plan-deferred** (plano:
> "FORA da fila F", "pós-F-6", "spec própria", "não é trabalho desta branch"; F-6 nem foi feito).
> **A próxima decisão é o MODELO (um ADR), não código.** **Decisão do dono: em aberto.**

**HEAD**: `3c213179a` (pós-P360). **Branch**: Tekt. Lente `98d8f9e` (66/0). Suíte 2737/0.

---

## 1 — Definição (a fonte: `f-plano-lotes-passo-333.md:119-141`)

Marco G = "converter os 65 nativos a passarem pela fronteira E1 (F-1), para o núcleo deixar de
importar cada um" (`:132-134`); métrica `edges(content → elements::*) → 0` (`:136`). **Status no
plano:** "pós-F-6, **FORA da fila F**" (`:119`); "migração grande, **spec própria**; **não é trabalho
desta branch**" (`:140-141`). **F-6 não foi feito** (adiado P360).

## 2 — Superfície de dispatch, medida (`file:line`)

~640 arms `Content::*(Arc<*Elem>)` no workspace. Por consumidor:

| Consumidor | # arms | Delegável pela trait? | Crux |
|---|---|---|---|
| **content.rs** hub (`plain_text` 75, `is_empty` 36, `map_content` 80, `map_text` 77, `get_field`, `eq` 73) | ~320 | **MAIORIA SIM** — colapsa em `Content::Dynamic(e) => e.dyn_*()`; `eq` via `dyn_eq` (downcast) **já existe**; `get_field` via `dyn_get_field`; `map_text` ~24 ramos terminais (folhas, ficam) | baixo |
| **layout/mod.rs** `layout_content` | **48** | **NÃO** — ~90% lêem campos concretos (`h.level`, `e.caption`, `e.kind`, `ShapeKind`, `e.dx/float`…); o arm `Content::Dynamic` (`:538`) é **trivial** (só `body`) → colapsar **perde** a lógica por-elemento | **ALTO — o crux** |
| **introspect.rs** (`walk` 36, `extract_payload` 15) | ~51 | **MAIORIA SIM** — `extract_payload` = `e.to_payload()`; `walk` recursa no body; `populate_intr` sobre `ElementPayload`. Precisa do arm `Dynamic` no walk (placeholder existe, `:356/1095`) | médio |
| **export** (`03_infra/export/`) | **0** | N/A — opera sobre `FrameItem` (pós-layout), **desacoplado** | nenhum |
| **registro** (`element_registry.rs`/`dynamic.rs`) | — | só **nome→construtor**; **sem tabela kind→handler**; `as_any` usado só em `dyn_eq` | infra a construir |

**O crux:** o `Content::Dynamic` layout arm é trivial; os 48 arms nativos carregam a lógica real
(numbering, caption…), lendo campos concretos. Colapsar **perderia** isso a menos que o layout
despache por **kind→handler** (downcast ao `*Elem`). → **o acoplamento RELOCA de `content.rs` para
uma tabela de handlers de layout** (que importa os `*Elem`); `content→elements→0` melhora a métrica,
**o acoplamento muda de morada, não some**.

## 3 — A tensão de MODELO (a decisão foundacional)

`content→elements→0` é alcançável por dois modelos **opostos**, e as fontes **discordam**:
- **α — `Content::Dynamic(Arc<dyn DynElement>)`** (plano `:132`): migrar os 65 nativos para o `dyn`.
  **É um vtable** — o que a **ADR-0026 rejeitou de propósito** ("enum linear … **sem vtable**"). α
  **revisaria a ADR-0026**.
- **β — reificação PropMap** (ADR-0105 `:101-106`): "o **enum fechado permanece**; muda a **morada da
  lógica** … → **descritores + PropMap** (F-destino)". Nó vira `{ kind, props }` + tabela const,
  **sem vtable**, sem importar `*Elem` → `content→elements→0`. Alinha com a ADR-0026.

**Esta escolha é um ADR**, governa a tabela de handlers (downcast α vs PropMap β) e o trava.

## 4 — A Trava (ADR-0105 cl.3) — obrigatória antes de qualquer código G

"F não começa sem repor a verificação mecânica que o compilador deixa de dar: **teste que varre a
tabela const × os backends** (cada `(kind, field)` → handler) **ou** regra do `crystalline-lint`."
Ao perder a exaustividade do `match`, Marco G **tem de** trazer este teste/lint (kind × {layout,
introspect, show}). Sem ele, kind-sem-handler vira falha silenciosa em runtime.

## 5 — Custo + slicing

content.rs (~320) = colapso mecânico (trait-delegável), baixo risco. layout (48) = o trabalho real
(tabela kind→handler + 48 handlers + Trava), **alto**. introspect (~51) = arm Dynamic + trait, médio.
export = 0. **~640 arms tocados → multi-lote** ("spec própria"). Slicing natural: (i) modelo+Trava
(ADR); (ii) content.rs colapso; (iii) layout handler-table; (iv) introspect Dynamic-walk. **Não cabe
num lote.**

## 6 — Recomendação marcada (a decisão é do dono — EM ABERTO)

**Não escrever código de Marco G agora.** Três bloqueios medidos: o **modelo (α/β) não está
decidido** e as ADRs discordam do plano (é um ADR, não um lote); a métrica `content→elements→0`
**reloca** o acoplamento (o dono deve confirmar se a métrica é o objetivo, ou se é desacoplamento
substantivo); o plano põe Marco G **fora desta branch / pós-F-6**.

**Recomendado:** **adiar o Marco G** (alinhado ao plano e à ADR-0107 — não erguer infra grande sem o
modelo decidido); registrar como **decisão-de-modelo pendente** (ADR α/β futuro). Avançar **nesta
branch** com um candidato real — **DEBT-59** (flag CLI, pequeno/isolado) ou **F-6** (limpeza das 3
folhas). **Alternativa:** o dono escolhe o modelo (α/β) e eu redijo o ADR + a Trava + a spec
multi-lote. **A escolha permanece do dono.**

**Estado:** nenhum código/L0 tocado; suíte 2737; lente 66/0; árvore limpa. Marco G **não iniciado**
(decisão de modelo pendente).

## Fora de escopo (confirmado)

A execução do Marco G (multi-lote, após o ADR de modelo); F-5/F-6 (adiados P360); DEBT-59
(disponível in-branch); qualquer toque no α / `morph_canon` / `==` / flag ou na F-realização fechada.
