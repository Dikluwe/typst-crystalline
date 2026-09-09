# P1324 — parecer independente anterior ao candidato

Revisor `/root/p1324_review`; A/B executado sem atestação técnica de isolamento.
Entradas: instruções locais, skill e suas duas referências, ADRs 0107/0108/
0127/0129/0130, L0 `compiler/eval/bindings/field_access.md` integral e seu
Núcleo, fontes do owner/Func/vanilla, apenas o cohort namespace-function-missing-field
de P1322 e fechamento/relatório P1323. Sem leitura de históricos de passos.
Escrita limitada a novos `00_nucleo/diagnosticos/p1324-review-*`; nenhum produto
ou artefato julgado foi corrigido. Ambiente e filesystem compartilhados; as
restrições são de autoridade declarada, não isolamento de capacidades.

Medição independente em `p1324-review-preflight.json`, iniciada em
2026-09-09T00:05:39.157Z sobre working tree não commitada de HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, com diff/stat e hashes. O recibo
preserva os bytes iniciais integrais do L0/owner e hashes dos seis arquivos
dirty herdados. Vanilla pinado upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
baseline P1323 SHA-256
`f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`.

As sondas novas reproduzem falta de nome público, aspas legadas e span total
em assert/json/yaml/toml/cbor/table; alias, With e posição de callee mantêm a
mesma causa. `type(json.encode)` funciona e `csv.encode` já coincide.
Closure nomeada permanece divergência distinta, cuja preservação não recebe
crédito de paridade. P1323 fecha somente warning HTML; não quita este cohort.

Fonte causal: `01_core/src/compiler/eval/bindings/field_access.rs:181` exclui
namespace Some do classificador de nativa; `:271` seleciona span;
`:420` emite o texto genérico de Some. `01_core/src/entities/func.rs:291,355`
já fornece nome e namespace através de With. No vanilla,
`lab/typst-original/crates/typst-library/src/foundations/func.rs:291` compõe
nome/field, e `lab/typst-original/crates/typst-eval/src/code.rs:347` entrega
field.span(). A existência de nome isolada não distingue Closure/Plugin/Element.

Classificação: correção interna de paridade diagnóstica em fluxo contínuo
ADR-0127; mensagem e origem são observáveis de língua ADR-0107/0108.
O owner único é suficiente por inferência apoiada nas fontes; necessidade de
novo carrier, API ou outro consumer causal refutaria a suficiência. Não é
necessário alterar Func, namespace, pipeline ou Núcleo.

Condição ex-ante: o L0 vigente P1311 protege explicitamente Some ausente e
seu span legado. O teste `p1311_existing_namespaces_including_empty_preserved`
codifica essa proteção. Antes de código, a obrigação P1324 precisa substituir
expressamente somente esse caso, incluindo Some vazio, NativeWithEngine e
With, preservando sucesso e categorias excluídas. Os bytes antigos estão no
snapshot independente; a sucessão do teste deverá ser explícita e congelada
pelo testador antes da implementação.

Veredito preliminar: seleção e owner/gate aprováveis sob essa sucessão L0;
implementação ainda não julgada. Gates finais exigem RED real, GREEN, A/B
congelado, preservação dos seis arquivos dirty e linhagem bilateral calculada
independentemente. `Nothing to fix` do linter sozinho não basta para o hash
recíproco, conforme limitação registrada em P1323 e algoritmo local inspecionado.
