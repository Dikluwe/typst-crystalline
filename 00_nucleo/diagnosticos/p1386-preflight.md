# P1386 — preflight de materialização

Regime: protocolo completo **sem atestação de isolamento**. A mesma autoridade
possui contrato, testes, candidato, ataques e veredito; hashes comprovam ordem
e identidade, não independência.

Quatro medições bilaterais precederam a decisão. Todos têm máximo inicial
estrito e deixam a cauda curta antes da implementação. Uma sonda com métricas
sintéticas rejeitou os casos 20pt no gate P1380, mas a matriz produtiva final
provou que eles atravessam o gate com métricas reais e ficam em `MATCH`; o
observável produtivo prevalece.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidato → GREEN focal →
matriz bilateral → resselo/lint. As revisões de margem foram invalidadas pela
matriz release; a causa foi localizada no fechamento terminal de Cursor e
corrigida no owner correspondente. Processos sempre seriais nos limites do
passo.

Não são autorizados recursão, `Vec` auxiliar, reconstrução de `Content`, API
pública, bypass do gate de largura, igualdade, ordem crescente, primeiro não
maior que o exterior, terceiro fragmento ou delta além de size.
