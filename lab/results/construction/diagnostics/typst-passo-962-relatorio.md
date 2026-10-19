# Passo 962 — Relatório (`dif`/`Dif` reto: wrapper upright no registo)

**Data**: 2026-08-04
**Estado da árvore**: commit base `24aecc438` (P959); alterações deste passo por cima.

---

## 1. Fase A — mecanismo confirmado: símbolo dedicado, não regra contextual

- O documento usa o símbolo dedicado `dif` (não um "d" comum) —
  `.typ/typst-math-comprehensive-test.typ` (linhas 70-74, 176, 187, 212-213,
  232, 308, 320, 323, 331).
- O vanilla define `dif`/`Dif` em `math/op.rs:52-56` como
  `HElem(THIN, weak) + ClassElem(Unary, upright(SymbolElem('d')))` — o
  **`upright(...)` explícito** é o que torna o "d" reto (medido no trace:
  vanilla emite `d` U+0064).
- O registo cristalino (P795, `make_math_module` em `structural.rs`)
  produzia `Value::Content(Content::MathText("d"))` — sem wrapper — e o
  `apply_math_default` (P809/P812) italicava-o (𝑑 U+1D451) como qualquer
  letra de 1 carácter. É a **mesma classe** de interacção de P961 Parte B
  (conteúdo de construção a receber o default de itálico quando não devia —
  aqui em sentido inverso: um símbolo upright a receber itálico por engano).
- Não é regra contextual: nenhuma detecção de "d seguido de variável" é
  necessária — o símbolo dedicado existe e basta embrulhá-lo bem.

## 2. Fase B — implementação (TDD directo)

- **L0 primeiro**: `stdlib/structural.md` §P962 (com o scope-out registado:
  o vanilla prefixa `dif` com espaço fino fraco + classe `Unary`; o registo
  cristalino não tem nenhum — diferença subtil de espaçamento fica para
  passo próprio se medida).
- **Testes RED** (`p962_dif_registado_com_wrapper_upright`, eval) +
  caracterização/guardas já verdes (`p962_dif_wrapper_upright_produz_d_reto`,
  `p962_identificador_d_simples_continua_italico`,
  `p962_identificador_d_sem_wrapper`).
- **Correcção**: `dif`/`Dif` registam
  `MathStyled { italic: Some(false), .. }` sobre o texto — o mesmo que
  `upright(d)` produz; `apply_math_default` não toca no wrapper (P809).
- Suite: `cargo test --workspace` — **5704 testes, 0 falhas**.
- **Glifo**: `pdftotext` no documento de 30 secções — **48 "d" retos + 4 "𝑑"
  itálicos, exactamente as contagens do vanilla** (48 + 4; os 4 itálicos são
  variáveis `d` genuínas). Antes: os `dif` saíam itálicos.
- `crystalline-lint .`: zero violations (resta só o V7 órfão pré-existente).

## 3. Fase C — revalidação

- `compare.py` (secções 4, 11, 12, 15, 17, 23, 25, 27): melhorias modestas
  de |dx| (o glifo reto/itálico tem larguras parecidas — posições quase não
  mexem; a correcção é de identidade de glifo): sec 17: 2.53 → **0.99**,
  sec 12: 2.10 → **1.73**, sec 15: 4.34 → **3.63**, sec 27: 5.73 → **4.96**.
- Benchmark: ver tabela (hyperfine, "antes" = release pós-P959; JSONs em
  `tools/perf/results/p962-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 88.28 | 89.68 | 1.016 |
| 02-lorem | 107.99 | 111.36 | 1.031 |
| 03-images | 97.48 | 98.10 | 1.006 |
| 04-math | 130.98 | 130.36 | 0.995 |
| 05-tables | 92.52 | 94.00 | 1.016 |
| 06-long | 297.90 | 303.72 | 1.020 |
| 07-context | 130.92 | 130.97 | 1.000 |

Ratio médio **1.012** — sem regressão (a mudança é um wrapper num registo
de tabela; o spread é o ruído de ambiente habitual).
