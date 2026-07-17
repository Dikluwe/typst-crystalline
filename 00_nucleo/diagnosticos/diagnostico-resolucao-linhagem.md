# Diagnóstico de Resolução de Linhagem e Integridade de Prompts

Este relatório serve como registro histórico das ações executadas para corrigir todas as inconformidades de linhagem (`V5` - Drift de Hash e `V7` - Prompts Órfãos) identificadas pelo `crystalline-lint`.

---

## 1. Contexto Inicial

No início da análise, o linter reportava múltiplos problemas de conformidade devido a:
* Um prompt de entidade (`shaped_glyph.md`) que descrevia uma struct integrada dentro de outro arquivo (`layout_types.rs`), gerando drift e conflito de linhagem.
* Um prompt duplicado de layout de referência (`layout/ref.md`).
* Múltiplos prompts de sub-especificações (como show rules com regex, aritmética decimal e dicionário de fontes) declarados em lote em arquivos Rust que não suportam múltiplas diretivas `@prompt` individuais no linter.

---

## 2. Ações Executadas e Arquivos Modificados

### A. Atomização da Entidade `ShapedGlyph`
Para cumprir a regra de relacionamento 1:1 entre código Rust e especificações L0, separamos o tipo do seu arquivo agregador original:
* **Criado**: `01_core/src/entities/shaped_glyph.rs`
  * Isolou a struct `ShapedGlyph` e seus métodos associados.
  * Declarou a linhagem com `@prompt 00_nucleo/prompts/entities/shaped_glyph.md`.
* **Modificado**: `01_core/src/entities/layout_types.rs`
  * Removeu a definição antiga da struct.
  * Adicionou o re-export `pub use crate::entities::shaped_glyph::ShapedGlyph;` para evitar quebras em consumidores externos.
* **Modificado**: `01_core/src/entities/mod.rs`
  * Registrou o novo submódulo: `pub mod shaped_glyph;`.

### B. Correção do Prompt de Operações (`ops.md`)
* **Modificado**: `01_core/src/engine/eval/operators.rs`
  * Atualizou o cabeçalho de linhagem para apontar diretamente para `00_nucleo/prompts/engine/eval/ops.md`, eliminando a orfandade deste prompt.

### C. Declaração de Exceções de Sub-prompts
Para os prompts de especificações específicas de comportamento que não possuem arquivos Rust exclusivos de mesma granularidade, a solução recomendada pelo design do projeto é adicioná-los à lista de exceções:
* **Modificado**: `crystalline.toml`
  * Adicionados à tabela `[orphan_exceptions]`:
    * `00_nucleo/prompts/engine/eval/decimal-arithmetic.md` (Aritmética decimal em `operators.rs`).
    * `00_nucleo/prompts/engine/eval/table.md` (Numeração de tabelas em `rules.rs`).
    * `00_nucleo/prompts/engine/show-regex.md` (Show rules com regex em `rules.rs`).
    * `00_nucleo/prompts/engine/style/font-dict.md` (Dicionário de fontes em `rules.rs`).

### D. Remoção de Prompt Redundante
* **Deletado**: `00_nucleo/prompts/engine/layout/ref.md`
  * Removido por ser uma cópia redundante e não referenciada de `00_nucleo/prompts/engine/layout_references.md`.

---

## 3. Estado Final

Após as modificações, a sincronização foi executada com sucesso:
```bash
crystalline-lint --fix-hashes .
```
E a verificação geral reportou conformidade total:
```bash
crystalline-lint .
```
**Resultado do Linter:**
```text
✓ No violations found
```

Todos os testes unitários e de integração foram validados via `cargo test`.
