# L0 — entities/elements/transform
Hash do Código: 2da70039

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

Camada L1. Consumer exclusivo: `01_core/src/entities/elements/transform.rs`.
ADRs 0107, 0108, 0109, 0127, 0128, 0129 e 0130.

## Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

## Contrato tipado autoritativo

Esta seção substitui o carrier matrix/body anterior. Os tipos públicos pertencem
a este único owner; os nomes e campos abaixo fazem parte do contrato Rust:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub operation: TransformOperation,
    pub body: Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformScale {
    Auto,
    Ratio(Ratio),
    Length(Length),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformOperation {
    Matrix(TransformMatrix),
    Move {
        dx: Option<Rel<Length>>,
        dy: Option<Rel<Length>>,
    },
    Rotate {
        angle: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Scale {
        x: Option<TransformScale>,
        y: Option<TransformScale>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Skew {
        ax: Option<Angle>,
        ay: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
}
```

Importar Content, Rel e os tipos geométricos vigentes; não criar outra família
de unidades/alinhamentos. Option representa presença, não default resolvido.
Some(TransformScale::Auto) distingue auto de omissão. Body é obrigatório na
construção linguística e sempre aparece em fields, mesmo vazio.
Não manter matrix redundante ao lado de operation.

## Projeção e preservação

Nome e função nativa derivam da variante, nunca de coeficientes. Ordem lexical:
Move dx,dy,body; Rotate angle,origin,reflow,body; Scale x,y,origin,reflow,body;
Skew ax,ay,origin,reflow,body. Omitir apenas campos None na projeção lexical.
get_field devolve valores explicitamente conhecidos; o classificador de métodos
separa declarado/omitido de não declarado. Nenhum default de layout aparece como
campo explícito. Identidade e igualdade de linguagem distinguem variantes e
presença; não comparar somente a matriz resultante. Observação conservadora não
reutiliza resultados de operações/presenças diferentes como se fossem iguais.

plain_text delega ao body; is_empty mantém o default false do Element. Clone,
Hash manual compatível com igualdade, map_content/map_text e reconstruções
preservam operation integralmente. O despacho de layout continua em
`00_nucleo/prompts/core/compiler/layout/transform.md`, sem import reverso ou lógica
de render na entidade. Campos não promovem a família a locatable por padrão.

## Compatibilidade Rust e Matrix

O campo público matrix é removido; literais externos e leituras desse campo
precisam migrar para operation. Para o uso matricial genérico, escrever
operation: TransformOperation::Matrix(matrix). A assinatura pública existente
Content::transform(matrix, body) permanece e constrói Matrix. Matrix conserva
a semântica genérica anterior, sem se anunciar como move/rotate/scale/skew,
sem herdar propriedades nativas dessas funções e sem novo construtor público.
Native constructors passam a criar as variantes tipadas diretamente.

## Aceitação

RED→GREEN de identidade, omissão/zero/false/auto, eixos parciais, units e mapas.
Integração deve cobrir construtores, set/show, getters contextuais, serialização,
layout e controles por target. Build verde sozinho não certifica a migração.
Sem I/O, estado global, vtable, PropMap de entidade ou inferência por render.

Hash não pode reutilizar Debug cru: a igualdade ignora o sinal do zero e
wrappers transparentes do body. Canonicalizar zero nos parâmetros e usar
projeção conservadora do texto do body; colisões entre bodies não textuais são
permitidas, jamais tratar colisão como igualdade. Refutar por -0/+0 e Source.
