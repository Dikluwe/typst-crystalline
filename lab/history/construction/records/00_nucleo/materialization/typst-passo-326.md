# Tarefa P326 — Lote 11 (largura: Footnote, Shape) + carona (C1-ter: skip-list por linha-do-grep)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P326 (confirmar livre).
**Pré-condição**: Lote 10 (P325) fechado — lint 0, suíte verde (typst-core
2653). Se não, parar.
**Tipo**: Lote 11 da migração D — **instância do modelo** — + uma carona de
manutenção de registro (zero código de produto fora do lote).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (receita + Contabilidade +
passo mecânico C1/C1-bis), relatório P325 (a 4ª reincidência apanhada e o
candidato C1-ter).
**Commits**: "Passo 326 — carona de registro (C1-ter)" (primeiro, tree limpo)
e "Passo 326 — lote 11".

---

## Carona de registro (commit próprio, antes do lote)

### C1-ter — Da deteção à prevenção: skip-list por linha-do-grep

Estado após C1-bis (P325): o erro do transformador é **apanhado** (verificação
pós-passada interseta sites tocados × lista do grep, antes de compilar), mas
não **impossível** — a heurística interna do transformador continua o elo
fraco (4 reincidências: P321/P323/P324/P325).

**Correção (gravada no modelo, Fase B, passo mecânico):** a exclusão deixa de
depender da heurística — a lista do grep do passo 1 vira **skip-list
explícita** do transformador: qualquer site cuja `ficheiro:linha` está na
lista é **pulado pela passada automática** (tratado à mão, como já manda a
regra). A verificação pós-passada da C1-bis **permanece**, rebaixada a rede
de segurança: a partir deste passo, interseção não-vazia significa que a
skip-list falhou — reportar como falha da C1-ter, não como rodapé.

Se o transformador for script no repo, a mudança da skip-list é mecânica e
entra neste commit (é ferramenta de migração, não código de produto;
registrar o caminho do script no texto da carona). Se for procedimento
manual/efémero, gravar o procedimento da skip-list no modelo. Diff esperado:
poucas linhas no modelo (+ o script, se existir).

---

## Lote 11 — largura crescente, instanciando o modelo

Sequência em vigor (decisão do dono, P325): L11 (este) → L12 bloco grid/table
cell (~193, penúltimo) → L13 `Figure`(89), que esgota os element-shaped e
dispara o gatilho do DEBT-58.

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 11`
- `LOTE` (a confirmar no checkpoint contra a Contabilidade), ordem por
  largura crescente: **`Footnote`(46) · `Shape`(57)** = **~103 sites** —
  dentro da faixa-guia (~110–150, por baixo).
- `NOTAS_FAMÍLIA`:
  - **Locatabilidade**: `Footnote` é candidata natural a locatável (família
    de introspecção de `Cite`/`Bibliography`, Lotes 9–10) — confirmar contra
    `introspect/locatable.rs`/`extract_payload.rs`; se for, precedente
    Heading/Lote 6 (`element_kind`/`to_payload` no trait; consumo por
    `ElementPayload` inalterado). `Shape` esperada não-locatável. Confirmar
    ambas uma a uma no checkpoint.
  - **Hash**: `Shape` provável manual via Debug (geometria com
    `Length`/f64); `Footnote` conferir os campos. Regra do modelo: Debug-hash
    só com floats/tipos sem `Hash` seguro; `Copy+Eq` sem floats recebe
    derive (precedentes Parity P320, CitationForm P324) — se um tipo
    dependente precisar do derive, é dependência do lote; atualizar o L0
    dele se o L0 pinar derives.
  - Classificar no checkpoint: leaf vs contentor vs unit; `map_content` vs
    `map_text` (assimetria tem precedente: `Equation` L10 — materializar
    ambos no `Elem` por contrato, hub preserva o `|`-terminal sem split);
    `is_empty` delega só se o hub atual delega (content-preserving).
  - **Passo mecânico C1 + C1-bis + C1-ter**: grep prévio registrado →
    skip-list explícita na passada → verificação pós-passada como rede de
    segurança. Sites tratados à mão listados no relatório; **zero
    conversões indevidas é o critério** (interseção não-vazia = falha da
    C1-ter, reportar como tal).
  - Inspecionar arms `|`-combinados envolvendo o LOTE antes de estimar
    (regra do preditor, P320): com binding → split; sem binding → custo =
    largura.

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente,
validação (`cargo build`, suíte com `RUST_MIN_STACK=33554432`,
`crystalline-lint .` = 0), medições vs preditor, contabilidade atualizada
(item obrigatório), proposta do Lote 12 — **como está no modelo**. Lembretes
que o modelo já carrega: prompt grosso que morder → fatiar primeiro; trait
não muda em lote; nenhuma asserção existente alterada; conserto oportunista
proibido.

---

## Relatório (`typst-passo-326-relatorio.md` + resumo no chat)

- C1-ter confirmada: o mecanismo da skip-list (script ou procedimento), o
  diff no modelo, e o resultado da estreia (interseção pós-passada vazia?).
- Composição confirmada (variante × largura × locatável × forma) e soma vs
  faixa.
- Resultado da locatabilidade de `Footnote` (o achado provável do lote).
- Sites de padrão tratados à mão (lista do grep) + resultado da verificação
  pós-passada.
- Medições ADR-0104: `content.rs` antes/depois (trajetória desde 5782),
  parte atómica, suíte antes/depois (+N só dos unitários novos), lint 0.
- **Contabilidade atualizada** (item obrigatório): migradas 55 → 57;
  restantes ~7 → ~5 (o bloco grid/table cell ×4 + `Figure`); conta de fecho
  contra 77.
- Proposta do Lote 12 (decisão humana): o **bloco grid/table cell**
  (`TableCell`32 · `GridCell`47 · `Table`41 · `Grid`73, ~193 sites) — com o
  que a Fase A do L12 precisará de atenção (nº de campos de
  `GridCell`/`TableCell`, possíveis `|`-combinados com binding entre as 4,
  locatabilidade), para o prompt do P327 dimensionar o checkpoint.
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lote 12 (bloco grid/table cell) e Lote 13 (`Figure`); DEBT-58 (gatilho
dispara quando `Figure` fechar — ainda não); F / `Set*` / 99.E; otimizações
sugeridas por medição (medir ≠ mexer).
