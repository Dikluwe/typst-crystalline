# Relatório de Execução — Passo 1077: `len` Global — Remoção de Extensão para Paridade com Vanilla Typst — Achado #12 do P1031

**Data**: 2026-08-18
**Passo**: 1077 — `len` Global (Achado #12 do P1031)
**Decisão do Dono (Gate `ADR-0127`)**: Opção (a) — Remoção da extensão global `len` para paridade estrita com o compilador Typst oficial.
**Status**: CONCLUÍDO COM ÊXITO (Extensão removida, métodos `.len()` preservados em paridade de bytes, 100% PASS na suíte)

---

## 1. Contexto, Proveniência e Motivação

### 1.1 Proveniência de `len` Global
O histórico do repositório (`git log -S "native_len"`) revelou que a função global `len` foi introduzida no Passo 10-23, no estágio inicial do bootstrapping do compilador, antes da padronização completa da sintaxe de métodos (`.len()`) da linguagem Typst (Typst 0.11+).

### 1.2 Problema de Divergência e Inconsistência
1. **Divergência com Vanilla Typst**: Na linguagem Typst oficial, `len` **não existe no escopo global** (`#len("abc")` emite `error: unknown variable \`len\``).
2. **Inconsistência Interna**: `#len("ação")` (global) contava codepoints e retornava `4`, enquanto `"ação".len()` (método) contava bytes UTF-8 e retornava `6`. Duas operações com o mesmo nome-base produziam resultados divergentes para a mesma entrada no mesmo compilador.

---

## 2. Ações Executadas (Opção a)

1. **Remoção do Escopo Global**:
   * Removida a definição `scope.define("len", ...)` e o import `native_len` de [`01_core/src/compiler/eval/mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs).
2. **Remoção do Módulo de Stdlib**:
   * Excluído o arquivo `01_core/src/compiler/stdlib/foundations/len.rs`.
   * Removidas as reexportações em [`01_core/src/compiler/stdlib/foundations/mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/foundations/mod.rs) e [`01_core/src/compiler/stdlib/mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/mod.rs).
3. **Limpeza e Atualização de Prompts L0**:
   * Excluído o prompt `00_nucleo/prompts/compiler/stdlib/foundations/len.md`.
   * Atualizado e selado o hub [00_nucleo/prompts/compiler/stdlib/foundations.md](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/foundations.md) (Hash: `bd454e29`).

---

## 3. Medição Diferencial e Validação

### 3.1 Comparativo de Comportamento

| Caso de Teste | Vanilla Typst (`/usr/local/bin/typst`) | Crystalline (Antes) | Crystalline (P1077) | Paridade |
| :--- | :--- | :--- | :--- | :---: |
| `#len("ação")` | `error: unknown variable \`len\`` | `4` ❌ | `error: unknown variable: len` | **Mesma classe de erro** (falha em compilar: variável desconhecida) |
| `#"ação".len()` | `6` (bytes UTF-8) | `6` | `6` | **100% IDÊNTICO** (resultado em bytes) |
| `#(1, 2, 3).len()` | `3` | `3` | `3` | **100% IDÊNTICO** (elementos) |
| `#(a: 1, b: 2).len()` | `2` | `2` | `2` | **100% IDÊNTICO** (entradas) |

---

## 4. Testes e Validação Final

* **Teste Unitário Dedicado**: Adicionado `p1077_len_global_removido_metodos_preservados` em [`01_core/src/compiler/eval/tests.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/eval/tests.rs).
* `crystalline-lint .`: APROVADO (0 erros, 0 avisos de drift).
* `cargo test --workspace`: APROVADO (5.949 testes, 100% PASS).
