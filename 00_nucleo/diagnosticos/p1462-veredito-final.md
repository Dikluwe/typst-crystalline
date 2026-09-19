# P1462 — veredito final de `format.svg.pretty`

**Veredito:** CERTIFICADO NO FRAGMENTO P1462 — nenhuma refutação bloqueante

**Papel:** verificador/certificador final, distinto do implementador, do autor
do teste A/B e do adversário final

**Atestação:** executado sem atestacao de isolamento

## 1. Escopo e estado certificado

A certificação foi decidida em `2026-09-19T17:58:16-03:00` sobre o commit
`7c909c97dc69cc910093467aa14fd86c36353450`, tree
`3cc6c07b02cd8040ec00208214ea22eff2b72f0f`. Antes de criar estes dois
artefatos finais, `git status --porcelain=v1` estava vazio.

O regime é o protocolo completo de materialização Tekt, retomado por
checkpoints segundo ADR-0131. O checkout e o filesystem são compartilhados;
portanto a separação comprovada é de papéis, entradas, ordem causal e
artefatos, não isolamento técnico.

Este veredito cobre exclusivamente o fragmento congelado P1462: identidade e
set rule de `format.svg`, opção documental `pretty`, fronteira root/container,
transporte no documento paginado, precedência `caller explícito > documento >
false`, forma compacta/pretty do SVG completo, neutralidade dos targets
declarados e ownership Tekt correspondente. Não declara paridade global de
`format.*`, SVG, Typst ou dos exporters.

## 2. Cadeia causal auditada

| Marco | Commit / SHA-256 |
|---|---|
| baseline da campanha R2 | `61796fa6e183c08207e3828cec6d2e43da1af846` |
| ADR-0131 | commit `68542c0c0`; arquivo `2ba422df0147a6f6469e869d91aa07e3ddac102432554c800da9d03e65fad3cc` |
| contrato R2 | `69cfdc0de39564cf542e2ca7fc99bcb7b80752d10bc8cdce3c809c1d9cf02fa2` |
| recovery3 checkpoint | `ab771563f013604dc7d0b24ca6f7e35ac0e4714d486265a8c694c3252ddb61ac` |
| recovery3 preseal | `789b1f1c7e2a2c407edb36041588269844df2b88ff667aa6332ced5016d1a08c` |
| commit do selo | `c5bef1306c8c0eaaf3e5b49e1d98ca7d5fd86d97` |
| recibo RED | `3e7191029cdf07a7d1e77bf2f843cf21a77be9885b7e19c91993a9cae64696d3` |
| teste congelado | commit `6a185d6c5`; arquivo atual `f48c24a73861affc40bd7668e84a5d1b1e540e7fc0a1f35365c992197c284000` |
| candidato produtivo | commit `3a997a3cd303bfe7e225a926253ecea2e633b883` |
| ataques finais | commit `7c909c97d`; Markdown `4d2fba0483a411ac7b2569f9df4f4a08d1a5e8d0b5a93e888be15a11923c65cd`; JSON `697c48e27bd934e0a2c197187ac2c108ad5f04cb7ace2c26b4b86169fd37611d` |

O teste A/B nasceu RED antes do candidato: 7 casos, 2 controles preservados e
5 falhas esperadas, sem timeout ou `Unknown`. A mudança posterior do hash
integral do teste limita-se às duas linhas mecânicas de linhagem; o corpo a
partir da linha 3 conserva SHA-256
`7722b12d6a3b4ed59540fa26ef24a4ebb9a2efd82141d941b60089a520c4c1f3`.
O adversário reconstruiu o hash RED original `e1c95a...` a partir desse corpo.

Os dez Prompts L0 tiveram mudança pós-selo somente em `Hash do Código`, como
parte obrigatória do resselo de linhagem. `git diff c5bef1306..3a997a3cd`
mostra exclusivamente a linha 2 em cada Prompt; nenhuma obrigação, expected,
Núcleo ou corpo semântico L0 mudou. Isso é registrado explicitamente para não
confundir o binding mecânico esperado ao consumer com deriva da intenção
protegida.

## 3. Gate discriminatório e repetição por risco

O recovery3 recomputou dos recibos pinados o vetor completo:

- P00: `Preserved`;
- M01–M18: `Violated`, 18/18;
- `mutation_score = 1.0`;
- `Unknown` semântico: 0;
- a execução parcial anterior de M12 é `Interrupted`, sem crédito e sem ser
  reclassificada como `Unknown`.

O adversário final repetiu no candidato as fronteiras exigidas por ADR-0131:
P00, M03, M12, M13, nested/no-artifact, vazio neutro, ambas as ordens,
preservação de `Option<bool>` e XML/morfologia. O teste mantido passou 7/7,
os testes L1 passaram 7/7 e os testes L3 passaram 6/6; nenhum ataque produziu
refutação ou `Unknown` semântico. Os binários mutantes congelados não foram
executados novamente, logo não se inventa um segundo mutation score.

## 4. Gates finais reproduzidos pelo certificador

Todas as execuções abaixo ocorreram no HEAD e tree indicados, com working tree
limpa antes da criação deste veredito.

| Comando | Resultado |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `git diff --check` | exit 0, sem saída |
| `cargo build` | exit 0; build concluído em 2.82 s |
| `crystalline-lint .` | exit 0; **0 violations**; warnings/info não bloqueantes permanecem no repositório |
| `crystalline-lint --checks v15,v26 --fail-on warning .` | exit 0; `No violations found` |
| `cargo test -p typst-wiring --test p1462_format_svg -- --nocapture` | 7 passed, 0 failed; 37.70 s |
| `cargo test -p typst-core p1462_ --lib -- --nocapture` | 7 passed, 0 failed; 5928 filtrados |
| `cargo test -p typst-infra p1462_ --lib -- --nocapture` | 6 passed, 0 failed; 959 filtrados |

O binário `target/debug/typst` após a última execução mantida tinha SHA-256
`2fe1e2f8cd47e495e85ac1d6a8bfa3dbc570660a71e86148c55ad880866fd996`.
O linter ainda relata warnings e infos gerais, inclusive V16 no `match` que
delimita a travessia root do novo coletor e V20 no harness. Eles não são
violações; a passagem exaustiva separada, os dois containers negativos e V15/
V26 verdes impedem que esse warning receba crédito semântico indevido.

## 5. Veredito por obrigação

| Obrigação | Veredito | Evidência final |
|---|---|---|
| O1 — identidade/default/perfis | **Preserved** | teste mantido nos quatro perfis, incluindo `std` e alias |
| O2 — construção manual proibida | **Preserved** | diagnóstico exato no teste mantido e no teste L3 |
| O3 — domínio/validação/vazio neutro | **Preserved** | erros normais, booleanos aceitos e `true → vazio` preservado |
| O4 — root/container/no-artifact | **Preserved** | root aceito; `block` e `box` rejeitados com mensagem exata e sem artefato |
| O5 — ordem lexical | **Preserved** | ambas as ordens e vazio transparente passaram |
| O6 — precedência SVG | **Preserved** | matriz `Some(bool) > documento > false` passou externamente e em L3 |
| O7 — XML/morfologia | **Preserved** | compacto e pretty válidos, distintos e canonicamente iguais para a mesma fonte |
| O8 — neutralidade de targets | **Preserved** | PDF, PNG, HTML e `html.frame` invariantes no teste A/B |
| O9 — escopo/inventário | **Preserved** | siblings ausentes; overlay canônico mantém raw 43 e normalizado 42; P1357 integral permanece pinado no selo |
| O10 — carriers/fail-closed | **Preserved** | defaults/clone/equality, valor interno não bool e relayout exercitados em L1/L3 |

## 6. Suíte completa e falhas históricas

O integrador executou `cargo test --workspace` sobre o mesmo produto e
registrou `5927 passed; 8 failed` em `typst-core`. As oito falhas são a classe
`p1292_c_v7_math_vec`, quatro casos `p981`/`p1132r` de delimitadores,
`cases_nao_panica_com_align_point` e dois casos HTML `p1423`/`p1168` — o
conjunto histórico P1450/P1451 já declarado no contrato.

O certificador não repetiu essa suíte integral: auditou o resultado comunicado
e verificou que o diff P1462 desde `c5bef1306` não toca nenhum dos arquivos que
contêm esses casos (`eval/repr.rs`, `eval/tests.rs`, `layout/tests.rs`,
`integration_tests.rs`, `export/html.rs`, `stdlib/html*` e `wiring/tests/cli.rs`).
Além disso, o caso O8 final prova que a nova opção não altera HTML nem
`html.frame`. Assim, as oito falhas continuam reais e não recebem crédito
verde, mas não constituem regressão causada pelo recorte P1462.

## 7. Observações e limite da conclusão

Não foi encontrado achado funcional ou arquitetural bloqueante. Duas
observações não bloqueantes ficam explícitas:

1. alguns Prompts novos ainda descrevem no campo narrativo `Estado` a fase
   pré-consumer/sentinela, embora o hash de código, V15 e o consumer já estejam
   selados; isso é texto de estado obsoleto, não divergência da obrigação;
2. o inventário P1357 integral não foi recalculado depois do candidato; sua
   evidência é o recibo recovery3 pinado. No candidato foram repetidas as
   precondições públicas fail-closed, a ausência dos siblings e a aritmética
   canônica do overlay.

Conclusão proporcional: o candidato no commit `3a997a3cd` satisfaz O1–O10 do
contrato R2, conserva o vetor discriminatório P00/M01–M18 e passa os gates
funcionais e arquiteturais finais. P1462 está certificado somente no fragmento
descrito acima, **executado sem atestacao de isolamento**.
