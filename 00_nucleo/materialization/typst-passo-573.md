---
# P573 — Mapear e resolver o estado completo do directório de trabalho

> **Passo:** 573
> **Data:** 2026-07-05
> **Foco:** P572 resolveu um bloco de código órfão (o de espaço em L1), mas deixou registado, na secção de próximos passos, que outro bloco continua pendente — o trabalho de extracção de codepoints (`builder.rs`, `fonts.rs`, `shaper.rs`, `export/mod.rs`) e uma alteração ao `Cargo.toml` de `04_wiring` que foi revertida "temporariamente" só para o build funcionar durante P572. Antes de continuar com qualquer outro passo, é preciso saber exactamente o que está no directório de trabalho agora, não só a parte que já foi investigada.
> **Tipo:** Sonda directa. Sem código, sem decisões, antes de ter o mapa completo.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P571, P572 (onde uma parte do directório de trabalho já foi mapeada e resolvida; falta o resto).

---

## Sonda

### Ver tudo o que está por commitar, sem excepção

```bash
git status
git diff HEAD --stat
```

Não olhar só para os ficheiros já mencionados nos relatórios anteriores. Listar todos, incluindo os que ainda não foram nomeados em nenhum passo.

### Para cada ficheiro alterado, confirmar a que trabalho pertence

```bash
git diff HEAD -- 03_infra/src/export/builder.rs
git diff HEAD -- 03_infra/src/export/fonts.rs
git diff HEAD -- 03_infra/src/shaper.rs
git diff HEAD -- 03_infra/src/export/mod.rs
git diff HEAD -- 04_wiring/Cargo.toml
```

Para cada um, confirmar se a alteração é a mesma descrita no relatório de extracção de codepoints, ou se há mais do que uma coisa misturada no mesmo ficheiro.

### Confirmar se o build funciona sem nenhuma das alterações pendentes

```bash
git stash push -u
cargo build --workspace
cargo test --workspace
git stash pop
```

Isto confirma se o estado commitado (sem nada pendente) está saudável por si só, antes de decidir o que fazer com o que está pendente.

### Critério de fecho da sonda

- [ ] Lista completa de ficheiros alterados no directório de trabalho, sem excepção.
- [ ] Cada alteração atribuída a um trabalho identificável (extracção de codepoints, ou outro, nomeado).
- [ ] Confirmado que o estado commitado, sem nada pendente, compila e passa os testes.
- [ ] Confirmado se existe mais algum bloco de código órfão, sem L0, para além do já descartado em P572.

---

## Decisão, para cada bloco de trabalho pendente encontrado

Aplicar o mesmo critério já usado em P572:

1. Resolve um problema real, com origem documentada? Escrever o L0 que falte, commitar.
2. É redundante com algo já resolvido de outra forma? Descartar.
3. Não é possível determinar com confiança? Registar como órfão, sem decisão apressada.

---

## Critério de fecho do passo

- [ ] Mapa completo do directório de trabalho produzido.
- [ ] Cada bloco de trabalho pendente classificado num dos três caminhos.
- [ ] `04_wiring/Cargo.toml` — decidido se a versão revertida ou a alterada é a que deve ficar, com razão escrita, não deixado como estava por P572 ("temporariamente").
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p573.md`.
- [ ] Directório de trabalho, no final deste passo, num estado onde cada ficheiro alterado tem uma razão conhecida para estar alterado — nada por explicar.
