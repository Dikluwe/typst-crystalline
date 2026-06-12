# Progresso P331 — Diagnóstico do F (execução autônoma)

**Retomável**: se a sessão morrer, ler este ficheiro para saber onde parar.
**Regra**: bloqueio não para o passo → vira pergunta no dossiê (Fase 3).

## Nomes reais descobertos (orientação)

- `entities/style.rs`: `enum Style` (l.33), `struct Styles` (l.131).
- `entities/style_chain.rs`: `struct StyleChain(Option<Arc<StyleNode>>)` (l.95).
- `entities/world_types.rs:158`: `pub struct Styles(())` — stub a investigar (1a).
- **Não há `PropMap`** por esse nome.

## Larguras (grep refeito P331)

`SetHeadingNumbering`66 · `SetEquationNumbering`20 · `SetPage`13 ·
`SetFigureNumbering`9 · `Styled`82 · `Text`93 · `MathText`47 · `MathIdent`108.

## Estado por fase

- [x] **Fase 1 — inventário** (3 frentes paralelas) ✅ — 1142 linhas, lint 0
  - [x] 1a cristalino → `f-inventario-1a-cristalino.md` (393 l)
  - [x] 1b vanilla → `f-inventario-1b-vanilla.md` (443 l)
  - [x] 1c custo → `f-inventario-1c-custo.md` (306 l)
- [x] **Fase 2 — rede de caracterização** (testes) ✅ — **+11** testes
  (`mod f_caracterizacao_estilo` em `layout/tests.rs`), suíte 2697→2708, lint 0
- [x] **Fase 3 — dossiê de opções** ✅ → `f-dossie-opcoes-passo-331.md`
  (4 opções A/B/C/D + §10 perguntas + §3 bugs). **P331 COMPLETO.**

## Achados-chave da Fase 1 (para a Fase 3)

- **3 mecanismos de estilo desconexos** (1a): `#set text` muta StyleChain em
  eval e **assa** `TextStyle` em `Content::Text` (chain descartada); `*bold*` →
  `Styled(Box, Styles)` re-resolvido numa **2ª StyleChain** no Layouter; as 4
  `Set*` são marcadores opacos que **não tocam `Styles`**, por 4 canais
  diferentes (2 via Introspector, SetPage muta `page_config`, SetFigureNumbering
  no-op assado em Figure). `world_types Styles(())` é **stub morto**.
- **Vanilla** (1b): chain lazy type-erased (`Property`/`Recipe`/`Revocation`
  numa lista); props de texto são `#[ghost]` (só na chain, não na folha); set
  propaga por **fallback** instance→chain→default + `materialize`; `PartialEq`
  por ponteiro (comemo).
- **Custo** (1c): **50 linhas** de arms próprios (teto 68); larguras Σ=877
  (Sequence 220, Empty 164 maiores); **10 propriedades de estilo distintas**
  (1:1 `TextStyle` ↔ `Style` ↔ `StyleChain`); ponto único de merge
  `mod.rs:587–602`. `content.rs` 4960.

## Log

- P331 iniciado. Precondição ✅ (lint 0, suíte 2697). Orientação feita.
- Fase 1 ✅ (3 frentes paralelas, agentes). Commit "F fase 1".
- Fase 2 ✅ (+11 testes de caracterização, observable-output). Commit "F fase 2".
  - Achados p/ §bugs: `SetEquationNumbering` sem produtor eval — efeito de
    numeração não caracterizável via `layout` puro (Introspector-dependente);
    o teste só fixa que o corpo renderiza.
- Próximo: Fase 3 (dossiê de opções).
