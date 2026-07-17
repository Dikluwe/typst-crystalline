---
# P765 — L0 e sonda: varredura sistemática da stdlib

> **Passo:** 765
> **Data:** 2026-07-15
> **Foco:** `relatorio-pendencias-proximos-passos.md` (secção 4.2.3 e 5) e o handoff de P762 registam que vários bugs graves da linha `cetz` (P678–P762) — short-circuit de `and`/`or`, `measure()` sempre `0pt`, desestruturação quebrada — só foram encontrados por acidente, ao perseguir outro objectivo. A metodologia de sonda usada em P663/P664 nunca foi aplicada ao resto da stdlib de forma sistemática. Não existe prompt L0 dedicado a este processo de varredura.
> **Tipo:** L0 + Sonda inicial (âmbito, não execução completa). A varredura em si é grande demais para um único passo — este passo decide o âmbito e o formato, e sonda uma amostra pequena para validar o método antes de comprometer o esforço total.
> **Tamanho:** L no total do processo; este passo (definir âmbito + amostra) é S/M.
> **ADR-0108 EM VIGOR** — medir antes de decidir. **ADR-0107** — mecânica-não-língua fica fora do critério de paridade; a varredura foca comportamento observável, não estrutura interna.
> **Dependências:** nenhuma dependência técnica directa. Recomendado depois de P762 (último passo fechado; relatório de pendências, secção 7), sem bloqueio técnico real.

---

## Contexto

Este passo não corrige nada — decide como a varredura vai ser feita e testa o método numa amostra pequena, antes de aplicar a todo o âmbito decidido.

---

## Sonda (amostra de validação do método)

Reconstituir a metodologia de P663/P664 antes de generalizar — não assumir de memória:

```bash
grep -rn "P663\|P664" 00_nucleo/diagnosticos/ 2>/dev/null
```

Aplicar essa metodologia a uma amostra pequena e já conhecida como suspeita — o próprio diagnóstico da lente de 2026-07-15 (secção 4.2) assinala `typst_library::diag` (24 itens: `At`, `Hint`, `Trace`, `PackageError`, ...) como candidato não verificado:

```bash
# Para cada função nativa candidata na amostra, comparar saída directa
cat > /tmp/p765-amostra.typ <<'EOF'
// casos mínimos para as funções da amostra, um por vez
EOF
lab/typst-original/target/release/typst compile /tmp/p765-amostra.typ /tmp/p765-vanilla.pdf
./target/release/typst compile /tmp/p765-amostra.typ /tmp/p765-cristalino.pdf
```

Registar quantas divergências a amostra revela, e quanto tempo/esforço custou por função — essa medição é o que decide se o âmbito total ("toda a stdlib" vs "só namespaces já parcialmente tocados") é viável num número razoável de passos seguintes.

---

## Decisões a registar no L0 (`00_nucleo/prompts/rules/stdlib_audit_methodology.md` — a criar)

| Decisão | Resolver com base em |
|---|---|
| Âmbito (toda a stdlib / só namespaces parciais) | Custo por função medido na amostra |
| Critério de comparação (só resultado do documento, ou também texto de mensagem de erro) | O achado da lente de 2026-07-15 (secção 4.2) já assinala mensagem de erro como observável legítimo — decidir se entra no âmbito aqui |
| Formato de registo de achados | Ficheiro novo (não reaproveitar `achados-adiados-cetz.md`, que é específico de `cetz`) |
| Geração de casos de teste (manual / semi-automática) | Resultado prático da amostra |

---

## Critério de fecho do passo

- [ ] Metodologia de P663/P664 confirmada por leitura directa, não por memória.
- [ ] Amostra (`typst_library::diag` ou outra definida) varrida, com achados classificados: bug real / diferença aceitável (regra 7 do handoff) / scope-out conhecido.
- [ ] Cada decisão da tabela resolvida com base no custo medido na amostra.
- [ ] L0 escrito em `00_nucleo/prompts/rules/stdlib_audit_methodology.md`, com hash calculado.
- [ ] Nenhuma correcção de bug feita neste passo — só registo, salvo se a amostra revelar algo tão simples e isolado que valha registar a decisão explícita de corrigir de imediato.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p765.md`.

---

## Próximo passo

P765a, P765b, ... (execução da varredura completa, em lotes definidos pelo âmbito decidido neste passo) — cada lote com o seu próprio relatório de achados, correcções tratadas em passos separados, um a um ou agrupados por módulo.
