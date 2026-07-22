# Relatório de Verificação — Passo 805: `text::lorem_` — byte-parity de `#lorem(n)` (achado P798 #10)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799–P804 (zonas não relacionadas)
- **Working tree na validação "depois":** P799–P805 + P805a
- **Hora da Medição:** 2026-07-21 ~17:20 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #10 de P798: "falta ponto final no output de `#lorem(n)`".

## 2. Diagnóstico e Medição

**A medição alargou o achado (ADR-0108):** não era só o ponto final. `#lorem(10)` — cristalino `Lorem ipsum dolor sit amet consectetur adipiscing elit sed do` vs vanilla `... sit amet, consectetur adipiscing elit, sed do.` (faltavam vírgulas); `#lorem(30)` — texto totalmente divergente a partir da palavra ~19. O cristalino usava um vocabulário cíclico próprio de 59 palavras (Passo 391, scope-out explícito "o texto exacto não precisa de coincidir"); o vanilla gera com **Markov chain** determinística: crate `lipsum` 0.9.1, cadeia de ordem 2 treinada com `LOREM_IPSUM` + `LIBER_PRIMUS`, iterada de `("Lorem", "ipsum")` com RNG interno `ChaCha20Rng::seed_from_u64(97)` (confirmado na fonte da crate) + lógica de junção própria do typst (`--`→en-dash, capitalização, ponto final).

## 3. A Solução Implementada

Decisão arquitectural registada: **byte-parity** com a **mesma crate `lipsum` 0.9.1** (revoga o scope-out de Passo 391):
- `Cargo.toml` workspace + `01_core/Cargo.toml`: nova dependência (MIT; resolve offline — crate em cache local).
- `crystalline.toml`: nova entrada `[l1_allowed_external.lipsum]` (precedente: P756 `icu_segmenter`, P785a `syntect`).
- L0 `stdlib/text.md`: secção `lorem(n)` reescrita (P805), com nota de performance — a cadeia é construída por chamada (L1 proíbe estado global; o vanilla usa `LazyLock`).
- `native_lorem` passa a delegar num `lorem_impl` portado 1:1 do vanilla.

**Bloqueio encontrado na validação (P805a):** `#lorem(30)`/`#lorem(100)` divergiam **só em palavras com "fi"** ("fieri"→"eri"). A string lorem já era byte-idêntica (teste unitário); a causa era um bug pré-existente separado — ligaduras fi/ffi sem entrada ToUnicode no caminho de embed integral CFF (renderizavam, mas a extracção perdia os caracteres, em **qualquer** documento). Corrigido no sub-passo **P805a** (ver `paridade-producao-p805a.md`). Validação final: `#lorem(1)`, `(10)`, `(30)`, `(100)` — extracção **idêntica** ao vanilla.

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `lorem_p805_byte_parity_vanilla` (novo): strings exactas do vanilla para n=1, 10, 30. Falhou antes em `stdlib/mod.rs:11502`.
- Testes P391 existentes (contagem de palavras n=0/1/5/100, erros de validação) — inalterados e a passar.
- (P805a) `p805a_ligatura_fi_tem_entrada_to_unicode_no_embed_integral` — ver relatório próprio.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4328 passed; 1 ignored → DEPOIS 4329 passed; 1 ignored (total 4330 = +1 ✓)
Suite 'typst-infra':       ANTES 656 passed; 5 ignored → DEPOIS 657 passed; 5 ignored (+1 de P805a ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
