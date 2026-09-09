# P1322 — relatório C de classificação causal

**Estado:** recomendação para revisão independente D; não é veredito nem autorização de implementação. Regime: executado sem atestação técnica de isolamento. A skill `tekt-materializacao-segregada` separou o inventário A, os recibos bilaterais do coordenador, a classificação C e o julgamento D. C não alterou produto/L0, não executou mutantes de produto e não escreveu seu próprio veredito.

## Proveniência e universo

Estado produtivo: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` mais working tree não commitada integral em `p1322-baseline.json`; os quatro arquivos modificados eram loading.md/rs e call_dispatch.md/rs. Os artefatos JSON de classificação pinam o baseline, diff/stat, manifesto, catálogo e cada recibo. Build cristalino dedicado SHA-256 `756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`; vanilla ratificado upstream `a51e02804`, binário SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O catálogo novo tem 4718 probes: 2182 históricos preservados e 2536 modificadores finitos de Symbol adicionados por descoberta estrutural. Cada probe foi medido nos quatro perfis default/html/a11y/combinado, nas ordens normal/repetida/reversa. IDs novos não são regressões por não existirem em P1309. Igualdade de lookup/kind/repr não certifica chamadas ausentes do corpus.

## Medição bruta e decisão nova

| Classe de célula principal | Quantidade |
| --- | ---: |
| MATCH_VALUE | 17960 |
| MATCH_DIAGNOSTIC | 260 |
| DIFFERENT_VALUE | 40 |
| DIFFERENT_DIAGNOSTIC | 120 |
| VANILLA_ONLY | 324 |
| CRYSTALLINE_ONLY | 168 |
| EXECUTION_UNKNOWN | 0 |

Total 18872 células; 18220 coincidências brutas, **96,545% deste corpus de lookup**, não da linguagem inteira. As três execuções coincidem nos canais íntegros comparados. Ajuste separado exclui somente 39 extensões explicitamente documentadas no L0 (156 células): 18220/18716 = **97,350%**. Não exclui os três extras calc sem intenção encontrada nem as deprecações de variante deliberadamente omitidas.

Por probe: 4532 coincidências estritamente de lookup/repr; um binding html com gate esperado; 103 membros faltantes; 39 extensões documentadas; três intenções não resolvidas; dez diferenças de valor/repr; quatro diferenças diagnósticas não intencionais; 26 diferenças diagnósticas de variantes individualizadas no L0. São classes do corpus, não fechamento de funcionalidades.

Reconciliação dos 2182 IDs históricos: 8716 células preservam a classe bilateral; 12 fecham agora (`csv.encode`, `read.encode`, `xml.encode`, quatro perfis cada), pois ambos rejeitam o campo com mesmo diagnóstico. Não são encoders faltantes. As 10144 células novas correspondem aos 2536 novos paths. Nenhuma transição principal de MATCH para não-MATCH foi observada. O erro transitório 2231/2487 no resumo do primeiro rascunho C e a reemissão dos nove artefatos estão descritos sem apagar o incidente em `p1322-classification-correction.md` e `p1322-classification-freeze.md`.

## Reconciliação funcional pós-P1310–P1321

O suplemento tem 816 casos/3186 células fora do denominador principal. Conserva 1646 MATCH_VALUE, 1052 MATCH_DIAGNOSTIC, 424 DIFFERENT_DIAGNOSTIC, 40 DIFFERENT_VALUE, 20 VANILLA_ONLY e quatro CRYSTALLINE_ONLY. Unknown zero. A projeção histórica congelada distingue o payload de linguagem do envelope de panic/contexto: 53 casos têm linguagem projetada igual apesar de canal bruto diferente, sem normalização silenciosa dos recibos. Dez casos de CBOR medidos por esse envelope têm valor projetado diferente, não mero texto de erro.

`p1322-classification-functional-reconciliation.json` fornece IDs concretos e decisões delimitadas para cada passo. Revalida casts dos cinco decoders (P1310), None-native field errors (P1311), cast de read (P1312), Bytes/cast CSV (P1313), ocorrência e origem de opções (P1314), ordinal de registro (P1315), origem Bytes (P1316), causa UTF-8 (P1317), posição textual multiline/CRLF (P1318), arquivo de buffer inválido (P1319) e precedência/remaining/missing do CSV (P1321). P1320 é auditoria focal, não denominador global: suas lacunas abertas continuam explícitas.

Continuam distintos: Symbol source e delimiter; arquivo CSV UTF-8 válido sem excerpt externo; I/O com wrapper/path/origem divergentes; read/decoders/CBOR missing; named/excesso CBOR e nome de trace indireto; campos de namespace Some e closures; XML sem namespace; serialização CBOR de Symbol/Content; sink não terminal; chamada math direta de csv antes do dispatcher; array(Bytes); origem de string detached e de closure importada.

O transversal R2 corrige flags de perfil do adapter R1 e valida 20 casos × quatro perfis × três ordens, mais 36 contrastes internos. Rejeição CLI `query --features` é observação pública íntegra, não Unknown nem crash. `QueryArgs`/`QueryIntent` não transportam features (`cli.rs:313,411`), e o L0 vigente contrata esse transporte somente em Compile/Eval; futuro requer gate público e fio L4/L3, não remendo barato de mensagem. Os três diagnósticos de arquivos externos divergem só no spelling relativo/absoluto e coincidem em cópias internas: fronteira ambiental, não regressão comparável P1320; intenção de spelling não foi inferida. Diferenças raster/font-resource ficam separadas da língua, com geometria/texto preservados nos observáveis medidos. Captured-only compila com assertions iguais; closure-only resolve data.csv no local errado. Falha de import em eval e origem de closure não foram fundidas por conveniência.

## Única recomendação P1323

`namespace-function-missing-field`, prioridade 3, ranking `[3,1,-6,1,id]`, owner produtivo único `01_core/src/compiler/eval/bindings/field_access.rs`, L0 único `00_nucleo/prompts/compiler/eval/bindings/field_access.md`. Paths efetivos: assert, json, yaml, toml, cbor e table; json.with é controle do mesmo path.

Medição precedente: `p1311.assert`, `p1311.json-missing`, `p1311.yaml-missing`, `p1311.toml-missing`, `p1311.cbor-missing`, `p1311.table-missing` e `p1311.json-with-missing` no recibo P1322. Fonte: field_access.rs:181/271/420 restringe nome/span à categoria None e emite erro genérico para Some. Func::name/namespace em entities/func.rs:291/355 já preserva identidade e With; vanilla func.rs:291 e eval/code.rs:347 fundamentam nome e field span. L0 field_access.md:170 preserva explicitamente Some fora do recorte P1311: portanto prioridade 3, não contradição L0 nem regressão P1311.

Hipótese: esses carriers bastam no owner para erro completo de campo ausente em Native/NativeWithEngine com namespace Some. Refutação: nome público irrecuperável, origem perdida antes deste owner ou necessidade de outro consumer. Futuro L0 deve alterar explicitamente só a preservação Some e proteger lookup presente, None, Closure/Plugin/Element, Module/Dict/Type/Content/Float. Gate proposto: ADR-0127 correção interna de paridade, L0-first + RED→GREEN + revalidação; nenhuma escrita de código está autorizada por esta seleção.

A seleção compara 125 coortes atuais, 16 elegíveis sob owners/risco demonstrados. No desempate, seis paths superam os quatro do warning math.join e os três de span não-Module. CBOR indireto não conserva o owner único herdado: o nome qualificado vem de eval/mod.rs:1836 e aparece no trace de call_dispatch.rs:1648. Causas ainda sem conjunto completo de owners ou risco demonstrado são inelegíveis, com Unknown explícito. Os 103 faltantes não foram transformados em reparos baratos por compartilharem o registro.

## Certificação e limites

`p1322-classification-certification-debt.json` mantém nominalmente F01–F20, S01–S12 e A01–A05 pendentes: 37 itens sem recibo de descarga localizado nos encerramentos inspecionados P1310–P1321. M01–M06 de P1308 não descarregam esses nomes. Isto não afirma que jamais houve execução fora do histórico inspecionado. Sem mutation score de produto; os ataques ao classificador são competência D e não quitam essa dívida.

Artefatos julgados e seus hashes estão em `p1322-classification-freeze.md`, conferidos às 2026-09-08T22:43:05Z. O relatório não se aprova: a validade das contagens, fontes, fronteiras, seleção e do incidente de revisão depende do veredito independente D contra esse congelamento.
