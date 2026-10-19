# Relatório de Verificação — Passo 801: `utils::protected` — repr de array de 1 elemento (achado P798 #4)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799+P800 (zonas não relacionadas — math layout)
- **Working tree na validação "depois":** P799–P801
- **Hora da Medição:** 2026-07-21 ~16:05 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #4 de P798: `#context [ #counter("mycounter").get() ]` → cristalino `(0)` vs vanilla `(0,)` — representação de array de 1 elemento diverge.

## 2. Diagnóstico e Medição

Fonte original `temp/p798/4_protected.typ` + controlo `#repr(()) #repr((1, 2)) #repr((5,))`: cristalino `() (1, 2) (5)` vs vanilla `() (1, 2) (5,)`. Confirmado: a diferença é **apenas** a vírgula final no caso de 1 elemento; 0 e 2+ já estavam correctos.

Pontos exactos: vanilla `crates/typst-library/src/foundations/array.rs:1188-1200` (`repr::pretty_array_like(&pieces, self.len() == 1)` — trailing comma quando len==1); cristalino `01_core/src/engine/eval/repr.rs` braço `Value::Array` (`format!("({})", items.join(", "))`, sem a regra). O display embutido em markup passa por `value_to_display_content` (`eval/mod.rs:707-713`), que delega em `repr_value` — uma correcção cobre os dois caminhos.

## 3. A Solução Implementada

L0 `stdlib/foundations.md` (linha `array` da tabela de `repr` passa a registar a regra, P801); hash corrigido (`stdlib/foundations.rs` → `17c5d094`). Código: `if items.len() == 1 { format!("({},)", items[0]) }`.

Validação depois: `#context [ #counter("mycounter").get() ]` → `(0,)` == vanilla; controlo `() (1, 2) (5,)` == vanilla (0 e 2+ inalterados).

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `repr_value_array_um_elemento_virgula_final` (novo, `eval/repr.rs`): `"(5,)"`, aninhado `"((5,),)"`, controlos `"()"` e `"(1, 2)"`. Falhou antes em `repr.rs:589`.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4320 passed; 1 ignored → DEPOIS 4321 passed; 1 ignored (total 4322 = +1 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
