---
# P786 — Triagem em lote: próximos 15 módulos de `lacuna-inventario`

> **Passo:** 786 (segundo lote da fase de triagem em lote, iniciada em P785)
> **Data:** 2026-07-20
> **Foco:** P785 triou os 15 maiores módulos restantes de `lacuna-inventario` e, apesar da classificação inicial "100% mecânica", 4 dos 15 (`typst_syntax::highlight`, `typst_library::foundations::fields`, `typst_syntax::lines`, e `typst_eval::code` como confirmação) continham sinal real quando testados com casos concretos — taxa real de 27% (4/15), não 0%. Todos os quatro já foram corrigidos (P785a/b/c). Este passo continua a triagem com os próximos 15 módulos, aplicando a mesma disciplina reforçada: nenhuma classificação como "mecânica" sem pelo menos um teste real executado.
> **Tipo:** Sonda de triagem em lote, com teste real obrigatório por módulo (não opcional como na primeira tentativa de P785).
> **Tamanho:** L.
> **ADR-0108 EM VIGOR** — a taxa real de P785 (27%, não 0%) mostra que classificar pelo nome/categoria do módulo sem testar é insuficiente; todo módulo precisa de pelo menos 1 documento `.typ` real comparado vanilla vs cristalino antes de classificar como mecânica.
> **ADR-0107** — critério de classificação inalterado: resultado observável de um documento Typst diverge (linguagem) vs estrutura interna sem efeito (mecânica).
> **Dependências:** P785 (lote 1, metodologia corrigida a meio do processo), P785a/b/c (achados do lote 1, todos corrigidos).

---

## Sonda — identificar os próximos 15 módulos

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

Excluir todos os módulos já tratados até agora (série P765a-P785c completa — usar a lista consolidada, não reconstruir de memória). Pegar os 15 módulos seguintes por contagem de itens (a partir do 16º lugar da lista ordenada).

---

## Metodologia (reforçada em relação a P785)

Para cada um dos 15 módulos:

1. Listar os itens do módulo.
2. Ler 1-2 itens representativos no código-fonte do vanilla.
3. **Obrigatório, sem exceção**: escrever pelo menos 1 documento `.typ` mínimo que exercite o comportamento observável do item (não só ler a assinatura Rust e supor) e comparar `vanilla` vs `cristalino` — mesmo se a primeira impressão for "claramente mecânico". P785 já mostrou que nomes como `foundations::fields` (parecia "dispatcher interno") escondiam bug real de campo de linguagem.
4. Só classificar como "mecânica, ADR-0107" depois do teste confirmar ausência de divergência observável.
5. Se o teste revelar divergência: registrar com evidência (comando, saída, não descrição vaga) e decidir se corrige neste passo (achado pequeno e isolado) ou abre passo dedicado (achado maior).

### Binário de referência (fixado, conforme correção já feita em P785)

- Vanilla: `lab/typst-original/target/release/typst`
- Cristalino: `./target/release/typst` (release, não debug)

---

## Tabela de saída esperada

| Módulo | Itens | Teste executado (comando/doc) | Resultado | Classificação | Ação |
|---|---:|---|---|---|---|
| ... | ... | ... | ... | mecânica confirmada / bug real | nenhuma / corrigido / passo dedicado |

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para qualquer correção feita neste passo: teste dedicado.

---

## Critério de fecho do passo

- [ ] 15 módulos seguintes identificados (a partir de onde P785 parou).
- [ ] Cada módulo com pelo menos 1 teste real executado (comando + saída mostrados no relatório, não descrição).
- [ ] Nenhuma classificação "mecânica" sem teste, mesmo quando a primeira impressão sugerir isso.
- [ ] Bugs reais encontrados corrigidos (se pequenos/isolados) ou registrados para passo dedicado (se maiores).
- [ ] Tabela de saída completa com evidência, não afirmações vagas.
- [ ] Taxa de sinal real deste lote registrada, para comparar com a taxa de P785 (27%).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p786.md`.

---

## Próximo passo

Conforme a taxa deste lote: continuar com P787 (lote 3) mantendo o tamanho de 15, ajustar o tamanho do lote, ou considerar se vale a pena fazer o resumo final pendente da série antes de continuar — decisão a registrar com os números acumulados, não por cansaço.
