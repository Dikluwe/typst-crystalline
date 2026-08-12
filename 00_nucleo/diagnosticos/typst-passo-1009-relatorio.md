# Passo 1009 — Relatório final

**Data**: 2026-08-12  
**Commit de base**: `5bb958bb3` (WIP: outputs e relatórios dos passos 1000-1009)  
**Ficheiros alterados**:

- `01_core/src/compiler/eval/mod.rs` — declaração do novo submódulo
- `01_core/src/compiler/eval/rules.rs` — hub delega loop α ao novo nó
- `01_core/src/compiler/eval/show_rule_termination.rs` — **novo nó**
- `01_core/src/compiler/eval/tests.rs` — 3 testes P348/P350c actualizados para o novo Desfecho 2
- `00_nucleo/prompts/compiler/eval/show_rule_termination.md` — **L0 do novo nó**
- `00_nucleo/diagnosticos/typst-passo-1009-fase-a.md` — auditoria Fase A

---

## Resumo

Fatiámos o mecanismo de paragem do loop α de show rules para o nó
`compiler/eval/show_rule_termination.rs`. O hub `rules.rs` mantém o matching de
selectores, a travessia `map_content`, o wrapping de show-set, e as regras
Regex/Text; o novo nó contém exclusivamente a estratégia de terminação.

### Decisão de arquitectura

A tentativa inicial de um **loop α global** (aplicar todas as regras a todo o
conteúdo em cada iteração) provocou crescimento exponencial de nested
Strong/Emph no teste `show_rule_encadeamento_duas_regras` (heading→strong,
strong→emph). A causa: `map_content` re-processava nós filhos criados por
transformações em iterações seguintes, algo que o loop α **por nó** do código
original evitava.

Mantivemos portanto a semântica do loop α **por nó**, extraindo apenas o
controlo de terminação para o novo nó. Isso preserva o comportamento existente
e ainda acrescenta a detecção de ciclos por histórico de `morph_canon`.

### Algoritmo revisto

`show_rule_termination::run_show_rule_loop` recebe:

- `initial: Content` (um nó);
- `apply_one_step: FnMut(&Content) -> SourceResult<Option<(Content, RuleId)>>`;
- `max_depth: usize`;
- `full_error: bool`.

Devolve `SourceResult<(Content, bool)>` onde o bool indica se alguma regra
foi aplicada.

A cada passo:

1. Aplica uma regra (`apply_one_step`).
2. Se nenhuma casa → **Desfecho 1** (ponto-fixo local).
3. Compara `morph_canon` do output com a anterior.
4. Se igual → **Desfecho 1b** (ponto-fixo morfológico).
5. Se a forma canónica já foi vista → **Desfecho 2** (`show rule cycle detected`).
6. Se atingir `max_depth` → **Desfecho 3** (`maximum show rule depth exceeded`,
   byte-idêntica ao vanilla, com hints vanilla + hint classificatório sob
   `full_error`).

### Ajustes de testes

Três testes existentes (P348 e dois de P350c) esperavam a mensagem de
profundidade para um ciclo a→b→a→…. Como o novo mecanismo deteta esse ciclo
antes do teto, actualizámos os testes para validar o Desfecho 2:

- `p348_show_recursao_ciclo_detectado_antes_do_teto`
- `p350c_flag_off_ciclo_detectado_antes_do_teto`
- `p350c_flag_on_ciclo_detectado_antes_do_teto`

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4963 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

```
crystalline-lint .
```

Zero erros. Dois warnings V7 pré-existentes (`auditar-spec.md`,
`package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- O hub `rules.rs` ainda acumula múltiplas responsabilidades (font-dict,
  selector matching, intercept, realize). Foram identificados como nós futuros
  na Fase A (`font_dict`, `selector_matching`), mas não materializados neste
  passo.
- O mecanismo de paragem assume `apply_show_rules` fora de `context`. Se isso
  mudar, a decisão tem de ser revista (gatilho de reabertura registado no L0).
