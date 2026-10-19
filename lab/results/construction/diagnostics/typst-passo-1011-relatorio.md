# Passo 1011 — Relatório final

**Data**: 2026-08-12  
**Commit de base**: `d0a0063f0` (fix(P1009): remove if/else redundante na criação do history de full_error)  
**Ficheiros alterados**:

- `01_core/src/compiler/eval/mod.rs` — declaração dos novos submódulos `font_dict` e `selector_matching`
- `01_core/src/compiler/eval/rules.rs` — hub delega parsing de font-dict e matching de selectores aos novos nós; removeu-se código e testes movidos
- `01_core/src/compiler/eval/font_dict.rs` — **novo nó**
- `01_core/src/compiler/eval/selector_matching.rs` — **novo nó**
- `00_nucleo/prompts/compiler/eval/font_dict.md` — **L0 do novo nó**
- `00_nucleo/prompts/compiler/eval/selector_matching.md` — **L0 do novo nó**
- `00_nucleo/prompts/compiler/eval.md` — secção de submódulos atomizados de `rules.rs`; hash actualizado
- Filhos de `eval.md` em `01_core/src/compiler/eval/` — resselo de `@prompt-hash` via `crystalline-lint --fix-hashes`

---

## Resumo

Continuámos o fatiamento de `compiler/eval/rules.rs` iniciado no Passo 1009, materializando os dois nós restantes identificados na Fase A desse passo: `font_dict` e `selector_matching`.

### Nó `font_dict`

Responsável pelo parsing do argumento `text.font`:

- `parse_font_dict_named_fields` — formato vanilla `(family: "…", variant: "…", weight: "…", style: "…", fallback: true)` (P414).
- `parse_font_dict_legacy` — formato cristalino antigo `("Name": "Variant")` ou `("Name": ("V1", "V2"))` (P407).
- `variants_from_value` — helper partilhado que valida strings/arrays de strings.

As funções de topo recebem `Scopes`, `EvalContext` e `Engine` porque precisam de avaliar as expressões dos camores/valores via `eval_expr`. Apesar disso, a natureza do nó é declarativa: transforma sintaxe de configuração de fonte numa lista de dicionários normalizados.

### Nó `selector_matching`

Responsável por todas as operações puras de matching e conversão de selectores usadas pelo hub:

- `query_selector_to_show_selector` — converte `entities::selector::Selector` (query) para `entities::show::Selector` (show rule).
- `selector_matches` — casa um `Content` contra um `Selector`; puro (`&Content`, `&Selector` → `bool`).
- `is_node_rule` — decide se um selector viaja pela travessia de nós (`map_content`).
- `splice_text_rule_matches` — fatia texto nas ocorrências de um padrão para aplicação de show rules de texto (P790).

Moveram-se também os helpers privados `values_eq_semantic` e `is_styled_origin`, que só eram usados por `selector_matches`.

### Hub `rules.rs`

O hub ficou reduzido a:

- dispatchers de set/show-rule (`eval_set_rule`, `eval_show_rule`);
- interceptadores (`intercept_content`, `intercept_labelled`);
- realização de parágrafos (`realize_paragraphs`);
- helpers de erro e constantes de configuração de texto.

Tamanho: **1920 linhas** (de 2659), ou seja, ~739 linhas movidas para os novos nós.

---

## Decisão de arquitectura

Mantivemos a **Opção B da ADR-0109**: o `match`/dispatcher fica magro no hub e delega em free functions no arquivo da unidade. Não houve alteração de contratos públicos além da criação dos novos módulos; as funções continuam a ser `pub(crate)` e chamadas apenas por `rules.rs`.

Critérios de separação confirmados:

1. **Isolamento de teste**: `selector_matching` é puro (`Selector`/`Content` → `bool`); `font_dict` é parsing declarativo com fallback legacy.
2. **Co-mudança**: `font_dict` já tinha histórico de mudança independente (P836/P407/P414); `selector_matching` evolui com regras de selector/show-rule.
3. **Sem import reverso**: os novos nós não importam `rules.rs`.

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

Total: **5834 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Dois warnings V7 pré-existentes (`auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- O hub `rules.rs` ainda acumula as responsabilidades de dispatch de set/show-rules, intercept e realize. Não identificámos neste passo outro nó com critérios tão claros quanto `font_dict` e `selector_matching`; o ficheiro está agora no tamanho certo para permanecer como hub.
- Os gatilhos de reabertura dos novos nós estão registados nos respectivos L0s: novo formato de font-dict, novo tipo de selector, ou mudança de fase eval↔layout.
