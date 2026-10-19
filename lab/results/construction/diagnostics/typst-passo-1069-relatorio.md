# Relatório de Execução — Passo 1069 (Consolidado com Evidências de Validação)

**Data**: 2026-08-17
**Passo**: 1069 — Criação de `export/pdf_defaults.rs` e Migração das Duplicações
**Gate**: `ADR-0127` (Classificação: Refatoração Estrutural de Domínio / Homologada pelo Dono)
**Status**: CONCLUÍDO COM ÊXITO (Módulo L3 criado sob L0 selado, 10 pontos migrados, paridade binária de streams de saída comprovada, 100% PASS na suíte de testes)

---

## 1. Nucleação do Prompt L0 e Criação do Módulo

1. **Prompt L0 Criado**:
   * [`00_nucleo/prompts/infra/export/pdf_defaults.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/pdf_defaults.md)
   * Hash selado: `bff063c2`
2. **Módulo de Domínio Criado**:
   * [`03_infra/src/export/pdf_defaults.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/03_infra/src/export/pdf_defaults.rs)
   * Registrado em `03_infra/src/export/mod.rs` como `pub mod pdf_defaults;`.
3. **Localização de `impl Default for FontDescriptorMetrics`**:
   * A struct `FontDescriptorMetrics` é um tipo privado definido originalmente em `03_infra/src/export/builder.rs:505` (não em `fonts.rs`, cuja menção no L0/P1068 decorreu de uma suposição preliminar incorreta de localização).
   * Em conformidade com a *Orphan Rule* do Rust e a co-localização de tipos, `impl Default for FontDescriptorMetrics` foi implementado em `builder.rs:517-529`, logo abaixo da definição da struct.

---

## 2. Inventário de Pontos Migrados

Foram migrados com sucesso todos os **10 pontos de uso** em `stream.rs` e `builder.rs`:

| Item | Arquivo e Linha | Conteúdo Anterior | Substituição Canônica |
| :---: | :--- | :--- | :--- |
| **1** | `stream.rs:242` | `const FAUX_BOLD_K: f64 = 0.04;` | `pdf_defaults::FAUX_BOLD_K` |
| **2** | `stream.rs:397` | `const FAUX_BOLD_K: f64 = 0.04;` | `pdf_defaults::FAUX_BOLD_K` |
| **3** | `stream.rs:1245` | `const KAPPA: f64 = 0.552_284_749_831;` | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| **4** | `stream.rs:1458` | `const K: f64 = 0.552_284_749_831;` | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| **5** | `stream.rs:1597` | `const KAPPA: f64 = 0.552_284_749_831;` | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| **6** | `builder.rs:658-659` | `width: 595.28, height: 841.89` | `pdf_defaults::A4_DEFAULT_WIDTH / HEIGHT` |
| **7** | `builder.rs:1090-1094` | Literal `FontDescriptorMetrics { ... }` | `.unwrap_or_default()` |
| **8** | `builder.rs:1578-1582` | Literal `FontDescriptorMetrics { ... }` | `.unwrap_or_default()` |
| **9** | `builder.rs:1682` | `unwrap_or((595.0, 842.0))` | `unwrap_or((pdf_defaults::A4_FALLBACK_WIDTH_ROUNDED, pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED))` |
| **10** | `builder.rs:2042, 2113` | `unwrap_or(842.0)` | `unwrap_or(pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED)` |

---

## 3. Verificação e Paridade Binária

1. **Greps de Limpeza**:
   * Ocorrências de `0.552_284_749_831`: única declaração no módulo `pdf_defaults.rs`.
   * Declarações locais `const KAPPA` e `const FAUX_BOLD_K` em `stream.rs`: **0 restantes (100% eliminadas)**.
2. **Análise de Paridade de Saída (Hash Bruto vs Streams Descompactados)**:
   * **Hash SHA-256 do arquivo bruto `.pdf`**: varia entre compilações sucessivas porque o trailer do PDF embutido pelo Typst contém campos dinâmicos em tempo de execução:
     - `/CreationDate` (timestamp dinâmico do relógio do sistema).
     - `/ID [<instance-id-aleatório> <doc-id>]` (UUID gerado aleatoriamente por compilação conforme ISO 32000-1).
   * **Paridade Semântica e Estrutural (`mutool clean -d` / `pdftotext`)**:
     - Ao descompactar a árvore de objetos e streams, a comparação direta dos streams de conteúdo gráfico (operadores de curvas de Bézier `c`, operadores de texto `2 Tr`, matrizes de transformação, fontes embutidas e dimensões de página) confirmou **identidade exata de 100% dos bytes de conteúdo**.
3. **Suíte de Testes do Workspace**: `cargo test --workspace` aprovando **5.942 testes (100% PASS)**.
4. **Linter e Rastreabilidade**: `crystalline-lint .` com **0 erros** e **0 drift**.
