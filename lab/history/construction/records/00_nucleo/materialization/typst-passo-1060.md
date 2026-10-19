# L0 — Investigação: mecanismo real de margin collapsing (base do P1060) + auditoria de nomes de arquivo duplicados

**Tipo**: investigação pura, sem alteração de código. Sem gate `ADR-0127` (não muda
comportamento).

**Motivo**: o L0 anterior do P1060 continha um mecanismo inventado (nomes de campo
plausíveis, não confirmados) e um caminho de módulo errado (`engine/layout/`, que não
existe — o nome correcto é `compiler/`, alinhado com o vanilla). Este passo substitui
suposição por leitura directa.

---

## Parte 1 — Localizar e transcrever o mecanismo real de margin collapsing

### Objectivo

Confirmar contra o código-fonte real (não os `.md`, que podem estar desactualizados
face aos passos mais recentes) os nomes exactos de campo, tipo e função que
implementam:

- Colapso `above`/`below`/`spacing` entre `Content::Block` consecutivos (P250).
- Avanço de `Content::Parbreak` (P1057).
- Consumer de `Sequence` com `peekable` (`block_chain_active`, mencionado em
  `entities/content.md` como P250, mas nunca visto no código real nesta conversa).

### Arquivos-alvo (caminho confirmado: `01_core/src/compiler/layout/`, não `engine/`)

- `01_core/src/compiler/layout/block.rs` — provável dono do braço `Content::Block`.
- `01_core/src/compiler/layout/sequence.rs` — provável dono do consumer de `Sequence`
  (citado em `compiler/layout.md`, secção P864, para outro mecanismo — `ItemGroup`/
  `parbreak_since_last_item` — mas é o candidato mais provável para o resto).
- `01_core/src/compiler/layout/mod.rs` — braço `Content::Parbreak`, se não estiver
  isolado num módulo próprio.
- `01_core/src/compiler/layout/cursor.rs` — **já lido nesta conversa**. Confirmado por
  grep no arquivo inteiro: **não contém** `Parbreak`, `block_chain_active`,
  `prev_block_below_pending`, nem `weakness`. Não voltar a pedir este arquivo.

### O que extrair de cada arquivo

1. Nome e tipo exactos de qualquer campo do `Layouter` relacionado com espaçamento
   vertical pendente entre blocos/parágrafos.
2. Corpo da função de colapso (se existir uma função dedicada) ou o trecho inline
   onde o `max(prev, curr)` (ou equivalente) acontece.
3. Pontos de chamada — onde esse campo é lido e onde é escrito.
4. Confirmar se `entities/content.md` (secção P250, já citada nesta conversa) bate
   com o código real, ou se ficou desactualizada.

### Saída esperada

Trecho de código real, citado, não resumido — os nomes exactos servem de base
directa ao L0 do P1060 (a reescrever depois desta investigação).

---

## Parte 2 — Auditoria de nomes de arquivo duplicados em `00_nucleo/prompts/`

### Objectivo

Nesta conversa já apareceram **3 arquivos chamados `layout.md`** em directórios
diferentes (`compiler/layout.md`, `stdlib/layout.md`, e um terceiro em `03_infra/` —
conteúdo pequeno, ponte para `FontBookMetrics`), cada um com conteúdo completamente
diferente. Isto já causou confusão real (uma vez nesta própria conversa). Preciso de
saber a extensão do problema, não só este caso isolado.

### Comando sugerido

```bash
find 00_nucleo/prompts -name "*.md" -exec basename {} \; | sort | uniq -c | sort -rn
```

Produz a contagem de cada nome-base, do mais repetido para o menos. Para cada nome
com contagem > 1, listar os caminhos completos:

```bash
find 00_nucleo/prompts -name "*.md" | awk -F/ '{print $NF, $0}' | sort | \
  awk '{print $1}' | uniq -d | while read n; do
    echo "== $n =="
    find 00_nucleo/prompts -name "$n"
  done
```

### Critério de decisão (proposto, não decidir sozinho — apresentar ao dono)

Duas opções para desambiguar nomes duplicados através de camadas diferentes (ex.:
`layout.md` em `compiler/`, `stdlib/`, `infra/`):

(a) Renomear para incluir a camada no nome do ficheiro (`compiler-layout.md`,
    `stdlib-layout.md`, `infra-layout.md`).
(b) Manter o nome, mas exigir caminho completo em toda referência textual entre
    prompts (nunca "ver `layout.md`" sozinho — sempre o caminho inteiro).

Este passo só produz o inventário e conta quantos arquivos cada opção afectaria — não
decide qual adoptar.

### Critério de conclusão

- Lista completa de nomes duplicados em `00_nucleo/prompts/`, com contagem e caminhos.
- Trecho de código real confirmando os nomes de campo do mecanismo de colapso
  (Parte 1).
- Nenhuma alteração de código.
