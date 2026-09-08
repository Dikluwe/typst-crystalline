# P1309 — rebaseline global pós-P1307/P1308

## Resultado observado

Veredito independente: **`P1309_PASS_P1310_COHORT_SELECTED`**. A auditoria R2
foi aceita e selecionou a correção dos diagnósticos de tipo de entrada de
`cbor/json/toml/xml/yaml`; não autorizou nem executou essa implementação.

O catálogo foi reenumerado e ampliado de 627 para **2182 probes**, mantendo
integralmente os objetos e IDs históricos. As três execuções completas produziram
os mesmos **8728 pares por ordem**, sem `EXECUTION_UNKNOWN`. Nenhum MATCH
histórico regrediu. Quinze paths fecharam nos quatro perfis: os doze de repr
P1305 e `json.encode`, `toml.encode`, `yaml.encode`.

Essas contagens são medições novas, não reaproveitamento da matriz focal P1308.
Valem para o corpus e os observáveis definidos abaixo, não para a linguagem
inteira, layout ou exportadores. A classificação causal, os ataques ao auditor
e o veredito independente são vinculados na seção final deste relatório.

## Estado medido e tentativa invalidada

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, branch `Tekt`, produto/L0
tracked e staged limpos. Os únicos untracked iniciais do dono eram o passo 1309.
Arquivos P1309 posteriores são evidências novas, fora das camadas produtivas.

A primeira tentativa foi invalidada: importlib no papel A criou um arquivo
`__pycache__/p1309-inventory-runner.cpython-312.pyc` fora do allowlist. O cache
novo foi movido para `/tmp`, recuperável, e sua ausência na origem verificada.
Nenhum produto/L0 mudou. O incidente está em `p1309-inventory-incident.json`;
a restauração não converte a tentativa anterior em válida.

Houve uma única retomada limpa, R2, com bytecode Python desativado e target de
build distinto. Os recibos iniciais continuam preservados, mas não contam como
gates R2. O problema de persistência de stdout extenso via argumento de shell
na primeira tentativa também foi corrigido para stdin antes dos gates R2.

Proveniência vigente:

- `p1309-baseline-r2.json`: `68dd669aba8d94ef80eca8a2bdf7f35ed27c60cf36f2a829ae03b1017d555bce`.
- `p1309-manifest-r2.json`: `d7d2f8ebfe7a5453c4fec7417248654a066d9859dd8d29abf259544c888cf9c3`.
- `p1309-build-r2.json`: `3553d0aa43ee277db4bbba9821f971466e6ad1283c952d0fe44359f7f9641a80`.
- Cristalino fresco: `/dev/shm/p1309-r2-target.R2ZoSj/release/typst`, SHA-256
  `e54dcc9bb6a4c45cdc710066027ea255af6fb13f17b3715583ba4e9f6f3287c7`.
- Vanilla ratificado: upstream `a51e02804`, `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Cada recibo registra argv, cwd, UTC, estado de fonte, saídas e hashes.
Não se usa `--version` como prova de identidade. O verificador independente
preliminar confirmou a admissibilidade da retomada e a preservação de 1033
arquivos de produto/L0, sem apagar a invalidade da primeira tentativa.

## Inventário e três matrizes

União estrutural fresca: 2172 paths. O catálogo contém os 627 probes históricos
e 1555 novas entradas, incluindo controles negativos explicitamente exigidos.
Neste catálogo principal, os 2182 probes correspondem a 2182 paths distintos;
isso não equivale a uma contagem de features. Nenhum ID histórico foi reciclado.

O autor A reenumerou scopes/namespaces e executou 8212 observações CLI de
kind/repr nos dois lados e perfis default/html, sem Unknown. Os campos estruturais
de parâmetros ausentes no enumerador cristalino não são chamados de metadata
verificada. O catálogo cobre a união, inclusive rotas bloqueadas por ancestors.
Detalhes, custos, fontes e limites: `p1309-inventory-author-receipt-r2.json`.

Catálogo: `p1309-probe-catalog.json`, SHA-256
`20a5bdec9a848d24ebe1fcf451f143ce65499184e57984e4e5389aef5aecde3a`.
Reconciliação: `p1309-catalog-reconciliation.json`, cópia dos dados selados
do autor A; preserva os 167 paths históricos e explicita added/removed/renamed/
split/merged/unchanged.

| Classe mecânica | Células por ordem |
|---|---:|
| MATCH_VALUE | 7948 |
| MATCH_DIAGNOSTIC | 248 |
| CRYSTALLINE_ONLY | 168 |
| VANILLA_ONLY | 324 |
| DIFFERENT_VALUE | 24 |
| DIFFERENT_DIAGNOSTIC | 16 |
| EXECUTION_UNKNOWN | 0 |
| Total | 8728 |

O runner exige JSON parseável no sucesso e preserva stderr inclusive quando
stdout coincide. Não remove warnings. A ordem é a submissão canônica de IDs/
perfis, com pares de processos independentes e concorrência limitada a oito;
a inversão inverte o corpus inteiro e a ordem dos lados. Os resultados são
recompostos por chave, não pelo instante de conclusão dos processos.

Pins:

- Normal: `ed6b603578ada1a3961bbb564c272652e52b4652231b5d324c8b9afa57889e51`.
- Repetição: `af86b508546934b60511502d3e3768bbe9449cd8c30c4cd573c81e9f2d35c51d`.
- Inversa: `26021665ea875e67f61be8d8f93bc97ecd9f80def748d58fa812da24720207bd`.
- Estabilidade: `3479b62231013a313493db0966109e3409d142dc0bb49d99107a73d0210df3df`.

## Métricas separadas

Igualdade bruta de células: `(7948 + 248) / 8728 = 8196 / 8728`,
**93,90467461044913%**. Denominador: todos os pares válidos de uma execução
completa; repetição e inversão não multiplicam o universo.

A igualdade ajustada exclui somente as 156 células de 39 extensões com
autorização normativa confirmada: `8196 / (8728 - 156) = 8196 / 8572`,
**95,61362575828278%**. Os três extras `calc.deg`, `calc.log10`, `calc.rad`
permanecem no denominador: não foi encontrada autorização L0 explícita e
eles não herdam a classe intencional somente porque o produto os aceita.

Cobertura dos **2182 paths do catálogo principal**, sem quadruplicar features:

| Classe semântica | Paths |
|---|---:|
| Fechados no observável medido | 2026 |
| Membros ausentes | 103 |
| Extensões intencionais documentadas | 39 |
| Intenção não resolvida | 3 |
| Feature esperadamente desligada | 1 |
| Repr divergente | 4 |
| Diagnóstico divergente | 4 |
| Valor divergente | 2 |

No subuniverso dos **167 paths históricos**, a reconciliação é
103 ausentes, 39 extensões, três não resolvidos, 21 fechados e um gated.
A diferença para a previsão 103/42/1/21 é inteiramente nominal: os três
extras calc acima perderam a classificação intencional por falta de fundamento
normativo. Não houve perda dos fechamentos funcionais previstos.

Os timestamps exatos de cada número estão nos recibos das matrizes e nos
artefatos de classificação. Todos usam o mesmo HEAD limpo descrito acima.

## Preservação não é igualdade bilateral

Suplemento separado: `p1309-sentinels.json`, SHA-256
`2af42cf65e4d59f0d199528a37a7bc7efb994d1c39bbda914b977a194618ce65`.
São 2170 pares fora do denominador principal. O contrato P1307/P1308 manteve
1982 resultados Preserved; os controles P1306 mantiveram 148. Arrays de
39/40/41/42/81/256 posições conservaram os dados integrais nos dois lados.
Os extras de módulos/arrays também foram repetidos nas três ordens, estáveis.

O suplemento bilateral observa 1630 MATCH_VALUE, 368 MATCH_DIAGNOSTIC,
120 DIFFERENT_DIAGNOSTIC, quatro VANILLA_ONLY e 48 DIFFERENT_VALUE. O oráculo
anterior preservava dívidas explícitas: portanto 1982 Preserved **não** significa
1982 igualdades vanilla. A projeção de linguagem R4 congelada é identificada
separadamente dos transcripts brutos, que permanecem disponíveis; os controles
de módulos/arrays comparam os canais brutos.

As 37 famílias de mutação de produto P1307 continuam num ledger próprio,
`p1309-certification-debt.json`. Os seis mutantes P1308 históricos não as
quitam. Nenhum mutante produtivo foi executado neste passo. A falta de ataque
não é falha funcional sem testemunha bilateral.

Recomenda-se um passo adversarial próprio para as 37 famílias pendentes, com
controles GREEN pinados e mutações causais compiláveis. Essa recomendação é
paralela ao próximo lote de paridade; não constitui execução nem quitação.

## Gates de produto

`p1309-gates.json`, SHA-256
`3bfa71d097287c8948e824987c6f60ff257c80d0994b66d228339043b18feb80`:

- Build workspace release e formatação: exit 0.
- Suíte workspace: 6620 passaram, zero falhas, três doctests ignorados.
- Linter: zero erros, 240 warnings e 1136 infos; mensagens preservadas.
- Produto/L0: 1033 arquivos byte-idênticos; tracked/staged limpos.

Os três ignorados são os doctests de `layout_with_introspector`,
`TagIntrospector::inject_pages` e `TagIntrospector::inject_positions`,
nomeados com localização exata no recibo da suíte. Não foram ocultados.

## Classificação e veredito

O classificador independente selecionou **`loader-data-source-cast`**:
diagnóstico de tipo inválido na entrada dos cinco decoders
`cbor`, `json`, `toml`, `xml`, `yaml`. Rank: `[3, 1, -5, 1, id]`.
São cinco paths com a mesma causa e um owner produtivo:
`01_core/src/compiler/stdlib/loading.rs`, helper `resolve_data`.

Testemunha concreta, no suplemento fresco `p1307.decoder.json.wrong-type`,
perfil default: `json(42)` dá no vanilla
`expected path, string, or bytes, found integer`, com âncora em `42`.
O cristalino dá `json() requer caminho (str) ou bytes, recebeu int`,
sem span primário resolvido, e acrescenta o trace da chamada. A coorte exige
o diagnóstico completo, não apenas trocar uma palavra ou traduzir a mensagem.

O próximo lote recomendado corrige mensagem e origem do erro de **tipo da
entrada**. Não inclui argumento ausente: esse caso exige a chamada inteira e
reabre também o contrato de `call_dispatch`, portanto é outra coorte. Também
não inclui parsing, I/O, encoders ou nova entidade. Gate provável: correção
interna de paridade em fluxo contínuo ADR-0127, sempre com L0 atualizado primeiro
e RED→GREEN próprios. P1309 não escreveu nem implementou o passo 1310.

Foram comparadas 114 coortes com prioridade, owners, paths, risco e motivo de
perda publicados integralmente em `p1309-cohort-table.md` e
`p1309-selection.json`. Diagnósticos de uma causa com menos paths, ou com mais
owners para resolver o observável completo, perdem antes do desempate lexical.
Coortes de valor/repr e membros ausentes ficam atrás da prioridade diagnóstica.
Não se agrupou todo HTML, todos os formatos ou todo math por conveniência.

O ledger tem 2182 linhas principais e 49 testemunhas suplementares divergentes,
sem misturar os denominadores. Pins finais do papel C:

- Owner ledger: `36deab222babd8edbf90b030a56977482d28c13c6b15a55b01f9ea546c7f23f4`.
- Transições: `8922aab7c85d3dfe69cf5914c5fff29b24a1ac8d82883e71b5949d171d55ce2b`.
- Seleção: `72e0505ebe6106dd458fa89555f0667c50a10eca8ef1135fd7008559d89aa808`.
- Resumo/denominadores: `0b5751799e50f07190322754582779b2ece69704cc79b28e394bc7459d543a1f`.

O adversário executou uma campanha com **18/18 ataques ao auditor rejeitados
com testemunha**, zero sobreviventes, 18 controles focais e um controle integral
aceitos. O score 1.0 pertence exclusivamente ao auditor; não quita as 37 famílias
de mutação do produto. Entradas permaneceram inalteradas. Recibo D:
`p1309-adversarial-ledger.json`, SHA-256
`1c693fffb797a0b49d69b8ac62e8de9f815aa68dc73b03c6920aaa0c97cf351d`.

Em `2026-09-07T23:37:41.893865Z`, E concluiu a verificação final com 300 checks
e zero falhas: reconstruiu independentemente os patches adversariais e seus
hashes, autenticou testemunhas/controles e conferiu os recibos anteriores,
baseline, binários, ownership, Núcleos e allowlist. Não reexecutou a matriz
para emitir o certificado. Resultado: `P1309_PASS_P1310_COHORT_SELECTED`.

- `p1309-verification.json`: `b981a87a4f261a260000ea23d3871ed9a062bab6bb3dfa413c97cb1d9c6a9199`.
- `p1309-certificate.json`: `ba40e1a5189efd2191dfad8d04841e1ddfad9a6cb6052ddf478efde8fe22c376`.

O certificado limita-se ao corpus pinado, à classificação e à seleção.
`CLOSED_CONFIRMED` no catálogo principal não certifica chamadas não medidas.
As três intenções normativas não resolvidas em `calc` não são falhas de execução
`EXECUTION_UNKNOWN`; permanecem explicitamente abertas.

A skill de materialização segregada determinou autorias distintas para
inventário (A), execução (B), classificação (C), ataques (D) e veredito (E).
Regime: **executado sem atestação de isolamento técnico**. Hashes identificam
artefatos; não provam isolamento de capacidades. Nenhum stage, commit, push,
mudança de L0/produto ou escrita do passo 1310 integra esta auditoria.
