# P1339 — precondição explícita de orçamento

Verificador `/root/p1311_review`, 2026-09-10; executado sem atestação de
isolamento. Manifesto r2 SHA-256
`842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
Sem alteração de entradas julgadas. Sem selo ou autorização de novas
calibrações/matrizes enquanto esta precondição não for resolvida.

## Medição e autoridade

O passo autorizado `00_nucleo/materialization/typst-passo-1339.md:305`
diz **três revisões do contrato/oráculos**, conjuntamente; não oferece três
revisões adicionais por arquivo, autor ou rótulo draft. O contrato r3
`c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`
registra três revisões publicadas, e `revision_budget_rule` exige redesenho
explícito diante de nova insuficiência. R1 e r2 continuam preservados.

Comparação literal dos planos de oráculos disponíveis em 2026-09-10:

- CLI r2, `bb1a716d7071f7a8af6379d0a647b05fb17298e8322267e74d90d6c1e80cf5bf`:
  718 casos.
- CLI r3, `1b120451ce26c22bcaecda783504155a1c7e2b3bdee826f8893a1c44c3357ba7`:
  mantém todos os anteriores, acrescenta `opaque-budget-while` e
  `opaque-budget-finite`, total 720.
- CLI r4, `daf82ba90cae9891de1ade5abd1d4aa569f578e5fa22c255957038c1148463c8`:
  troca `opaque-budget-while` por `opaque-budget-workload`; demais casos
  idênticos, total 720.

Os focais de opacidade r1/r2, com respectivos 16 processos e sem sucesso
bilateral, estão auditados em `p1339-verifier-opacity-design-review-r1.md`.
O relatório de achados CLI r2 contém duas insuficiências de tradução de
escopo ainda presentes nesses planos. Não são alterações do contrato r3,
mas são publicações/tentativas reais de calibração de oráculos e não podem
ser ocultadas como custo zero. O ledger dos autores deve registrar também
as demais publicações de bridges, transportes, fixtures e mutantes, separando
criação inicial, revisão, simples correção mecânica e execução, com causa,
predecessor e custo efetivo. Este inventário parcial não afirma ser toda a
cronologia nem inventa um número exato agregado de revisões.

## Decisão e proposta à coordenação

O orçamento inicial está ao menos esgotado pelos três contratos. Nenhuma
interpretação de `draft` ou correção de tradução autoriza automaticamente
uma tentativa adicional. A aprovação focal do desenho assimétrico de
opacidade não dispensa esta precondição de orçamento.

Propõe-se um suplemento de protocolo explicitamente autorizado e pinado
antes da próxima execução, sem modificar contrato, L0 ou evidência anterior:

1. Ledger completo das publicações e execuções até a decisão; custos e falhas
   preservados, sem reiniciar contadores nem alegar ganho não medido.
2. No máximo **dois lotes focais adicionais** para fechar conjuntamente a
   lista de pendências já identificadas: políticas CLI, controle opaco
   assimétrico, inventário exaustivo e definições F06/F07/F08 (e respectivos
   mapas de cláusulas F01–F10). Não é licença para alterar a intenção r3.
3. Cada lote congela suas alterações antes do recorte; medir só as causas
   afetadas e controles de fronteira. Falta de ganho na mesma causa ou nova
   insuficiência além desse escopo provoca nova parada, não terceiro lote.
4. Permanecem todas as obrigações, os 20 mutantes válidos, M12 estrutural,
   M20 raw Unknown, opacidade real, preservações, repetição e reordenação.
   Nenhuma mudança de predicado para aceitar candidato.
5. O limite de duas matrizes completas preseal permanece; o verificador
   executou zero. O ledger deve confirmar também as execuções dos demais
   papéis antes de aprovar a primeira matriz. Focais não viram retroativamente
   matrizes completas e matrizes completas não são rebatizadas de focais.

A coordenação deve registrar se possui autoridade para esse ajuste de
protocolo sob a autorização vigente do usuário; caso não possua, a decisão
deve ser solicitada ao usuário. O verificador não concede a si próprio novo
budget nem reduz gates. A avaliação é de insuficiência de protocolo/custo,
não de falha de implementação (ainda inexistente).
