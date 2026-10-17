# Proposta: transformações tipadas e resolução no layout

**Aguardando confirmação. Nenhum código ou L0 canônico desta proposta foi alterado.**

Substituir `TransformElem.matrix` por `operation: TransformOperation` e manter
`body: Content`. A enum pública distingue Matrix, Move, Rotate, Scale e Skew;
os argumentos das quatro funções são tipados e opcionais para preservar presença.
`TransformScale` distingue Auto, Ratio e Length. A assinatura de
`Content::transform(matrix, body)` permanece e cria Matrix para uso Rust genérico.
Literais Rust externos e leituras de `.matrix` precisarão migrar.

O [contrato exato](candidates/00_nucleo/prompts/entities/elements/transform.md)
contém todos os tipos, campos e regras. Os candidatos de
[construção nativa](candidates/00_nucleo/prompts/compiler/stdlib/transforms.md),
[layout](candidates/00_nucleo/prompts/compiler/layout/transform.md) e
[construção Rust](candidates/00_nucleo/prompts/entities/content/construction/layout.md)
definem as responsabilidades, com núcleo compartilhado pinado. Cada prompt
mantém um único consumer. A validação V15/V26 passou na cópia de revisão.

## Mudanças observáveis incluídas

- Preservar `origin`, `reflow`, units, auto e argumentos omitidos; corrigir fields,
  func, repr, igualdade, observação, encoders, set/show/where e getters contextuais.
- Resolver em/percentual/escala por tamanho no layout, com região, fonte e body
  efetivos. Aplicar pivô sobre frame lógico e reflow explícito, preservando fluxo,
  baseline, recursos e ordem de leitura.
- Corrigir a validação de chamadas que hoje são aceitas ou ignoradas indevidamente.
  `scale(2)` passa a erro: usar `scale(200%)`. `scale(x:150%)` deixa y em100%.
  `rotate(angle:23deg)` é erro; o ângulo é posicional. `skew(12deg)` é erro;
  usar `skew(ax:12deg)`. `move(dx:2)` requer unidade, como `2pt`.
- Conservar a rejeição estrita de transformações não suportadas em HTML fora de
  ilhas; o pin avisa e elimina TEXT nos controles. Não reproduzir essa perda de
  conteúdo nem transformar todo HTML em documento paginado.

## Aceitação autorizada pela proposta

X1: casts, ordem/consumo/spans e diagnóstico de argumentos, incluindo negativos.
X2: campos explícitos/omitidos, identidade nativa, igualdade/observação, mapas e
encoders, inclusive zero/false/auto e identidades matriciais coincidentes.
X3: estilos/aliases/with, fold por eixo, show/where, getters contextuais e ausência
de defaults assados nos campos; sem efeitos duplicados na medição.
X4: origin/default/RTL, Move relativo/em, Scale Ratio/Length/Auto, reflow, limites,
baseline, texto/matemática, following content, nested, clips, Parent e recursos.
X5: integração PDF/SVG/PNG, controles HTML e Matrix Rust, suíte completa,
build/doc/fmt, inventário de avisos e linhagem direta/reversa.

Depois de confirmar, instalar os candidatos, atualizar os L0 dependentes da lista
de impacto antes de seus consumers, testar RED→GREEN e ressellar apenas owners
alterados. A lista de impacto é conservadora; presença na lista não obriga editar
arquivo que só atravessa body. Nenhum contrato público adicional fica autorizado
por inferência: se surgir outro, exige revisão própria. Não inclui release.

Manifesto SHA-256: `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`. [Manifesto](manifest.json),
[fontes/owners base](base-files.json.gz), [checagem do grafo](graph-validation.log).

A [ADR-0127 §2](/repos/Antigravity/typst-crystalline/00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md)
exige a confirmação: “adicionar, remover ou alterar campo público” e “Mudança
arquitectural de fase/ordem de pipeline”. Aplica-se à remoção de matrix/adição de
operation e à resolução tardia de valores antes reduzidos em eval. As aprovações
anteriores de Raw e tiling não cobrem esse contrato novo.
