# ADR-0114 — Sonda A.0 antes da spec: gate duro de medição

**Status**: IMPLEMENTADO  
**Data**: 2026-06-23  
**Decisão**: A sonda do substrato (Fase A.0) é pré-condição para redigir uma spec de materialização. Se a spec for escrita primeiro, deve ser tratada como verificação retroativa, não como plano de materialização.  
**Decisor**: Plano de correção de derivas P388–P427.

## 1. Contexto

O Protocolo de Nucleação, ADR-0065 (inventariar primeiro) e ADR-0084 (Fase A antes de decisão) já exigiam sonda antes de decisão. Na prática, entre P388 e P427, cinco specs foram redigidas como materialização antes de a sonda A.0 confirmar o substrato:

- **P388**: spec de `bibliography`/`cite` escrita antes da sonda de viabilidade do runtime de introspecção.
- **P409**: aritmética `Duration` assumida como não implementada; sonda revelou que P405 já a fizera.
- **P413**: aritmética `Decimal` assumida como não implementada; sonda revelou que P404 já a fizera.
- **P416**: footnote body no rodapé assumido como não implementado; sonda revelou que P304/P305 já o faziam.
- **P421**: `repr()` nativa assumida como S; sonda forçou reclassificação para M.

Em todos os casos, o conteúdo factual estava correto *a posteriori*, mas a ordem invertida gerou trabalho retroativo (reclassificações, relatórios de verificação, correções documentais). Esta ADR transforma a regra existente num gate operacional verificável.

## 2. Decisão

**Gate duro**: uma spec só pode declarar-se "spec de materialização" se o ficheiro de output da sonda A.0 existir em `00_nucleo/diagnosticos/` **antes** da redação da spec.

**Se a spec for escrita primeiro** (por qualquer razão processual), ela é reclassificada automaticamente como **verificação retroativa** quando a sonda revelar que o trabalho já existe ou que o escopo é diferente. Nesse caso:
- O cabeçalho da spec ganha uma nota de reclassificação retroativa.
- O passo não produz código novo além do que a verificação exigir.
- O artefacto principal passa a ser o relatório de verificação.

**Output da sonda**: deve ser um ficheiro imutável (paridade ADR-0085, diagnóstico imutável) com:
- Comandos executados.
- `file:line` dos substratos verificados.
- Resultado pass/fail por critério.
- Decisão derivada do resultado.

## 3. Consequências

- **Positiva**: evita specs de materialização para trabalho já feito.
- **Positiva**: força medição antes de estimativa de custo/tamanho.
- **Positiva**: torna a reclassificação retroativa um caminho normal, não uma exceção.
- **Negativa**: adiciona um artefacto obrigatório antes da redação da spec.
- **Negativa**: specs rápidas de emergência precisam do mesmo gate; não há atalho.

## 4. Passos base empírica

| Passo | O que a sonda revelou | Correcção aplicada |
|-------|----------------------|--------------------|
| P388 | Runtime de introspecção já suporta query ordenada, `And`/`Or`, `position_of` | Nota retroativa na spec confirmando que o faseamento não precisou de ajuste |
| P409 | `Duration` já tem aritmética desde P405 | Spec reclassificada como verificação retroativa |
| P413 | `Decimal` já tem aritmética desde P404 | Spec reclassificada como verificação retroativa |
| P416 | Footnote body no rodapé já existe desde P304/P305 | Spec reclassificada como verificação retroativa |
| P421 | `native_repr` não existe; passo é M, não S | Reclassificação S→M registada; lição reforça o gate |

## 5. Anti-padrões

- **Não** escrever uma spec de materialização baseada apenas em memória ou suposição de estado.
- **Não** confundir "a sonda pode correr depois" com "a sonda é opcional".
- **Não** apagar uma spec escrita antes da sonda; convertê-la em verificação retroativa preserva histórico.

## 6. Referências

- Protocolo de Nucleação em `CLAUDE.md`.
- ADR-0065 — Inventariar antes de decidir.
- ADR-0084 — Fase A antes de decisão.
- ADR-0085 — Diagnósticos imutáveis.
- ADR-0108 — Medir antes de decidir.
