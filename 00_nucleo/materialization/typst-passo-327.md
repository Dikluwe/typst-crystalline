# Tarefa P327 — Lote 12 (bloco grid/table cell) + carona (C1-quater: skip-list em ambos os transformadores)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P327 (confirmar livre).
**Pré-condição**: Lote 11 (P326) fechado — lint 0, suíte verde (typst-core
2662). Se não, parar.
**Tipo**: Lote 12 da migração D — **instância do modelo**, o maior lote do
roteiro — + uma carona de manutenção de registro (zero código de produto
fora do lote).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (receita + Contabilidade +
passo mecânico C1/C1-bis/C1-ter), relatório P326 (achado do transformador de
padrões; reconhecimento prévio do bloco).
**Commits**: "Passo 327 — carona de registro (C1-quater)" (primeiro, tree
limpo) e "Passo 327 — lote 12".

---

## Carona de registro (commit próprio, antes do lote)

### C1-quater — A skip-list cobre ambos os transformadores, por regra mecânica

O P326 deixou uma **nota** no modelo ("a skip-list deve cobrir ambos os
transformadores") após o transformador de **padrões** quebrar patterns
aninhados (`Shape { kind: Path(items), .. }`, binding perdido). O histórico
do roteiro mostra que nota não previne (a nota do P321 não impediu
P323/P324/P325; o passo mecânico impediu). Antes do maior lote, converter a
nota em regra mecânica no modelo (Fase B, passo mecânico):

1. A skip-list por `ficheiro:linha` é entregue a **ambos** os transformadores
   (construções e padrões) — nenhum decide por heurística.
2. **Padrões aninhados são excluídos por classe**: qualquer padrão com
   destruturação interna além do `Content::X` de topo (enum/struct interno,
   ex.: `{ kind: ShapeKind::Path(items), .. }`) é tratado **à mão sempre**
   (receita validada no P326: `let … else`), nunca pela passada automática.
3. A verificação pós-passada (C1-bis) permanece a rede de segurança para os
   dois; interseção não-vazia = falha da regra, reportar como tal.

Diff esperado: poucas linhas no modelo (substituir a nota pela regra), zero
código.

---

## Lote 12 — bloco grid/table cell, instanciando o modelo

Sequência em vigor (decisão do dono): L12 (este, penúltimo) → L13
`Figure`(89), que esgota os element-shaped e **dispara o gatilho do DEBT-58**
(triagem — conversa de desenho, fora dos lotes).

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 12`
- `LOTE` (a confirmar no checkpoint contra a Contabilidade), ordem por
  largura crescente: **`TableCell`(32) · `Table`(41) · `GridCell`(47) ·
  `Grid`(73)** = **~193 sites** — o maior lote do roteiro; fora da faixa-guia
  por desenho (bloco coeso, decisão antiga do dono).
- **Válvula no checkpoint (decisão do dono na hora):** inspecionar os
  `|`-combinados **com binding** entre as 4 (família coesa — partilha de arms
  em `map_content`/`map_text`/`eq`/materialize/walk é provável; regra do
  preditor P320: binding → split, custo extra). Se o custo efetivo
  (sites + splits) inflar muito além dos ~193, o checkpoint **propõe a
  divisão 2+2** (`TableCell`+`Table` ~73 · `GridCell`+`Grid` ~120) com os
  números na mão — e PARA para a decisão, como manda a Trava.
- `NOTAS_FAMÍLIA`:
  - **Campos densos** (reconhecimento do P326): `GridCell`/`TableCell` ~10
    campos (body/x/y/colspan/rowspan/stroke/fill/align/inset/breakable);
    `Grid` ~10 (columns/rows/cells/gutter/align/inset/header/footer/stroke/
    fill); `Table` ~5. **Hash provável manual** via Debug (`Length`/`Color`/
    `Stroke` — regra do modelo); confirmar campo a campo no checkpoint.
  - **Contentores recursivos em Vec**: `Grid`/`Table` recursam em
    `cells`/`children` (Vec de Content) e nos `header`/`footer` (que já são
    `Arc<…Elem>` desde o Lote 5 — conferir a interação); `GridCell`/
    `TableCell` recursam no `body`. `is_empty`/`plain_text` delegam só se o
    hub atual delega (content-preserving).
  - **Locatabilidade**: todas não-locatáveis (pré-confirmado no P326 contra
    `locatable.rs`); reconfirmar no checkpoint, sem absorção esperada.
  - **Validação intermediária (regra deste lote, pelo tamanho)**: após cada
    variante migrada, `cargo build` + suíte do typst-core verdes antes de
    passar à próxima — não acumular as 4 para validar só no fim.
  - **Passo mecânico C1…C1-quater integral**: grep prévio registrado →
    skip-list a **ambos** os transformadores → padrões aninhados à mão por
    classe → verificação pós-passada. Sites tratados à mão listados no
    relatório; **zero conversões indevidas é o critério**.

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente,
validação final (`cargo build`, suíte com `RUST_MIN_STACK=33554432`,
`crystalline-lint .` = 0), medições vs preditor, contabilidade atualizada
(item obrigatório), proposta do Lote 13 — **como está no modelo**. Lembretes:
prompt grosso que morder → fatiar primeiro; trait não muda em lote; nenhuma
asserção existente alterada; conserto oportunista proibido; quebra de
desenho → parar e voltar ao L0.

---

## Relatório (`typst-passo-327-relatorio.md` + resumo no chat)

- C1-quater confirmada (a regra substituiu a nota; diff) e o resultado da
  estreia nos **dois** transformadores (interseções vazias? padrões
  aninhados todos à mão?).
- Composição confirmada; o resultado da inspeção dos `|`-combinados com
  binding (a válvula: rodou inteiro ou dividiu 2+2 — e com que números).
- Forma das 4 (tabela: campos × locatável × is_empty × map_* × Hash);
  interação com `GridHeader`/`TableHeader`/`Footer` do Lote 5.
- Validação intermediária: o estado (build+suíte) após cada variante.
- Sites de padrão tratados à mão (lista do grep, incl. aninhados por
  classe) + verificação pós-passada dos dois transformadores.
- Medições ADR-0104: `content.rs` antes/depois (trajetória desde 5782; lote
  denso — encolhimento esperado grande, os arms das 4 são verbosos), parte
  atómica, suíte antes/depois (+N só dos unitários novos), lint 0.
- **Contabilidade atualizada** (item obrigatório): migradas 57 → 61;
  restantes ~5 → ~1 (`Figure`); conta de fecho contra 77.
- Proposta do Lote 13 (decisão humana): `Figure`(89) — com o aviso de que o
  fecho dela **dispara o gatilho do DEBT-58** (a triagem é conversa de
  desenho, não lote; o prompt do P328 fecha Figure e o do passo seguinte
  abre a triagem).
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lote 13 (`Figure`); DEBT-58 (gatilho dispara quando `Figure` fechar — a
triagem é o passo seguinte ao L13, não este); F / `Set*` / 99.E; otimizações
sugeridas por medição (medir ≠ mexer).
