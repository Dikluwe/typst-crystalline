# L0 — extração exata de cobertura cmap por intervalos
Hash do Código: 0a13b3af

Camada L3. Consumer produtivo exclusivo: `03_infra/src/fonts/coverage.rs`. Pacote ratificado pelo dono com `confirmo` em2026-10-07; owner canônico vigente.

## Medição anterior à decisão

`lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/{measurement-direct,measurement,execution}.json.gz` conserva programa/receita, fontes, binários, HEAD/dirty tree, UTC, argv e amostras. O experimento reduziu extração/união isoladas de53.19 para10.85ms; sete testes sintéticos passaram. Nas1112 faces instaladas daquele estado, havia1520598 intervalos/3582 subtabelas nativas e nenhum fallback. Não são contagens atuais nem ganho da CLI. `lab/results/construction/diagnostics/coverage-ranges-preparacao-2026-10-07/audit.json` confirma o consumer puro byte-idêntico à medição e o extrator antigo L3 byte-idêntico ao mesmo HEAD limpo. O resto da árvore e o release atual diferem; precisam de nova medição após implementação.

Fontes primárias lidas: ttf-parser0.25.1, `src/tables/cmap/format4.rs:103–115`, `format12.rs:68–75`, `format13.rs:41–48`; callbacks enumeram limites declarados, sem filtrar glyph0. `cmap/mod.rs:196–219,251–260` usa get tipado, offsets relativos à cmap e encerra o iterador no primeiro registro não parseável. Format4 parse usa segCountX2/2 e exige seus arrays completos; o campo length não autoriza aqui uma validação incompatível adicional. Vanilla pin9f2b6e8715237cb086899a42873660fe744622e8, `text/font/info.rs:119–134`, enumera e une pontos; não tem o mecanismo direto que será implementado.

## Decisão e fronteira

Usar segmentos nativos somente nas variantes4/12/13 aceitas pelo parser tipado. Manter classificação Unicode, ordem/terminação de subtabelas e todos os membros declarados do extrator existente. Bytes, validação e seleção do caminho são exclusivos desta unidade L3; união/codificação do conjunto é exclusivamente de `00_nucleo/prompts/entities/font_book.md`, via Coverage::from_ranges. Normalização neutra não importa ttf-parser em L1. Não inferir intenção upstream nem trocar a semântica por suporte aproximado de blocos Unicode.

Não criar API pública, counters de telemetria, flags, imports de lab, unsafe, globals ou política nova de catálogo/caches. Parent `00_nucleo/prompts/infra/fonts.md` conserva o facade extract_coverage(&Face)->Coverage e delega ao filho privado. I/O/catalog/discovery/shaping/FontInfo permanecem com seus owners; o adapter só lê bytes já recebidos da Face.

## Contrato privado do adapter

1. Face sem cmap produz Coverage vazia, como o caminho antigo. Consultar cmap.subtables.get(i), em ordem; se falhar, terminar ali, sem saltar para um registro posterior. Ignorar subtables não Unicode pela classificação tipada vigente.
2. Em cada subtable Unicode, selecionar formato nativo pela enumeração tipada. Ler a tabela raw cmap da mesma Face e seu record offset correspondente; validar formato bruto, acesso e aritmética com checked_add/checked_mul e slices.get. Não tratar os bytes de uma subtable diferente como sua origem.
3. Formato4 lê start/end dos segmentos declarados; converter limites inclusivos para Range semiaberto. start>end contribui vazio. Parar no singleton FFFF, inclusive quando há segmentos posteriores. Glyph0/idDelta/idRangeOffset não filtram membros: são campos de mapping, não o critério do callback codepoints existente.
4. Formatos12/13 leem grupos declarados em sua ordem, preservando overlap/duplicatas e buracos. Grupo start>end é vazio. End inclusive converte por checked_add(1), sem saturação, clamp ou wrapping.
5. Guardar checkpoint por subtable. Qualquer falha de leitura/validação nativa reverte integralmente o que aquela tentativa anexou e usa seu callback tipado existente. Outros formatos Unicode usam esse callback; não virar suporte nativo adicional sem nova medição.
6. Compactar pontos de fallback em intervalos conectados com acumulador local; ordem arbitrária/duplicatas não alteram a união. Um endpoint exclusivo não representável sinaliza que esta Face precisa do extrator legado inteiro; descartar a tentativa parcial e enumerar a Face na fronteira antiga. Não fabricar intervalo vazio ou perder membros. O domínio Unicode suportado do point constructor fica inalterado; valores inválidos fora desse domínio não recebem promessa nova.
7. Ao terminar coleta válida, usar uma chamada Coverage::from_ranges. O resultado preserva membership/iter/fallback order/canonical equality/hash e não muda indexação de slots. Não guardar referências à Face nos valores L1.
8. Todo output é Coverage própria. Não executar shaping/export, reler arquivo, iniciar World ou forçar catálogo dentro deste adapter.

## Aceitação e testes antes de código

Escrever testes no próprio módulo contra um oráculo independente de membros/BTreeSet e, como controle adicional, o callback legado. Cobrirem4 com glyph0/sentinela/múltiplos segmentos/invertidos;12/13 com buracos/overlap/duplicatas/zero/Unicode máximo; não Unicode; formatos6 e outro callback-only; cmap ausente; registro inválido seguido de válido; truncamento/count/offset/formato incoerente com rollback; endpoint exclusivo não representável no coletor privado. Não usar velocidade como threshold de unit test nem exigir nova política para fonte malformada.

Comparar Coverage de cada face instalada/embutida/fixtures com o caminho antigo fora dos timers. Registrar arquivos e índices, exact hashes e casos de fallback. Preservar seleção de fontes/fallback/estilo, textos/glifos/posições e mensagens de erro na CLI/PDF, incluindo fontes com cobertura ausente e texto multiescrita. Sem font asset equivalente, classificar a lacuna em vez de alegar paridade.

Após GREEN e resselo restrito: builds, suites core/infra/shell/query, linter sem erros e owners sem V5/V6/V15/V26; benchmark de extração/catalog/CLI com executáveis congelados atual/anterior/vanilla pinado, três rodadas com gate/monitor e estado/fontes/argv/UTC completos. Critério de performance é ganho repetível em trabalho/saída equivalentes, incluindo RSS, não extrapolação dos42ms do experimento. Preservar resultados negativos e a árvore recebida; sem commit/reset ou otimização de leitura seletiva da prioridade3 neste pacote.


Ratificação: dono confirmou o pacote coverage-ranges-2026-10-07 com `confirmo` em2026-10-07; manifesto f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549.

## Reuso privado de payloads nativos cmap (2026-10-10)

### Medição antes da decisão

`03_infra/src/fonts/coverage.rs:76–105` extrai cada subtabela Unicode mesmo quando mais de um encoding record aponta ao mesmo offset bruto/formato da mesma cmap. `lab/results/construction/diagnostics/cmap-payload-reuse-2026-10-10/prototype-probe.json.gz` conserva fontes/copias integrais, hashes, HEAD/dirty tree, UTC, fontes/assets, argv, receita e amostras de um protótipo diagnóstico:1.140 faces,1.111 com payloads nativos duplicados,1.819 registros nativos repetidos;1.533.618→767.936 intervalos. Cobertura de cada face igual ao caminho recebido e ao callback legado. Mediana isolada por corpus17,7808196→11,81227295ms (três rodadas, oito pares, dez corpora por amostra); não é ganho da CLI nem promessa de RSS. Fontes não mudaram entre auditorias.

### Decisão e limites

Cobertura como conjunto/iter/fallback é superfície de língua; reextrair o mesmo payload imutável para a mesma união é mecânica (ADR0107/0108). Não inferir intenção upstream. Hipótese de menor custo total exige CLI/RSS pareados; será refutada por divergência de membership, fronteira de terminação/fallback ou regressão persistente sem benefício combinado. Alteração privada reversível em L3, fluxo contínuo ADR0127, sem novo cache no catálogo, API, modo, flag, fase ou representação L1.

Dentro de uma chamada collect, manter no máximo oito chaves nativas bem-sucedidas em array local na pilha, sem alocação heap. Chave=(offset relativo à cmap da mesma Face, formato nativo4/12/13). Continuar consultando get(i) tipado em ordem e aplicar is_unicode antes de qualquer consulta à lista. Continuar verificando offset bruto/formato/acesso da origem correspondente. Apenas depois dessas validações, uma chave nativa já extraída com sucesso dispensa nova native append. Guardar a chave somente após extração nativa integral bem-sucedida. Se a lista encher, processar as demais normalmente e sem memorizá-las; não abandonar subtabelas nem mudar o limite de terminação.

Nunca deduplicar somente por conteúdo/format, ignorar subtabela com offset distinto, memorizar tentativa nativa parcial/falha ou callback-only, cruzar Faces/arquivos, persistir referencias ou trocar união/codificação do owner L1. Os checkpoints, rollback, callback, overflow→legado e os oito contratos acima permanecem vigentes. Mesmo payload nativo válido possui os mesmos intervalos; eliminar repetições preserva sua união, independentemente de plataforma/encoding Unicode.

### Testes e aceitação

Testes antes do código: aliases Unicode consecutivos e separados por outro formato, payloads idênticos em offsets distintos, marcador inválido entre aliases, alias não Unicode, mais de oito payloads únicos e duplicatas posteriores; preservar oráculo independente de membros e callback legado. RED de execução deve mostrar a coleta duplicada no baseline. Contagem de intervalos é controle mecânico suplementar, não definição da língua. Revalidar corpus real completo, fallback e documentos/glifos/posições/raster/query. Resselo somente deste owner. Builds e lint com exit0 sem V5/V15/V26 do owner; preservar avisos recebidos. Não mudar o helper de aquisição seletiva neste pacote.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/fonts/coverage.rs` nas linhas 155 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/fonts/coverage.rs:216 (clippy::manual_is_multiple_of). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Somente no preparador de fontes sintéticas de teste, substituir len % 4 != 0 por !len.is_multiple_of(4). Preservar padding de zeros, offset/length e conteúdo cmap. Não alterar extração/união produtiva nem os arrays unitários de Range neste lote.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Nos testes alias_memoization_respects_invalid_and_nonunicode_records e checked_native_reader_rejects_truncation_after_a_partial_group, os arrays/Vec são coleções de Range<u32>, não coleções dos inteiros abrangidos. Manter literalmente [2..6], vec![40..42], [40..42], seus limites, checkpoint, append parcial e rollback. Autorizar #[expect(clippy::single_range_in_vec_init, reason="motivo nominal de intervalo")] somente nesses dois testes, com os motivos completos registrados no plano desta etapa. A primeira anotação cobre os dois esperados [2..6]; a segunda, a fixture e esperado [40..42]. Não trocar tipos, enumerar pontos, criar helper de construção apenas para evitar o lint, modificar a extração de cobertura ou suprimir no módulo. Se deixar de existir a coleção de um intervalo, rever/remover a expectativa; unfulfilled_lint_expectations continua erro.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.

Prescrições literais deste owner:

- `alias_memoization_respects_invalid_and_nonunicode_records`: `#[expect(clippy::single_range_in_vec_init, reason = "Esperados são coleções de Range semiabertos de cobertura, preservando o intervalo único 2..6")]`.
- `checked_native_reader_rejects_truncation_after_a_partial_group`: `#[expect(clippy::single_range_in_vec_init, reason = "Fixture e esperado guardam um Range 40..42 para verificar append parcial e rollback de intervalos")]`.
