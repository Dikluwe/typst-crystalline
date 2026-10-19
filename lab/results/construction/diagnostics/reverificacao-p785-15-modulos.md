# Reverificação Independente do Passo 785 — Triagem dos 15 Módulos

> **Passo:** 785 (re-execução independente, para verificar os relatórios existentes)
> **Data:** 2026-07-20, medições entre ~16:08Z e ~16:40Z
> **Commit:** `0774275fe1b340d823e624958f66e5b6b344a51a`
> **Estado git no momento das medições:** working tree limpo, exceto 2 ficheiros não commitados (`paridade-producao-p785-15-modulos.md` e `varredura-profunda-p785-15-modulos.md`) — ambos **eliminados por este documento** (ver §7).
> **Binários:** vanilla `lab/typst-original/target/release/typst` (0.15.0, rev `969087ec`); cristalino `target/debug/typst` (debug build do commit acima — o relatório original usou release; não afeta os observáveis medidos).
> **Artefactos de teste:** `temp/temp_p785_verify/` (`t1`–`t7`, `t4a`, `s1`–`s8`).
> **Regras aplicadas:** ADR-0107 (paridade com a língua; mensagens de erro são observáveis), ADR-0108 (medição precede classificação), regra de proveniência de P569.

---

## 1. Método

1. Triagem executada **antes** de ler qualquer relatório P785 (anti-ancoragem).
2. Sonda idêntica à do passo (`awk`/`cut`/`sed` sobre `lente-lista-B-2026-07-15.txt`, exclusões P765a–P784).
3. Leitura de código vanilla com evidência `file:line` para cada módulo + grep da funcionalidade equivalente no cristalino (3 frentes: typst-utils, typst-syntax, typst+typst-library).
4. **Todo** sinal de exposição à língua foi confirmado ou refutado com teste `.typ` real vanilla vs cristalino — nenhuma classificação ficou por inferência.
5. Só depois: leitura dos 3 relatórios existentes e comparação.

---

## 2. Sonda e nota de desempate

Os 13 módulos determinísticos (contagem ≥ 4): `typst_utils` (14), `typst` (8), `reparser` (7), `typst_library` (7), `node` (6), `ast` (6), `pico` (5), `lines` (5), `highlight` (5), `routines` (5), `hash` (4), `fat` (4), `fields` (4).

**Desempate (3 itens, 7 módulos para 2 vagas):** a ordem do próprio pipeline da sonda dá `pico::exceptions` e `pico::bitcode`. Os relatórios anteriores pegaram `typst_eval::code` e `layout::container::callbacks` — desvio não declarado da sonda. Impacto de conteúdo ~zero (os 4 são mecânica), mas o conjunto triado aqui segue a sonda: **13 + `pico::exceptions` + `pico::bitcode`**. `eval::code` foi verificado à parte (§4, teste 4A).

---

## 3. Tabela da triagem (evidência file:line)

| # | Módulo | Itens | Classificação | Evidência |
|---|---|---:|---|---|
| 1 | `typst_utils` | 14 | mecânica | `typst-utils/src/lib.rs:43` `pub fn debug`, `:288` `Static<T>`, `:356` `trait Numeric` — helpers genéricos; cristalino usa std direto |
| 2 | `typst` | 8 | **SINAL (T4)** | `typst/src/lib.rs:208-244` `hint_invalid_main_file` gera hints observáveis; os outros 7 são mecânicos (`trace` `:87` é API de IDE, não função da língua) |
| 3 | `typst_syntax::reparser` | 7 | mecânica | `reparser.rs:15` chamado só por `Source::edit` (`source.rs:111`); invariante testada reparse == parse completo (`reparser.rs:360-367`) |
| 4 | `typst_library` | 7 | mecânica | `lib.rs:289` `Category` sem chamadores em `crates/`; suspeita `prelude` (cores nomeadas) **refutada** por T5 |
| 5 | `typst_syntax::node` | 6 | **SINAL (T1+T2)** | `node.rs:304` `errors_and_warnings`; parser vanilla emite warnings (`parser.rs:145`); cristalino descarta erros sintáticos (`01_core/src/engine/eval/mod.rs:308-321`) |
| 6 | `typst_syntax::ast` | 6 | mecânica | `ast.rs:1086-1116` wrappers tipados sobre CST; observável vive no eval (`engine/eval/math.rs:523-549`) |
| 7 | `typst_utils::pico` | 5 | mecânica | `pico.rs:40` `PicoStr(NonZeroU64)` interner; cristalino `Label(pub String)` (`entities/label.rs:12`) |
| 8 | `typst_syntax::lines` | 5 | mecânica (resolvido P785c) | `lines.rs:12`; cristalino `Source::span_to_line_col` (`entities/source.rs:138`), convenção alinhada (verificado em 4A: coluna 3:2 em ambos) |
| 9 | `typst_syntax::highlight` | 5 | **SINAL (T3)** | `highlight.rs:142`; consumidor de render `typst-library/src/text/raw.rs:934` para `typ/typc/typm`; two-face 0.4.3 sem gramática Typst; `highlight_html` sem chamadores no repo |
| 10 | `typst_library::routines` | 5 | mecânica + **nota de risco confirmada (T6)** | `routines.rs:28-30` "dynamic linking for crate splitting"; `SpanMode` doc `:121-125` afeta spans de diagnóstico; cristalino `native_eval` (`engine/stdlib/eval.rs:37`) usa spans detached |
| 11 | `typst_utils::hash` | 4 | mecânica | `hash.rs:13` `hash128` SipHash p/ memoização; divergência intencional documentada (`entities/layout_types.rs:542`) |
| 12 | `typst_utils::fat` | 4 | mecânica | `fat.rs:19` fat pointers p/ vtable de `#[elem]`; `Content` cristalino é enum fechado (ADR-0026) |
| 13 | `typst_library::foundations::fields` | 4 | mecânica (resolvido P785b) | `fields.rs:72` mensagem verbatim em `engine/eval/bindings.rs:1456-1493`; verificado s6 |
| 14 | `typst_utils::pico::exceptions` | 3 | mecânica | `pico.rs:297` binary search const sobre `LIST`; sub-maquinaria do interner |
| 15 | `typst_utils::pico::bitcode` | 3 | mecânica | `pico.rs:165` encoding 5-bit; `EncodingError` só em const panic em compile-time do Rust |

**Contagem: 12 mecânica pura, 3 SINAL confirmado (`node`, `typst`, `highlight`) + 1 nota de risco confirmada (`routines` → spans de `eval()`).**

---

## 4. Testes empíricos (reproduzíveis em `temp/temp_p785_verify/`)

| Teste | Documento | Vanilla 0.15.0 | Cristalino | Veredicto |
|---|---|---|---|---|
| T1 | `**` | `warning: no text within stars` + hint, exit 0 | silêncio, exit 0 | **divergência** — warnings sintáticos ausentes |
| T2 | `#let x = (` | `error: unclosed delimiter` 1:9, exit 1 | **exit 0, gera PDF de 1897 bytes** | **divergência grave** — erro sintático descartado |
| T3 | ```` ```typ #let x = 1 + 2 // h``` ```` | 434 px coloridos (PPM 150dpi) | 0 px coloridos (texto renderiza, monocromático) | **divergência** — sem highlight de código Typst |
| T4 | main não-UTF8 (`Ol\xe1 Mundo`) | `error: file is not valid UTF-8`, exit 1 | `error: main file not found: <path>`, exit 2 | **divergência** — mensagem errada e enganadora |
| T5 | `#rect(fill: red, ...)` | exit 0, 400 px vermelhos | exit 0, 400 px vermelhos | alinhado (refuta suspeita sobre `prelude`) |
| T6 | `#eval("1 + x")` | `error: unknown variable: x` em `1:6` (span do call site) | mesma mensagem com span `<detached>` | **divergência** — span de erro em `eval()` |
| 4A | `return 42` descartando `[Hello]` | warning + hint em 3:2 | warning + hint em 3:2, idênticos | alinhado (confirma P785 + P785c) |
| s1 | `int.bit-and(0b1100, 0b1010)` | exit 0 | `type int does not contain field "bit-and"` | lacuna stdlib real |
| s2 | `cmyk(100%, 0%, 0%, 0%)` | exit 0 | `cmyk(c): espera Float/Int, recebeu relative length` | lacuna parcial — existe mas não aceita `%` |
| s3 | `bytes((65, 66, 67))` | exit 0 | `type bytes does not have a constructor` | lacuna real |
| s4 | `datetime(year: 2024, ...)` | exit 0 | `type datetime does not have a constructor` | lacuna real |
| s5 | `str(label("s"))` | exit 0 | `str() não suporta content` (span `<detached>`) | lacuna real |
| s6 | `(10pt + 50%).ratio` | exit 0 | exit 0 | alinhado (confirma correção P785b) |
| s7 | `(1,2,3).join("-")` | `error: cannot join integer with string` (join existe) | `array does not contain field "join"` | lacuna real |
| s8 | `(-5).signum()` | exit 0 | `cannot access fields on type int` | lacuna real |

---

## 5. Validação do workspace

- `cargo test --workspace`: **4975 passed, 0 failed**, exit 0.
- `crystalline-lint .`: **zero violações**; 2 warnings V7 (prompts órfãos `engine/eval/field-access.md`, `infra/package_version_resolution.md`) — pré-existentes, não introduzidos por este passo.

---

## 6. Comparação com os relatórios anteriores

### `paridade-producao-p785.md` (original) — CONCORDO em grande parte

Tem proveniência, comandos e logs. Os 3 bugs que encontrou eram reais e foram corrigidos (P785a/b/c — verificado em s6, 4A). **Divergências:** classificou `node` e `typst` como "mecânica pura" com evidência só de nome — T1/T2/T4 refutam; e o conjunto dos 15 desvia da sonda no desempate (§2). Taxa de sinal: dele 3/15 (`highlight`, `fields`, `lines`), minha 3/15 (`node`, `typst`, `highlight`) — módulos diferentes a acender.

### Os dois relatórios `-15-modulos` — NÃO CONCORDO (eliminados, §7)

Inconsistências provadas:

1. **Erro de categoria:** afirmam que `typst_utils` "contém as 29 lacunas funcionais" (`array.contains`, `int.bit-and`, `cmyk`...). Os 14 itens de `typst_utils` são helper traits Rust (`SliceExt`, `OptionExt`...); essas lacunas vivem em `typst_library::foundations` e nunca foram itens deste módulo no inventário.
2. **Substância verdadeira, atribuição errada:** as lacunas stdlib existem (verifiquei 8: s1–s5, s7, s8 — 7 confirmadas, 1 parcial), mas o relatório mistura-as com a triagem dos 15 módulos.
3. **Contagem inconsistente:** um diz 28, lista 30; o outro diz 29. Sem hash de commit, sem comandos, sem script — viola a regra de proveniência de P569.
4. **`node` = "Mecânica pura"** — empiricamente falso (T1/T2). O descarte silencioso de erros sintáticos (T2) é o pior achado do lote e nenhum dos relatórios o regista.
5. **`typst` = "Mecânica pura"** — falso (T4).
6. **`highlight` = "RESOLVIDO EM P785a"** — só para linguagens externas; highlighting de código Typst (os itens reais do módulo) continua divergente (T3).
7. **"100% AUDITADOS"** sem uma linha de evidência por módulo — viola o critério de fecho do próprio passo 785.

---

## 7. Registo de eliminação

Eliminados por este documento, a pedido do dono, após prova das inconsistências do §6:

- `00_nucleo/diagnosticos/paridade-producao-p785-15-modulos.md`
- `00_nucleo/diagnosticos/varredura-profunda-p785-15-modulos.md`

Ambos eram não commitados (untracked) — sem recuperação por git. **Aproveitamento:** a lista de lacunas funcionais de stdlib (grupos A–F desses relatórios) é substancialmente real e fica registada em §4 (s1–s8) como insumo para o passo 785d — mas como auditoria de escopo da stdlib, não como itens dos 15 módulos.

---

## 8. Achados novos (não registados em nenhum relatório anterior)

1. **T2 (crítico) — erros sintáticos descartados com exit 0:** `#let x = (` gera PDF. Causa: `01_core/src/engine/eval/mod.rs:308-321` só propaga `InvalidHexNumber | InvalidUnicodeCodepoint`; `03_infra/src/pipeline.rs:91` é o único caminho. **Recomendação: passo dedicado antes do 785d** — divergência de língua visível em qualquer documento com erro de sintaxe.
2. **T1 — warnings sintáticos ausentes** (ex.: `no text within stars` + hint). Sem emissão de warnings em `01_core/src/engine/parse`.
3. **T4 — main não-UTF8 reportado como "main file not found":** `03_infra/src/world.rs:167-168` converte qualquer falha de leitura nessa mensagem; vanilla distingue UTF-8 inválido e emite hints (`typst/src/lib.rs:208-244`).
4. **T6 (+s2, s5) — spans `<detached>` em erros de `eval()` e conversões:** vanilla sintetiza o span do call site (`SpanMode::Uniform`, `routines.rs:121-125`); cristalino não chama `synthesize` (que existe, `entities/syntax_node.rs:185`) em `native_eval`.
5. **T3 — highlight de ` ```typ ` ausente:** P785a cobriu linguagens externas via syntect/two-face; falta a gramática Typst (os scopes `.typst` do tema em `engine/layout/raw.rs:58-73` nunca são atingidos).

---

## 9. Conclusão

A direção estratégica dos relatórios (maioria mecânica; continuar em lotes) está correta. A tabela de classificação dos relatórios `-15-modulos` estava errada em 3 módulos (`node`, `typst`, `highlight`), continha um erro de categoria na linha `typst_utils`, contagens internas inconsistentes (28/29/30) e zero proveniência — eliminados. O relatório original `paridade-producao-p785.md` permanece como registo válido da primeira execução. Este documento é a triagem de referência para os 15 módulos no commit `0774275f`.
