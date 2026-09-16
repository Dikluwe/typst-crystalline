# Passo 1421 — expandir `context` no target HTML sem layout paginado

## Objetivo

Fechar a causa nova que bloqueou o P1420: o caminho HTML avalia
`#context ...` como `Content::ContextBlock` e entrega esse marcador diretamente
ao serializer, que termina com `HTML export does not yet support this content`.

O passo introduz expansão contextual semântica entre eval e export HTML. Não
implementa qualquer uma das 66 rotas P1420, não altera o serializer HTML, não
cria `PagedDocument`, não executa layout e não retoma a campanha integral
P1420 antes de os controles focais ficarem verdes.

## Estado congelado e proveniência

Medição em `2026-09-16T11:27:51-03:00`:

- HEAD `ef33ba5204870646a7c4f3046d8b210d6cd8e8cf`;
- tree `dc519971074a1455c05b5bf20fc5eebdbff042df`;
- working tree limpa;
- vanilla ratificado
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário vanilla SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- binário cristalino pré-candidato SHA-256
  `1eb24a50c4bebd08febe976da73c9b275d8c1200db7781ad8e9d7484089e547e`;
- relatório P1420 `00_nucleo/diagnosticos/p1420-report.md`;
- verificação P1420 SHA-256
  `0251e7d443679b679ec8fcfa03f6d429b83649cc2bd2dac346328c8bcea06992`.

Hashes das entradas L0 e dos consumers antes desta proposta:

- `prompts/infra/pipeline.md`:
  `2f54bd4ed504849fd31d06a0f008cfda912d2a3851af48bc2b20e7a4ee62d607`;
- `prompts/infra/pipeline/context_stabilization.md`:
  `6e918f2f6f9f09f7fe5200a2d1573348a17612375c5512be5cd22e30eb7a9b2f`;
- `03_infra/src/pipeline.rs`:
  `435bc7a81f7d86d9818365d9c61b84ae828e7713109bc168da705e22354e0e6a`;
- `03_infra/src/pipeline/context_stabilization.rs`:
  `91188ee82f8e15554d619d3e1381e454f3521bfaeff3c360c07b39dba59773f1`.

## Medição anterior à decisão

O caminho produtivo `compile_to_html_string_with_features_and_serialization_and_pretty`
em `03_infra/src/pipeline.rs:232-263` faz:

```text
eval(target=html) → module.content → export_html
```

Não há introspecção nem expansão contextual nesse caminho. O serializer em
`03_infra/src/export/html.rs:480-516` não possui braço para
`Content::ContextBlock` e devolve o diagnóstico genérico medido. O caminho
paginado, por contraste, introspecta e chama `context_stabilization::prepare`
antes de layout.

O módulo privado de estabilização fixa atualmente `ctx.target =
EvalTarget::Paged` em `context_stabilization.rs:172`; reutilizá-lo sem tornar o
target causal faria `target()` devolver `"paged"` durante um compile HTML.
Além disso, a substituição paginada conserva o marcador locatável junto ao
resultado para o layout; esse marcador não pode chegar ao serializer HTML.

Medição bilateral do vanilla ratificado, transporte
`compile --features html --format html`, um processo por fixture:

| Fixture | Exit | Bytes | SHA-256 | Morfologia pública |
|---|---:|---:|---|---|
| `#context assert.eq(target(), "html")` | 0 | 154 | `52569b3f0175d7ded4383171fa34c563019206cbc8bac63a3572e775bb72adcf` | body vazio |
| `#context target()` | 0 | 165 | `1f51255aef2eaf4fcbfdb0b08152ba5710258427f93be68692d5c200bc9321d0` | `<body><p>html</p></body>` |
| `#context [#html.elem("div")]` | 0 | 165 | `fe18965c5ffb798226ea7d3335fda25c93b8a69817e9647e49812aa99c23f04c` | `<body><div></div></body>` |

O cristalino pré-candidato termina as três antes de produzir artefato HTML.
Isto mede uma lacuna de fase, não autoriza ensinar o serializer a ignorar o
marcador nem a devolver conteúdo vazio por default.

## Decisão arquitetural proposta

No caminho HTML, depois de eval e antes do exporter:

1. construir a topologia de introspecção do `Content` sem layout;
2. executar cada `ContextBlock` alcançado com `EvalTarget::Html`, features e
   política `full_error` causais da compilação;
3. propagar erros e warnings das closures pelo canal já existente;
4. substituir os blocos pelo conteúdo produzido;
5. retirar somente os marcadores `ContextBlock` residuais da vista entregue
   ao serializer HTML;
6. chamar o mesmo exporter e o mesmo modo de serialização/pretty já escolhidos.

A variante paginada continua a conservar marcadores para locations e continua
com o seu fixpoint/layout vigente. HTML não cria páginas, posições ou
`PagedDocument`, não entra no fixpoint paginado e não usa igualdade de bytes ou
estrutura Rust como critério linguístico.

Leituras cuja estabilidade dependa de páginas/posições ou de seleção paginada
permanecem fora do recorte. Incapacidade de provar uma expansão HTML completa
é erro/`Unknown`, nunca sucesso por remoção silenciosa. Contextos aninhados não
são gate positivo deste passo; qualquer ocorrência residual depois da
preparação deve impedir o export.

## Classificação ADR-0127 e gate obrigatório

Este passo muda a fase do pipeline HTML ao inserir introspecção/expansão entre
eval e export. É a classe 3 do ADR-0127. Portanto:

1. os L0s `infra/pipeline.md` e
   `infra/pipeline/context_stabilization.md` são atualizados primeiro;
2. **parar para confirmação humana explícita**;
3. somente após confirmação e resselo autorizado iniciar testes RED e código.

O pedido genérico para “implementar o próximo passo” não elimina o gate porque
a decisão concreta de fase só se torna avaliável depois desta medição e destes
L0s.

## Materialização segregada após aprovação

Regime completo, executado sem atestação de isolamento se os papéis partilharem
workspace.

- **Autor da obrigação/L0:** este passo e os dois prompts proprietários.
- **Autor do contrato/testes:** congela as três fixtures acima, um controle
  paginado e casos negativos de marcador residual sem ler o patch candidato.
- **Adversário:** muta target causal, feature causal, ordem da fase, remoção de
  marker e propagação de erro/warning.
- **Implementador:** edita somente os dois consumers L3 e testes locais depois
  do preseal.
- **Verificador:** executa oráculos bilaterais, mutation gate, lints e preserva
  as entradas seladas sem corrigir o candidato.

Mutation score obrigatório `1.0`. `Unknown` não vira sucesso. Um mutante
válido sobrevivente impede materialização.

## Testes RED obrigatórios após aprovação

1. integração HTML da fixture `assert.eq(target(), "html")` termina com body
   vazio e sem marcador residual;
2. `#context target()` emite texto `html`, nunca `paged`;
3. `#context [#html.elem("div")]` preserva feature e morfologia da tag;
4. o caminho paginado continua a expandir `#context target()` para `paged`;
5. feature HTML ausente continua a falhar antes da expansão;
6. erro da closure contextual impede qualquer artefato HTML;
7. marker residual/expansão incompleta não é descartado silenciosamente.

Os testes devem falhar no baseline antes da implementação. Aceitação compara
DOM/morfologia, exit e diagnósticos públicos; não fixa bytes internos do Rust.

## Gates finais após implementação

```sh
cargo test -p typst-infra p1421_ -- --nocapture
cargo test -p typst-infra p821_context_target_expande_paged -- --nocapture
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Reexecutar também o focal P1420 para `html.area` e `math.abs` nas três ordens.
Somente se o controle HTML ficar verde o P1420 pode ser retomado numa campanha
integral separada; P1421 não reclassifica as outras 64 rotas.

## Critério de fecho

P1421 fecha quando as três fixtures HTML são bilaterais no fragmento medido,
o caminho paginado permanece estável, erros não são escondidos, os dois L0s
estão ressellados, mutation score é `1.0` e todos os gates passam. O veredito
cobre expansão contextual HTML sem layout paginado, não paridade HTML geral.
