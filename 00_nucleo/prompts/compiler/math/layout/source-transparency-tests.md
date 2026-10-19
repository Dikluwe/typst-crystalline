# L0 — math/layout/source-transparency-tests
Hash do Código: 45aae044

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/math/layout/source_transparency_tests.rs`

## Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

## Contrato exclusivo

Possuir apenas regressões puras de transparência Source para o layout matemático.
Usar MathLayouter pela entrada pública e FontMetrics determinístico, com advance,
arestas, TopAccentAttachment, kerns assinados e variantes horizontais declaradas.
Não carregar fonte, World, relógio, arquivo ou rede. Manter a fixture atribuível;
ela não demonstra seleção de fonte real ou paridade de exportação inteira.

Os 23 pares da análise suplementar são casos separados. Comparar geometria
(width/ascent/descent, baselines, posições e tamanhos de texto/glyph) com 1e-9,
texto/caractere observável e identidade extraível por ocorrência, sem exigir bytes,
GID, pointer, formato privado de frame ou igualdade Rust de Content. O observador
conhece Text/TextShaped/Glyph, TextReplacement e Semantic; os seis demais tipos
atuais de FrameItem são Unknown e falham explicitamente, sem fabricar observação.
Text/TextShaped partilham observação visual; os seis tipos recusados partilham
a mesma falha, justificando só esses dois grupos V19. Não ignorar item desconhecido.

Conservar controles positivos independentes: limits inline empilha e scripts fica
lateral; o mock fornece variantes extensíveis com avanços distintos; a matriz
retém alinhamento de coluna por ampersand; espaço fraco interno permanece e nas
bordas colapsa. Source não altera identidade extraível do acento atômico ou do
acento aninhado, nem dotless i/j. Nunca usar identity text para medir glyphs.

Adicionar controle de equação display com duas linhas: Source conserva line_count
2 e primeira/última baseline. Adicionar controle de flatten: cada fragmento retém
origem própria ou recebe origem comprovada do container, sem substituir origem
já presente e sem alterar a árvore autoral. Um controle negativo de observador
com TextSeparator deve falhar Unknown. A transformação dotless terá seu teste
causal e de origem co-localizado no owner de accent; não exportar helper só para
este consumer nem compartilhar propriedade do código produtivo.

## Aceitação

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.


### Marcadores Source no grid — complemento medido em 2026-10-17

source-markers-before.json confirma: Source(Linebreak) transforma duas linhas
em uma (a/b na mesma baseline); Source(MathAlignPoint) conserva duas linhas,
mas desloca 1/2 em relação à coluna de 333/4. Preservar esse RED antes do reparo.
No owner pai, needs_grid_layout e partition_grid também classificam os marcadores
por node.source_body(), sem mudar separadores, linhas/células vazias ou ordem.
No braço de conteúdo de partition_grid, reter o clone original com sua origem;
usar complemento nominal plano se isso eliminar o binding normalizado anterior,
sem captura @ ou suppressão. A nova origem nunca fabrica Location.
O novo consumer de regressão acrescenta dois testes públicos display, para
Linebreak e MathAlignPoint Source, exigindo as mesmas baselines e posições.
Os 27 controles anteriores não mudam: o total passa a 29, com RED esperado
5 passando/24 falhando. Nenhum novo contrato, fase, default ou API pública.


### Empréstimo no controle de origem — 2026-10-17

Clippy estrito mede `cloned_ref_to_slice_refs` no controle de flatten,
source_transparency_tests.rs:378; recibo e fontes exatas do candidato em
attempt-clippy/ e clippy-refinement.json. O contêiner é apenas emprestado
para a entrada de um item. Prescrever `std::slice::from_ref(&container)`
em vez da fatia temporária com clone, mantendo os mesmos dados, spans,
asserções, observação e contagem de testes. Não introduzir supressão ou API.
