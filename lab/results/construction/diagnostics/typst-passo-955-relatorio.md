# Passo 955 — Relatório (origem do formato actual do exportador PDF: `Td`, um `BT…ET` por item, sem `q`/`cm`/`Q`, sem `BDC`/`EMC`)

**Data**: 2026-08-03
**Estado da árvore**: commit base `0c6bf3c16` (P954); este passo só toca
documentação (ADR-0126 reforçada + índice + este relatório) — zero código.
**Pré-condição**: árvore limpa, ADR-0126 presente ✓.

---

## 1. Resposta curta

**Não existe decisão antiga sobre o formato do content stream.** O formato
actual foi **especificado directamente na spec do Passo 20** ("export_pdf() e
PDF mínimo válido", 2026-03-28, commit `f0f81549c` "Passo 10-23" — commit que
cria `03_infra/src/export.rs`) e **nunca foi reconsiderado**. Não é "formato
simples escolhido conscientemente sobre `Tm`/`q`/`cm`/`Q`" — é a implementação
mínima viável da altura. A distinção pedida pela Fase A.4 do passo fica
registada na ADR-0126 §2 (nota de complemento).

## 2. Fase A — a varredura (comandos e resultados)

| Passo | Busca | Resultado |
|---|---|---|
| Intervalo de nascimento | `git log --diff-filter=A --follow -- 03_infra/src/export.rs` | Criado em `f0f81549c` "Passo 10-23" (2026-03-28) |
| Afinação do intervalo | grep `pdf\|BT\|Td\|export` em `materialization/typst-passo-1[0-9].md` e `2[0-3].md` | **Passo 20**: "export_pdf() e PDF mínimo válido" — cria `export.rs`; o emit já é `BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n({safe}) Tj\nET\n` (`typst-passo-20.md:212`) |
| Decisão explícita `Td` vs `Tm` | grep `` `Tm`` ``, `Td ou Tm`, `Td vs Tm`, `em vez de Tm`, etc. em todo `00_nucleo/` | Só 2 menções posteriores, nenhuma uma decisão sobre o formato base: **P282 §A1.1** (pergunta de inventário de paridade top-level/local) e **P486 §B.3** (`Td`/`Tm` para `y_offset` por glifo dentro de `TJ` — sub-item adiado por sonda, âmbito diferente) |
| `BT…ET` por item / `q`/`cm`/`Q` | greps em materialization + relatorios | Passos posteriores **estendem** o formato sem o reavaliar: P137 (`Tc` dentro do bloco), P139 (wrapper `q BT … ET Q` só para stroke), P281/282 (helper de emit unificado), P483+ (`TJ`/shaping) |
| ADR antiga sobre o stream | leitura de ADR-0019/0020/0022/0027/0053/0054/0055 + grep `FlateDecode\|content stream\|compress` em `adr/` | Nenhuma trata da estrutura do stream; todas são de fontes/embedding (0027 CIDFont, 0055 consumer) ou de critérios de fecho (0054). ADR-0054 menciona "compressão, IDs" como inalcançáveis para o fecho de DEBT-1 — critério de aceitação, não formato |
| L0 actual | grep `Td\|BT\|Tm` em `prompts/infra/export/` | `stream.md` documenta `Td` como convenção factual (ex.: P788) sem registar alternativas |
| Relatório do Passo 20 | `ls materialization/*passo-20*` | Não existe `-relatorio`; a spec do passo (secção "Ao terminar, reportar") não discute o formato |

**Observação sobre a spec do Passo 20**: o único gate de decisão explícito do
passo é a Tarefa 1 — "se o original usa `pdf-writer`/`krilla` → criar ADR-0027"
(decisão sobre *crate*, não sobre formato). O formato do stream entra directo
no código de referência da spec, sem secção de alternativas.

## 3. Fase B — reconciliação com ADR-0126

Não havendo decisão antiga (com ou sem razões registadas), **não há tensão a
reconciliar**: a prioridade "verboso primeiro, compacto depois" da ADR-0126 não
contradiz nenhuma decisão registada — dá estatuto de baseline de auditoria a um
formato que já existe de facto desde o Passo 20.

Actualização aplicada à ADR-0126 (caso "nada encontrado" da Fase B.3):

- §2 ganha o **complemento de proveniência P955**: varredura desta vez sobre
  todo o histórico; origem do formato no Passo 20; registo explícito da
  distinção "nunca decidido contra alternativas" ≠ "escolha consciente do
  formato simples"; identificação concreta do que o "modo verboso" é hoje
  (o formato P20: um `BT…ET` por item, `Td`, sem `q`/`cm`/`Q`, sem `BDC`/`EMC`).
- Cabeçalho e §6 (Referências) actualizados com o complemento.
- Ledger do `adr/README.md` anotado (complemento P955, sem ADR nova).

## 4. Notas de margem (achados da varredura, sem acção neste passo)

- O Passo 20 não tem relatório próprio em `materialization/` (época anterior à
  disciplina de relatório por passo) — a spec é a única fonte.
- O sub-item `y_offset` por glifo de P486 §B.3 ficou adiado "por sonda" — se
  alguma vez for retomado, a sequência `0 y_pt Td … 0 -y_pt Td` que ele descreve
  interage com o modo compacto da ADR-0126; registar nessa spec futura.
