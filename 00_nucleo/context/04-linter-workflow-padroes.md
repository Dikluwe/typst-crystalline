# Linter, Workflow e Padrões do Projecto

**Data**: 2026-04-01

---

## 1. crystalline-lint — violations arquitecturais

O linter verifica 15 tipos de violação (V0–V14):

### Fatais (bloqueiam CI incondicionalmente)

| Código | Nome | Descrição |
|--------|------|-----------|
| V0 | UnreadableSource | Ficheiro não pode ser lido |
| V8 | UnmappedFile | Ficheiro fora da topologia de camadas |
| V10 | QuarantineLeak | Código de produção (L1–L4) importa de `lab/` |

### Erros

| Código | Nome | Descrição |
|--------|------|-----------|
| V1 | MissingHeader | Ficheiro sem header "Crystalline Lineage" |
| V2 | MissingTests | Componente sem ficheiro de teste correspondente |
| V3 | ForbiddenImport | Import viola topologia de camadas |
| V4 | ForbiddenSymbol | Uso de símbolo proibido na camada (ex: `std::fs` em L1) |
| V5 | PromptDrift | `@prompt-hash` diverge do conteúdo actual do prompt |
| V6 | PromptStale | Interface pública diverge do snapshot no prompt |
| V7 | OrphanPrompt | Prompt sem materialização correspondente |
| V9 | PubLeak | Import acede a subdirectório interno de L1 |
| V11 | DanglingContract | Trait em `L1/contracts/` sem `impl` em L2 ou L3 (configurável, ADR-0014) |
| V13 | MutableStateInCore | Estado global mutável em L1 |
| V14 | ExternalTypeInContract | Externo não autorizado em assinatura pública de L1 |

### Warnings

| Código | Nome | Descrição |
|--------|------|-----------|
| V12 | WiringLogicLeak | Declaração de tipo em L4 que não é adapter |

---

## 2. Workflow de desenvolvimento

O ciclo de cada passo segue uma estrutura consistente:

```
┌─────────────────────────────────────┐
│  1. Pré-condições                   │
│     - cargo test (baseline)         │
│     - crystalline-lint . (zero)     │
│     - Ler ADRs relevantes           │
├─────────────────────────────────────┤
│  2. Diagnóstico (bash)              │
│     - grep/find no oráculo          │
│     - Reportar API real             │
│     - STOP: reportar antes de agir  │
├─────────────────────────────────────┤
│  3. ADR (se necessário)             │
│     - Uma decisão = um ADR          │
│     - Antes de qualquer código      │
├─────────────────────────────────────┤
│  4. Implementação                   │
│     - Testes PRIMEIRO (devem falhar)│
│     - Código depois (testes passam) │
│     - Header de linhagem            │
├─────────────────────────────────────┤
│  5. Verificação                     │
│     - cargo test -p typst-core      │
│     - cargo test -p typst-infra     │
│     - cargo build                   │
│     - crystalline-lint .            │
│     - crystalline-lint --fix-hashes │
├─────────────────────────────────────┤
│  6. Relatório                       │
│     - Diagnóstico real vs esperado  │
│     - Contagem de testes            │
│     - Go/No-Go para próximo passo   │
└─────────────────────────────────────┘
```

### Actores no workflow

- **Diego (humano)**: define prioridades, reporta diagnósticos, decide Go/No-Go
- **Claude (chat)**: gera ADRs, prompts L0, prompts de materialização
- **Claude Code**: executa implementação, testes, verificação
- **Gemini**: auditor externo periódico (auditoria após Passo 22 gerou DEBT-1 a DEBT-6)

---

## 3. Padrões de implementação

### Header de linhagem (obrigatório)

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/<nome>.md
//! @prompt-hash <sha256[0..8]>
//! @layer L<n>
//! @updated YYYY-MM-DD
```

### Testes primeiro

A ordem é: escrever testes a partir dos critérios do prompt → verificar que falham → implementar → verificar que passam. Um teste que passa sem código de produção é um teste errado.

### Padrão B3 — blanket implementation

Para contornar a limitação de `#[comemo::track]` com supertraits:

```rust
#[comemo::track]
pub trait TrackedWorld {
    // Redeclara todos os métodos de World
}

impl<T: World> TrackedWorld for T {
    // Delega para World
}
```

### `eval_for_test<W: TrackedWorld>`

Padrão para testes que precisam de avaliação: genérico sobre `TrackedWorld`, permitindo mocks em L1.

### `Arc::ptr_eq` para PartialEq

Tipos wrapped em `Arc` usam `Arc::ptr_eq` para igualdade, evitando comparação profunda.

### Debt registration before code

Dívida técnica é registada em `DEBT.md` antes de escrever implementação, nunca depois. Cada item tem passo estimado de resolução.

### Divergências intencionais do original

- `calc` como `Value::Dict` em vez de `Value::Module` (simplificação documentada)
- `Content::Sequence` usa `Arc<[Content]>` com `PartialEq` manual
- `Source` tem `content_hash: u64` via early hashing (ADR-0031)
- Bullet usa `-` ASCII no Layouter como fallback até DEBT-5

---

## 4. Restrições operacionais

| Restrição | Detalhe |
|-----------|---------|
| Pasta `00_nucleo/materialization/` | Acesso só quando explicitamente indicado com path completo |
| `lab/typst-original/` | Oráculo imutável — nunca modificar |
| Versionamento de prompts | Prompts corrigidos recebem sufixo `v2`, `v3` |
| Um ADR por crate | Crate nova em `[l1_allowed_external]` requer ADR aprovado antes de código |
| Granularidade máxima | Passos divididos quando scope é largo demais |

---

## 5. Métricas do projecto

| Métrica | Valor (Passo 27) |
|---------|-----------------|
| Testes L1 | ~350+ |
| Testes L3 | ~30+ |
| Total | ~380+ |
| Violations do linter | 0 |
| ADRs produzidos | 31 |
| ADRs revogados | 2 (ADR-0007, ADR-0028) |
| Crates autorizadas em L1 | 10 |
| Passos concluídos | ~27 de ~120 |
| Progresso estimado | ~22% |
