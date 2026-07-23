# Prompt — typst-passo-875: filtrar a busca de fallback de fonte (tempo de execução em math, causa dominante)

**Origem**: causa 2 de P873 — `CandidateSet::covering_all` (`03_infra/src/shaper.rs:664-685`) percorre o `FontBook` inteiro sem filtro nem early-exit quando um glifo não é coberto pelas listas curadas de fallback; para símbolos matemáticos/gregos incomuns, isso força abrir/ler cada face de cada fonte de sistema (incluindo `.ttc` CJK de 19-27MB), e `FontSlot::get()` (`03_infra/src/fonts.rs:56-75`) lê o arquivo inteiro da coleção por face, sem compartilhar bytes entre faces irmãs do mesmo arquivo físico. P873 mediu 79% do tempo de compilação de math em `system time`, atribuído a isso.
**Estado**: aguardando execução — **esta é a causa dominante do tempo, prioridade sobre P874/P876 se só um puder ser feito primeiro**.

---

## Duas correções distintas aqui, não confundir

1. **Evitar abrir fontes que não vão cobrir o glifo de qualquer forma** — CJK não cobre símbolos matemáticos/gregos, então nem precisava tentar. Isso é filtro/early-exit na busca.
2. **Quando uma coleção `.ttc` precisa mesmo ser aberta, não reler o arquivo inteiro por face** — as 10 faces de `NotoSansCJK-Regular.ttc` compartilham o mesmo arquivo físico; hoje cada uma lê os ~19MB do zero. Isso é cache/compartilhamento de bytes entre faces irmãs.

As duas resolvem o mesmo sintoma por ângulos diferentes — fazer as duas, não escolher uma achando que basta.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate.

## Passo 1 — Confirmar a inferência que P873 deixou marcada como não-instrumentada

P873 identificou a causa por correlação (delta de `openat` batendo com o número de faces do `.ttc`), mas registrou explicitamente que não instrumentou o código para confirmar de fato qual glifo dispara qual abertura. Fazer essa confirmação antes de corrigir:

1. Adicionar instrumentação temporária em `CandidateSet::covering_all` (`shaper.rs:664`) — exatamente como P873 sugeriu: imprimir `c` (o caractere) e `slot_idx` a cada chamada de `load_fallback`.
2. Rodar o documento de math do benchmark (`04-math.typ` de P872, reaproveitar o arquivo se ainda existir em `/tmp/p872-bench/`) e confirmar que os `slot_idx` correspondentes às faces de `NotoSansCJK-Regular.ttc` (e as outras `.ttc` CJK) de fato aparecem, para os glifos matemáticos/gregos esperados.
3. Remover a instrumentação temporária depois de confirmar (não deixar `eprintln!` de debug no código final).

## Passo 2 — Filtrar a busca por classe/cobertura antes de tentar abrir a fonte

1. Confirmar se o `FontBook` já guarda alguma informação de cobertura Unicode por fonte sem precisar abrir o arquivo (ex.: um bitmap/faixa de cobertura pré-calculado nos metadados leves já lidos na descoberta inicial, que P873 confirmou ser barata via mmap). Se essa informação já existir, usá-la para pular fontes que claramente não cobrem a faixa de código do caractere buscado (CJK não cobre símbolos matemáticos Unicode, por exemplo) sem nunca chegar a abrir o arquivo.
2. Se essa informação de cobertura não existir ainda nos metadados leves, avaliar o custo de adicioná-la (provavelmente já é extraída durante o parse leve de descoberta, só não está sendo usada para filtrar) — antes de abrir mão dessa otimização e escolher outra abordagem.
3. Alternativa/complementar: ordenar a busca de fallback para tentar primeiro fontes mais prováveis de cobrir o tipo de caractere (símbolos matemáticos, gregos) e só cair nas fontes CJK gigantes por último, quando estritamente necessário — reduz o custo médio mesmo sem filtro perfeito.

## Passo 3 — Compartilhar bytes entre faces irmãs de uma coleção `.ttc`

1. Em `FontSlot::get()` (`fonts.rs:56-75`), trocar a leitura individual por face por um cache compartilhado por arquivo físico: a primeira face de um `.ttc` que precisar ser lida carrega o arquivo inteiro uma vez (ou usa mmap, como a descoberta inicial já faz, em vez de `std::fs::read` completo), e as faces seguintes do mesmo arquivo reaproveitam esses bytes já carregados, só recortando (`extract_collection_face`) a face específica.
2. Confirmar que isso não quebra o padrão de cache por `OnceLock` já existente por slot — é um cache adicional por arquivo, não uma substituição do cache por face.

## Passo 4 — Validação

1. Repetir a medição de `strace -c` de P873 para o documento de math — confirmar que o número de chamadas `read()` e o tempo gasto nelas caíram de forma proporcional ao Passo 2/3 (filtro evita abrir; compartilhamento evita reler).
2. Repetir o benchmark de tempo de P872 para o cenário de math — este é o critério de fechamento: a razão cristalino/vanilla deve cair de 22× para algo muito mais próximo de 1×, já que esta era a causa dominante (79% do tempo).
3. Confirmar que nenhum outro cenário do benchmark de P872 regride (o filtro de cobertura não pode fazer um glifo que devia ser encontrado deixar de ser encontrado — testar os casos de fallback que achados anteriores já validaram, como o fallback CJK de P838, para garantir que não quebrou).
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-875-relatorio.md` com: a confirmação por instrumentação (Passo 1, removida depois), o diff das duas correções (filtro + compartilhamento de bytes), a medição de `strace`/tempo antes e depois, a confirmação de que o benchmark de math melhorou substancialmente sem regredir outros fallbacks (especialmente o de P838), e as contagens de teste discriminadas por crate.
