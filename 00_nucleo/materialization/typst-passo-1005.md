# Passo 1005 — Renomear `engine::` → `compiler::` (mecânico, sem gate)

**Tipo**: Renomeação mecânica, zero mudança de comportamento. **Sem gate ADR-0127** — não
altera contrato público nem comportamento observável, só o caminho/nome do módulo. Mesma
classe de mudança do rename anterior `rules/` → `engine/` (commit `6636c5ea6`, já feito
sem controvérsia).
**Motivo**: `engine::` não tem equivalente vanilla (confirmado — o vanilla separa em
crates: `typst-syntax`, `typst-eval`, `typst-layout`, sem facade agregadora com esse
nome). `compiler::` é mais descritivo do que a facade realmente é (lexer→parse→eval→
layout). Confirmado com o dono: liberdade total para nomear, sem nome vanilla a respeitar.
**Efeito colateral desejado**: resolve a ambiguidade de nome já registada (Passo 1000
discussão) entre `engine::` (módulo/namespace) e `entities::Engine<'a>` (struct de
contexto threading através do eval). **`entities::Engine<'a>` NÃO é tocado por este
passo** — mantém-se, porque o seu nome tem paridade nominal deliberada com
`typst_library::engine::Engine` do vanilla (ADR-0044: "Nomes batem com vanilla"). Depois
deste passo, `compiler::` (pipeline) e `Engine<'a>` (contexto) deixam de colidir por nome.

**Pré-condição**: `git status` limpo. Confirmar HEAD ≥ Passo 1004 (inclui o fatiamento de
`operators.rs` do Passo 1002 — este rename tem de mover também os ficheiros/prompts novos
criados lá).

---

## Fase A — Inventário exaustivo, antes de qualquer mudança

**Cuidado de desambiguação**: "engine" também aparece como palavra comum em prosa
(inglês/português), em nomes não relacionados (`entities::Engine`, `FontMetrics`
mencionar "engine" em comentário, etc.). O inventário tem de distinguir:

1. **Caminho de módulo/crate** (`crate::engine::`, `use ... engine::`, `mod engine`,
   ficheiros sob `01_core/src/engine/`) — **estes mudam**.
2. **Caminho de prompt** (`00_nucleo/prompts/engine/...`, `@prompt
   00_nucleo/prompts/engine/...` nos headers) — **estes mudam**.
3. **`entities::Engine<'a>` / `entities/engine.rs` / `entities/engine.md`** — **não
   mudam** (é outra coisa, ver acima).
4. Menções em prosa dentro de ADRs/relatórios antigos que citam `engine::` como parte de
   uma explicação histórica — **não mudam** (ADRs ficam fora de âmbito, decisão já
   tomada; vão ficar com paths desactualizados, mesma situação que já existe com
   referências antigas a `rules/`).

Comandos:
```bash
# 1. Módulo de código
find 01_core/src/engine -type f | sort
grep -rln 'crate::engine::\|use crate::engine\|mod engine' 01_core 02_shell 03_infra 04_wiring

# 2. Prompts
find 00_nucleo/prompts/engine -type f | sort
grep -rln '@prompt 00_nucleo/prompts/engine/' 01_core 02_shell 03_infra 04_wiring 00_nucleo/prompts

# 3. Confirmar que entities::Engine fica de fora do escopo do grep acima
grep -n 'entities::engine\|entities/engine' <(grep -rl 'engine' 01_core/src) 2>/dev/null || true
```

Produzir lista completa antes de tocar em nada — contagem total de ficheiros `.rs`
afectados, ficheiros de prompt afectados, e ocorrências de `@prompt`/`use` a corrigir.

## Fase B — Renomear directórios (preservar história)

```bash
git mv 01_core/src/engine 01_core/src/compiler
git mv 00_nucleo/prompts/engine 00_nucleo/prompts/compiler
```

Se `crystalline.toml` ou qualquer config do linter referenciar `engine` como nome de
módulo/camada mapeado, actualizar aí também (confirmar por leitura antes, não presumir
que existe).

## Fase C — Substituição mecânica de referências

Para cada ficheiro `.rs` que cita `crate::engine::` ou `use crate::engine`: substituir
por `crate::compiler::`. Para cada header `@prompt 00_nucleo/prompts/engine/...`:
substituir por `@prompt 00_nucleo/prompts/compiler/...`. Para `mod engine;` em
`01_core/src/lib.rs` (ou onde for declarado): `mod compiler;`.

Não usar substituição cega de string "engine"→"compiler" em todo o repositório — só nos
padrões confirmados na Fase A (caminho de módulo, caminho de prompt). Confirmar caso a
caso onde a ferramenta de substituição em massa hesitar (ex.: comentários em prosa que
mencionem "o engine" como palavra comum, não como caminho — esses ficam como estão, ou
são reescritos à mão se fizer sentido, não é obrigatório mudar prosa).

## Fase D — Hashes e validação

```bash
crystalline-lint --fix-hashes .
crystalline-lint .
cargo test --workspace
```

Zero violations esperado, zero regressão de testes (rename puro, sem mudança de lógica).

## Fase E — O que fica intencionalmente por tocar

- `entities/engine.rs`, `entities/engine.md` — inalterados (ver nota do cabeçalho).
- ADRs — inalterados (decisão já tomada: ADRs ficam fora até à paridade fechar). Nota a
  registar: `ADR-0104`/`ADR-0109` já citavam paths `rules/` desactualizados (achado do
  Passo 999); este passo acrescenta mais uma geração de paths desactualizados (`engine/`)
  aos mesmos ADRs — aceite como o mesmo tipo de dívida, não corrigido agora.
- Relatórios de passos antigos (P1000-P1004) — inalterados, são registo histórico do
  estado no momento em que foram escritos.

## Resultado esperado

`01_core/src/compiler/` e `00_nucleo/prompts/compiler/` substituem `engine/` em todo o
código e prompts activos; zero regressão; `entities::Engine<'a>` permanece com esse nome,
agora sem colisão conceptual com o módulo. Relatório final com o hash do commit.
