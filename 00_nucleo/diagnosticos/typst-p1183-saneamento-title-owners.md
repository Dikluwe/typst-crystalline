# P1183 — saneamento dos owners de `Title`

**Executado em:** 2026-08-25T21:37:09-03:00 a 2026-08-25T21:40:41-03:00  
**HEAD:** `00f402e875956304aa435f749a251f359287e2ba`  
**Estado:** working tree não commitado; índice vazio  
**Linter:** `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256
`eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d`

## Proveniência da árvore

Antes do lote, `git diff HEAD --stat` continha somente 23 linhas adicionadas e
2 removidas em `CLAUDE.md`, além de 2 linhas adicionadas em `crystalline.toml`;
os artefatos P1181–P1183 e ADR-0129 já estavam não rastreados. O lote preservou
essas alterações e manteve o índice vazio.

## RED estrutural

`crystalline-lint --checks v15,v26 --fail-on warning .` foi executado duas vezes.
Os outputs foram byte-idênticos, SHA-256
`f1feeb0620caa94210e3e6b3aade45f429d5d04729d5570dacde28e98134b70a`,
com V15=24, V26=0 e exatamente dois consumers produtivos de
`compiler/stdlib_audit_methodology.md`: os `title.rs` de entidade e layout.

## Owners individualizados

| Prompt L0 | Consumer único | Hash do Código | `@prompt-hash` efetivo |
|---|---|---:|---:|
| `prompts/entities/elements/title.md` | `entities/elements/title.rs` | `ef2b41ab` | `e8135432` |
| `prompts/compiler/layout/title.md` | `compiler/layout/title.rs` | `88f73a98` | `daaeeb55` |

Os dois hashes de código foram reproduzidos removendo somente a própria linha
`@prompt-hash` do source. A execução revelou que a fórmula de `prompt_hash`
escrita no P1183 estava incorreta: o linter usa o SHA-256 do arquivo L0 completo,
incluindo `Hash do Código`. A primeira tentativa baseada na fórmula do passo
produziu V5 focal; os valores acima são os hashes efetivos confirmados pelo
linter e eliminaram esse V5.

## Reclassificação documental

O conteúdo substantivo de
`00_nucleo/prompts/compiler/stdlib_audit_methodology.md` foi preservado e
reclassificado em
`00_nucleo/diagnosticos/typst-stdlib-audit-methodology.md`. O diagnóstico:

- não contém `Hash do Código`;
- declara que não é owner nem legitima código;
- registra o path anterior e a reclassificação P1183;
- subordina o registro histórico às ADR-0107/0108 e regras vigentes.

O path antigo foi removido de `prompts/`. Antes do repoint, busca produtiva
encontrou somente os dois headers focais; depois do repoint, não encontra nenhum.
Referências históricas em passos e diagnósticos foram preservadas.

## GREEN estrutural

- V15: 24 → 23;
- V26: 0 → 0;
- V5: 421 → 419;
- ambos os `title.rs`: ausentes do output V5 final;
- o grupo `stdlib_audit_methodology.md`: ausente de V15;
- `--fix-hashes --dry-run .`: exit 2, bloqueado pelas 23 colisões V15 restantes;
- comparação do status antes/depois do dry-run: idêntica, zero writes.

Os dois sources foram comparados contra HEAD removendo apenas as linhas
`@prompt` e `@prompt-hash`; o restante é byte-idêntico. Nenhum corpo Rust,
assinatura, default, fase ou comportamento mudou. Nenhum Núcleo Tekt foi criado.

## Testes e build

- `cargo test -p typst-core entities::elements::title`: 2 passed, 0 failed;
- `cargo test -p typst-core compiler::layout::tests -- title`: 719 passed,
  0 failed; o filtro não foi vazio;
- `cargo build`: GREEN;
- `git diff --check`: limpo;
- `git diff --cached --quiet`: GREEN, índice vazio.

Warnings preexistentes do workspace permanecem fora do escopo.

## Conclusão

P1183 fecha uma colisão de ownership sem mudança funcional: entidade, layout e
metodologia agora têm categorias e owners coerentes com ADR-0129. Restam 23
colisões V15. O próximo lote previsto é P1184, separando os owners de
`03_infra/src/lib.rs` e `03_infra/src/integration_tests.rs` atualmente unidos por
`00_nucleo/prompts/infra.md`.
