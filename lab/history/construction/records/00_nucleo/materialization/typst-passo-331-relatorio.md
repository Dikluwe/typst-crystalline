# Relatório P331 — Diagnóstico do F (execução autônoma, 3 fases)

**Pré-condição**: P330 fechado — roteiro de lotes encerrado (65 migradas),
baseline 10× registrado, lint 0, suíte 2697. ✅ Verificado.
**Tipo**: diagnóstico-primeiro do F / DEBT 99.E — **zero decisão de desenho,
zero código de produto** (só teste novo na Fase 2). A decisão do F é do dono.
**Commits**: `F fase 1 (inventário)`, `F fase 2 (caracterização)`, `F fase 3
(dossiê)` — isoláveis.

---

## Estado por fase

| Fase | Estado | Entregável | Progresso |
|------|--------|-----------|-----------|
| **1 — inventário** | ✅ completa | `f-inventario-1{a,b,c}-passo-331.md` (1142 l) | 3 frentes paralelas (agentes) |
| **2 — caracterização** | ✅ completa | +11 testes em `mod f_caracterizacao_estilo` | suíte 2697→2708 |
| **3 — dossiê** | ✅ completa | `f-dossie-opcoes-passo-331.md` | 4 opções + 10 perguntas + 3 bugs |

Progresso retomável em `00_nucleo/diagnosticos/f-progresso-passo-331.md`.
**P331 completo** — as 3 fases fecharam dentro da sessão.

---

## Fase 1 — inventário (3 frentes paralelas, só leitura)

Dividida em 1a (cristalino) ‖ 1b (vanilla, `lab/`) ‖ 1c (custo), executadas por
3 agentes concorrentes; cada um escreveu o seu ficheiro com `file:line`.

**Os 3–5 factos mais relevantes para a decisão:**

1. **3 mecanismos de estilo desconexos** (1a): `#set text` muta StyleChain em
   eval e **assa** `TextStyle` em `Content::Text` (chain descartada); `*bold*` →
   `Styled(Box, Styles)` re-resolvido numa **2ª StyleChain** no Layouter; as 4
   `Set*` são marcadores opacos por **4 canais distintos** (2 via Introspector,
   `SetPage` muta `page_config`, `SetFigureNumbering` assa em `Figure`).
2. **Alvo pequeno e fechado** (1c): **10 propriedades** de estilo, 1:1
   `TextStyle` ↔ `enum Style` ↔ `StyleChain`; ponto único de merge
   (`layout/mod.rs:587–602`). Sem `PropMap`/vtable — enum fechado consciente
   (`style.rs:18-19`).
3. **Custo do estado misto** (1c): **~50 linhas** de arms próprios (teto 68);
   `content.rs` 4960. Larguras Σ=877 (`Sequence` 220, `Empty` 164 dominam).
4. **Vanilla** (1b): chain lazy type-erased (`Property`/`Recipe`/`Revocation`);
   props de texto `#[ghost]` (só na chain, não na folha); set propaga por
   **fallback** instance→chain→default + `materialize`; `PartialEq` por ponteiro
   (comemo). Contrato de fidelidade **C1–C8** (comportamental).
5. **`world_types::Styles(())` é stub morto**; o real é `style::Styles`.

## Fase 2 — rede de caracterização (só testes)

**+11 testes** em `mod f_caracterizacao_estilo` (`engine/layout/tests.rs`), todos
sobre **saída observável** (`layout().plain_text()`, `FrameItem::Text.style`) —
não representação interna (o F muda a representação; a rede protege o
comportamento). Suíte **2697 → 2708** (+11), 0 failed; lint 0; zero asserção
existente alterada; só ficheiro de teste tocado.

**Cobre:** SetHeadingNumbering (com/sem prefixo); SetFigureNumbering (caption →
"Figura 1"); SetEquationNumbering (estado atual); SetPage (dims 123×456
propagam); `Styled` bold/italic aplica ao corpo e **não vaza** para o irmão;
`Text` plain_text idêntico com/sem `Styled` mas layout difere; heading numerado
**dentro de columns** (set atravessa o contentor migrado).

**§bugs** (caracterizados, não consertados): `SetEquationNumbering` sem produtor
eval (efeito não caracterizável via layout puro); `Styled` sem arm `is_empty`
(cai em `_ => false`); `world_types::Styles(())` stub morto.

## Fase 3 — dossiê de opções (decisão NÃO tomada)

`f-dossie-opcoes-passo-331.md` — **4 opções** (resumo de 2 linhas cada):

- **A — Declarar e congelar**: não unifica; formaliza os 3 mecanismos como
  desenho. Custo ~0, 0 lotes. Deixa o estado misto permanente.
- **B — PropMap tipado fechado**: chain única (10 campos), `Set*` → entradas,
  `#set text` deixa de assar. ~283 sites, 2–3 lotes. Paga o de-bake.
- **C — StyleChain à vanilla** (type-erased + show/recipes/`#[elem]` 3os):
  máxima fidelidade estrutural. >283 sites + infra, 4+ lotes. ⚠️ tensão com o
  critério do dono (fidelidade estrutural, não comportamental).
- **D — Híbrido mínimo**: unifica só os 4 canais das `Set*` (~108 sites, 1–2
  lotes); `Styled`/folhas/`#show` ficam para um F-2.

Há uma secção **"Leitura do executor"** (marcada como tal, não vinculativa): D e
B alinham mais com o critério do dono; C só se `#show` virar requisito. **Sem
recomendação final obrigatória.**

**§Perguntas ao dono**: 10 (Q1 escopo · Q2 de-bake `#set text` · Q3
`SetEquationNumbering` lacuna · Q4 `SetPage` ×2 produtores · Q5 `#show`/
`Value::Styles` · Q6 extensibilidade 3os · Q7 comemo/igualdade-ponteiro · Q8
`Styled.is_empty` · Q9 contagem 50-vs-68 · Q10 props 10-vs-2 efetivas).

---

## Próximo passo do roteiro

A **decisão do F** — do dono, no checkpoint, com este dossiê na mão (responde
Q1–Q10 e escolhe A/B/C/D). Este passo **não a inicia**: deixa o material pronto
— inventário factual (1142 l), rede de regressão (+11 testes), 4 opções
dimensionadas pelo preditor, baseline 10× para o antes/depois.

## Fora de escopo (confirmado)

A decisão do F; qualquer mudança de produto em `Styled`/`Set*`/`Text`/`MathText`/
`MathIdent`; conserto dos bugs §bugs (registrar, não consertar); otimizações
(o baseline serve o antes/depois do F, não mexer agora). Caveat: stack default
em `recursao_infinita_*` — não é regressão (`RUST_MIN_STACK=33554432`).
