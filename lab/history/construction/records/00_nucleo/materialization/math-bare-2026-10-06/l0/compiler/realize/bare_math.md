# L0 — compiler/realize/bare_math
Hash do Código: pending

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff

**Layer:** L1.
**Exclusive consumer:** `01_core/src/compiler/realize/bare_math.rs`.
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
00_nucleo/diagnosticos/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
00_nucleo/diagnosticos/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Owner puro da normalização de inserção matemática. Descendente de compiler/realize,
sem imports reversos entities→compiler. Expor apenas helper interno ao compiler:

```rust
pub(in crate::compiler) fn prepare(
    content: &crate::entities::content::Content,
    styles: &crate::entities::style_chain::StyleChain,
) -> crate::entities::source_result::SourceResult<crate::entities::content::Content>;
```

Percorrer por fronteiras antes dos filhos usando o helper interno pré-ordem do
owner traversal. Equation existente é fronteira matemática: manter corpo sem
Equation adicional. Em Mathy nu suportado, envolver a raiz uma vez com
Content::equation(body_original, block_efetivo), sem normalizar seus descendentes.
block vem de equation.block booleano do escopo de inserção, default false;
numbering, number-align, supplement e alt conservam o transporte estilístico
existente, sem campos públicos novos. O helper não aloca Location nem chama Func;
identidade e aplicação de regra pertencem aos coordenadores atuais.

Usar classificação fechada por variante do enum e catálogo Mathy medido da fonte
pinada, não todo nome iniciado por Math. Frac/Matrix/Root/Lr/Mid/Attach/Primes/
Stretch/Op/Cases/Vec/Accent/Cancel/Underover e seus carriers concretos implementados
são estruturais. MathIdent/MathText isolados não equivalem a Mathy upstream;
MathSequence é fluxo a percorrer, não Equation sintética envolvendo tudo.
MathStyled é estilo transparente: texto isolado fica texto; descendente Mathy
fora de Equation vira Equation. ClassOverride/LimitsOverride representam os
wrappers Mathy da fonte quando efetivamente existentes no enum. Confirmar cada
entrada com file:line e teste pertinente antes da materialização; variante sem
correspondência não recebe aceitação por semelhança do nome.

Styled conserva o wrapper e desce com a chain sobreposta; ShowRecipe usa seu
snapshot lexical próprio e conserva regras/guards/file/textual/context_id_floor.
Realized conserva original intocado e processa somente o visual ainda não
preparado. Located preparado conserva sua Location e não é reinterpretado como
inserção nova. Labels ficam no elemento original; não se promovem à Equation
criada. Sequence, containers de conteúdo comum e contribuições contextual/show
visitam as inserções reais, preservando ordem e regiões matemáticas.

Não alterar constructors, repr ou Value cru. Não juntar duas raízes Mathy em
uma Equation por igualdade/adjacência e não envolver conteúdo normal por ter
font/math style. Idempotência: uma árvore preparada não ganha equações extras.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.
