# Relatório P332 — Experimento da fronteira de extensão (spikes E1/E2/E3)

**Pré-condição**: P331 fechado — dossiê, inventários 1a/1b/1c, rede de
caracterização (+11, suíte 2708), lint 0, baseline 10× (P330). ✅ Verificado.
**Tipo**: diagnóstico **experimental** — spikes descartáveis + medição
comparativa. **Zero código de produto. Zero decisão de desenho.** A decisão da
fronteira é do dono, no checkpoint.
**Commits**: `requisito de extensibilidade (registro)` (já em P332 Parte 0) ·
`spikes E1/E2/E3` · `medição comparativa` — isoláveis.

---

## Estado por fase

| Parte | Estado | Entregável |
|-------|--------|-----------|
| **0 — registro do requisito** | ✅ (commit `a77fbaec5`) | DEBT 99.E §requisito + adendo no dossiê P331 |
| **1 — desenho dos candidatos** | ✅ | `f-experimento-extensao-passo-332.md` Parte 1 (E1/E2/E3) |
| **2 — spikes** | ✅ | `lab/spikes/f-extensao/{e1,e2,e3}/` (3 crates standalone) |
| **3 — medição comparativa** | ✅ | tabela única, 7 critérios, 3 colunas |
| **4 — relatório + checkpoint** | ✅ | leitura do executor (não vinculativa) + §Perguntas 1-7 |

---

## Parte 0 — onde mora o registro

`debt-stylechain-nao-materializada.md` §"Requisito de extensibilidade (P332)"
+ adendo em `f-dossie-opcoes-passo-331.md`. Extensibilidade **total** (elemento
de utilizador = cidadão pleno: `#set`/`#show`/`query`/render sem tocar o core);
dois públicos (Rust + autor typst); critérios do dono (atomização, separação de
camadas, economia para IA); Opção A **rejeitada**; B/C/D re-avaliadas **pelo
experimento**, não por argumento.

## Parte 1 — os candidatos (2 linhas cada)

- **E1 — Fronteira por trait**: `Content::Dynamic(Arc<dyn Element>)`, uma
  variante de extensão; despacho pelo `trait Element` **existente**; 65 nativos
  ficam monomórficos; props de utilizador em mapa aberto.
- **E2 — Type-erased à vanilla**: chain `enum Style{Property/Recipe}`,
  resolução por fallback `(ElementId,PropId)→Box<dyn Any>`; a forma mínima do
  vanilla para medir o custo real do `dyn`.
- **E3 — Registro aberto por kind**: `(kind_id, PropMap)` no `Content`, **sem
  `dyn`**; tabela `kind_id→ElementDescriptor` com `fn` ptrs; `PropKey` aberta,
  `enum Value` fechado.

## Parte 2 — os spikes (o que cada um demonstrou / stubou)

3 crates standalone (`[workspace]` próprio → fora do gate de lint; 0 imports do
produto/vanilla), cada uma com o **mesmo** `callout{body,title,tone}` de ponta
a ponta. Correm com `cd lab/spikes/f-extensao/eN && cargo run --release`.

- **E1** (`e1/`, callout 103 l / 78 LOC, total 606): demonstrou o elemento de
  utilizador no **mesmo `trait Element`** dos 65 nativos (precedente direto),
  nativos monomórficos. Stubou: `#show` `fn(&Content)->Content` 1 regra/1 passe;
  Registry explícito (pureza L1).
- **E2** (`e2/`, callout 103 l / 66 LOC efet., total 500): demonstrou a chain
  type-erased mínima + fallback walk-up real. Stubou: render→String; `#show`
  sobre string; chain `Vec` plana. **Risco medido**: `#set` de tipo errado →
  downcast `None` **silencioso**.
- **E3** (`e3/`, callout 67 l, total 457): demonstrou extensão por **dados + `fn`
  registradas** sem `dyn`; `PropKey` aberta (utilizador cunha `PropKey("tone")`
  sem tocar core). Stubou: sem eval/parse; mapa plano; render String. **Teto
  medido**: `enum Value` fechado — tipo de valor **novo** toca o core.

## Parte 3 — a tabela (resumo; íntegra no diagnóstico)

7 critérios × 3 spikes. Destaques:

- **Atomização**: os três zeram os ficheiros de core por elemento novo
  (alvo 0). Utilizador: E1 78 LOC · E2 66 · E3 67.
- **Custo-IA**: E1 menor raciocínio não-local + **precedente vivo** (mesmo
  trait dos 65); E2 maior (downcast silencioso); E3 médio (`fn` ptrs, `Value`
  fechado).
- **Performance** (relativa, **stub** — não comparável a 0.6518s/P330):
  native E1 `0.375` · E2 `0.202` · E3 `0.190` ms; mixed E1 `0.889` · E2 `0.541`
  · E3 `1.183` ms. **Facto robusto**: nos três, o caminho dinâmico **não onera
  os nativos** (satisfaz ADR-0029/0030 para os 65 migrados).
- **Escala M1~273**: E1/E3 props utilizador = 1 entrada de mapa; E3 tem teto
  no `Value` para tipo de valor novo; E2 273 pares `PropId↔tipo` não verificados.
- **Migração**: E1 mais barata (aditiva, 65 nativos ficam); E3 ~108 sites
  (eco D/B); E2 ~283 sites (de-bake, eco C).

## §Perguntas ao dono (numeradas)

1. Eixo de desempate (custo-IA/precedente → E1 · sem `dyn` → E3 · fidelidade
   estrutural → E2)?
2. `dyn` numa única variante `Content::Dynamic` (E1), nativos monomórficos —
   aceitável face a ADR-0029/0030?
3. Teto do `Value` (E3): tipo de valor novo a tocar o core é aceitável em troca
   de não ter `dyn`?
4. Downcast silencioso (E2): eliminatório ou mitigável?
5. Destino de `Set*`/`Styled`/3 folhas: absorver já (108 vs 283 sites) ou
   incremental?
6. `#show` na linguagem: spike-2 multi-regra antes de fixar, ou argumentado?
7. A fronteira escolhida vira F-D/F-B e o próximo passo é redigir o L0 do F?

---

## Verificação final

- **Produto intocado**: `git status` em `01_core/`/`02_shell/`/`03_infra/`/
  `04_wiring/` limpo (só `lab/spikes/` + 1 diagnóstico novos).
- **Suíte**: `cargo test -p typst-core --release` → **2708 passed; 0 failed**
  (3 ignored = `recursao_infinita_*`, caveat de stack abaixo).
- **Lint produto**: `crystalline-lint .` → **✓ No violations found**. Os spikes
  ficam **fora do gate** (workspace próprio) — exclusão registrada.
- **Caveat de stack**: `RUST_MIN_STACK=33554432` para a suíte (não-regressão,
  herdado do P331).
- **git log** (P332): `a77fbaec5` Parte 0 · `506ce8880` spikes · `516c566c9`
  medição.

## Próximo passo do roteiro

A **decisão da fronteira** — do dono, no checkpoint, com a tabela na mão
(responde §1-7 e escolhe E1/E2/E3 ou fusão). Só então: redigir o **L0 do F**
sob a fronteira escolhida (F-D lado elemento + F-B lado StyleChain/DEBT 99.E) e,
depois do hash humano, código. Este passo **não inicia** essa decisão.

## Fora de escopo (confirmado)

A decisão da fronteira; F-D/F-B (re-desenham depois da decisão); qualquer código
de produto; consertos B1/B2/B3 (registros do P331 mantidos, destino decide-se
com a fronteira); otimizações (o baseline P330 serve o antes/depois do F).
