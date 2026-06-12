# Tarefa P328 — Lote 13 (`Figure`, o último element-shaped) + carona (Arc-wrap no modelo) + dossiê do DEBT-58

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P328 (confirmar livre).
**Pré-condição**: Lote 12 (P327) fechado — lint 0, suíte verde (typst-core
2678). Se não, parar.
**Tipo**: Lote 13 da migração D — **instância do modelo**, a última — + uma
carona de manutenção de registro + o **dossiê do gatilho DEBT-58** (material
de leitura para a triagem; zero decisão de desenho neste passo).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (receita + Contabilidade +
passo mecânico C1…C1-quater), relatório P327 (achado Arc-wrap),
`introspect/{locatable,extract_payload}.rs` (Figure/M1), DEBT-58,
`figure_image.rs` (tocado no Lote 7).
**Commits**: "Passo 328 — carona de registro" (primeiro, tree limpo) e
"Passo 328 — lote 13".

---

## Carona de registro (commit próprio, antes do lote)

### C2 — O achado Arc-wrap do P327 mora no modelo, não no script efémero

O P327 adaptou o transformador de construções: a premissa
"construtor-cobre-todos-os-campos" é falsa para variantes densas; a emissão
correta é `Content::X(Arc::new(Elem{…}))` (e `..(**e).clone()` em
materialize, para preservar cosméticos). **Verificar se essa regra foi
gravada no modelo no P327**; se não (o transformador é efémero — script
`/tmp` por lote; adaptação não gravada se perde), gravá-la agora no passo
mecânico da Fase B: o transformador usa o construtor `Content::x(…)` **só**
quando ele cobre todos os campos da variante; caso contrário, Arc-wrap. Se
já estiver gravada, confirmar com a referência e encerrar a carona sem diff.
Diff esperado: 0 ou poucas linhas no modelo; zero código.

---

## Lote 13 — `Figure`, instanciando o modelo pela última vez

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 13`
- `LOTE`: **`Figure`(89)** — variante única, a mais larga, o último
  element-shaped. Largura acima da faixa-guia por desenho (sem com quem
  dividir).
- `NOTAS_FAMÍLIA`:
  - **Locatável (M1)** — pré-confirmado no P327, junto de Heading/Cite;
    reconfirmar no checkpoint contra `locatable.rs`/`extract_payload.rs`.
    Absorve `element_kind`/`to_payload` no trait (precedente Heading/Lote 6);
    consumo por `ElementPayload::Figure` inalterado.
  - **Campos** (~4: `body`/`caption`/`kind`/`numbering`) — confirmar a lista
    real no checkpoint; `map_*` esperado recursar em `body` **e** `caption`
    (precedente `Quote` L8, body+attribution); conferir assimetria
    `map_content`/`map_text` (precedente `Equation` L10, se houver).
  - **`Hash`**: conferir os tipos de `kind`/`numbering` — regra do modelo
    (derive se seguro; Debug-hash se floats/sem `Hash`; tipo dependente
    `Copy+Eq` sem floats recebe derive + L0 se pinar — precedentes Parity
    P320, CitationForm P324).
  - **`figure_image.rs`** (`infer_kind_from_body`, tocado no L7): site de
    interação direta — o kind inferido do body matcheia
    `Content::Image(_)`/`Content::Raw(_)`; a migração de `Figure` muda o
    lado de fora desse consumo. Incluir no plano de toque.
  - **Passo mecânico C1…C1-quater integral** (+ a regra Arc-wrap da carona):
    grep prévio → skip-list aos dois transformadores → aninhados à mão por
    classe → verificação pós-passada. Zero conversões indevidas é o
    critério.
  - Inspecionar `|`-combinados envolvendo `Figure` antes de estimar (regra
    P320).

Tudo o mais — Fase A, checkpoint humano, Fase B, validação (`cargo build`,
suíte com `RUST_MIN_STACK=33554432`, `crystalline-lint .` = 0), medições vs
preditor — **como está no modelo**. Lembretes: trait não muda em lote;
nenhuma asserção existente alterada; conserto oportunista proibido; quebra
de desenho → parar e voltar ao L0.

---

## Dossiê do gatilho DEBT-58 (entregável deste passo, junto ao relatório)

Ao fechar `Figure`, os element-shaped esgotam e **o gatilho do DEBT-58
dispara**. A triagem é **conversa de desenho, não lote** — este passo **não
decide nada** dela; produz o material com que ela começa. Gravar (no
relatório ou em ficheiro irmão `00_nucleo/…/dossie-triagem-debt-58.md`, à
escolha do executor, com referência cruzada):

1. **Inventário das variantes não migradas** (a conta final: 4 `Set*` +
   11 DEBT-58 + o que a triagem somar), cada uma com: nome × largura atual
   (grep refeito hoje, não o de P317) × forma no hub (campos, arms próprios
   ou partilhados) × classe provisória:
   - primitivos de AST/math: `MathSequence`/`MathText`/`MathIdent`;
   - cola estrutural: `Sequence`/`Empty`/`Block`;
   - cola de texto (C2/P319): `Space`, e a correlata `Text` (folha, 40);
   - wrappers a triar: `Styled`/`Boxed`/`Labelled`;
   - `Set*` (4) — **fora da triagem** (destino: F/99.E), listadas só para a
     conta fechar.
2. **As perguntas que a triagem deve responder** (sem respondê-las):
   primitivo permanece no hub ou vira `Elem`? wrappers são primitivos ou
   lote tardio? `Space`/`Text` seguem a guideline de cola? o que o estado
   misto do hub (arms remanescentes) custa para o F?
3. **O estado final do hub** após L13: o que resta em cada um dos 6 matches
   (lista de arms por match) — a fotografia que a triagem e o F vão
   consumir.

---

## Relatório (`typst-passo-328-relatorio.md` + resumo no chat)

- C2 confirmada (gravada no P327 ou agora; referência/diff).
- Composição/forma de `Figure` (campos × locatável × is_empty × map_* ×
  Hash); o toque em `figure_image.rs`.
- Sites tratados à mão + verificação pós-passada (zero conversões).
- Medições ADR-0104 do lote: `content.rs` antes/depois, parte atómica,
  suíte antes/depois, lint 0.
- **Balanço da fase de lotes** (piloto P316 + Lotes 2–13): trajetória
  completa do hub (5782 → final, por lote), soma da parte atómica, suíte
  (2521 → final), nº de variantes migradas (62) — a consolidação da métrica
  ADR-0104 num lugar só.
- **Contabilidade final da fase** (item obrigatório): migradas 61 → 62;
  element-shaped restantes **0**; conta: 62 + 4 `Set*` + 11 (DEBT-58) =
  **77** ✓. **Gatilho do DEBT-58 disparado** — registrar.
- O dossiê da triagem (onde mora; resumo de 1 parágrafo).
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

A **triagem do DEBT-58 em si** (conversa de desenho — passo seguinte, com o
dossiê na mão); F / `Set*` / 99.E (o diagnóstico consome a medição P318 com
o caveat C1/P319); qualquer migração além de `Figure`; otimizações sugeridas
por medição (medir ≠ mexer).
