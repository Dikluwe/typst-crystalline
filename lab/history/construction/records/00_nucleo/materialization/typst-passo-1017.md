# Passo 1017 — Fechar `vanilla_type_name` como canónica; remover `long_type_name`

**Tipo**: Dedup, fluxo contínuo (ADR-0107 — mesma mecânica, sem mudança de comportamento
observável, prova de equivalência já feita no Passo 1015).
**Decisão do dono**: canónica = `vanilla_type_name` (tabela explícita de 36 arms,
`operators/error_formatting.rs`). Razão registada: falha explícita quando `Value` ganha
variante nova é preferível a herança silenciosa de nome via `type_name()`.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1016.

---

## Fase A — Confirmar as 4 cópias restantes antes de tocar

Do Passo 1015: `long_type_name` (`bindings/access.rs`, canónica actual, `pub(crate)`) e
três `vanilla_type_name` (`stdlib/loading.rs`, `stdlib/pdf.rs`, byte-idênticas entre si;
`operators/error_formatting.rs`, tabela de 36 arms, `pub(crate)`, equivalência já provada
arm a arm no P1015).

```bash
grep -n 'fn long_type_name\|fn vanilla_type_name' \
  01_core/src/compiler/eval/bindings/access.rs \
  01_core/src/compiler/eval/operators/error_formatting.rs \
  01_core/src/compiler/stdlib/loading.rs \
  01_core/src/compiler/stdlib/pdf.rs
```
Confirmar que o estado bate com o relatório do P1015 — se algo mudou entretanto (outro
passo pode ter tocado nestes ficheiros), reportar a diferença antes de prosseguir.

## Fase B — Consolidar

Canónica: `eval/operators/error_formatting.rs::vanilla_type_name` (`pub(crate)`).

1. Remover a implementação em `bindings/access.rs`; todos os chamadores passam a importar
   `vanilla_type_name` de `operators::error_formatting`.
2. Remover as duas cópias em `stdlib/loading.rs` e `stdlib/pdf.rs`; mesmo import.
3. Actualizar os 4 pontos de chamada (`eval/mod.rs`, `call_dispatch.rs`, `join.rs`,
   `stdlib/*` — confirmar lista exacta por `grep`, o P1015 já a levantou parcialmente)
   que hoje chamam `long_type_name` para chamar `vanilla_type_name`.
4. Confirmar que as mensagens de erro produzidas não mudam de texto — `vanilla_type_name`
   e `long_type_name` já foram provadas equivalentes no P1015 (bijectividade de
   `type_name()`); esta troca não deve alterar nenhuma mensagem existente.

## Fase C — L0

- `eval/operators/error_formatting.md` — passa a declarar-se ponto único de verdade,
  citando a prova de equivalência do P1015 (bijectividade, 36/36) como a razão por que a
  troca de `long_type_name` para esta forma não altera comportamento.
- `eval/bindings/access.md` — remove a secção que se declarava dona (adicionada no P1015);
  a função já não existe aqui.
- `stdlib/loading.md`/`stdlib/pdf.md` — se citavam a função localmente, corrigir para
  referenciar a canónica.

## Fase D — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — mesma mecânica do P1015, agora na direcção inversa.

---

## Resultado esperado

Uma única implementação (`vanilla_type_name`, tabela completa), zero cópias residuais,
todos os L0 dos antigos donos corrigidos. Seis implementações históricas (P1015) reduzidas
a uma.
