# P1333 — testes congelados e integração

Revisão `/root/p1333_review`, A/B sem atestação de isolamento. Manifesto
`75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`.
Freeze completo conferido:
`bdbadb8d0c91dc9823c085489ae2c058f0a20e0d1db844ae1346176a316c1199`;
freeze nativo:
`337648c97f82afdd275570c8fa1eb344ce59fb9bb852e0d71dac3b7ec00e5967`.
Todos os hashes de artefatos listados no freeze correspondem aos arquivos.
Integração `b79ec811edd8f702477700ff3a5de24f68493dc219b7571c0f853f15bfa1b122`:
os seis snippets congelados estão presentes literalmente no owner calc.

O ledger restringe a migração histórica a content/fallback/misto/overflow
que deixam de ser mascarados pelos guards. Preserva ausência e primeiro
válido. A migração P1332 também ancora o boolean de With na origem original,
mantendo trace abs na chamada externa, consequência necessária da mudança.
Os testes novos cobrem famílias rejeitadas, sinais de misto, overflow,
extras/named e combinações, detached, valores válidos seguidos de erro e
avaliação eager. Helpers conferem campos integrais do diagnóstico, e os
casos públicos verificam origem/trace e warnings.

O runner CLI compara exit/stdout/stderr literais e congela as expectativas
antes de C. Somente casos nominalmente listados de primeiro inválido passam
a usar vanilla; os restantes preservam baseline, incluindo dívidas de
ausência, primeiro válido e resolução math. O freeze registra 712 células,
das quais 616 históricas; 40 históricas e 36 novas exigem a correção. Esses
números provêm do freeze e de seu baseline pinado, não de execução nova
deste revisor. A auditoria de integridade conferiu hashes; não reexecutou CLI.

Limitação explícita: o teste nativo novo injeta occurrences deliberadamente
incoerentes com items/named, inclusive valores e quantidades conflitantes.
São ensaios sintéticos de robustez fora do domínio causal válido de
`entities/args.md`; não provam paridade nem autorizam writers produtivos
stale. Casos públicos coerentes verificam a obrigação de linguagem sem
depender desses estados. A limitação não bloqueia materialização da
correção local especificada, mas não pode ser ocultada na conclusão.

Veredito: cobertura congelada apta para C após RED compilado confirmado.
Não há selo de refinamento, mutation score ou alegação de isolamento técnico.
