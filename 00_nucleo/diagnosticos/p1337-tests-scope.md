# P1337 — escopo independente de testes A/B

Autor: `/root/p1337_tests`, contexto inicial limitado. Regime A/B executado sem
atestação técnica de isolamento; nenhum selo de refinamento. Manifesto efetivo
R1 SHA-256 `7c09595cde021b89fffa7c316b7706797ba3898aa5b6687cdfaf2deacb0e65ff`.
O manifesto R0 foi recebido primeiro e permanece histórico; R1 altera somente
identidade/contexto do revisor. Nenhuma implementação candidata foi lida.

Entradas: L0 integral, instruções do repositório e skill de materialização
segregada com suas duas referências; fonte baseline somente para integração
dos testes; mecânica pública do runner P1336 reutilizada em arquivo novo.
O novo módulo copia helpers privados dentro do mesmo owner sem nova API.
A migração legada muda exclusivamente três tuplas Bool/None/Auto e renomeia
honestamente o sucessor. A reversão mecânica desse patch recompõe exatamente
todos os bytes da fonte pré-candidata registrados no manifesto.

Obrigação: Bool true/false, None e Auto ausentes que chegam ao lookup; nomes
boolean/none/auto; span puro recebido, inclusive detached e vazio; AST somente
field, com Unicode, aliases, repetição lexical, parênteses e multilinha.
Diagnósticos locais com cardinalidade, severidade, hints, traces e warnings
explicitamente comparados. Valores válidos, type/repr e chamadas comuns têm
controles positivos independentes.

O corpus CLI contém 45 expressões: 12 convergências intencionais e 33 controles
de preservação. Cada expressão roda contra vanilla ratificado a51e02804 e
baseline P1336 nos perfis default/html/a11y/html+a11y em ordem normal,
repetida e inversa. Antes de C, os bytes completos classificam cada controle
como paridade preservada ou dívida preservada, sem escolher retrospectivamente
o que o candidato deve fazer. A política de convergência exige ambos os produtos
em erro e divergência baseline real; não usa normalização de diagnóstico.

Comparações preservam exit, stdout e stderr completos em UTF-8 e base64; falha
de transporte, crash, JSON inválido, incompatibilidade de canais ou observação
não classificada produzem Unknown bloqueante. Controles opacos deliberados do
harness esperam Unknown e não contam como caso do produto. A repetição e a ordem
inversa devem produzir exatamente a mesma assinatura em cada produto/perfil.

Int/Str P1336 conservam testes originais, acrescidos de controles CLI de erro,
campo de método e chamada válida. Array e valores-tipo (Bool/Int) são dívidas
de fronteira, não convergência P1337. Os três callees e os três argumentos panic
conservam a saída baseline, pois a seleção de método/ordem anterior ao lookup
não pertence a esta correção. Todos os controles são classificados antes de C.

Dict, Content raw e disponibilidade residual de text, Module, Float/is-nan,
nativas Some/None, closure e With possuem sentinelas de preservação CLI; o trio
PDF cobre disponibilidade por feature, nomes e hints completos, mas não exporta
PDF. Text.size e sym.integral.cont são controles de fronteira de contexto e
warnings; a igualdade integral preserva o comportamento observado, mas não prova
todos os estilos contextuais nem todo o catálogo de warnings. LocatedContent
não é construído pela CLI eval deste corpus: a proteção é a permanência byte a
byte dos testes locais antecedentes, a executar pelo operador, sem crédito
independente de nova cobertura de introspecção. Não se afirma paridade geral
de fields, layout, PDF, acessibilidade, plugin/Element ou compilador.

Freeze: runner, casos, módulo novo, patch/migração e este escopo protegidos por
hash; outputs baseline e expectativas imutáveis criados via apply_patch.
O executor de testes não executa Cargo antes da coordenação do operador.
Após C, somente binário e recibos são entradas; fonte candidata não será lida.
