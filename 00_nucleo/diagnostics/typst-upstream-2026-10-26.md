# Referência Typst e oportunidades Tekt — 2026-10-26

A referência vigente passou de `bd5ebec285505889e8ff1d4f68804ab147a699f3` para
`64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`.
A atualização foi autorizada pelo usuário nesta conversa. O upstream oficial foi
consultado novamente ao encerrar a investigação e manteve esse HEAD.

Medições: HEAD local `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**;
lista exata de alterações, diffstat e hashes em
`lab/results/construction/diagnostics/vanilla-resync-2026-10-26/before.json`.
Cada execução bilateral tem UTC, comandos e hashes próprios em `issues/`.
Fechamento: `2026-10-26T13:38:27.084144+00:00`. Os 2165 inputs produtivos/L0 foram
preservados byte a byte. Os arquivos previamente alterados também foram
preservados; o índice de diagnósticos recebeu apenas uma nova entrada. O produto
release manteve SHA-256 `c964cf75e8483b0efcea691cba57d5683229989c553fc94cc3d65c747d793567`.

## Evidência da atualização

- Delta upstream: 23 commits, 63 arquivos; os 2.889 caminhos da árvore foram
  comparados por blob. Apenas `Cargo.lock` permanece como overlay rastreado;
  o extrator local anterior também foi preservado.
- Build Rust 1.92.0, `--release --locked -p typst-cli`, um job, teto de 12 GiB,
  zero swap e `TYPST_COMMIT_SHA` explícito. `sys.version` permaneceu
  `version(0, 15, 1)`, sem alterar o contrato público do produto.
- Novo oráculo SHA-256: `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`.
  Anterior preservado em `lab/typst-original/target/release/typst-bd5ebec285-2026-10-17`.
- Referências ativas atualizadas no guia, README, CI, construtor do oráculo,
  validador e ferramentas/matriz de paridade. L0 e relatórios históricos
  mantêm suas evidências anteriores; esta troca não recertifica esses relatórios.
- Validação: 12 gates com exit 0; 9954 testes Rust aprovados e 7
  ignorados; 42 testes Python de CI e 85 da matriz aprovados. Linter completo
  sem warnings/errors (notas informativas V19/V20 permanecem).
- Sonda de símbolos: 162 MATCH e 36 extensões; linguagem: 9 MATCH, 3 casos
  desabilitados por perfil, zero violações. Matriz com 43 casos por perfil:
  padrão 27 MATCH/14 desabilitados/2 diferenças esperadas; HTML+bundle
  40 MATCH/1 desabilitado/2 diferenças esperadas. As diferenças esperadas são
  rasterização de 11 pixels por um nível de canal e sufixo interno do nome da
  fonte; não constituem nova prova de divergência da linguagem.

## Issues: medições antes da prioridade

Foram catalogadas 65 issues únicas, examinadas nove em detalhe e lidos os
comentários de cinco. Seis issues receberam reproduções bilaterais, incluindo
dois tamanhos de JPEG. A seleção não pretende cobrir todas as issues abertas.

| Issue | Resultado no novo vanilla e no Tekt atual | Oportunidade |
|---|---|---|
| [#8984 — JPEG e memória](https://github.com/typst/typst/issues/8984) | JPEG RGB 6000×4000: RSS máximo 93.752 KiB no vanilla e 163.516 KiB no Tekt; ambos geram PDF válido. Controle 1200×800: 25.160/21.596 KiB. Uma execução por combinação, sem inferir médias. | Primeiro candidato de melhoria: validação de imagens em L3. |
| [#8507 — atributos HTML duplicados](https://github.com/typst/typst/issues/8507) | Vanilla emite `href="A" href="B"`; Tekt emite apenas `href="B"`. | Comportamento já correto no reproducer; acrescentar regressão específica custa pouco. |
| [#8965 — texto em tiling SVG](https://github.com/typst/typst/issues/8965) | Vanilla emite três NUL e XML inválido; Tekt emite XML válido, sem NUL, com todas as referências de glifos resolvidas. | Proteger a ordem de emissão de recursos com regressão. Não foi certificada identidade visual pixel a pixel. |
| [#8966 — palavra UTF-8 maior que 64 KiB](https://github.com/typst/typst/issues/8966) | Vanilla termina com panic/exit 101; Tekt termina 0 e `qpdf --check` passa. | Regressão de robustez é pequena; o ensaio não certifica extração integral do texto fora da página. |
| [#9013 — releitura no watch](https://github.com/typst/typst/issues/9013) | Inspeção mostra releitura e hash no polling do Tekt e cache binário restrito a cada World. Sem benchmark bilateral de watch nesta rodada. | Relevante, mas não é vantagem pronta nem reparo trivial. |
| [#8935 — nota em cabeçalho repetido](https://github.com/typst/typst/issues/8935) | Vanilla rejeita label duplicado; Tekt rejeita estabilidade contextual. | Aberta nos dois; exige investigação própria de identidade e fragmentação. |

## Por que pode ser mais simples no Tekt

**Medido no código:** `03_infra/src/export/images/scan.rs:78` centraliza a
validação JPEG por decodificação completa, enquanto `images/xobjects.rs:25` já
embute bytes JPEG originais com DCTDecode. No upstream fixado,
`lab/typst-original/crates/typst-library/src/visualize/image/raster.rs:25`
retém `Arc<DynamicImage>`, e as linhas 72–78 decodificam o JPEG na construção.
O Tekt já separa esse buffer de validação dos bytes transportados para PDF;
isso delimita o experimento, sem garantir que um decoder em fluxo esteja disponível.
O L0 proprietário
`00_nucleo/prompts/infra/export/images/scan.md:20` exige hoje essa decodificação.
O primeiro recorte pequeno seria validar cada imagem compartilhada uma vez por
exportação. Isso reduz trabalho repetido, mas **não** resolve o pico de uma
imagem isolada. Reduzir esse pico requer estudar validação em fluxo/decodificação
limitada sem perder rejeição de JPEG truncado, ICC, orientação e espaços de cor.
É hipótese de implementação localizada em L3, não ganho de memória já obtido;
seria refutada se os decodificadores disponíveis exigissem o buffer integral ou
se os testes de arquivos corrompidos regredissem. Atualizar L0 antes de implementar.

**Vantagens já presentes no escopo testado:** `03_infra/src/export/svg.rs:218`
emite paints antes das definições finais de glifos; isso permite que o tiling
registre seus recursos. `01_core/src/compiler/stdlib/html/typed_element.rs:82`
consome argumentos nomeados e insere atributos por chave. O cluster de texto em
`01_core/src/entities/shaped_glyph.rs:21` é `u32`, e o exportador PDF próprio
evita o caminho de truncamento `u16`/krilla apontado na issue #8966. Essas
observações explicam por que regressões pequenas podem proteger esses casos;
não provam cobertura geral de HTML, SVG ou Unicode.

**Candidatos adiados:** `03_infra/src/watch.rs:40` lê todo arquivo e
`wait_for_change_since` repete a operação; `SourceSession` retém sintaxe,
não binários entre compilações. Uma mudança deve preservar detecção de troca
com mesmo tamanho/mtime e publicação atômica. Na [#8938](https://github.com/typst/typst/issues/8938),
os comentários revelam conflito entre documentação e testes de circle/square;
não mudar a semântica sem resolver a intenção. Na [#8915](https://github.com/typst/typst/issues/8915),
sucesso em PDF sem tagging não provaria correção de acessibilidade. Na
[#9015](https://github.com/typst/typst/issues/9015), ambos exportaram SVG válido,
mas a cor do highlighting Python não foi certificada; Tekt também usa syntect.

A prioridade sugerida é investigar JPEG; depois proteger HTML, SVG e UTF-8 com
regressões específicas. Consumo, cache e representação são mecânica; atributos,
integridade do documento, erros e conteúdo observável são linguagem. Falhas do
vanilla não devem ser copiadas como requisitos (ADR-0107/0108).

Recibos, fontes e resultados reproduzíveis:
`lab/results/construction/diagnostics/vanilla-resync-2026-10-26/`.
