# L0 — infra/shaper/text_identity
Hash do Código: 079c3a40

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/text_identity.rs`

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

## Responsabilidade exclusiva

Own private protect_authored_shy(Vec<FrameItem>) -> Vec<FrameItem>. Para
cada TextShaped com texto visual contendo U+00AD autoral, envolver o item
em TextReplacement com text clonado da ocorrência e um único filho original.
Outros itens ficam idênticos. Não resolver fontes, executar shape, medir, filtrar
glifos emitidos, transformar coordenadas ou percorrer wrappers.
Own também configure_authored_shy_buffer(&mut UnicodeBuffer, &str) privado,
que aplica a flag documentada conforme a cláusula abaixo. O caller só chama
essa unidade fora de envelope existente. U+00AD é a identidade normativa
do caractere, sem exceção de fixture, família, GID ou largura.

## Aceitação

Testes causais em shaper/tests usam fixture Noto versionada e comprovam
texto lógico único, glifos/avanços/posições preservados e não aninhamento.
Sondas fonte→PDF devem conservar SHY por ocorrência e espaços/hífens
literais, incluindo local/top e compacto/verboso. Seleção estreita e fallback
sem fonte não são certificados. Ownership 1:1; dependências não ampliam owner.

## SHY autoral — controle de glifos invisíveis

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/first-round/after/`
refuta a primeira materialização: trailing e ligature conservam os codepoints
mas ampliam a caixa de ActualText em 3.12pt e perdem o espaço na extração comum.
O caso tracking também inclui um intervalo derivado do placeholder invisível.
`first-round/final-receipt.json` conserva o estado e as falhas.
Rustybuzz 0.20.1, src/lib.rs:32-35, documenta que o tratamento padrão troca
Default_Ignorable por glifo de espaço com avanço zero; REMOVE_DEFAULT_IGNORABLES
remove esse placeholder depois do shaping. Um cluster compartilhado de f+SHY+fi
também pode dar hex vazio ao GID de espaço, bloqueando seu mapeamento posterior.
A decisão é refutável por alteração de glyphs visíveis/ligaduras no controle
com a flag documentada ou por contaminação de runs sem SHY.

### Responsabilidade

Configurar REMOVE_DEFAULT_IGNORABLES somente nos sub-runs cujo texto autoral
contém U+00AD, em medição e emissão. Delegar a configuração a text_identity,
antes de shape; não remover SHY do texto de entrada, não filtrar por GID
ou avanço zero e não alterar shaping de runs sem SHY. Os caracteres lógicos
completos continuam na ocorrência. A flag remove placeholders, não glyphs
visíveis de seleção de quebra. Tracking incide nos clusters que restam e
não acrescenta intervalo por um placeholder que não existe mais.
Cache permanece por texto completo; a política é determinística pelo texto.
Não mudar assinatura pública, fase, fallback, features OpenType ou fontes.
Texto exclusivamente invisível e seleção de quebras continuam fora do escopo.
