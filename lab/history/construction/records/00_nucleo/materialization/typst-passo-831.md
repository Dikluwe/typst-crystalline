# Prompt — typst-passo-831: triagem sistemática, lote 5 (até 15 dos 22 módulos restantes)

**Origem**: continuação da varredura sistemática — P785 (lote 1), P786 (lote 2), P798 (lote 3, corrigido), P810 (lote 4, 15 módulos, 15 achados, todos já fechados em P811–P830). Restam **22 módulos não triados** (confirmado em P810, corrigindo a estimativa de "~21" do handoff anterior a esse).
**Estado**: aguardando execução

---

## Antes de começar — pré-requisito

Confirmar que o passo 828 (consolidação de P813–P827) já foi commitado e que a árvore está limpa (`git status --short`), e que a suíte completa (`cargo test --workspace`) está verde no estado actual — incluindo P829 e P830. Não iniciar a triagem sobre uma árvore com trabalho não commitado de outros passos em paralelo, pelo mesmo motivo que motivou o P828.

---

## Lições obrigatórias dos lotes anteriores (não repetir)

1. **P798 (lote 3)**: proibido usar documento de teste genérico (`Hello World`) reaproveitado entre módulos — cada módulo precisa de um caso que exercite a funcionalidade real dele. A primeira tentativa de P798 classificou 7 de 15 módulos como "correctos" sem isso; a correcção baixou a taxa de sinal aparente de 100% para 60% real.
2. **P810 (lote 4)**: nenhum "corrigido"/"mecanicamente correcto" sem comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. A contagem de testes da suíte `typst-core` tem de aparecer e bater com os testes novos declarados.
3. **P829/P830**: se algum achado exigir uma decisão de escopo (não só correcção), registar a decisão com proveniência real — nunca escrever "decisão do dono" sem uma decisão do dono de facto ter acontecido. Se não houver decisão disponível no momento, o achado fica registado como pendente de decisão, não como fechado.

---

## Passo 1 — Selecção do lote

1. Ler `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (ou o inventário `lacuna-inventario` equivalente vigente) e cruzar com os módulos já triados: os 45 de P785+P786+P798, mais os 15 de P810 (lote 4) = 60 módulos já triados. Confirmar que restam 22 (ou o número real, se o inventário tiver mudado desde P810 — não assumir).
2. Seleccionar até 15 dos 22 restantes. Registar o critério de selecção usado (P810 escolheu "maior superfície de língua" — pode manter o mesmo critério ou justificar outro).
3. Registar a lista exacta dos módulos seleccionados no relatório antes de começar a triagem.

## Passo 2 — Triagem módulo a módulo

Para cada módulo seleccionado:
1. Escrever um documento `.typ` que exercite a funcionalidade específica do módulo (não genérico — lição de P798).
2. Compilar com os dois binários (vanilla `lab/typst-original/target/release/typst`, cristalino `./target/release/typst`), comparar a saída literal.
3. Classificar: paridade confirmada, ou achado (divergência real).
4. Para cada achado: registar comando + saída literal de ambos os binários, e localizar o ponto do código vanilla e do cristalino relevante (sonda, sem corrigir ainda — corrigir é trabalho de passos dedicados posteriores, como foi feito para os achados de P810 em P811–P830).
5. Se um módulo já tiver sido tocado incidentalmente por um passo anterior (ex.: um achado de outro módulo levou a uma correcção que passa por este módulo também), verificar se ainda há divergência real ou se já está em paridade por efeito colateral — não assumir que precisa de achado novo só porque está na lista.

## Passo 3 — Relatório

Produzir `00_nucleo/diagnosticos/typst-passo-831-relatorio.md` com:
- Lista dos módulos seleccionados (Passo 1), com o critério de selecção.
- Para cada módulo: o comando/documento de teste usado, a saída de ambos os binários, e a classificação.
- Taxa de sinal real (achados reais / módulos triados) — mostrar o cálculo, não só o número final.
- Tabela de achados pendentes, no mesmo formato usado nos handoffs anteriores (número, módulo, achado), para dar entrada na fila de próximos passos dedicados. Continuar a numeração dos achados a partir de onde a fila de P810 parou (ela foi até #16), para não colidir com os números já usados.
- Contagem actualizada: quantos módulos restam não triados após este lote (22 menos os triados aqui).
- Se restarem 0 módulos, registar que a varredura sistemática original (lista de P772t) está completa.
