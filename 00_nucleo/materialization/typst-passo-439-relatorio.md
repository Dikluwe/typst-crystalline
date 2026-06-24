# Passo 439 — relatório: reconciliação ADR-0062 hayagriva e fecho de DEBT-55

**Tipo:** administrativo / reconciliação documental (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `6b56dc999`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

O passo P439 pretendia criar a ADR-0062 hayagriva como pré-condição para
fecho futuro de DEBT-55. Durante a execução verificou-se que a integração
hayagriva já tinha sido materializada no Passo 418:

- `Cargo.toml` workspace e `01_core/Cargo.toml` já incluem `hayagriva = "0.10"`.
- `crystalline.toml` já lista `hayagriva` em `[l1_allowed_external]`.
- `01_core/src/rules/layout/bib_csl.rs` (P418) e
  `01_core/src/rules/eval/bibliography.rs` usam hayagriva/citationberg para
  renderização CSL (built-ins e `.csl` customizados via P420).
- `Content::Bibliography`, `Content::Cite`, `native_bibliography` e
  `native_cite` já existem.

Assim, P439 foi reclassificado como passo de **reconciliação administrativa**:

- **ADR README (`00_nucleo/adr/README.md`):**
  - Secção de reservas actualizada: ADR-0062 promovida a `IMPLEMENTADO`
    (P418) e reconciliada em P439.
  - Tabela "Estado por ADR": linha ADR-0062 alterada de `PROPOSTO` para
    `IMPLEMENTADO`.
  - Distribuição de status: `PROPOSTO` 11 → 10; `IMPLEMENTADO` 31 → 32.

- **Débito (`00_nucleo/diagnosticos/debt/DEBT.md`):**
  - DEBT-55 reclassificado de **PARCIALMENTE RESOLVIDO** para **FECHADO
    (Passo 439)**.
  - Actualizadas secções de estado, contexto, diferença face ao vanilla,
    plano e critério de fecho para reflectir a materialização real.
  - Scope-outs residuais documentados (múltiplas bibliografias, locales
    externos, superscript/subscript, `CitationStyle` enum runtime).

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Não duplicar ADR-0062:** o ficheiro
  `00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md` já existia
  e já declarava status `IMPLEMENTADO` (P418). Criar um segundo ficheiro
  `typst-adr-0062-hayagriva.md` introduziria duplicação de número ADR e
  desactualizaria referências históricas. Optou-se por actualizar o índice
  canónico (README) e fechar a dívida.
- **Fecho de DEBT-55:** com ADR-0062 implementada e código CSL real em L1,
  a pré-condição que mantinha DEBT-55 aberto deixou de existir. A dívida
  foi fechada, preservando scope-outs menores como candidatos futuros não
  reservados.

## Verificação

| Critério | Resultado |
|----------|-----------|
| ADR-0062 reconhecida como `IMPLEMENTADO` no README | ✓ |
| Contagens de status ajustadas (PROPOSTO 10, IMPLEMENTADO 32) | ✓ |
| DEBT-55 reclassificado como FECHADO (P439) | ✓ |
| Estado actual de bibliography + cite documentado | ✓ |
| Scope-outs residuais declarados | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- Índice de ADRs:
  - `00_nucleo/adr/README.md`
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-439.md`
- este relatório.

## Próximo passo

Com DEBT-55 fechado, os débitos em aberto restantes são:
- **DEBT-43** (linter type-level);
- **DEBT-42** (`get_unchecked` bloqueado por benchmark).
