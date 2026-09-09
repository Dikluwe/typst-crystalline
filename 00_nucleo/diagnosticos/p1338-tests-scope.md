# P1338 — testes independentes do erro de field em Array

Autor `/root/p1337_tests`: contexto de autoria P1337 retido, conforme manifesto.
Regime A/B sem atestação técnica de isolamento e sem selo de refinamento.
Manifesto efetivo R1 SHA-256
`14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843`.
R0 permanece histórico; R1 corrige apenas uma referência canônica L0 e o
header causal. A obrigação Array não mudou. Nenhum C P1338 foi lido ou existia
durante a redação/integração/freeze destes testes.

Skill de materialização segregada e ambas as referências, CLAUDE raiz/L1 e
ADRs 0107/0108/0127/0129/0130 lidas; L0 proprietário
`compiler/eval/bindings/field_access.md` lido integralmente. Fontes baseline
foram consultadas somente para integrar helpers e os tipos usados nos testes.
Helpers privados foram copiados para o módulo p1338_tests no mesmo owner.

O módulo verifica diretamente a mensagem e o span recebido pelo lookup puro
para arrays vazios, singleton, heterogêneos e aninhados; detached, span vazio
e spans deslocados; nomes Unicode, longos, vazios e próximos a campos válidos.
Os acessos AST independentes verificam exatamente os bytes do identificador
em aliases, parênteses, Unicode, repetição lexical e multilinha. Cardinalidade,
severidade, hints, trace e warnings são comparados pelos helpers, sem normalizar.

As preservações locais incluem len/first/last do lookup puro, None quando
vazio, e o pré-despacho AST existente de first/last vazio: erro detached, sem
confundi-lo com o lookup puro. Métodos como valor e chamadas reais ligadas,
estáticas e por alias permanecem como baseline. Length e Type::Array preservam
mensagem e span total; Bool/None/Auto e Int/Str conservam mensagens/spans
contratados. Todos esses testes estão sob o filtro adversarial p1338_tests.

A única migração anterior é nome/âncora/mensagem de
p1337_preserve_array_ast_message_and_total_span_debt para sucessor P1338,
preservando o caso, helper/comparadores e comentário histórico literalmente.
Rustfmt apenas dobra a chamada check_ast no novo comprimento. Nenhum outro
teste anterior, metadata/header ou corpo produtivo foi editado pelo autor.
A prova prefreeze reconstrói todos os bytes da fonte a partir do manifesto R1.

Corpus CLI: 45 expressões, sendo 10 convergências e 35 controles. Quatro
perfis default/html/a11y/html+a11y e ordens normal/repeat/reverse, nos binários
vanilla ratificado a51e02804 e baseline P1337 identificados pelo manifesto.
Os controles são classificados antes de C em paridade ou dívida existente,
sem alterar sua política em função do futuro resultado candidato. Comparação
integral de exit/stdout/stderr, texto UTF-8 mais base64, sem normalização.
Transporte inválido, crash, JSON não interpretável ou ausência de observação
gera Unknown bloqueante. Controles opacos deliberados só testam o harness;
Unknown esperado nesses controles não é crédito de produto.

Limites: len/first/last como valor, pré-despacho vazio, Type Array, Length e
ordem de panic são fronteiras ou dívidas preservadas, não convergência P1338.
Dict, Content raw, Module, nativas Some/None, closure/With, Float/is-nan têm
sentinelas CLI. Content.text conserva disponibilidade baseline. LocatedContent
não é criado pela CLI eval: somente cobertura antecedente protegida a executar
no gate Cargo do operador, sem crédito novo de introspecção independente.
PDF trio mede feature/kind/repr/hints completos, não exporta PDF. O controle
text.size/sym.integral.cont observa a fronteira atual, sem provar toda a
contextualização ou geração de warnings. Não se afirma paridade geral Array,
fields, PDF, acessibilidade ou compilador.

O runner P1337 foi reutilizado mecanicamente em arquivo P1338 novo, com pins
R1 e contexto retido explícitos. Arquivos P1337 não foram reescritos. Todos
os artefatos finais de entrada/saída são congelados por hash e as saídas são
criadas com apply_patch, via stdin para preservar recibos grandes sem ARG_MAX.
O autor não executa Cargo funcional: o operador coordena RED focal p1338_
e os gates posteriores; cargo fmt check é somente validação de formatação.
