# Passo 1001 — Auditoria dos 10 prompts órfãos: conteúdo real + onde a responsabilidade vive no vanilla

**Tipo**: Auditoria — ler, catalogar, **nada de presumir, nada de corrigir**.
**Motivo**: os achados anteriores sobre `decimal-arithmetic.md` e `eval/table.md` foram
reconstruídos a partir de relatórios de passos antigos, não do texto real dos prompts.
Isso não serve de base para decisão nenhuma. Este passo lê o conteúdo actual, literal.
**Pré-condição**: `git status` — confirmar árvore limpa antes de começar.

---

## Lista dos 10 órfãos (do Passo 999, Critério A.3)

```
engine/eval/table.md
engine/eval/decimal-arithmetic.md
engine/show-regex.md
engine/stdlib/grid_hline.md
engine/stdlib/grid_vline.md
engine/stdlib/table_hline.md
engine/stdlib/table_vline.md
engine/style/font-dict.md
infra/package_version_resolution.md
package-spec-dto.md
```

## Parte 1 — Ler cada prompt, literalmente

Para cada um dos 10:
```
view 00_nucleo/prompts/<caminho>
```
Registar, sem interpretar ainda: título, secções presentes, o que a "Instrução" (ou
equivalente) diz que o código deve fazer, e se cita algum ficheiro `.rs` como alvo (mesmo
que esse ficheiro não exista hoje — registar o que o prompt **diz**, não o que é verdade).

## Parte 2 — Confirmar por que é órfão, caso a caso (não presumir que é o mesmo mecanismo)

Para cada um, verificar:
1. O ficheiro `.rs` que o prompt nomeia como alvo existe hoje?
   ```
   find 01_core 02_shell 03_infra 04_wiring -iname '<nome candidato>'
   ```
2. Se existe: por que não tem `@prompt` a apontar para este ficheiro? (grep o header do
   `.rs` encontrado, ver o que ele cita em vez deste prompt)
3. Se não existe: o conteúdo descrito no prompt foi implementado dentro de outro ficheiro
   (like `decimal-arithmetic.md` → `operators.rs`), ou nunca foi implementado de todo?
   ```
   grep -rn '<termo característico do prompt># ex: nome de função, struct>' 01_core 02_shell 03_infra 04_wiring
   ```

**Não assumir que todos os 10 seguem o mesmo padrão dos dois já vistos.** Cada um pode ter
uma causa diferente — nome desactualizado após rename, funcionalidade nunca implementada,
prompt duplicado por engano, etc. Registar a causa real de cada um.

## Parte 3 — Onde a responsabilidade vive no vanilla (grelha do Passo 1000)

Para cada um dos 10, localizar no vanilla (`lab/typst-original/crates/`) onde a
responsabilidade descrita pelo prompt realmente vive:
```
grep -rln '<termo característico>' lab/typst-original/crates/*/src/
```
Registar: crate, ficheiro, e se é o mesmo "nível" que o cristalino usa hoje (ex.: dentro de
`eval` vs fora, como aconteceu com `table.md`) ou nível diferente.

## Parte 4 — Catálogo final (output deste passo, sem corrigir nada)

Uma tabela, um por linha:

| Prompt | Conteúdo real (resumo do que a Instrução diz) | Causa da orfandade | Onde vive no cristalino hoje (se vive) | Onde vive no vanilla | Nota |
|---|---|---|---|---|---|

Sem coluna de "acção recomendada" ainda — isso é decisão do passo seguinte, depois deste
catálogo estar completo e correcto.

---

## O que este passo NÃO faz

- Não corrige nenhum prompt.
- Não move nenhum código.
- Não presume que os 10 casos partilham a mesma causa (mesmo que dois já confirmados
  pareçam parecidos — confirmar os outros 8 individualmente).
