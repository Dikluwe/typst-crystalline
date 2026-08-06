# Passo 980 — módulo oráculo de paridade de operador (não a saída principal): primeira transformação, `Tj` em vez de `TJ` sem ajuste real

**Precede este passo**: correção do dono — o ajuste `Tj`/`TJ` (achado da auditoria: vanilla usa
`Tj` como padrão 92.5%, cristalino usa `TJ` como padrão 68.8%, sem nunca ter convergido em nove
rodadas) **não deve ir para o exportador de produção**. Diferente de P979 (agrupamento de
`BT…ET`, que reduz tamanho de arquivo de verdade — benefício real ao utilizador final), a escolha
`Tj` vs `TJ` não muda posição nem tem benefício visível fora do contexto de comparação — serve só
para tornar a paridade de operador mais fácil de verificar. Isso pertence ao **oráculo de
paridade de operador**, não ao caminho normal de compilação.

**Nota de numeração**: isto retoma uma ideia já desenhada antes (prompt original de P975 desta
conversa, "módulo oráculo de paridade de operador, flag `--oracle-report`") que nunca chegou a ser
construída — o número 975 acabou usado, do lado da execução, para outra investigação (P952 §6.4).
Este passo constrói o oráculo, com o ajuste `Tj`/`TJ` como primeira transformação concreta.

**Pré-condição de árvore**: `git status`. Confirmar P979 presente.

---

## Fase A — desenhar o oráculo como caminho separado (gate obrigatório — nova superfície, mesmo
que não seja exposta como recurso normal ao utilizador final)

1. Confirmar onde este código vive — **não** em `stream.rs`/`export/builder.rs` como parte do
   caminho normal de emissão verbose. Candidato: um módulo `03_infra/src/export/oracle.rs` (ou
   `03_infra/src/oracle/`), que recebe a saída já construída pelo modo verboso normal e aplica
   transformações adicionais **só neste caminho** — reescrever `TJ` sem ajuste real como `Tj`,
   e (se fizer sentido juntar) os checks de paridade de operador já desenhados no P975 original
   (proporção `Tr 2`, `BDC`/`EMC`, balanceamento `q`/`cm`/`Q`).
2. Confirmar a forma de activação — uma flag de CLI separada e claramente marcada como ferramenta
   de diagnóstico (`--oracle-pdf` ou nome semelhante), não a flag `--compact` de P956 nem o
   comportamento por defeito.
3. Confirmar se o oráculo gera **só** o PDF ajustado, ou o PDF ajustado **mais** um relatório de
   paridade (o desenho original de P975 incluía os dois) — decidir o escopo desta primeira versão.
4. Editar L0s, sincronizar hashes, **parar para confirmação do dono antes da Fase B** — mesmo que
   seja ferramenta de diagnóstico, é superfície nova de CLI, per `ADR-0127`.

## Fase B — Implementação (TDD directo para a transformação `Tj`/`TJ` em si; confirmar com o dono
se o resto do oráculo — relatório de paridade — entra neste passo ou fica para P981+)

1. Teste: dado o array de ajustes de um run já construído pelo modo verboso, se todos os deltas
   forem zero (ou dentro da tolerância confirmada contra o vanilla), o oráculo reescreve para `Tj`;
   caso contrário, mantém `TJ`.
2. Implementar, isolado do caminho normal — confirmar que `typst compile doc.typ` (sem a flag do
   oráculo) continua a produzir exactamente o mesmo PDF de antes deste passo (zero mudança na
   saída principal).
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Rodar o oráculo no documento de 30 secções e num documento de prosa — confirmar que a
   proporção `Tj`/`TJ` do PDF gerado pelo oráculo se aproxima da do vanilla.
2. Confirmar que a saída principal (sem a flag) está bit-a-bit igual à de antes deste passo —
   prova de que o oráculo é genuinamente aditivo, não contaminou o caminho normal.
3. `compare.py` na saída do oráculo — confirmar zero divergência de posição.
4. Benchmark do caminho normal (sem flag) — deve ficar em 1.00× exacto, já que o código não muda
   para quem não usa a flag.

## Resultado esperado

- Módulo oráculo separado do exportador de produção, activado só por flag própria.
- Transformação `Tj`/`TJ` implementada dentro do oráculo, não na saída principal.
- Saída principal (sem flag) comprovadamente inalterada por este passo.
- Proporção `Tj`/`TJ` da saída do oráculo próxima da do vanilla.
