# Passo 937 — implementar coverage exata + I/O barato (mmap) juntos, per estudo holístico de P936

**Precede este passo**: `typst-passo-936-relatorio.md` — estrutura do vanilla confirmada por
leitura de código-fonte real e `strace`: mmap barato no arranque + coverage exata (runs de
codepoints, não bitmap de bloco) → `select_fallback` nunca abre face durante o shaping. Hipótese
confirmada: as duas peças são interdependentes, portar só uma não resolve (P935 já mostrou isso —
coverage eager sozinha, sem mmap, piorou o caso comum e não resolveu emoji).

**Pré-condição de árvore**: `git status`. Confirmar estado P933-fixed + reversão de P936 presente
(scan condicional de P927 intacto, `face_covers_char` de P933 intacto).

---

## Fase A — desenhar a implementação conjunta (não separar em dois passos)

1. **Coverage exata**: substituir o bitmap por bloco de 256 codepoints (`Coverage` actual do
   cristalino) por uma representação de runs de codepoints, mesmo formato que P936 já leu no
   vanilla (`info.rs:269-318`, `Vec<u32>` como runs alternadas dentro/fora, busca binária em
   `contains`). Confirmar se dá para reaproveitar essa lógica quase literalmente (per `ADR-0123`
   — fórmula/estrutura de dados que resolve um problema físico real, não "mecânica" a reinventar
   livremente) ou se há alguma razão para o cristalino precisar de uma representação diferente
   (por exemplo, se `Coverage` for consumida por mais lugares que só fallback, confirmar que a
   troca de formato não quebra nada mais).
2. **I/O barato**: adicionar `mmap` (crate `memmap2`, já usada e depois removida em P935 —
   reconsiderar) a `FontSlot`, para ficheiros do disco (fontes embutidas continuam como estão,
   já em memória). `source_bytes()`/`get()` passam a operar sobre o mmap, não `std::fs::read`
   para `Vec<u8>`.
3. Confirmar como as duas peças se encaixam na ordem de execução: a extração de coverage exata
   precisa dos bytes da fonte — com mmap, essa extração no arranque fica barata (per P935, ~50ms
   medidos no programa isolado para 1112 faces via mmap, contra ~4000ms via `std::fs::read`).
   Confirmar se faz sentido extrair a coverage exata **eager no arranque** (como o vanilla) agora
   que o custo de acesso aos bytes deixou de ser proibitivo, ou se ainda vale manter lazy (só na
   primeira falha de cobertura da fonte primária) — decidir com números, não por analogia cega ao
   vanilla.
4. Confirmar se `shared_source`/partilha de bytes entre faces do mesmo `.ttc` (P875, mencionado
   em P935 como secundário) ainda é necessário depois do mmap — mmap já evita cópia para memória
   do processo, então a partilha explícita pode deixar de ser precisa. Confirmar, não presumir.

## Fase A.1 — gate (mudança estrutural em `FontSlot`/`Coverage`/`FontBook`)

Editar os L0s afectados (`fonts.md`, `fontdb.md`, `system-world.md`, possivelmente
`entities/font_book.md` se `Coverage` mudar de representação), sincronizar hashes,
**parar para confirmação do dono antes da Fase B** — mesmo protocolo de sempre nesta frente.

## Fase B — Implementação (protocolo de dois agentes de P898 — mudança estrutural de risco alto,
toca carregamento de fontes usado por todo o pipeline)

1. Agente A escreve testes cobrindo: coverage exata não tem falsos positivos (caso concreto já
   catalogado — grego, `U+03B1`, contra uma fonte que cobre o bloco mas não o glifo, mesmo caso
   de P933); mmap devolve os bytes corretos comparados a `std::fs::read` para a mesma fonte;
   nenhuma face é aberta durante `select_fallback`/`candidates_for_char` depois da coverage estar
   pronta (instrumentação de contagem, mesmo padrão de P890/925/935).
2. Agente B implementa.
3. Revisão do orquestrador — testar pelo menos: um `.ttc` com múltiplas faces (confirmar que o
   mmap parametrizado por face funciona), e o caso de fonte corrompida/ilegível (confirmar que o
   fallback para "sem essa fonte" continua a funcionar, não crasha).
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Medição completa (attestation, `L11`)

**Vanilla real confirmado por string distintiva, não por nome de caminho** — lição de P934, não
repetir esse erro. Medir com warmup suficiente para evitar a variação de ~20% que P936 observou
com `--min-runs 3` (usar `--min-runs 10` ou mais, per a remediação de P936).

1. Os 7 cenários canônicos, `depois/antes` (contra o estado revertido P933-fixed) — confirmar
   zero regressão no caso comum.
2. Os 5 casos UTF-8 (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`, `05-utf8`), `depois/
   antes` **e** `cristalino/vanilla-real` — confirmar quanto da distância de 6.37×-22.22× (medida
   por P936 contra o vanilla real) foi fechada.
3. Repetir a instrumentação `strace`/contagem de aberturas de fonte de P936 no estado novo —
   confirmar que as aberturas se concentram no arranque, como no vanilla, não espalhadas pelo
   shaping.

## Resultado esperado

- Coverage exata e I/O barato implementados juntos, gate cumprido.
- Contagem/distribuição temporal de aberturas de fonte comparável à do vanilla real (concentrada
  no arranque, não espalhada).
- Benchmark completo, 7 canônicos (zero regressão) + 5 UTF-8 (distância ao vanilla real fechada
  ou parcialmente fechada, com números).
- Se sobrar distância depois desta implementação: registar quanto sobra, não forçar fechamento
  total num passo só.
