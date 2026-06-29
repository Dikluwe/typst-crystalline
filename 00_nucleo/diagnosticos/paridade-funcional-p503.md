---

# P503 — Re-baseline de Paridade contra Typst 0.15.0

> **Passo:** 503
> **Data:** 2026-06-29
> **Foco:** Re-executar empiricamente a bateria P490 (20 ficheiros) contra vanilla 0.15.0 para descobrir que MATCHs viraram DIFFs/AUSENTEs devido às breaking changes.
> **Tipo:** Diagnóstico empírico via re-execução de bateria.
> **Vanilla 0.15.0:** `/tmp/typst-0.15.0/typst-x86_64-unknown-linux-musl/typst` (`typst 0.15.0 (3ae52774)`)
> **Vanilla 0.14.2 (baseline P498):** `/usr/local/bin/typst` (`typst 0.14.2 (b33de9de)`)
> **Cristalino:** commit atual (`HEAD`)

---

## 1. Resumo Executivo

A bateria P490 foi re-executada contra o **Typst 0.15.0**. O resultado confirma a hipótese do passo:

| Métrica | Valor |
|---------|-------|
| Ficheiros testados | 20/20 |
| MATCH cristalino vs 0.15.0 | **20/20** |
| DIFF | 0 |
| ERRO_DESCRITIVO | 0 |
| AUSENTE | 0 |
| PANIC | 0 |

**Conclusão:** A paridade P490 é **estável** contra o Typst 0.15.0. Nenhum dos breaking changes do 0.15.0 afeta os 20 ficheiros da bateria. Os 3 ficheiros *non-locatable* (`test-page.typ`, `test-place.typ`, `test-stroke-sides.typ`) continuam a devolver `count=0` para `heading` sem PANIC em ambos os lados.

---

## 2. Metodologia

1. Obteve-se o binário oficial `typst 0.15.0` para `x86_64-unknown-linux-musl` a partir do release `v0.15.0` do repositório `typst/typst`.
2. Adicionou-se infraestrutura em `lab/parity/src/vanilla_invoke.rs` para invocar um binário vanilla específico por caminho absoluto.
3. Adicionou-se o teste `p503_rebaseline_0150` em `lab/parity/tests/structural_parity.rs`, que:
   - percorre os 20 ficheiros da bateria P490 em `lab/parity/corpus/p490/`;
   - executa query cristalino, vanilla 0.14.2 e vanilla 0.15.0 com os mesmos selectors;
   - classifica cada comparação cristalino vs 0.15.0 como `MATCH` / `DIFF` / `ERRO_DESCRITIVO` / `PANIC` / `AUSENTE`.
4. Comando de execução:

```bash
cd lab/parity
cargo test --test structural_parity p503_rebaseline_0150 -- --nocapture
```

---

## 3. Tabela Comparativa — P498 (0.14.2) vs P503 (0.15.0)

| Ficheiro | Selector | Descrição | P498 (0.14.2) | Vanilla 0.14.2 | Vanilla 0.15.0 | Cristalino | Classificação P503 |
|----------|----------|-----------|---------------|----------------|----------------|------------|--------------------|
| `test-list-marker-array.typ` | `list` | marker:Array | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-enum-start.typ` | `enum` | start:5 + (a) | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-par.typ` | `par` | leading/spacing/justify | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-show-link.typ` | `link` | #show link: ... | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-table.typ` | `table` | table.header/footer | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-raw.typ` | `raw` | raw com lang rust | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-quote.typ` | `quote` | attribution | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-footnote.typ` | `footnote` | footnote body | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-calc.typ` | `metadata` | calc args nomeados | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-array.typ` | `metadata` | array métodos | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-str.typ` | `metadata` | str métodos | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-dict.typ` | `metadata` | dict.at(default:) | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-show-regex.typ` | `heading` | show regex | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-set-local.typ` | `heading` | #set local em bloco | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-show-where-multi.typ` | `heading` | show.where(multi) scope-out | MATCH | count=1 | count=1 | count=1 | MATCH |
| `test-math.typ` | `math.equation` | vec/mat/cases | MATCH | count=5 | count=5 | count=5 | MATCH |
| `test-columns.typ` | `heading` | columns + colbreak | MATCH | count=0 | count=0 | count=0 | MATCH |
| `test-page.typ` | `heading` | page (non-locatable) | ERRO_DESCRITIVO (non-loc) | count=0 | count=0 | count=0 | MATCH |
| `test-place.typ` | `heading` | place (non-locatable) | ERRO_DESCRITIVO (non-loc) | count=0 | count=0 | count=0 | MATCH |
| `test-stroke-sides.typ` | `heading` | stroke-sides (non-locatable) | ERRO_DESCRITIVO (non-loc) | count=0 | count=0 | count=0 | MATCH |

**MATCHs preservados:** 20/20  
**PANICs:** 0

---

## 4. Verificação das Hipóteses de Breaking Change

As breaking changes listadas no changelog do 0.15.0 foram verificadas quanto ao impacto na bateria P490:

| # | Breaking Change | Afeta P490? | Evidência |
|---|-----------------|-------------|-----------|
| 1 | Backslash em paths proibido | Não | Nenhum ficheiro usa `\` em paths. |
| 2 | `slice(end:, count:)` = erro | Não | Nenhum teste usa ambos os argumentos. |
| 3 | `str(base:)` só inteiros | Não | `test-str.typ` usa `str(255, base: 16)` (inteiro) → continua ok. |
| 4 | Font suffixes omitidos | Não | Nenhum teste depende de suffixos de fonte variable. |
| 5 | `text.features` strict | Não | Nenhum teste configura `text.features`. |
| 6 | `math.class` não recursivo | Não | Nenhum teste usa `math.class`. |
| 7 | Delimiters callable → `lr` | Não | Nenhum teste usa `chevron.l` callable. |
| 8 | Baseline retained em `box`/`block` | Não | Não medido pela query estrutural desta bateria. |
| 9 | HTML `box`/`block` redefinidos | Não | Export HTML fora do escopo desta bateria. |
| 10 | HTML paragraph grouping | Não | Export HTML fora do escopo desta bateria. |
| 11 | `html.script`/`html.style` só string | Não | Nenhum teste usa HTML. |
| 12 | Non-Unicode paths rejeitados | Não | Paths do corpus são ASCII. |
| 13 | `--timings` requer nome | Não | Não usado na bateria. |
| 14 | `path` element removido | Não | Nenhum teste usa `path`. |
| 15 | `pattern` type removido | Não | Nenhum teste usa `pattern`. |
| 16 | `pdf.embed` removido | Não | Nenhum teste usa `pdf.embed`. |
| 17 | Scoped decode functions removidos | Não | Nenhum teste usa `*.decode`. |

**Veredito:** nenhuma breaking change do 0.15.0 altera o resultado da bateria P490. Todos os ficheiros que eram MATCH contra 0.14.2 continuam MATCH contra 0.15.0.

---

## 5. Análise das Hipóteses de Melhoria (Funcionalidades 0.15.0 já no Cristalino)

O P502 identificou funcionalidades 0.15.0 já implementadas no cristalino (`str.to-upper`, `str.to-unicode`, `str.repeat`, `calc.log10`, `calc.deg`, `calc.rad`). Essas funcionalidades não fazem parte da bateria P490 (são do P500), pelo que **não influenciam diretamente** o resultado deste re-baseline. Ficam registadas como contexto para o planeamento do P504.

---

## 6. Critério de Fecho

- [x] Todos os 20 ficheiros da bateria P490 re-executados contra vanilla 0.15.0.
- [x] Todos os 20 ficheiros re-executados contra cristalino (commit atual).
- [x] Tabela comparativa: P498 (0.14.2) vs P503 (0.15.0) vs cristalino.
- [x] Breaking changes identificados: **nenhum afeta a bateria P490**.
- [x] MATCHs preservados: **20/20**.
- [x] PANICs: 0.
- [x] `00_nucleo/diagnosticos/paridade-funcional-p503.md` produzido.
- [x] Decisão documentada para P504.

---

## 7. Decisão para P504

Como o P503 confirmou **20/20 MATCH** e zero breaking changes relevantes na bateria P490, o próximo passo (P504) deve focar-se em **novas funcionalidades do 0.15.0** a materializar no cristalino, por ordem de impacto:

1. `within` selector (introspection)
2. `dict.map` / `dict.filter`
3. Field access em `arguments`
4. `calc.asinh` / `calc.acosh` / `calc.atanh` / `calc.erf`
5. `int.min` / `int.max`
6. `range(inclusive)`
7. `int(base:)`
8. `counter.display(at:)`
9. `list.marker-align`
10. `page.bleed`
11. `divider` element

Se algum destes itens já estiver parcialmente implementado, o P504 deve começar pelo gap de menor tamanho que maximize MATCHs no audit P500/P501.

---

## A. Apêndice — Comando de Re-execução

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Confirmar versões
/usr/local/bin/typst --version
/tmp/typst-0.15.0/typst-x86_64-unknown-linux-musl/typst --version

# Executar sentinela P503
cd lab/parity
cargo test --test structural_parity p503_rebaseline_0150 -- --nocapture
```

## B. Apêndice — Ficheiros Alterados para Suporte ao P503

- `lab/parity/src/vanilla_invoke.rs` — adicionadas `run_typst_query_with_bin` e `vanilla_cli_available_with_bin` para permitir invocação de um binário vanilla específico (0.15.0) sem alterar o PATH.
- `lab/parity/tests/structural_parity.rs` — adicionado o teste `p503_rebaseline_0150` que executa a bateria P490 contra 0.14.2, 0.15.0 e cristalino, produzindo a matriz de classificação.

---
