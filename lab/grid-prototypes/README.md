# Protótipos de layout de células

Este laboratório **modela** os quatro eixos da investigação de grid em código
executável. Não importa o L1 e não produz PDF Typst; suas medidas de tempo **não
são estimativas de ganho do Tekt**. O propósito é testar a compatibilidade das
estratégias, descobrir condições de invalidação e escolher experimentos no motor
real. O comportamento de produção permanece inalterado.

```sh
cargo test --manifest-path lab/grid-prototypes/Cargo.toml
cargo run --release --manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes -- --rows 80
cargo run --release --manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes -- --rows 320 --unique
cargo run --release --manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes -- --rows 320 --first-page
taskset -c 0 cargo run --release --manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes-measure -- --rows 80 --blocks 31 --iterations 200
taskset -c 0 cargo run --release --manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes-measure -- --rows 80 --word-repeat 16 --blocks 31 --iterations 200
```

O executável percorre as 16 combinações dos quatro eixos, compara a saída de
cada uma ao baseline e imprime tempo mediano e contadores. A saída inclui
larguras, alturas e linhas posicionadas por página. Uma divergência termina a
execução com erro. Execute em processo ocioso e compare contadores antes de
interpretar tempos. `--unique` acrescenta uma identidade ao texto de cada
célula; assim testa quando o cache de largura perde sua vantagem.

`grid-prototypes-measure` faz a medição temporal em blocos, aquece cada modo,
alterna a ordem dos 16 modos e imprime mediana, percentis 10/90 e contadores.
Use este binário para comparar tempos; o binário principal é uma exploração
rápida. Os percentis refletem variação entre blocos, não intervalo de confiança.
`--word-repeat N` multiplica as palavras de cada célula para estudar o custo
da largura intrínseca com conteúdo mais longo.

| Eixo | Operação experimental | Evidência que esperamos | Limite do modelo |
| --- | --- | --- | --- |
| `reuse` | Guarda o frame da medição e o move para emissão quando largura, estilo e inset coincidem; recusa conteúdo sensível à página, span e alinhamento vertical. | Menos layouts completos, saída igual. | Identidade real de `Content`, `place_cells`, decoração e `PendingAlign*` não modelados. |
| `height` | Mede linhas sem guardar itens, para células cujo frame não será reutilizado. | Menos frames produzidos; a contagem total de quebras não diminui. | O caminho real `layout_sub_frame` tem efeitos que o modelo não representa. |
| `window` | Resolve cada altura imediatamente antes de emitir a linha. | Menos linhas medidas até a primeira página; menor pico de frames com `reuse`. | Em documento completo, todas as linhas ainda são medidas; ganhos totais não decorrem da janela sozinha. |
| `width` | Guarda largura intrínseca por identidade de uma alocação de palavras já compartilhada e escala de fonte; conteúdo não compartilhado passa direto. | Acertos sem percorrer/copiar palavras; saída igual. | A fixture repetida compartilha `Arc<[u16]>` antes do cronômetro. Ainda não há prova de que as células repetidas do Tekt compartilhem a mesma identidade; Tekt já usa cache de métricas de fonte. |

## Combinações

`reuse+height` mede um frame completo somente para células elegíveis a reúso;
as demais usam o caminho de altura. `reuse+window` conserva no máximo os frames
da linha corrente, enquanto `reuse` sem janela pode conservar frames de todas
as linhas antes de emitir. `width` é independente dos outros eixos no modelo.

O primeiro cache estrutural (chave copiada), o cache estrutural com chave
emprestada e hash rápido foram mais lentos no modelo. A chave por identidade
evita construir e comparar conteúdo em cada consulta, mas **só** produz
acertos quando o mesmo objeto já é compartilhado. A construção da identidade
compartilhada não está no tempo de `run` e não pode ser atribuída gratuitamente
ao compilador real.

## Passagem para o motor real

1. Registrar identidade e entradas causais em `tracks.rs` e `cell_body.rs`:
   corpo, estilo, largura, inset, alinhamento, posição e página. Contar
   elegíveis e recusados; não assumir que duas chamadas são equivalentes.
2. Testar `reuse` apenas em células simples de uma página, com comparação de
   texto, geometria e decoração; ampliar para spans e paginação somente após
   um caso equivalente demonstrado.
3. Testar `height` separadamente para medir custo de itens descartados. A
   medição de texto e de quebra permanece necessária.
4. Testar `window` com documento de muitas páginas, comparando latência da
   primeira página, pico de memória e tempo total; preservar a decisão de
   paginação anterior a tracks fracionários.
5. Verificar se há identidade compartilhada entre células reais antes de
   testar `width`; medir também o custo de criá-la, caso ela não exista, e
   comparar aos caches de fonte existentes.

Antes de alterar o L1, atualizar os owners L0 de `grid/tracks`, `grid/cell_body`
e, se necessário, `grid/row_pagination` e `layout/sub_frame`. O L0 atual de
tracks devolve somente tamanhos, e não legitima transmitir frames à emissão.
