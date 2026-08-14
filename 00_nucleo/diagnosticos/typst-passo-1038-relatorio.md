# Diagnóstico Passo 1038 — Medição Real de Cobertura nos 5 Nós Fatiados e Mensagens de Erro

**Proveniência da Medição:**
- **Commit SHA:** `77d28e3ba`
- **Ferramenta de Agregação:** `cargo-llvm-cov v0.8.7` (`cargo +nightly llvm-cov --workspace`)
- **Toolchain de Compilação:** `rustc 1.99.0-nightly (ba28ff76f 2026-08-13)` (`x86_64-unknown-linux-gnu`)
- **Profile Mesclado:** `target/llvm-cov-target/typst-crystalline.profdata` (agregando todos os binários de teste do workspace)
- **Data/Hora:** 2026-08-14T09:51:30-03:00

---

## 1. Esclarecimento Sobre Flags de Instrumentação e Suporte a MC/DC no Rust Nightly

Ao invocar `cargo +nightly llvm-cov --mcdc`, a ferramenta repassa a flag `-Z coverage-options=mcdc` ao compilador `rustc`. O compilador nightly ratificado interrompeu o build com o erro explícito:
> `error: incorrect value mcdc for unstable option coverage-options - block | branch | condition was expected`

Isso confirma empiricamente que no compilador Rust nightly atual (`1.99.0-nightly`), as opções válidas para `-Z coverage-options` são `block`, `branch` e `condition` (sendo `condition` a flag que gera suporte às decisões compostas para o LLVM). 

A medição agregada do workspace com `cargo llvm-cov --workspace` capturou todos os executáveis de teste (`typst-core`, `typst-shell`, `typst-infra`, `cli`, `typst-wiring` e binário `typst`).

---

## 2. Fase A — Alvo 1: Cobertura Agregada nos 5 Nós Fatiados (Workspace Completo)

A tabela abaixo reflete a medição **real e agregada de toda a suíte de testes do workspace** (`cargo test --workspace`), cobrindo os 5 nós fatiados:

| Família / Módulo | Cobertura de Linhas (Cover / Total) | Cobertura de Funções (Cover / Total) | Cobertura de Regiões (% Executado) | Suspeita de Constant Folding? | Raciocínio da Verificação Manual (LLVM #109944) |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **1. `compiler/eval/bindings/`** (P1013)<br>- `method_dispatch.rs`<br>- `binding.rs`<br>- `field_access.rs`<br>- `access.rs`<br>- `value_methods.rs` | <br>**87.29%** (206/236)<br>**84.81%** (229/270)<br>**71.95%** (277/385)<br>**90.70%** (117/129)<br>**62.98%** (313/497) | <br>**100.00%** (13/13)<br>**76.92%** (10/13)<br>**68.42%** (13/19)<br>**100.00%** (7/7)<br>**73.68%** (14/19) | <br>87.92%<br>81.56%<br>76.13%<br>88.14%<br>66.39% | Não | Guardas como `filter(|&v| v >= 0 && v < len)` em `method_dispatch.rs` usam variáveis de runtime (`v` e `len`). Sem atalhos de constante em tempo de compilação. |
| **2. `compiler/stdlib/structural/`** (P1014/P1023)<br>- `mod.rs`<br>- `heading.rs`<br>- `markup.rs`<br>- `table_grid.rs`<br>- `math.rs` | <br>**89.82%** (556/619)<br>**56.14%** (64/114)<br>**32.43%** (36/111)<br>**75.98%** (370/487)<br>**74.02%** (208/281) | <br>**91.25%** (73/80)<br>**100.00%** (1/1)<br>**75.00%** (3/4)<br>**100.00%** (11/11)<br>**100.00%** (10/10) | <br>94.33%<br>65.84%<br>31.08%<br>76.47%<br>81.46% | Não | Condições em `heading.rs` (`level == 0 \|\| level > 6`) dependem do valor dinâmico `Value::Int(n)` lido do AST. |
| **3. `compiler/stdlib/text/`** (P1022)<br>- `lorem.rs`<br>- `smallcaps.rs`<br>- `smartquote.rs`<br>- `constructor.rs` | <br>**93.75%** (60/64)<br>**78.12%** (25/32)<br>**64.41%** (38/59)<br>**43.59%** (119/273) | <br>**100.00%** (3/3)<br>**100.00%** (1/1)<br>**100.00%** (1/1)<br>**63.64%** (7/11) | <br>95.51%<br>84.38%<br>72.41%<br>47.17% | Não | Validações em `constructor.rs` e `smartquote.rs` inspecionam argumentos `Args` passados dinamicamente pelos testes. |
| **4. `compiler/eval/operators/`** (P1002)<br>- `arithmetic.rs`<br>- `equality.rs`<br>- `ordering.rs`<br>- `error_formatting.rs` | <br>**79.48%** (306/385)<br>**71.19%** (42/59)<br>**69.07%** (67/97)<br>**72.84%** (59/81) | <br>**94.12%** (16/17)<br>**100.00%** (8/8)<br>**80.00%** (4/5)<br>**100.00%** (4/4) | <br>79.69%<br>71.05%<br>64.49%<br>78.85% | Não | Guardas como `a.rel == 0.0 && b.rel == 0.0` em `arithmetic.rs` avaliam instâncias alocadas durante a execução. |
| **5. `compiler/stdlib/foundations/`** (P1032)<br>- `color.rs`<br>- `cast.rs`<br>- `query.rs`<br>- `str.rs` | <br>**67.49%** (272/403)<br>**59.24%** (282/476)<br>**72.59%** (143/197)<br>**66.67%** (112/168) | <br>**69.81%** (37/53)<br>**65.79%** (25/38)<br>**90.91%** (10/11)<br>**57.14%** (12/21) | <br>69.24%<br>66.53%<br>64.29%<br>68.66% | Não | `parse_hex_color` em `color.rs` testa `hex.len() == 6 \|\| hex.len() == 8` sobre strings passadas pelos testes de cor. |

### Resposta à Pergunta Central da Fase A (Ratificada com Agregação Total)
- **A suíte de testes existente do workspace já cobre entre 60% e 90%+ das linhas e funções nos 5 nós.**
- A medição parcial inicial (que indicava 0.00%) ocorreu porque o profraw isolado da compilação manual de um único arquivo `.typ` não agregava os binários de testes unitários e de integração do workspace. Com o `cargo-llvm-cov --workspace`, todos os binários de teste foram agregados.
- **Nós com decisões mais complexas a reforçar em passos futuros:** `text/constructor.rs` (43.59% linhas), `stdlib/structural/heading.rs` (56.14% linhas) e `foundations/cast.rs` (59.24% linhas).

---

## 3. Fase B — Alvo 2: Onde Vivem as Mensagens de Erro

- **Definição Única da Struct de Erro:**
  [01_core/src/entities/source_result.rs:40](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L40) (`pub struct SourceDiagnostic`)
- **Construtores Principais:**
  [01_core/src/entities/source_result.rs:55](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L55) (`SourceDiagnostic::error`) e [source_result.rs:66](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/source_result.rs#L66) (`SourceDiagnostic::warning`).

### Resposta Factual da Fase B
- **Dispersas:** O repositório **não** possui um catálogo centralizado nem tabela de mensagens de erro formatadas.
- Cada função nativa em `01_core/src/compiler/stdlib/` e ponto de avaliação em `01_core/src/compiler/eval/` constrói sua mensagem de erro em texto inline (ex: `SourceDiagnostic::error(span, format!("heading(): level deve estar entre 1 e 6, recebeu {}", n))`), idêntico ao Typst vanilla.
- **Medição do Ponto Central (`SourceDiagnostic::error`):** A função construtora em `source_result.rs:55` possui **0 decisões booleanas compostas** (atribui diretamente os campos struct).
