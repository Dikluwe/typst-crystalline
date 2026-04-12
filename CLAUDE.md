# CLAUDE.md — typst-crystalline

Este ficheiro é a diretriz suprema para o assistente de IA neste repositório.
Para decisões arquiteturais específicas: **ler os ADRs em `00_nucleo/adr/`**.

---

## Terminologia Crítica: Passos de Execução vs Prompts L0

Para evitar a corrupção da arquitetura por escrita de código não especificado, é obrigatório distinguir estas duas entidades:

- **Passo de Execução** (ex: `typst-passo-59.md` na raiz): documento tático, logístico e temporário, usado para coordenar tarefas imediatas, depurar erros ou planear a sessão entre o humano e a IA. **Não é o L0.**
- **Prompt L0** (ex: `00_nucleo/prompts/rules/layout.md`): especificação arquitetural pura, perene e definitiva do sistema. **Este é o L0 — a única fonte da verdade que legitima o código.**

**Regra de Ouro (A Trava Arquitetural):**
O assistente **nunca** pode instruir a escrita de código L1/L2/L3 se o Prompt L0 correspondente não existir em `00_nucleo/prompts/` ou estiver desatualizado.

O fluxo de trabalho imutável é:

```text
1. Passo planeia as tarefas → 2. IA redige o Prompt L0 → 3. Humano guarda L0 e calcula Hash → 4. IA escreve Código (L1)
```

---

## ⚠️ Restrição de leitura — pastas de materialização e context

```text
00_nucleo/materialization/
00_nucleo/context/
```

- Não ler estas pastas por iniciativa própria.
- Só aceder quando explicitamente indicado com o path completo.
- Nunca varrer ou listar o conteúdo destas pastas.
- Se uma tarefa parece exigir este contexto mas não referencia um ficheiro explícito: **perguntar antes de agir**.

**Motivo:** estes ficheiros contêm instruções sequenciais históricas. Lê-los fora de contexto injeta estado passado e gera alucinações arquiteturais.

---

## A Arquitetura Cristalina (Tekt)

O código original do compilador está em `lab/typst-original/` (quarentena). A migração acontece gradualmente para as camadas cristalinas. O critério de sucesso primário é `crystalline-lint .` com zero violations.

### Camadas e Topologia de Imports

| Camada | Diretório | Regras de Importação e Propósito |
|--------|-----------|----------------------------------|
| L0 | `00_nucleo/prompts/` | Especificações. Não é código. A origem da linhagem. |
| L1 | `01_core/` | Domínio puro. Zero I/O. Só importa stdlib pura e whitelist. |
| L2 | `02_shell/` | CLI, formatadores. Conhece apenas L1. |
| L3 | `03_infra/` | I/O, filesystem, fontes. Conhece apenas L1. |
| L4 | `04_wiring/` | Composição. Conhece L1, L2, L3. Zero lógica de negócio. |
| lab | `lab/` | Quarentena. Nunca importado por L1–L4. |

### Restrições absolutas do L1

**Nunca:**

- Lê ou escreve ficheiros (`std::fs`), faz chamadas de rede, acede ao relógio (`SystemTime`) ou variáveis de ambiente (`std::env`).
- Tem estado global mutável (`static mut`, `static Mutex<T>`, `static OnceLock<T>`).
- Importa crates externas não declaradas em `[l1_allowed_external]`.

**Permitido** (performance de RAM não é I/O):

- `Arc<T>` em campos de struct (ADR-0029), `Vec`, `Box`, `String`, `HashMap`.
- `EcoString` (ADR-0024), `rustc-hash` / `FxHashMap` (ADR-0018).

---

## Protocolo de Nucleação (obrigatório antes de código)

1. **Auditoria L0:** existe prompt em `00_nucleo/prompts/` para o módulo afetado? Está atualizado face às ADRs vigentes?
2. **Validação L0:** se não existe ou está desatualizado, a IA deve redigir o novo L0 e **PARAR**. Só prossegue quando o humano confirmar que guardou o ficheiro e tem o hash.
3. **Testes primeiro:** escrever os testes no módulo (`#[cfg(test)]`). Confirmar que falham.
4. **Implementação:** escrever o código para os testes passarem.
5. **Linhagem:** adicionar header obrigatório com `@prompt-hash` e `@prompt` corretos.
6. **Validação final:** `cargo build && crystalline-lint .` — zero violations.

---

## Travas do linter e Erros Comuns

| Cód | Nome | Solução |
|-----|------|---------|
| V3 | `ForbiddenImport` | Import viola a topologia de camadas. Reorganize a dependência. |
| V4 | `ImpureCore` | Símbolo de I/O detetado em L1. Mova para L3 e injete via trait. |
| V5 | `PromptDrift` | Hash do prompt diverge. Corra `crystalline-lint --fix-hashes .` após editar L0. |
| V13 | `MutableStateInCore` | Estado global detetado em L1. (`Arc` instanciado em struct não aciona isto.) |
| V14 | `ExternalTypeInContract` | Import externo em L1 não declarado. Padrão: não usar `pub use self::X::Y` em L1. |

---

## ADRs Vigentes — ler antes de propor arquitetura

| ADR | Decisão |
|-----|---------|
| ADR-0018 | `rustc_hash` autorizado em L1 (revoga ADR-0007) |
| ADR-0024 | `EcoString` em `Value::Str` — clone O(1) em `eval()` |
| ADR-0026 | `Content` como enum fechado; `Arc` em `Sequence` |
| ADR-0029 | Pureza física — `Arc` em struct de domínio permitido (revoga ADR-0028) |
| ADR-0030 | Performance de RAM é domínio de L1; corrige ADR-0004/0015 |
| ADR-0031 | Early hashing em `Source`; complementa ADR-0016 |

ADRs revogadas não constam na tabela e não devem ser seguidas.
