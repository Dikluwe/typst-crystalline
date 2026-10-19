# Prompt L0 — `entities/elements/math_matrix` — `MathMatrixElem`
Hash do Código: 9a045de7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_matrix.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathMatrixElem {
    pub rows:  Vec<Vec<Content>>,
    pub delim: (char, char),
}
```

`Content::MathMatrix { rows, delim }` → `Content::MathMatrix(Arc<MathMatrixElem>)`.
Construtor ergonómico: `Content::math_matrix(rows: Vec<Vec<Content>>, delim: (char, char))`.

## `impl Element for MathMatrixElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | linhas → células `join(", ")`, linhas `join("; ")` (`content.rs:1634`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em cada célula de cada linha, `delim` preservado (`content.rs:2116`): `Content::MathMatrix(Arc::new(MathMatrixElem { rows: new_rows?, delim: self.delim }))` |
| `map_text` | **terminal** (math structural; `content.rs:2625`): `Content::MathMatrix(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `rows + delim` (paridade `content.rs:1826`).

## Critério

`plain_text` células `, ` e linhas `; `; `map_content` recurse na grelha; `delim`
preservado; `map_text` terminal; igualdade estrutural.

## P1431 — carrier público completo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O L0 histórico descreve só `rows + delim`, enquanto o consumer já acumulou
gaps absolutos opcionais e um augment inteiro. Nenhuma das formas conserva a
assinatura vanilla completa ou presença explícita.

### Struct proposta

`MathMatrixElem` preserva:

- `rows: Vec<Vec<Content>>`, já retangularizadas;
- `delim: (Option<char>, Option<char>)`;
- `align: HAlign`;
- `augment: Option<MathMatrixAugment>`;
- `row_gap: Rel<Length>` e `column_gap: Rel<Length>`;
- `explicit: MathMatrixExplicit { delim, align, augment, row_gap, column_gap }`.

`MathMatrixAugment` preserva `hline: Vec<isize>`, `vline: Vec<isize>` e stroke
opcional, onde ausência significa `auto`; offsets continuam assinados até ao
consumer. O bit `augment` distingue omissão de `none` explícito. `gap` não é
campo do carrier: a fronteira expande-o causalmente nos dois gaps.

### Invariantes

Igualdade e hash discriminam todos os campos, valores, ordens e bits.
`plain_text` mantém células por `, ` e linhas por `; `. `map_content` recursa
em cada célula, preservando integralmente os demais campos; `map_text` continua
terminal. Nenhum walker resolve cascata, offsets negativos, stroke auto ou
pontos `&`.

## D07 — correção de morfologia de matriz, fluxo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz guarda
UTC/diff/fontes/binários e casos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Mat raw/query sai como {func:math.matrix} sem rows; getter/métodos perdem campos
e func-equality retorna false. Fonte upstream math/matrix.rs:91–221 declara
propriedades e rows. Tekt MathMatrixElem conserva rows/explicit em math_matrix.rs:45–52,
mas Element.get_field é default; content_methods.rs:151–191 não enumera mat;
cli.rs:1154–1348 cai no fallback body. É morfologia/semântica da linguagem,
não exigência de estrutura Rust ou bytes PDF; nenhuma intenção foi inferida.

### Responsabilidade exclusiva

Implementar o método get_field já existente no trait Element: rows sempre assente; delim/align/augment/row-gap/column-gap somente quando o bit explicit correspondente estiver assente. Projetar arrays de rows/células, delim como par de str/none, align horizontal como Value::Align, augment como Dict hline/vline/stroke (auto quando ausente), gaps como Value::Relative. Projetar padding Empty como Sequence vazia da linguagem, sem alterar carrier/render. Não adicionar campo, método de trait ou assinatura; preservar hash/igualdade/travessia e flags existentes. Supersede apenas o default get_field None histórico para esta unidade.

### Aceitação

RED→GREEN e coleta bilateral de fields/has/at/func, JSON/query, padding e
presença; build/lint e regressões D05/D06/query. Gate contínuo: contratos
públicos de tipos/traits/assinaturas e fase ficam iguais. Não certificar
mat selectors, render, Styled geral ou outros elementos não medidos.

### Canonização de augment medida na fonte

additional-before.json.gz augment conserva inteiro no vanilla. Isso não exige
campo público novo de entrada crua: math/matrix.rs:404–417 converte o carrier
normalizado em inteiro se stroke é auto, hline vazio e vline tem um item; nos
demais casos usa dict com arrays e stroke. Aplicar essa mesma forma observável
no getter, sem copiar SmallVec ou adicionar struct/campo/assinatura. A regra
supersede a descrição genérica anterior de augment sempre como Dict.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Getter augment.stroke projeta pelo From<Stroke> existente; MathMatrixAugment continua usando Option<Stroke> próprio. Preservar integer/dict, auto, arrays, presença, offsets e todos os campos.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_matrix.rs:162` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T285: 01_core/src/entities/elements/math_matrix.rs:127. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir Some(augment) no getter: no corpo manter stroke ausente, hline vazio e vline unitário como Int; demais casos conservam Dict completo, arrays, auto/stroke e ordem. Bits explicit, None e travessia permanecem.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.
