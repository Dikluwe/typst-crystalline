# Relatório P354 — Recon caso 1 (composição): reconciliar com α ou divergência declarada

> **Tipo**: recon read-only (zero código de produto, zero L0). Probes descartáveis compiladas e
> **revertidas**; suíte **2733/0** antes/depois; `RUST_MIN_STACK=33554432`. O passo termina numa
> **decisão proposta ao dono** (como P337/P347d). Saída: `00_nucleo/diagnosticos/f-recon-caso1-passo-354.md`.
>
> **⚠️ ADENDO DE FECHO (ver §6):** a decisão deste passo (rumo **B**, escolhida pelo dono) foi
> **revertida no P355** — na Trava de selagem da ADR-0109, a classificação "composição = mecânica"
> revelou-se **inferida**, e a medição da fonte a **refutou** (composição same-kind é **língua**
> documentada). Este relatório regista o P354 como correu; o veredito vigente é o do P355.

**HEAD**: pós-P353. **Branch**: Tekt. **Pré-condição**: P352 fechado (colisão caso-1/α medida na
Fase A §4). Lint 0/0.

---

## 1 — O modelo do vanilla, medido (`file:line`)

- **Iteração innermost-first, 1 func/passe**: `lab/typst-realize/src/lib.rs:449` (`for (r, recipe)
  in styles.recipes().enumerate()`), `foundations/styles.rs:835` (`next_back` → mais-recente
  primeiro); demais func saltadas no passe (`lib.rs:467-469`).
- **Guard por-instância (`RecipeIndex`)**: `lib.rs:472-474` (`if elem.is_guarded(index) continue`);
  bitset de lifecycle no elemento empacotado (`content/mod.rs:148-156`); input guardado ao aplicar
  (`lib.rs:363-367`, `recipe.apply(.., output.guarded(guard))`).
- **Reconciliação vanilla**: o **mesmo** guard por-instância serve **dois papéis** — (i) composição
  (cada recipe uma vez → as outras aplicam) e (ii) terminação de recursão (input guardado é saltado;
  instância nova re-aplica → erro no teto). Os dois são o mesmo maquinário.

## 2 — A decisão P347d/P348 (o "GEROU")

`typst-passo-348-relatorio.md:23-30,126-140` + L0 §3a.7-bis: o P347d/P348 recusou o **papel (ii)** (a
terminação por **identidade de instância** é mecânica gerada — commit vanilla #3327; Revocation
interna) e adotou o **ponto-fixo morfológico (α)**. O P348 considerou a composição **cross-kind
cascata** já resolvida (interceção aninhada) e tratou só a recursão. **O papel (i) (composição
same-kind acumulativa) nunca foi construído** (caso 1 adiado), **não foi rejeitado**.

## 3 — O crystalline hoje (medido + probes revertidas)

- α loop: `rules/eval/rules.rs:97-248` — aplica a 1ª regra declarada que casa (`:124-187`), re-feed,
  ponto-fixo morfológico (`:210-213`), teto-64.
- **Funciona**: cascata cross-kind (interceção aninhada; `show_rule_encadeamento_duas_regras`);
  recursão same-rule (α; `p348_show_recursao_converge_para_ponto_fixo`).
- **Não funciona (B1, probes P354)**: same-kind multi-regra — output muda de tipo → só a 1ª
  declarada ("AA orig"); ordem → 1ª declarada vence ("R1FIRST"); output fica do mesmo kind → encadeia
  via interceção + converge a UM output ("BB"), não acumula.

## 4 — As três saídas (custo, `file:line`)

- **A1** (guard por-`RuleId`-uma-vez): composição exata, mas **regride o caso 2** (a regra guardada
  não re-aplica ao output recursivo → a→b→c pára em b). **Inviável.**
- **A2** (guard por-`(RuleId, morph_canon)`): composição + preserva o α, sem reabrir o "GEROU"; mas
  **diverge na ordem** das same-kind não-comutativas. Toca só `rules.rs:97-248`.
- **A3** (guard por-instância do vanilla): paridade **exata**, mas **reabre o α/caso 2 fechado**.
- **(B)** divergência declarada + ordem innermost-first: zero risco ao α; nomeia a divergência.

## 5 — Recomendação (P354) + decisão do dono

Recomendação do agente no P354: **(B)** (divergência declarada). Razões dadas: ADR-0107 (a
acumulação seria mecânica de realização); demanda não medida (0 testes de B1). **Decisão do dono
(P354): (B)**, registrada em `f-recon-caso1-passo-354.md §7`.

## 6 — ADENDO DE FECHO: a decisão (B) foi REVERTIDA no P355

Na Trava de selagem da ADR-0109 (P355), o dono aplicou a ADR-0108 à própria ADR e exigiu a medição
da classificação **língua-vs-mecânica**. Medido: a recomendação do P354 (e da ADR-0109) classificava
a composição como **mecânica** por **inferência** — eu medira o *mecanismo* (o guard produz a
acumulação) mas **não a intenção**. A fonte refutou: a referência do Typst
(`docs/reference/language/styling.md`, secção *Show rules*) **promove** múltiplas regras same-kind
("keeping styling composable", "good practice"; exemplo com 4× `#show heading`). **Composição
same-kind é LÍNGUA, não mecânica** — o erro inverso-P347d. **Veredito vigente (P355): reconciliar,
não declarar divergência** (`f-recon-composicao-passo-355.md`). A recomendação **(B)** deste P354
fica **superseded**; o que sobrevive do P354 é a **medição do mecanismo** (A1/A2/A3, o guard
por-instância) — válida e reutilizada pelo P355 para dimensionar o conserto.

**Estado**: nenhum código/L0 tocado no P354; suíte 2733/0; árvore limpa.
