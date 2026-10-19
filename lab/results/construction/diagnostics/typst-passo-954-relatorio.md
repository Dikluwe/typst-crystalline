# Passo 954 — Relatório (origem da decisão "verboso primeiro, compacto depois, acessibilidade em eixo separado" + verificação de colisão de ADR)

**Data**: 2026-08-03
**Estado da árvore**: commit base `8bda8f44e`; este passo só adiciona documentação
(ADR nova + índice + este relatório) — zero código.

---

## 1. Fase A — origem da decisão: NÃO está registada em nenhum passo

Varreduras executadas (padrão → resultado):

| Busca | Âmbito | Resultado |
|---|---|---|
| `verbos\|compact\|acessibil\|BDC\|EMC\|tagged\|artifact` (icase) | `diagnosticos/*95[0-3]*` | Só P953 §4: `BDC`/`EMC` como scope-out mantido |
| mesmo padrão + `eixo separado\|ordem de prioridade` | `diagnosticos/*94[4-9]*` | Zero ocorrências |
| `verbos\|compact\|acessibil\|BDC\|EMC\|tagged\|decalque` | `00_nucleo/prompts/` | Zero ocorrências relevantes (só usos não relacionados: "tabela compacta", marker trait `Tagged` da introspecção, etc.) |
| `verbos` | `diagnosticos/` (todo) | Só frentes antigas de tamanho de PDF: P883 ("redução de verbosidade é outra frente separada") e P884 §4 (sonda da frente 2: ~199 vs ~18 blocos `BT … ET` por página vs vanilla, "requer passo dedicado") |
| `L11` | `00_nucleo/*.md`, `adr/` | `L11` = lição "alegações atestadas" de `Tekt/LESSONS.md` (externa), não uma ADR deste repo |

**Antecedentes registados mais próximos** (nenhum contém a formulação):

1. **P953 §4** — `BDC`/`EMC` (PDF tagueado) scope-out deliberado; "se se quiser
   promover a prioridade activa, é uma decisão separada do dono".
2. **P883/P884** — frente 2 (verbosidade dos operadores PDF) sondada e adiada.

**Veredicto (Fase A.2 do passo)**: a decisão foi tomada **directamente na conversa
com o dono, fora do ciclo normal de passo**, entre P953 e P954 — o cabeçalho do
próprio P954 é o primeiro registo escrito ("nova descoberta, ordem de prioridade
que acabamos de fechar"). A ADR referencia esta proveniência real; **não se
inventou um passo de origem**.

## 2. Fase B — verificação de colisão com ADRs existentes

Varredura de `00_nucleo/adr/` pelos padrões `verbos|compact|acessibil|BDC|EMC|
tagged|decalque|L11|FlateDecode|content stream|compress` + leitura integral de
ADR-0114, ADR-0117 e ADR-0119:

- **ADR-0114 / ADR-0117** (sonda antes da spec): método de investigação — não
  decidem ordem de implementação de modos de exportação. **Sem colisão.**
- **ADR-0119** (disciplina de verificação): complementar — o "decalque" da
  decisão nova é uma aplicação concreta de ADR-0119 §5, não uma duplicação.
  **Sem colisão.**
- **ADR-0120** (`TextShaped`): "acessibilidade" só de passagem (`char_code`
  para ToUnicode). **Sem colisão.**
- **ADR-0060**: alt-text de imagem — acessibilidade de conteúdo, não PDF
  tagueado. **Sem colisão.**
- Nenhuma ADR trata de modos de verbosidade do export nem de PDF tagueado.

**Número novo confirmado por listagem real do directório** (`ls 00_nucleo/adr/`):
último ficheiro `typst-adr-0125-*` → **slot livre = 0126**. (Não assumido — o
erro de P910 com ADR-0112 não se repete.)

## 3. Fase C — decisão: ADR nova (0126)

- **Não é extensão** de 0114/0117/0119: essas são de método (como investigar e
  verificar); esta é de produto/prioridade (o que se constrói primeiro e como se
  valida). Assuntos distintos, ficheiros distintos.
- **Não é nota leve em L0**: ainda não existe código desta frente — não há L0 a
  que pendurar a nota; a decisão é perene e o veículo do projecto para decisões
  perenes é ADR.
- **Escrita**: `00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md`, com a
  formulação do dono citada verbatim, a interpretação do âmbito (content streams
  PDF) **marcada como inferência** com o que a refutaria (ADR-0108), alternativas
  consideradas e consequências.
- Índice `adr/README.md`: linha na tabela (após 0125) + entrada no ledger
  ("Total pós-P954").

## 4. Resultado

- Origem: conversa directa com o dono (entre P953 e P954) — citada, não presumida.
- Colisão: nenhuma (tabela em §2 e na ADR §3).
- ADR-0126 escrita e indexada. `crystalline-lint .` sem violações novas (passo
  sem código; resta só o V7 órfão pré-existente alheio).
