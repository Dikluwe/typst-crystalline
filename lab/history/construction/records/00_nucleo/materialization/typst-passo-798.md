---
# P798 — Triagem em lote: lote 3, próximos 15 módulos de `lacuna-inventario`

> **Passo:** 798 (lote 3 da triagem em lote, continuação de P785/P786)
> **Data:** 2026-07-21
> **Foco:** P785 (lote 1, 15 módulos) e P786 (lote 2, 15 módulos) cobriram 30 módulos do total de ~66 restantes desde P772t. Este passo continua com os próximos 15, mantendo a disciplina reforçada: nenhuma classificação sem teste real executado (lição de P785, onde a primeira tentativa classificou "100% mecânica" sem testar e errou em 4 de 15).
> **Tipo:** Sonda de triagem em lote, com teste real obrigatório por módulo.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR** — nenhuma classificação como "mecânica" sem pelo menos um teste `.typ` real comparado vanilla vs cristalino.
> **ADR-0107** — critério de classificação: resultado observável de um documento Typst diverge (linguagem) vs estrutura interna sem efeito (mecânica).
> **Dependências:** P785 (lote 1), P786 (lote 2, taxa de sinal 80%), toda a cadeia de correções P787-P797 (10 achados de P786 já fechados).

---

## Sonda — identificar os próximos 15 módulos

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

Excluir todos os módulos já tratados: os 30 de P785+P786 (lista consolidada nos respectivos relatórios), mais qualquer módulo que tenha sido coberto incidentalmente pelas correções de P787-P797 (confirmar se algum desses passos tocou em módulos ainda não formalmente triados na lista). Pegar os 15 seguintes por contagem.

---

## Metodologia (mesma disciplina de P786, reforçada)

Para cada um dos 15 módulos:

1. Listar os itens do módulo.
2. Ler 1-2 itens representativos no código-fonte do vanilla.
3. **Obrigatório**: pelo menos 1 documento `.typ` real comparando vanilla vs cristalino antes de classificar, mesmo se a primeira impressão for "claramente mecânico".
4. Registrar evidência real (comando + saída) para cada módulo, não descrição vaga.
5. Achados confirmados: registrar para passo dedicado (não forçar correção neste passo, dado o volume) — mesma decisão de P786 §5.

### Binário de referência (fixado)

- Vanilla: `lab/typst-original/target/release/typst`
- Cristalino: `./target/release/typst` (release)

---

## Tabela de saída esperada

| Módulo | Itens | Teste executado | Resultado | Classificação | Ação |
|---|---:|---|---|---|---|

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] 15 módulos seguintes identificados (a partir de onde P786 parou, confirmado por contagem, não suposição).
- [x] Cada módulo com pelo menos 1 teste real executado, comando e saída mostrados.
- [x] Nenhuma classificação "mecânica" sem teste.
- [x] Achados registrados para passos dedicados (não forçados neste passo).
- [x] Taxa de sinal deste lote registrada, comparável com P785 (27%) e P786 (80%).
- [x] `cargo test --workspace` verde, contagem da suíte mostrada.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p798.md`, com comandos e saídas reais.

---

## Próximo passo

Lote 4 (P799), até cobrir os ~36 módulos restantes — ou reavaliar o tamanho do lote conforme o rendimento observado aqui.
