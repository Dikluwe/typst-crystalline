# Tarefa P324 — Lote 9 (largura: 5 a partir de SmartQuote) + carona (transformador × padrões)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P324 (confirmar livre).
**Pré-condição**: Lote 8 (P323) fechado — lint 0, suíte verde (typst-core
2615). Se não, parar.
**Tipo**: Lote 9 da migração D — **instância do modelo** — + uma carona de
manutenção de registro (zero código fora do lote).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (a receita + seção
Contabilidade de variantes), relatórios P321 e P323 (as duas ocorrências do
erro do transformador), tabela de largura (P317, atualizada pela
Contabilidade).
**Commits**: "Passo 324 — carona de registro" (primeiro, tree limpo) e
"Passo 324 — lote 9".

---

## Carona de registro (commit próprio, antes do lote)

### C1 — Transformador de construções × posições de padrão (2ª ocorrência)

O transformador de construções errou em posição de padrão **duas vezes**:

- **P321**: converteu padrões `{ field: bind }` em chamadas de construtor
  (inválido em posição de padrão) e sobre-removeu `Box::new` aninhado.
- **P323**: converteu um `matches!` (posição de padrão) no caso `Ref` —
  corrigido à mão para `Content::Ref(e) if e.target.0 == …`.

A nota gravada no P321 ("padrões e `Box` aninhados são manuais") não impediu
a reincidência. Reforçar a nota no modelo (seção da Fase B, junto ao uso do
transformador) convertendo a lição em **passo mecânico obrigatório**:

> Antes de rodar o transformador sobre os sites do LOTE:
> 1. `grep -rn 'matches!' --include='*.rs'` filtrado pelas variantes do
>    LOTE, e grep equivalente para destruturações em `if let`/`match` com
>    struct-literal (`Content::Nome {`) — registrar a lista.
> 2. **Excluir esses sites da passada automática**; tratá-los à mão.
> 3. O transformador só serve **construções** com `Box` externo único;
>    qualquer posição de padrão é manual por regra, não por lembrete.

Diff esperado: poucas linhas no modelo. Nenhum código.

---

## Lote 9 — largura crescente, instanciando o modelo

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 9`
- `LOTE` (regra de composição, a confirmar no checkpoint da Fase A contra a
  seção Contabilidade): **as 5 element-shaped restantes de menor largura, em
  ordem crescente, começando por `SmartQuote`(28)**. Exclusões fixas:
  - o **bloco grid/table cell** (`TableCell`/`GridCell`/`Table`/`Grid`,
    ~193 sites) — lote próprio, penúltimo (sequência do dono);
  - **`Figure`(89)** — a mais larga fora do bloco; fica para o Lote 10
    (sozinha ou com as sobras, decisão no relatório deste lote).
  No checkpoint, emitir a lista nominal com larguras e a soma de sites
  (esperado: na faixa de ~110–150, comparável aos Lotes 5–8).
- `NOTAS_FAMÍLIA`:
  - Lote misto por largura (precedente Lotes 7–8) — sem família temática;
    classificar cada variante no checkpoint: leaf/terminal vs contentor
    (recurse no body) vs unit, e a forma de `plain_text`/`is_empty`/`Hash`
    (derive vs manual via Debug — regra do modelo: Debug-hash só com
    `Length`/`f64`/tipos sem `Hash`; `Copy+Eq` sem floats recebe derive,
    precedente Parity P320).
  - Verificar locatabilidade de cada uma contra `introspect/locatable.rs`;
    locatável segue o precedente Heading/Lote 6 (`element_kind`/`to_payload`
    no trait; consumo por `ElementPayload` inalterado).
  - Aplicar o passo mecânico da C1 (grep de padrões antes do transformador)
    e registrar no relatório a lista de sites tratados à mão.
  - Inspecionar os arms `|`-combinados envolvendo o LOTE antes de estimar
    (regra do preditor, P320): com binding → custo extra de separação; sem
    binding → custo = largura.

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente,
validação (`cargo build`, suíte com `RUST_MIN_STACK=33554432`,
`crystalline-lint .` = 0), medições vs preditor, contabilidade atualizada
(item obrigatório), proposta do Lote 10 — **como está no modelo**. Lembretes
que o modelo já carrega e aqui só ecoam: prompt grosso que morder → fatiar
primeiro; trait não muda em lote; nenhuma asserção existente alterada;
conserto oportunista proibido.

---

## Relatório (`typst-passo-324-relatorio.md` + resumo no chat)

- C1 confirmada (o diff no modelo).
- Composição confirmada do Lote 9 (lista nominal × largura × locatável ×
  família/forma) e a soma de sites vs faixa esperada.
- Medições da métrica ADR-0104: `content.rs` antes/depois (trajetória desde
  5782/P313), parte atómica, suíte antes/depois (+N só dos unitários novos),
  lint 0.
- Sites de padrão tratados à mão (a lista do grep da C1) — valida o passo
  mecânico novo.
- **Contabilidade atualizada** (item obrigatório): migradas 47 → 52;
  restantes ~15 → ~10; conta de fecho contra 77.
- Proposta do Lote 10 (decisão humana): as sobras + `Figure`, ou `Figure`
  separada — com as larguras do mapa.
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default estoura em `recursao_infinita_*` — não é
  regressão (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lotes 10+ e o bloco grid/table cell (lote próprio); DEBT-58 (gatilho: fim dos
element-shaped — ainda não disparou); F / `Set*` / 99.E; otimizações
sugeridas por medição (medir ≠ mexer).
