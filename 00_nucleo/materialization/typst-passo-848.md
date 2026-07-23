# Prompt — typst-passo-848: triagem sistemática, lote 6 — os 7 módulos finais

**Origem**: continuação da varredura sistemática — P785 (lote 1) → P786 (lote 2) → P798 (lote 3, corrigido) → P810 (lote 4) → P831 (lote 5). Restam exatamente **7 módulos**, confirmados por P831: `typst_library::foundations::styles::rule`, `typst_library::layout::abs`, `typst_library::layout::axes`, `typst_library::layout::corners`, `typst_library::layout::fragment`, `typst_utils::pico::bitcode`, `typst_utils::pico::exceptions`.
**Estado**: aguardando execução — **este é o último lote**. Depois dele, a varredura sistemática da lista original de P772t está completa.

---

## Antes de começar

Confirmar que a árvore está limpa e a suíte completa está verde (mesmo pré-requisito de P831/P848 anteriores — não iniciar sobre trabalho não commitado).

## Contexto sobre estes 7 módulos

P831 já classificou os 7 como "Tier D — mecânica pura", e nota que os 2 módulos de `pico` (`bitcode`, `exceptions`) já têm uma verificação escrita de uma reverificação anterior (de P785) que os classificou como mecânica. Isso **não dispensa a sonda deste passo** — a lição de P798 continua valendo: nenhuma classificação como "mecânica" sem pelo menos um teste real por módulo, mesmo que a expectativa seja de que sairá limpo.

## Passo 1 — Confirmação da lista

Reconfirmar contra o inventário atual (`00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` ou o que estiver vigente) que estes 7 são de fato os únicos módulos não triados. Se o inventário tiver mudado desde P831 (novos itens adicionados, por exemplo), registrar a diferença antes de prosseguir.

## Passo 2 — Triagem módulo a módulo

Para cada um dos 7:
1. Escrever um documento `.typ` que exercite a funcionalidade específica do módulo — mesmo sendo "mecânica pura" esperada, o teste tem que ser real, não pulado.
2. Compilar com os dois binários, comparar a saída literal.
3. Classificar: paridade confirmada (com o teste que prova isso), ou achado.
4. Se algum dos 7 revelar um achado real (contra a expectativa de P831), tratar como qualquer achado novo — sonda completa, código identificado, sem corrigir neste passo (é passo de triagem).

## Passo 3 — Fechamento da varredura sistemática

Se os 7 saírem todos em paridade confirmada (ou os achados encontrados forem registrados normalmente): declarar formalmente, no relatório, que a lista original de P772t (que começou com ~178 itens em ~66 módulos, depois revisada por P831 para 82 módulos) está **completamente triada**. Isso fecha um ciclo que vem desde P772t/P785 — vale marcar com clareza no relatório e no próximo handoff, não deixar como só mais um lote igual aos anteriores.

## Relatório

`00_nucleo/diagnosticos/typst-passo-848-relatorio.md`, mesmo formato dos lotes anteriores: lista dos 7 módulos com o teste usado e a saída de cada binário, taxa de sinal real, tabela de achados novos (se houver, continuando a numeração da fila a partir de onde P831 parou — ela foi até #60), e a declaração de fechamento da varredura sistemática.
