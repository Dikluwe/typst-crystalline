# Qualificação do reúso adiado da stdlib

O candidato passou na qualificação funcional deste corpus, incluindo pacotes publicados e inputs grandes. Há ganho consistente com vários imports, mas **a integração fica condicionada ao residual dos controles pequenos**: a confirmação ainda encontrou +0,74% de CPU na CLI sem imports com inputs grandes. Não há autorização quantitativa para declarar esse custo aceitável. Nenhuma otimização foi integrada, nem foi alterado o candidato recebido.

## Estado e escopo

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-06T13:02:20.473955+00:00`. A lista exata de `git diff HEAD --stat=240` recebida (184 arquivos, 8388 inserções, 537 remoções) é **idêntica** ao apêndice de [stdlib-reuse-2026-11-05.md](stdlib-reuse-2026-11-05.md). As alterações prévias, inclusive fontes, imagens e plugins, fazem parte do baseline. Foram protegidos os bytes de 53169 arquivos recebidos; nenhuma edição em produto, L0, dependências, configuração ou experimentos anteriores foi necessária.

O laboratório novo é [stdlib-qualification](../../lab/experiments/stdlib-qualification/README.md). Aplicou-se o skill [Tekt com código e testes independentes](/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md): A produziu o adaptador/driver, B os testes sem consultar A e C compilou/executou. O helper `snapshot.rs` permaneceu byte a byte igual ao experimento `watch-startup-neutral`. Correções nos harnesses ajustaram uma chamada Rust a `replacen` e o teste privado aos aliases efetivamente presentes no registry; não alteraram o candidato ou seus critérios de validade.

Três ELFs nominais, separados: `bas` (produto recebido), `neu` (controle estático neutro, com slot/helper privado sem reúso) e `laz` (candidato adiado). Builds release offline/locked, Rust 1.92.0, jobs=1, incremental desativado, mesmo lockfile, `.cargo/config.toml` **completo**, incluindo `-C llvm-args=-align-all-functions=3`; perfil recebido `opt-level=s`, thin LTO, codegen-units=1 e exceções por pacote preservadas. Fontes de build, targets, ELFs, testes executáveis, caches e PDFs ficaram exclusivamente em `/dev/shm/tekt-stdlib-qualification-20261106`, sob limites de memória/swap dos launchers. Os builds nominais usaram MemoryHigh/Max=6 GiB e swap=0. O cache RAM anterior foi somente copiado.

## O que se está qualificando

O root continua construindo sua stdlib original. O primeiro import cria o template da compilação; os seguintes só o reutilizam quando coincidem **todos os pares ordenados de inputs e Features completas**. Cada arquivo recebe novas identidades de Module/Element/With, preservando aliases internos. Inputs continuam consultados no mesmo ponto. Não há memoização de Module avaliado, template entre compilações, mudança de World, layout, fontes ou comemo.

As contagens por debugger, fora dos relógios, confirmaram:

| Fixture, com 512 inputs | Builders bas/neu → laz | World.inputs em todos | Source.new em todos |
| --- | ---: | ---: | ---: |
| Sem imports | 1 → 1 | 1 | 1 |
| Fanout de 16 módulos | 17 → 2 | 17 | 17 |
| Oxifmt repetido | 17 → 2 | 17 | 3 |
| Numbly | 2 → 2 | 2 | 2 |
| Suiji | 9 → 2 | 9 | 7 |

O contador de `eval.import` não ficou observável no ELF otimizado: **Unknown**, não zero. Dados em [builder-counts.csv](../../lab/results/construction/diagnostics/stdlib-qualification-2026-11-06/builder-counts.csv).

## Correção: entradas grandes e adversários

- Harness público: **23 testes passaram no baseline e 23 no candidato**, com um benchmark de timing ignorado em cada lado. Abrange 0/1/64/512/4096 pares, mais de 1 MiB UTF-8, Unicode, diferenças no início/meio/fim, reordenação, inclusão/remoção, chaves repetidas, World com respostas distintas por consulta, imports irmãos/aninhados/repetidos, novas avaliações, features/targets, aliases, erros/ciclos/spans/traces, show e isolamento de plugin host.
- Guard privado real: **3 testes passaram**; **3/3 mutações compiladas foram mortas por asserções executadas**: compartilhar identidades, ignorar inputs e ignorar Features. Esse build funcional separado compilou 8356 testes do core, mas executou somente os três do helper em cada variante; não foi uma execução da suíte inteira. Não se ampliou API privada para linkar um harness externo. Como Features é constante num EvalContext nominal, a mutação de Features foi qualificada no domínio privado, sem alegar uma transição pública inalcançável.
- Checker independente: **6 testes Python passaram**, incluindo rejeição de medições/publicações e normalização restrita de paths nos diagnósticos.

Na CLI, o conjunto grande tem 512 pares e 74734 bytes UTF-8 de chaves+valores, transportados por flags existentes `--input`. O conjunto intermediário tem 64 pares/4869 bytes. O driver verifica orçamento de argumentos sem truncar/coagir/ordenar strings. Casos de 4096 pares/mais de 1 MiB foram testados via World/API, sem ultrapassar o limite do sistema operacional por argumento.

## Pacotes reais: aceitação e limites existentes

Foram usados dez alvos publicados: CeTZ 0.5.2, Oxifmt 1.0.0, Tablex 0.0.9, Codly 1.3.0, Fletcher 0.5.8, Showybox 2.0.4, Physica 0.9.3, MiTeX 0.2.4, Numbly 0.1.0 e Suiji 0.3.0. O cache privado também continha CeTZ 0.3.4, Codly 1.2.0, Oxifmt 0.2.1 e Xarrow 0.2.0: **14 versões/170 arquivos** no total. Xarrow veio do cache recebido, é referenciado por MiTeX e não sustenta aceitação funcional própria. Os outros pacotes foram obtidos dos arquivos versionados de `packages.typst.org/preview`; os dois inicialmente copiados do cache (CeTZ 0.5.2 e Oxifmt 1.0.0) foram comparados com os arquivos oficiais, sem diferenças ou extras. Publicação/versionamento não significam compatibilidade com toda a linguagem: [registro oficial de pacotes](https://github.com/typst/packages/blob/main/README.md).

| Resultado no baseline e no candidato | Pacotes/casos |
| --- | --- |
| Uso efetivo passou | Oxifmt (formatação e imports repetidos), Numbly (heading/numeração), Suiji (gerador e sequência) |
| Import passou; uso recusado igualmente | Showybox: tipo de `stroke` em `block` |
| Diagnóstico existente igual | Codly (`bool`), Tablex (`str(type)`), Physica (`strike(symbol)`), MiTeX (`h` negativo) |
| Fora da qualificação no envelope de recursos | CeTZ, Fletcher e fixture mista |

O primeiro smoke exploratório estava sem teto e observou custo de memória/tempo excessivo em Fletcher; foi interrompido. CeTZ chegou a passar nesse smoke, mas isso **não** o qualifica dentro do envelope posterior. A matriz foi refeita com limite de **espaço de endereçamento** de 4 GiB e timeout de 30 s por processo (launcher limitado a 6 GiB, swap=0). Abort/timeout desses casos são limitações de recursos, não prova de equivalência semântica. Candidato nos casos já excluídos não foi repetido para todos os perfis; linhas não executadas ficam explicitamente vazias.

Nos casos elegíveis, compararam-se **78 pares de PDFs** entre variantes, e **54 pares de diagnósticos existentes** coincidiram, preservando mensagem, ordem, spans e traces; somente o prefixo absoluto do diretório do caso foi normalizado. São 13 fixtures aceitas por perfil, incluindo controles locais. [Matriz funcional](../../lab/results/construction/diagnostics/stdlib-qualification-2026-11-06/package-functional-results.csv).

O harness SystemWorld/API passou nos dois ELFs com quatro wrappers compatíveis (Oxifmt, Numbly, Suiji, Showybox import) e sequência A/B/A de inputs em Worlds novos. Em outro teste, uma edição **somente no pacote** Oxifmt, numa cópia privada modificada para o teste, acionou o watch e mudou o valor observado de A para B nos dois ELFs. O corpus publicado usado nos relógios ficou imutável. Isso não generaliza o resultado aos pacotes excluídos acima.

## Desempenho com o ambiente normal de fontes

O primeiro lote usou `XDG_DATA_HOME` vazio para impedir que pacotes do host sombreassem o corpus. Isso também ocultou fontes do usuário: 1887 linhas na lista, contra 3203 no ambiente normal. Os resultados foram retidos em `isolated-fonts`, **sem misturá-los aos resultados normais**. O driver recebeu `--inherit-data-home` em `2026-11-06T13:57:38.109007+00:00`, antes do novo lote: preserva as fontes normais e recusa, antes dos relógios, qualquer diretório de pacotes do data-home não vazio/inverificável. A lista completa voltou a ser idêntica à normal. Os três ELFs continuaram os mesmos. Também mudou a preparação do ambiente para fora do intervalo cronometrado; portanto os protocolos dos dois lotes não são intercambiáveis.

Lote principal normal: 12 rodadas, uma repetição, seis fixtures, três ELFs, CLI e watch com dois pares de ciclos same-bytes/edit. CPU0, Rayon=1, stack=64 MiB, ordem balanceada, mesmo corpus e inputs; nenhuma compilação/debugger/extração de PDF durante os relógios. Wall inclui latência de publicação e debounce do watch; CPU do watch é delta do processo por ciclo. RSS é pico do processo inteiro, não por fase/ciclo.

Tabela com **512 inputs**, CPU em ms (baseline → candidato); porcentagens são medianas das razões pareadas por rodada. Valores absolutos são medianas das médias por rodada, por isso a porcentagem não precisa coincidir com a razão entre as duas medianas.

| Fixture | CLI | Startup watch | Watch após edição |
| --- | ---: | ---: | ---: |
| Sem imports | 93,36 → 93,56 (+0,08%) | 136,60 → 136,28 (−0,07%) | 19,57 → 19,36 (−0,68%) |
| Fanout 16 | 109,96 → 104,15 (−5,41%) | 153,62 → 146,67 (−4,41%) | 36,24 → 29,31 (−19,38%) |
| Oxifmt uso | 99,65 → 95,60 (−4,17%) | 141,76 → 137,17 (−2,98%) | 22,86 → 22,64 (−1,00%) |
| Oxifmt repetido | 120,21 → 113,70 (−5,09%) | 161,88 → 156,13 (−4,06%) | 38,34 → 31,59 (−17,34%) |
| Numbly | 95,77 → 96,13 (+0,52%) | 138,10 → 138,73 (+0,33%) | 21,01 → 21,23 (+0,41%) |
| Suiji | 105,09 → 101,67 (−3,22%) | 147,10 → 143,61 (−2,18%) | 30,31 → 27,18 (−10,94%) |

ICs bootstrap pareado de 95%, 10000 reamostragens/seed=0: em Oxifmt repetido, CLI [−5,70%; −4,46%], startup [−4,89%; −2,77%], edição [−17,65%; −16,93%]; em Suiji, respectivamente [−3,72%; −2,74%], [−3,35%; −1,22%], [−11,33%; −8,97%]. Os ganhos correspondentes em wall também foram observados: Oxifmt repetido −5,04%/−4,04%/−5,94%; Suiji −3,13%/−2,00%/−3,04%. No lote sem inputs, os ganhos de CPU na CLI foram menores: Oxifmt repetido −1,95%, Suiji −2,58%. Os CSVs incluem comparações contra o neutro e todos os controles, sem descarte posterior de rodadas. Intervalos são exploratórios, sem correção para múltiplas comparações.

O pico mediano de RSS normal ficou entre aproximadamente 25 e 41 MiB no corpus. Por exemplo, fanout grande no watch: 29,24 → 30,67 MiB; Oxifmt repetido: 41,15 → 41,19 MiB. **Não é uma otimização de memória demonstrada**, nem uma prova sobre retenção em sessões longas de watch.

### Confirmação do residual

Uma rodada adicional delimitada, mesmas fontes/ELFs, 20 rodadas com sem-imports/Numbly, perfis vazio/512 e um par same-bytes/edit por processo, não reproduziu como efeito estável a pequena regressão inicial de Numbly na CLI. Com 512 inputs, CLI Numbly: +0,05% [−0,74%; +0,59%] contra baseline e +0,13% [−0,18%; +0,34%] contra neutro.

Porém, sem-imports/512 apresentou CLI +0,74% [+0,30%; +1,17%] contra baseline e +0,56% [+0,01%; +0,75%] contra neutro; CPU absoluta mediana 93,99 → 94,40 ms. No watch edit de Numbly/512 apareceu +1,46% [+0,01%; +2,73%] contra baseline, sem separação conclusiva do neutro (+2,30% [−1,52%; +3,54%]). Não se escolheu apenas o lote favorável. São resíduos pequenos e dependentes da bancada, mas impedem declarar ausência de regressões. Contagens iguais de builders/inputs no controle não atribuem a causa ao algoritmo; codegen/disposição do ELF permanecem hipóteses não demonstradas.

## Dados e aceitação para integração futura

[Resultados de domínio](../../lab/results/construction/diagnostics/stdlib-qualification-2026-11-06/) conservam CSVs de medições, análises pareadas, RSS, contagens e classificação dos pacotes. Fixtures e inputs estão no laboratório. Tempos exatos de cada observação ficam em `sample_epoch`; a fonte produtiva congelada foi a mesma em todos os lotes. Os relógios ocorreram aproximadamente entre 13:44–13:50 UTC no ambiente isolado, 13:59–14:06 UTC no ambiente normal e 14:13–14:16 UTC na confirmação, em 2026-11-06.

Ao todo, os lotes de timing geraram 6144 publicações: todas passaram na verificação de texto/posições/fontes e marcadores/cardinalidade, com 4096 comparações entre variantes. Uma seleção de 166 pares/332 páginas também passou em rasterização a 50 dpi. Comparar geometria/raster aqui é controle adicional de regressão no mesmo compilador, não redefinição de paridade com a linguagem como igualdade de bytes do PDF. O oráculo vanilla não foi atualizado ou usado para certificar pacotes novos nesta rodada.

**Delimitação:** o candidato pode avançar como proposta interna de L1 para o registry builtin atual, sem cache de módulos ou mudança da prova de frescor de fontes. Não está qualificado como clone genérico de módulos de usuário, closures, plugins ou construtores dinâmicos. Novas variantes do registry exigem requalificação: o helper atual contém recusas/asserts para grafos fora do escopo.

Antes da integração, resolver ou aceitar explicitamente o orçamento do residual acima. Depois, atualizar primeiro os L0s atuais de `core/compiler/eval/mod.md` e `modules.md`, criar um owner 1:1 para o helper escolhido e transportar os testes para seus owners, preservando a invariável de identidade builtin. Os L0s atuais foram consultados: hashes de prompt nos consumidores `78301f9b`/`cb3a895c`, respectivamente. Reúso não está autorizado pelo simples fato de o laboratório passar. Não foram criados prompts órfãos ou alterados esses owners agora. Classificar a mudança final pela ADR-0127 antes de materializá-la; preservar assinaturas, defaults e fases permite tratar o mecanismo interno no fluxo correspondente.

Os três builds release e `crystalline-lint --quiet .` passaram. O lint refere-se ao produto ativo preservado; overlays RAM não foram apresentados como código produtivo selado. Esta rodada executou os harnesses proporcionais descritos, não toda a suíte core/infra. Em `2026-11-06T14:20:12.553334+00:00`, os 53169 arquivos recebidos e o diffstat continuavam idênticos. Builds/logs/ELFs/PDFs/mutantes/caches próprios foram descartados após a coleta, preservando o cache RAM anterior; mantiveram-se código de laboratório, testes e medições de domínio.
