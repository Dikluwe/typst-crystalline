# Tarefa P325 — Lote 10 (largura: Pad, Bibliography, Equation) + carona (C1-bis: o elo que falhou)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P325 (confirmar livre).
**Pré-condição**: Lote 9 (P324) fechado — lint 0, suíte verde (typst-core
2637). Se não, parar.
**Tipo**: Lote 10 da migração D — **instância do modelo** — + uma carona de
manutenção de registro (zero código fora do lote).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (receita + Contabilidade +
passo mecânico C1 do P324), relatório P324 (a reincidência E0164 com o passo
já em vigor).
**Commits**: "Passo 325 — carona de registro (C1-bis)" (primeiro, tree limpo)
e "Passo 325 — lote 10".

---

## Carona de registro (commit próprio, antes do lote)

### C1-bis — Diagnosticar o elo que falhou e fechar a receita

No P324, **com o passo mecânico C1 já em vigor**, o transformador ainda
converteu 3 padrões aninhados (`if let Value::Content(Content::{Stack,Cite}
{…})`) em construtores (E0164). Antes de reforçar a receita, **diagnosticar
qual elo falhou** (consultar o histórico do P324 / reproduzir o grep):

- **(i) O grep não capturou** os padrões aninhados (o `Content::Nome {`
  dentro de `Value::Content(…)` escapou do padrão de busca)? → corrigir a
  receita: o grep busca `Content::Nome {` e `Content::Nome(` em **qualquer
  posição da linha** (sem âncora de início de padrão), em `if let`, braços de
  `match` e `matches!`.
- **(ii) O grep listou e a exclusão não foi aplicada**? → corrigir a receita:
  a exclusão vira **verificável** — após a passada do transformador, conferir
  a lista de sites tocados contra a lista do grep; interseção não-vazia =
  parar e reverter antes de compilar.

Gravar no modelo a correção do elo que de facto falhou (e só ele — não
inflar a receita com o conserto do elo que funcionou). Registrar no texto:
esta é a 3ª reincidência (P321, P323, P324); o objetivo da regra é tornar o
erro **impossível**, não documentado. Diff esperado: poucas linhas no
modelo, zero código.

---

## Lote 10 — largura crescente, instanciando o modelo

Decisão do dono sobre a proposta do P324: **divisão da opção (A)** — as 5
avulsas (~227 sites) excederiam a faixa validada em ~45%; o preditor
(custo ∝ largura) manda dividir. Sequência resultante: L10 (este) → L11
(`Footnote`/`Shape`, ~103) → L12 bloco grid/table cell (~193, penúltimo) →
L13 `Figure`(89), que esgota os element-shaped e dispara o gatilho do
DEBT-58.

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 10`
- `LOTE` (a confirmar no checkpoint contra a Contabilidade), ordem por
  largura crescente: **`Pad`(39) · `Bibliography`(40) · `Equation`(45)** =
  **~124 sites** — dentro da faixa-guia (~110–150).
- `NOTAS_FAMÍLIA`:
  - **Locatabilidade — atenção deste lote**: `Bibliography` e `Equation` são
    candidatas naturais a locatáveis (como `Cite`/M1 no Lote 9). Confirmar
    cada uma contra `introspect/locatable.rs`/`extract_payload.rs` no
    checkpoint; locatável segue o precedente Heading/Lote 6
    (`element_kind`/`to_payload` no trait; consumo por `ElementPayload`
    inalterado). `Pad` esperada não-locatável.
  - Classificar no checkpoint: leaf vs contentor (recurse no body) vs unit;
    forma de `plain_text`/`is_empty` (delegação só se o hub atual delega —
    content-preserving, precedentes `Align` L7 / `Transform`/`Place` L9);
    `Hash` derive vs manual via Debug (regra do modelo: Debug-hash só com
    `Length`/f64/tipos sem `Hash` seguro; `Copy+Eq` sem floats recebe
    derive — precedentes Parity P320, CitationForm P324; se um tipo
    dependente precisar do derive, é dependência do lote: atualizar o L0
    dele se o L0 pinar derives).
  - **Passo mecânico C1 (com a correção C1-bis)**: grep de padrões pelas 3
    variantes antes do transformador; lista registrada; verificação
    pós-passada (se a C1-bis a instituir). Sites de padrão tratados à mão
    listados no relatório.
  - Inspecionar arms `|`-combinados envolvendo o LOTE antes de estimar
    (regra do preditor, P320): com binding → split; sem binding → custo =
    largura.

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente,
validação (`cargo build`, suíte com `RUST_MIN_STACK=33554432`,
`crystalline-lint .` = 0), medições vs preditor, contabilidade atualizada
(item obrigatório), proposta do Lote 11 — **como está no modelo**. Lembretes
que o modelo já carrega: prompt grosso que morder → fatiar primeiro; trait
não muda em lote; nenhuma asserção existente alterada; conserto oportunista
proibido.

---

## Relatório (`typst-passo-325-relatorio.md` + resumo no chat)

- C1-bis confirmada: **qual elo falhou** (i ou ii), a correção gravada no
  modelo, o diff.
- Composição confirmada do Lote 10 (variante × largura × locatável × forma)
  e soma de sites vs faixa.
- Resultado da locatabilidade de `Bibliography`/`Equation` (o achado do
  lote).
- Sites de padrão tratados à mão (lista do grep) + resultado da verificação
  pós-passada — **zero conversões indevidas é o critério** (4ª reincidência
  = a receita ainda está errada; reportar como falha da C1-bis, não como
  rodapé).
- Medições ADR-0104: `content.rs` antes/depois (trajetória desde 5782),
  parte atómica, suíte antes/depois (+N só dos unitários novos), lint 0.
- **Contabilidade atualizada** (item obrigatório): migradas 52 → 55;
  restantes ~10 → ~7; conta de fecho contra 77.
- Proposta do Lote 11 (decisão humana): `Footnote`(46) · `Shape`(57),
  ~103 sites — confirmar contra o mapa.
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lotes 11+ (`Footnote`/`Shape`), bloco grid/table cell (L12, penúltimo),
`Figure` (L13, fecha os element-shaped), DEBT-58 (gatilho dispara no fim dos
element-shaped — ainda não), F / `Set*` / 99.E, otimizações sugeridas por
medição (medir ≠ mexer).
