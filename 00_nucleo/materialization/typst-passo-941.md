# Passo 941 — glifos bitmap (CBDT/CBLC) como imagem XObject, não fonte embutida

**Precede este passo**: `typst-passo-940-relatorio.md` — confirmado que o `subsetter` falha em
CBDT (`UnknownKind`), forçando embed da fonte inteira (~10.8MB, PDF final ~10MB, ~135× maior que o
vanilla ~70-80KB). A correção de P940 (sem compressão Flate) resolveu o eixo de tempo (`render_ms`
300ms→19ms) sem piorar significativamente o tamanho (a compressão já era quase inútil sobre dados
CBDT, que são PNG internamente) — mas não resolve o tamanho absoluto, nem o defeito de os emojis
renderizarem monocromáticos (incorreto face ao vanilla, que renderiza a cores).

**Este passo resolve os três problemas de uma vez**, porque são a mesma causa: parar de tentar
embutir a fonte CBDT como fonte, e desenhar cada glifo bitmap usado como imagem.

**Escopo de arquitetura — não é ajuste pontual.** Mesmo cuidado de P896/906/909/918/935/937/938.

**Pré-condição de árvore**: `git status`. Confirmar estado P940 presente.

---

## Fase A — ler o mecanismo real do vanilla antes de desenhar (per `ADR-0123`, mesma disciplina
já aplicada a esta frente inteira desde P934/935/936)

1. Ler `krilla/src/text/glyph/bitmap.rs` (já citado em P940) linha a linha — confirmar
   exatamente: como o glifo bitmap é extraído da fonte (CBDT/CBLC, formato PNG por strike),
   como a posição é calculada (bearing, ppem — mencionado pelo dono, confirmar a fórmula exata,
   não presumir), como o dedup por glifo funciona (mesmo glifo emoji repetido no documento usa o
   mesmo XObject de imagem, não é reextraído/reembutido cada vez), e como isso se integra com o
   restante do texto (a fonte primária continua embutida normalmente para os caracteres não-CBDT
   do documento — só os glifos bitmap saem desse caminho).
2. Confirmar qual estrutura de tabela (`CBLC`, o "location" que mapeia glifo→strike/offset) precisa
   de ser lida para extrair os bytes PNG de cada glifo — via `ttf_parser` (confirmar se já expõe
   isso) ou leitura manual da tabela.
3. Confirmar se há mais de um "strike" (tamanho) disponível por glifo na fonte de teste
   (`NotoColorEmoji.ttf`) e como o vanilla escolhe qual usar para um dado tamanho de texto no
   documento — não presumir que há só um tamanho.

## Fase A.1 — desenhar o caminho de exportação (gate obrigatório)

1. Desenhar o fluxo: detectar fonte com CBDT/CBLC (ou, mais genericamente, qualquer formato que o
   `subsetter` não reconheça e que tenha dados de glifo bitmap) → para os glifos efetivamente
   usados no documento, extrair os bytes PNG do strike apropriado → emitir cada glifo único como
   um PDF Image XObject (dedup: mesmo glifo, mesmo XObject, reusado) → posicionar via `Tj`/matriz
   de transformação no lugar de desenhar via glifo de fonte.
2. Confirmar o que acontece com a métrica/posicionamento de texto ao redor — o glifo emoji ainda
   precisa de ocupar o espaço certo na linha (advance width), mesmo sendo desenhado como imagem,
   não como glifo de fonte tradicional.
3. Decidir se a fonte CBDT continua a ser embutida de alguma forma mínima (só para
   metadata/mapeamento, sem os dados bitmap) ou se deixa de ser embutida por completo, com o
   texto ao redor calculado só a partir das métricas já extraídas.
4. Editar L0s afetados (`03_infra/src/export/` — `builder.md`/`subset.md`/novo módulo se for o
   caso), sincronizar hashes, **parar para confirmação do dono antes da Fase B**.

## Fase B — Implementação (protocolo de dois agentes de P898 — novo caminho de exportação, risco
alto, mexe em como texto e imagem se combinam no PDF final)

1. Agente A escreve testes cobrindo: um documento com um único emoji (glifo único, um XObject);
   um documento com o mesmo emoji repetido várias vezes (confirmar dedup — um XObject só, várias
   referências `Do`); um documento misto (emoji + texto CJK/latino normal, confirmar que o
   caminho normal de fonte continua intacto para os outros caracteres).
2. Agente B implementa.
3. Revisão do orquestrador — confirmar visualmente (`mutool draw`, não só "compila") que os
   emojis aparecem **a cores** desta vez, corretamente posicionados em relação ao texto ao redor,
   comparado lado a lado com o vanilla real.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Medição completa (attestation, `L11`)

1. **Tamanho do PDF** (o eixo que motivou este passo) — `utf8-emoji.typ`/`05-utf8.typ`, antes
   (P940, ~10MB) e depois deste passo, comparado ao vanilla real (~70-80KB). Meta: mesma ordem de
   grandeza do vanilla, não só "menor que antes".
2. **`render_ms`** isolado — confirmar se cai ainda mais que os ~19ms de P940 (a cópia de 10.8MB
   deixa de existir, per estimativa do dono).
3. 7 cenários canônicos, `depois/antes`, zero regressão no caso comum.
4. 5 casos UTF-8, `depois/antes` e `cristalino/vanilla-real` — confirmar quanto da distância total
   ainda restante (per P938/939, ~4.4×-6.1×) foi fechada.
5. Confirmação visual lado a lado com o vanilla — cor correta, posição correta.

## Resultado esperado

- Glifos bitmap exportados como imagem XObject, com dedup por glifo único.
- Tamanho do PDF de emoji/UTF-8 na mesma ordem de grandeza do vanilla (não mais ~135× maior).
- Emojis renderizando a cores (correção de um defeito pré-existente, não só otimização).
- `render_ms` residual reduzido ainda mais.
- Confirmação visual lado a lado com o vanilla.
- Nota sobre quanto da distância total (tempo) ainda resta depois desta correção, e se aponta de
  volta para a extração de coverage (P938/939) como próximo alvo, ou se fica dentro da banda
  aceitável.
