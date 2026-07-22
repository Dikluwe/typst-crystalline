# Prompt — typst-passo-835: `image::pdf` — PDF como fonte de imagem (achado #20, decisão de escopo já tomada em P781) + parâmetro `page:` ausente

**Origem**: achado #20 de P831 (lote 5)
**Estado**: aguardando execução parcial — a parte de escopo já tem decisão (P781); a parte do parâmetro `page:` é um achado novo dentro do mesmo módulo

---

## Achado (medição de P831)

`#image("pdf1_fonte.pdf")` — cristalino `error: PDF images are not supported yet` (exit 1, scope-out já consciente e documentado desde P781, handoff antigo — peso de dependência `hayro`/`vello`); vanilla compila. **Ponto novo, não coberto pela decisão de P781**: o parâmetro nomeado `page:` de `image()` não existe no cristalino (`error: argumento nomeado inesperado em image(): 'page'`), mesmo fora do contexto de PDF-como-fonte.

---

## Passo 1 — Confirmar que o scope-out de PDF-como-fonte continua válido

Não é uma sonda nova — só confirmar que a decisão de P781 (registrada no handoff antigo) ainda está formalizada como dívida (procurar em `00_nucleo/diagnosticos/debt/DEBT.md`). Se sim, não reabrir esta parte. Se a entrada tiver desaparecido ou nunca ter sido formalizada como as outras (mesmo padrão da descoberta de P807), formalizar agora com a medição deste passo.

## Passo 2 — Sonda do parâmetro `page:` (achado novo, não coberto por P781)

1. Testar `page:` em `image()` com uma fonte que **não** seja PDF (ex.: um PNG multi-nada, ou confirmar se `page:` só faz sentido para fontes com múltiplas páginas — provavelmente só PDF e talvez GIF animado/TIFF multi-página no vanilla). Localizar no vanilla (`lab/typst-original/`) a assinatura completa de `image()` e onde `page:` se aplica.
2. Confirmar se `page:` é relevante **apenas** para fontes PDF (caso em que ele fica sem uso prático até o scope-out de PDF-como-fonte ser revertido) ou se também afeta outros formatos multi-frame.

## Passo 3 — Implementação

Se `page:` só se aplica a PDF (e PDF continua scope-out): registrar `page:` como argumento aceito mas sem efeito útil ainda (documentado como decorrente do scope-out de PDF), ou decidir não implementá-lo até o scope-out de PDF ser resolvido — decisão a registrar, não assumir sozinho qual das duas.

## Passo 4 — Validação

Se `page:` for implementado (mesmo que só para formatos que já existem): teste cobrindo o argumento. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-835-relatorio.md` — confirmação do scope-out de PDF-como-fonte (Passo 1), sonda e decisão sobre `page:` (Passos 2-4).
