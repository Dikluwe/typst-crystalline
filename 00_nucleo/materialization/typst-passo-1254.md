# P1254 — pré-selar o tiling declarativo materializado no layout

**Estado:** FECHADO E SANEADO — MATERIALIZADO; REVALIDAÇÃO FOCAL PASS  
**Predecessor:** P1245 aprovado e L0 confirmado  
**Saída:** contrato, oráculos e ataques congeláveis antes de código.

## Intenção congelada

Materializar a decisão aprovada no P1245 sem introduzir um contrato público
`ResolvedTiling`: `Tiling` permanece entidade L1 declarativa com Content, size,
spacing, offset, angle e relative; a fase de layout existente produz uma
representação privada e cada target consome apenas a saída normal do pipeline.

## Ordem obrigatória

1. verificador independente ataca o pré-selo sem ler implementação candidata;
2. somente mutation score `1.0` sela contrato/oráculos/ataques;
3. criar o Prompt L0 proprietário do futuro consumer de layout junto ao caminho
   produtivo definitivo, preservando ADR-0129;
4. testes A/B RED derivados dos artefatos selados;
5. implementação mínima, GREEN e gates finais;
6. certificado limitado ao fragmento coberto.

Nenhum código, resselo ou novo consumer é autorizado enquanto este pré-selo não
for aceito. `Unknown` nunca promove paridade.

## Artefatos

- `00_nucleo/diagnosticos/p1254-tiling-contract.tsv`;
- `00_nucleo/diagnosticos/p1254-tiling-oracles.tsv`;
- `00_nucleo/diagnosticos/p1254-tiling-attacks.tsv`;
- `00_nucleo/diagnosticos/p1254-tekt-manifesto.tsv`.

Esta autoridade redigiu o candidato de contrato e não pode emitir seu selo,
implementar a solução ou proferir o veredito final.

## Verificação independente

O agente `/root/p1254_verificador`, sem histórico herdado, verificou as entradas
congeladas sem editar contrato, oráculos, ataques, L0 ou código. Cobertura
C01–C15 completa; 18/18 mutações válidas classificadas `Violated`;
`mutation_score=1.0`; positivo, opaco e determinismo lógico passaram.

O pré-selo abre a implementação somente contra os hashes exatos registrados em
`p1254-tekt-preseal.tsv`. Qualquer alteração invalida a cadeia. A segregação do
papel verificador foi processual; sandbox e autoria independente das suites não
têm atestação forte por causa do filesystem compartilhado.

## Certificação final

- C01–C15 cobertos; A01–A18 rejeitados; mutation score `1.0`;
- testes P1254: 5/5; testes públicos P1245: 3/3;
- join produtivo SVG: 1/1, do Content ao exporter;
- workspace build, lint completo, V5/V15/V26 e diff-check: PASS;
- PDF, HTML e stroke tiling: `Unknown` exato, sem promoção.

Certificado e resumo: `00_nucleo/diagnosticos/p1254-final-certificate.tsv` e
`p1254-final-summary.json`.

Receipts: `00_nucleo/diagnosticos/p1254-verifier-receipt.tsv` e
`00_nucleo/diagnosticos/p1254-tekt-preseal.tsv`.

## Saneamento e revalidação — 2026-08-28

A auditoria posterior encontrou uma contradição documental: os L0 de entidade
e stdlib ainda descreviam a confirmação do dono como pendente, apesar da
confirmação P1245 e da materialização P1254. O texto foi corrigido para estado
histórico e a linhagem dos dois consumers foi ressellada; nenhuma semântica
produtiva mudou.

Essa alteração de bytes invalida estritamente o pré-selo anterior. O seu
`mutation_score=1.0` permanece evidência histórica, mas não é apresentado como
novo selo. Contrato, oráculos e ataques ficaram byte-idênticos. A revalidação
atual foi executada sem atestação forte de isolamento.

Resultados atuais:

- P1254 core `5/5`, join produtivo SVG `1/1` e compatibilidade P1245 `3/3`;
- gate P1254 repetido byte a byte, SHA-256
  `5e492d8ab2304d2e1c1422fb0eb364790fea31665ca208ec13521271da484cbd`;
- build, fmt, diff-check e V1/V5/V15/V26 passaram; lint integral terminou com
  exit `0`, mantendo dívida global V16=211, V19=358 e V20=635;
- duas execuções integrais do workspace falharam somente no teste externo ao
  escopo `p1137_watch_dependencias_recuperacao_e_filtro`, por timeout de 20s;
  sua repetição isolada passou em 2.01s. Nenhum código foi alterado para ocultar
  essa intermitência.

O fragmento tiling permanece fechado. Stroke, PDF, HTML e fallback não coberto
continuam `Unknown`, sem promoção. Manifesto e certificado do saneamento:
`p1254-saneamento-manifest.tsv` e `p1254-saneamento-certificate.tsv`.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
