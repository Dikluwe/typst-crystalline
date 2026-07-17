# Prompt L0 — Meta: Varredura Mecânica P425
Hash do Código: *n/a* (prompt de processo, não gera código direto)

**Camada**: Meta / orquestração de agentes
**Ficheiro alvo**: n/a
**Criado em**: 2026-06-23
**ADRs relevantes**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B)

---

## Contexto

P425 é um passo de varredura mecânica executado durante ausência do dono. O objetivo é fazer progresso incremental em gaps que são puramente mecânicos — isto é, não exigem decisão arquitetural — delegando tarefas a agentes autônomos.

A distinção mecânico vs linguagem é a chave (ADR-0107):
- **Mecânico**: adicionar arm faltante em `match`, remover import não usado, mover lógica gorda para free function, adicionar test unitário mínimo, sincronizar hash, adicionar braço de transporte de dado já existente.
- **Linguagem/arquitetural**: adicionar novo variant a `Content`/`Value`/`Selector`, escolher crate externa, redefinir API pública, alterar contrato de paridade, decidir semântica de erro, introduzir vtable/`dyn`/PropMap.

Se um gap revelar sub-gaps linguísticos, o agente para e documenta; não força a solução.

---

## Agentes definidos

| Agente | Tarefa | Critério de aceitação | Escopo máximo |
|--------|--------|----------------------|---------------|
| **A5** — Hash sync | Sincronizar `@prompt-hash` driftados | `crystalline-lint --fix-hashes` → 0 drift | Só hashes |
| **A6** — Warning hunter | Corrigir warnings mecânicos de `cargo check -p typst-core` | `cargo check -p typst-core` → 0 warnings mecânicos | unused imports/variables/mut, unreachable patterns; dead_code só se claramente residual |
| **A7** — PDF link consumer | Adicionar braços `FrameItem::Link` em `typst-infra` para compilar; emitir annotation URI se viável mecanicamente | `cargo check -p typst-infra` passa; `FrameItem::Link` transportado/recursado nos walkers | Não decidir se adiciona `pos/size` a `FrameItem::Link`; se precisar, documentar como bloqueador |
| **A3** — Atomizador mecânico | Extrair arms monolíticos (>20 LOC) de `layout_content` para free functions em `engine/layout/<elem>.rs` (forma B) | Lint zero; `cargo test -p typst-core --lib` passa; match magro delega | Não alterar semântica; não introduzir vtable |
| **A4** — Test gap filler | Adicionar tests unitários mínimos para variants sem cobertura direta | +N tests verdes; 0 regressões | Priorizar variants visuais/estruturais; metadata/state deixar como scope-out se baixo valor |

**Ordem recomendada**: A5 → A6 → A7 → A3 → A4.

Agentes A1 (layout stubs) e A2 (repr gaps) estão **vazios** na sonda P425: todos os variants de `Content` têm arm em `layout_content` e todos os variants de `Value`/`Content` têm arm em `repr.rs`.

---

## Protocolo de execução

Para cada agente:
1. Ler a lista de gaps da sonda A.0.
2. Se vazia → pular.
3. Para cada item:
   - Medir: abrir arquivo, verificar contexto.
   - Decidir: é mecânico?
   - Se sim → implementar (código + test mínimo).
   - Se não → documentar no scope-out/relatório.
4. Validar: `cargo check -p typst-core` / `cargo test -p typst-core --lib` / `crystalline-lint .`
5. Se falhar → reverter (`git stash` ou `git checkout -- .`) e documentar bloqueador.
6. Commit: `git add -A && git commit -m "P425-A<N>: <agente> — <resumo>"`.

---

## Critério de parada do agente

- **Tempo**: 4 horas máximo. Se um agente exceder 45 min, parar e documentar.
- **Qualidade**: cada commit deve manter `cargo test -p typst-core --lib` passando (exceto regressões documentadas).
- **Escopo**: ao encontrar sub-gap linguístico, parar e documentar.

---

## Scope-out explícito (não tocar)

- Decisões arquiteturais: novos variants, crates, APIs públicas, contratos de paridade.
- Refactor estrutural: enum → vtable, match → dyn, Box → Arc.
- Features linguísticas novas: `text.lang` rustybuzz, bibliography CSL-native, cite supplement.
- Otimizações de performance.
- Documentação de usuário.
- Se `FrameItem::Link` exigir adicionar `pos/size` ao tipo para emitir annotation, isso é decisão arquitetural — documentar como bloqueador em vez de forçar.

---

## Resultado esperado

- L0: este ficheiro.
- Commits P425-A5, P425-A6, P425-A7, P425-A3, P425-A4 (conforme aplicável).
- Relatório em `/tmp/p425-relatorio.md` (ou similar) e/ou no fim de `00_nucleo/materialization/typst-passo-425.md`.
- `cargo check -p typst-core` e `cargo check -p typst-infra` passando.
- `cargo test -p typst-core --lib` sem regressões (exceto bloqueadores documentados).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-23 | P425: criação do L0 de varredura mecânica | `meta/p425-varredura-mecanica.md` |
