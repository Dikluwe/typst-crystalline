# Tarefa P316 — Lote piloto do modelo D (trait `Element`: Divider + Heading + MathStyled)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P316, conforme gravado na ADR-0105 ("P314 → P316+"). O
P315 foi absorvido pelo P314 (decisão do dono de unir os dois); registrar essa
nota no relatório para o leitor futuro não procurar um P315 que não existe.
**Tipo**: primeira implementação do candidato D (ADR-0105) — refatoração de
**comportamento idêntico**: nenhuma semântica muda; a lógica muda de morada.
**Fontes de decisão**: ADR-0104 (atomicidade), ADR-0105 (D agora, F destino),
`diagnostico-modelo-elemento-passo-313.md` (§1–§3), relatório P314.
**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` não
autorizadas.
**Commits**: permitidos. Sugestão: um commit para as pré-tarefas (P316-pre) e
um para o passo principal ("Passo 316"), para o lote piloto ficar isolável no
histórico — a lição da medição do P298, que não foi isolável por commit.

---

## Pré-tarefas (pequenas, antes da Fase A)

### Pre-1 — Zerar as V7 (decisão do dono: excluir, não mover)

`git rm` dos dois prompts-índice criados no P314 (`rules/stdlib.md` e
`rules/math/layout.md`). A trilha fica no histórico git + ADR-0104/0105 +
relatório P314 — suficiente por decisão do dono. Verificar:
`crystalline-lint .` → **0 V7**. Commitar.

### Pre-2 — Registrar o débito das specs ausentes

Entrada nova no `00_nucleo/DEBT.md` (próximo número livre, convenção do
arquivo): **"Specs L0 ausentes para ~70 funções stdlib"** — os `.rs` que o
P314 encontrou sem especificação dedicada (`layout.rs` 17 fns,
`structural.rs` 21, `shapes.rs` 6, `transforms.rs` 4, `gradients.rs` 3,
`assert.rs`, e as funções de `foundations.rs` listadas no relatório P314 §2).
Origem: deriva F4 retroativa exposta pela partição do P314. Critério de
fecho: cada `.rs` da lista com prompt L0 dedicado. Magnitude M (fatiável).
Sem corrigir nada agora — só o registro.

---

## FASE A — Redação dos L0 (a IA redige e PARA)

### A.1 — As decisões de desenho que os L0 devem tomar

O L0 do trait e dos elementos deve decidir e especificar (não herdar de
suposição):

1. **A assinatura do trait `Element`** — quais dos 6 matches do hub viram
   métodos. Esperado: `plain_text`, `map_content`, `map_text`, `get_field`
   como métodos; **`eq`/`hash` exigem desenho explícito** por causa do
   `Arc` (igualdade estrutural do payload, não do ponteiro — derive no
   struct do payload + dispatch no `Content`; especificar, com a interação
   com `content_hash` registrada).
2. **A morada do layout do elemento** — método do trait com dispatcher de 1
   linha no Layouter, ou arquivo por elemento sob `rules/layout/` com
   dispatcher. Decidir pela topologia entities/rules existente (L1 puro dos
   dois lados; escolher o que minimiza o toque futuro por elemento) e
   registrar a razão no L0.
3. **A absorção do locatável (caso Heading)** — como o trait expõe o que
   `ElementKind`/`ElementPayload` davam (ex.: `element_kind()` e extração de
   payload como métodos). No piloto, **só o braço do Heading** migra; os
   enums permanecem para as outras variantes — o estado misto é esperado e
   permitido pela ADR-0105 (migração incremental). O L0 declara o estado
   misto e o destino final (enums esvaziam lote a lote).
4. **A forma compatível com F** (trava da ADR-0105): o `impl Element` nasce
   de modo que acrescentar `fn descriptor()` no futuro seja natural —
   campos e defaults agrupados, não espalhados.

### A.2 — Os prompts L0 a redigir

Seguindo a atomicidade (ADR-0104): `entities/elements/_comum.md` (o trait, as
regras partilhadas, o estado misto) + um fino por elemento do lote —
`entities/elements/divider.md`, `entities/elements/heading.md`,
`entities/elements/math_styled.md`. Atualizar `entities/content.md` (as três
variantes mudam para `Nome(Arc<nome::Nome>)`; os braços delas nos 6 matches
viram dispatch). Partição content-preserving onde houver spec velha; spec
nova só para o que o desenho A.1 introduz.

**Se a implementação precisar editar `rules/layout.md`** (linhagem de 11
`.rs`, candidato medido no P314 §6): o imposto morde agora — **fatiá-lo
neste passo**, mesma receita do P314 (content-preserving, `_comum.md`,
`git rm` do velho — sem índice, conforme Pre-1). Se a opção 2 do A.1
dispensar editar esse prompt, não fatiar (o princípio é fatiar quando morde).

### A.3 — CHECKPOINT obrigatório

**Parar.** Apresentar ao humano: as decisões do A.1 com as razões, os L0
redigidos, e o plano de toque da Fase B (lista de arquivos). Prosseguir só
com a confirmação — Trava Arquitetural do `CLAUDE.md`.

---

## FASE B — Implementação (após confirmação humana)

Ordem do protocolo de nucleação:

1. **Testes primeiro**: para cada elemento do lote, testes unitários do
   `impl Element` (plain_text/map/eq/payload do Heading) no módulo novo —
   escrever, confirmar que falham (o módulo não existe ainda). A suíte
   existente inteira é o teste de comportamento idêntico.
2. **Migrar os 3, do piso ao teto**: `Divider` (singleton — prova o
   mecanismo), depois `MathStyled` (campos + math layout — prompts já finos
   do P314), depois `Heading` (locatável — prova a absorção; é onde o
   desenho pode quebrar; se quebrar, **parar e voltar ao L0**, não adaptar
   em silêncio).
3. **Linhagem**: headers `@prompt` dos módulos novos e dos tocados;
   `crystalline-lint --fix-hashes .`.
4. **Validação**:
   - `cargo build` verde; `cargo test --workspace` — contagem ≥ a do P314
     (2981; os testes novos do passo 1 somam) com **0 failed**; a ressalva
     da stack do P314 continua registrada se reaparecer.
   - `crystalline-lint .` — **0 V7** (Pre-1), **nenhuma violação nova**; as
     3 V9 pré-existentes de `03_infra` inalteradas (fora de escopo).
   - Comportamento idêntico: nenhum teste existente alterado para passar
     (alterar teste = mudança de comportamento = bug do passo).

5. **As medições que fecham o passo** (a métrica da ADR-0104):
   - **Encolhimento do hub**: linhas dos 6 matches de `content.rs`
     antes/depois (os braços dos 3 migrados viram dispatch de 1 linha) e o
     total do arquivo (baseline: 5782).
   - **Custo-por-elemento medido**: para cada um dos 3, quantos arquivos e
     linhas a migração tocou fora do módulo próprio do elemento — o número
     comparável ao baseline do 313 §2 ("~6 ficheiros hub/wiring + imposto").
   - **Projeção honesta**: com os números do piloto, estimar o custo dos
     ~74 restantes por lote — vai no relatório para o dono dimensionar os
     próximos lotes.

---

## Relatório (`typst-passo-316.md` + resumo no chat)

- Pre-1/Pre-2 confirmados (0 V7; número do DEBT novo).
- As decisões A.1 tomadas (assinatura do trait; morada do layout; forma da
  absorção; compatibilidade-F) em meia página.
- As medições do passo 5 (hub antes/depois; custo-por-elemento dos 3;
  projeção dos lotes).
- Se `rules/layout.md` foi fatiado: a tabela de mapeamento.
- Proposta de composição do **lote 2** (a decisão é humana; a proposta vem
  com critério — ex.: a família math restante, ou os singletons).
- `git log --oneline` dos commits do passo; `git status` limpo.

## Fora de escopo (confirmar intocado no relatório)

- Os outros ~74 elementos (lotes futuros, decisão por lote).
- F / PropMap / StyleChain (entra com o DEBT 99.E, ADR-0105).
- As 3 V9 pré-existentes.
- As specs ausentes do Pre-2 (registradas, não escritas).
- Fatiar `rules/eval.md` / `rules/parse.md` (não mordem neste passo).

## Restrições finais

- A Fase B não começa sem a confirmação humana do checkpoint A.3.
- Heading quebrar o desenho → parar e voltar ao L0; nunca adaptar em
  silêncio.
- Toda medição com o comando registrado.
