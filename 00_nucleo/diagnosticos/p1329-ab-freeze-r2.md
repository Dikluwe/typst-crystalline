# P1329 — freeze A/B r2 anterior a C

UTC: 2026-09-09T12:35:03.597Z. Autor /root/p1329_tests. A/B executado sem atestação técnica de isolamento; sem selo de refinamento. Sucede a interpretação de domínio no freeze r1 (dec98ea7e8b79a6b35412efe7ef1818e9e24325ed29caf29722172df32dba26a); todos os artefatos r1 permanecem imutáveis.

L0 calc.md R2 lido completamente pelo autor; norma SHA-256 07f83fc24dc13837f54a25f0bec6be20ff495e1f679c4975bec3f1fb583ef5ae, removendo somente Hash do Código. Manifesto p1329-manifest-r2.json lido e conferido, SHA-256 c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1. Nenhum runtime, patch C, recibo root ou evidência de domínio protegida foi lido; a medição de domínio foi recebida apenas através do L0.

## Domínio revisto

A obrigação local f64abs é idêntica à r1. NaN dimensional já construído no cristalino não corresponde a uma entrada dimensional vanilla equivalente: Scalar normaliza NaN para zero antes da construção. Conforme medição referida no L0 R2, a dívida anterior também é alcançável pela linguagem, em (calc.inf - calc.inf) * 1deg e * 1%; não é apenas sintética. O teste nativo NaN prova a regra local, não paridade. Inf permanece representável no vanilla, mas alegação de paridade exige entradas equivalentes já construídas. Comprimentos mantêm a condição abs zero OU em zero e a impressão não prova identidade de componentes. Nenhuma correção de construção/casts/operadores foi incorporada aos testes.

## Igualdade e sucessores

Comparação literal do sufixo JSON desde a propriedade expectations até EOF: igualdade byte a byte entre expected-r1 e expected-r2, incluindo as 252 entradas completas. Hash desse sufixo: 559eb63e8f3ca104b8fa20e11f3a76f014bb095f841556a347e29ab253c2ea00. Não houve alteração de caso, expressão, perfil, classificação, comparação ou expectativa. As transcrições integrais das dívidas de trace permanecem iguais.

Runner r2 foi comparado por igualdade exata com runner r1 após somente três substituições autorizadas: norma L0 no assert; referências ao manifesto-r2; referência ao expected-r2. Todo o catálogo, comparador e restante dos bytes são iguais. Expected r2 foi produzido do mesmo baseline r1 por --freeze-from, sem executar binários, com UTC/manifesto/norma R2 corretos. Os snippets e a medição baseline continuam com os mesmos hashes; não foi repetido corpus nem teste de runtime.

- p1329-ab-tests-r1.rs: 567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838
- p1329-ab-p1328-successor.rs: 1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c
- p1329-ab-cli-baseline-r1.json: b89049333f76a7f49b2828a6ced0fc0c7ba747f6b7ca9ea019fba5a27e0bd91d
- p1329-ab-cli.py: 392144197866dc2c9539256de1a657b8fa14a1ab61d17e3516ccb6f01a37b70a
- p1329-ab-cli-expected-r1.json: 699ab2fe050a1207a7ad9a98d8b4d78e7366fa421324f5d64aa06826c2289c51
- p1329-ab-freeze-r1.md: dec98ea7e8b79a6b35412efe7ef1818e9e24325ed29caf29722172df32dba26a
- p1329-ab-delta-r1.json: 6573ca57b9f0ea24abc49807f641b53489993bf4a616116fd3b87e80113917e7
- p1329-ab-cli-r2.py: ee0d6a5da2283d50b0591a24db58f93a8300091729b6d6f2ad48e3457b393ab2
- p1329-ab-cli-expected-r2.json: 9501ae018ada17565bfc54227c244bea9af99e3988c9a212b262e92a0ee7c239

## Proveniência, ordem e capacidades

A medição original permanece p1329-ab-cli-baseline-r1.json, HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado, com vínculo ao baseline público e binários pinados. Não há nova medição funcional nesta revisão: somente leitura normativa e comparações de igualdade dos artefatos identificados acima. A revisão acrescenta explicitação de domínio, sem ganho alegado em discriminação e sem novo selo.

Mantêm-se allowlist, proibições, budget, gates e limitações do freeze r1. Nenhum C foi lido/executado. Próxima sequência: root integra os mesmos snippets, RED dos filtros P1328/P1329, C, GREEN e CLI normal/repetida/reversa com p1329-ab-cli-r2.py. Unknown obrigatório bloqueia fechamento. Root conserva a responsabilidade por build/workspace/fmt/lint/V5/V15/V26.

Comando atualizado: python3 -B 00_nucleo/diagnosticos/p1329-ab-cli-r2.py --candidate CAMINHO --output 00_nucleo/diagnosticos/p1329-ab-cli-NOME.json --order normal|reverse. Gravação apply_patch via stdin no host, sem sobrescrever recibos.
