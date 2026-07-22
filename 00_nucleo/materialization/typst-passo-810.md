# Prompt — typst-passo-810: triagem sistemática, lote 4 (~15 de ~21 módulos restantes)

**Origem**: continuação da varredura sistemática iniciada em P772t (lista congelada), lotes anteriores P785 (lote 1), P786 (lote 2), P798 (lote 3, corrigido)
**Estado**: aguardando execução

---

## Contexto

A lista congelada de `lacuna-inventario` está em `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (~178 itens em ~66 módulos originalmente). P785+P786+P798 já triaram 45 módulos. Restam aproximadamente **21 módulos não triados** — este passo cobre o próximo lote de até 15.

---

## Lição obrigatória de P798 (não repetir o erro)

A primeira tentativa de P798 classificou 7 de 15 módulos como "correctos" usando testes genéricos (`Hello World`) que não exercitavam o módulo real. A correcção baixou a taxa de sinal aparente de 100% para 60% real. **Cada módulo triado neste lote precisa de um caso de teste que exercite especificamente a funcionalidade desse módulo** — não um documento genérico reaproveitado entre módulos.

Regra geral já estabelecida no projecto: nenhum relatório de "corrigido"/"mecanicamente correcto" é aceite sem comando exacto + saída literal (vanilla vs cristalino) para cada afirmação.

---

## Passo 1 — Selecção do lote

1. Ler `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (ou o inventário equivalente que exista no momento da execução) e cruzar com os módulos já triados em P785, P786 e P798 (ver `00_nucleo/handoff-novo-chat-p798.md` para a lista dos 45 já cobertos, se o inventário em si não marcar isso explicitamente).
2. Seleccionar até 15 módulos ainda não triados. Se restarem menos de 15, o lote cobre todos os que restam (fecha a varredura sistemática original).
3. Registar a lista exacta dos módulos seleccionados no relatório, antes de começar a triagem.

## Passo 2 — Triagem módulo a módulo

Para cada módulo seleccionado:
1. Escrever um documento `.typ` que exercite a funcionalidade específica do módulo (não genérico).
2. Compilar com os dois binários (vanilla e cristalino), comparar a saída literal.
3. Classificar: paridade confirmada, ou achado (divergência real).
4. Para cada achado: registar comando + saída literal de ambos os binários, e localizar o ponto do código vanilla e do cristalino relevante (sonda, sem corrigir ainda — corrigir achados de triagem é trabalho de passos dedicados posteriores, como foi feito para os achados de P798 em P799-P807).

## Passo 3 — Relatório

Produzir `00_nucleo/materialization/typst-passo-810-relatorio.md` com:
- Lista dos módulos seleccionados (Passo 1).
- Para cada módulo: o comando/documento de teste usado, a saída de ambos os binários, e a classificação.
- Taxa de sinal real (achados reais / módulos triados) — mostrar o cálculo, não só o número final.
- Tabela de achados pendentes, no mesmo formato usado no handoff `p798` (número, módulo, achado), para dar entrada na fila de próximos passos dedicados.
- Contagem actualizada: quantos módulos restam não triados após este lote.
- Se restarem 0 módulos, registar que a varredura sistemática original (lista de P772t) está completa.
