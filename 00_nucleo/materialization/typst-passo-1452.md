# Passo 1452 — recertificar as 66 rotas HTML/math após a convergência vertical

**Estado:** EXECUÇÃO AUTORIZADA — diagnóstico sem mudança de produto
**Data da medição:** 2026-09-18T20:35:40-03:00
**Commit medido:** `635e8b0a9338ee453fed168e416eb85b16b2718a`
**Árvore medida:** `0b1bc770df9f4f13031882e4a7c2e721cab2df46`, limpa
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

P1422 classificou as 66 rotas congeladas como `PARTIAL`: 44 HTML tinham a
forma qualificada ausente e alternativa equivalente; 22 math tinham a forma
qualificada ausente e alternativa divergente. P1423–P1424 materializaram as 44
rotas HTML e P1425–P1451 materializaram, repararam e certificaram as 22 rotas
math. `math.undershell`, fechada em P1451, era a última rota do denominador.

Logo, a próxima unidade não é outro constructor. É a recertificação integral do
mesmo denominador e do mesmo payload público P1420, agora contra o produto
convergido. As 66 rotas devem ser medidas novamente; resultados dos passos
verticais não podem ser somados por inferência.

## 2. Medição antes da decisão

O HEAD inicial está limpo. O binário release anterior estava obsoleto e ainda
rejeitava `math.undershell`; por isso foi reconstruído a partir do HEAD medido.
O binário reconstruído:

- tem SHA-256 `6e0f6e94d698f2e13bee63de37b4984649e454138a0d437bd1bf514279506eda`;
- reporta `typst 0.15.1 (635e8b0a)`;
- devolve `"undershell(body: [x])"` para
  `repr(math.undershell([x]))`.

O vanilla ratificado permanece `/usr/local/bin/typst`, revisão
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
O payload autoritativo continua `p1420-oracles.json`, SHA-256
`1d904d795515de5acf18b5eab544bb56c7edad9a1ee2e88efb4dca056a4ef1d4`:
66 rotas, 154 probes e 12 controles por ordem.

## 3. Classificação anti-deriva

Nome qualificado, assinatura, diagnóstico, repr, bytes/codepoints math e
morfologia HTML canônica são observáveis da linguagem (ADR-0107). Processo,
diretórios temporários e ordem física são mecânica, mas timeout, truncamento,
OOM, swap ou assimetria de transporte tornam a medição `Unknown` porque
impedem observar a linguagem. A hipótese de que as 66 rotas convergiram será
aceita somente se a campanha bilateral a medir; qualquer witness divergente a
refuta localmente.

## 4. Protocolo segregado

Aplicar o protocolo completo `tekt-materializacao-segregada`:

- dono da obrigação: este passo;
- contrato/oráculos: congelam o payload P1420, identidades e política de
  `Unknown` antes do runner candidato;
- adversário: preserva controles positivos, negativos e opacos e exige score
  `1.0` nas ordens direct, direct-repeat e reverse;
- implementador: cria somente o runner P1452 após o preseal;
- verificador: executa o candidato, produz resultados, relatório e certificado
  sem editar entradas seladas.

Os papéis compartilham sessão e filesystem. A alegação máxima é “executado sem
atestação de isolamento”.

## 5. Decisão e escopo

Criar um sucessor P1452 do runner P1422 que:

1. reutiliza byte a byte os 66 casos e 12 controles públicos P1420;
2. valida 154 probes por ordem, hashes, binários e árvore causal;
3. executa focal `html.area`/`math.abs` antes da campanha integral;
4. executa alternativas independentemente do resultado qualificado;
5. conserva limites de 512 MiB, 15 s, 64 KiB, serialização e
   `RAYON_NUM_THREADS=1`;
6. exige vetores idênticos em direct, direct-repeat, reverse e repetição
   integral independente;
7. fecha `Unknown` como `INCONCLUSIVE`, nunca como paridade;
8. exige `MATCH` nas 66 formas qualificadas para declarar o backlog P1422
   fechado; diferenças permanecem resultado conclusivo, mas não recebem
   crédito de convergência.

Produto, Prompt L0, Núcleos, fixtures e expected históricos são somente leitura.
Não há mudança de contrato público, comportamento padrão ou fase de pipeline;
portanto ADR-0127 não exige nova parada humana.

## 6. Gates e fecho

Executar validação sintática, gate discriminatório, focal, campanha integral,
repetição independente, `cargo fmt --all -- --check`, `crystalline-lint .` e
`git diff --check`. O passo fecha somente com hashes selados intactos, score de
mutação `1.0`, zero `Unknown`, vetores estáveis e classificação explícita das
66 rotas. A conclusão cobre apenas este denominador e estes observáveis; não
prova equivalência geral de HTML, math ou Typst.
