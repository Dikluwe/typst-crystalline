# P1339 — revisão delimitada do desenho público de `function.where`

Revisão anterior ao candidato, executada por `/root/p1339_where_l0_review`.
Regime: inspeção de desenho e fontes; não é contrato de refinamento, teste
A/B nem veredito de implementação. Não se alega isolamento atestado: o
ambiente de filesystem é compartilhado. Escrita limitada a este diagnóstico;
nenhum L0, teste ou consumer foi alterado por este revisor.

## Proveniência

HEAD observado: `2f42d64253547734564513a1159ee6b584c1c4b4`.
No começo da revisão, `git diff HEAD --stat` estava vazio; `git status --short`
continha apenas artefatos não rastreados P1339 em `diagnosticos` e o passo
explicitamente autorizado. As fontes produtivas abaixo são desse HEAD.
Durante a revisão o autor principal pode editar L0 em paralelo; os hashes
a seguir identificam o L0 antecedente realmente lido, não o rascunho final.

| Entrada | SHA-256 anterior à leitura integral/focal |
|---|---|
| `entities/selector.rs` | `c2d64d1aa8f54c1b027c8166904b67389c39dcb5ee969d8f7c76f3fdb01d44bd` |
| `entities/show.rs` | `b4714e0ab3aac95fbb12daf1a0a50495aa62cf08309f8663cd0dfe4fd06db89f` |
| `entities/func.rs` | `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde` |
| `entities/introspector.rs` | `553f602a5b3c38f2b7c8ff9add6495fe8feeeb37b3a6fc409fcebd2084568f02` |
| `entities/content.rs` | `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98` |
| `compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `compiler/eval/repr.rs` | `a54ee85615e458bbc5b102a53769e4c756e9502d69abe2be1feaed5077e78efb` |
| `compiler/eval/rules.rs` | `d1b2a493682ba0989d397360b5c3781b15a1e130d460f6e6854980bc001b267c` |
| `compiler/stdlib/counter.rs` | `c9058627737fb4726f3fd627e84f5050972b20f817752d01e050d3188f47f270` |
| L0 `entities/selector.md` | `8dcd7bfbc6789ed0ef669d8ffd943df63aa06343f0c4ef39f23e797ec276b429` |
| L0 `entities/show.md` | `2bf028594608a4ee68b0153dd671ef3176b7c28030713c848f00a280c0d6e92a` |
| L0 `compiler/eval/selector_matching.md` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `p1339-full-a2.md` | `68cfd9a20d8ad4b0c5682b7ede7d14b67eb71f7e63ed085b897402a54e7c86c6` |
| `p1339-a1-decision-required.md` | `d2b98674a8444860be93d6998d3a313ac99a308b7fafc445e5f99e2c55f9605d` |
| `p1339-protocol-review-r1.md` | `55596d0e0f26838f5060210a0db6dbc89917d1bb45808ce94bfc9276d4c04f14` |

Paths Rust da tabela são relativos a `01_core/src/`; L0 a
`00_nucleo/prompts/`; diagnósticos a `00_nucleo/diagnosticos/`.
As ADRs 0107, 0109, 0127 e 0129 e a skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md`
com ambas as referências foram lidas. Busca `rg -l segregad 00_nucleo/adr`
não localizou ADR local de materialização segregada. Algumas buscas de
descoberta por símbolos precederam hashes focais; não se alega que todo
snippet de descoberta foi ocultado antes de registrar hashes.

## Medição anterior à avaliação da hipótese

`entities/selector.rs:28–61` não possui identidade para Strong/Emph/Text.
`show.rs:27–28` possui NodeKind Strong/Emph, porém esse enum não é o carrier
de `Value::Selector` (`value.rs:160`). `Where` somente restringe uma base,
e `And` vazio é falso (`selector_matching.rs:168–171`). Portanto nenhuma
composição dessas variantes representa esses elementos sem reinterpretar
regex, label ou outra variante. Não foi encontrada alternativa existente
equivalente à extensão pública proposta.

O L0 vigente `entities/selector.md`, cláusula P1284, congela o enum atual e
veda novas variantes sem gate. O L0 `selector_matching.md` prevê reabertura
por novo QuerySelector. A proposta `Element { function: Func, fields:
EcoVec<(EcoString, Value)> }` cria variante pública e tem gate ADR-0127.
É compatível em princípio com dados puros, enum fechado e ownership 1:1;
este diagnóstico não a autoriza nem materializa.

No vanilla ratificado, `foundations/selector.rs:75–80` guarda o elemento e
`Option<SmallVec<[(u8, Value); 1]>>`; `:308–320` distingue ausência de filtro
de filtro presente vazio. `foundations/func.rs:430–450` normaliza named,
valida nomes de campos e não faz cast dos seus valores. Hashes dessas fontes:
`84615405b19d4869d261029936d3d5d9077f06e7198e3f60f28273c1ecdfcbdc`
e `a119f7e8aae459a8358357f398d44c48bc24783b67891ff3901b8d4b043c6a38`.
O relatório `p1339-full-a2.md:120–139` registra linguagem pública para
heading, figure, strong, emph, raw, text e table; filtro vazio; validação de
campo; spread duplicado; rejeição de With/closure/nativa não-elemento.
Não foram executadas novas sondas por este revisor.

## Inventário dos dispatches afetados

Busca reproduzível: `rg -n 'Selector::|selector::Selector|show::Selector'
01_core/src 02_shell 03_infra 04_wiring -g '*.rs'`, seguida de leituras
focais. O inventário de dispatch abaixo exclui testes e construções de
constantes que não despacham sobre o selector recebido.

| Consumer e ponto | Efeito da nova variante |
|---|---|
| `entities/introspector.rs:634` | Match exaustivo de QuerySelector. Deve ter decisão explícita de execução/limitação para Element; não reutilizar silenciosamente o stub Where. |
| `compiler/eval/repr.rs:1040` | Match exaustivo. Preservar um grupo de campos, inclusive `.where(:)`, apresentação e ordem. |
| `compiler/stdlib/counter.rs:92` | Match exaustivo `selector_to_key`. Existing Where descarta filtros e extrai a base; Element exige decisão de compatibilidade explícita. |
| `compiler/eval/selector_matching.rs:42` | QuerySelector→ShowSelector contém wildcard de rejeição. Nova variante compilaria sem integração e continuaria rejeitada. |
| `compiler/eval/selector_matching.rs:98` | Match exaustivo de ShowSelector. NativeElement precisará reconhecer Content pela identidade nativa aceita. |
| `compiler/eval/selector_matching.rs:193` | Match exaustivo de ShowSelector. NativeElement deve ser node-like para alcançar a travessia de nós. |
| `compiler/eval/rules.rs:476,833` | Reconhecimento especial de Par apenas por `NodeKind(Par)`; risco se NativeElement for ampliado para par. |
| `compiler/eval/rules.rs:639,652,719,883,899,2522` | Caminhos especializados Regex/Text/Label e rejeição show-set textual; NativeElement(Text) deve permanecer semântica de elemento e não cair nesses caminhos por conversão a regex. |
| `compiler/eval/rules.rs:2403–2480` | Construção por Value::Func e Value::Selector; a segunda usa conversão. Preservar as identidades legadas diretas. |

Outros consumers/carriers medidos: `bindings/value_methods.rs:670–748`
constrói Where legado e rejeita filtro vazio; `:767–824` constrói combinadores;
`:860` constrói Within. `stdlib/foundations/selector.rs:263–297` passa
Value::Selector por clone a query/locate; `stdlib/counter.rs:427` e
`layout/references.rs:397–411` reconhecem CounterKey::Selector(Kind) com
fallbacks e não devem passar a receber chaves desconhecidas por acidente.
`entities/counter.rs`, `counter_registry.rs`, layout/heading, equation,
table, footnote e introspect constroem/transportam chaves Kind; `Value`
transporta/deriva igualdade e hash do selector. `03_infra/query_helpers.rs`
usa um enum diferente, ParsedSelector; não é match do QuerySelector público.
Não foi encontrado dispatch produtivo desse enum em L2 ou L4.

## Riscos que o L0 precisa resolver

1. **Identidade versus igualdade.** `Func::native_fn_addr` (`func.rs:316–336`)
   distingue funções nativas e rejeita With/closure; seu consumidor usa
   `fn_addr_eq`. `Func::PartialEq` (`:384–398`) e `Hash` (`:403–418`) usam nome
   para Native e NativeWithEngine. Igualdade de Func não prova que a função
   é um elemento. Validar por identidade nativa e delimitar nomes canônicos
   e aliases aceitos; não reescrever Func::eq incidentalmente. Uma colisão
   pública dentro do conjunto aceito não foi demonstrada.
2. **Filtro agrupado.** EcoVec mantém ordem e permite vazio; normalização de
   nomes duplicados é responsabilidade de Args antes de construir o grupo.
   Não transformar vazio em Kind, nem perder morfologia de um grupo usando
   Where sucessivos na representação pública. Igualdade/ordem exigem decisão
   apoiada nos observáveis medidos, não equivalência lógica presumida.
3. **Show precisa também dos campos.** NativeElement(Func) como base de uma
   cadeia Where pode preservar todos os predicados sem duplicar fields em
   ShowSelector, onde o grupo não tem repr público. Porém o matcher Where
   atual (`selector_matching.rs:159–165`) lê `Content::get_field`;
   `content.rs:3522–3570` não expõe Text.text nem Raw.text/lang. Só adicionar
   a identidade deixaria `text.where(text:"Hi")` sem matching. O L0 deve
   localizar a leitura de campos faltantes no owner adequado, preservando
   a separação entities/compiler e as regras de estilo semântico.
4. **Introspecção já guarda Content.** `TagIntrospector.elements`
   (`introspector.rs:372`) contém `IntrospectedContent` com Content e snapshot
   opcional de campos (`value.rs:26–51`); o walk registra esse carrier
   (`introspect.rs:1348–1360`), com snapshot de Heading; `from_tags.rs:465`
   enriquece Equation. O comentário histórico Where que diz haver somente
   ElementPayload já não justifica impossibilidade geral de filtragem.
   Isso não torna text/strong/emph locatable automaticamente. Query precisa
   preservar índices, ordem e fronteira de locatability existentes.
5. **Helper de identidade existente não é universal.**
   `bindings/field_access.rs:1146–1166` tem `content_elem_func` privado,
   mapeando Text/Strong/Emph/Heading/Raw/Figure e outros. O fallback produz
   função por nome com chamada não suportada. Portanto esse helper não
   prova identidade para todos os Content nem legitima ampliar a aceitação
   de where a partir apenas de `content.func().name()`.

Avaliação preliminar: a dupla QuerySelector::Element +
ShowSelector::NativeElement é representacionalmente viável, condicionada às
decisões acima. Não foi encontrada necessidade de registry, vtable, novo
ElementKind ou deslocamento de fase. Gate humano da variante pública,
contrato discriminatório, selo, implementação e gates finais permanecem
pendentes; este parecer não declara PASS de implementação.

## Adendo — observáveis de grupo e igualdade entregues pelo autor

Após a revisão preliminar, li o recibo
`p1339-where-l0-vanilla.json`, SHA-256
`0f59dfa5bf2d2e5538cc4aad90116824af4593eacb05d4db26dbd23452468455`,
que referencia o manifesto
`fb5833583245cd3d67af199c124117c3e099027afac5f04f538a0556375cf09f`.
Seu estado está em HEAD acima, `diff_stat` vazio, UTC
`2026-09-10T00:04:21.279738+00:00`, binário vanilla ratificado com hash
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
As linhas normal/reverse observam: reordenar campos altera repr e produz
igualdade falsa; spread mantém a posição primeira do campo e substitui o
valor; filtro vazio difere de selector nu; alias de função mantém identidade;
campos Int(1)/Float(1) comparam iguais; selector contendo NaN compara falso
consigo mesmo. Esse recibo resolve as dúvidas focais de grupo/ordem acima.

Decisão de desenho plausível comunicada pelo autor: manter PartialEq/Hash
estruturais no carrier e implementar igualdade da linguagem no owner
`compiler/eval/operators/equality`, sem alterar Func::eq. A separação é
coerente com ADR-0107 e com esses observáveis. O proprietário de igualdade
precisará especificar também recursão em combinadores/Where/Within e valores
aninhados, sem curto-circuito por identidade que tornasse NaN reflexivo.
Não avaliei rascunho L0 final nesta fase e não há código candidato.
