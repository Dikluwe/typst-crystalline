# Passo 1005 — Relatório: renomeação `engine::` → `compiler::`

**Commit final**: `0f5575cd05087e7be69d8575f2ce57163461ede9`
**Data de execução**: 2026-08-12
**Tipo**: renomeação mecânica, zero mudança de comportamento. Sem gate ADR-0127.
**Execução**: 5 agentes em fatias disjuntas (Fase C), com `git mv`, config, varredura
residual e validação executados centralmente.

**Proveniência das medições**: salvo indicação em contrário, todos os números deste
relatório foram medidos no working tree imediatamente antes do commit `0f5575cd0`
(2026-08-12, ~14:00 −03). Os números pós-validação (lint/testes) foram medidos nesse
mesmo estado, com `git diff HEAD --stat` a mostrar apenas
`00_nucleo/prompts/template-prompts.md | 6 ++++++` — alteração **anterior** e alheia a
este passo (campo "Técnica" do template), deliberadamente **não** incluída no commit.

---

## Fase A — Inventário (antes de tocar em nada)

| Alvo | Contagem | Comando de origem |
|------|----------|-------------------|
| Ficheiros `.rs` sob `01_core/src/engine/` | 152 | `find 01_core/src/engine -type f` |
| Ficheiros sob `00_nucleo/prompts/engine/` | 94 | `find 00_nucleo/prompts/engine -type f` |
| Ocorrências `crate::engine` | 315 | `grep -rhoE 'crate::engine' --include=*.rs` |
| Ocorrências `typst_core::engine` | 33 | idem (todas em `03_infra`) |
| Ficheiros `.rs` com `@prompt 00_nucleo/prompts/engine/` | 157 | `grep -rl` |
| Ficheiros de prompt a citar `prompts/engine/` | 31 | `grep -rl 00_nucleo/prompts` |
| Ficheiros totais em âmbito activo | ~207 | `git ls-files` menos histórico |

### Desambiguação — o que **não** é o módulo

Confirmado por medição antes de qualquer substituição:

1. `grep -rhoE 'crate::engine[a-zA-Z0-9_]'` → **vazio**. Não existe identificador
   `crate::engineXxx`; o âncora `crate::engine` é seguro.
2. `crate::entities::engine` / `typst_core::entities::engine` → é a struct
   `entities::Engine<'a>`. Não contém a substring `crate::engine`, logo os padrões não
   a apanham. **Não mudou** (verificado antes/depois por contagem em cada fatia).
3. `base64::engine::general_purpose` em `03_infra/src/export/svg.rs:409` e
   `use base64::Engine;` em `:20` → crate externa. **Não mudou.**
4. `pub mod engine;` em `01_core/src/entities/mod.rs:53` → é `entities::engine`.
   **Não mudou** (continua a existir, verificado).
5. `typst-layout/src/engine.rs` citado em 3 doc-comments → ficheiro do **vanilla**.
   **Não mudou.**
6. `engine/ctx` em `01_core/src/compiler/introspect.rs:23` e `:3182` → nome de par de
   parâmetros, não caminho. **Não mudou.**

---

## Fase B — Directórios

```
git mv 01_core/src/engine        01_core/src/compiler
git mv 00_nucleo/prompts/engine  00_nucleo/prompts/compiler
```

246 renomeações registadas pelo git com deteção de similaridade (`git diff --cached
--name-status | grep -c '^R'` = 246). História preservada — `git log --follow`
atravessa este rename tal como já atravessava `rules/` → `engine/` (`6636c5ea6`).

---

## Fase C — Substituição de referências

### Padrões aplicados

| # | De | Para |
|---|----|------|
| 1 | `crate::engine` | `crate::compiler` |
| 2 | `typst_core::engine` | `typst_core::compiler` |
| 3 | `00_nucleo/prompts/engine/` | `00_nucleo/prompts/compiler/` |
| 4 | `01_core/src/engine/` | `01_core/src/compiler/` |
| 5 | `` `engine/… `` (forma curta em doc-comments e L0) | `` `compiler/… `` |
| 6 | `prompts/engine/` (forma curta) | `prompts/compiler/` |
| 7 | `` `engine::x` `` bare em doc-comment | `` `compiler::x` `` |

Os padrões 5–7 **não constavam do plano do passo**; foram descobertos pelos agentes
durante a Fase C e aplicados depois de verificação caso a caso. Sem eles, ~250
referências de doc-comment e de campo "Arquivos gerados" dos L0 ficariam a apontar para
directórios inexistentes.

### Distribuição por fatia

| Fatia | Ficheiros alterados | Linhas |
|-------|--------------------:|-------:|
| `compiler/{eval,lexer,parse}/` | 31 | 127 |
| `compiler/layout/` | 59 | 140 |
| `compiler/{math,introspect,lang,stdlib,scopes,mod}` | 60 | 219 |
| `entities/`, `lib.rs`, `utils.rs`, `03_infra/`, `benches/`, config, `CLAUDE.md` | 30 | 83 |
| `00_nucleo/prompts/**` | 139 | 478 |
| Varredura residual de forma curta em `.rs` (central) | 25 | — |
| **Commit final** | **357** | **1397 +/−** |

Duas fatias fizeram prova de ausência de alteração colateral extraindo as linhas `-` do
diff, aplicando-lhes os padrões, e comparando com as linhas `+` — igualdade exacta
(127/127 e 219/219). Nas restantes, `insertions == deletions` confirma substituição
in-place sem deriva de contagem de linhas.

### Config e ficheiros de topo

- `crystalline.toml`: `[module_layers] engine = "L1"` → `compiler = "L1"`;
  `[l1_ports] engine = "engine"` → `compiler = "compiler"`; 10 chaves de excepção de
  órfão reapontadas de `00_nucleo/prompts/engine/…` para `…/compiler/…`. O ficheiro
  ficou com **zero** ocorrências da palavra `engine`.
- `01_core/src/lib.rs`: `pub mod engine;` → `pub mod compiler;`, reposicionado de
  linha 8 para linha 7 (as declarações estão em ordem alfabética; `compiler` <
  `contracts`).
- `CLAUDE.md`: 5 exemplos de path actualizados. **`AGENTS.md` é um symlink para
  `CLAUDE.md`** (modo `120000` no índice), não uma cópia — actualiza-se sozinho.

---

## Fase D — Validação

Todos medidos no working tree correspondente ao commit `0f5575cd0`:

| Verificação | Resultado |
|-------------|-----------|
| `cargo build --workspace` | ✅ ok (`Finished dev profile in 14.13s`) |
| `crystalline-lint .` antes de `--fix-hashes` | 187 warnings: 185 × V5 (deriva de hash), 2 × V7. **Zero errors, zero fatais.** |
| `crystalline-lint --fix-hashes .` | `0 drift warnings remaining` |
| `crystalline-lint .` depois | **2 warnings**, ambos V7 |
| `cargo test --workspace` | **5828 passed, 0 failed, 3 ignored** |

### Sobre os 2 V7 restantes

`00_nucleo/prompts/auditar-spec.md` e `00_nucleo/prompts/infra/package_version_resolution.md`
são prompts órfãos. **Pré-existentes, não causados por este passo** — verificado com
`git grep -l '<nome>' HEAD -- '*.rs'` → 0 referências já em `HEAD` (`302b1de85`).
`auditar-spec.md` está untracked (documento de trabalho do dono).

### Sobre a contagem de testes

O passo pede "zero regressão de testes". Como não havia baseline registado nesta
sessão, a prova é estrutural em vez de temporal: uma renomeação pura não pode alterar o
número de `#[test]` do repositório.

```
git grep -c '#\[test\]' HEAD -- '01_core/**/*.rs' … → 5828
grep -rc '#\[test\]' 01_core 02_shell 03_infra 04_wiring --include=*.rs → 5828
cargo test --workspace → 5828 passed
```

Os três números coincidem. `HEAD` aqui é `302b1de85` (pré-1005), medido antes do commit.

---

## Verificação final de resíduos

```
git ls-files | grep -vE '^(lab/|tools/|00_nucleo/(adr|materialization|diagnosticos|context|debt-anexos)/|00_nucleo/handoff)' \
  | xargs grep -lnE 'crate::engine|typst_core::engine|prompts/engine/|01_core/src/engine/|`engine/'
```
→ **vazio**. Zero resíduos em âmbito activo.

Carve-outs confirmados presentes depois da renomeação: `pub mod engine;` em
`entities/mod.rs` (1), `01_core/src/entities/engine.rs`,
`00_nucleo/prompts/entities/engine.md`, `base64::engine` em `export/svg.rs` (1).

---

## Fase E — O que ficou intencionalmente por tocar

- **`entities::Engine<'a>`** — nome mantido por paridade nominal com
  `typst_library::engine::Engine` (ADR-0044). Depois deste passo, `compiler::` (o
  pipeline) e `Engine<'a>` (o contexto) deixam de colidir por nome, que era o efeito
  colateral desejado registado no cabeçalho do passo.
- **ADRs** — inalterados. `ADR-0104`/`ADR-0109` já citavam paths `rules/`
  desactualizados (achado do Passo 999); agora acumulam uma segunda geração
  (`engine/`). Aceite como a mesma dívida, não corrigida aqui.
  - **Consequência registada**: `00_nucleo/prompts/entities/plugin_func.md:12` e `:23`
    citam verbatim a redacção da ADR-0109 (`import reverso entities→engine`) e ficaram
    como estavam, para não dessincronizar a citação da fonte. O `CLAUDE.md`, por ser
    documento vivo e não histórico, **foi** actualizado para `entities→compiler`. Há
    portanto uma divergência deliberada entre a ADR (histórica) e o CLAUDE.md (vivo).
- **Relatórios de passos antigos (P999–P1004)** e handoffs — registo histórico.
- **`tools/perf/results/**.log`** — logs de benchmark, fora da topologia (excluídos em
  `crystalline.toml`).

---

## Achados laterais (não corrigidos — deriva prévia, não deste passo)

Dois caminhos em campos "Arquivos gerados" de L0 apontam para ficheiros inexistentes.
Confirmado em `HEAD` (`302b1de85`) que já estavam errados antes da renomeação, com a
grafia `engine/`:

| L0 | Diz | Real |
|----|-----|------|
| `compiler/introspect/extract_payload.md:106` | `01_core/src/compiler/introspect/mod.rs` | `introspect.rs` |
| `compiler/parse.md:6`, `entities/source.md:118` | `01_core/src/compiler/parse.rs` | `parse/mod.rs` |

Deriva prévia de V15. Deixada intacta para não misturar correcção de conteúdo com
renomeação mecânica.

---

## Resultado

`01_core/src/compiler/` e `00_nucleo/prompts/compiler/` substituem `engine/` em todo o
código e prompts activos. Zero regressão. `entities::Engine<'a>` permanece.

**Commit**: `0f5575cd05087e7be69d8575f2ce57163461ede9`
