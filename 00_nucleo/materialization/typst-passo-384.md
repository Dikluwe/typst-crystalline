# 🔎 Passo 384 — Recon amplo read-only (estado do projeto + o que falta)

**Tipo**: diagnóstico read-only (não materializa código, não move ficheiros)
**Status**: `PLANO` — definição do passo antes de execução
**Data**: 2026-06-19
**Caminho previsto no repositório**: `00_nucleo/diagnosticos/diagnostico-recon-amplo-passo-384.md`
**ADRs relevantes**: ADR-0109 (atomização — não revogada), ADR-0033 (paridade vanilla), ADR-0065 (critério #5 — escopo determinado por inventário)
**Precedentes diretos**: P148 (Inventário cobertura), P154A (diagnóstico Model), P156A (historiograma), P156B (diagnóstico Layout), P160 (diagnóstico Introspection)

---

## §0 — Natureza e travas (ler antes de correr)

Este passo é **read-only**. As travas que governam o trabalho continuam em vigor e este passo não as toca:

| Trava | Estado neste passo |
|-------|--------------------|
| Não mover ficheiros | Cumprido — recon não move nada |
| Não materializar código | Cumprido — zero `.rs` alterado |
| L0-commit antes de mover | Não aplicável (nada se move); mas o relatório do passo deve ser **commitado** assim que escrito, por causa do mecanismo externo que apaga `.md` não-commitados |
| Anti-deriva | Cumprido — passo apenas lê e descreve |
| ADR-0109 (atomização) | Referência, não execução. Atomização = distribuir lógica para ficheiros legíveis sozinhos. **NÃO** zerar `content→elements`. **NÃO** despacho dinâmico. Manter `match` exaustivo |
| Marco G / desacoplamento | Descartado — não entra no backlog |

O recon **mapeia e prioriza**; **não decide** o que atomizar nem decide crates. Essas decisões ficam para depois (ver §7).

---

## §1 — Objetivo

Produzir uma fotografia atual e factual do projeto inteiro e, a partir dela, a lista priorizada do que falta fazer. Em concreto, responder a quatro perguntas:

1. **Onde está a cobertura hoje?** (vanilla vs cristalino, por domínio) — refrescar o Inventário 148, que está parado há ~220 passos.
2. **Onde estão os monólitos?** (ficheiros grandes, candidatos a atomização per ADR-0109) — por camada, sem mover nada.
3. **O que está em aberto?** (DEBTs abertos; ADRs `PROPOSTO` por materializar; divergências vanilla não-formalizadas).
4. **Qual a próxima sequência defensável?** (backlog priorizado, com tamanho estimado e bloqueios).

---

## §2 — Âmbito da varredura

Toda a árvore de produção e os documentos do núcleo:

- `00_nucleo/` — prompts L0, ADRs, DEBT.md, diagnósticos, materialização.
- `01_core/` (L1) — entities, contracts, rules.
- `02_shell/` (L2) — CLI, formatadores.
- `03_infra/` (L3) — I/O, fontes, filesystem.
- `04_wiring/` (L4) — composição.
- `lab/` — incluído **apenas** para confirmar que continua isolado (sem import de produção); não entra no backlog.

---

## §3 — Eixos de análise

### Eixo A — Cobertura por domínio (refresh do Inventário 148)

Documento-mestre a refrescar: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

Classes do inventário (manter as 5 já usadas): `implementado` / `implementado⁺` / `parcial` / `ausente` / `scope-out`.

Última cobertura conhecida (a confirmar — pode ter mudado desde então):

| Domínio | Cobertura registada | Passo da última medição |
|---------|--------------------:|-------------------------|
| User-facing (agregado) | ~54% → ~61% | P148 → ~P157A |
| Arquitetural (agregado) | ~70% → 72% | P148 → P149 |
| Layout | ~78% | P156J |
| Model | ~50% | série P157–P159 |
| Introspection | ~17% | P160 |
| Math / Parse / Lexer / Eval / Stdlib / Visualize | não re-medidos recentemente | — |

Tarefa do eixo: recontar cada domínio contra a árvore atual e marcar deltas face ao valor registado. Atenção especial à Introspection, porque M3–M9 + sealing entraram **depois** da medição de 17% — o número quase de certeza subiu.

### Eixo B — Monólitos por camada

Definição operacional de "monólito" para este passo (critério a fixar antes de correr; sugestão de partida):

- Ficheiro `.rs` de produção acima de um limiar de linhas (sugestão: **> 600 LOC**), **ou**
- Ficheiro que concentra um `match` exaustivo grande **mais** lógica por-arm densa que poderia viver em ficheiros legíveis sozinhos (espírito ADR-0109).

Saída: tabela por camada. Exclui o que já foi atomizado nas duas camadas fechadas (Layout máquina + math; introspect máquina) per P383.

| Camada | Ficheiro | LOC | Tem `match` grande? | Candidato a atomização? | Notas |
|--------|----------|----:|:-------------------:|:-----------------------:|-------|
| L1 | _(preencher)_ | | | | |
| L2 | _(preencher)_ | | | | |
| L3 | _(preencher)_ | | | | |
| L4 | _(preencher)_ | | | | |

Regra: este eixo **descreve**, não propõe movimentos. A decisão de quais atomizar é §7.

### Eixo C — DEBTs abertos

Ler `00_nucleo/DEBT.md` inteiro e listar cada DEBT aberto com: estado atual, o que falta para fechar, tamanho estimado, e se está bloqueado.

| DEBT | Título curto | Estado | Falta para fechar | Bloqueio |
|------|--------------|--------|-------------------|----------|
| _(preencher)_ | | | | |

### Eixo D — ADRs `PROPOSTO` por materializar

Ler `00_nucleo/adr/README.md` e listar cada ADR com status `PROPOSTO` (backlog arquitetural). Conhecidas à data: ADR-0061 (Layout Fase X roadmap), ADR-0066 (Introspection runtime), ADR-0067 (attribute-grammar scoping), entre outras — **confirmar a lista completa** contra o README, pois a contagem muda a cada passo administrativo.

| ADR | Tópico | O que falta para `EM VIGOR`/`IMPLEMENTADO` |
|-----|--------|---------------------------------------------|
| _(preencher)_ | | |

### Eixo E — Divergências vanilla

Cruzar com ADR-0033. Separar divergências **registadas** (já têm ADR/nota) das **não-registadas** (aparecem no código mas sem formalização). As não-registadas são dívida documental, não código.

---

## §4 — Método (read-only)

Tudo abaixo é leitura. Nenhum comando escreve na árvore de código.

1. **LOC por ficheiro** (insumo do Eixo B): contar linhas dos `.rs` de produção, ordenado decrescente, por camada. Confirmar o limiar de "monólito" com base na distribuição real antes de classificar.
2. **`match` arms** (insumo dos Eixos A e B): localizar os `match content`/`match self` grandes e contar arms por sítio. Cruzar com os ~7 sítios já conhecidos de cobertura exaustiva de `Content` (`plain_text`, `is_empty`, `PartialEq`, `map_content`, `map_text`, `materialize_time`, `walk`, `layout_content`).
3. **Inventário 148** (Eixo A): abrir o documento-mestre, anotar a data da última atualização, recontar por domínio contra a árvore atual.
4. **DEBT.md** (Eixo C): leitura integral; uma linha de tabela por DEBT aberto.
5. **README ADRs** (Eixo D): leitura da secção de distribuição; extrair a lista `PROPOSTO`.
6. **Confirmar isolamento de `lab/`** (Eixo B, sanidade): `crystalline-lint` não deve reportar V10 (QuarantineLeak). Correr o linter em modo de leitura serve de verificação cruzada do estado "verde" herdado do P383.

Limiares e critérios exatos (o que conta como monólito, que domínios re-medir primeiro) ficam fixados no topo do relatório, antes das tabelas, para a contagem ser reproduzível.

---

## §5 — Artefactos de saída

O passo produz `.md` (e nada além de `.md`). Todos commitados assim que escritos.

| Artefacto | Conteúdo |
|-----------|----------|
| `typst-cobertura-vanilla-vs-cristalino.md` (atualizar) | Refresh do Inventário 148 com cobertura por domínio à data do P384 e deltas face à última medição |
| `diagnostico-recon-amplo-passo-384.md` (este, materializado) | Os cinco eixos preenchidos: cobertura, monólitos por camada, DEBTs, ADRs `PROPOSTO`, divergências |
| `backlog-priorizado-passo-384.md` | A lista do que falta, ordenada por prioridade, com tamanho (XS/S/M/L/XL) e bloqueios |
| `typst-passo-384-relatorio.md` | Relatório do passo no formato de `00_nucleo/materialization/`: o que se fez, sem regressão (recon não toca testes), lint verde, sem ADR nova, sem DEBT criado/fechado |

Formato do backlog priorizado (o produto que responde "o que falta"):

| # | Item | Domínio | Tamanho | Bloqueado por | Razão da prioridade |
|---|------|---------|:-------:|---------------|---------------------|
| 1 | _(preencher)_ | | | | |

---

## §6 — Critérios de conclusão (checklist)

- [ ] Limiar de "monólito" fixado e justificado pela distribuição real de LOC.
- [ ] Inventário 148 refrescado; cobertura por domínio re-medida; deltas anotados.
- [ ] Mapa de monólitos por camada (L1–L4) completo; `lab/` confirmado isolado.
- [ ] Todos os DEBTs abertos listados com estado e falta-para-fechar.
- [ ] Todas as ADRs `PROPOSTO` listadas com o que falta para promover.
- [ ] Divergências vanilla separadas em registadas vs não-registadas.
- [ ] Backlog priorizado escrito, com tamanho e bloqueios.
- [ ] `crystalline-lint` verde (verificação cruzada, leitura).
- [ ] Suíte continua verde (não foi tocada; confirmar 0 alterações de código).
- [ ] Zero ADR nova, zero DEBT criado ou fechado (é diagnóstico).
- [ ] Relatório do passo escrito **e commitado**.

---

## §7 — Portões de decisão pós-recon (fora deste passo)

O P384 entrega dados; as decisões são passos próprios, **decisão sua**, depois de ler o backlog:

1. **Quais monólitos atomizar primeiro** — escolher do mapa do Eixo B, respeitando ADR-0109 (legibilidade isolada; sem zerar `content→elements`; sem despacho dinâmico; `match` exaustivo mantido).
2. **Decisão de crates** — a frente que estava a seguir à varredura no plano original; só se aborda depois do backlog estar à frente.

Nenhum dos dois entra no P384.

---

## §8 — Riscos e mitigações

| Risco | Mitigação |
|-------|-----------|
| Limiar de monólito arbitrário inflaciona ou esvazia a lista | Fixar o limiar a partir da distribuição real de LOC, não a priori |
| Re-medir cobertura à mão introduz erro | Usar as mesmas 5 classes e a mesma metodologia do Inventário 148 para os deltas serem comparáveis |
| Recon "derrapa" para começar a mover/atomizar | Trava §0: read-only; qualquer movimento é passo separado pós-§7 |
| Relatório apagado pelo mecanismo externo | Commitar os `.md` assim que escritos (regra L0-commit) |

---

## §9 — Referências

- Inventário 148 — `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- ADR-0109 (atomização) — define o que conta como atomização legítima.
- ADR-0033 (paridade vanilla) — base do Eixo E.
- ADR-0065 critério #5 — escopo determinado por inventário (justifica diagnóstico-primeiro).
- P383 — fecho das duas camadas atomizadas (Layout máquina + math; introspect máquina); HEAD atual.
- P377 — renumeração da ADR de atomização para 0109.
- P160 — diagnóstico Introspection (estrutura §1–§6 reutilizada e ampliada aqui).
- `CLAUDE.md` — protocolo de nucleação e travas de camada.

---

## §10 — Histórico de revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-06-19 | Plano do P384 redigido (recon amplo read-only) — definição antes de execução | este ficheiro |
