# Passo 350c — relatório: flag de erro completo, capacidade interna (C-com-origem intermédio)

> **Resultado.** A capacidade interna da flag de "erro completo" está **feita** (forma
> C-com-origem intermédio, P350b). Sob a flag, o erro de recursão de `#show` ganha um **3º
> hint** classificando em **DOIS rótulos** — **cíclico** (uma morfologia do caminho repetiu,
> fato medido pelo `==`/`morph_canon` do P345) e **não-convergente** (teto sem repetição). O
> **3º rótulo "converge-fundo" foi cortado** (decisão do dono; ADR-0108 regra 4 — afirmar só
> o medido). A **mensagem base + os 2 hints do vanilla são byte-idênticos** sem a flag. A
> **assinatura pública de L3 (`compile_to_pdf_bytes`) NÃO mudou** (`pipeline.rs` intocado). A
> flag entra por um **sibling `eval_with_full_error`** (`eval()` segue delegando com `false`).
> CLI + fio `RunIntent`→L3-interno = **débito (DEBT-59)**. Gates: lint **0/0**, lib **2729**
> (2726 + 3), workspace **3248**.

## C0 / C1
- **C0**: HEAD `046ab3a91` (pós-P350b), árvore limpa, lint 0/0, suíte 2726/3245. Bate.
- **C1**: `§3a.7-bis` movido de "flag adiada" (P348) → "**capacidade interna FEITA (P350c)**;
  parsing CLI + fio `RunIntent`→L3 = débito".

## Fase de código — por estágio

### Estágio Canal — origem → leitura, sem tocar a API pública de L3
- **Origem (L2):** `full_error: bool` em `RunIntent` (`02_shell/src/cli.rs`, ao lado de
  `colored`); `cli::parse()` fixa `false` (o `Arg --full-error` é débito). `main.rs` (L4)
  destrutura com `..` (ignora `full_error` — o fio a L3 é débito).
- **Canal interno (L1):** `eval()` foi convertido em **delegado** — mantém a assinatura
  estável para os ~165 callers (produção L3 + 148 testes via wrappers + 17 diretos) — e
  chama `eval_with_full_error(…, false)`. A **nova sibling** `eval_with_full_error(…,
  full_error: bool)` carrega o body e faz `ctx.full_error = full_error`. **`pipeline.rs` (L3)
  não mudou** — continua a chamar `eval()` (false). (Diff de `pipeline.rs` vs P349: vazio.)
- **Leitura (L1):** `EvalContext.full_error` (campo novo, default `false`) lido no ponto do
  erro, em `apply_all` (`rules/eval/rules.rs`).
- **Prova:** `git diff … -- 03_infra/src/pipeline.rs` = vazio → assinatura **pública** de L3
  intacta. `eval()` mantém a mesma assinatura (delegado).

### Estágio Histórico+Classificação — sob a flag, dois rótulos
No loop de revisitação (`apply_all`): `let full_error = ctx.full_error;` (Copy, lido uma vez).
**Se** `full_error`: mantém `history: Vec<Content>` das `morph_canon` do caminho; a cada
output, se a morfologia **já está** no histórico → `cycle = true` (usa o `==` do P345 para
ler, não o altera). **Se desligada (default): `history` fica vazio** — nada é alocado/
computado (`if full_error`), caminho quente intacto. No erro do teto, **se** a flag está
ligada, acrescenta o **3º hint**: `cycle` → "recursão CÍCLICA"; senão → "recursão
NÃO-CONVERGENTE". A terminação continua **no teto** (timing idêntico ao flag-off — não corta
cedo); só o hint muda. Mensagem base + 2 hints do vanilla permanecem.

**Por que 2 rótulos, não 3:** distinguir *divergente* de *converge-fundo* exigiria adivinhar
o futuro **pós-corte** (uma recursão que estabilizaria mais fundo é indistinguível, no teto,
de uma que diverge). Afirmar isso seria confiança falsa → **ADR-0108 regra 4** (afirmar só o
medido). Cíclico é **fato** (repetiu); não-convergente é **fato** (teto sem repetição).

### Estágio Teste (`rules/eval/tests.rs`)
3 testes novos (via `eval_for_test_full_error`, helper que chama `eval_with_full_error(…,
true)`):
- `p350c_flag_off_mensagem_byte_identica_ao_vanilla` — flag off (eval normal) → erro com
  **exatamente 2 hints** (sem 3º). Prova de que a flag é aditiva.
- `p350c_flag_on_ciclo_classifica_ciclico` — `==[a]→[= b]; else [= a]` · `= a` → 3º hint
  "CÍCLICA"; base + 2 hints intactos.
- `p350c_flag_on_nao_convergente_classifica` — `#show heading: it => [= #it.body x]` · `= a`
  (cresce sem repetir) → 3º hint "NÃO-CONVERGENTE".

### Estágio L0 + débito
- **L0**: `f_fronteira_e1.md §3a.7-bis` (capacidade feita, 2 rótulos + por que não 3, canal,
  débito), `rules/eval.md` (bullet da flag), `shell/cli.md` (`RunIntent.full_error` + débito).
- **DEBT-59** (novo, `00_nucleo/DEBT.md`): exposição CLI (`--full-error`) + fio
  `RunIntent`→L1 pelo interior de L3; critério de conclusão registrado. A assinatura pública
  de L3 fica intacta — **decisão, não débito**.

### Estágio F — linhagem
`@updated 2026-06-17` em `mod.rs`/`rules.rs`/`tests.rs`/`cli.rs`/`main.rs`; `--fix-hashes`
(eval.md/f_fronteira_e1.md/cli.md mudaram → re-sync dos headers); V5/V7 limpas.

## Aceitação (observável)
- **flag-off (default):** mensagem de erro de recursão **byte-idêntica ao vanilla** — base +
  exatamente 2 hints (`p350c_flag_off…` assere `hints.len() == 2`). O padrão não muda. ✓
- **flag-on:** 3º hint com o rótulo certo — "CÍCLICA" ou "NÃO-CONVERGENTE"; base + 2 hints
  intactos. ✓
- **DOIS rótulos, não três:** nenhum rótulo afirma o futuro pós-corte. ✓
- **caminho quente:** o `Vec` do histórico **não** é alocado com a flag off (`if full_error`
  — por construção). ✓

## Verificação (gates)
```
build: limpo. lint: crystalline-lint . = 0/0.
suíte: lib 2729 (2726 + 3), workspace 3248 (3245 + 3). 0 quebrados.
assinatura pública L3 intacta: pipeline.rs UNCHANGED (diff vazio vs P349). Confirmado.
== / morph_canon intactos (P345): a detecção de ciclo os USA (history + `==`), não os altera.
dois sistemas: derive(PartialEq) do Rust inalterado.
débito: DEBT-59 (parsing CLI + fio RunIntent→L3-interno) registrado no L0 + DEBT.md.
lente: delta de aresta ZERO — nenhum `use` novo de produção (o único `use comemo::Track` é
  #[cfg(test)] function-local no helper, fora do grafo de produção); `full_error` é bool em
  RunIntent/EvalContext, `morph_canon`/`Vec`/`Content` já reachable. Baseline 219|676|[90,4]|66|0.
perf: caminho quente ~nulo — o histórico está atrás de `if ctx.full_error` (default false →
  não aloca, não computa). A classificação só corre no erro + flag (cold). Sub-resolução nos
  docs de teste; suíte inalterada (0,41s).
```

## Mapa de filtro (campo)
**Lugar lógico:** a flag de erro completo afirma **só o que mede** — cíclico é fato (a
morfologia repetiu), não-convergente é fato (o teto sem repetição); o rótulo que adivinharia
o futuro pós-corte foi **cortado** (ADR-0108 regra 4 aplicada ao próprio diagnóstico). E o
canal foi aberto **até onde a origem real exige** (L2 `RunIntent` + sibling em L1), **não**
até a assinatura pública de L3 (sem demanda — uma flag off-by-default não justifica mudar a
API pública). **Rastro:** P348 adiou a flag; P350 mediu que não há canal L4→L1; P350b mediu a
origem (`RunIntent`) + o ponto de leitura (`EvalContext`) e o dono confirmou o intermédio +
cortou o 3º rótulo; **P350c implementa** a forma confirmada, com a CLI como DEBT-59.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.

## Ficheiros tocados
- **Código:** `01_core/.../eval/mod.rs` (EvalContext.full_error + `eval` delegado +
  `eval_with_full_error`), `eval/rules.rs` (histórico + classificação sob flag),
  `eval/tests.rs` (helper + 3 testes), `02_shell/src/cli.rs` (`RunIntent.full_error`),
  `04_wiring/src/main.rs` (`..` no destructure).
- **L0:** `entities/f_fronteira_e1.md §3a.7-bis`, `rules/eval.md`, `shell/cli.md`.
- **DEBT:** `00_nucleo/DEBT.md` (DEBT-59).
- **Mecânico:** bump de `@prompt-hash` nos ficheiros que referenciam os 3 L0 tocados.
- **Decisão, não débito:** a assinatura pública de `compile_to_pdf_bytes` (L3) — intacta.
