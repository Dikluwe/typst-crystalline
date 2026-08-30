# P1246 — medir e implementar clip masks SVG modeláveis

**Estado:** FECHADO — MATERIALIZADO, SEGREGADO E CERTIFICADO  
**Predecessor:** P1245  
**Saída:** proposta L0 e rascunhos diagnósticos; nenhuma promoção ou código.

O exporter atualmente recebe `clip_mask` em grupos e o ignora. Medir no vanilla
rect, ellipse e path como clips de grupos aninhados, com transform, fill-rule e
ordem. Distinguir clip geométrico de máscara alpha; esta última permanece fora
se o contrato não a modelar.

Atualizar L0 antes de código. Emitir `<clipPath>` determinístico apenas para o
subconjunto provado, sem imports reversos ou rasterização. Exigir referências
locais resolvidas, nesting, fill-rule, transforms, ataques de ID dangling e
regressão dos paint servers.

## Resultado saneado

Nenhuma implementação foi iniciada. O L0 SVG agora propõe o subconjunto
geométrico representável e preserva fill-rule não transportada e máscara alpha
como `Unknown`/`CONTRACT-GAP`. Foram materializados 12 critérios, 10 oráculos e
12 ataques como rascunhos diagnósticos, todos `NOT_EXECUTED`.

O antigo `3/9` foi removido: não era reproduzível e não constituía mutation
score. O auditor determinístico foi executado duas vezes com saída
byte-idêntica. O dono aprovou semanticamente o contrato em 2026-08-28. Naquele
ponto ainda não havia preseal, resselo de header ou autorização de código.

## Fechamento materializado

O protocolo completo separou contrato, oráculos, adversário, implementação e
veredito. O preseal rejeitou 12/12 mutações válidas (score 1.0). A implementação
L3 emite clips locais para Rect, RoundedRect, Ellipse e Path fechado não-zero;
preserva nesting e a mesma base de transform dos filhos. Geometria não
representável recebe fallback explícito; even-odd sem carrier e alpha mask
permanecem `Unknown`.

RED→GREEN: 4/4 testes P1246. Suíte SVG: 23/23. Build workspace, V15/V26 e
`git diff --check` passaram. O header do owner SVG foi ressellado e o
verificador independente publicou certificado `PASS`. P1246 não bloqueia mais
P1250.
