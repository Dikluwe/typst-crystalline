# Bidi: contexto completo e origem dos fragmentos — 2026-10-05

A auditoria confirmou que o bidi precisa da entrada lógica completa do parágrafo, além da direção-base. Preparado um pacote L0 para transportar esse contexto e os intervalos originais dos fragmentos. **Nenhum código produtivo foi alterado nesta etapa.** A materialização aguarda confirmação ADR-0127 porque acrescenta módulo/API de entidade e campos públicos em FrameItem.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [initial.json.gz](bidi-contexto-2026-10-05/initial.json.gz) registra UTC, status, lista exata `git diff HEAD --stat`, fontes produtivas/L0 herdadas, comandos, textos/caixas e hashes dos binários. Tekt medido: `64e35e6b7e69881b44c287daac6bc0d3bb668993f4019e97e1002909edcc8e27`; cópia em `/tmp/tekt-bidi-contexto/baseline-typst`. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, receita/árvore ratificadas no P1489. CLI com `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`. Não se usou a string de versão como prova.

[probe.json.gz](bidi-contexto-2026-10-05/probe.json.gz) registra a sonda independente, seu código, receita rustc, versão, hashes do executável e rlibs unicode-bidi/serde_json, UTC e estado da árvore. Não é código L1/L2/L3 nem candidato produtivo. O código da sonda está em [probe.rs](bidi-contexto-2026-10-05/probe.rs).

## Medir antes de decidir

Treze fontes CLI foram comparadas com fonte DejaVu Sans, página 220pt e margens 20pt. Os dados completos estão no recibo inicial; estes exemplos isolam responsabilidades:

| Caso | Tekt | Vanilla |
|---|---|---|
| Base RTL, `ABC 123 مرحبا` | ABC em x=124,020510, árabe à direita | árabe à esquerda, ABC em x=152,949220 |
| Base LTR, `مرحبا ABC 123` | latim à esquerda, árabe à direita | árabe à esquerda, latim à direita |
| Base LTR, `123 456 مرحبا` | árabe, 456, 123 na ordem visual | 123, 456, árabe |
| Base RTL, prefixo árabe e trecho LTR explícito | posições próximas do vanilla | controle positivo |
| Um único trecho LTR sobre o parágrafo inteiro | à esquerda | à esquerda |

`layout_bidi.rs:73–82` detecta base pelo primeiro caractere forte da linha. Em `:227–240`, qualquer item com style.dir RTL força o default RTL da resolução; isso confunde direção-base e direção dos trechos. `shaper/bidi.rs:34` resolve novamente cada string com base automática. A direção privada que agora estabiliza a âncora em L1 não chega a esses consumers.

No vanilla pinado, `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados, e `inline/collect.rs:156–178` acrescenta embeddings auxiliares para TextElem com direção diferente da base. São duas responsabilidades distintas; não implica copiar sua representação de frames ou bytes PDF.

### A direção-base sozinha foi refutada

A sonda de biblioteca usa `ABC 123 456 789 مرحبا` com base RTL e consulta o intervalo original da futura linha `456 789`. Sobre o **parágrafo completo**, esse intervalo é um run LTR único `456 789`. Sobre a **string isolada**, com a mesma base RTL, os runs visuais são `789`, espaço RTL, `456`. Os caracteres fortes anteriores à linha influenciam a resolução de números/neutros. Portanto, adicionar somente `paragraph_dir` e continuar resolvendo strings fragmentadas não corrigiria o wrap.

Essa sonda mede níveis/runs e byte ranges do algoritmo, não certifica geometria ou língua do Tekt. Seus resultados fundamentam a necessidade de contexto; a futura aceitação continua fonte→PDF contra vanilla.

As sondas com RLM/embeddings autorais também produziram agrupamentos e controles extraídos diferentes no Tekt. Inserir marcas no texto para forçar o detector seria uma alteração do conteúdo e não fecha origem, shaping ou extração. Os controles **auxiliares** do resolver devem ter mapa para bytes originais e nunca chegar ao texto copiável/glifos/ToUnicode. Controles que já pertencem ao texto autoral não podem ser removidos para declarar paridade.

## Contrato pronto para revisão

[Manifesto L0](../materialization/bidi-contexto-l0-manifest-2026-10-05.json), SHA-256:

`1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043`

O manifesto fixa os hashes completos de três prompts:

- [entities/bidi_context.md](../prompts/entities/bidi_context.md): consumer futuro único `01_core/src/entities/bidi_context.rs`. Dados imutáveis de parágrafo e span, validação de direção/ranges/UTF-8, construtores e getters públicos.
- [entities/layout_types.md](../prompts/entities/layout_types.md): campo opcional `bidi: Option<BidiSpan>` em Text, TextSeparator e TextShaped. TextStyle permanece com a direção do trecho, sem receber contexto de parágrafo.
- [entities/mod.md](../prompts/entities/mod.md): declaração estática do novo módulo público, sem lógica de domínio/reexports extras.

```mermaid
flowchart LR
    P[Parágrafo textual certificado em L1] --> I[Entrada lógica imutável compartilhada]
    I --> S[Fragmento: segmento e intervalo UTF-8 original]
    S --> B[Bidi em L3: parágrafo completo e ranges de linha]
    B --> G[Shaping: runs com a mesma origem lógica]
```

A entrada contém base e segmentos Text com direção efetiva, espaços reais com origem e quebras explícitas. Cada span retém Arc da entrada, índice do segmento e range **original**, validado. Tradução e reordenação mudam posições, não a origem. Split usa subspan relativo validado; palavras repetidas não são remapeadas por igualdade de string. Merge não apaga ranges distintos. Clones/checkpoints conservam a referência imutável.

A resolução Unicode permanece no L3 nesta proposta, mantendo a fase vigente e a whitelist L1. Preparar os dados lógicos em L1 não muda a fase do algoritmo. Uma eventual migração da parte pura de Unicode para L1 exigiria proposta própria, revisão da whitelist e confirmação de mudança de fase; esta etapa não a ratifica nem a descarta por princípio. Não manter a atual mecânica só por herança, mas isolar primeiro a perda comprovada de informação.

O resolver L3 pode reutilizar uma entrada por compilação com registro proprietário e views emprestadas em variáveis separadas. Não se propõe self-reference, unsafe, global mutável ou cache entre watch/compilações. A entidade não carrega fontes nem cálculos de glyphs; controles virtuais pertencem à preparação L3. Necessidade futura de ampliar FontMetrics deve ser medida e passar por L0/gate próprio.

## Migração e limites

[inventory.json](bidi-contexto-2026-10-05/inventory.json) identifica **72 arquivos/owners e 879 referências textuais** às três variantes. Inclui constructors, patterns, testes e comentários; não são 879 mudanças obrigatórias. O compilador e a auditoria de cada owner definem a migração concreta. Todas essas unidades já têm @prompt canônico. O inventário registra hashes/linhas para tornar a revisão reproduzível.

Campos opcionais usarão None em produtores sem certificação e frames manuais legados, com L0 atualizado antes de cada alteração de código. Não adicionar variante a SemanticKind/FrameItem ou método ao visitor. Adicionar campo a variantes públicas altera struct literals/patterns Rust: não afirmar compatibilidade automática. O produto Typst/CLI mantém defaults, e a certificação inicial fica no subconjunto textual integral. Objetos, matemática, links, subframes/colunas, transforms sem mapa exato, hifenização e demais caminhos recusados permanecem explícitos.

A auditoria do cache de frames deve provar que um span carregado é independente do contexto exterior ou que esse contexto faz parte da validade de reúso. Igualdade de texto/caixa não basta. Não modificar chaves de fonte por meio de TextStyle para esconder dependências de parágrafo.

Após confirmação: atualizar os L0 internos dos produtores/consumers, introduzir os testes RED, materializar dados/migração/transporte/resolução e verificar GREEN, idioma/morfologia/codepoints/caixas, sentinelas de rollback/regiões e custo por parágrafo. Comparar explicitamente espaços finais e a borda oposta do wrap; não atribuir todos os defeitos ao bidi.

## Estado e gate

O pacote está **proposto, não materializado**. Foram alterados somente os L0 públicos e produzidas evidências/documentos; a fonte produtiva e o SHA do binário permaneceram iguais ao começo desta auditoria. Não executar resselo sobre código antigo para esconder a proposta. O novo owner ainda não possui consumer produtivo; ownership/linhagem só serão revalidados após sua criação autorizada. Não se afirma lint limpo, teste RED→GREEN produtivo ou ganho de tempo nesta etapa.

ADR-0127 exige parada para “adicionar, remover ou alterar campo público de qualquer entidade” e “alterar assinatura pública de função L1–L4”. Aplicação concreta: três variantes FrameItem ganham campo e o novo módulo expõe uma API validada. A autorização geral para continuar a frente não substitui a confirmação desse contrato específico. Ver [ADR-0127](../adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md). Código permanece aguardando confirmação do manifesto acima.
