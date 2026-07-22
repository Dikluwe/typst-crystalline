# Prompt — typst-passo-833: `image::raster` — GIF/WebP ausentes (#17) e PNG corrompido omitido silenciosamente (#18, GRAVE)

**Origem**: achados #17 e #18 de P831 (lote 5)
**Estado**: aguardando execução — prioridade máxima da fila, junto com o achado #58 (P832)

---

## Achado #18 — GRAVE: PNG corrompido é omitido silenciosamente, exit 0

**Medição de P831**: PNG corrompido — vanilla `error: failed to decode image (Format error decoding Png: ...)` (exit 1); cristalino **exit 0 com PDF válido e a imagem simplesmente ausente** (um `eprintln!` interno diz "PNG inválido — imagem omitida", mas isso não chega ao usuário nem falha a compilação). Causa: validação só de assinatura em `figure_image.rs:189`; a falha real só aparece tarde, em `03_infra/src/export/images.rs:348`, onde é engolida. Vanilla: `raster.rs:448-453`.

### Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

### Sonda
1. Reproduzir com um PNG corrompido (assinatura válida, dados inválidos) nos dois binários — confirmar exit code e mensagem.
2. Confirmar exatamente onde a decodificação falha no cristalino (`03_infra/src/export/images.rs:348`) e por que esse erro não propaga para o nível de compilação (é um `Result` sendo descartado, um `.ok()`, um `unwrap_or_default`?).
3. Localizar a mensagem de erro exata do vanilla e o formato (`Format error decoding Png: {detalhe}`).

### Implementação
Propagar o erro de decodificação de `03_infra/src/export/images.rs` até o ponto de compilação, transformando em erro de compilação (não warning, não `eprintln!` interno) com a mensagem no formato do vanilla. Remover o comportamento de "omitir e seguir" — qualquer imagem que falhe a decodificação deve interromper a compilação com erro, como o vanilla.

### Validação
1. Recompilar. PNG corrompido agora erra com exit 1 e mensagem batendo com o vanilla.
2. Confirmar que PNG válido continua funcionando sem regressão.
3. Testar também JPEG corrompido, se o mesmo caminho de código for compartilhado, para confirmar que a correção não é específica de PNG.
4. Suíte completa, comando + contagem antes/depois.

---

## Achado #17 — GIF e WebP não suportados

**Medição de P831**: `#image("arquivo.gif")` — cristalino `error: unknown image format` (exit 1); vanilla compila (GIF fica estático no frame 1, sem animação). Vanilla: `raster.rs:80-81,248-251` (`GifDecoder`/`WebPDecoder`). Cristalino: `01_core/src/entities/image_format.rs:14-30` (enum só `Jpeg|Png|Unknown`).

### Sonda
Confirmar com GIF e WebP simples nos dois binários. Confirmar se o vanilla usa alguma crate específica para decodificação (`image` crate com features `gif`/`webp`, provavelmente) e se essa crate (ou as features) já está disponível como dependência no cristalino.

### Implementação
Adicionar as variantes `Gif`/`WebP` ao enum de formato e a decodificação correspondente, usando a mesma crate/abordagem do vanilla se possível (evita reinventar o decoder). Se a crate não estiver disponível como dependência L1/L3, isso é uma decisão de escopo (peso de dependência) — seguir o padrão de decisão explícita (P807/P812-C) em vez de implementar parcialmente.

### Validação
GIF e WebP simples renderizando (frame estático, como o vanilla — sem necessidade de suportar animação). Suíte completa, comando + contagem antes/depois.

---

## Relatório

`00_nucleo/diagnosticos/typst-passo-833-relatorio.md`, uma seção por achado (#17, #18), com medição antes, código identificado, diff (ou decisão de escopo formal para #17 se for o caso), medição depois, contagem de testes.
