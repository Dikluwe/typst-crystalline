# P1307-R5 — entrega do contrato ampliado

O L0 está pronto para aprovação da API pública. A revisão independente
terminou em `READY_FOR_PUBLIC_API_REVIEW`, sem bloqueios documentais abertos
no recorte. Isso **não** significa que o 1307 foi implementado.

A proposta e as dívidas delimitadas estão em `p1307-r5-contract-report.md`
(SHA-256 `4c2e8000c3ac179486a67fcd8b7e5ac186092288b9af890aa6e967523f3985ed`).
Ela preserva os campos de Heading consultado desde o walk puro até a CLI e
os encoders, mantendo Content de render separado. A aprovação abrange o
carrier IntrospectedContent, a variante LocatedContent, o store público,
Vec<Value>/&[Value] na query CLI e os efeitos delimitados em fields, repr,
igualdade e fallback CBOR. Não inclui novos callbacks ou fases.

## Bytes revisados e validação

- Freeze final: `p1307-r5-l0-freeze-v5.json`, SHA-256
  `f51e45a61b388a498a0dd58465bc98d163d679561c8da28b5946f3d5956ca13f`.
- Revisão independente: `p1307-r5-review.json`, SHA-256
  `bce1842bbf00dfeaa5a8f45419a2f66f8c28fee0fba8de51e1711a7cf2b5cff5`.
- Gates finais completos: `p1307-r5-final-gates.json`, SHA-256
  `d7ed5dd8e2b0e67912fcf6b55f8cecd5fe390f03001e794673a010f49d868c4f`.

Proveniência: HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working
tree não commitado; diff/stat integrais e horários em cada recibo. V15/V26
e `git diff --check` passaram independentemente. `crystalline-lint .`
também terminou com exit 0, mas **não** com zero violations: há V5 esperado
do L0 ainda não materializado e avisos de código preexistente. Não houve
resselo de headers para esconder esse estado.

Os gates confirmam L0 estável durante sua execução, fontes Rust idênticas
ao baseline R5 e predecessores intactos. Alterações anteriores de P1306
permanecem preservadas. Não houve build de candidato, RED/GREEN, mutantes
Rust, certificado, stage, commit, push ou remoção de temporários.

Após a aprovação concreta ADR-0127, a continuação é selar o manifesto de
aceitação sucessor, completar as testemunhas focais independentes explicitadas
na review e materializar o contrato. A skill `tekt-materializacao-segregada`
manteve autoria e veredito separados; o regime foi executado sem atestação
de isolamento técnico. A pausa atual é o gate público, não uma nova rodada
de diagnóstico sem decisão.
