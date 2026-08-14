# Diagnóstico Passo 1038 — Medição MC/DC nos 5 Nós Fatiados e Ponto de Mensagens de Erro

**Proveniência da Medição:**
- **Commit SHA:** `895761c330441ad568b749ff2cb2900d4c619639`
- **Estado da Working Tree:** `00_nucleo/materialization/typst-passo-1038.md` (não monitorado) e `temp/mcdc_test.typ` (criado para execução de teste)
- **Toolchain:** `rustc 1.99.0-nightly (ba28ff76f 2026-08-13)` (`x86_64-unknown-linux-gnu`)
- **Flags de Instrumentação:** `RUSTFLAGS="-C instrument-coverage -Z coverage-options=condition"` com `llvm-profdata` e `llvm-cov show --show-branches=count`
- **Data/Hora:** 2026-08-14T09:41:15-03:00

---

## Fase A — Alvo 1: Cobertura de Decisões nos 5 Nós Fatiados

Medição das funções que concentram as decisões combinadas (`&&` / `||`) mais complexas dentro de cada família fatiada:

| Família / Módulo | Função / Área Inspecionada | Nº de Decisões (Branches) | Max Condições Combinadas | % MC/DC (Condition/Branch) Medido | Suspeito de Constant Folding? | Verificação Manual |
| :--- | :--- | :---: | :---: | :---: | :---: | :--- |
| **1. `compiler/eval/bindings/`** (P1013) | `field_access::eval_field_access`<br>`method_dispatch::resolve_index` | 34 (`field_access`) <br> 30 (`method_dispatch`) | 2 (`v >= 0 && v < len`) | **8.82%** (`field_access`) <br> **0.00%** (`method_dispatch`) | Não | Expressões dinâmicas baseadas em tamanho de vetores/dicionários em tempo de execução. |
| **2. `compiler/stdlib/structural/`** (P1014/P1023) | `heading::heading_elem` / `heading_layout` | 18 (`heading.rs`) <br> 80 (`structural/mod.rs`) | 2 (`level == 0 \|\| level > 6`) | **27.78%** (`heading.rs`) <br> **0.00%** (`structural/mod.rs`) | Não | `level` vem do valor dinâmico `Value::Int(n)` do AST. |
| **3. `compiler/stdlib/text/`** (P1022) | `text/constructor.rs`<br>`text/smartquote.rs` | 42 (`constructor.rs`) <br> 6 (`smartquote.rs`) | 2 (`FontList::new(...)`) | **0.00%** (`constructor.rs`) <br> **0.00%** (`smartquote.rs`) | Não | Condições avaliam estruturas de fontes e argumentos passados na chamada. |
| **4. `compiler/eval/operators/`** (P1002) | `operators/arithmetic.rs`<br>`operators/equality.rs` | 76 (`arithmetic.rs`) <br> 24 (`equality.rs`) | 3 (`a.rel == 0.0 && b.rel == 0.0`) | **3.95%** (`arithmetic.rs`) <br> **0.00%** (`equality.rs`) | Não | Comparações dinâmicas de tipos e unidades (`Length`, `Relative`, `Ratio`). |
| **5. `compiler/stdlib/foundations/`** (P1032) | `foundations/color.rs`<br>`foundations/cast.rs` | 44 (`color.rs`) <br> 52 (`cast.rs`) | 3 (`len == 6 \|\| len == 8`, `!long && !short`) | **25.00%** (`color.rs`) <br> **0.00%** (`cast.rs`) | Não | Parse de strings Hex e tamanhos de mapas de argumentos em runtime. |

### Resposta à Pergunta da Fase A
- **`eval/operators/`** e **`eval/bindings/`** são **caros de cobrir por MC/DC**: possuem nós densos de combinatória de tipos e limites (operações entre `Length`, `Relative`, `Ratio` em `arithmetic.rs` e dispatch em `method_dispatch.rs`). Exigem matrizes de testes dedicadas para exercitar todas as permutantes das tabelas de condição (`True/False`, `False/True`, `True/True`).
- **`stdlib/structural/`** e **`stdlib/text/`** são **baratos a médios**: poucas guardas compostas por função. A validação de intervalos (ex: nível de cabeçalho `1..=6`) requer poucos casos de teste adicionais para atingir 100% de MC/DC.
- **`stdlib/foundations/`** é **médio**: parse de formatos hex e mapas de argumentos nomeados exige exercitar cenários específicos (comprimentos 3, 4, 6 e 8 em cores).

---

## Fase B — Alvo 2: Onde Vivem as Mensagens de Erro

- **Definição Única da Struct de Erro:**
  [01_core/src/entities/source_result.rs:40](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L40) (`pub struct SourceDiagnostic`)
- **Construtores Principais:**
  [01_core/src/entities/source_result.rs:55](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L55) (`SourceDiagnostic::error`) e [source_result.rs:66](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L66) (`SourceDiagnostic::warning`).

### Resposta à Pergunta da Fase B
- **Dispersas:** O sistema **não** possui um catálogo centralizado de mensagens de erro nem formatador de mensagens i18n.
- Cada função nativa em `01_core/src/compiler/stdlib/` e ponto de avaliação em `01_core/src/compiler/eval/` constrói sua própria mensagem de erro em texto inline (ex: `SourceDiagnostic::error(span, format!("heading(): level deve estar entre 1 e 6, recebeu {}", n))`), exatamente como o vanilla faz.
- **Medição MC/DC do Ponto Central (`SourceDiagnostic::error`):** O construtor em `source_result.rs:55` possui **0 decisões booleanas compostas** (atribui diretamente os campos `severity`, `span`, `message` e inicializa vetores vazios).
