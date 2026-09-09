# P1329-R2 — domínio corrigido e norma apta

Revisor `/root/p1329_review`, A/B sem atestação técnica de isolamento e sem
refinement seal. L0 completo relido; sem escrita em material verificado.

**A reabertura normativa foi resolvida.** R2 explicita entradas equivalentes
já construídas como domínio comparável, NaN dimensional como obrigação local,
possibilidade de construção pública divergente, Inf representável e proibição
de inferir componentes de Length por repr. As obrigações locais de módulo,
condição OR, origens e preservação não mudaram. Nenhum outro owner é necessário.

Entradas conferidas:

- norma R2 `07f83fc24dc13837f54a25f0bec6be20ff495e1f679c4975bec3f1fb583ef5ae`;
- manifesto R2 `c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1`;
- medição de domínio `8ca6f55fbcde70a3a4edcc521dccaa248caedd872650b6b735e113991ea84b9c`.

O manifesto aponta explicitamente para o predecessor e para a nova medição.
Não há motivo para descartar os testes r1 apenas por sucessão desses hashes;
seu significado deve ser reconhecido no freeze R2 antes da integração.

O diff real do sucessor unitário P1328 foi inspecionado: adição do import
Angle/Length/Ratio, passagem das expressões -2pt/-2deg/-2%/-2fr da tabela de
rejeições à de sucessos e nenhum outro delta. O novo snippet foi lido por
inteiro e contém os observáveis requeridos (nativos, origens, rotas, warnings,
guards, quatro perfis), sujeito a RED real. Não é certificado de poder
discriminatório completo nem equivalência geral.

Nota de gate: o runner r1 imprime a lista de falhas mas não devolve exit não
zero ao haver diferença de candidato. Portanto a conclusão deve verificar
`candidate_matches_frozen_policy == true` em todas as células; exit zero do
processo sozinho não comprova aceitação. O sucessor r2 deve preservar a mesma
leitura rigorosa ou tornar esse resultado bloqueante.

Pendente para liberar C: freeze R2, confirmação independente de células
históricas/expectativas e RED. O HOLD por insuficiência normativa está encerrado;
os gates de execução continuam obrigatórios.
