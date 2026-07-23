# Prompt — typst-passo-877: diagnosticar a regressão de performance deixada por P874-876 antes de qualquer nova correção

**Origem**: reprodução completa do benchmark de P872 após P874-876 mostrou que as sete razões pioraram, não melhoraram — regressão uniforme de ~+0.21× nos cenários simples, e uma explosão desproporcional em imagens (2334 `openat` + 2981 `readlink`, contra ~20 do vanilla), enquanto matemática mal se moveu apesar do PDF ter caído de 1.8MB para 157KB.
**Estado**: aguardando execução — **diagnóstico primeiro, de novo**. A tentativa anterior (P875) corrigiu algo real mecanicamente mas não validou contra o benchmark completo antes de considerar fechado, e isso escondeu que a correção não resolvia a causa dominante e ainda introduzia uma regressão nova. Não repetir esse erro: nenhuma correção neste passo sem confirmar a causa primeiro, e nenhum fechamento sem rodar o benchmark completo de novo no final.

---

## Duas coisas distintas a investigar, não uma só

Não assumir que a regressão uniforme (cenários simples) e a explosão de imagens têm a mesma causa. Os números não sustentam isso: +0.21× é pequeno e consistente; +7.32× em imagens é grande e específico. Tratar como duas investigações separadas até haver evidência de que são a mesma coisa.

---

## Passo 1 — Isolar a regressão uniforme (hello, lorem, tables, context)

1. Confirmar a hipótese já levantada: `font_info_from_bytes` (P875) passou a extrair `coverage` (percorrendo a tabela `cmap`) — confirmar **onde exatamente** essa extração é chamada. Se for durante `discover_fonts`/`pair_slots_with_book`, incondicionalmente para todas as ~1086 fontes do sistema em toda compilação (mesmo quando o documento não usa fallback nenhum), isso explica um custo fixo por execução que não existia antes de P875.
2. Medir diretamente: `strace -c` do cenário `01-hello` antes e depois de P875 (reverter temporariamente só a parte de extração de coverage, sem mexer no resto, para isolar) — confirmar quantas chamadas de leitura de `cmap` acontecem e quanto tempo isso consome.
3. Se confirmado: a extração de coverage precisa ser lazy (só quando a fonte é de fato candidata a ser usada, não para as 1086 de uma vez na descoberta inicial) — mas não implementar a correção ainda neste passo, só confirmar e localizar.

## Passo 2 — Isolar a explosão de imagens (2334 openat + 2981 readlink)

1. Este número é grande demais para ser só o custo fixo do Passo 1 — confirmar isso comparando a contagem de syscalls do cenário `03-images` contra o cenário `01-hello` (que sofre só a regressão uniforme). Se `03-images` tiver muito mais chamadas que `01-hello`, a causa é específica de imagens, não o custo fixo geral.
2. Investigar se o caminho de imagem está de alguma forma acionando busca de fallback de fonte repetidamente — parece contraintuitivo (imagem não devia precisar de fonte nenhuma), mas os números sugerem isso. Confirmar se há algum texto no documento `03-images.typ` do benchmark (legendas, numeração de figura, etc.) que dispare shaping de texto, e se esse shaping está de alguma forma iterando o fallback de forma repetida por imagem (uma vez por chamada a `image()`, por exemplo, em vez de uma vez só).
3. Usar `strace` com `-T` (tempo por chamada) e `-e trace=openat,readlink` filtrado, comparando com/sem as correções de P874-876 aplicadas uma de cada vez (não as três juntas) — isolar qual das três (ou qual interação entre elas) introduziu especificamente esse comportamento em imagens. É possível que nenhuma delas sozinha cause isso, e seja uma interação entre duas.
4. Confirmar se `readlink` (não só `openat`) tem relação com a resolução de fontes do sistema via `fontconfig`/`fontdb` — esse padrão de syscall é típico de percorrer symlinks de diretório de fontes; confirmar se isso está sendo repetido por chamada de imagem, ou uma vez só.

## Passo 3 — Não corrigir ainda, só relatar com precisão

Este passo termina com as duas causas (regressão uniforme e explosão de imagens) explicadas e localizadas no código, com evidência de medição (não suposição), mas sem nenhuma correção implementada. A correção é o próximo passo, escrito depois com base no que este encontrar — mesmo padrão que funcionou bem em P873 (diagnóstico) → P874/875/876 (correção), só que desta vez a correção anterior não validou contra o benchmark e este passo existe para não repetir isso.

## Relatório

`00_nucleo/diagnosticos/typst-passo-877-relatorio.md` com: a confirmação (ou refutação) da hipótese de coverage eager para os cenários simples, com medição direta; a causa isolada da explosão de syscalls em imagens, com a evidência de qual dos três passos (ou qual interação) a introduziu; e nenhuma correção de código. O relatório precisa deixar claro, para quem escrever o próximo passo de correção, exatamente onde mexer e por quê — sem repetir o erro de P875 de corrigir sem confirmar o efeito real no benchmark completo.
