# Prompt — typst-passo-876: corrigir deduplicação de imagens repetidas (tempo e tamanho do PDF)

**Origem**: causa 3 de P873 — o mecanismo de deduplicação de imagem em `03_infra/src/export/images.rs:263-268` usa identidade de ponteiro `Arc` como chave, o que é correto *se* o mesmo `Arc` for reutilizado entre chamadas idênticas a `image()` — mas `Content::Image(Arc::new(self.clone()))` (`01_core/src/entities/elements/image.rs:46,53`) aloca um `Arc` novo a cada avaliação do elemento, então 50 chamadas a `image("mesmo-arquivo.png")` produzem 50 `Arc`s distintos e 0 reaproveitamento — confirmado por P873 via `mutool info` (50 XObjects no cristalino vs 1 no vanilla).
**Estado**: aguardando execução

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate.

## Passo 1 — Confirmar a inferência que P873 deixou marcada como não-instrumentada

P873 identificou a origem provável (novo `Arc` a cada avaliação) mas não instrumentou para confirmar que o problema está na avaliação de `image()` e não em algum outro ponto do caminho até a decodificação. Confirmar antes de corrigir:

1. Instrumentar temporariamente `scan_all_images` (ou o ponto equivalente) para imprimir `Arc::as_ptr(data)` de cada uma das 50 imagens do documento de teste — confirmar que os 50 ponteiros são de fato todos distintos (esperado, dado o resultado do PDF), e não algum outro tipo de falha no mecanismo de dedup em si.
2. Confirmar também se a **decodificação** da imagem (não só a alocação do `Arc`) está sendo repetida 50 vezes — ou seja, se o custo extra é só de memória (Arc duplicado apontando para os mesmos bytes decodificados uma vez) ou se cada chamada decodifica o PNG do zero de novo (custo de CPU adicional, não só de armazenamento/serialização no PDF).
3. Remover a instrumentação temporária depois de confirmar.

## Passo 2 — Decidir o nível certo do cache

A correção pode acontecer em pontos diferentes, com trade-offs diferentes — decidir com base no que o Passo 1 confirmar:

1. **Cache por conteúdo de arquivo (mais simples, mais amplo)**: cachear a imagem decodificada por caminho/`FileId` (não por identidade de `Arc`), de forma que chamadas repetidas a `image()` com o mesmo caminho reaproveitem tanto a decodificação quanto o `Arc` de dados. Isso resolveria tanto o custo de decodificação repetida (se existir, confirmado no Passo 1.2) quanto o de duplicação no PDF.
2. **Cache só no ponto de export (mais restrito)**: se a decodificação já é barata e o problema é só a duplicação no PDF final, o cache pode viver só em `scan_all_images`/no exportador, usando conteúdo (hash dos bytes) em vez de identidade de ponteiro como chave de deduplicação — sem mudar como `image()` é avaliado.
3. Preferir a opção que resolve a causa mais cedo no pipeline (Passo 2.1) se o Passo 1.2 confirmar que há decodificação repetida — isso economiza tanto tempo de CPU quanto tamanho de PDF. Se não houver decodificação repetida (só duplicação de referência no export), a opção mais restrita (2.2) é suficiente e mais simples.

## Passo 3 — Implementação

Conforme a decisão do Passo 2, implementando o cache no ponto certo, com chave baseada em conteúdo/caminho do arquivo, não em identidade de ponteiro de execução (que nunca vai deduplicar entre avaliações distintas do mesmo elemento).

## Passo 4 — Validação

1. Repetir a medição `mutool info` de P873 — confirmar que o cristalino agora produz **1** XObject de imagem para o documento com `image()` chamado 50 vezes com o mesmo arquivo, referenciado 50 vezes nas páginas (igual ao vanilla), não 50 XObjects distintos.
2. Repetir o benchmark de tempo e tamanho de PDF de P872 para o cenário de imagens — confirmar melhora substancial na razão cristalino/vanilla (de 16× tempo / 7× tamanho para algo muito mais próximo de 1×).
3. Testar um caso de controle: duas chamadas a `image()` com arquivos **diferentes** — confirmar que continuam gerando XObjects distintos (a deduplicação não pode fundir imagens diferentes por engano).
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-876-relatorio.md` com: a confirmação por instrumentação (Passo 1, removida depois, incluindo se havia decodificação repetida ou só duplicação de referência), a decisão de onde o cache foi implementado e por quê, o diff, a medição antes/depois de `mutool info` e do benchmark, o teste de controle de imagens diferentes, e as contagens de teste discriminadas por crate.
