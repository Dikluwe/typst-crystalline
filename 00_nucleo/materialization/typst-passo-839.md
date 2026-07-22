# Prompt — typst-passo-839: `text::font::info` — quatro achados de resolução de nome/estilo de fonte (#25-#28)

**Origem**: achados #25, #26, #27, #28 de P831 (lote 5)
**Estado**: aguardando execução

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos. Fixtures com fontes sintéticas geradas por fontTools (mesma técnica usada em P831, arquivos em `temp/p831/fonts/`) — reaproveitar essas fixtures se ainda existirem, ou regenerar de forma equivalente.

---

## Achado #25 (I1) — sem aparo de sufixos do name ID1

`#set text(font: "TriagX")` numa fonte cujo ID1 é `"TriagX Bold"` — vanilla compila (reconhece o nome base aparando o sufixo de peso/estilo); cristalino `warning: unknown font family: triagx` (usa o ID cru). Vanilla: `info.rs:73-77,206-267`. Cristalino: `03_infra/src/fonts.rs:189-198`.

### Sonda/Implementação
Testar variações de sufixo (Bold, Italic, Bold Italic, Regular) e confirmar a lógica exata de aparo do vanilla (provavelmente baseada em uma lista de sufixos conhecidos). Implementar a mesma lógica em `fonts.rs`.

## Achado #26 (I2) — sem `decode_mac_roman`

Fonte só com nomes no formato Macintosh (não Windows/Unicode) — vanilla decodifica e encontra; cristalino não decodifica esse formato e não encontra a fonte (`warning: unknown font family`). Vanilla: `info.rs:168-203`. Cristalino: `fonts.rs:196`.

### Sonda/Implementação
Confirmar a tabela de decodificação Mac Roman do vanilla (charset de 8 bits específico da Apple) e implementar equivalente no cristalino.

## Achado #27 (I3) — sem inferência de estilo pelo full name

Fonte sem bits de estilo explícitos na tabela OS/2, mas com "Oblique" no full name (ex. `"TriagSlant Oblique"`) — vanilla infere `style: "oblique"` a partir do nome; cristalino marca `Normal`. Vanilla: `info.rs:80-103`. Cristalino: `fonts.rs:200-206`.

### Sonda/Implementação
Confirmar a heurística do vanilla (provavelmente busca de substring no full name: "Bold", "Italic", "Oblique") e replicar.

## Achado #28 (I4) — FontBook e font_slots desalinhados

Quando uma fonte falha a extração de informações (info malformada), o cristalino cria o slot de qualquer forma (`discover_fonts`, `fonts.rs:159-164`) mas só faz push condicional no FontBook (`build_font_book`, `fonts.rs:223-234`) — os índices dessincronizam, e o fallback acaba renderizando com a face errada (medido: 11pt/22pt em vez de 16.5pt do vanilla). Vanilla sempre empareiha slot e info (`typst-kit/src/fonts.rs:39-43`).

### Sonda/Implementação
Reproduzir uma fonte com info malformada isolando o caso. Corrigir para que o slot só seja criado quando a info é extraída com sucesso (ou, alternativamente, sempre inserir no FontBook mesmo com info parcial, replicando o que o vanilla de fato faz — confirmar qual das duas abordagens o vanilla usa antes de escolher).

---

## Validação (comum aos quatro achados)
1. Recompilar. Repetir os quatro casos de sonda, batendo com o vanilla.
2. Confirmar que fontes "normais" (sem esses casos de borda) continuam resolvendo sem regressão.
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-839-relatorio.md`, uma seção por achado (#25-#28), testes nomeados `p839a_...` a `p839d_...`.
