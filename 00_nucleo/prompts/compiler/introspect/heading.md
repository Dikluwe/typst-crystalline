# Prompt L0 — `heading`
Hash do Código: 0a6eefb1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`entities/elements/heading.rs:24–52` distingue nível e máscara de presença;
`compiler/introspect.rs:1327–1350` ainda dispõe da chain. Fonte ratificada
`model/heading.rs:249–285` sintetiza level/supplement;
`text/lang.rs:619–654` localiza pela língua/região e fallback inglês.
As translations ratificadas `crates/typst-library/translations/*.txt`
definem heading; por exemplo en:5 Section, pt:5 Seção, fr:5 Chapitre.

Medição adicional de fonte: Content::heading_numbered em
`entities/content.rs:1500–1501` cria gate true sem pattern para fixtures;
`compiler/layout/heading.rs:85–104` usa nesse caso um prefixo legado, não um
pattern recuperável.

### Decisão proprietária

Acrescentar helper puro `pub(super)` da feature, chamado pelo walk magro,
que recebe `&HeadingElem`, `&StyleChain` e `Option<&Label>` e devolve
`IndexMap<EcoString, Value, FxBuildHasher>`. Não há API pública de callback,
Engine, acesso a filesystem/lab em runtime ou import reverso.

A ordem e os valores para o domínio produtor atualmente modelado são:

| Campo | Origem/valor |
|---|---|
| level | Inteiro h.level já representado pelo produtor. |
| depth | h.level se HEADING_SET_DEPTH; 1 no constructor sem depth modelado. |
| offset | Inteiro 0: offset custom não está modelado neste recorte. |
| numbering | String exata somente quando gate heading.numbering é true e heading.numbering.pattern é Str; nos demais estados, None. Não usar só o pattern: set none pode deixar um pattern ancestral. |
| supplement | Content::Text com nome localizado de Heading para a língua causal; auto é resolvido aqui, não no encoder. |
| outlined | Bool h.outlined. |
| bookmarked | Bool explícito h.bookmarked ou Auto se None, não is_bookmarked(). |
| hanging-indent | Auto: custom não modelado neste recorte. |
| body | Value::Content do mesmo h.body congelado. |
| label | Value::Label da label causal, somente quando presente. |

Decisão explícita para o estado interno medido de gate sem pattern: expor
numbering None no snapshot, mantendo o gate e o render legado inalterados.
Não afirmar paridade desse input sintético nem que None descreve seu render;
é ausência de pattern de linguagem modelado, não inferência de string pelo
prefixo. Teste de domínio fixa essa projeção e a independência do gate.
Os testes obrigatórios de linguagem numerada usam gate+pattern realmente
transportados. Um Func ou tipo interno não modelado também não vira pattern.

Para língua ausente, usar en somente neste helper (não alterar StyleChain::lang
nem ativar hifenização). Para códigos de língua presentes, usar tabela pura
literal transcrita das entradas heading ratificadas, com fallback en quando
não há tradução. Não carregar translations em runtime, não importar lab,
não tratar todo idioma diferente de pt como inglês. Região não transportada
pelo produtor atual continua dívida explícita; não inventar região do ambiente.

Limite deliberado: snapshot completo significa todos os campos do estado
**efetivamente modelado**, não paridade de todo named que o parser tolera.
Este adendo não amplia native_heading/eval rules: supplement custom/none/Func,
numbering Func, set level/depth/offset/outlined/bookmarked/hanging-indent e
precedência de numbering:none no constructor sob set ancestral permanecem
dívidas anteriores. Esses inputs não podem ser usados para alegar paridade;
também não justificam apagar casos R4 obrigatórios de Heading padrão.
Este L0 de transporte é completo no seu recorte; não há constructor novo
parcialmente materializado nem promessa de completar todas essas dívidas em
um passo ainda inexistente.

Aceitação inclui default, markup em mais de um nível, constructor level
explícito, numbering "1"/"I"/set none, língua en/pt e controle de outra
tradução da fonte, label presente/ausente, snapshot anterior preservado após
novo estilo. TOC/counters e suas funções existentes conservam comportamento.

---



**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/introspect/heading.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Computar metadados de heading para TOC automático e explícito, preservando nível, corpo, localização e política de inclusão.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.
