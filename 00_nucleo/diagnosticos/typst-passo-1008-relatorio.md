# Passo 1008 — Triagem: critério-zero nos 5 candidatos restantes do Passo 1003

**Tipo**: Triagem barata — só critério-zero (agregado ou interface?) + confirmação de granularidade do fan-in. Não aplica os 4 critérios completos do P1002.  
**Estado da árvore**: commit `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`, `git status` limpo.  
**Data**: 2026-08-12.

---

## Metodologia

### Critério A — agregado ou interface?

Para cada ficheiro, verificado se define `trait` público com múltiplos implementadores reais:

```bash
grep -n '^pub trait \|^trait ' \
  01_core/src/compiler/eval/bindings.rs \
  01_core/src/compiler/eval/rules.rs \
  01_core/src/compiler/eval/closures.rs \
  01_core/src/compiler/stdlib/structural.rs \
  01_core/src/compiler/stdlib/text.rs
```

### Critério B — fan-in de módulo ou de símbolo?

**Lacuna do método original**: a contagem abaixo conta **linhas de import** (`use ...::rules`), não chamadas. Se um ficheiro importa `apply_show_rules` uma vez e a chama 10 vezes, conta 1. Isso explica a aparente discrepância entre "imports directos do módulo = 1" e "usados 32 vezes" em `eval::rules`.

```bash
for m in eval::bindings eval::rules eval::closures stdlib::structural stdlib::text; do
  echo "--- $m ---"
  grep -rn "use.*compiler::$m\|compiler::$m\|super::$m" \
    01_core/src 02_shell/src 03_infra/src 04_wiring/src 2>/dev/null \
    | grep -v "^01_core/src/compiler/$m" | wc -l
done
```

Para corrigir a lacuna, adicionámos uma métrica complementar: **número de ficheiros distintos que chamam símbolos do módulo**. Esta contagem é obtida procurando pelos nomes dos símbolos exportados (ex.: `apply_show_rules`, `long_type_name`, `native_table`) em todo o código de produção, excluindo o próprio ficheiro e `tests.rs`. O objectivo é ver se o fan-in do DSM reflecte consumo real do módulo como agregador (vários ficheiros usam vários dos seus símbolos) ou se é inflacionado por um único símbolo de interface (trait/tipo).

---

## Resultados por candidato

### 1. `compiler::eval::bindings` (`01_core/src/compiler/eval/bindings.rs`)

| Questão | Resposta |
|---|---|
| **A: agregado/interface?** | **Agregado**. Nenhum `trait` definido. O ficheiro contém funções livres (`fn`/`pub(crate) fn`) — destruturação, field access, method dispatch, assignment, etc. |
| **B: fan-in confirma módulo?** | **Fan-in do DSM = 5** (Passo 1003). **Imports directos do módulo = 0** (contagem de `use`). **Ficheiros distintos que chamam símbolos de `bindings.rs` = 10**: `eval/mod.rs`, `eval/closures.rs`, `eval/control_flow.rs`, `eval/math.rs`, `eval/operators/join.rs`, `stdlib/calc.rs`, `stdlib/eval.rs`, `stdlib/figure_image.rs`, `stdlib/foundations.rs`, `stdlib/plugin.rs`. O fan-in não vem de um único símbolo inflacionado — `bindings.rs` não define `trait`. O símbolo mais referenciado é `long_type_name` (7 ficheiros), seguido de `eval_let`, `destructure_let`, `unknown_variable`, `eval_field_access`, `field_callee_error` (2 ficheiros cada). O resto é esparsamente usado por 1 ficheiro. O DSM agrega estes 10 ficheiros em 5 módulos Rust (submódulos de `eval` e `stdlib` contam como módulos distintos no DSM); a conclusão qualitativa é a mesma: fan-in real do módulo agregador, não de interface. |
| **Veredicto** | **Prosseguir para P1002-completo**. Nota: não é interface; o fan-in é real mas interno a `eval`. |

### 2. `compiler::eval::rules` (`01_core/src/compiler/eval/rules.rs`)

| Questão | Resposta |
|---|---|
| **A: agregado/interface?** | **Agregado**. Nenhum `trait` definido. Contém `apply_show_rules`, `intercept_content`, `eval_set_rule`, `eval_show_rule`, helpers de realização, etc. |
| **B: fan-in confirma módulo?** | **Fan-in do DSM = 2**. **Imports directos do módulo = 1** (contagem de `use`). **Ficheiros distintos que chamam símbolos de `rules.rs` = 6**: `eval/mod.rs`, `eval/markup.rs`, `eval/closures.rs`, `eval/modules.rs`, `stdlib/layout.rs`, `stdlib/text.rs`. Os símbolos mais usados são `intercept_content` (3 ficheiros) e `eval_set_rule` (3 ficheiros); `apply_show_rules` aparece em 2 ficheiros. Nenhum é trait/tipo. O fan-in do DSM (2) subestima o consumo real porque vários submódulos de `eval` são agregados sob o módulo `compiler::eval` na lente. |
| **Veredicto** | **Prosseguir para P1002-completo**. Nota: fan-in baixo, mas uso denso — candidato a hub funcional. |

### 3. `compiler::eval::closures` (`01_core/src/compiler/eval/closures.rs`)

| Questão | Resposta |
|---|---|
| **A: agregado/interface?** | **Agregado**. Nenhum `trait` definido. Contém `apply_func`, `eval_closure_expr`, `eval_func_call`, `eval_args`, etc. |
| **B: fan-in confirma módulo?** | **Fan-in do DSM = 7**. **Imports directos do módulo = 9** (contagem de `use`). **Ficheiros distintos que chamam símbolos de `closures.rs` = 14**: `eval/bindings.rs`, `eval/math.rs`, `eval/mod.rs`, `eval/rules.rs`, `introspect.rs`, `introspect/from_tags.rs`, `introspect/locatable.rs`, `stdlib/collections.rs`, `stdlib/counter.rs`, `stdlib/foundations.rs`, `stdlib/layout.rs`, `stdlib/numbering.rs`, `stdlib/state.rs`, `03_infra/src/pipeline.rs`. `apply_func` aparece em 12 ficheiros; `eval_func_call` em 3; `eval_args` em 2. O fan-in é claramente de módulo agregador, não de um único símbolo de interface. |
| **Veredicto** | **Prosseguir para P1002-completo**. |

### 4. `compiler::stdlib::structural` (`01_core/src/compiler/stdlib/structural.rs`)

| Questão | Resposta |
|---|---|
| **A: agregado/interface?** | **Agregado**. Nenhum `trait` definido. Contém ~30 nativas (`native_strong`, `native_emph`, `native_raw`, `native_heading`, `native_table`, `native_grid`, etc.). |
| **B: fan-in confirma módulo?** | **Fan-in do DSM = 1**. **Imports directos do módulo = 1** (contagem de `use`). **Ficheiros distintos que chamam símbolos de `structural.rs` = 10**: `eval/bindings.rs`, `eval/mod.rs`, `eval/rules.rs`, `introspect.rs`, `math/layout/mod.rs`, `math/layout/spacing.rs`, `stdlib/counter.rs`, `stdlib/foundations.rs`, `stdlib/layout.rs`, `stdlib/mod.rs`. As nativas mais usadas são `native_heading` (6 ficheiros), `native_table` (5), `native_strong`/`native_emph`/`native_raw`/`native_link`/`make_math_module` (4 cada). O fan-in baixo do DSM reflecte o padrão facade: `stdlib/mod.rs` reexporta tudo, mas o consumo real espalha-se por 10 ficheiros. Não há símbolo de interface inflacionando. |
| **Veredicto** | **Prosseguir para P1002-completo**. |

### 5. `compiler::stdlib::text` (`01_core/src/compiler/stdlib/text.rs`)

| Questão | Resposta |
|---|---|
| **A: agregado/interface?** | **Agregado**. Nenhum `trait` definido. Contém nativas de texto (`native_text`, `native_upper`, `native_lower`, `native_replace`, `native_regex`, etc.). |
| **B: fan-in confirma módulo?** | **Fan-in do DSM = 1**. **Imports directos do módulo = 1** (contagem de `use`). **Ficheiros distintos que chamam símbolos de `text.rs` = 5**: `eval/bindings.rs`, `eval/mod.rs`, `eval/modules.rs`, `eval/rules.rs`, `stdlib/mod.rs`. As nativas mais usadas são `native_text` e `native_smallcaps` (4 ficheiros cada), seguidas de `native_underline`/`native_strike`/`native_overline`/`native_subscript`/`native_superscript` (3 cada). Idêntico a `structural`: fan-in baixo do DSM por causa da facade `stdlib/mod.rs`, mas consumo real disperso. Nenhum símbolo de interface inflaciona. |
| **Veredicto** | **Prosseguir para P1002-completo**. |

---

## Tabela única

| Candidato | A: agregado/interface | B: fan-in confirma módulo? | Veredicto | Nota |
|---|---|---|---|---|
| `compiler::eval::bindings` | Agregado (funções livres) | Sim — 10 ficheiros chamam os seus símbolos; nenhum é trait/tipo | **Prosseguir** | Hub interno do eval; `long_type_name` é o símbolo mais disperso (7 ficheiros) |
| `compiler::eval::rules` | Agregado (funções livres) | Sim — 6 ficheiros chamam os seus símbolos; uso denso de `intercept_content`/`eval_set_rule` | **Prosseguir** | 2720 linhas; fan-in do DSM subestima consumo real por agregação de submódulos |
| `compiler::eval::closures` | Agregado (funções livres) | Sim — 14 ficheiros chamam os seus símbolos; `apply_func` em 12 | **Prosseguir** | Hub central de aplicação de funções |
| `compiler::stdlib::structural` | Agregado (nativas) | Sim — 10 ficheiros chamam as nativas; fan-in baixo do DSM é efeito da facade | **Prosseguir** | Maior ficheiro stdlib (4116 linhas); aglomera markup + table/grid + bib |
| `compiler::stdlib::text` | Agregado (nativas) | Sim — 5 ficheiros chamam as nativas; fan-in baixo do DSM é efeito da facade | **Prosseguir** | 1212 linhas; nativas de texto/fontes |

---

## Limitação metodológica corrigida

A contagem original do critério B usava `grep` por padrões de import (`use.*compiler::$m`). Isto conta **linhas de import**, não chamadas. Como um único import pode preceder dezenas de chamadas não qualificadas, os números "imports directos do módulo" subestimam o consumo real.

Para corrigir, adicionámos a métrica "ficheiros distintos que chamam símbolos do módulo", obtida procurando pelos nomes dos símbolos exportados em todo o código de produção. Esta métrica confirma que o fan-in do DSM, embora por vezes baixo, reflecte consumo real do módulo como agregador — não inflação por um único `trait`/tipo. As conclusões qualitativas mantêm-se; os números de B passam a ser mais informativos.

## Conclusão

**Nenhum dos 5 candidatos é interface (mesma classe de `metrics.rs` do Passo 1006).** Todos são agregados de funções livres e passam no critério-zero.

A triagem não descarta nenhum candidato. A ordem sugerida para P1002-completo, considerando tamanho e impacto:

1. `compiler::eval::rules` — menor fan-in mas funções de show-rules são centrais e o ficheiro é grande (2720 linhas).
2. `compiler::eval::closures` — `apply_func` é hub transversal (49 usos).
3. `compiler::eval::bindings` — maior ficheiro (2235 linhas), mas o acoplamento é interno a `eval`.
4. `compiler::stdlib::structural` — maior ficheiro stdlib (4116 linhas), já identificado no P1003/P1004.
5. `compiler::stdlib::text` — menor (1212 linhas), mas nativas de texto/fontes.

---

## O que este passo NÃO fez

- Não aplicou os 4 critérios completos do P1002 a nenhum candidato.
- Não escreveu nenhum L0 nem moveu código.
- Não decidiu a ordem definitiva entre os que passaram — só filtrou os que não vale a pena tentar (nenhum, neste caso).
