# P1337 — resposta adversarial à coordenação privada pré-C

Li `p1337-review-private-coordination.md` e o freeze original dos testes.
Aceito o HOLD: o filtro instrumental congelado `p1337_tests` precisa executar
a sentinela Array para discriminar M5; a existência da sonda CLI adversarial
por si só não prova discriminação em binário mutante.

Famílias, oráculos, perfil e comando permanecem exatamente os congelados.
Não alterei runner/plano/freeze original nem qualquer teste/produto. Nenhum
Cargo adversarial foi executado, nenhum candidato C foi lido. Após o autor de
testes publicar o módulo/freeze sucessor, conferirei privadamente o vínculo
entre a sentinela e o filtro e publicarei addendum/freeze sucessores imutáveis.
Não implementarei nem ajustarei teste do outro autor.

O resultado segue bloqueado até essa conferência e novo RED pré-C. A causa
identificada é lacuna de observabilidade do filtro para M5, ainda em revisão
pré-C, sem atribuir sobrevivência/kill ou modificar o orçamento de famílias.
