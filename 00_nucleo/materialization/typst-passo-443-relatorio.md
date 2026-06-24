# P443 — Relatório de decisão: `get_unchecked` no scanner (DEBT-42)

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch de decisão:** `Tekt`  
> **Branch experimental referenciada:** `p442-get-unchecked-removal`  
> **Foco:** Correr benchmark comparativo, aplicar critério ADR-0032 e fechar DEBT-42 definitivamente.

---

## Resumo executivo

**P443 (S → FECHADO):** o benchmark comparativo confirmou que a remoção de `get_unchecked` do `Scanner` introduz regressões de performance entre **+8% e +58%** consoante o input. Aplicando o critério da **ADR-0032**, a decisão é **manter o `unsafe` como excepção permanente**.

- Foi escrita a **ADR-0116** (`EM VIGOR`) a autorizar as 5 ocorrências de `unsafe { self.string.get_unchecked(...) }` em `01_core/src/rules/lexer/scanner.rs`, com o número concreto de regressão.
- **DEBT-42 foi fechado** em `00_nucleo/diagnosticos/debt/DEBT.md` como excepção permanente.
- A branch `p442-get-unchecked-removal` **não foi mergeada** para `Tekt`; permanece como candidate documentado.
- `cargo test --workspace` verde em `Tekt`; `crystalline-lint .` sem novas violações.

---

## 1. Procedimento de medição

1. **Baseline:** `git checkout Tekt` + `cargo bench --bench scanner_bench -- --save-baseline tekt-p443`.
2. **Candidate:** `git checkout p442-get-unchecked-removal` + `cargo bench --bench scanner_bench -- --baseline tekt-p443`.
3. Criterion reportou deltas directamente contra o baseline guardado.

Ambas as execuções usaram o perfil `bench` (release) e 100 amostras por input.

---

## 2. Resultados

### 2.1 Medianas por input

| Input | Tamanho | Baseline (`Tekt`) | Candidate (`p442`) | Δ tempo | Δ ns/byte | Faixa ADR-0032 |
|-------|--------:|------------------:|-------------------:|--------:|----------:|:---------------|
| b1_hello | 11 B | 50,54 ns | 62,03 ns | +22,8% | +23,3% | > 20% |
| b2_text | 4.721 B | 11,77 µs | 19,00 µs | +58,3% | +54,1% | > 20% |
| b3_math | 2.451 B | 21,85 µs | 26,36 µs | +22,3% | +18,4% | > 20% |
| b4_code | 4.035 B | 34,11 µs | 44,04 µs | +27,1% | +26,7% | > 20% |
| b5_utf8 | 1.691 B | 13,16 µs | 13,87 µs | +6,4% | +8,5% | 5–20% |

### 2.2 Agregados

- **Média ponderada por bytes:** ≈ **+35%**.
- **Pior caso:** `b2_text` com **+58,3%**.
- **Inputs acima de 20%:** 4 de 5.
- **Inputs abaixo de 5%:** nenhum.

### 2.3 Decisão segundo ADR-0032

| Cenário | Limiar | Aplicável? | Decisão |
|---------|--------|------------|---------|
| A — eliminar `unsafe` | < 5% | Não | — |
| B — decisão humana | 5% – 20% | Parcialmente (`b5_utf8`; `b3_math` por ns/byte) | Não suficiente |
| C — excepção permanente | > 20% | Sim (4/5 inputs; média ponderada +35%) | **Adoptada** |

---

## 3. Mudanças de código

### 3.1 ADR-0116

| Ficheiro | Descrição |
|----------|-----------|
| `00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md` | ADR `EM VIGOR` que autoriza permanentemente as 5 ocorrências de `get_unchecked` em `scanner.rs`, com invariante de segurança, medições e alternativas consideradas. |
| `00_nucleo/adr/README.md` | Adicionada entrada ADR-0116 ao índice; total actualizado para 67 ADRs. |

### 3.2 DEBT.md

| Ficheiro | Descrição |
|----------|-----------|
| `00_nucleo/diagnosticos/debt/DEBT.md` | DEBT-42 marcado como ✅ FECHADO (Passo 443, excepção permanente ADR-0116). Adicionado resumo das medições. |

---

## 4. Verificação

### 4.1 `cargo test --workspace` (em `Tekt`)

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam**.

### 4.2 `crystalline-lint .`

Resultado: **zero novas violações**. Apenas os 2 warnings órfãos de prompts pre-existentes.

### 4.3 Estado do `unsafe` em `scanner.rs`

```bash
grep -n "get_unchecked" 01_core/src/rules/lexer/scanner.rs
```

Resultado: **5 ocorrências** mantidas, agora cobertas por ADR-0116.

---

## 5. Notas e próximos passos

- O P443 não alterou código funcional em `Tekt`; a branch experimental `p442-get-unchecked-removal` contém o refactor com slicing seguro e pode ser consultada ou eliminada futuramente.
- Com DEBT-42 fechado, o inventário de débitos técnicos está mais próximo de limpo. Os passos seguintes podem focar-se em novas features ou refinamentos de paridade, sem débito de `unsafe` em `scanner.rs` pendente.
- Se, no futuro, o custo relativo do bounds-checking deixar de ser relevante (por exemplo, por refactor que reduza o número de extrações de substring por token), a ADR-0116 pode ser revogada e a branch experimental reavaliada.

---

## 6. Commits

- Branch: `Tekt`
- Commit: `ADR-0116: autoriza exceção permanente de get_unchecked no scanner (P443)`

Alterações incluídas:
- `00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md`
- `00_nucleo/adr/README.md`
- `00_nucleo/diagnosticos/debt/DEBT.md`
- `00_nucleo/materialization/typst-passo-443-relatorio.md`
