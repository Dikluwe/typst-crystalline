# Tarefa P319 — Lote 4 (decorações de texto) + caronas (caveat M3, Space → DEBT-58, mapa de lotes)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P319 (confirmar livre).
**Pré-condição**: Lote 3 (P318) fechado — lint 0, suíte verde. Se não, parar.
**Tipo**: Lote 4 da migração D — **instância do modelo** — + três caronas de
manutenção de registro (zero código fora do lote).
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md`, tabela de largura (P317),
relatório P318, `medicao-pre-f-passo-318.md`, DEBT-58.
**Commits**: "Passo 319 — caronas de registro" (primeiro, tree limpo) e
"Passo 319 — lote 4".

---

## Caronas de registro (commit próprio, antes do lote)

### C1 — Caveat de resolução no M3

Em `00_nucleo/diagnosticos/medicao-pre-f-passo-318.md`, seção M3, adicionar o
caveat: as 5 execuções deram exatamente 0.07 s — granularidade de 0.01 s do
`/usr/bin/time` ⇒ quantização ~15%; este baseline detecta só regressões
grosseiras. **A medição do "depois" do F deve refazer o "antes" e o "depois"
no par de commits, com `hyperfine` (média±σ) ou corpus ~10×, mesmo corpus e
comando.** (O hash de linhagem não se aplica: diagnósticos não são L0.)

### C2 — `Space` entra na triagem do DEBT-58

Adicionar `Space` (largura 13) ao conjunto de triagem do DEBT-58, com a
razão: é **cola de texto** — parente de `Empty` (116, já no conjunto), não de
`Divider` (comando unit). A triagem decide; o registro impede que um lote
futuro a arraste por engano.

### C3 — O mapa de lotes restantes (o roteiro mora no repo)

Adicionar ao final de `00_nucleo/modelo-lote-migracao-d.md` (ou arquivo
irmão, se o modelo ficar longo) a seção **"Contabilidade de variantes"**:

- migradas até P319 (lista: 3 do P316 + 11 do Lote 2 + 5 do Lote 3 + 3 do
  Lote 4);
- `Set*` (4) → decisão F/99.E, fora de lote;
- primitivos DEBT-58 (6 + Space em triagem) e wrappers a triar
  (`Styled`/`Boxed`/`Labelled`);
- **element-shaped restantes** (lista derivada da tabela P317, por largura) —
  os lotes 5+ saem daqui;
- a estimativa: ~5–8 lotes restantes no ritmo validado (5–8 variantes/lote).

Atualizar esta seção passa a ser item do relatório de **todo** lote (adicionar
essa linha à seção Relatório do modelo). O roteiro deixa de depender de
conversa externa.

---

## Lote 4 — decorações de texto, instanciando o modelo

Executar o modelo com:

- `N_LOTE = 4`
- `LOTE` (ordem por largura crescente): **`Overline`(10) · `Strike`(10) ·
  `Underline`(31)** = 51 sites.
- `NOTAS_FAMÍLIA`:
  - Contentores de prosa com corpo + campos cosméticos: `map_text`/`map_content`
    **recursam** no corpo (precedente Lote 3/Heading); `is_empty` delega ao
    corpo se a forma atual fizer isso (conferir braço a braço — content-
    preserving).
  - Não-locatáveis (confirmar contra `introspect/locatable.rs`; esperado:
    nenhuma é).
  - A família "quebras/espaços" proposta no P318 **não** entra neste lote:
    `Space` foi para a triagem (C2); `Linebreak`/`Colbreak`/`Pagebreak`/
    `VSpace`/`HSpace` ficam para o Lote 5 com a nota de que os três primeiros
    são comandos unit (precedente Divider, elegíveis) — registrar no mapa do
    C3.

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente, validação,
medições vs preditor, proposta do Lote 5 — **como está no modelo**. Lembretes
que o modelo já carrega e aqui só ecoam: prompt grosso que morder → fatiar
primeiro; trait não muda em lote; nenhuma asserção existente alterada.

---

## Relatório (`typst-passo-319-relatorio.md` + resumo no chat)

- As três caronas confirmadas (diff de uma linha onde couber).
- O que o modelo manda para o lote (medições vs preditor; `content.rs`
  antes/depois — trajetória até aqui: 5782 → 5700; suíte; lint 0).
- O mapa do C3 criado e a linha nova no modelo.
- Proposta do Lote 5 derivada do mapa (decisão humana).
- `git log --oneline` dos dois commits; `git status` limpo.

## Fora de escopo

- Lotes 5+ (instância do modelo, a partir do mapa do C3).
- DEBT-58 (a triagem tem gatilho: fim dos lotes element-shaped).
- F / `Set*` / 99.E (o diagnóstico do F consome a medição do P318, com o
  caveat do C1).
- Otimizações sugeridas por qualquer medição (medir ≠ mexer).
