# P1339 — adendo independente ao parecer r1

Este adendo corrige a precisão da hipótese de representação e registra uma pendência de A.1. Não altera o parecer congelado `p1339-protocol-review-r1.md` (SHA-256 `55596d0e0f26838f5060210a0db6dbc89917d1bb45808ce94bfc9276d4c04f14`), a autoridade do passo, L0, código, contrato ou oráculos. Executor `/root/p1339_protocol_r1`; regime `executado sem atestação de isolamento`. Inputs abaixo foram hashados antes da leitura desta extensão.

Estado observado às `2026-09-09T23:40:15Z`: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, `git diff HEAD --stat` vazio; os diagnósticos P1339 continuam não rastreados. O parecer anterior qualificou a hipótese como plausível, não como desenho aprovado; este adendo torna explícitas duas limitações adicionais.

## O carrier ainda precisa de desenho válido em Rust

Medição: `01_core/src/entities/value.rs:9-11,76` declara `Value::Dict(IndexMap<EcoString, Value, FxBuildHasher>)`; `:495-497` expõe exatamente esse tipo pelo cast. O SHA-256 dessa fonte é `f792590f48799067f2f00767319c97aa498a519f207a154332ffc4deb6c8712b`. Não há uma entidade `Dict` usada como tipo desse carrier; a hipótese `Selector::Element { function: Func, fields: Dict }` empregou abreviatura como se fosse um tipo Rust existente.

Portanto essa hipótese não é um desenho de L0 pronto. Não legitima introduzir entidade/alias `Dict`, copiar o tipo vanilla ou escolher incidentalmente um novo carrier. `EcoVec<(EcoString, Value)>` é uma alternativa a avaliar, somente se a medição e a especificação fixarem normalização de chaves duplicadas, ordem observável, igualdade/hash e morfologia (incluindo filtro vazio e grupo único de campos). Um vetor de pares não preserva automaticamente a semântica de dicionário. `Args` continua sendo a origem da ordem de avaliação, das ocorrências e dos spans; o carrier final não deve reconstruir essa história. A escolha de representação e a alteração pública de `Selector` permanecem pendentes do desenho L0 e do gate ADR-0127.

## Angle NaN não foi demonstrado como entrada pública

Recibos inspecionados, sem reexecutar os binários:

- `p1339-full-final-manifest.json`: SHA-256 `a7afdbf4e5bea1cc897adf330bb307a162b17cd7024806d8b45111619e477336`.
- `p1339-full-final-vanilla-runs.json`: SHA-256 `a7280e76e03c3b98b4681259f6a7e11daca28ce821e2a1bd9b30394458118953`.
- `p1339-full-focal-r1-manifest.json`: SHA-256 `0446ac3b7da2aea11d0f4a56e8e2a5d6d87586d9dd7f0fb92c5d4294bd506ebe`.
- `p1339-full-focal-r1-vanilla-runs.json`: SHA-256 `88b7f57ea294847c1372ae3aa8b84ba78c37dc575f969bb445342411307a7379`.

No recibo final, sobre o mesmo HEAD e diff tracked vazio, `/usr/local/bin/typst` tem SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`. A linha identificada por `id = angle.deg-static-(float("nan") * 1rad)`, `order = normal`, `profile = default`, iniciou às `2026-09-09T23:35:53.388560+00:00`: saída `(float, 0.0, true, 1.0)`, exit 0, stderr vazio. A forma ligada e as conversões para radianos devolvem a mesma tupla. A revisão focal anterior registra o mesmo resultado. Isso observa conversão para zero na construção/uso dessa expressão; não demonstra que a função tenha recebido um Angle NaN. Não atribuir a normalização à conversão `deg`/`rad` sem análise adicional.

Não confundir esse achado com perda geral de zero negativo: `angle.deg-zero-observable`, às `23:35:53.639085+00:00`, retorna `(0.0, -0.0, 1.0, -1.0)`. A normalização citada concerne à tentativa de construir NaN. A auditoria dos recibos selecionados não prova impossibilidade universal de construir Angle NaN, mas também não fornece uma construção pública bilateral válida.

O P1339 vigente, SHA-256 `817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9`, exige em :121-122 classificar como `Unknown` NaN/infinito quando a sintaxe pública bilateral não os construir. Em :185, `Unknown` obrigatório bloqueia o passo. Não é legítimo contar o zero observado como cobertura NaN, promover esse caso a controle opaco deliberado ou removê-lo do gate por interpretação conveniente.

Antes da Fase B, essa obrigação precisa de resolução: demonstrar uma construção pública bilateral de Angle NaN, ou obter decisão normativa explícita do dono sobre o tratamento da ausência de construtibilidade. Enquanto isso, a viabilidade condicional de C/E exposta no parecer não autoriza ultrapassar a pendência de A.1. A mensagem que solicitou este adendo não autoriza mudança normativa, e nenhuma foi feita.
