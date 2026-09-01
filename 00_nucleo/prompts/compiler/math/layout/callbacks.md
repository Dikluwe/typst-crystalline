# Prompt L0 — `compiler/math/layout/callbacks` — transcript selado de callbacks math
Hash do Código: 818f5ca1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Estado:** MATERIALIZADO — consumer e linhagem selados no Passo 1291.

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/math/layout/callbacks.rs`
**Único consumer produtivo proprietário:** o próprio alvo acima
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

`MathLayouter` mede o body de `cancel`, mas não possui `Engine`. A pipeline
possui `Engine`, porém só pode conhecer o ângulo default depois de uma passagem
de layout. Os precedentes P240/P241/P1159 executam callbacks exclusivamente na
pipeline e entregam ao layouter vistas seladas da passagem anterior; o layouter
permanece puro (`ADR-0081:344-345,491-494` e
`03_infra/src/pipeline.rs:678-680`).

No vanilla, `draw_cancel_line` chama a função depois de calcular
`default_angle(body_size)` (`typst-layout/src/math/cancel.rs:92-103`). A função
fica dentro de `draw_cancel_line`; por isso `cross: true` executa a callback
duas vezes, uma para cada linha (`:22-65`), em vez de reutilizar um resultado.
O contexto da chamada recebe os estilos vigentes na posição (`:99`).

`MathCancelElem` é não-locatável tanto no vanilla quanto no contrato cristalino.
Promovê-lo a locatável apenas para resolver a callback mudaria a identidade da
linguagem e dessincronizaria os locators. `EquationElem`, porém, já é locatável.
A identidade estável sem deriva é a `Location` da equação combinada com a ordem
estrutural dos cancels dentro dela e o índice da linha.

## Contrato público proposto

Este módulo contém somente dados e coordenação em memória; não contém trait de
execução, `Engine`, `World`, I/O nem import L3.

```rust
pub struct MathCancelRequest {
    pub id: MathCancelRequestId,
    pub func: Func,
    pub default: Angle,
    pub span: Span,
    pub styles: StyleChain,
}

pub struct MathCancelRequestId {
    pub equation: Location,
    pub occurrence: usize,
    pub line: u8,
}

pub struct MathCancelResolution {
    pub request: MathCancelRequest,
    pub angle: Angle,
}

pub struct SealedMathCallbacks { /* sequência imutável de resolutions */ }

pub enum MathLayoutPassOutcome {
    Pending(Vec<MathCancelRequest>),
    Complete(PagedDocument),
}
```

Cada tentativa concreta de `Layouter::new` cria um `MathCallbackPassState`
novo. Esse estado possui somente a store selada de leitura e um
`RefCell<Vec<MathCancelRequest>>` local para o transcript; não usa
`static`, `thread_local`, estado global, `Arc` nem cache entre tentativas.
Cada `MathLayouter` possui um `Cell<usize>` local, iniciado em zero para a
equação corrente. O braço `Content::MathCancel` reserva e incrementa esse
occurrence exatamente uma vez, antes de `layout_cancel` descer no body; as
duas linhas de `cross` derivam apenas `line=0|1` do mesmo valor reservado.

Ao encontrar uma linha com `MathCancelAngle::Func`:

1. mede o body e calcula o ângulo default;
2. materializa o snapshot de estilo da posição combinando a `StyleChain`
   léxica com o `TextStyle` math efetivo pela normalização abaixo;
3. constrói o id com a `Location` da equação, occurrence do elemento e linha
   `0|1`;
4. usa o ângulo selado somente se a resolução desse id corresponder à mesma
   função, default e estilo colapsado;
5. sem correspondência, usa o default apenas para produzir a passagem
   provisória e registra a request como pendente.

O documento provisório nunca atravessa a API: quando houver request pendente,
store incompatível ou item selado não consumido, a passagem descarta o
`PagedDocument` construído e retorna somente `Pending(requests)`. Apenas uma
passagem com store integralmente correspondente retorna `Complete(document)`.
Store com item extra, ausente ou diferente é stale: nunca é aceita
parcialmente nem convertida implicitamente em sucesso.

O transcript pertence à tentativa concreta que o produziu. Se o loop interno
de layout rejeitar a tentativa (inclusive por iteração de TOC), seu estado e
suas requests são destruídos juntos. Somente o transcript da tentativa
candidata pode produzir `Pending`; somente uma tentativa candidata sem
pendências pode produzir `Complete`. A pipeline estabiliza essa candidata
antes de a entregar à realização de page numbering ou a qualquer exporter.

O occurrence conta todos os `MathCancelElem` visitados na equação, inclusive
os de ângulo Auto/explícito; não depende de página, frame, ponteiro ou hash de
`Func`. `cross: true` usa o mesmo occurrence com `line=0` e `line=1`, produz
duas requests e duas execuções. **As duas requests recebem o mesmo ângulo
default positivo calculado da caixa**; `line` distingue chamadas, não altera o
argumento. A segunda linha reflete somente o ângulo já devolvido. `Span` é dado
diagnóstico, nunca identidade. Sem `cross`, `inverted` é aplicado depois do
ângulo devolvido e não faz parte da chamada; com `cross`, `inverted` é ignorado
e a primeira/segunda linhas usam respectivamente resultado direto/refletido.

### Normalização de estilo

O snapshot começa em `style_chain.clone()` para preservar todos os `custom` e
demais valores léxicos. Sobre essa chain empurra um único `StyleDelta` formado
campo a campo por **todo campo de `TextStyle` que possua slot correspondente em
`StyleDelta`**, sem aplicar defaults novamente; o `custom` desse delta é vazio.
Isso inclui tamanho/fonte/peso/fill e os demais eixos textuais representáveis,
inclusive alterações de script/math já refletidas no `TextStyle.size`.

Flags internas math sem representação em `StyleDelta` continuam somente
geométricas e não são falsificadas como propriedades consultáveis por
`Engine.styles`. Se uma API contextual futura as expuser, este contrato volta
ao gate. A equivalência de requests compara `StyleChain::collapse()` após essa
normalização (`StyleDelta::PartialEq`), não ponteiro de chain, chain default ou
hash textual.

## Estabilidade e política de falha

O ângulo altera somente a geometria do `FrameItem::Line`; não altera
`width`/`ascent`/`descent` do `MathBox`. Assim o transcript deve estabilizar na
passagem seguinte. Mesmo assim, a pipeline compara os transcripts e usa limite
finito de convergência: drift ou teto excedido produz `SourceDiagnostic`, não
`Unknown`, `Auto` final ou resultado parcial.

Erro da função ou retorno que não faz cast para `Angle` aborta a realização no
span capturado. A store é local à compilação, não é global e não é reutilizada
entre compilações. Hash identifica bytes/entradas; não substitui a comparação
semântica do transcript.

## Ownership e aceitação

Este prompt possui exatamente o futuro consumer
`01_core/src/compiler/math/layout/callbacks.rs`, incluindo o outcome fechado.
`layout/mod.rs` somente cria e transporta o estado da passagem sob
`compiler/layout.md`; `cancel.rs` produz e
consome requests sob `compiler/math/layout/cancel.md`; `pipeline.rs` executa as
funções sob `infra/pipeline.md`.

Testes deste owner provam: identidade equação+occurrence+line, mesmo default
positivo nas duas chamadas de `cross`, precedência de `cross` sobre `inverted`
e reserva antes
da descida no body; estado novo por tentativa concreta e descarte do transcript
de iteração interna rejeitada; store stale
rejeitada; estabilidade por repetição e reflow; dois requests para `cross`;
chain lexical com `custom` preservada junto a `TextStyle` math não-default;
erro da closure e cast inválido ancorados no span original depois de
clone/walk/reflow; `Pending` incapaz de fornecer `PagedDocument`; e ausência
total de `Engine`/L3/dyn execution no módulo. Integração negativa tenta
exportar a primeira passagem em PDF, PNG e SVG e deve falhar antes do exporter.
A mudança pública e a nova passagem eval↔layout exigem selo ADR-0127 antes de
código.
