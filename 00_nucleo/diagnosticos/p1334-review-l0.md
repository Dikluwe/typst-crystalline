# P1334 — L0 apto para congelar testes e executar RED

Parecer: **apto para RED após o freeze A/B**, sem objeção substantiva ao
contrato. Este parecer não autoriza antecipar candidato nem atesta testes
ainda não congelados. Regime A/B sem atestação técnica de isolamento.

Manifesto conferido por SHA-256
`2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da`.
Baseline público confere
`bb1e8dde73ba8ae771b72d5c2e344092d797b0a6cf89c9418fa3c553503631aa`;
baseline integral confere
`6643993d43b905e878095c722c99019c5f98cdd724cddfc3b90961153b205819`.
Ambos identificam HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, árvore
não commitada e seu diff/stat/inventário. As conclusões abaixo pertencem
a essa árvore com apenas os amendments L0 e headers P1334 anteriores ao RED.

Li os três amendments, preservados como apêndices aos L0 completos já
auditados. `calc.md:1061` contempla primeira ocorrência named value mesmo
após outro named, hint apenas sem posicional, cast do primeiro antes das
sobras, ordem conjunta, arg-span versus value-span e missing agregado.
Síntese None tem ordem/spans explícitos; histórico incoerente fica somente
em robustez. O recorte substitui nominalmente os guards antigos e preserva
fórmulas, eager, nomes e resolução matemática importada fora do escopo.

`call_dispatch.md:745` acrescenta exclusivamente a identidade de calc_abs
ao transporte incondicional por ponteiro e With. `_comum.md:148` autoriza
somente reexport pub(crate), sem API Rust pública. A ligação/validação
separadas obedecem ownership 1:1 e ADR-0127 contínua. Testes requeridos
incluem falsos homônimos, origens distintas, preargs intactos e identidades
anteriormente autorizadas. Não surgiu necessidade de novo gate humano.

Auditoria executada com Node somente leitura contra `original_files` e
`product_inventory` do baseline: conteúdo de cada L0 anterior é prefixo
integral do vigente; cada consumer continua byte-idêntico após excluir
unicamente o header `@prompt-hash`. O delta do inventário produtivo está
restrito aos três pares manifestados. Isso confirma ausência de candidato
runtime no instante desta revisão, não só a declaração do coordenador.

Recalculei separadamente hashes normativos, payload do núcleo existente,
pins, headers e hash inverso do código. Todos coincidiram:

```text
calc         05736e494ca184ce7c13b8eb5a9296275cce0e5c4605fde251b7ab2b17ec8666
dispatcher   1ddee9967f21df96078e8906ef52d405247bc79703e3c9dfaedc74479c02f7a2
hub          ea0a25b75a6d59aa3f6085072379499003df314435c3ed98468bb16a6439bfe6
```

O recibo `p1334-preflight.json`, SHA-256
`eb5c372c86207afe79cddde23ff7cd67cad2c2a03a4a09f20fb27c40f1675ccf`,
registra `crystalline-lint --checks v5,v15,v26 --fail-on warning .`, exit 0,
sem violações, UTC `2026-09-09T15:56:47.476740+00:00` a
`2026-09-09T15:56:51.435607+00:00`. Seus inventários antes/depois coincidem
e o manifesto referenciado é o conferido acima. Não substituí a evidência
por nova execução redundante. Build, workspace, RED/GREEN e CLI finais
continuam pendentes; Unknown obrigatório bloqueia o fechamento.

Li somente o passo novo explicitamente autorizado
`00_nucleo/materialization/typst-passo-1334.md`; nenhuma leitura de passo
histórico ou context. O incidente anterior de `git status --short` que
exibiu nomes restritos permanece registrado em `p1334-review-scope.md`,
sem leitura de conteúdo e sem alegação de isolamento. Nesta revisão,
escrita limitada a este parecer; não editei os artefatos julgados.
