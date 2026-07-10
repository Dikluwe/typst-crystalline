# P671 — Relatório de Paridade de Produção

**Passo:** 671  
**Data:** 2026-07-10  
**Foco:** A verificação de Python/fontTools (P667) estava a correr incondicionalmente para todos os documentos, mesmo os sem fontes variáveis.  
**Commit base:** `0cba95181a0cf01aa9a6237aef69915fedf0a12a` (P670)  
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

---

## 1. Sonda

### 1.1 Hipótese

P670 registou um aumento uniforme nos tempos dos documentos micro (cristalino ~150–170 ms → ~250–290 ms) e atribuiu-o a "variação de ambiente". Como o aumento era fixo e uniforme em 34 documentos, a hipótese alternativa é um custo de arranque novo introduzido por P666–P669 (verificação de disponibilidade de Python/fontTools para fontes variáveis).

### 1.2 Bissecção temporal directa

Mediu-se `lab/parity/corpus/p490/test-array.typ` no commit imediatamente anterior a P666 (`0e51b31f2`, P665) e no commit actual (`0cba95181`, P670), no mesmo ambiente e na mesma sessão.

| Commit | Descrição | Tempo médio (ms) | σ |
|---|---|---:|---:|
| `0e51b31f2` | Antes de P666–P669 | **210,3** | ±1,9 |
| `0cba95181` | Depois de P666–P669 (P670) | **260,2** | ±3,8 |
| Diferença | Custo de P666–P669 | **+49,9** | — |

A hipótese foi **confirmada**: P666–P669 adicionaram ~50 ms de overhead a cada documento, independentemente de usar fontes variáveis.

### 1.3 Localização exacta

A verificação de Python era chamada incondicionalmente em `03_infra/src/pipeline.rs:434`:

```rust
if !variable_font_instancer_available() {
    // ...
}
```

`variable_font_instancer_available()` (`03_infra/src/font_variant.rs:140`) invoca um subprocesso Python (`python3 -c "import fontTools"`) para verificar se o fontTools está disponível. Este subprocesso corria para **todos** os documentos, mesmo os que não usavam fontes variáveis.

---

## 2. Implementação

### 2.1 `03_infra/src/pipeline.rs`

A verificação de disponibilidade de Python só corre agora se houver de facto uma fonte variável com eixos não-default no documento. Antes de chamar `variable_font_instancer_available()`, calcula-se:

```rust
let needs_variable_font_instancer = resolved.iter().any(|((_, font_variant), bytes)| {
    is_variable_font(bytes) && !axis_variations_for_font_variant(font_variant).is_empty()
});
if needs_variable_font_instancer && !variable_font_instancer_available() {
    // ... devolve erro claro ...
}
```

Isto mantém o comportamento de P667 (erro claro quando uma VF precisa de instanciação mas Python/fontTools não está disponível), mas elimina o custo de arranque do subprocesso para documentos sem fontes variáveis.

---

## 3. Validação

### 3.1 Documento sem fontes variáveis

Após a correcção:

| Commit | Descrição | Tempo médio (ms) | σ |
|---|---|---:|---:|
| `0cba95181` + fix | P671 corrigido | **208,8** | ±3,0 |

O tempo recuperou para o nível pré-P666 (~210 ms).

Confirmou-se ainda com `strace` que nenhum subprocesso Python é invocado para `test-array.typ` após a correcção.

### 3.2 Documento com fontes variáveis (caminho de erro)

Teste com `TYPST_CRYSTALLINE_PYTHON` apontando para um interpretador inexistente:

```bash
cat > /tmp/p671-vf-sem-python.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.
EOF
TYPST_CRYSTALLINE_PYTHON=/tmp/p671-python-que-nao-existe ./target/release/typst /tmp/p671-vf-sem-python.typ /tmp/p671-vf.pdf
```

Resultado:

```text
/tmp/p671-vf-sem-python.typ:<detached>: error: fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível
Exit code: 1
```

O erro claro de P667 continua a funcionar; o caminho de erro só é atingido quando necessário.

### 3.3 `cargo test --workspace`

Resultado: todos os crates passaram.

### 3.4 `crystalline-lint .`

Resultado: `✓ No violations found`.

### 3.5 Benchmark completo

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
```

Tempo total da execução: ~5 min 8 s.

#### Documentos micro (após correcção)

| Documento | Vanilla (ms) | Cristalino (ms) | Rácio P671 | Rácio P670 |
|---|---|---:|---:|---:|---:|
| test-array | 109,34 | 206,89 | 1,89× | 2,29× |
| test-calc | 109,51 | 204,87 | 1,87× | 2,20× |
| test-columns | 114,03 | 221,08 | 1,94× | 2,25× |
| test-dict | 112,82 | 206,80 | 1,83× | 2,21× |
| test-enum-start | 108,82 | 207,82 | 1,91× | 2,12× |
| test-footnote | 109,13 | 206,32 | 1,89× | 3,19× |
| test-list-marker-array | 108,61 | 207,65 | 1,91× | 2,27× |
| test-math | 110,78 | 204,68 | 1,85× | 2,22× |
| test-page | 112,77 | 214,92 | 1,91× | 2,29× |
| test-par | 109,87 | 208,64 | 1,90× | 2,31× |
| test-place | 112,81 | 208,71 | 1,85× | 2,24× |
| test-quote | 108,81 | 205,49 | 1,89× | 2,27× |
| test-raw | 139,82 | 205,69 | 1,47× | 1,82× |
| test-set-local | 110,12 | 208,09 | 1,89× | 2,36× |
| test-show-link | 111,10 | 220,81 | 1,99× | 2,29× |
| test-show-regex | 109,96 | 208,73 | 1,90× | 2,32× |
| test-show-where-multi | 110,09 | 204,41 | 1,86× | 2,27× |
| test-str | 111,32 | 204,54 | 1,84× | 2,22× |
| test-stroke-sides | 6,85 | 203,01 | 29,63× | 32,94× |
| test-table | 110,26 | 216,73 | 1,97× | 2,30× |
| test-bibliography-csl | 173,79 | 275,93 | 1,59× | 1,88× |
| test-dict-methods | 109,35 | 213,01 | 1,95× | 2,29× |
| test-enum-advanced | 113,27 | 221,89 | 1,96× | 2,31× |
| test-figure-advanced | 110,05 | 205,67 | 1,87× | 2,30× |
| test-footnote-advanced | 110,95 | 209,51 | 1,89× | 2,28× |
| test-image-fit | 7,02 | 205,81 | 29,31× | 35,76× |
| test-list-advanced | 110,45 | 205,57 | 1,86× | 2,33× |
| test-metadata-query | 110,94 | 206,36 | 1,86× | 2,29× |
| test-outline-advanced | 111,69 | 205,12 | 1,84× | 2,28× |
| test-page-header-footer | 120,81 | 213,58 | 1,77× | 2,27× |
| test-par-advanced | 109,78 | 209,21 | 1,91× | 2,32× |
| test-place-absolute | 112,69 | 208,62 | 1,85× | 2,26× |
| test-quote-advanced | 110,81 | 205,49 | 1,85× | 2,29× |
| test-raw-advanced | 166,43 | 204,00 | 1,23× | 1,49× |
| test-state-counter | 112,35 | 203,58 | 1,81× | 2,27× |

A maioria dos documentos micro recuperou ~45–60 ms, regressando a rácios próximos de P618 (1,6–1,9× em vez de 2,1–2,3×).

#### Macro

| Passo | Vanilla (ms) | Cristalino (ms) | Rácio |
|---|---|---:|---:|---:|
| P618 | 5303,54 | 35958,10 | 6,78× |
| P657 | 5035,43 | 29097,89 | 5,78× |
| P670 | 6047,79 | 36129,28 | 5,97× |
| **P671** | **5728,07** | **36357,03** | **6,35×** |

O `macro-10x` não foi directamente afectado pela correcção (o documento não usa fontes variáveis, mas o tempo dominado por `shape_ms` torna o overhead de Python irrelevante). A variação do rácio reflecte sobretudo flutuação do tempo vanilla entre execuções.

---

## 4. Decisão

- A hipótese de P671 foi **confirmada**: a verificação de Python/fontTools corria incondicionalmente para todos os documentos, adicionando ~50 ms de overhead por compilação.
- A correcção moveu a verificação para um caminho condicional: só corre quando existe uma fonte variável com eixos não-default no documento.
- Documentos sem fontes variáveis recuperaram o tempo de arranque pré-P666.
- O erro claro de P667 para documentos com VF continua a funcionar sem regressão.
- `cargo test --workspace`, `crystalline-lint .` e o benchmark completo passaram.

Ação: P671 fecha a pendência de desempenho levantada por P670 e actualiza o registo de benchmark.

---

## 5. Proveniência da medição

- Commit base da sonda: `0cba95181a0cf01aa9a6237aef69915fedf0a12a` (P670).
- Commit anterior a P666: `0e51b31f23d4d24b286e3c78de3073d3345a40b6` (P665).
- Comando de bissecção: `hyperfine --warmup 3 --runs 10 './target/release/typst lab/parity/corpus/p490/test-array.typ /tmp/p671-*.pdf'`.
- Comando de validação VF: `TYPST_CRYSTALLINE_PYTHON=/tmp/p671-python-que-nao-existe ./target/release/typst /tmp/p671-vf-sem-python.typ /tmp/p671-vf.pdf`.
- Comando de testes: `cargo test --workspace`.
- Comando de linter: `crystalline-lint .`.
- Comando de benchmark: `time timeout 900 python3 tools/perf/benchmark-p507.py`.
- Ficheiro de resultados: `tools/perf/results/benchmark-p507-summary.json`.
