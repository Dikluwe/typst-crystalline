# L0 — stdlib/layout/casting — conversões de medidas e lados
Hash do Código: 6436e728

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/casting.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contém a lógica desta família em `01_core/src/compiler/stdlib/layout.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem nova prova bilateral contra o vanilla. As decisões históricas, medidas e qualificações do pin continuam em `00_nucleo/prompts/compiler/stdlib/layout.md`.

## Contrato de linguagem

Conversores de length, largura/altura de block e box, sides, corners e stroke preservam casts, defaults, precedência, tipos relativos e diagnósticos. extract_stroke e extract_corners_length_value continuam disponíveis aos consumers estruturais.

A extração preserva assinatura, identidade de função, sintaxe, morfologia, erros e precedência de validação. O hub e o registro continuam a encaminhar chamadas estaticamente, sem duplicação dos algoritmos. Não há mudança de contrato público, comportamento por defeito ou fase do pipeline.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. A paridade é da linguagem, segundo ADR-0107/0108; a mecânica pode divergir. Owner exclusivo conforme ADR-0129/0130.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: extrair Stroke próprio pela referência ao payload compartilhado; assinatura e tipos de retorno continuam Stroke, incluindo overhang, paint, cap/join/dash/miter/specification.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Domínio de figura e medidas — 2026-10-17

### Medição anterior à decisão

00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17/before.json.gz e
before-expanded-measurement.json.gz fixam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC, status/diffstat e fontes/L0/config/binários.
São 130 pares CLI. O pin é 9f2b6e8715237cb086899a42873660fe744622e8,
binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
figure_image.rs:68-105 silencia body inválido como Empty, caption inválida como
nome do tipo e kind inválido como inferência. O pin rejeita essas entradas;
body precede caption, que precede kind, mesmo se a ordem nomeada for invertida.
kind aceita string/auto/função de elemento, mas não função arbitrária/With.
O pin valida FigureKind em model/figure.rs:666-681 como Element ou EcoString.

casting.rs:24-53 conserva Rel na largura de block, mas extract_length reduz
Relative à parte absoluta e Ratio a zero. h/box aceitam números onde o pin
rejeita; .fields() também é incompleto. Isto mede dívida real, não prova
intenção upstream nem autoriza trocar percentuais por erro. Os carriers
públicos Spacing::Absolute(Length), BoxedElem.width:Option<Length> e
FigureElem.kind:Option<String> não transportam todos os dados de linguagem.
Essa evolução pública e eventual síntese de figure exigem proposta ADR-0127;
não as simular com nomes, valores neutros ou deslocamento informal de fase.

### Decisão exclusiva — cobertura fechada dos conversores

extract_length e extract_block_width classificam Value exaustivamente. Os
mesmos cinco casos Length/Float/Int/Relative/Ratio conservam seus RHS nesta
etapa. O grupo explícito das outras 34 variantes é coeso por ausência de cast
nesta projeção; retorna None para o caller validar, não um comprimento neutro.
Uma variante futura exige classificação no compilador. Preservar o contrato
atual de block, Fraction em seus owners e os adaptadores height/box. Não
restringir Float/Int ou percentuais globalmente, pois isso afeta callers de
contratos diferentes. A truncagem já medida permanece dívida para a evolução
do carrier público; os dois V16 estruturais não são prova de paridade completa.

### Aceitação

Correção interna/paridade em fluxo contínuo ADR-0127, sem assinatura pública,
default deliberado de produto, compatibilidade Rust ou fase novos. L0 primeiro,
testes funcionais RED para rejeições reais e controles válidos; GREEN exige
mensagens e âncoras causais e precedência sem avaliação repetida. Testes dos
casts controlam comprimentos afins, block relativo e rejeições de famílias;
não cristalizar truncagem como semântica final. Sondas estruturais extraem
padrões produtivos para provar variante futura rejeitada nos dois matches;
payloads stub/RHS apagado não constituem equivalência funcional.

Repetir os 130 pares, distinguir correções intencionais, controles e dívidas
remanescentes, sem declarar todo JSON/fields/reflexão equivalente. Inspecionar
os consumers dos carriers e registrar inventário antes de propor API. V15/V26
válido antes de resselo apenas destes dois owners. Build, suíte workspace,
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings
com quatro índices, linhagem estrita/reversa, diff e SARIF diferencial.
12 GiB rígido, high 10 GiB, swap zero, jobs=1; execução pesada sequencial.
Sem release, merge ou commit/push do Typst. RG1 permanece independente.

### Incompletude e passos de conclusão nomeados

Esta correção não fecha a construção integral de figura nem medidas relativas.
FG1 (00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17/follow-ups.md)
completa identidade tipada de kind e decide síntese/recursão após auditoria dos
owners e gate público; RM1 no mesmo documento completa o transporte de medidas
relativas e a revisão de casts por consumer. São trabalhos pendentes, não L0
materializável nem autorização de API. Não sobrescrever esta etapa/TS1/RG1 com
candidatos antigos ao concluir os passos.
