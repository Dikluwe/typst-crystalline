# Prompt — typst-passo-872: medir tempo de compilação de documentos — cristalino vs vanilla

**Origem**: primeira medição de performance do projeto até agora — todos os passos anteriores mediram paridade de comportamento/observável, nenhum mediu velocidade.
**Estado**: aguardando execução — **isto é medição, não otimização**. Se o cristalino for mais lento, este passo não conserta nada, só relata com precisão onde e por quanto.

---

## Por que ter metodologia importa aqui mais do que em outros passos

Divergência de comportamento (o que a maioria dos achados deste projeto mede) é binária: bate ou não bate. Velocidade é ruidosa — varia por cache do sistema de arquivos, por outros processos rodando na máquina, por variação de sistema entre corridas. Uma medição de performance feita sem cuidado metodológico não é só imprecisa, é enganosa: pode dizer "cristalino é 3x mais lento" quando na verdade é uma corrida com o disco ocupado por outra coisa. Este passo precisa de disciplina de benchmark, não só rodar `time` uma vez e anotar o número.

---

## Passo 1 — Preparar os binários corretamente

1. Confirmar que os dois binários estão em build de release (`--release`), não debug — comparar debug com release não mede nada útil. `lab/typst-original/target/release/typst` e `./target/release/typst` (recompilado do estado atual, pós-P871).
2. Confirmar as versões exatas (mesmo cuidado que faltou em P870): vanilla `0.15.0 (969087ec)`, cristalino no commit atual.
3. Rodar cada binário uma vez "a frio" antes de medir, para eliminar efeitos de cache de disco/SO na primeira execução de cada um (descartar essa primeira corrida, não contar nas médias).

## Passo 2 — Escolher os documentos de teste

Não usar um único documento pequeno — isso mede overhead de inicialização do processo, não velocidade de compilação real. Usar uma variedade:

1. **Trivial**: `Hello World` (mede overhead fixo/startup).
2. **Texto corrido médio**: `#lorem(500)` ou equivalente (mede shaping/layout de texto em volume).
3. **Documento com muitas imagens**: repetir `#image(...)` várias dezenas de vezes (mede o caminho de decodificação/embedding de imagem).
4. **Documento com matemática**: várias equações, incluindo as combinações já tratadas em achados anteriores (sub/superscript, símbolos gregos) — mede o caminho de layout matemático, que teve bastante trabalho extra no cristalino (P799, P809, etc.).
5. **Documento com tabelas/grids grandes**: várias tabelas com muitas células (mede o caminho de grid/placement).
6. **Documento longo com muitas páginas**: algo que force paginação real (dezenas de páginas), incluindo headers/footers/numeração.
7. **Documento com `#context`/`measure()` repetido muitas vezes**: relevante especificamente por causa da P858 — a injeção de `FontMetrics` no `Engine` durante `expand_context_blocks` tem custo que nunca foi medido; um documento com muitos `#context` isola esse caminho.

## Passo 3 — Metodologia de medição

1. Para cada documento, rodar **pelo menos 10 corridas** de cada binário (mais, se o tempo por corrida for pequeno o suficiente para caber), alternando entre os dois binários a cada corrida (não rodar 10x cristalino seguido e depois 10x vanilla — isso confunde variação temporal do sistema com diferença real entre binários).
2. Reportar não só a média, mas também mediana e desvio padrão (ou min/max) — para saber se a diferença medida é maior que o ruído.
3. Usar uma ferramenta de benchmark adequada se disponível no ambiente (`hyperfine` é o padrão para isso — confirmar se está instalado; se não, `time` com múltiplas corridas manuais e cálculo próprio de estatística serve, mas é mais trabalhoso e propenso a erro).
4. Rodar num momento em que a máquina não esteja sob carga de outro processo pesado (nada de rodar isso enquanto outro passo compila em paralelo).

## Passo 4 — Relatar sem interpretar demais

1. Tabela com os sete casos do Passo 2, cada um com: tempo médio vanilla, tempo médio cristalino, razão (cristalino/vanilla), e a variação (desvio padrão ou min/max) dos dois.
2. Não arredondar para "cristalino é 2x mais lento" como conclusão geral se os sete casos tiverem razões muito diferentes entre si — reportar caso a caso, porque o motivo de lentidão (se houver) provavelmente é diferente em cada um, e misturar tudo numa única razão esconde onde o problema real está.
3. Se algum caso individual mostrar uma diferença muito maior que os outros (ex.: o caso de `#context`/`measure()` sendo desproporcionalmente mais lento), isso é a pista mais valiosa deste passo — não é para investigar a causa aqui, é para apontar com precisão para um passo futuro de otimização, se o dono decidir que vale a pena.

## Relatório

`00_nucleo/diagnosticos/typst-passo-872-relatorio.md` com: a tabela dos sete casos (Passo 4), a metodologia usada (ferramenta, número de corridas, forma de alternância), e qualquer caso que se destoe claramente dos outros, sinalizado para investigação futura — sem propor solução, só apontar onde olhar.
