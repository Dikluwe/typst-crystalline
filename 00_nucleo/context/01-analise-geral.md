# Análise do Projecto typst-crystalline (Cristalino)

**Data**: 2026-04-01
**Fonte**: Knowledge base do projecto, ADRs 0001–0031, prompts L0, CLAUDE.md

---

## 1. O que é este projecto

O **typst-crystalline** (codinome "Cristalino") é um fork do compilador Typst — um sistema de tipografia moderno escrito em Rust — que está a ser migrado para a **Arquitetura Cristalina (Tekt)**.

A Arquitetura Cristalina é um framework estrutural desenhado especificamente para desenvolvimento assistido por IA. A sua premissa central: quando agentes de linguagem geram código a partir de contexto ad-hoc, o sistema sofre de **crescimento amorfo** — expansão funcional sem preservação estrutural. A solução proposta é tornar os prompts que originaram cada componente artefactos de primeira classe, versionados dentro do próprio projecto.

---

## 2. Arquitectura — as 5 camadas

O projecto segue uma topologia estrita de camadas:

```
        L₄ (Wiring)        ← composição, main()
       /  \
      /    \
    L₂     L₃              ← L2 e L3 são ramos independentes
  (Shell) (Infra)
      \    /
       \  /
        L₁ (Core)          ← domínio puro, zero I/O
         |
        L₀ (Seed)          ← prompts e ADRs (sem código executável)
```

| Camada | Directório | Conteúdo | Pode importar |
|--------|-----------|----------|---------------|
| L0 | `00_nucleo/` | Prompts, ADRs | — |
| L1 | `01_core/` | Tipos de domínio, lógica pura | Apenas stdlib pura + crates autorizadas |
| L2 | `02_shell/` | CLI, formatadores | L1 |
| L3 | `03_infra/` | I/O, fontes, PDF export | L1 |
| L4 | `04_wiring/` | Composição e injecção | L1, L2, L3 |
| lab | `lab/` | Código original (quarentena) | Qualquer (mas ninguém importa lab) |

**Regra fundamental de L1**: zero system I/O — sem `std::fs`, `std::net`, `std::env`, sem relógio do sistema. Crates como `Arc<T>`, `rustc_hash` e `indexmap` são permitidas porque são gestão de RAM e computação pura, não I/O.

---

## 3. Mecanismo de nucleação

Nenhum código pode existir sem um prompt correspondente em `00_nucleo/`:

1. Cada ficheiro de código tem um header de "Crystalline Lineage" com `@prompt`, `@layer`, `@prompt-hash`, `@updated`
2. O linter `crystalline-lint` verifica estas ligações (V1 para header em falta, V5 para hash divergente)
3. ADRs documentam decisões que transcendem um único componente
4. Prompts L0 contêm contexto, restrições, critérios de verificação e histórico de revisões

---

## 4. Pipeline do compilador

O Typst segue um pipeline clássico de compilação:

```
Parsing → Evaluation → Layout → Export
```

Na migração cristalina:
- **Parsing** (L1): `parse()` em `01_core/rules/parse.rs` — transforma source text em AST
- **Evaluation** (L1): `eval()` em `01_core/rules/eval.rs` — transforma AST em Content/Module
- **Layout** (L1): `layout()` em `01_core/rules/layout.rs` — transforma Content em PagedDocument
- **Export** (L3): PDF, SVG em `03_infra/src/export.rs` — materializa PagedDocument em ficheiros

---

## 5. Ferramentas e dependências

| Ferramenta | Papel |
|-----------|-------|
| Rust | Linguagem de implementação |
| Claude Code | Agente primário de implementação |
| `crystalline-lint` | Linter arquitectural (V0–V14) |
| `comemo` | Memoização/tracking (L1 autorizado, ADR-0001) |
| `ttf-parser` / `rustybuzz` | Parsing de fontes (L3, ADR-0019) |
| `rustc_hash` / `FxHasher` | Hashing optimizado (L1, ADR-0018) |
| `indexmap` | Maps com ordem de inserção (L1, ADR-0023) |
| `ecow` / `EcoString` | Strings eficientes (L1, ADR-0024) |
| Gemini | Auditor externo periódico |

### Crates autorizadas em L1 (`[l1_allowed_external]`)

```
thiserror, comemo, unicode_ident, unicode_math_class,
unicode_script, unicode_segmentation, rustc_hash, time, indexmap, ecow
```

---

## 6. Estrutura de ficheiros actual

```
typst-crystalline/
├── 00_nucleo/
│   ├── adr/                    ← ADRs 0001–0031
│   ├── prompts/                ← Prompts L0 organizados por subdirectório
│   │   ├── entities/           ← Prompts para tipos de domínio
│   │   ├── rules/              ← Prompts para lógica (parse, eval, layout)
│   │   └── contracts/          ← Prompts para traits (World)
│   └── materialization/        ← Instruções de passos (acesso restrito)
├── 01_core/src/
│   ├── entities/               ← FileId, Span, SyntaxKind, SyntaxNode,
│   │                              Content, Value, Module, Source, FontBook,
│   │                              layout_types, world_types, ast/
│   ├── contracts/              ← World, TrackedWorld
│   └── rules/                  ← parse.rs, eval.rs, layout.rs
├── 02_shell/src/               ← CLI (esqueleto)
├── 03_infra/src/
│   ├── world.rs                ← SystemWorld (impl World)
│   ├── export.rs               ← PDF export com CIDFont/ToUnicode
│   └── fonts.rs                ← FontBookMetrics com ttf-parser
├── 04_wiring/src/              ← main.rs (composição)
├── lab/typst-original/         ← Código original (oráculo imutável)
├── crystalline.toml            ← Configuração do linter
├── CLAUDE.md                   ← Guia autoritativo do projecto
└── DEBT.md                     ← Inventário de dívida técnica
```
