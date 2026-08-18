# L0 — Passo 1070: Reclassificação N16[α/β/γ] dos 204 Casos `// neutro:` (V16)

**Gate**: nenhum para a Parte 0/1/2 (auditoria + classificação). Execução em massa
(Parte 3) segue `ADR-0127` — mudança em ~204 pontos de código, mesma cautela já
usada para mudanças multi-arquivo nesta conversa.

**Base**: pendência do documento de continuação — os 204 casos de V16 anotados
com `// neutro:` em prosa livre ainda não usam a taxonomia formal `N16[α/β/γ]`
citada nas regras de processo desta conversa ("Taxonomia `N16[α/β/γ]` para
classificar o risco de cada wildcard anotado").

---

## Parte 0 — Ler a definição real de α/β/γ antes de classificar qualquer coisa

**Não tenho, nesta conversa, a definição do que distingue α de β de γ** — só a
menção de que a taxonomia existe e serve para classificar risco. Classificar 204
casos sem saber o critério real seria repetir o erro já cometido antes nesta
conversa (inventar mecanismo sem ler o real).

```bash
find 00_nucleo/prompts -iname "*adr-0017*" -o -iname "*n16*"
grep -rn "N16\[" 00_nucleo/ tekt-linter/ 2>/dev/null | head -20
```

Ler o arquivo encontrado por inteiro. Se a definição de α/β/γ não estiver
completamente clara ou tiver casos ambíguos, registar isso antes de prosseguir —
não assumir um critério razoável por conta própria.

## Parte 1 — Inventariar os 204 casos

```bash
grep -rn "// neutro:" 01_core/src/ 03_infra/ | wc -l
grep -rln "// neutro:" 01_core/src/ 03_infra/
```

Reconciliar contra "204" — mesmo processo de reconciliação já usado no P1064/
P1066 (não assumir que o número antigo está certo, confirmar ou corrigir).

## Parte 2 — Amostragem antes de classificar todos

Escolher uma amostra (mínimo 15-20 casos, distribuídos pelos arquivos com mais
ocorrências) e classificar cada um em α/β/γ segundo o critério real da Parte 0.

Para cada caso da amostra, registar:
1. O texto actual do `// neutro:`.
2. A classificação atribuída (α/β/γ) e o porquê, citando o critério real, não uma
   impressão.
3. Se o caso for ambíguo entre duas classes — não forçar uma escolha, registar
   como "ambíguo" e decidir depois com o dono se ambíguos viram uma classe à
   parte ou são resolvidos caso a caso.

**Se a amostra revelar que a classificação exige julgamento semântico caso a
caso** (não é mecânica, ao contrário do que o documento de continuação supõe —
"mecânico, baixa prioridade"), isso é um achado a reportar antes de prosseguir
para os 204 completos — muda a estimativa de esforço do passo seguinte.

## Parte 3 — Execução, condicional à Parte 2

Só depois de confirmar que o critério é aplicável de forma consistente: propor
ao dono se a reclassificação completa (204 casos) é feita:
(a) manualmente, caso a caso, ou
(b) com heurística assistida (script que sugere classe por padrão textual,
revisão humana em lote).

Não decidir isto sem ver como a amostra se comportou.

## Critério de conclusão deste passo

- Parte 0: definição real de α/β/γ citada, não presumida.
- Parte 1: inventário reconciliado.
- Parte 2: amostra classificada, com nota sobre ambiguidade/consistência do
  critério.
- Parte 3: recomendação de abordagem para os 204, não execução ainda.
