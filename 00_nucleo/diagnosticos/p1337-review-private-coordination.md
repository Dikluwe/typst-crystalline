# P1337 — coordenação privada pré-C dos autores independentes

Destinatários: autores de testes e ataques; o implementador não deve ler este
conteúdo antes de C. Revisor não edita entradas julgadas. Não modificar Rust
enquanto o RED atual estiver executando: preservar origem daquele recibo.

Achado: a família M5 altera exclusivamente a âncora AST de Array. O runner
instrumental executa somente `p1337_tests`; o módulo congelado contém teste puro
Bool/None/Auto, teste AST dessas categorias e positivos. Não contém testemunha
Array, e o runner de mutação não roda a CLI mutante. Assim, o orçamento de cinco
famílias inclui uma mutação sem teste discriminante realmente executado.

Solicitação ao autor independente de testes: após RED atual finalizar, derivar
do L0 a sentinela local de preservação Array no módulo P1337 (incluindo mensagem
e span integral baseline; não lhe atribuir crédito de paridade). Não mudar os
três testes existentes, oráculos CLI, casos, expectativas, norma ou produto.
O revisor não fornece implementação do teste. Publique sucessor do módulo,
patch, freeze e recibo de sucessão; mantenha todos os artefatos originais intactos.
Aplique somente a adição local depois da liberação operacional do root.

Solicitação ao adversário: conferir privadamente que a testemunha sucessora
está realmente sob o filtro congelado. As cinco famílias, seus oráculos e o
perfil instrumental podem permanecer iguais; se necessário, publique addendum
e freeze sucessores vinculados ao novo módulo sem sobrescrever originais.
Não contar sobrevivência de M5 por ausência de teste como sucesso. Falha de
compilação/cache continua Unknown, não kill.

Novo freeze e novo RED com assertions são necessários antes do GO de C. O RED
original permanece válido como histórico dos testes então presentes. O único
objetivo da reabertura é executar a testemunha da fronteira já exigida pelo L0
e pela família pré-C; não ampliar escopo ou adaptar ao candidato inexistente.

Perfil `--release --config profile.release.package.typst-core.opt-level=0`,
jobs=2, aceito somente para discriminação instrumental com C pareado. Release
normal build/workspace permanece gate distinto obrigatório. A/B sem atestação
técnica de isolamento ou selo de refinamento; contexto retido do revisor está
explicitado no manifesto R1.
