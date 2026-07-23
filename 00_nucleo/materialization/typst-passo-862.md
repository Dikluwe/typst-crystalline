# Prompt — typst-passo-862: fusão de texto no parser (achado #41, F2, de P843/P859/P861)

**Origem**: achado #41 de P843, confirmado ainda aberto por P861, com a causa identificada por P861: `01_core/src/engine/parse/markup.rs:96-102` — `SyntaxKind::Text` é consumido como um único nó de uma vez, sem separar em `Text`/`Space`/`Text` como o vanilla.
**Estado**: aguardando execução

---

## Achado

`[hello world]` — vanilla produz três nós de content tree (`Text("hello")`, `Space`, `Text("world")`); cristalino produz um único `Text("hello world")`. O `repr()` de texto simples já bate (P843 corrigiu a formatação), mas a granularidade interna diverge — isso é a causa raiz confirmada por P859/P861 de pelo menos parte dos sintomas de composição encontrados neste lote (embora `show par`/agrupamento de listas, tratados em prompts separados, possam ter causas próprias, não necessariamente esta).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Confirmar a divergência de granularidade com `#repr([hello world])` mostrando a estrutura interna (não só o texto renderizado) — usar o mesmo método de P843 para inspecionar a árvore de content, não só a string final.
2. Localizar exatamente onde o parser decide juntar palavras num único `Text` (`markup.rs:96-102`, confirmado por P861) e entender por que essa decisão foi tomada assim originalmente — procurar no histórico/L0 se há alguma razão registrada (performance, simplicidade) ou se foi só como o parser cresceu.
3. Confirmar no vanilla como o parser mantém a granularidade — cada palavra e espaço como nó separado — e como isso se relaciona com a fase `realize` (que depois os reagrupa, conforme já documentado por P859).

## Passo 2 — Avaliar o impacto de mudar

Antes de implementar, medir o custo: separar cada palavra em nós distintos multiplica o número de nós de content tree por documento — confirmar se isso tem impacto de performance perceptível (compilar um documento grande, medir tempo antes/depois de um protótipo mínimo) e se algum consumidor do content tree no cristalino hoje assume implicitamente que texto vem fundido (buscar por `Content::Text` no código e ver se algum lugar depende do texto já vir concatenado).

## Passo 3 — Implementação

Se o Passo 2 não revelar impedimento sério: alterar o parser para produzir nós separados (`Text`/`Space`/`Text`), replicando a granularidade do vanilla. Isso pode exigir ajustes em qualquer lugar do cristalino que hoje itere sobre `Content::Text` esperando texto já concatenado (levantado no Passo 2).

## Passo 4 — Validação

1. `#repr([hello world])` mostrando a estrutura granular, batendo com o vanilla.
2. Confirmar que o texto renderizado (não só a estrutura interna) continua idêntico — essa mudança não deve alterar nada visível no PDF final, só a representação interna.
3. Testar documentos maiores/mais complexos para confirmar que a mudança de granularidade não introduziu problema de performance nem de layout (quebra de linha, por exemplo, que pode depender de como o texto é agrupado).
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-862-relatorio.md` com medição antes, código identificado, avaliação de impacto (Passo 2), diff, medição depois, contagem de testes.
