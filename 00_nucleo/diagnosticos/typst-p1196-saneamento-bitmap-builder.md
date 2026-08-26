# P1196 — saneamento bitmap glyphs / PDF builder

## Resultado

O owner conjunto foi separado sem alteração de corpos Rust:

- `infra/export/builder.md` permanece proprietário de `builder.rs`;
- `infra/export/bitmap_glyphs.md` passa a ser proprietário de
  `bitmap_glyphs.rs`;
- `_nuclei/export/bitmap-embedding.toml` contém somente identidade,
  deduplicação, metadados de posicionamento e fallback compartilhados.

O builder retém alocação/emissão de recursos PDF. O novo owner retém coleta
por face e normalização raster. O stream continua proprietário da fórmula de
posicionamento.

## Medições reproduzíveis

Medição em `2026-08-26T07:00:14-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
estado exato dos ficheiros alterados é o `git diff HEAD --stat` desta sessão:
44 ficheiros, 172 inserções e 206 remoções; inclui o acumulado P1181–P1196.
Os artefactos novos não rastreados são enumeráveis por `git status --short`.

- Baseline imediatamente anterior ao P1196: V15=11 e V26=0.
- Fechamento: V15=10 e V26=0.
- V5 global=396; nenhum V5 focal em `builder.rs` ou `bitmap_glyphs.rs`.
- Digest efetivo do núcleo:
  `016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def`.
- Hashes efetivos dos prompts: builder `3350b2db`; bitmap collector
  `a26848a5`.
- Os SHA-256 dos dois sources, removidas as linhas `@prompt` e
  `@prompt-hash`, são iguais aos respectivos conteúdos no HEAD:
  builder `b1d422008647e26372a5f5d953d65400e90bd2a2a5c946b86bc0f62b0ec20386`;
  collector `80c0125543be0923c0c492512ff994c659bc1073d1fc1df2356176035ccb67fe`.
- `cargo test -p typst-infra p941_ -- --nocapture`: 3 passados, 0 falhas.
- `cargo build -p typst-infra`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: bloqueado como esperado pelos
  10 V15 restantes; exit 2; digest do diff antes/depois idêntico
  (`5e3071ecd0b79d65a7b1f690a45ff450a93e89846c592d89de54c70a94d7fb59`),
  portanto zero escritas.

## Decisão

P1196 fecha GREEN: reduziu exatamente uma colisão de ownership, não criou
V26, manteve os consumers focalmente selados e preservou integralmente os
corpos produtivos. Os 10 V15 restantes pertencem aos próximos passos e não
foram alterados aqui.
